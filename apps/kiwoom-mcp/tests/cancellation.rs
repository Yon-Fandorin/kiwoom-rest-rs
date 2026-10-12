//! 실제 MCP 취소 통지가 진행 중인 SDK 전송과 후속 페이지를 중단하는지 검사합니다.
use async_trait::async_trait;
use kiwoom_client::{
    core::Secret,
    rest::{HttpRequest, HttpResponse, Transport, TransportError},
};
use kiwoom_mcp::{BackendTransport, KiwoomMcp};
use rmcp::ServiceExt;
use serde_json::json;
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    sync::Notify,
};

struct Slow {
    started: Arc<Notify>,
    release: Arc<Notify>,
    dropped: Arc<Notify>,
    count: Arc<AtomicUsize>,
}

struct Dropped(Arc<Notify>);

impl Drop for Dropped {
    fn drop(&mut self) {
        self.0.notify_one();
    }
}

#[async_trait]
impl Transport for Slow {
    async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, TransportError> {
        let n = self.count.fetch_add(1, Ordering::SeqCst);
        if n == 0 {
            let _guard = Dropped(self.dropped.clone());
            self.started.notify_one();
            self.release.notified().await;
        }
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::from([
                ("cont-yn".into(), if n == 0 { "Y" } else { "N" }.into()),
                ("next-key".into(), "synthetic-cursor".into()),
            ]),
            body: json!({"return_code":0}),
        })
    }
}

async fn send_message(writer: &mut (impl AsyncWrite + Unpin), message: serde_json::Value) {
    let mut line = serde_json::to_vec(&message).unwrap();
    line.push(b'\n');
    writer.write_all(&line).await.unwrap();
}

// notifications/cancelled가 첫 전송을 폐기해 다음 페이지가 시작되지 않습니다.
#[tokio::test]
async fn cancellation_notification_drops_pending_io_and_prevents_next_page() {
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let dropped = Arc::new(Notify::new());
    let count = Arc::new(AtomicUsize::new(0));
    let server = KiwoomMcp::new(
        BackendTransport::new(Slow {
            started: started.clone(),
            release: release.clone(),
            dropped: dropped.clone(),
            count: count.clone(),
        }),
        Secret::new("synthetic-only".into()),
        true,
    )
    .unwrap();
    let (server_stream, client_stream) = tokio::io::duplex(65536);
    let task = tokio::spawn(async move {
        server
            .serve(server_stream)
            .await
            .unwrap()
            .waiting()
            .await
            .unwrap();
    });
    let (read, mut write) = tokio::io::split(client_stream);
    let mut read = BufReader::new(read);
    send_message(
        &mut write,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-11-25",
                "capabilities": {},
                "clientInfo": {
                    "name": "synthetic-cancellation",
                    "version": "0"
                }
            }
        }),
    )
    .await;
    let mut response = String::new();
    read.read_line(&mut response).await.unwrap();
    send_message(
        &mut write,
        json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }),
    )
    .await;
    send_message(
        &mut write,
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "kiwoom_stock_info",
                "arguments": {
                    "stk_cd": "000001"
                }
            }
        }),
    )
    .await;
    tokio::time::timeout(Duration::from_secs(1), started.notified())
        .await
        .unwrap();
    send_message(
        &mut write,
        json!({
            "jsonrpc": "2.0",
            "method": "notifications/cancelled",
            "params": {
                "requestId": 2,
                "reason": "synthetic test"
            }
        }),
    )
    .await;
    tokio::time::timeout(Duration::from_secs(1), dropped.notified())
        .await
        .expect("취소 통지가 30초 마감시간 이전에 전송을 폐기해야 합니다.");
    release.notify_one();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    drop(write);
    drop(read);
    tokio::time::timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap();
}

