//! 합성 전송과 임시 loopback 포트로 HTTP 경계 및 프로토콜 수명을 검증합니다.
use kiwoom_client::core::Secret;
use kiwoom_mcp::{
    BackendTransport, DemoTransport, HttpAccess, KiwoomMcp, SessionMode, ToolPolicy, http_router,
};
use reqwest::{Client, Response, StatusCode};
use serde_json::{Value, json};
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
    task::JoinHandle,
};

const TOKEN: &str = "synthetic-http-access-0123456789";
const LEGACY: &str = "2025-11-25";
const MODERN: &str = "2026-07-28";
struct Server {
    url: String,
    host: String,

    stop: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

impl Server {
    async fn start(mode: SessionMode) -> Self {
        Self::with_policy(mode, None).await
    }
    async fn with_policy(mode: SessionMode, policy: Option<ToolPolicy>) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let access = HttpAccess::new(address, Secret::new(TOKEN.into())).unwrap();
        let handler = KiwoomMcp::new(
            BackendTransport::new(DemoTransport),
            Secret::new("synthetic-broker".into()),
            true,
        )
        .unwrap();
        let handler = match policy {
            Some(policy) => handler.with_tool_policy(policy),
            None => handler,
        };
        let (router, shutdown) = http_router(handler, access, mode);
        let (stop, receive) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = receive.await;
                    shutdown.shutdown().await;
                })
                .await
                .unwrap();
        });
        Self {
            url: format!("http://{address}/mcp"),
            host: address.to_string(),

            stop,
            task,
        }
    }
    fn post(&self, method: &str, version: &str) -> reqwest::RequestBuilder {
        Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap()
            .post(&self.url)
            .bearer_auth(TOKEN)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", version)
            .header("Mcp-Method", method)
    }
    async fn initialize(&self) -> Response {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": LEGACY,
                "capabilities": {},
                "clientInfo": { "name": "synthetic-local-test", "version": "0" }
            }
        });
        self.post("initialize", LEGACY)
            .json(&request)
            .send()
            .await
            .unwrap()
    }

    async fn finish(self) {
        self.stop.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(5), self.task)
            .await
            .unwrap()
            .unwrap();
    }
}

fn modern_body(method: &str, mut params: Value) -> Value {
    params["_meta"] = json!({
        "io.modelcontextprotocol/protocolVersion": MODERN,
        "io.modelcontextprotocol/clientInfo": {
            "name": "synthetic-local-test",
            "version": "0"
        },
        "io.modelcontextprotocol/clientCapabilities": {}
    });
    json!({"jsonrpc":"2.0","id":2,"method":method,"params":params})
}

async fn rpc_result(response: Response) -> Value {
    let is_sse = response
        .headers()
        .get("Content-Type")
        .is_some_and(|value| value.to_str().unwrap().starts_with("text/event-stream"));
    if !is_sse {
        return response.json().await.unwrap();
    }
    let body = response.text().await.unwrap();
    body.lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .filter_map(|data| serde_json::from_str::<Value>(data).ok())
        .find(|message| message.get("result").is_some())
        .unwrap_or_else(|| panic!("MCP 응답이 없습니다: {body}"))
}

#[test]
fn access_configuration_rejects_external_bind_and_invalid_header_token() {
    let secret = || Secret::new(TOKEN.into());
    assert!(HttpAccess::new("0.0.0.0:8000".parse().unwrap(), secret()).is_err());
    assert!(HttpAccess::new("[::]:8000".parse().unwrap(), secret()).is_err());
    let address: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    for value in [
        "short",
        "synthetic\r\ninjected-token",
        "비ASCII접근토큰123456789",
        "invalid-token:0123456789",
    ] {
        assert!(HttpAccess::new(address, Secret::new(value.into())).is_err());
    }
    assert!(HttpAccess::new("[::1]:8000".parse().unwrap(), secret()).is_ok());
}

