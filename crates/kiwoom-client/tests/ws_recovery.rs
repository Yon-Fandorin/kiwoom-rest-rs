use async_trait::async_trait;
use kiwoom_client::{
    ClientError, Secret,
    ws::realtime::RealtimeType,
    ws::recovery::*,
    ws::session::{SessionOptions, WsTransport},
    ws::subscription::{
        RealtimeSubscription, RefreshMode, SubscriptionAction, SubscriptionEntry, UsExchange,
        UsInstrument,
    },
};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;

struct Mock {
    frames: VecDeque<(Duration, Value)>,
    sent: Arc<Mutex<Vec<Value>>>,
    dropped: Arc<AtomicUsize>,
    receive_gate: Option<(usize, Arc<Notify>)>,
    received: usize,
}

impl Drop for Mock {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}

#[async_trait]
impl WsTransport for Mock {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }
    async fn receive(&mut self) -> Result<String, ClientError> {
        self.received += 1;
        if let Some((at, entered)) = &self.receive_gate
            && self.received == *at
        {
            entered.notify_one();
            return std::future::pending().await;
        }
        let (delay, value) = self.frames.pop_front().ok_or(ClientError::Transport)?;
        tokio::time::sleep(delay).await;
        Ok(value.to_string())
    }
    async fn close(&mut self) -> Result<(), ClientError> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Ok(())
    }
}

struct Factory {
    transports: VecDeque<Result<Mock, ClientError>>,
    calls: Arc<AtomicUsize>,
}

#[async_trait]
impl WsTransportFactory for Factory {
    type Transport = Mock;
    async fn connect(&mut self) -> Result<Mock, ClientError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.transports
            .pop_front()
            .unwrap_or(Err(ClientError::Transport))
    }
}

struct Tokens {
    delay: Duration,
    calls: Arc<AtomicUsize>,
}

#[async_trait]
impl WsTokenProvider for Tokens {
    async fn token(&self) -> Result<Secret, ClientError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        tokio::time::sleep(self.delay).await;
        Ok(Secret::new("synthetic-token".into()))
    }
}

fn ack(service: &str) -> Value {
    json!({"trnm":service,"return_code":0})
}

fn mock(values: Vec<Value>) -> Mock {
    delayed(values.into_iter().map(|v| (Duration::ZERO, v)).collect())
}

fn delayed(frames: Vec<(Duration, Value)>) -> Mock {
    Mock {
        frames: frames.into(),
        sent: Arc::default(),
        dropped: Arc::default(),
        receive_gate: None,
        received: 0,
    }
}

fn options() -> RecoveryOptions {
    RecoveryOptions {
        max_attempts: 2,
        deadline: Duration::from_millis(150),
        backoff: Duration::ZERO,
        ..Default::default()
    }
}

fn recovery(transports: Vec<Mock>) -> WsRecovery<Factory, Tokens> {
    recovery_with_options(transports, options())
}

fn recovery_with_options(
    transports: Vec<Mock>,
    recovery_options: RecoveryOptions,
) -> WsRecovery<Factory, Tokens> {
    WsRecovery::new(
        Factory {
            transports: transports.into_iter().map(Ok).collect(),
            calls: Arc::default(),
        },
        Tokens {
            delay: Duration::ZERO,
            calls: Arc::default(),
        },
        SessionOptions {
            timeout: Duration::from_millis(50),
            ..Default::default()
        },
        recovery_options,
    )
    .unwrap()
}

fn subscription(
    action: SubscriptionAction,
    mode: RefreshMode,
    group: &str,
    symbol: &str,
) -> RealtimeSubscription {
    RealtimeSubscription::new(
        action,
        group,
        mode,
        vec![SubscriptionEntry::domestic(RealtimeType::StockExecution, Some(symbol)).unwrap()],
    )
    .unwrap()
}

fn reg(symbol: &str) -> RealtimeSubscription {
    subscription(
        SubscriptionAction::Register,
        RefreshMode::KeepGroup,
        "1",
        symbol,
    )
}

// 확정된 구독만 재등록하고 PING 처리와 연결 공백 기록을 유지한다.
#[tokio::test]
async fn confirmed_replay_ping_interleaving_and_gap_remain() {
    let first = mock(vec![
        ack("LOGIN"),
        json!({"trnm":"PING","nonce":"n"}),
        json!({"trnm":"REAL","data":[]}),
        ack("REG"),
    ]);
    let old_log = first.sent.clone();
    let old_dropped = first.dropped.clone();
    let second = mock(vec![
        ack("LOGIN"),
        json!({"trnm":"REAL","data":[]}),
        ack("REG"),
    ]);
    let replay_log = second.sent.clone();
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert_eq!(r.pop_pending().unwrap()["trnm"], "REAL");
    assert_eq!(old_log.lock().unwrap()[2]["trnm"], "PING");
    assert!(r.next_event().await.is_err());
    assert!(!r.status().connected);
    assert_eq!(old_dropped.load(Ordering::SeqCst), 1);
    r.recover().await.unwrap();
    let status = r.status();
    assert!(status.connected);
    assert_eq!(status.confirmed_generation, Some(2));
    assert_eq!(status.unresolved_gaps.len(), 1);
    assert!(!status.history_verified_complete);
    {
        let log = replay_log.lock().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0]["trnm"], "LOGIN");
        assert_eq!(log[1]["refresh"], "0");
    }
    assert_eq!(r.next_event().await.unwrap()["trnm"], "REAL");
    r.recover().await.unwrap(); // 이미 연결된 상태이므로 중복 재등록하지 않는다.
    assert_eq!(replay_log.lock().unwrap().len(), 2);
}

