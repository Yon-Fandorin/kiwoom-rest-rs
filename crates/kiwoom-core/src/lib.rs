//! REST·WS가 공유하는 값과 오류입니다. 네트워크나 실행 상태를 소유하지 않습니다.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt};

/// 전송 시 직렬화할 수 있으며 `Debug` 출력에서는 값을 가리는 비밀 문자열.
///
/// 직렬화와 [`Secret::expose`]는 원문을 반환하므로 호출자가 출력·보관 범위를 관리해야 합니다.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Secret(String);

impl Secret {
    /// 호출자가 제공한 비밀 문자열을 보관합니다.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// 인증 헤더 등 명시적인 사용을 위해 비밀 원문을 빌립니다.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

/// 요청 검증, 전송, 브로커 응답과 실행 정책에서 발생한 오류.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// 요청 입력 또는 클라이언트 제한이 유효하지 않음.
    #[error("invalid request: {0}")]
    InvalidRequest(&'static str),

    /// 전송 실패. 금융 변경의 서버 처리 여부는 이 오류만으로 판단할 수 없음.
    #[error("HTTP transport failure")]
    Transport,

    /// HTTP 실패 상태와 해석 가능한 서버 재시도 대기 시간.
    #[error("HTTP status {status}; retry-after seconds: {retry_after:?}")]
    Http {
        /// 서버가 반환한 HTTP 상태 코드.
        status: u16,

        /// `Retry-After`를 초 단위로 해석한 값.
        retry_after: Option<u64>,
    },

    /// 성공 HTTP 응답에 포함된 0이 아닌 브로커 결과 코드.
    #[error("broker returned code {0}")]
    Api(i64),

    /// 응답 형식이나 연속 조회 증거가 계약에 맞지 않음.
    #[error("invalid broker response: {0}")]
    Protocol(&'static str),

    /// 페이지 상한에 도달하여 전체 결과로 반환할 수 없음.
    #[error("pagination exceeded configured page limit")]
    PageLimit,

    /// 호출자가 금융 변경 실행을 명시적으로 허용하지 않음.
    #[error("financial mutations are disabled")]
    WritesDisabled,

    /// 검증된 typed 계약이나 정확한 API·경로 조합이 없음.
    #[error("unsupported API: no verified typed contract")]
    UnsupportedApi,
}

/// 명시적으로 연결을 허용할 키움 API 환경.
#[derive(Debug, Clone, Copy)]
pub enum Environment {
    /// 실계좌 API 서버.
    Production,

    /// 모의투자 API 서버.
    Sandbox,
}

/// 타입으로 해석한 응답과 원본 JSON·연속 조회 헤더를 함께 보존합니다.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub typed: T,
    pub raw: serde_json::Value,

    /// 문서에 명시된 페이지네이션·API 헤더만 반환된 그대로 보존합니다.
    pub response_headers: BTreeMap<String, String>,
}