// HTTP 중단·서버 종료·이전 규격 세션 삭제가 진행 중인 SDK 조회를 즉시 폐기합니다.
#[tokio::test]
async fn http_disconnect_session_delete_and_shutdown_stop_sdk_io() {
    use kiwoom_mcp::{HttpAccess, SessionMode, http_router};
    for action in [
        "modern_disconnect",
        "modern_shutdown",
        "legacy_delete",
        "legacy_delete_missing_version",
        "legacy_shutdown",
    ] {
        let legacy = action.starts_with("legacy");
        let started = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());
        let dropped = Arc::new(Notify::new());
        let count = Arc::new(AtomicUsize::new(0));
        let handler = KiwoomMcp::new(
            BackendTransport::new(Slow {
                started: started.clone(),
                release: release.clone(),
                dropped: dropped.clone(),
                count: count.clone(),
            }),
            Secret::new("synthetic-only".into()),
            true,
        )
        .unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let token = "synthetic-http-cancellation-0123456789";
        let mode = if legacy {
            SessionMode::Stateful
        } else {
            SessionMode::Stateless
        };
        let (router, shutdown) = http_router(
            handler,
            HttpAccess::new(address, Secret::new(token.into())).unwrap(),
            mode,
        );
        let (stop, receive) = tokio::sync::oneshot::channel::<()>();
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = receive.await;
                    shutdown.shutdown().await;
                })
                .await
                .unwrap();
        });
        let url = format!("http://{address}/mcp");
        let client = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let mut session = None;
        if legacy {
            let request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-11-25",
                    "capabilities": {},
                    "clientInfo": { "name": "synthetic-cancellation", "version": "0" }
                }
            });
            let init = client
                .post(&url)
                .bearer_auth(token)
                .header("Accept", "application/json, text/event-stream")
                .header("MCP-Protocol-Version", "2025-11-25")
                .json(&request)
                .send()
                .await
                .unwrap();
            assert!(init.status().is_success());
            let id = init.headers()["Mcp-Session-Id"]
                .to_str()
                .unwrap()
                .to_owned();
            drop(init);
            client
                .post(&url)
                .bearer_auth(token)
                .header("Accept", "application/json, text/event-stream")
                .header("MCP-Protocol-Version", "2025-11-25")
                .header("Mcp-Session-Id", &id)
                .json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
                .send()
                .await
                .unwrap();
            session = Some(id);
        }
        let mut params = json!({"name":"kiwoom_stock_info","arguments":{"stk_cd":"000001"}});
        if !legacy {
            params["_meta"] = json!({
                "io.modelcontextprotocol/protocolVersion": "2026-07-28",
                "io.modelcontextprotocol/clientInfo": {
                    "name": "synthetic-cancellation",
                    "version": "0"
                },
                "io.modelcontextprotocol/clientCapabilities": {}
            });
        }
        let body =
            json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":params}).to_string();
        let version = if legacy { "2025-11-25" } else { "2026-07-28" };
        let session_header = session
            .as_ref()
            .map(|id| format!("Mcp-Session-Id: {id}\r\n"))
            .unwrap_or_default();
        let version_header = if action == "legacy_delete_missing_version" {
            String::new()
        } else {
            format!("MCP-Protocol-Version: {version}\r\n")
        };
        let mut connection = tokio::net::TcpStream::connect(address).await.unwrap();
        let request = format!(
            concat!(
                "POST /mcp HTTP/1.1\r\n",
                "Host: {address}\r\n",
                "Authorization: Bearer {token}\r\n",
                "Accept: application/json, text/event-stream\r\n",
                "Content-Type: application/json\r\n",
                "{version_header}",
                "Mcp-Method: tools/call\r\n",
                "Mcp-Name: kiwoom_stock_info\r\n",
                "{session_header}",
                "Content-Length: {}\r\n\r\n{body}"
            ),
            body.len(),
            address = address,
            token = token,
            version_header = version_header,
            session_header = session_header,
            body = body
        );
        connection.write_all(request.as_bytes()).await.unwrap();
        tokio::time::timeout(Duration::from_secs(1), started.notified())
            .await
            .unwrap();
        let mut stop = Some(stop);
        match action {
            "modern_disconnect" => drop(connection),
            "modern_shutdown" | "legacy_shutdown" => {
                stop.take().unwrap().send(()).unwrap();
                drop(connection);
            }
            "legacy_delete" | "legacy_delete_missing_version" => {
                let response = client
                    .delete(&url)
                    .bearer_auth(token)
                    .header("MCP-Protocol-Version", version)
                    .header("Mcp-Session-Id", session.unwrap())
                    .send()
                    .await
                    .unwrap();
                assert!(response.status().is_success());
                drop(connection);
            }
            _ => unreachable!(),
        }
        tokio::time::timeout(Duration::from_secs(1), dropped.notified())
            .await
            .unwrap_or_else(|_| {
                panic!("{action}: HTTP·세션 종료가 진행 중인 전송을 즉시 폐기해야 합니다.")
            });
        release.notify_one();
        assert_eq!(
            count.load(Ordering::SeqCst),
            1,
            "{action}: 취소 이후 추가 페이지를 요청하면 안 됩니다."
        );
        if let Some(stop) = stop {
            stop.send(()).unwrap();
        }
        tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .unwrap()
            .unwrap();
    }
}
