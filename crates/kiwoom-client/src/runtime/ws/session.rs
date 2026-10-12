//! 단일 소유자가 관리하는 타입 기반 WebSocket 세션입니다.
//! 전체 작업 기한을 적용하며 실패 시 세션을 무효화합니다.

use async_trait::async_trait;
use kiwoom_core::{ClientError, Secret};
pub use kiwoom_domain::ws::request::WsRequest;
use serde_json::Value;
use std::{collections::VecDeque, time::Duration};

/// 세션이 주입받는 WebSocket 텍스트 전송 포트. 연결 구현은 이 계약을 구현합니다.
#[async_trait]
pub trait WsTransport: Send {
    /// 텍스트를 한 번 전송하며 오류에 민감한 원문을 담지 않습니다.
    async fn send(&mut self, text: String) -> Result<(), ClientError>;

    /// 다음 텍스트 메시지를 받습니다.
    async fn receive(&mut self) -> Result<String, ClientError>;

    /// 연결 종료를 시도합니다.
    async fn close(&mut self) -> Result<(), ClientError>;
}

/// 한 작업의 전체 기한과 프레임·대기 이벤트 한도.
#[derive(Debug, Clone)]
pub struct SessionOptions {
    /// 전송·수신·PING 처리를 포함하며 시계에 더할 수 있는 유한한 기한.
    pub timeout: Duration,

    /// 직렬화된 요청과 받은 텍스트 프레임의 최대 바이트 수.
    pub max_frame_bytes: usize,

    /// 다른 응답을 기다리는 동안 보관할 이벤트의 최대 개수.
    pub max_pending_events: usize,

    /// 응답·이벤트를 기다리며 처리할 텍스트 메시지의 최대 개수. JSON PING을 포함합니다.
    /// 코덱의 프로토콜 Ping/Pong 개수 제한은 전송 어댑터가 따로 적용합니다.
    pub max_interleaved_frames: usize,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_frame_bytes: 8 * 1024 * 1024,
            max_pending_events: 200,
            max_interleaved_frames: 1000,
        }
    }
}

/// 같은 응답의 typed 뷰, 원본 JSON과 성공 확인 여부.
#[derive(Debug, Clone)]
pub struct WsReply<R> {
    /// 같은 원본에서 디코딩한 응답.
    pub typed: R,

    /// 알 수 없는 필드까지 보존한 JSON.
    pub raw: Value,

    /// 선택적인 증권사 성공 코드가 없으면 false입니다.
    pub acknowledged: bool,
}

/// 한 번에 한 작업을 수행하며 취소·실패 이후 재연결을 요구하는 세션.
pub struct WsSession<T> {
    transport: T,
    options: SessionOptions,
    authenticated: bool,
    usable: bool,
    pending: VecDeque<Value>,
}

impl<T: WsTransport> WsSession<T> {
    /// 정상 한도를 확인하고 주입된 전송으로 미인증 세션을 구성합니다.
    pub fn new(transport: T, options: SessionOptions) -> Result<Self, ClientError> {
        if options.timeout.is_zero()
            || options.max_frame_bytes == 0
            || options.max_pending_events == 0
            || options.max_interleaved_frames == 0
            || tokio::time::Instant::now()
                .checked_add(options.timeout)
                .is_none()
        {
            return Err(ClientError::InvalidRequest("invalid WebSocket limits"));
        }
        Ok(Self {
            transport,
            options,
            authenticated: false,
            usable: true,
            pending: VecDeque::new(),
        })
    }

    /// 인증을 마쳤고 재사용할 수 있는 세션인지 반환합니다.
    pub fn is_authenticated(&self) -> bool {
        self.authenticated && self.usable
    }

    /// 실패·취소·종료로 새 전송이 필요한지 반환합니다.
    pub fn requires_reconnect(&self) -> bool {
        !self.usable
    }

    fn invalidate(&mut self) {
        self.usable = false;
        self.authenticated = false;
        self.pending.clear();
    }

    /// LOGIN 성공 코드를 확인합니다. 한 전송에서 인증을 다시 시작하지 않습니다.
    pub async fn login(&mut self, token: &Secret) -> Result<(), ClientError> {
        if !self.usable || self.authenticated {
            return Err(ClientError::Protocol(
                "new WebSocket transport required for LOGIN",
            ));
        }
        if token.expose().trim().is_empty() {
            return Err(ClientError::InvalidRequest("empty WebSocket token"));
        }
        self.pending.clear();
        // 작업 중 취소되어도 미확인 세션을 재사용하지 않도록 먼저 무효화합니다.
        self.usable = false;
        let deadline = self.options.timeout;
        let result = tokio::time::timeout(deadline, async {
            self.send_value(&serde_json::json!({"trnm":"LOGIN","token":token.expose()}))
                .await?;
            let response = self.wait_for(&["LOGIN"], None).await?;
            if response.get("return_code").and_then(Value::as_i64) != Some(0) {
                return Err(ClientError::Protocol("invalid LOGIN acknowledgement"));
            }
            Ok(())
        })
        .await
        .unwrap_or(Err(ClientError::Transport));
        if result.is_ok() {
            self.usable = true;
            self.authenticated = true;
        } else {
            self.invalidate();
        }
        result
    }