#[tokio::test]
async fn modern_requests_are_stateless_in_both_modes() {
    for mode in [SessionMode::Stateless, SessionMode::Stateful] {
        let server = Server::start(mode).await;
        for method in ["server/discover", "tools/list"] {
            let response = server
                .post(method, MODERN)
                .json(&modern_body(method, json!({})))
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(!response.headers().contains_key("Mcp-Session-Id"));
            let result = response.json::<Value>().await.unwrap();
            assert!(result.get("result").is_some(), "{result}");
            if method == "tools/list" {
                assert_eq!(result["result"]["tools"].as_array().unwrap().len(), 5);
            }
        }
        let response = server
            .post("tools/call", MODERN)
            .header("Mcp-Name", "kiwoom_stock_info")
            .json(&modern_body(
                "tools/call",
                json!({"name":"kiwoom_stock_info","arguments":{"stk_cd":"005930"}}),
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(!response.headers().contains_key("Mcp-Session-Id"));
        let result = response.json::<Value>().await.unwrap();
        assert_ne!(result["result"]["isError"], true, "{result}");
        server.finish().await;
    }
}

#[tokio::test]
async fn legacy_stateless_initialize_and_call_need_no_session() {
    let server = Server::start(SessionMode::Stateless).await;
    let response = server.initialize().await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.headers().contains_key("Mcp-Session-Id"));
    let response = server
        .post("tools/list", LEGACY)
        .json(&json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    server.finish().await;
}

#[tokio::test]
async fn legacy_sessions_are_distinct_reused_and_deleted() {
    let server = Server::start(SessionMode::Stateful).await;
    let first = server.initialize().await;
    assert_eq!(first.status(), StatusCode::OK);
    let first_id = first.headers()["Mcp-Session-Id"]
        .to_str()
        .unwrap()
        .to_owned();
    let second = server.initialize().await;
    let second_id = second.headers()["Mcp-Session-Id"]
        .to_str()
        .unwrap()
        .to_owned();
    assert_ne!(first_id, second_id);
    for id in [&first_id, &second_id] {
        let response = server
            .post("notifications/initialized", LEGACY)
            .header("Mcp-Session-Id", id)
            .json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let response = server
            .post("tools/list", LEGACY)
            .header("Mcp-Session-Id", id)
            .json(&json!({"jsonrpc":"2.0","id":3,"method":"tools/list"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            rpc_result(response).await["result"]["tools"]
                .as_array()
                .unwrap()
                .len(),
            5
        );
    }
    let request = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "kiwoom_stock_info",
            "arguments": {
                "stk_cd": "005930"
            }
        }
    });
    let response = server
        .post("tools/call", LEGACY)
        .header("Mcp-Session-Id", &first_id)
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_ne!(rpc_result(response).await["result"]["isError"], true);
    let response = Client::new()
        .delete(&server.url)
        .bearer_auth(TOKEN)
        .header("MCP-Protocol-Version", LEGACY)
        .header("Mcp-Session-Id", &first_id)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    for id in [&first_id, &"unknown-session-id".to_owned()] {
        let response = server
            .post("tools/list", LEGACY)
            .header("Mcp-Session-Id", id)
            .json(&json!({"jsonrpc":"2.0","id":4,"method":"tools/list"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    let response = server
        .post("tools/list", LEGACY)
        .header("Mcp-Session-Id", second_id)
        .json(&json!({"jsonrpc":"2.0","id":5,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "다른 세션 삭제가 남은 세션에 영향을 주면 안 됩니다."
    );
    server.finish().await;
}

#[tokio::test]
async fn authentication_host_and_origin_boundaries_are_enforced() {
    let server = Server::start(SessionMode::Stateless).await;
    let body = modern_body("tools/list", json!({}));
    for authorization in [None, Some("Bearer invalid-access-token")] {
        let mut request = Client::new().post(&server.url).json(&body);
        if let Some(value) = authorization {
            request = request.header("Authorization", value);
        }
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.headers()["WWW-Authenticate"], "Bearer");
    }
    let response = server
        .post("tools/list", MODERN)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "같은 값이어도 중복 인증 헤더는 거부해야 합니다."
    );
    for (name, value) in [
        ("Host", "attacker.example"),
        ("Origin", "https://attacker.example"),
    ] {
        let response = server
            .post("tools/list", MODERN)
            .header(name, value)
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let response = server
        .post("tools/list", MODERN)
        .header("Origin", format!("http://{}", server.host))
        .header("Origin", "https://attacker.example")
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        StatusCode::FORBIDDEN,
        "첫 Origin이 허용되어도 중복 헤더는 거부해야 합니다."
    );
    let response = server
        .post("tools/list", MODERN)
        .header("Origin", format!("http://{}", server.host))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    server.finish().await;
}

#[tokio::test]
async fn body_limit_applies_to_nested_service_and_chunked_requests() {
    let server = Server::start(SessionMode::Stateless).await;
    let bytes = " ".repeat(16 * 1024 + 1);
    let response = server
        .post("tools/list", MODERN)
        .body(bytes.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    // 전체 길이를 알 수 없는 body도 서비스가 읽는 중에 제한합니다.
    let mut connection = tokio::net::TcpStream::connect(&server.host).await.unwrap();
    let request = format!(
        concat!(
            "POST /mcp HTTP/1.1\r\n",
            "Host: {}\r\n",
            "Authorization: Bearer {TOKEN}\r\n",
            "Accept: application/json, text/event-stream\r\n",
            "Content-Type: application/json\r\n",
            "MCP-Protocol-Version: {MODERN}\r\n",
            "Mcp-Method: tools/list\r\n",
            "Transfer-Encoding: chunked\r\n",
            "Connection: close\r\n\r\n",
            "{:X}\r\n{bytes}\r\n0\r\n\r\n"
        ),
        server.host,
        bytes.len(),
        TOKEN = TOKEN,
        MODERN = MODERN,
        bytes = bytes
    );
    connection.write_all(request.as_bytes()).await.unwrap();
    let mut reply = Vec::new();
    tokio::time::timeout(Duration::from_secs(5), connection.read_to_end(&mut reply))
        .await
        .unwrap()
        .unwrap();
    assert!(String::from_utf8_lossy(&reply).starts_with("HTTP/1.1 413"));
    server.finish().await;
}

#[tokio::test]
async fn legacy_expired_sessions_return_not_found_without_wall_clock_wait() {
    let server = Server::start(SessionMode::Stateful).await;
    let response = server.initialize().await;
    let id = response.headers()["Mcp-Session-Id"]
        .to_str()
        .unwrap()
        .to_owned();
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(601)).await;
    tokio::task::yield_now().await;
    tokio::time::resume();
    let response = server
        .post("tools/list", LEGACY)
        .header("Mcp-Session-Id", id)
        .json(&json!({"jsonrpc":"2.0","id":8,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    server.finish().await;
}

#[tokio::test]
async fn legacy_session_cannot_bypass_selected_tool_policy() {
    let policy = ToolPolicy::allow_only(vec!["kiwoom_stock_info".to_owned()]).unwrap();
    let server = Server::with_policy(SessionMode::Stateful, Some(policy)).await;
    let response = server.initialize().await;
    let id = response.headers()["Mcp-Session-Id"]
        .to_str()
        .unwrap()
        .to_owned();
    let response = server
        .post("notifications/initialized", LEGACY)
        .header("Mcp-Session-Id", &id)
        .json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    let response = server
        .post("tools/list", LEGACY)
        .header("Mcp-Session-Id", &id)
        .json(&json!({"jsonrpc":"2.0","id":9,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    let result = rpc_result(response).await;
    let tools = result["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0]["name"], "kiwoom_stock_info");
    let request = json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "kiwoom_account_evaluation",
            "arguments": {
                "qry_tp": "2",
                "dmst_stex_tp": "KRX"
            }
        }
    });
    let response = server
        .post("tools/call", LEGACY)
        .header("Mcp-Session-Id", &id)
        .json(&request)
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    let error = text
        .lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .filter_map(|data| serde_json::from_str::<Value>(data).ok())
        .find(|message| message.get("error").is_some())
        .unwrap_or_else(|| serde_json::from_str(&text).unwrap());
    assert!(
        error.get("error").is_some(),
        "세션 ID를 포함해도 숨긴 도구를 실행할 수 없어야 합니다: {error}"
    );
    server.finish().await;
}