// 구독 교체와 제거는 확인 응답 이후에만 확정한다.
#[tokio::test]
async fn replacement_and_removal_commit_only_after_ack() {
    let first = mock(vec![
        ack("LOGIN"),
        ack("REG"),
        ack("REG"),
        ack("REG"),
        ack("REMOVE"),
    ]);
    let second = mock(vec![ack("LOGIN"), ack("REG")]);
    let log = second.sent.clone();
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert_eq!(r.status().confirmed_entries, 1);
    r.subscribe(&subscription(
        SubscriptionAction::Register,
        RefreshMode::ReplaceGroup,
        "1",
        "000660",
    ))
    .await
    .unwrap();
    r.subscribe(&subscription(
        SubscriptionAction::Remove,
        RefreshMode::KeepGroup,
        "1",
        "000660",
    ))
    .await
    .unwrap();
    assert_eq!(r.status().confirmed_groups, 0);
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    assert_eq!(log.lock().unwrap().len(), 1); // 확정된 구독이 없으므로 LOGIN만 전송한다.
}

// 추가나 제거가 불확실하면 마지막으로 확정된 구독 구성을 유지한다.
#[tokio::test]
async fn uncertain_addition_and_removal_keep_last_confirmed_configuration() {
    for action in [SubscriptionAction::Register, SubscriptionAction::Remove] {
        let first = mock(vec![
            ack("LOGIN"),
            ack("REG"),
            json!({"trnm":match action{SubscriptionAction::Register=>"REG",SubscriptionAction::Remove=>"REMOVE"}}),
        ]);
        let second = mock(vec![ack("LOGIN"), ack("REG")]);
        let log = second.sent.clone();
        let mut r = recovery(vec![first, second]);
        r.recover().await.unwrap();
        r.subscribe(&reg("005930")).await.unwrap();
        let symbol = match action {
            SubscriptionAction::Register => "000660",
            SubscriptionAction::Remove => "005930",
        };
        assert!(
            r.subscribe(&subscription(action, RefreshMode::KeepGroup, "1", symbol))
                .await
                .is_err()
        );
        assert_eq!(r.status().confirmed_entries, 1);
        assert!(!r.status().connected);
        r.recover().await.unwrap();
        assert_eq!(log.lock().unwrap()[1]["data"][0]["item"], "005930");
        assert_eq!(
            r.status().unresolved_gaps[0].reason,
            GapReason::SubscriptionUncertain
        );
    }
}

// 구독 취소는 연결을 폐기하고 미확정 상태를 재등록하지 않는다.
#[tokio::test]
async fn canceled_subscription_drops_transport_and_never_replays_speculative_state() {
    let first = delayed(vec![
        (Duration::ZERO, ack("LOGIN")),
        (Duration::ZERO, ack("REG")),
        (Duration::from_secs(1), ack("REG")),
    ]);
    let dropped = first.dropped.clone();
    let second = mock(vec![ack("LOGIN"), ack("REG")]);
    let log = second.sent.clone();
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(5), r.subscribe(&reg("000660")))
            .await
            .is_err()
    );
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert!(!r.status().connected);
    assert_eq!(r.status().confirmed_generation, None);
    assert_eq!(r.status().unresolved_gaps.len(), 1);
    r.recover().await.unwrap();
    assert_eq!(log.lock().unwrap()[1]["data"].as_array().unwrap().len(), 1);
    assert_eq!(log.lock().unwrap()[1]["data"][0]["item"], "005930");
    assert_eq!(r.status().unresolved_gaps.len(), 1);
}

// 수신 취소는 오래된 이벤트와 전송 연결을 함께 폐기한다.
#[tokio::test]
async fn canceled_receive_drops_stale_events_and_transport() {
    let first = delayed(vec![
        (Duration::ZERO, ack("LOGIN")),
        (Duration::from_secs(1), json!({"trnm":"REAL","old":true})),
    ]);
    let dropped = first.dropped.clone();
    let second = mock(vec![ack("LOGIN"), json!({"trnm":"REAL","new":true})]);
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(5), r.next_event())
            .await
            .is_err()
    );
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert!(r.pop_pending().is_none());
    r.recover().await.unwrap();
    let v = r.next_event().await.unwrap();
    assert_eq!(v["new"], true);
    assert!(v.get("old").is_none());
}

// 재등록 실패는 부분 세션을 폐기하고 다음 새 연결에서 한 번씩 재등록한다.
#[tokio::test]
async fn failed_replay_drops_partial_session_and_replays_once_on_next_new_connection() {
    let first = mock(vec![ack("LOGIN"), ack("REG")]);
    let second = mock(vec![ack("LOGIN")]);
    let second_drop = second.dropped.clone();
    let third = mock(vec![ack("LOGIN"), ack("REG")]);
    let log = third.sent.clone();
    let mut r = recovery(vec![first, second, third]);
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    assert_eq!(second_drop.load(Ordering::SeqCst), 1);
    assert_eq!(log.lock().unwrap().len(), 2);
    assert_eq!(r.status().confirmed_generation, Some(3));
}

// 재등록의 업무 거부와 확인 응답 누락은 재시도하거나 확정하지 않는다.
#[tokio::test]
async fn replay_business_refusal_and_missing_ack_do_not_retry_or_confirm() {
    for reply in [json!({"trnm":"REG","return_code":7}), json!({"trnm":"REG"})] {
        let first = mock(vec![ack("LOGIN"), ack("REG")]);
        let second = mock(vec![ack("LOGIN"), reply]);
        let unused = mock(vec![ack("LOGIN"), ack("REG")]);
        let unused_log = unused.sent.clone();
        let mut r = recovery(vec![first, second, unused]);
        r.recover().await.unwrap();
        r.subscribe(&reg("005930")).await.unwrap();
        assert!(r.next_event().await.is_err());
        assert!(r.recover().await.is_err());
        assert!(!r.status().connected);
        assert_eq!(r.status().confirmed_generation, None);
        assert!(unused_log.lock().unwrap().is_empty());
    }
}

