//! 명시적인 네트워크 연결, 고정 증권사 도메인, 프레임 크기 제한과 민감 정보 없는 오류를
//! 사용합니다.

use super::session::WsTransport;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use kiwoom_core::{ClientError, Environment};
use std::time::Duration;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async_with_config,
    tungstenite::{Message, protocol::WebSocketConfig},
};

// 한 텍스트 수신 동안 처리할 코덱 제어 프레임의 로컬 상한. 증권사 할당량이 아닙니다.
const MAX_PROTOCOL_CONTROLS_PER_RECEIVE: usize = 1000;

/// 크기가 제한된 실제 WebSocket 연결과 제어 프레임 코덱.
pub struct TungsteniteTransport {
    socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
}

/// 생성 시에는 네트워크에 연결하지 않는 조합 어댑터입니다.
/// `connect`가 기존의 고정·검증된 증권사 엔드포인트로 연결하는 명시적 네트워크 작업입니다.
pub struct TungsteniteFactory {
    environment: Environment,
    max_frame_bytes: usize,
    timeout: Duration,
}

impl TungsteniteFactory {
    /// 연결 환경·크기·기한을 검증합니다. 생성만으로 네트워크에 연결하지 않습니다.
    pub fn with_explicit_network_opt_in(
        environment: Environment,
        max_frame_bytes: usize,
        timeout: Duration,
    ) -> Result<Self, ClientError> {
        if max_frame_bytes == 0
            || timeout.is_zero()
            || tokio::time::Instant::now().checked_add(timeout).is_none()
        {
            return Err(ClientError::InvalidRequest("invalid socket limits"));
        }
        Ok(Self {
            environment,
            max_frame_bytes,
            timeout,
        })
    }
}

#[async_trait]
impl super::recovery::WsTransportFactory for TungsteniteFactory {
    type Transport = TungsteniteTransport;
    async fn connect(&mut self) -> Result<Self::Transport, ClientError> {
        TungsteniteTransport::connect_with_explicit_network_opt_in(
            self.environment,
            self.max_frame_bytes,
            self.timeout,
        )
        .await
    }
}

impl TungsteniteTransport {
    /// 고정된 증권사 주소에 명시적으로 연결하며 연결 기한과 수신 코덱 한도를 적용합니다.
    pub async fn connect_with_explicit_network_opt_in(
        environment: Environment,
        max_frame_bytes: usize,
        timeout: Duration,
    ) -> Result<Self, ClientError> {
        let url = match environment {
            Environment::Production => "wss://api.kiwoom.com:10000/api/websocket",
            Environment::Sandbox => "wss://mockapi.kiwoom.com:10000/api/websocket",
        };
        Self::connect(url, max_frame_bytes, timeout).await
    }

    async fn connect(url: &str, max: usize, timeout: Duration) -> Result<Self, ClientError> {
        if max == 0
            || timeout.is_zero()
            || tokio::time::Instant::now().checked_add(timeout).is_none()
        {
            return Err(ClientError::InvalidRequest("invalid socket limits"));
        }
        let config = WebSocketConfig::default()
            .max_message_size(Some(max))
            .max_frame_size(Some(max));
        let (socket, _) =
            tokio::time::timeout(timeout, connect_async_with_config(url, Some(config), false))
                .await
                .map_err(|_| ClientError::Transport)?
                .map_err(|_| ClientError::Transport)?;
        Ok(Self { socket })
    }
}

#[async_trait]
impl WsTransport for TungsteniteTransport {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        let value: serde_json::Value = serde_json::from_str(&text)
            .map_err(|_| ClientError::InvalidRequest("invalid WebSocket JSON"))?;
        if !matches!(
            value.get("trnm").and_then(serde_json::Value::as_str),
            Some(
                "LOGIN"
                    | "PING"
                    | "REG"
                    | "REMOVE"
                    | "CNSRLST"
                    | "CNSRREQ"
                    | "CNSRCLR"
                    | "GCNSRLST"
                    | "GCNSRREQ"
                    | "GCNSRCLR"
            )
        ) {
            return Err(ClientError::UnsupportedApi);
        }
        self.socket
            .send(Message::Text(text.into()))
            .await
            .map_err(|_| ClientError::Transport)
    }

    async fn receive(&mut self) -> Result<String, ClientError> {
        let mut controls = 0;
        while let Some(message) = self.socket.next().await {
            let message = message.map_err(|_| ClientError::Transport)?;
            if matches!(&message, Message::Ping(_) | Message::Pong(_)) {
                controls += 1;
                if controls > MAX_PROTOCOL_CONTROLS_PER_RECEIVE {
                    return Err(ClientError::Protocol(
                        "WebSocket protocol control frame limit reached",
                    ));
                }
            }
            match message {
                Message::Text(text) => return Ok(text.to_string()),
                Message::Ping(payload) => {
                    self.socket
                        .send(Message::Pong(payload))
                        .await
                        .map_err(|_| ClientError::Transport)?;
                }
                Message::Pong(_) => {}
                Message::Close(_) => return Err(ClientError::Transport),
                _ => return Err(ClientError::Protocol("unsupported binary WebSocket frame")),
            }
        }
        Err(ClientError::Transport)
    }

    async fn close(&mut self) -> Result<(), ClientError> {
        self.socket
            .close(None)
            .await
            .map_err(|_| ClientError::Transport)
    }
}

#[cfg(test)]
mod tests;
