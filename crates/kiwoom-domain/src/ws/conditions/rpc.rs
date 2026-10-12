//! 조건검색 RPC 요청 검증, 응답 계약과 서비스별 디코딩을 소유합니다.

use crate::ClientError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::BTreeMap;

/// 조건검색 RPC의 요청 서비스명입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionService {
    /// 국내 조건 목록 조회
    #[serde(rename = "CNSRLST")]
    DomesticList,

    /// 국내 조건 검색
    #[serde(rename = "CNSRREQ")]
    DomesticSearch,

    /// 국내 조건 검색 해제
    #[serde(rename = "CNSRCLR")]
    DomesticClear,

    /// 미국 조건 목록 조회
    #[serde(rename = "GCNSRLST")]
    UsList,

    /// 미국 조건 검색
    #[serde(rename = "GCNSRREQ")]
    UsSearch,

    /// 미국 조건 검색 해제
    #[serde(rename = "GCNSRCLR")]
    UsClear,
}

fn service(actual: ConditionService, expected: ConditionService) -> Result<(), ClientError> {
    if actual != expected {
        return Err(ClientError::InvalidRequest(
            "wrong condition-search service",
        ));
    }
    Ok(())
}

fn nonempty(value: &str, max: usize) -> Result<(), ClientError> {
    if value.trim().is_empty() || (max != 0 && value.len() > max) {
        return Err(ClientError::InvalidRequest(
            "invalid condition-search field length",
        ));
    }
    Ok(())
}

fn selector(value: &str, choices: &[&str]) -> Result<(), ClientError> {
    if !choices.contains(&value) {
        return Err(ClientError::InvalidRequest(
            "unsupported condition-search selector",
        ));
    }
    Ok(())
}

fn continuation(yn: Option<&str>, key: Option<&str>) -> Result<(), ClientError> {
    if let Some(yn) = yn {
        selector(yn, &["Y", "N"])?;
    }
    if let Some(key) = key
        && key.len() > 20
    {
        return Err(ClientError::InvalidRequest(
            "condition-search cursor exceeds 20",
        ));
    }
    if yn == Some("Y") {
        nonempty(key.unwrap_or_default(), 20)?;
    }
    Ok(())
}

fn encode<T: Serialize>(request: &T) -> Result<Value, ClientError> {
    serde_json::to_value(request)
        .map_err(|_| ClientError::InvalidRequest("condition-search encoding failed"))
}

macro_rules! list_request {
    ($name:ident, $expected:expr) => {
        #[doc = concat!(stringify!($name), " 조건검색 요청입니다.")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            pub trnm: ConditionService,
        }

        impl Default for $name {
            fn default() -> Self {
                Self { trnm: $expected }
            }
        }

        impl $name {
            /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
            pub fn validate(&self) -> Result<(), ClientError> {
                service(self.trnm, $expected)
            }

            /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
            pub fn encode(&self) -> Result<Value, ClientError> {
                self.validate()?;
                encode(self)
            }
        }
    };
}

list_request!(Ka10171Request, ConditionService::DomesticList);

list_request!(Usa20280Request, ConditionService::UsList);

/// 국내 일회성 조건검색 요청입니다. 연속조회 선택값을 함께 보냅니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10172Request {
    pub trnm: ConditionService,

    /// 비어 있지 않은 조건 순번: 최대 3바이트
    pub seq: String,

    /// 검색 구분: 0
    pub search_type: String,

    /// 국내 거래소 선택값 K/A
    pub stex_tp: String,

    /// 연속조회 선택값 Y/N
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cont_yn: Option<String>,

    /// 연속조회 커서
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
}

impl Ka10172Request {
    /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        service(self.trnm, ConditionService::DomesticSearch)?;
        nonempty(&self.seq, 3)?;
        selector(&self.search_type, &["0"])?;
        selector(&self.stex_tp, &["K", "A"])?;
        continuation(self.cont_yn.as_deref(), self.next_key.as_deref())
    }

    /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
    pub fn encode(&self) -> Result<Value, ClientError> {
        self.validate()?;
        encode(self)
    }
}

/// 미국 일회성 조건검색 요청입니다. 연속조회 선택값을 함께 보냅니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa20281Request {
    pub trnm: ConditionService,

    /// 비어 있지 않은 조건 순번: 최대 3바이트
    pub seq: String,

    /// 검색 구분: 0
    pub search_type: String,

    /// 연속조회 선택값 Y/N
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cont_yn: Option<String>,

    /// 연속조회 커서
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
}