// 연결 생성, 토큰 공급, 재시도 대기에 시도 횟수와 전체 마감 시간을 적용한다.
#[tokio::test]
async fn retries_and_total_deadline_bound_factory_token_and_backoff() {
    let calls = Arc::new(AtomicUsize::new(0));
    let tokens = Arc::new(AtomicUsize::new(0));
    let mut r = WsRecovery::new(
        Factory {
            transports: VecDeque::new(),
            calls: calls.clone(),
        },
        Tokens {
            delay: Duration::ZERO,
            calls: tokens.clone(),
        },
        SessionOptions::default(),
        options(),
    )
    .unwrap();
    assert!(r.recover().await.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(tokens.load(Ordering::SeqCst), 2);
    let opts = RecoveryOptions {
        deadline: Duration::from_millis(10),
        backoff: Duration::from_secs(1),
        ..options()
    };
    let mut r = WsRecovery::new(
        Factory {
            transports: VecDeque::new(),
            calls: calls.clone(),
        },
        Tokens {
            delay: Duration::from_secs(1),
            calls: tokens.clone(),
        },
        SessionOptions::default(),
        opts,
    )
    .unwrap();
    let started = tokio::time::Instant::now();
    assert!(r.recover().await.is_err());
    assert!(started.elapsed() < Duration::from_millis(100));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

// 복구와 종료가 취소되면 사용할 수 있는 연결로 남기지 않는다.
#[tokio::test]
async fn canceled_recover_and_shutdown_are_fail_closed() {
    let first = delayed(vec![(Duration::from_secs(1), ack("LOGIN"))]);
    let dropped = first.dropped.clone();
    let second = mock(vec![ack("LOGIN")]);
    let mut r = recovery(vec![first, second]);
    assert!(
        tokio::time::timeout(Duration::from_millis(5), r.recover())
            .await
            .is_err()
    );
    assert!(!r.status().connected);
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    r.recover().await.unwrap();
    assert_eq!(r.status().unresolved_gaps.len(), 1);
    assert!(
        tokio::time::timeout(Duration::from_millis(1), r.shutdown())
            .await
            .is_err()
    );
    assert!(r.status().shutdown);
    assert!(!r.status().connected);
    assert!(r.recover().await.is_err());
    assert!(r.subscribe(&reg("005930")).await.is_err());
    r.shutdown().await.unwrap();
}

// 미국 구독은 항목 형태를 섞지 않고 객체 단위로 한꺼번에 제거한다.
#[tokio::test]
async fn us_objects_remove_atomically_without_mixing_entry_shapes() {
    let first = mock(vec![ack("LOGIN"), ack("REG"), ack("REG"), ack("REMOVE")]);
    let second = mock(vec![ack("LOGIN"), ack("REG")]);
    let log = second.sent.clone();
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    let entry = SubscriptionEntry::us_market(
        RealtimeType::UsTradePrice,
        vec![
            UsInstrument::new("AAA", UsExchange::Nasdaq).unwrap(),
            UsInstrument::new("BBB", UsExchange::Nyse).unwrap(),
        ],
    )
    .unwrap();
    let req = RealtimeSubscription::new(
        SubscriptionAction::Register,
        "1",
        RefreshMode::KeepGroup,
        vec![entry],
    )
    .unwrap();
    r.subscribe(&req).await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    let remove = RealtimeSubscription::new(
        SubscriptionAction::Remove,
        "1",
        RefreshMode::KeepGroup,
        vec![
            SubscriptionEntry::us_market(
                RealtimeType::UsTradePrice,
                vec![UsInstrument::new("AAA", UsExchange::Nasdaq).unwrap()],
            )
            .unwrap(),
        ],
    )
    .unwrap();
    r.subscribe(&remove).await.unwrap();
    assert_eq!(r.status().confirmed_entries, 2);
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    let log = log.lock().unwrap();
    let data = log[1]["data"].as_array().unwrap();
    assert_eq!(data.len(), 2);
    assert_eq!(data[0]["item"][0]["jmcode"], "BBB");
    assert_eq!(data[1]["item"], "005930");
}

// 알 수 없는 구독 제거, 용량 초과, 공백 기록 한도는 입출력 전에 거부한다.
#[tokio::test]
async fn unknown_removal_capacity_and_gap_audit_reject_before_io() {
    let first = mock(vec![ack("LOGIN"), ack("REG")]);
    let log = first.sent.clone();
    let mut r = recovery(vec![first]);
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert!(
        r.subscribe(&subscription(
            SubscriptionAction::Remove,
            RefreshMode::KeepGroup,
            "1",
            "000660"
        ))
        .await
        .is_err()
    );
    assert!(
        r.subscribe(&subscription(
            SubscriptionAction::Remove,
            RefreshMode::ReplaceGroup,
            "1",
            "005930"
        ))
        .await
        .is_err()
    );
    assert!(r.status().connected);
    assert_eq!(log.lock().unwrap().len(), 2);
    let opts = RecoveryOptions {
        max_entries: 1,
        max_gap_markers: 1,
        ..options()
    };
    let first = mock(vec![ack("LOGIN"), ack("REG")]);
    let mut r = WsRecovery::new(
        Factory {
            transports: vec![Ok(first)].into(),
            calls: Arc::default(),
        },
        Tokens {
            delay: Duration::ZERO,
            calls: Arc::default(),
        },
        SessionOptions::default(),
        opts,
    )
    .unwrap();
    r.recover().await.unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    assert!(r.subscribe(&reg("000660")).await.is_err());
    assert!(r.next_event().await.is_err());
    assert!(r.recover().await.is_err());
    assert_eq!(r.status().unresolved_gaps.len(), 1);
}

// 그룹이 다른 확인 응답으로는 구성을 확정하거나 재등록하지 않는다.
#[tokio::test]
async fn wrong_group_ack_cannot_promote_or_replay_configuration() {
    for during_replay in [false, true] {
        let wrong = json!({"trnm":"REG","return_code":0,"grp_no":"999"});
        let first = mock(vec![
            ack("LOGIN"),
            if during_replay {
                ack("REG")
            } else {
                wrong.clone()
            },
        ]);
        let second = mock(vec![ack("LOGIN"), wrong]);
        let mut r = recovery(vec![first, second]);
        r.recover().await.unwrap();
        if during_replay {
            r.subscribe(&reg("005930")).await.unwrap();
            assert!(r.next_event().await.is_err());
            assert!(r.recover().await.is_err());
        } else {
            assert!(r.subscribe(&reg("005930")).await.is_err());
            assert_eq!(r.status().confirmed_groups, 0);
        }
        assert!(!r.status().connected);
        assert_eq!(r.status().confirmed_generation, None);
    }
}

// 세션 시간 초과는 새 연결과 로그인으로 재시도하고 매 공백을 기록한다.
#[tokio::test]
async fn session_timeout_retries_new_factory_with_fresh_login_and_records_each_gap() {
    let first = delayed(vec![(Duration::from_secs(1), ack("LOGIN"))]);
    let dropped = first.dropped.clone();
    let second = mock(vec![ack("LOGIN")]);
    let log = second.sent.clone();
    let mut r = recovery(vec![first, second]);
    r.recover().await.unwrap();
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert_eq!(log.lock().unwrap().len(), 1);
    assert_eq!(log.lock().unwrap()[0]["trnm"], "LOGIN");
    assert_eq!(r.status().unresolved_gaps.len(), 1);
    assert_eq!(r.status().unresolved_gaps[0].generation, 1);
}

struct SlowFactory;

#[async_trait]
impl WsTransportFactory for SlowFactory {
    type Transport = Mock;
    async fn connect(&mut self) -> Result<Mock, ClientError> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(mock(vec![ack("LOGIN")]))
    }
}

// 연결 생성과 재시도 대기도 절대 마감 시간 안에 포함한다.
#[tokio::test]
async fn factory_and_backoff_are_inside_absolute_deadline() {
    let opts = RecoveryOptions {
        deadline: Duration::from_millis(10),
        ..options()
    };
    let mut r = WsRecovery::new(
        SlowFactory,
        Tokens {
            delay: Duration::ZERO,
            calls: Arc::default(),
        },
        SessionOptions::default(),
        opts,
    )
    .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(100), r.recover())
            .await
            .unwrap()
            .is_err()
    );
    assert!(!r.status().connected);
    let calls = Arc::new(AtomicUsize::new(0));
    let opts = RecoveryOptions {
        deadline: Duration::from_millis(10),
        backoff: Duration::from_secs(1),
        ..options()
    };
    let mut r = WsRecovery::new(
        Factory {
            transports: VecDeque::new(),
            calls: calls.clone(),
        },
        Tokens {
            delay: Duration::ZERO,
            calls: Arc::default(),
        },
        SessionOptions::default(),
        opts,
    )
    .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(100), r.recover())
            .await
            .unwrap()
            .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(r.status().unresolved_gaps.len(), 2);
}

