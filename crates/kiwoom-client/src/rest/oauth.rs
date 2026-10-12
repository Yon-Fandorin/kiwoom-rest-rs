//! auth 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::auth::token_manager::{TokenIssueError, TokenIssuer};
use crate::runtime::http::Transport;
use crate::runtime::http::decode;
use async_trait::async_trait;
use kiwoom_core::ClientError;
use kiwoom_domain::auth::*;
use std::fmt;

impl<T: Transport> Client<T> {
    /// OAuth 토큰을 한 번 발급 요청하고 Bearer 형식과 비어 있지 않은 토큰을 확인합니다.
    pub async fn token(&self, r: &TokenRequest) -> Result<TokenResponse, ClientError> {
        r.validate()?;

        let response = self
            .send(Self::request("/oauth2/token", "au10001", None, r)?, false)
            .await?;
        let token: TokenResponse = decode(&response.body)?;
        if token.token_type != "Bearer" || token.token.expose().is_empty() {
            return Err(ClientError::Protocol("invalid token type or empty token"));
        }

        Ok(token)
    }
}

impl<T: Transport> Client<T> {
    /// OAuth 토큰 폐기를 한 번 요청합니다. 전송 실패 후 자동으로 다시 제출하지 않습니다.
    pub async fn revoke(&self, r: &RevokeRequest) -> Result<RevokeResponse, ClientError> {
        r.validate()?;

        let response = self
            .send(Self::request("/oauth2/revoke", "au10002", None, r)?, false)
            .await?;
        decode(&response.body)
    }
}

/// au10001의 client_credentials만 사용하며, 발급마다 기존 클라이언트 작업을 한 번 호출합니다.
pub struct ClientCredentialsIssuer<T> {
    client: Client<T>,
    request: TokenRequest,
}

impl<T: Transport> ClientCredentialsIssuer<T> {
    pub fn new(client: Client<T>, request: TokenRequest) -> Self {
        Self { client, request }
    }
}

impl<T> fmt::Debug for ClientCredentialsIssuer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClientCredentialsIssuer([REDACTED])")
    }
}

#[async_trait]
impl<T: Transport> TokenIssuer for ClientCredentialsIssuer<T> {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError> {
        self.client
            .token(&self.request)
            .await
            .map_err(|_| TokenIssueError)
    }
}