    /// 요청을 전송하고 서비스·선택 순번에 맞는 응답을 기다립니다.
    /// 순번이 양쪽에 있을 때만 비교하며, 생략된 순번은 정확한 상관관계를 보장하지 않습니다.
    /// 직렬화 이후 프레임 크기 거부도 보수적으로 세션을 무효화합니다.
    pub async fn call<Q: WsRequest>(
        &mut self,
        request: &Q,
    ) -> Result<WsReply<Q::Response>, ClientError> {
        if !self.is_authenticated() {
            return Err(ClientError::Protocol("WebSocket LOGIN/reconnect required"));
        }
        request.validate()?;
        if !crate::domain::ws::conditions::CONTRACT_IDS.contains(&request.api_id())
            && !crate::domain::ws::realtime::CONTRACT_IDS.contains(&request.api_id())
        {
            return Err(ClientError::UnsupportedApi);
        }
        let wire = serde_json::to_value(request)
            .map_err(|_| ClientError::InvalidRequest("WebSocket serialization failed"))?;
        let seq = wire.get("seq").cloned();
        // 작업 중 취소되어도 미확인 세션을 재사용하지 않도록 먼저 무효화합니다.
        self.usable = false;
        let deadline = self.options.timeout;
        let result = tokio::time::timeout(deadline, async {
            self.send_value(&wire).await?;
            let raw = self
                .wait_for(request.expected_services(), seq.as_ref())
                .await?;
            let typed = serde_json::from_value(raw.clone())
                .map_err(|_| ClientError::Protocol("typed WebSocket response decode failed"))?;
            // 응답 디코딩과 성공 확인은 별개입니다.
            // 코드가 없으면 성공으로 간주하지 않습니다.
            let acknowledged = raw.get("return_code").and_then(Value::as_i64) == Some(0);
            Ok(WsReply {
                typed,
                raw,
                acknowledged,
            })
        })
        .await
        .unwrap_or(Err(ClientError::Transport));
        if result.is_err() {
            self.invalidate();
        } else {
            self.usable = true;
        }
        result
    }

    /// 인증된 세션의 대기 이벤트를 수신 순서로 꺼냅니다.
    pub fn pop_pending(&mut self) -> Option<Value> {
        if !self.is_authenticated() {
            return None;
        }
        self.pending.pop_front()
    }

    /// 대기 이벤트 또는 새 이벤트를 반환하며 전체 수신 작업의 기한을 적용합니다.
    pub async fn next_event(&mut self) -> Result<Value, ClientError> {
        if !self.is_authenticated() {
            return Err(ClientError::Protocol("WebSocket LOGIN/reconnect required"));
        }
        if let Some(e) = self.pending.pop_front() {
            return Ok(e);
        }
        // 작업 중 취소되어도 미확인 세션을 재사용하지 않도록 먼저 무효화합니다.
        self.usable = false;
        let deadline = self.options.timeout;
        let result = tokio::time::timeout(deadline, async {
            for _ in 0..self.options.max_interleaved_frames {
                if let Some(e) = self.receive_one().await? {
                    return Ok(e);
                }
            }
            Err(ClientError::Protocol(
                "WebSocket control frame limit reached",
            ))
        })
        .await
        .unwrap_or(Err(ClientError::Transport));
        if result.is_err() {
            self.invalidate();
        } else {
            self.usable = true;
        }
        result
    }

    /// 종료 전 세션을 무효화해 종료 실패·시간 초과·취소 뒤의 재사용을 막습니다.
    pub async fn close(&mut self) -> Result<(), ClientError> {
        self.invalidate();
        tokio::time::timeout(self.options.timeout, self.transport.close())
            .await
            .map_err(|_| ClientError::Transport)?
    }

    async fn send_value(&mut self, v: &Value) -> Result<(), ClientError> {
        let text = v.to_string();
        if text.len() > self.options.max_frame_bytes {
            return Err(ClientError::InvalidRequest("WebSocket frame too large"));
        }
        self.transport.send(text).await
    }

    async fn receive_one(&mut self) -> Result<Option<Value>, ClientError> {
        let text = self.transport.receive().await?;
        if text.len() > self.options.max_frame_bytes {
            return Err(ClientError::Protocol("WebSocket frame too large"));
        }
        let v: Value = serde_json::from_str(&text)
            .map_err(|_| ClientError::Protocol("invalid WebSocket JSON"))?;
        let service = v
            .get("trnm")
            .and_then(Value::as_str)
            .ok_or(ClientError::Protocol("missing WebSocket service"))?;
        // JSON PING은 결과 응답이 아닌 제어 프레임으로 취급해 원문 필드를 그대로 돌려줍니다.
        // 그 외 프레임에 제공된 결과 코드는 아래에서 검증합니다.
        if service == "PING" {
            self.send_value(&v).await?;
            return Ok(None);
        }
        if let Some(value) = v.get("return_code") {
            let code = value
                .as_i64()
                .ok_or(ClientError::Protocol("invalid WebSocket return_code"))?;
            if code != 0 {
                return Err(ClientError::Api(code));
            }
        }
        Ok(Some(v))
    }

    async fn wait_for(
        &mut self,
        services: &[&str],
        seq: Option<&Value>,
    ) -> Result<Value, ClientError> {
        for _ in 0..self.options.max_interleaved_frames {
            if let Some(v) = self.receive_one().await? {
                let same_service = services.contains(&v["trnm"].as_str().unwrap_or(""));
                let same_seq = seq
                    .zip(v.get("seq"))
                    .is_none_or(|(expected, actual)| expected == actual);
                if same_service && same_seq {
                    return Ok(v);
                }
                if self.pending.len() >= self.options.max_pending_events {
                    return Err(ClientError::Protocol(
                        "WebSocket event buffer full; recovery required",
                    ));
                }
                self.pending.push_back(v);
            }
        }
        Err(ClientError::Protocol(
            "WebSocket response frame limit reached",
        ))
    }
}