fn domestic_condition(seq: &str) -> kiwoom_client::ws::conditions::Ka10173Request {
    serde_json::from_value(json!({"trnm":"CNSRREQ","seq":seq,"search_type":"1","stex_tp":"K"}))
        .unwrap()
}

fn us_condition(seq: &str) -> kiwoom_client::ws::conditions::Usa20290Request {
    serde_json::from_value(json!({"trnm":"GCNSRREQ","seq":seq,"search_type":"1"})).unwrap()
}

fn domestic_clear(seq: &str) -> kiwoom_client::ws::conditions::Ka10174Request {
    serde_json::from_value(json!({"trnm":"CNSRCLR","seq":seq})).unwrap()
}

fn us_clear(seq: &str) -> kiwoom_client::ws::conditions::Usa20291Request {
    serde_json::from_value(json!({"trnm":"GCNSRCLR","seq":seq})).unwrap()
}

fn condition_ack(service: &str, seq: &str) -> Value {
    json!({"trnm":service,"return_code":0,"return_msg":"synthetic","seq":seq,"data":[]})
}

// 국내와 미국 연속 조건검색은 새 스냅샷으로 재등록하고 정확한 키만 해제한다.
#[tokio::test]
async fn continuous_domestic_and_us_conditions_replay_fresh_snapshots_and_clear_exact_keys() {
    let first = mock(vec![
        ack("LOGIN"),
        condition_ack("CNSRREQ", "1"),
        condition_ack("GCNSRREQ", "1"),
    ]);
    let second = mock(vec![
        ack("LOGIN"),
        json!({"trnm":"REAL","fresh":true}),
        json!({"trnm":"CNSRREQ","return_code":0,"seq":"1","data":[{"jmcode":"synthetic-domestic"}],"future":"raw-preserved"}),
        json!({"trnm":"GCNSRREQ","return_code":0,"seq":"1","data":[{"jmcode":"synthetic-us","stexTp":"ND"}]}),
        condition_ack("CNSRCLR", "1"),
        condition_ack("GCNSRCLR", "1"),
    ]);
    let log = second.sent.clone();
    let third = mock(vec![ack("LOGIN")]);
    let third_log = third.sent.clone();
    let mut r = recovery(vec![first, second, third]);
    r.recover().await.unwrap();
    r.subscribe_domestic_condition(&domestic_condition("1"))
        .await
        .unwrap();
    r.subscribe_us_condition(&us_condition("1")).await.unwrap();
    assert_eq!(r.status().confirmed_conditions, 2);
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    assert_eq!(r.condition_replay_snapshots().len(), 2);
    assert_eq!(r.condition_replay_snapshots()[0].generation, 2);
    assert_eq!(r.condition_replay_snapshots()[0].api_id, "ka10173");
    assert_eq!(
        r.condition_replay_snapshots()[0].raw["future"],
        "raw-preserved"
    );
    assert_eq!(r.condition_replay_snapshots()[1].api_id, "usa20290");
    assert_eq!(r.pop_pending().unwrap()["fresh"], true);
    {
        let log = log.lock().unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log[1]["search_type"], "1");
        assert_eq!(log[2]["search_type"], "1");
    }
    r.clear_domestic_condition(&domestic_clear("1"))
        .await
        .unwrap();
    assert_eq!(r.status().confirmed_conditions, 1);
    r.clear_us_condition(&us_clear("1")).await.unwrap();
    assert_eq!(r.status().confirmed_conditions, 0);
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    assert_eq!(third_log.lock().unwrap().len(), 1);
    assert!(r.condition_replay_snapshots().is_empty());
    assert!(!r.status().history_verified_complete);
    assert_eq!(r.status().unresolved_gaps.len(), 2);
}

