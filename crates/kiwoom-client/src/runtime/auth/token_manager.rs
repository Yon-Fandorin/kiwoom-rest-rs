//! 토큰은 메모리에서 관리하며, 만료 시각 해석 정책과 자격 증명을 명시적으로 주입합니다.

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use kiwoom_core::Secret;
use kiwoom_domain::auth::TokenResponse;
use std::{fmt, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};

/// 발급 실패에는 공급자 메시지, 응답 본문 또는 자격 증명을 담지 않습니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenIssueError;

#[async_trait]
pub trait TokenIssuer: Send + Sync {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Debug)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpiryInterpretationError;
pub trait ExpiryInterpreter: Send + Sync {
    fn interpret(&self, raw_expires_dt: &str) -> Result<DateTime<Utc>, ExpiryInterpretationError>;
}

/// 공개 예제는 14자리 날짜·시간만 제시하고 시간대는 명시하지 않습니다.
/// 호출자가 별도로 확인한 시간대 오프셋을 제공해야 하며, 이 정책 생성 자체가 오프셋을
/// 검증하지는 않습니다.
#[derive(Debug)]
pub struct CompactExpiryWithOffset {
    offset: FixedOffset,
}

impl CompactExpiryWithOffset {
    pub fn new(offset: FixedOffset) -> Self {
        Self { offset }
    }
}

