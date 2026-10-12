//! loopback HTTP 접근과 이전 규격의 세션 수명을 구성합니다.
mod session;
use crate::KiwoomMcp;
use axum::{
    Router,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use clap::ValueEnum;
use kiwoom_client::core::Secret;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use session::BoundedSessions;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;

/// 최신 MCP는 항상 stateless이며 이 설정은 이전 규격에만 적용됩니다.
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum SessionMode {
    /// 이전 규격에서도 호출 사이의 MCP 세션을 유지하지 않습니다.
    #[default]
    Stateless,

    /// 이전 규격에 한해 메모리 세션을 유지합니다. 브로커 WS 구독은 만들지 않습니다.
    Stateful,
}

/// 브로커 토큰과 독립적인 HTTP 접근 토큰 및 loopback 수신 주소.
#[derive(Clone)]
pub struct HttpAccess {
    address: SocketAddr,
    bearer: Secret,
}

impl HttpAccess {
    /// loopback 외부 바인딩과 HTTP 헤더에 사용할 수 없는 접근 토큰을 거부합니다.
    pub fn new(address: SocketAddr, bearer: Secret) -> Result<Self, &'static str> {
        if !address.ip().is_loopback() {
            return Err("MCP HTTP는 loopback 주소만 허용합니다.");
        }
        if bearer.expose().len() < 16
            || !bearer
                .expose()
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-._~+/=".contains(&b))
        {
            return Err(
                "MCP HTTP 접근 토큰은 Bearer 문자로 구성된 16바이트 이상 문자열이어야 합니다.",
            );
        }
        Ok(Self { address, bearer })
    }
}

/// HTTP 서버 종료 시 요청과 모든 이전 규격 세션을 정리합니다.
pub struct HttpShutdown {
    cancellation: CancellationToken,
    sessions: Arc<BoundedSessions>,
}

impl HttpShutdown {
    /// 세션 종료를 기다립니다. 이후 신규 요청은 받지 않아야 합니다.
    pub async fn shutdown(&self) {
        self.cancellation.cancel();
        self.sessions.shutdown().await;
    }
}

impl Drop for HttpShutdown {
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}

#[derive(Clone)]
pub(crate) struct RequestCancellation(pub(crate) CancellationToken);

struct RequestAccess {
    access: HttpAccess,
    sessions: Arc<BoundedSessions>,
}

/// 단일 계정 범위의 서버를 구성합니다. 세션 ID가 계좌 접근 권한을 부여하지 않습니다.
/// 이전 규격 세션은 최대 32개이며 생성 후 10분이 지나면 종료합니다.
/// 반환된 종료 핸들은 HTTP 서버가 실행되는 동안 보유해야 합니다.
pub fn http_router(
    server: KiwoomMcp,
    access: HttpAccess,
    mode: SessionMode,
) -> (Router, HttpShutdown) {
    let port = access.address.port();
    let cancellation = CancellationToken::new();
    let server = server.with_shutdown_token(cancellation.clone());
    let sessions = Arc::new(BoundedSessions::default());
    sessions.spawn_reaper(cancellation.clone());
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(matches!(mode, SessionMode::Stateful))
        .with_json_response(true)
        .with_max_request_body_bytes(16 * 1024)
        .with_cancellation_token(cancellation.clone())
        .with_allowed_hosts(vec![
            access.address.to_string(),
            format!("localhost:{port}"),
        ])
        .with_allowed_origins(vec![
            format!("http://{}", access.address),
            format!("http://localhost:{port}"),
        ]);
    let service = StreamableHttpService::new(move || Ok(server.clone()), sessions.clone(), config);
    let router = Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn_with_state(
            Arc::new(RequestAccess {
                access,
                sessions: sessions.clone(),
            }),
            authorize,
        ));
    (
        router,
        HttpShutdown {
            cancellation,
            sessions,
        },
    )
}

async fn authorize(
    State(state): State<Arc<RequestAccess>>,
    mut request: Request,
    next: Next,
) -> Response {
    let access = &state.access;
    let authorized = request
        .headers()
        .get_all(header::AUTHORIZATION)
        .iter()
        .collect::<Vec<_>>();
    let expected = format!("Bearer {}", access.bearer.expose());
    if authorized.len() != 1 || authorized[0].as_bytes() != expected.as_bytes() {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Bearer")],
            "MCP HTTP 인증이 필요합니다.",
        )
            .into_response();
    }
    if [header::HOST, header::ORIGIN]
        .iter()
        .any(|name| request.headers().get_all(name).iter().count() > 1)
    {
        return (
            StatusCode::FORBIDDEN,
            "중복 Host 또는 Origin 헤더는 허용하지 않습니다.",
        )
            .into_response();
    }
    if request.headers().get_all("Mcp-Session-Id").iter().count() > 1 {
        return (
            StatusCode::BAD_REQUEST,
            "중복 MCP 세션 헤더는 허용하지 않습니다.",
        )
            .into_response();
    }
    let legacy = request
        .headers()
        .get("MCP-Protocol-Version")
        .and_then(|v| v.to_str().ok())
        .is_none_or(|version| version != "2026-07-28");
    if legacy {
        let id = request
            .headers()
            .get("Mcp-Session-Id")
            .and_then(|v| v.to_str().ok());
        if let Some(id) = id
            && let Some(token) = state.sessions.cancellation_for(id).await
        {
            request.extensions_mut().insert(RequestCancellation(token));
        }
    }
    match tokio::time::timeout(Duration::from_secs(30), next.run(request)).await {
        Ok(response) => response,
        Err(_) => (
            StatusCode::REQUEST_TIMEOUT,
            "MCP HTTP 요청 시간이 초과되었습니다.",
        )
            .into_response(),
    }
}