// 조건 확인 응답 누락과 해제 취소는 마지막 확정 상태를 유지한다.
#[tokio::test]
async fn missing_condition_ack_and_canceled_clear_keep_last_confirmed_state() {
    let first = delayed(vec![
        (Duration::ZERO, ack("LOGIN")),
        (Duration::ZERO, condition_ack("CNSRREQ", "1")),
        (Duration::from_secs(1), condition_ack("CNSRCLR", "1")),
    ]);
    let second = mock(vec![
        ack("LOGIN"),
        condition_ack("CNSRREQ", "1"),
        json!({"trnm":"GCNSRREQ","seq":"2"}),
    ]);
    let third = mock(vec![ack("LOGIN"), condition_ack("CNSRREQ", "1")]);
    let log = third.sent.clone();
    let mut r = recovery(vec![first, second, third]);
    r.recover().await.unwrap();
    r.subscribe_domestic_condition(&domestic_condition("1"))
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(
            Duration::from_millis(5),
            r.clear_domestic_condition(&domestic_clear("1"))
        )
        .await
        .is_err()
    );
    assert_eq!(r.status().confirmed_conditions, 1);
    assert!(!r.status().connected);
    r.recover().await.unwrap();
    assert!(r.subscribe_us_condition(&us_condition("2")).await.is_err());
    assert_eq!(r.status().confirmed_conditions, 1);
    r.recover().await.unwrap();
    assert_eq!(log.lock().unwrap().len(), 2);
    assert_eq!(log.lock().unwrap()[1]["seq"], "1");
    assert_eq!(r.status().unresolved_gaps.len(), 2);
}

// 조건 재등록 거부와 시퀀스 불일치는 확정하거나 부분 스냅샷을 노출하지 않는다.
#[tokio::test]
async fn condition_replay_refusal_and_wrong_sequence_never_confirm_or_expose_partial_snapshots() {
    for reply in [
        json!({"trnm":"CNSRREQ","seq":"1","return_code":8}),
        json!({"trnm":"CNSRREQ","seq":9,"return_code":0}),
    ] {
        let first = mock(vec![ack("LOGIN"), condition_ack("CNSRREQ", "1")]);
        let second = mock(vec![ack("LOGIN"), reply]);
        let mut r = recovery(vec![first, second]);
        r.recover().await.unwrap();
        r.subscribe_domestic_condition(&domestic_condition("1"))
            .await
            .unwrap();
        assert!(r.next_event().await.is_err());
        assert!(r.recover().await.is_err());
        assert!(!r.status().connected);
        assert_eq!(r.status().confirmed_generation, None);
        assert!(r.condition_replay_snapshots().is_empty());
        assert_eq!(r.status().confirmed_conditions, 1);
    }
}

// 추가 취소, 잘못된 모드·거래소, 알 수 없는 해제는 상태를 바꾸지 않는다.
#[tokio::test]
async fn condition_cancel_add_invalid_mode_exchange_and_unknown_clear_do_not_change_state() {
    let first = delayed(vec![
        (Duration::ZERO, ack("LOGIN")),
        (Duration::ZERO, condition_ack("CNSRREQ", "1")),
        (Duration::from_secs(1), condition_ack("GCNSRREQ", "1")),
    ]);
    let log = first.sent.clone();
    let mut r = recovery(vec![first]);
    r.recover().await.unwrap();
    r.subscribe_domestic_condition(&domestic_condition("1"))
        .await
        .unwrap();
    let mut changed = domestic_condition("1");
    changed.stex_tp = "A".into();
    assert!(r.subscribe_domestic_condition(&changed).await.is_err());
    let mut invalid = us_condition("1");
    invalid.search_type = "0".into();
    assert!(r.subscribe_us_condition(&invalid).await.is_err());
    assert!(r.clear_us_condition(&us_clear("1")).await.is_err());
    assert_eq!(log.lock().unwrap().len(), 2);
    assert!(r.status().connected);
    assert!(
        tokio::time::timeout(
            Duration::from_millis(5),
            r.subscribe_us_condition(&us_condition("1"))
        )
        .await
        .is_err()
    );
    assert_eq!(r.status().confirmed_conditions, 1);
    assert!(!r.status().connected);
}

// 실제 연결 팩토리의 생성은 오프라인에서 한도를 검증한다.
#[test]
fn concrete_factory_construction_is_offline_and_validates_limits() {
    use kiwoom_client::{transport::Environment, ws::transport::TungsteniteFactory};
    assert!(
        TungsteniteFactory::with_explicit_network_opt_in(
            Environment::Sandbox,
            1024,
            Duration::from_secs(1)
        )
        .is_ok()
    );
    assert!(
        TungsteniteFactory::with_explicit_network_opt_in(
            Environment::Production,
            0,
            Duration::from_secs(1)
        )
        .is_err()
    );
    assert!(
        TungsteniteFactory::with_explicit_network_opt_in(
            Environment::Production,
            1024,
            Duration::ZERO
        )
        .is_err()
    );
}