impl ExpiryInterpreter for CompactExpiryWithOffset {
    fn interpret(&self, raw: &str) -> Result<DateTime<Utc>, ExpiryInterpretationError> {
        if raw.len() != 14 || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(ExpiryInterpretationError);
        }
        let naive = NaiveDateTime::parse_from_str(raw, "%Y%m%d%H%M%S")
            .map_err(|_| ExpiryInterpretationError)?;
        // 예제와 응답 표에서 윤초 지원을 확인할 수 없습니다.
        if chrono::Timelike::nanosecond(&naive) != 0 {
            return Err(ExpiryInterpretationError);
        }
        self.offset
            .from_local_datetime(&naive)
            .single()
            .map(|date| date.with_timezone(&Utc))
            .ok_or(ExpiryInterpretationError)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TokenManagerOptions {
    /// 클라이언트의 사전 갱신 여유 시간이며 서버 토큰 유효기간을 나타내지 않습니다.
    pub refresh_skew: Duration,

    pub issuance_timeout: Duration,
    pub failure_cooldown: Duration,
}

impl Default for TokenManagerOptions {
    fn default() -> Self {
        Self {
            refresh_skew: Duration::from_secs(30),
            issuance_timeout: Duration::from_secs(30),
            failure_cooldown: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TokenManagerError {
    #[error("invalid token manager limits")]
    InvalidOptions,
    #[error("token issuance failed")]
    IssuanceFailed,
    #[error("token issuance timed out")]
    IssuanceTimedOut,
    #[error("previous token issuance was cancelled; cooldown is active")]
    IssuanceCancelled,
    #[error("token expiry cannot be interpreted by the configured policy")]
    InvalidExpiry,
    #[error("issued token is unusable under the configured refresh policy")]
    UnusableToken,
    #[error("token generation exhausted")]
    GenerationExhausted,
}

#[derive(Clone)]
pub struct TokenLease {
    token: Secret,
    generation: u64,
    expires_at: DateTime<Utc>,
    raw_expires_dt: String,
}

impl TokenLease {
    pub fn token(&self) -> &Secret {
        &self.token
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn raw_expires_dt(&self) -> &str {
        &self.raw_expires_dt
    }
}

impl fmt::Debug for TokenLease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenLease")
            .field("generation", &self.generation)
            .field("expires_at", &self.expires_at)
            .field("token", &"[REDACTED]")
            .field("raw_expires_dt", &"[REDACTED]")
            .finish()
    }
}

struct State {
    lease: Option<TokenLease>,
    generation: u64,
    retry_not_before: Option<Instant>,
    last_failure: TokenManagerError,
}

struct PendingIssuance<'a> {
    state: &'a mut State,
    cooldown: Duration,
    finished: bool,
}

impl Drop for PendingIssuance<'_> {
    fn drop(&mut self) {
        if !self.finished {
            // 기존 예약 기한 뒤에 취소되더라도 뮤텍스를 풀기 전에 실행합니다.
            // 대기 중인 호출자도 이 대기 시간을 모두 적용받습니다.
            self.state.last_failure = TokenManagerError::IssuanceCancelled;
            self.state.retry_not_before = Some(Instant::now() + self.cooldown);
        }
    }
}

pub struct TokenManager {
    issuer: Arc<dyn TokenIssuer>,
    clock: Arc<dyn Clock>,
    expiry: Arc<dyn ExpiryInterpreter>,
    options: TokenManagerOptions,
    state: Mutex<State>,
}

impl fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenManager")
            .field("options", &self.options)
            .field("issuer", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl TokenManager {
    pub fn new(
        issuer: Arc<dyn TokenIssuer>,
        clock: Arc<dyn Clock>,
        expiry: Arc<dyn ExpiryInterpreter>,
        options: TokenManagerOptions,
    ) -> Result<Self, TokenManagerError> {
        if options.issuance_timeout.is_zero()
            || options.issuance_timeout > Duration::from_secs(120)
            || options.failure_cooldown.is_zero()
            || options.failure_cooldown > Duration::from_secs(300)
            || options.refresh_skew > Duration::from_secs(86400)
        {
            return Err(TokenManagerError::InvalidOptions);
        }
        Ok(Self {
            issuer,
            clock,
            expiry,
            options,
            state: Mutex::new(State {
                lease: None,
                generation: 0,
                retry_not_before: None,
                last_failure: TokenManagerError::IssuanceCancelled,
            }),
        })
    }

    fn usable(&self, expires_at: DateTime<Utc>) -> bool {
        chrono::Duration::from_std(self.options.refresh_skew)
            .ok()
            .and_then(|skew| self.clock.now().checked_add_signed(skew))
            .is_some_and(|threshold| threshold < expires_at)
    }

    /// 관리자·공급자 범위에서 토큰 발급을 하나로 모읍니다.
    /// 발급 중인 호출이 취소되면 뮤텍스는 풀리지만 대기 시간은 유지해 후속 호출의 발급 폭주를
    /// 막습니다.
    /// 백그라운드 작업, 숨은 발급 재시도 또는 리소스 작업 재실행은 없습니다.
    pub async fn lease(&self) -> Result<TokenLease, TokenManagerError> {
        let mut state = self.state.lock().await;
        if let Some(lease) = &state.lease
            && self.usable(lease.expires_at)
        {
            return Ok(lease.clone());
        }
        if state
            .retry_not_before
            .is_some_and(|deadline| Instant::now() < deadline)
        {
            return Err(state.last_failure);
        }
        state.lease = None;
        state.last_failure = TokenManagerError::IssuanceCancelled;
        state.retry_not_before = Some(Instant::now() + self.options.failure_cooldown);
        let mut pending = PendingIssuance {
            state: &mut state,
            cooldown: self.options.failure_cooldown,
            finished: false,
        };
        let result =
            match tokio::time::timeout(self.options.issuance_timeout, self.issuer.issue()).await {
                Ok(Ok(response)) => self.accept(response, pending.state.generation),
                Ok(Err(_)) => Err(TokenManagerError::IssuanceFailed),
                Err(_) => Err(TokenManagerError::IssuanceTimedOut),
            };
        pending.finished = true;
        match result {
            Ok(lease) => {
                pending.state.generation = lease.generation;
                pending.state.lease = Some(lease.clone());
                pending.state.retry_not_before = None;
                Ok(lease)
            }
            Err(error) => {
                pending.state.last_failure = error;
                pending.state.retry_not_before =
                    Some(Instant::now() + self.options.failure_cooldown);
                Err(error)
            }
        }
    }

    fn accept(
        &self,
        response: TokenResponse,
        generation: u64,
    ) -> Result<TokenLease, TokenManagerError> {
        if response.token_type != "Bearer" || response.token.expose().trim().is_empty() {
            return Err(TokenManagerError::UnusableToken);
        }
        let expires_at = self
            .expiry
            .interpret(&response.expires_dt)
            .map_err(|_| TokenManagerError::InvalidExpiry)?;
        if !self.usable(expires_at) {
            return Err(TokenManagerError::UnusableToken);
        }
        Ok(TokenLease {
            token: response.token,
            generation: generation
                .checked_add(1)
                .ok_or(TokenManagerError::GenerationExhausted)?,
            expires_at,
            raw_expires_dt: response.expires_dt,
        })
    }

    /// 이전 작업에서 늦게 도착한 인증 실패가 새 토큰을 지우지 못하도록 세대를 확인합니다.
    /// 로컬 상태만 무효화하며 서버 토큰을 폐기하거나 요청을 재실행하지 않습니다.
    pub async fn invalidate_if_generation(&self, generation: u64) -> bool {
        let mut state = self.state.lock().await;
        if state
            .lease
            .as_ref()
            .is_some_and(|lease| lease.generation == generation)
        {
            state.lease = None;
            true
        } else {
            false
        }
    }
}
