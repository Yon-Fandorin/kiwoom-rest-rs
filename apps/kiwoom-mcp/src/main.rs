//! MCP 실행 설정으로 데모 또는 명시적인 브로커 전송을 조합합니다.
use clap::{Parser, ValueEnum};
use kiwoom_client::{
    core::Secret,
    rest::{Environment, ReqwestTransport},
};
use kiwoom_mcp::{
    BackendTransport, DemoTransport, HttpAccess, KiwoomMcp, SessionMode, ToolPolicy, http_router,
};
use rmcp::{ServiceExt, transport::stdio};
use std::{net::SocketAddr, path::PathBuf};

#[derive(Clone, Copy, Default, ValueEnum)]
enum Mode {
    #[default]
    Stdio,
    Http,
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum Backend {
    #[default]
    Demo,
    Broker,
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum BrokerEnvironment {
    #[default]
    Sandbox,
    Production,
}

#[derive(Parser)]
#[command(
    version,
    about = "키움 비공식 조회 전용 MCP 서버. 기본 실행은 합성 데모이며 증권사에 연결하지 않습니다."
)]
struct Options {
    #[arg(long, value_enum, default_value_t = Mode::Stdio)]
    transport: Mode,

    #[arg(long, value_enum, default_value_t = Backend::Demo)]
    backend: Backend,

    #[arg(long, value_enum, default_value_t = SessionMode::Stateless)]
    session_mode: SessionMode,

    #[arg(long, default_value = "127.0.0.1:8080")]
    bind: SocketAddr,

    #[arg(long, value_enum, default_value_t = BrokerEnvironment::Sandbox)]
    environment: BrokerEnvironment,

    /// 도구 허용 목록만 담은 사용자 설정 파일을 명시적으로 선택합니다.
    #[arg(long)]
    config: Option<PathBuf>,

    /// 설정 안의 프로필. 설정 파일만 지정하면 default를 선택합니다.
    #[arg(long, requires = "config")]
    profile: Option<String>,

    /// 브로커 네트워크 구성을 명시적으로 허용합니다. 기본값은 차단입니다.
    #[arg(long)]
    allow_broker_network: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();

    if matches!(options.transport, Mode::Stdio)
        && matches!(options.session_mode, SessionMode::Stateful)
    {
        return Err("--session-mode stateful은 --transport http에서만 적용합니다.".into());
    }
    if matches!(options.transport, Mode::Http)
        && (!options.bind.ip().is_loopback() || options.bind.port() == 0)
    {
        return Err("MCP HTTP는 포트가 지정된 loopback 주소만 허용합니다.".into());
    }
    if matches!(options.backend, Backend::Demo)
        && (options.allow_broker_network
            || matches!(options.environment, BrokerEnvironment::Production))
    {
        return Err("브로커 네트워크·실계좌 환경 설정에는 --backend broker가 필요합니다.".into());
    }

    let policy = match options.config.as_deref() {
        Some(path) => ToolPolicy::from_file(path, options.profile.as_deref().unwrap_or("default"))?,
        None => ToolPolicy::default(),
    };

    let (transport, token, demo) = match options.backend {
        Backend::Demo => (
            BackendTransport::new(DemoTransport),
            Secret::new("synthetic-only".into()),
            true,
        ),
        Backend::Broker => {
            if !options.allow_broker_network {
                return Err("--backend broker에는 --allow-broker-network가 필요합니다.".into());
            }
            let token = environment_secret("KIWOOM_ACCESS_TOKEN")?;
            let environment = match options.environment {
                BrokerEnvironment::Sandbox => Environment::Sandbox,
                BrokerEnvironment::Production => Environment::Production,
            };
            (
                BackendTransport::new(ReqwestTransport::with_explicit_network_opt_in(environment)?),
                token,
                false,
            )
        }
    };

    let http_bearer = if matches!(options.transport, Mode::Http) {
        let bearer = environment_secret("KIWOOM_MCP_HTTP_TOKEN")?;
        if !demo && bearer.expose() == token.expose() {
            return Err("MCP 접근 토큰과 브로커 토큰은 서로 달라야 합니다.".into());
        }
        Some(bearer)
    } else {
        None
    };

    let server = KiwoomMcp::new(transport, token, demo)?.with_tool_policy(policy);
    match options.transport {
        Mode::Stdio => {
            server.serve(stdio()).await?.waiting().await?;
        }
        Mode::Http => {
            let bearer = http_bearer.ok_or("MCP HTTP 접근 토큰이 없습니다.")?;
            let access = HttpAccess::new(options.bind, bearer)?;
            let listener = tokio::net::TcpListener::bind(options.bind).await?;
            eprintln!("kiwoom-mcp HTTP {} /mcp", listener.local_addr()?);
            let (router, shutdown) = http_router(server, access, options.session_mode);
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = tokio::signal::ctrl_c().await;
                    shutdown.shutdown().await;
                })
                .await?;
        }
    }
    Ok(())
}

fn environment_secret(name: &str) -> Result<Secret, String> {
    let value = std::env::var(name).map_err(|_| format!("{name} 환경 변수가 필요합니다."))?;
    if value.is_empty()
        || value
            .bytes()
            .any(|b| b.is_ascii_control() || b.is_ascii_whitespace())
    {
        return Err(format!("{name} 값은 공백 없는 문자열이어야 합니다."));
    }
    Ok(Secret::new(value))
}