// 최초 등록은 역순으로 진행하여 복구의 그룹·시장 정렬 순서도 확인한다.
fn multi_configuration_initial_frames() -> Vec<Value> {
    vec![
        ack("LOGIN"),
        json!({"trnm":"REG","return_code":0,"grp_no":"2"}),
        json!({"trnm":"REG","return_code":0,"grp_no":"1"}),
        condition_ack("GCNSRREQ", "1"),
        condition_ack("CNSRREQ", "1"),
    ]
}

fn multi_configuration_replay_frames() -> Vec<Value> {
    vec![
        ack("LOGIN"),
        json!({"trnm":"REG","return_code":0,"grp_no":"1"}),
        json!({"trnm":"REG","return_code":0,"grp_no":"2"}),
        json!({"trnm":"CNSRREQ","return_code":0,"seq":"1","data":[],"attempt":"fresh-domestic"}),
        json!({"trnm":"GCNSRREQ","return_code":0,"seq":"1","data":[],"attempt":"fresh-us"}),
    ]
}

async fn confirm_multi_configuration(r: &mut WsRecovery<Factory, Tokens>) {
    r.recover().await.unwrap();
    r.subscribe(&subscription(
        SubscriptionAction::Register,
        RefreshMode::KeepGroup,
        "2",
        "000660",
    ))
    .await
    .unwrap();
    r.subscribe(&reg("005930")).await.unwrap();
    r.subscribe_us_condition(&us_condition("1")).await.unwrap();
    r.subscribe_domestic_condition(&domestic_condition("1"))
        .await
        .unwrap();
    assert_eq!(r.status().confirmed_generation, Some(1));
}

fn expected_multi_replay() -> Vec<Value> {
    vec![
        json!({"trnm":"LOGIN","token":"synthetic-token"}),
        serde_json::to_value(subscription(
            SubscriptionAction::Register,
            RefreshMode::ReplaceGroup,
            "1",
            "005930",
        ))
        .unwrap(),
        serde_json::to_value(subscription(
            SubscriptionAction::Register,
            RefreshMode::ReplaceGroup,
            "2",
            "000660",
        ))
        .unwrap(),
        serde_json::to_value(domestic_condition("1")).unwrap(),
        serde_json::to_value(us_condition("1")).unwrap(),
    ]
}

fn assert_multi_configuration_unpublished(r: &WsRecovery<Factory, Tokens>) {
    let status = r.status();
    assert!(!status.connected);
    assert_eq!(status.confirmed_generation, None);
    assert_eq!(status.confirmed_groups, 2);
    assert_eq!(status.confirmed_entries, 2);
    assert_eq!(status.confirmed_conditions, 2);
    assert!(!status.history_verified_complete);
    assert!(r.condition_replay_snapshots().is_empty());
}

fn assert_fresh_multi_snapshots(r: &WsRecovery<Factory, Tokens>, generation: u64) {
    assert!(r.status().connected);
    assert_eq!(r.status().confirmed_generation, Some(generation));
    let snapshots = r.condition_replay_snapshots();
    assert_eq!(snapshots.len(), 2);
    assert_eq!(snapshots[0].generation, generation);
    assert_eq!(snapshots[0].api_id, "ka10173");
    assert_eq!(snapshots[0].raw["attempt"], "fresh-domestic");
    assert_eq!(snapshots[1].generation, generation);
    assert_eq!(snapshots[1].api_id, "usa20290");
    assert_eq!(snapshots[1].raw["attempt"], "fresh-us");
}

// 첫 그룹 ACK 뒤의 거부·그룹 불일치·전송 예산 소진은 모든 확정 설정을 보존한다.
#[tokio::test]
async fn later_group_failure_discards_successful_subset_and_requires_fresh_full_replay() {
    for (reply, attempts) in [
        (Some(json!({"trnm":"REG","return_code":7})), 1),
        (
            Some(json!({"trnm":"REG","return_code":0,"grp_no":"999"})),
            1,
        ),
        (None, 2),
    ] {
        let first = mock(multi_configuration_initial_frames());
        let initial_log = first.sent.clone();
        let initial_drop = first.dropped.clone();
        let mut transports = vec![first];
        let mut partial_logs = Vec::new();
        let mut partial_drops = Vec::new();
        for _ in 0..attempts {
            let mut frames = multi_configuration_replay_frames()[..2].to_vec();
            if let Some(reply) = &reply {
                frames.push(reply.clone());
            }
            let failed = mock(frames);
            partial_logs.push(failed.sent.clone());
            partial_drops.push(failed.dropped.clone());
            transports.push(failed);
        }
        let fresh = mock(multi_configuration_replay_frames());
        let fresh_log = fresh.sent.clone();
        transports.push(fresh);
        let mut r = recovery(transports);
        confirm_multi_configuration(&mut r).await;
        assert_eq!(initial_log.lock().unwrap().len(), 5);
        assert!(r.next_event().await.is_err());
        assert_eq!(initial_drop.load(Ordering::SeqCst), 1);
        let error = r.recover().await.unwrap_err();
        match reply {
            Some(value) if value["return_code"] == 7 => {
                assert!(matches!(error, ClientError::Api(7)));
            }
            Some(_) => assert!(matches!(error, ClientError::Protocol(_))),
            None => assert!(matches!(error, ClientError::Transport)),
        }
        assert_multi_configuration_unpublished(&r);
        assert!(r.pop_pending().is_none());
        assert!(fresh_log.lock().unwrap().is_empty());
        for (log, dropped) in partial_logs.iter().zip(&partial_drops) {
            // 두 번째 REG 전송은 첫 번째 REG의 ACK 검증이 끝났음을 증명한다.
            assert_eq!(*log.lock().unwrap(), expected_multi_replay()[..3]);
            assert_eq!(dropped.load(Ordering::SeqCst), 1);
        }
        let gaps = r.status().unresolved_gaps;
        assert_eq!(gaps.len(), attempts + 1);
        assert_eq!(
            gaps[0],
            RecoveryGap {
                generation: 1,
                reason: GapReason::ReceiveUncertain
            }
        );
        for (index, gap) in gaps[1..].iter().enumerate() {
            assert_eq!(gap.generation, index as u64 + 2);
            assert_eq!(gap.reason, GapReason::Recovery);
        }
        r.recover().await.unwrap();
        assert_eq!(*fresh_log.lock().unwrap(), expected_multi_replay());
        assert_fresh_multi_snapshots(&r, attempts as u64 + 2);
        assert_eq!(r.status().unresolved_gaps, gaps);
        r.recover().await.unwrap();
        assert_eq!(fresh_log.lock().unwrap().len(), 5);
    }
}

