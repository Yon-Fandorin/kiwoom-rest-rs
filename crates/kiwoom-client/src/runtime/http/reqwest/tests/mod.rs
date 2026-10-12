mod protocol_nack;

use super::*;
use crate::runtime::http::Secret;

// 미허용 금융 실행과 잘못된 API·경로 조합은 네트워크 연결 전에 차단됩니다.
#[tokio::test]
async fn concrete_transport_denies_unknown_api_before_network() {
    let transport = ReqwestTransport::with_explicit_network_opt_in(Environment::Sandbox).unwrap();

    let request = HttpRequest {
        path: "/api/dostk/ordr",
        api_id: "kt10000",
        authorization: None,
        continuation: None,
        body: Value::Null,
        financial_write_authorized: false,
    };
    assert!(transport.execute(request).await.is_err());

    let request = HttpRequest {
        path: "/api/dostk/ordr",
        api_id: "kt00015",
        authorization: None,
        continuation: None,
        body: Value::Null,
        financial_write_authorized: false,
    };
    assert!(transport.execute(request).await.is_err());
}

// 로컬 mock 서버에서 인증·연속 조회 헤더와 JSON 본문의 실제 전송을 확인합니다.
#[tokio::test]
async fn reqwest_sends_headers_and_json_to_local_mock_http_server() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut received = Vec::new();
        loop {
            let mut chunk = [0u8; 4096];
            let size = stream.read(&mut chunk).await.unwrap();
            assert!(size > 0 && received.len() + size < 16384);
            received.extend_from_slice(&chunk[..size]);
            let text = String::from_utf8_lossy(&received);
            if let Some(end) = text.find("\r\n\r\n") {
                let header = text[..end].to_ascii_lowercase();
                let length = header
                    .lines()
                    .find_map(|line| line.strip_prefix("content-length: "))
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                if received.len() >= end + 4 + length {
                    break;
                }
            }
        }
        let body = r#"{"return_code":0}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCont-Yn: N\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).await.unwrap();
        String::from_utf8(received).unwrap()
    });
    let transport = ReqwestTransport {
        client: reqwest::Client::builder().no_proxy().build().unwrap(),
        base: format!("http://{address}"),
    };

    let request = HttpRequest {
        path: "/api/us/acnt",
        api_id: "ust21100",
        authorization: Some(Secret::new("offline-http-token".into())),
        continuation: Some("offline-cursor".into()),
        body: serde_json::json!({"tp":"0"}),
        financial_write_authorized: false,
    };

    let response = transport.execute(request).await.unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.headers["cont-yn"], "N");
    assert_eq!(response.body["return_code"], 0);

    let wire = server.await.unwrap();
    let header = wire.split("\r\n\r\n").next().unwrap().to_ascii_lowercase();
    assert!(header.starts_with("post /api/us/acnt http/1.1"));
    assert!(header.contains("api-id: ust21100"));
    assert!(header.contains("authorization: bearer offline-http-token"));
    assert!(header.contains("cont-yn: y"));
    assert!(header.contains("next-key: offline-cursor"));
    assert!(header.contains("content-type: application/json;charset=utf-8"));
    assert_eq!(
        serde_json::from_str::<Value>(wire.split("\r\n\r\n").nth(1).unwrap()).unwrap(),
        serde_json::json!({"tp":"0"})
    );
}

// 알려진 오류 상태와 긴 Retry-After는 깨진 본문을 기다리지 않고 그대로 반환한다.
#[tokio::test]
async fn error_status_and_retry_after_survive_unfinished_error_body() {
    for status in [429, 503] {
        let head = format!(
            "HTTP/1.1 {status} Error\r\nRetry-After: 120\r\nContent-Length: 8388609\r\nConnection: close\r\n\r\nshort"
        );
        let (transport, server) = serve_response(head.into_bytes()).await;
        let client = crate::Client::new(
            transport,
            crate::ClientOptions {
                max_read_retries: 2,
                min_request_interval: Duration::ZERO,
                retry_base_delay: Duration::ZERO,
                ..Default::default()
            },
        )
        .unwrap();
        let request = crate::transactions::Ust21100Request {
            strt_dt: crate::transactions::BrokerDate::new("20250101").unwrap(),
            end_dt: crate::transactions::BrokerDate::new("20251231").unwrap(),
            tp: crate::transactions::UsTransactionType::All,
            stex_tp: None,
            stk_cd: None,
            krw_repl_skip_yn: crate::transactions::YesNo::No,
        };
        assert!(
            matches!(client.ust21100(&Secret::new("offline".into()), &request).await,
            Err(ClientError::Http { status: code, retry_after: Some(120) }) if code == status)
        );
        server.await.unwrap();
    }
}

// 값 수집 전 중복·디코딩 불가능한 제어 헤더를 구분하여 재시도할 전송 오류로 숨기지 않는다.
#[tokio::test]
async fn malformed_and_duplicate_wire_control_headers_are_terminal_protocol_errors() {
    for header in [
        b"Cont-Yn: \xff\r\n".to_vec(),
        b"Retry-After: \xff\r\n".to_vec(),
        b"Cont-Yn: Y\r\nCont-Yn: N\r\n".to_vec(),
        b"Next-Key: a\r\nNext-Key: b\r\n".to_vec(),
        b"Api-Id: ust21100\r\nApi-Id: ust21100\r\n".to_vec(),
        b"Retry-After: 0\r\nRetry-After: 120\r\n".to_vec(),
    ] {
        let mut raw = b"HTTP/1.1 200 OK\r\n".to_vec();
        raw.extend_from_slice(&header);
        raw.extend_from_slice(
            b"Content-Length: 17\r\nConnection: close\r\n\r\n{\"return_code\":0}",
        );
        let (transport, server) = serve_response(raw).await;
        let request =
            HttpRequest::read_only("/api/us/acnt", "ust21100", None, serde_json::json!({}));
        assert!(matches!(
            transport.execute_checked(request).await,
            Err(ClientError::Protocol(_))
        ));
        server.await.unwrap();
    }
}

async fn serve_response(raw: Vec<u8>) -> (ReqwestTransport, tokio::task::JoinHandle<()>) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut received = Vec::new();
        loop {
            let mut chunk = [0; 4096];
            let count = socket.read(&mut chunk).await.unwrap();
            assert!(count > 0 && received.len() < 16384);
            received.extend_from_slice(&chunk[..count]);
            if received.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        socket.write_all(&raw).await.unwrap();
    });
    (
        ReqwestTransport {
            client: ReqwestTransport::client_builder()
                .no_proxy()
                .build()
                .unwrap(),
            base: format!("http://{address}"),
        },
        server,
    )
}
