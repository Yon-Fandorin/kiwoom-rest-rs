//! OAuth 발급·폐기의 요청·응답과 입력 검증 계약입니다.
//! 자격 증명은 Secret으로 감싸고 전송 전에 빈 값을 검증합니다.

use crate::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TokenRequest {
    grant_type: &'static str,
    appkey: Secret,
    secretkey: Secret,
}

impl TokenRequest {
    pub fn validate(&self) -> Result<(), crate::ClientError> {
        if self.appkey.expose().trim().is_empty() || self.secretkey.expose().trim().is_empty() {
            return Err(crate::ClientError::InvalidRequest(
                "required OAuth credentials are empty",
            ));
        }
        Ok(())
    }

    pub fn new(appkey: Secret, secretkey: Secret) -> Self {
        Self {
            grant_type: "client_credentials",
            appkey,
            secretkey,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    /// 시간대 해석을 적용하지 않은 증권사 만료 시각 원문입니다.
    pub expires_dt: String,

    pub token_type: String,
    pub token: Secret,
}

#[derive(Debug, Serialize)]
pub struct RevokeRequest {
    appkey: Secret,
    secretkey: Secret,
    token: Secret,
}

impl RevokeRequest {
    pub fn validate(&self) -> Result<(), crate::ClientError> {
        if [&self.appkey, &self.secretkey, &self.token]
            .into_iter()
            .any(|value| value.expose().trim().is_empty())
        {
            return Err(crate::ClientError::InvalidRequest(
                "required OAuth credentials are empty",
            ));
        }
        Ok(())
    }

    pub fn new(appkey: Secret, secretkey: Secret, token: Secret) -> Self {
        Self {
            appkey,
            secretkey,
            token,
        }
    }
}

/// 공개 토큰 폐기 가이드에는 추가 응답 필드가 없습니다.
#[derive(Debug, Deserialize)]
pub struct RevokeResponse {
    pub return_code: i64,
}