// 두 그룹과 첫 조건 스냅샷이 성공해도 뒤 조건이 실패하면 이벤트·스냅샷을 공개하지 않는다.
#[tokio::test]
async fn later_condition_failure_discards_earlier_snapshot_and_preserves_confirmed_configuration() {
    for reply in [
        json!({"trnm":"GCNSRREQ","seq":"1","return_code":8}),
        json!({"trnm":"GCNSRREQ","seq":"1"}),
    ] {
        let first = mock(multi_configuration_initial_frames());
        let mut frames = multi_configuration_replay_frames()[..3].to_vec();
        frames.push(json!({"trnm":"REAL","attempt":"partial-event"}));
        frames.push(json!({"trnm":"CNSRREQ","return_code":0,"seq":"1","data":[],"attempt":"partial-domestic"}));
        frames.push(reply.clone());
        let failed = mock(frames);
        let failed_log = failed.sent.clone();
        let failed_drop = failed.dropped.clone();
        let fresh = mock(multi_configuration_replay_frames());
        let fresh_log = fresh.sent.clone();
        let mut r = recovery(vec![first, failed, fresh]);
        confirm_multi_configuration(&mut r).await;
        assert!(r.next_event().await.is_err());
        let error = r.recover().await.unwrap_err();
        if reply.get("return_code").is_some() {
            assert!(matches!(error, ClientError::Api(8)));
        } else {
            assert!(matches!(error, ClientError::Protocol(_)));
        }
        assert_eq!(*failed_log.lock().unwrap(), expected_multi_replay());
        assert_eq!(failed_drop.load(Ordering::SeqCst), 1);
        assert_multi_configuration_unpublished(&r);
        assert!(r.pop_pending().is_none());
        assert!(fresh_log.lock().unwrap().is_empty());
        let gaps = r.status().unresolved_gaps;
        assert_eq!(
            gaps,
            vec![
                RecoveryGap {
                    generation: 1,
                    reason: GapReason::ReceiveUncertain
                },
                RecoveryGap {
                    generation: 2,
                    reason: GapReason::Recovery
                },
            ]
        );
        r.recover().await.unwrap();
        assert_eq!(*fresh_log.lock().unwrap(), expected_multi_replay());
        assert_fresh_multi_snapshots(&r, 3);
        assert!(r.pop_pending().is_none());
        assert_eq!(r.status().unresolved_gaps, gaps);
    }
}

// 지정한 수신에 도달한 알림으로 첫 그룹 ACK·첫 조건 스냅샷 뒤의 복구를 결정적으로 취소한다.
#[tokio::test]
async fn canceled_recovery_after_ack_or_snapshot_keeps_pending_gap_and_requires_fresh_connection() {
    for receive_at in [3, 5] {
        let first = mock(multi_configuration_initial_frames());
        let entered = Arc::new(Notify::new());
        let mut partial = mock(multi_configuration_replay_frames());
        partial.receive_gate = Some((receive_at, entered.clone()));
        let partial_log = partial.sent.clone();
        let partial_drop = partial.dropped.clone();
        let fresh = mock(multi_configuration_replay_frames());
        let fresh_log = fresh.sent.clone();
        let mut r = recovery(vec![first, partial, fresh]);
        confirm_multi_configuration(&mut r).await;
        assert!(r.next_event().await.is_err());
        {
            let recovering = r.recover();
            tokio::pin!(recovering);
            tokio::select! {
                result = &mut recovering => panic!("수신 게이트 전에 복구가 끝남: {result:?}"),
                result = tokio::time::timeout(Duration::from_secs(1), entered.notified()) => {
                    result.expect("복구가 지정한 수신 게이트에 도달해야 함");
                }
            }
        }
        // 다음 요청이 전송되었으므로 앞선 ACK와 조건 결과는 이미 처리되었다.
        assert_eq!(
            *partial_log.lock().unwrap(),
            expected_multi_replay()[..receive_at]
        );
        assert_eq!(partial_drop.load(Ordering::SeqCst), 1);
        assert_multi_configuration_unpublished(&r);
        assert!(r.pop_pending().is_none());
        assert!(fresh_log.lock().unwrap().is_empty());
        let gaps = r.status().unresolved_gaps;
        assert_eq!(
            gaps,
            vec![
                RecoveryGap {
                    generation: 1,
                    reason: GapReason::ReceiveUncertain
                },
                RecoveryGap {
                    generation: 2,
                    reason: GapReason::Recovery
                },
            ]
        );
        assert!(matches!(
            r.next_event().await,
            Err(ClientError::Protocol(_))
        ));
        assert_eq!(r.status().unresolved_gaps, gaps);
        r.recover().await.unwrap();
        assert_eq!(*fresh_log.lock().unwrap(), expected_multi_replay());
        assert_fresh_multi_snapshots(&r, 3);
        assert_eq!(r.status().unresolved_gaps, gaps);
    }
}