impl Usa20281Request {
    /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        service(self.trnm, ConditionService::UsSearch)?;
        nonempty(&self.seq, 3)?;
        selector(&self.search_type, &["0"])?;
        continuation(self.cont_yn.as_deref(), self.next_key.as_deref())
    }

    /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
    pub fn encode(&self) -> Result<Value, ClientError> {
        self.validate()?;
        encode(self)
    }
}

/// 국내 실시간 조건검색 요청입니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10173Request {
    pub trnm: ConditionService,

    /// 비어 있지 않은 조건 순번: 최대 3바이트
    pub seq: String,

    /// 검색 구분: 1
    pub search_type: String,

    /// 국내 거래소 선택값 K/A
    pub stex_tp: String,
}

impl Ka10173Request {
    /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        service(self.trnm, ConditionService::DomesticSearch)?;
        nonempty(&self.seq, 3)?;
        selector(&self.search_type, &["1"])?;
        selector(&self.stex_tp, &["K", "A"])
    }

    /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
    pub fn encode(&self) -> Result<Value, ClientError> {
        self.validate()?;
        encode(self)
    }
}

/// 미국 실시간 조건검색 요청입니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa20290Request {
    pub trnm: ConditionService,

    /// 비어 있지 않은 조건 순번: 최대 3바이트
    pub seq: String,

    /// 검색 구분: 1
    pub search_type: String,
}

impl Usa20290Request {
    /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        service(self.trnm, ConditionService::UsSearch)?;
        nonempty(&self.seq, 3)?;
        selector(&self.search_type, &["1"])
    }

    /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
    pub fn encode(&self) -> Result<Value, ClientError> {
        self.validate()?;
        encode(self)
    }
}

macro_rules! clear_request {
    ($name:ident, $expected:expr) => {
        #[doc = concat!(stringify!($name), " 조건검색 요청입니다.")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            pub trnm: ConditionService,

            /// 조건 순번
            pub seq: String,
        }

        impl $name {
            /// 서비스명과 요청 필드의 계약 제약을 검사합니다.
            pub fn validate(&self) -> Result<(), ClientError> {
                service(self.trnm, $expected)?;
                nonempty(&self.seq, 0)
            }

            /// 요청 검증 후 원본 필드명으로 JSON을 만듭니다.
            pub fn encode(&self) -> Result<Value, ClientError> {
                self.validate()?;
                encode(self)
            }
        }
    };
}

clear_request!(Ka10174Request, ConditionService::DomesticClear);

clear_request!(Usa20291Request, ConditionService::UsClear);

macro_rules! row {
    ($name:ident { $($field:ident => $wire:literal),* $(,)? }) => {
        #[doc = concat!(stringify!($name), " 조건검색 결과 행입니다.")]
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[doc = concat!("원본 필드 `", $wire, "`의 선택적 문자열 값")]
                #[serde(rename = $wire, default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,

            )*

            /// 알 수 없는 원본 필드
            #[serde(flatten)]
            pub extra: BTreeMap<String, Value>,
        }
    }
}

row!(Ka10171Row {
    seq => "seq",
    name => "name"
});

row!(Usa20280Row {
    seq => "seq",
    name => "name"
});

row!(Ka10172Row {
    instrument => "9001",
    name => "302",
    current_price => "10",
    change_sign => "25",
    change => "11",
    change_percent => "12",
    cumulative_volume => "13",
    open => "16",
    high => "17",
    low => "18"
});

row!(Usa20281Row {
    instrument => "9001",
    name => "302",
    current_price => "10",
    change_sign => "25",
    change => "11",
    change_percent => "12",
    cumulative_volume => "13",
    open => "16",
    high => "17",
    low => "18",
    industry => "318",
    exchange => "stex_tp"
});

row!(Ka10173Row {
    jmcode => "jmcode"
});

row!(Usa20290Row {
    jmcode => "jmcode",
    stex_tp => "stexTp"
});

macro_rules! response {
    ($name:ident, $row:ident; $($field:ident),* $(,)?) => {
        #[doc = concat!(stringify!($name), " 조건검색 응답의 원본 필드를 보존합니다.")]
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            /// 원본 결과 코드
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub return_code: Option<i64>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub return_msg: Option<String>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub trnm: Option<String>,

            $(
                #[doc = concat!("선택적 원본 필드 `", stringify!($field), "`")]
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,

            )*

            /// 조건검색 결과 행
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub data: Option<Vec<$row>>,

            /// 알 수 없는 원본 필드
            #[serde(flatten)]
            pub extra: BTreeMap<String, Value>,
        }
    }
}

response!(Ka10171Response, Ka10171Row;);

response!(Usa20280Response, Usa20280Row;);

response!(Ka10172Response, Ka10172Row; seq, cont_yn, next_key);

response!(Usa20281Response, Usa20281Row; seq, cont_yn, next_key);

