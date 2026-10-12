//! HTTP 요청·응답 계약, 비밀 값과 금융 실행 정책을 정의하는 전송 경계.

mod client;
mod reqwest;

pub(crate) use client::decode;
pub use client::{Client, ClientOptions};
pub use reqwest::ReqwestTransport;

use async_trait::async_trait;
pub use kiwoom_core::{ClientError, Environment, Page, Secret};
use serde_json::Value;
use std::{collections::BTreeMap, fmt};

/// 금융 변경 실행 정책. 기본값은 실행 차단입니다.
#[derive(Debug, Clone, Copy, Default)]
pub enum WritePolicy {
    /// 전송 전에 금융 변경을 차단함.
    #[default]
    Deny,

    /// 호출자가 금융 변경을 허용함. 자동 재시도는 허용하지 않음.
    ExplicitOptIn,
}

impl WritePolicy {
    /// 명시적 실행 허용을 확인하고 차단 정책이면 오류를 반환합니다.
    pub fn require(self) -> Result<(), ClientError> {
        match self {
            Self::Deny => Err(ClientError::WritesDisabled),
            Self::ExplicitOptIn => Ok(()),
        }
    }
}

/// 주입된 전송 포트에 전달할 HTTP 요청과 비공개 금융 실행 허용 표시.
#[derive(Clone)]
pub struct HttpRequest {
    /// 허용 목록에서 확인할 API 경로.
    pub path: &'static str,

    /// `api-id` 헤더에 사용할 API 식별자.
    pub api_id: &'static str,

    /// 호출자가 제공한 Bearer 토큰.
    pub authorization: Option<Secret>,

    /// 후속 페이지의 `next-key` 값. 있으면 `cont-yn: Y`를 보냄.
    pub continuation: Option<String>,

    /// 브로커에 직렬화할 JSON 본문.
    pub body: Value,

    // 명시적 쓰기 정책을 통과한 클라이언트만 설정하며 외부 생성자는 허용하지 않습니다.
    financial_write_authorized: bool,
}

impl HttpRequest {
    pub(crate) fn validate_dispatch(&self) -> Result<(), ClientError> {
        if !crate::registry::implemented_pair(self.api_id, self.path) {
            return Err(ClientError::UnsupportedApi);
        }
        if crate::registry::financial_write_pair(self.api_id, self.path)
            && !self.financial_write_authorized
        {
            return Err(ClientError::WritesDisabled);
        }
        Ok(())
    }

    /// mock 포트 등에 사용할 요청을 생성합니다. 금융 실행 허용 표시는 설정할 수 없습니다.
    pub fn read_only(
        path: &'static str,
        api_id: &'static str,
        authorization: Option<Secret>,
        body: Value,
    ) -> Self {
        Self {
            path,
            api_id,
            authorization,
            continuation: None,
            body,
            financial_write_authorized: false,
        }
    }
}

impl fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpRequest")
            .field("path", &self.path)
            .field("api_id", &self.api_id)
            .field("body", &"[REDACTED]")
            .field("authorization", &"[REDACTED]")
            .field("continuation", &"[REDACTED]")
            .finish()
    }
}

/// HTTP 상태, 응답 헤더와 원본 JSON을 보존한 전송 결과.
#[derive(Clone)]
pub struct HttpResponse {
    /// 서버가 반환한 HTTP 상태 코드.
    pub status: u16,

    /// 연속 조회와 재시도 판단에 사용할 응답 헤더.
    pub headers: BTreeMap<String, String>,

    /// 전송 포트가 반환한 JSON 본문. Reqwest의 HTTP 오류 응답에서는 `Null`.
    pub body: Value,
}

// 연속 조회·상관관계·재시도에 사용하는 헤더는 단일 값이어야 합니다.
const CONTROL_HEADERS: &[&str] = &["cont-yn", "next-key", "api-id", "retry-after"];

impl HttpResponse {
    pub(crate) fn normalized(mut self) -> Result<Self, ClientError> {
        let mut headers = BTreeMap::new();
        for (name, value) in self.headers {
            let name = name.to_ascii_lowercase();
            if CONTROL_HEADERS.contains(&name.as_str()) {
                if !value.is_ascii() || value.bytes().any(|b| b < 32 || b == 127) {
                    return Err(ClientError::Protocol("invalid HTTP control header"));
                }
                if headers.contains_key(&name) {
                    return Err(ClientError::Protocol("duplicate HTTP control header"));
                }
            }
            headers.insert(name, value);
        }
        self.headers = headers;
        Ok(self)
    }
}

/// 자격 증명이나 원본 오류 본문을 포함하지 않는 전송 실패.
#[derive(Debug, Clone, Copy)]
pub struct TransportError;

/// 클라이언트가 주입받는 비동기 HTTP 전송 포트.
#[async_trait]
pub trait Transport: Send + Sync {
    /// 요청을 한 번 전송합니다. 전송 오류나 취소만으로 서버 처리 여부를 확정할 수 없습니다.
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError>;

    /// 전송 실패와 재시도하지 않을 응답 계약 오류를 구분합니다.
    /// 기존 구현은 기본 메서드로 헤더 검증을 적용하며 구현을 추가할 필요가 없습니다.
    async fn execute_checked(&self, request: HttpRequest) -> Result<HttpResponse, ClientError> {
        self.execute(request)
            .await
            .map_err(|_| ClientError::Transport)?
            .normalized()
    }
}