// seq 생략 ACK는 단일 소유 세션의 서비스 일치만 확인하며 정확한 순번 상관을 증명하지 않는다.
#[tokio::test]
async fn omitted_condition_sequence_ack_is_accepted_with_only_service_correlation() {
    let domestic = json!({"trnm":"CNSRREQ","return_code":0,"data":[],"future":"without-seq"});
    let us = json!({"trnm":"GCNSRREQ","return_code":0,"data":[]});
    let first = mock(vec![ack("LOGIN"), domestic.clone(), us.clone()]);
    let fresh = mock(vec![ack("LOGIN"), domestic.clone(), us.clone()]);
    let log = fresh.sent.clone();
    let mut r = recovery(vec![first, fresh]);
    r.recover().await.unwrap();
    let reply = r
        .subscribe_domestic_condition(&domestic_condition("1"))
        .await
        .unwrap();
    assert!(reply.acknowledged);
    assert_eq!(reply.raw, domestic);
    assert!(reply.raw.get("seq").is_none());
    let reply = r.subscribe_us_condition(&us_condition("2")).await.unwrap();
    assert!(reply.acknowledged);
    assert_eq!(reply.raw, us);
    assert!(reply.raw.get("seq").is_none());
    assert_eq!(r.status().confirmed_conditions, 2);
    assert!(r.next_event().await.is_err());
    r.recover().await.unwrap();
    assert_eq!(
        *log.lock().unwrap(),
        vec![
            json!({"trnm":"LOGIN","token":"synthetic-token"}),
            serde_json::to_value(domestic_condition("1")).unwrap(),
            serde_json::to_value(us_condition("2")).unwrap(),
        ]
    );
    let snapshots = r.condition_replay_snapshots();
    assert_eq!(snapshots.len(), 2);
    assert_eq!(snapshots[0].raw, domestic);
    assert_eq!(snapshots[1].raw, us);
    assert!(
        snapshots
            .iter()
            .all(|snapshot| snapshot.generation == 2 && snapshot.raw.get("seq").is_none())
    );
    assert_eq!(r.status().confirmed_generation, Some(2));
    assert!(!r.status().history_verified_complete);
}

// 그룹 한도와 실시간·조건의 합산 한도는 양쪽 추가 경로 모두 전송·상태 변경 전에 검증한다.
#[tokio::test]
async fn group_and_combined_feed_condition_capacity_reject_before_io() {
    for (max_groups, max_entries) in [(1, 4), (2, 2)] {
        let first = mock(vec![
            ack("LOGIN"),
            ack("REG"),
            condition_ack("CNSRREQ", "1"),
            condition_ack("CNSRREQ", "1"),
        ]);
        let log = first.sent.clone();
        let mut r = recovery_with_options(
            vec![first],
            RecoveryOptions {
                max_groups,
                max_entries,
                ..options()
            },
        );
        r.recover().await.unwrap();
        r.subscribe(&reg("005930")).await.unwrap();
        r.subscribe_domestic_condition(&domestic_condition("1"))
            .await
            .unwrap();
        assert!(matches!(
            r.subscribe(&subscription(
                SubscriptionAction::Register,
                RefreshMode::KeepGroup,
                "2",
                "000660"
            ))
            .await,
            Err(ClientError::InvalidRequest(_))
        ));
        assert_eq!(log.lock().unwrap().len(), 3);
        if max_entries == 2 {
            // 같은 그룹에 더해도 조건 한 항목을 포함한 합산 용량을 초과한다.
            assert!(matches!(
                r.subscribe(&reg("000660")).await,
                Err(ClientError::InvalidRequest(_))
            ));
            assert!(matches!(
                r.subscribe_us_condition(&us_condition("2")).await,
                Err(ClientError::InvalidRequest(_))
            ));
            assert_eq!(log.lock().unwrap().len(), 3);
        }
        let status = r.status();
        assert!(status.connected);
        assert_eq!(status.confirmed_generation, Some(1));
        assert_eq!(status.confirmed_groups, 1);
        assert_eq!(status.confirmed_entries, 1);
        assert_eq!(status.confirmed_conditions, 1);
        assert!(status.unresolved_gaps.is_empty());
        // 기존 조건 교체는 새 용량을 소비하지 않아 한도에서도 허용한다.
        r.subscribe_domestic_condition(&domestic_condition("1"))
            .await
            .unwrap();
        assert_eq!(log.lock().unwrap().len(), 4);
        assert_eq!(r.status().confirmed_conditions, 1);
    }
}

// 단조 시계에 표현할 수 없는 기한·대기·세션 제한은 생성 단계에서 입출력 없이 거부한다.
#[test]
fn duration_max_recovery_limits_are_rejected_without_factory_or_token_io() {
    for overflow in 0..3 {
        let mut opts = options();
        let mut session_options = SessionOptions::default();
        match overflow {
            0 => opts.deadline = Duration::MAX,
            1 => opts.backoff = Duration::MAX,
            _ => session_options.timeout = Duration::MAX,
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let token_calls = Arc::new(AtomicUsize::new(0));
        let result = WsRecovery::new(
            Factory {
                transports: VecDeque::new(),
                calls: calls.clone(),
            },
            Tokens {
                delay: Duration::ZERO,
                calls: token_calls.clone(),
            },
            session_options,
            opts,
        );
        assert!(matches!(result, Err(ClientError::InvalidRequest(_))));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(token_calls.load(Ordering::SeqCst), 0);
    }
}
