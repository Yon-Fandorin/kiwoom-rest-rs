use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::task::JoinSet;

// HTTP/2 스트림 거부 뒤에도 명시적으로 허용한 금융 POST를 다시 제출하지 않습니다.
#[tokio::test]
async fn reqwest_does_not_replay_financial_post_after_http2_refused_stream() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let accepted = Arc::new(AtomicUsize::new(0));
    let validated = Arc::new(AtomicUsize::new(0));
    let expected_body = serde_json::json!({
        "dmst_stex_tp": "KRX",
        "stk_cd": "005930",
        "ord_qty": "1",
        "trde_tp": "3"
    });

    let server_accepted = Arc::clone(&accepted);
    let server_validated = Arc::clone(&validated);
    let server_body = expected_body.clone();
    let server = tokio::spawn(async move {
        let mut connections = JoinSet::new();
        // 연결 종료와 재연결 모두 관찰하며, 대기 중인 작업은 제한 시간 뒤 정리합니다.
        let observation = tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                tokio::select! {
                    incoming = listener.accept() => {
                        let (socket, _) = incoming.unwrap();
                        let accepted = Arc::clone(&server_accepted);
                        let validated = Arc::clone(&server_validated);
                        let expected_body = server_body.clone();
                        connections.spawn(async move {
                            let Ok(mut connection) = h2::server::handshake(socket).await else {
                                return;
                            };
                            let mut requests = JoinSet::new();
                            loop {
                                tokio::select! {
                                    incoming = connection.accept() => {
                                        let Some(Ok((request, mut respond))) = incoming else {
                                            break;
                                        };
                                        accepted.fetch_add(1, Ordering::SeqCst);
                                        let validated = Arc::clone(&validated);
                                        let expected_body = expected_body.clone();
                                        requests.spawn(async move {
                                            assert_eq!(request.method().as_str(), "POST");
                                            assert_eq!(request.uri().path(), "/api/dostk/ordr");
                                            assert_eq!(request.headers()["api-id"], "kt10000");
                                            assert!(request.headers().get("authorization").is_some_and(
                                                |value| value == "Bearer offline-http2-token"
                                            ));
                                            let mut body = request.into_body();
                                            let mut bytes = Vec::new();
                                            while let Some(chunk) = body.data().await {
                                                let chunk = chunk.expect("로컬 HTTP/2 주문 본문을 수신해야 합니다");
                                                assert!(bytes.len() + chunk.len() <= 4096);
                                                body.flow_control().release_capacity(chunk.len()).unwrap();
                                                bytes.extend_from_slice(&chunk);
                                            }
                                            assert_eq!(
                                                serde_json::from_slice::<Value>(&bytes).unwrap(),
                                                expected_body
                                            );
                                            validated.fetch_add(1, Ordering::SeqCst);
                                            // 본문 수신을 마친 실제 POST에 재전송 가능한 프로토콜 거부를 보냅니다.
                                            respond.send_reset(h2::Reason::REFUSED_STREAM);
                                        });
                                    }
                                    completed = requests.join_next(), if !requests.is_empty() => {
                                        completed.unwrap().unwrap();
                                    }
                                }
                            }
                            while let Some(completed) = requests.join_next().await {
                                completed.unwrap();
                            }
                        });
                    }
                    completed = connections.join_next(), if !connections.is_empty() => {
                        completed.unwrap().unwrap();
                    }
                }
            }
        }).await;
        assert!(observation.is_err());
        connections.abort_all();
        while let Some(completed) = connections.join_next().await {
            if let Err(error) = completed {
                assert!(error.is_cancelled(), "로컬 HTTP/2 서버 작업이 실패했습니다");
            }
        }
    });

    let transport = ReqwestTransport {
        client: ReqwestTransport::client_builder()
            .no_proxy()
            .http2_prior_knowledge()
            .build()
            .unwrap(),
        base: format!("http://{address}"),
    };
    let request = HttpRequest {
        path: "/api/dostk/ordr",
        api_id: "kt10000",
        authorization: Some(Secret::new("offline-http2-token".into())),
        continuation: None,
        body: expected_body,
        financial_write_authorized: true,
    };
    let result = tokio::time::timeout(Duration::from_secs(2), transport.execute_checked(request))
        .await
        .expect("HTTP/2 스트림 거부를 제한 시간 안에 반환해야 합니다");
    server.await.unwrap();

    assert!(matches!(result, Err(ClientError::Transport)));
    assert_eq!(validated.load(Ordering::SeqCst), 1);
    assert_eq!(accepted.load(Ordering::SeqCst), 1);
}
