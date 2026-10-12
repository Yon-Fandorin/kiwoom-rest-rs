//! 전송·세션 상태를 참조하지 않는 봉인된 요청 메타데이터 계약.

use crate::ClientError;
use serde::{Serialize, de::DeserializeOwned};

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// 세션에서 호출할 수 있는 봉인된 typed 요청 계약.
pub trait WsRequest: sealed::Sealed + Serialize {
    /// 해당 요청의 typed 응답.
    type Response: DeserializeOwned;

    /// 등록된 공개 계약의 식별자.
    fn api_id(&self) -> &'static str;

    /// 응답으로 받을 수 있는 서비스 이름.
    fn expected_services(&self) -> &'static [&'static str];

    /// 전송 전에 확인할 요청 제약.
    fn validate(&self) -> Result<(), ClientError>;
}
