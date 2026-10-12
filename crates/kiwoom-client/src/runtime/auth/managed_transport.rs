//! 선택적으로 사용하는 메모리 기반 인증 래퍼입니다.
//! 재실행 가능 여부는 HTTP 메서드가 아닌 경로 분류로 결정하며, 모든 금융 요청은 최대 한 번
//! 실행합니다.

use super::token_manager::TokenManager;
use crate::{
    ClientError,
    runtime::http::{HttpRequest, HttpResponse, Transport, TransportError},
};
use async_trait::async_trait;
use std::{fmt, sync::Arc};

/// HTTP401은 인증 실패의 근거지만 키움 토큰의 만료를 증명하지는 않습니다.
/// 호출자가 이 정책을 명시적으로 선택하며, 증권사 업무 코드는 변경하지 않습니다.
#[derive(Debug, Clone, Copy, Default)]
pub enum AuthenticationRetry {
    #[default]
    Never,
    SafeReadHttp401Once,
}

pub struct ManagedTransport<T> {
    inner: T,
    tokens: Arc<TokenManager>,
    retry: AuthenticationRetry,
}

impl<T> ManagedTransport<T> {
    pub fn new(inner: T, tokens: Arc<TokenManager>, retry: AuthenticationRetry) -> Self {
        Self {
            inner,
            tokens,
            retry,
        }
    }
}

impl<T> fmt::Debug for ManagedTransport<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedTransport")
            .field("retry", &self.retry)
            .field("inner", &"[REDACTED]")
            .field("tokens", &"[REDACTED]")
            .finish()
    }
}

#[async_trait]
impl<T: Transport> Transport for ManagedTransport<T> {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.execute_checked(request)
            .await
            .map_err(|_| TransportError)
    }

    async fn execute_checked(&self, mut request: HttpRequest) -> Result<HttpResponse, ClientError> {
        request.validate_dispatch()?;
        let financial = crate::registry::financial_write_pair(request.api_id, request.path);
        // 발급·폐기는 별도의 명시적 작업입니다.
        // 토큰을 재귀적으로 얻거나 재시도하지 않고, 전달받은 본문·토큰도 바꾸지 않습니다.
        if request.api_id.starts_with("au") {
            return self.inner.execute_checked(request).await;
        }
        let safe_read = crate::registry::read_only_pair(request.api_id, request.path);
        if !financial && !safe_read {
            return Err(ClientError::UnsupportedApi);
        }
        let lease = self
            .tokens
            .lease()
            .await
            .map_err(|_| ClientError::Transport)?;
        request.authorization = Some(lease.token().clone());
        let response = self
            .inner
            .execute_checked(request.clone())
            .await?
            .normalized()?;
        if response.status != 401 {
            return Ok(response);
        }
        self.tokens
            .invalidate_if_generation(lease.generation())
            .await;
        if financial || matches!(self.retry, AuthenticationRetry::Never) {
            // 금융 변경 요청의 인증 실패 응답은 그대로 반환합니다.
            // 경제적 결과는 호출자가 대사해야 하며, 요청을 재실행하지 않습니다.
            return Ok(response);
        }
        let renewed = self
            .tokens
            .lease()
            .await
            .map_err(|_| ClientError::Transport)?;
        request.authorization = Some(renewed.token().clone());
        let response = self.inner.execute_checked(request).await?.normalized()?;
        if response.status == 401 {
            self.tokens
                .invalidate_if_generation(renewed.generation())
                .await;
        }
        Ok(response)
    }
}
