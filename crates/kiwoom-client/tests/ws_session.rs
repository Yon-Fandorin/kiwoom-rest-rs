use async_trait::async_trait;
use kiwoom_client::{ClientError, Secret, ws::conditions::*, ws::session::*};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    future::{Future, poll_fn},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::Poll,
    time::Duration,
};

#[derive(Default)]
struct Mock {
    sent: Arc<Mutex<Vec<Value>>>,
    received: VecDeque<(Duration, String)>,
    send_steps: VecDeque<(Duration, bool)>,
    send_started: Arc<tokio::sync::Notify>,
    receive_count: Arc<AtomicUsize>,
    receive_gate: Option<(usize, Arc<tokio::sync::Notify>)>,
    close_delay: Duration,
    close_count: Arc<AtomicUsize>,
}

impl Mock {
    fn with(values: Vec<Value>) -> Self {
        Self {
            received: values
                .into_iter()
                .map(|v| (Duration::ZERO, v.to_string()))
                .collect(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl WsTransport for Mock {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        self.send_started.notify_one();
        if let Some((delay, fail)) = self.send_steps.pop_front() {
            if !delay.is_zero() {
                tokio::time::sleep(delay).await;
            }
            if fail {
                return Err(ClientError::Transport);
            }
        }
        Ok(())
    }
    async fn receive(&mut self) -> Result<String, ClientError> {
        let index = self.receive_count.fetch_add(1, Ordering::SeqCst);
        if let Some((blocked_index, gate)) = &self.receive_gate
            && index == *blocked_index
        {
            gate.notified().await;
        }
        let (delay, text) = self.received.pop_front().ok_or(ClientError::Transport)?;
        tokio::time::sleep(delay).await;
        Ok(text)
    }
    async fn close(&mut self) -> Result<(), ClientError> {
        self.close_count.fetch_add(1, Ordering::SeqCst);
        if !self.close_delay.is_zero() {
            tokio::time::sleep(self.close_delay).await;
        }
        Ok(())
    }
}

fn request() -> Ka10172Request {
    serde_json::from_value(json!({"trnm":"CNSRREQ","seq":"1","search_type":"0","stex_tp":"K"}))
        .unwrap()
}

fn options() -> SessionOptions {
    SessionOptions {
        timeout: Duration::from_millis(100),
        ..Default::default()
    }
}

async fn login(session: &mut WsSession<Mock>) {
    session
        .login(&Secret::new("synthetic".into()))
        .await
        .unwrap();
}

async fn cancel_after_first_poll(future: impl Future) {
    let mut future = std::pin::pin!(future);
    poll_fn(|cx| {
        assert!(future.as_mut().poll(cx).is_pending());
        Poll::Ready(())
    })
    .await;
}

// 응답을 요청과 연결하고 PING을 처리하면서 원본 응답을 보존한다.
#[tokio::test]
async fn correlation_ping_and_original_response_preserved() {
    let mock = Mock::with(vec![
        json!({"trnm":"LOGIN","return_code":0}),
        json!({"trnm":"PING","nonce":"same"}),
        json!({"trnm":"REAL","data":[]}),
        json!({"trnm":"CNSRREQ","seq":"2","return_code":0}),
        json!({"trnm":"CNSRREQ","seq":"1","return_code":0,"unknown":null,"data":[]}),
    ]);
    let sent = mock.sent.clone();
    let mut session = WsSession::new(mock, options()).unwrap();
    assert!(session.next_event().await.is_err());
    login(&mut session).await;
    let reply = session.call(&request()).await.unwrap();
    assert_eq!(reply.typed.seq.as_deref(), Some("1"));
    assert_eq!(reply.raw["unknown"], Value::Null);
    assert!(reply.raw.get("unknown").is_some());
    assert!(reply.acknowledged);
    assert_eq!(session.pop_pending().unwrap()["trnm"], "REAL");
    assert_eq!(session.pop_pending().unwrap()["seq"], "2");
    assert_eq!(
        sent.lock().unwrap()[2],
        json!({"trnm":"PING","nonce":"same"})
    );
    assert!(session.login(&Secret::new("other".into())).await.is_err());
    session.close().await.unwrap();
    assert!(!session.is_authenticated());
}

// 성공 코드가 없으면 응답을 명시적으로 미확정 상태로 남긴다.
#[tokio::test]
async fn missing_success_code_is_explicitly_unconfirmed() {
    let mock = Mock::with(vec![
        json!({"trnm":"LOGIN","return_code":0}),
        json!({"trnm":"CNSRREQ","seq":"1","data":[]}),
    ]);
    let mut s = WsSession::new(mock, options()).unwrap();
    login(&mut s).await;
    assert!(!s.call(&request()).await.unwrap().acknowledged);
}

// 잘못된 응답 코드와 버퍼 초과 이후 세션을 재사용하지 않는다.
#[tokio::test]
async fn bad_code_and_buffer_overflow_poison_session() {
    for frames in [
        vec![json!({"trnm":"CNSRREQ","return_code":"0"})],
        vec![
            json!({"trnm":"REAL","data":[]}),
            json!({"trnm":"REAL","data":[]}),
        ],
    ] {
        let mock = Mock::with([vec![json!({"trnm":"LOGIN","return_code":0})], frames].concat());
        let sent = mock.sent.clone();
        let mut s = WsSession::new(
            mock,
            SessionOptions {
                max_pending_events: 1,
                ..options()
            },
        )
        .unwrap();
        login(&mut s).await;
        assert!(s.call(&request()).await.is_err());
        assert!(s.requires_reconnect());
        assert!(!s.is_authenticated());
        assert!(s.pop_pending().is_none());
        assert!(s.call(&request()).await.is_err());
        assert!(s.login(&Secret::new("synthetic".into())).await.is_err());
        assert_eq!(sent.lock().unwrap().len(), 2);
    }
}

// 제어 프레임이 끼어들어도 전체 작업의 절대 마감 시간을 적용한다.
#[tokio::test]
async fn absolute_deadline_covers_interleaved_control_frames() {
    let mut mock = Mock::with(vec![json!({"trnm":"LOGIN","return_code":0})]);
    for _ in 0..10 {
        mock.received.push_back((
            Duration::from_millis(10),
            json!({"trnm":"PING"}).to_string(),
        ));
    }
    let mut s = WsSession::new(
        mock,
        SessionOptions {
            timeout: Duration::from_millis(25),
            ..options()
        },
    )
    .unwrap();
    login(&mut s).await;
    assert!(matches!(
        s.call(&request()).await,
        Err(ClientError::Transport)
    ));
    assert!(s.requires_reconnect());
}

// 외부 취소 이후에는 새 전송 연결이 필요하다.
#[tokio::test]
async fn external_cancellation_requires_new_transport() {
    let mut mock = Mock::with(vec![json!({"trnm":"LOGIN","return_code":0})]);
    mock.received.push_back((
        Duration::from_millis(50),
        json!({"trnm":"CNSRREQ","seq":"1","return_code":0}).to_string(),
    ));
    let mut s = WsSession::new(mock, options()).unwrap();
    login(&mut s).await;
    assert!(
        tokio::time::timeout(Duration::from_millis(5), s.call(&request()))
            .await
            .is_err()
    );
    assert!(s.requires_reconnect());
    assert!(s.next_event().await.is_err());
    assert!(s.call(&request()).await.is_err());
}

// 절대 마감 시간을 표현할 수 없는 설정은 전송을 시작하기 전에 거부한다.
#[tokio::test]
async fn overflowing_timeout_is_rejected_before_io() {
    let mock = Mock::default();
    let sent = mock.sent.clone();
    let received = mock.receive_count.clone();
    assert!(matches!(
        WsSession::new(
            mock,
            SessionOptions {
                timeout: Duration::MAX,
                ..options()
            }
        ),
        Err(ClientError::InvalidRequest(_))
    ));
    assert!(sent.lock().unwrap().is_empty());
    assert_eq!(received.load(Ordering::SeqCst), 0);
}

// LOGIN·이벤트 수신·종료를 처음 폴링한 뒤 취소하면 같은 연결로 재개하지 않는다.
#[tokio::test]
async fn cancelled_login_event_and_close_cannot_resume() {
    for operation in ["LOGIN", "event", "close"] {
        let mut mock = Mock::with(vec![
            json!({"trnm":"LOGIN","return_code":0}),
            json!({"trnm":"REAL","data":[]}),
        ]);
        mock.received[usize::from(operation != "LOGIN")].0 = Duration::from_secs(1);
        mock.close_delay = Duration::from_secs(1);
        let sent = mock.sent.clone();
        let received = mock.receive_count.clone();
        let closed = mock.close_count.clone();
        let mut session = WsSession::new(mock, options()).unwrap();
        if operation != "LOGIN" {
            login(&mut session).await;
        }
        let token = Secret::new("synthetic".into());
        match operation {
            "LOGIN" => cancel_after_first_poll(session.login(&token)).await,
            "event" => cancel_after_first_poll(session.next_event()).await,
            "close" => cancel_after_first_poll(session.close()).await,
            _ => unreachable!(),
        }
        assert!(session.requires_reconnect(), "{operation}");
        assert!(!session.is_authenticated(), "{operation}");
        let io_counts = (
            sent.lock().unwrap().len(),
            received.load(Ordering::SeqCst),
            closed.load(Ordering::SeqCst),
        );
        assert_eq!(io_counts.0, 1);
        assert_eq!(io_counts.1, if operation == "event" { 2 } else { 1 });
        assert_eq!(io_counts.2, usize::from(operation == "close"));
        assert!(session.pop_pending().is_none());
        assert!(session.login(&token).await.is_err());
        assert!(session.call(&request()).await.is_err());
        assert!(session.next_event().await.is_err());
        assert_eq!(
            io_counts,
            (
                sent.lock().unwrap().len(),
                received.load(Ordering::SeqCst),
                closed.load(Ordering::SeqCst),
            )
        );
    }
}

// 호출 Future를 폴링하지 않고 버리면 전송과 수신 없이 인증 상태를 보존한다.
#[tokio::test]
async fn dropping_unpolled_call_preserves_session_and_io() {
    let mock = Mock::with(vec![
        json!({"trnm":"LOGIN","return_code":0}),
        json!({"trnm":"CNSRREQ","seq":"1","return_code":0,"data":[]}),
    ]);
    let sent = mock.sent.clone();
    let received = mock.receive_count.clone();
    let mut session = WsSession::new(mock, options()).unwrap();
    login(&mut session).await;
    let request = request();
    drop(session.call(&request));
    assert!(session.is_authenticated());
    assert!(!session.requires_reconnect());
    assert_eq!(sent.lock().unwrap().len(), 1);
    assert_eq!(received.load(Ordering::SeqCst), 1);
    let reply = session.call(&request).await.unwrap();
    assert!(reply.acknowledged);
    assert_eq!(reply.typed.seq.as_deref(), Some("1"));
    assert_eq!(sent.lock().unwrap().len(), 2);
    assert_eq!(received.load(Ordering::SeqCst), 2);
}

// 요청 전송과 PING 반향의 지연·실패도 전체 기한과 세션 무효화 경계에 포함된다.
#[tokio::test]
async fn send_and_ping_echo_share_deadline_and_invalidate_on_failure() {
    for (ping, fail) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut mock = Mock::with(vec![json!({"trnm":"LOGIN","return_code":0})]);
        if ping {
            mock.received
                .push_back((Duration::ZERO, json!({"trnm":"REAL","data":[]}).to_string()));
            mock.received.push_back((
                if fail {
                    Duration::ZERO
                } else {
                    Duration::from_millis(30)
                },
                json!({"trnm":"PING","nonce":"echo"}).to_string(),
            ));
        }
        mock.received.push_back((
            if ping || fail {
                Duration::ZERO
            } else {
                Duration::from_millis(30)
            },
            json!({"trnm":"CNSRREQ","seq":"1","return_code":0,"data":[]}).to_string(),
        ));
        mock.send_steps.push_back((Duration::ZERO, false));
        if ping {
            mock.send_steps.push_back((Duration::ZERO, false));
        }
        mock.send_steps.push_back((
            if fail {
                Duration::ZERO
            } else {
                Duration::from_millis(30)
            },
            fail,
        ));
        let sent = mock.sent.clone();
        let mut session = WsSession::new(
            mock,
            SessionOptions {
                timeout: Duration::from_millis(50),
                ..options()
            },
        )
        .unwrap();
        login(&mut session).await;
        assert!(matches!(
            session.call(&request()).await,
            Err(ClientError::Transport)
        ));
        assert!(session.requires_reconnect());
        assert!(!session.is_authenticated());
        assert!(session.pop_pending().is_none());
        assert!(session.call(&request()).await.is_err());
        assert!(session.next_event().await.is_err());
        let sent = sent.lock().unwrap();
        assert_eq!(sent.len(), if ping { 3 } else { 2 });
        if ping {
            assert_eq!(sent[2], json!({"trnm":"PING","nonce":"echo"}));
        }
    }
}

// seq 생략 응답의 0 ACK를 허용하되 요청별 상관관계가 증명됐다고 간주하지 않는다.
#[tokio::test]
async fn omitted_sequence_with_zero_code_is_acknowledged_without_correlation() {
    let mock = Mock::with(vec![
        json!({"trnm":"LOGIN","return_code":0}),
        json!({"trnm":"CNSRREQ","return_code":0,"data":[],"marker":"unsequenced"}),
    ]);
    let mut session = WsSession::new(mock, options()).unwrap();
    login(&mut session).await;
    let reply = session.call(&request()).await.unwrap();
    assert!(reply.acknowledged);
    assert!(reply.typed.seq.is_none());
    assert!(reply.raw.get("seq").is_none());
    assert_eq!(reply.raw["marker"], "unsequenced");
    assert!(session.is_authenticated());
    assert!(session.pop_pending().is_none());
}

// 공유 세션의 동시 작업은 Mutex로 직렬화되고 잘못된·늦은 ACK는 도착 순서대로 보관된다.
#[tokio::test]
async fn concurrent_tasks_serialize_requests_and_preserve_unmatched_ack_fifo() {
    let mut mock = Mock::with(vec![
        json!({"trnm":"LOGIN","return_code":0}),
        json!({"trnm":"REAL","data":[],"marker":"first"}),
        json!({"trnm":"CNSRREQ","seq":"9","return_code":0,"marker":"wrong"}),
        json!({"trnm":"CNSRREQ","seq":"1","return_code":0,"data":[],"marker":"one"}),
        json!({"trnm":"CNSRREQ","seq":"1","return_code":0,"marker":"late"}),
        json!({"trnm":"CNSRREQ","seq":"2","return_code":0,"data":[],"marker":"two"}),
    ]);
    let gate = Arc::new(tokio::sync::Notify::new());
    mock.receive_gate = Some((1, gate.clone()));
    let sent = mock.sent.clone();
    let send_started = mock.send_started.clone();
    let mut session = WsSession::new(mock, options()).unwrap();
    login(&mut session).await;
    send_started.notified().await;
    let session = Arc::new(tokio::sync::Mutex::new(session));
    let first_session = session.clone();
    let first = tokio::spawn(async move { first_session.lock().await.call(&request()).await });
    send_started.notified().await;
    assert!(session.try_lock().is_err());
    let second_started = Arc::new(tokio::sync::Notify::new());
    let second_notice = second_started.clone();
    let second_session = session.clone();
    let second = tokio::spawn(async move {
        let mut request = request();
        request.seq = "2".into();
        second_notice.notify_one();
        second_session.lock().await.call(&request).await
    });
    second_started.notified().await;
    assert_eq!(sent.lock().unwrap().len(), 2);
    gate.notify_one();
    let (first, second) = tokio::time::timeout(Duration::from_secs(1), async {
        (
            first.await.unwrap().unwrap(),
            second.await.unwrap().unwrap(),
        )
    })
    .await
    .unwrap();
    assert_eq!(first.typed.seq.as_deref(), Some("1"));
    assert_eq!(first.raw["marker"], "one");
    assert_eq!(second.typed.seq.as_deref(), Some("2"));
    assert_eq!(second.raw["marker"], "two");
    assert!(first.acknowledged && second.acknowledged);
    {
        let wire = sent.lock().unwrap();
        assert_eq!(wire.len(), 3);
        assert_eq!(wire[0]["trnm"], "LOGIN");
        assert_eq!(wire[1]["seq"], "1");
        assert_eq!(wire[2]["seq"], "2");
    }
    let mut session = session.lock().await;
    for marker in ["first", "wrong", "late"] {
        assert_eq!(session.pop_pending().unwrap()["marker"], marker);
    }
    assert!(session.pop_pending().is_none());
    assert!(session.is_authenticated());
}