response!(Ka10173Response, Ka10173Row; seq);

response!(Usa20290Response, Usa20290Row; seq);

macro_rules! clear_response {
    ($name:ident) => {
        #[doc = concat!(stringify!($name), " 조건검색 해제 응답입니다.")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            /// 원본 결과 코드
            pub return_code: i64,

            pub return_msg: String,

            pub trnm: String,

            /// 조건 순번
            pub seq: String,

            /// 알 수 없는 원본 필드
            #[serde(flatten)]
            pub extra: BTreeMap<String, Value>,
        }
    };
}

clear_response!(Ka10174Response);

clear_response!(Usa20291Response);

/// 선택적 성공 필드는 선택적으로 유지합니다.
/// return_code나 trnm이 없으면 디코딩 성공만으로 구독 확인 응답의 성공을 인정하지 않습니다.
#[derive(Debug, Clone)]
pub struct ConditionFrame<T> {
    /// 타입으로 해석한 응답
    pub typed: T,

    /// 전체 원본 JSON
    pub raw: Value,
}

// RPC와 REAL 알림이 같은 결과 코드·서비스명 검사 및 원본 보존 규칙을 사용합니다.
pub(super) fn decode<T: DeserializeOwned>(
    raw: Value,
    services: &[&str],
) -> Result<ConditionFrame<T>, ClientError> {
    if let Some(code) = raw.get("return_code") {
        let code = code
            .as_i64()
            .ok_or(ClientError::Protocol("invalid condition result code"))?;
        if code != 0 {
            return Err(ClientError::Api(code));
        }
    }
    if let Some(trnm) = raw.get("trnm") {
        let trnm = trnm
            .as_str()
            .ok_or(ClientError::Protocol("invalid condition service"))?;
        if !services.contains(&trnm) {
            return Err(ClientError::Protocol("condition service mismatch"));
        }
    }
    let typed = serde_json::from_value(raw.clone())
        .map_err(|_| ClientError::Protocol("invalid condition response"))?;
    Ok(ConditionFrame { typed, raw })
}

macro_rules! decoder {
    ($method:ident, $res:ident, $services:expr) => {
        #[doc = concat!(stringify!($res), "를 디코딩하며 원본 JSON과 선택 필드 부재를 보존합니다.")]
        pub fn $method(raw: Value) -> Result<ConditionFrame<$res>, ClientError> {
            decode(raw, $services)
        }
    };
}

decoder!(decode_ka10171, Ka10171Response, &["CNSRLST"]);

// 공개 표에서 usa20280의 요청·응답 서비스명이 서로 다릅니다.
// 두 표기를 모두 허용하되 어느 쪽도 실제 서버의 표기라고 단정하지 않습니다.
decoder!(decode_usa20280, Usa20280Response, &["GCNSRLST", "GCNSRLT"]);

decoder!(decode_ka10172, Ka10172Response, &["CNSRREQ"]);

decoder!(decode_usa20281, Usa20281Response, &["GCNSRREQ"]);

decoder!(decode_ka10173, Ka10173Response, &["CNSRREQ"]);

decoder!(decode_usa20290, Usa20290Response, &["GCNSRREQ"]);

decoder!(decode_ka10174, Ka10174Response, &["CNSRCLR"]);

decoder!(decode_usa20291, Usa20291Response, &["GCNSRCLR"]);

macro_rules! session_contract {
    ($req:ident, $res:ident, $id:literal, $services:expr) => {
        impl crate::ws::request::sealed::Sealed for $req {}
        impl crate::ws::request::WsRequest for $req {
            type Response = $res;
            fn api_id(&self) -> &'static str {
                $id
            }

            fn expected_services(&self) -> &'static [&'static str] {
                $services
            }

            fn validate(&self) -> Result<(), ClientError> {
                $req::validate(self)
            }
        }
    };
}

session_contract!(Ka10171Request, Ka10171Response, "ka10171", &["CNSRLST"]);

session_contract!(Ka10172Request, Ka10172Response, "ka10172", &["CNSRREQ"]);

session_contract!(Ka10173Request, Ka10173Response, "ka10173", &["CNSRREQ"]);

session_contract!(Ka10174Request, Ka10174Response, "ka10174", &["CNSRCLR"]);

session_contract!(
    Usa20280Request,
    Usa20280Response,
    "usa20280",
    &["GCNSRLST", "GCNSRLT"]
);
session_contract!(Usa20281Request, Usa20281Response, "usa20281", &["GCNSRREQ"]);

session_contract!(Usa20290Request, Usa20290Response, "usa20290", &["GCNSRREQ"]);

session_contract!(Usa20291Request, Usa20291Response, "usa20291", &["GCNSRCLR"]);
