//! 등록·해제 요청의 항목 형식과 그룹 검증을 함께 소유합니다.
//! 요청 메타데이터와 확인 응답은 세션 및 복구 계층이 사용할 수 있습니다.

use super::realtime::RealtimeType;
use crate::ClientError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// 그룹에 항목을 등록하거나 등록된 항목을 해제하는 서비스 선택입니다.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum SubscriptionAction {
    /// `REG` 등록 요청입니다.
    #[serde(rename = "REG")]
    Register,

    /// `REMOVE` 해제 요청입니다.
    #[serde(rename = "REMOVE")]
    Remove,
}

/// 그룹의 기존 항목을 교체하거나 유지하는 요청 선택입니다.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RefreshMode {
    /// `refresh="0"`으로 기존 그룹 항목을 교체합니다.
    #[serde(rename = "0")]
    ReplaceGroup,

    /// `refresh="1"`으로 기존 그룹 항목을 유지합니다.
    #[serde(rename = "1")]
    KeepGroup,
}

/// 미국 종목 객체의 `stex_tp`에 사용하는 공개 거래소 코드입니다.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UsExchange {
    /// 나스닥의 공개 코드 `ND`입니다.
    #[serde(rename = "ND")]
    Nasdaq,

    /// 뉴욕증권거래소의 공개 코드 `NY`입니다.
    #[serde(rename = "NY")]
    Nyse,

    /// 아멕스의 공개 코드 `NA`입니다.
    #[serde(rename = "NA")]
    Amex,
}

/// 미국 등록 요청의 종목 코드와 거래소 객체입니다. 입력 종목 코드를 그대로 보존합니다.
#[derive(Debug, Clone, Serialize)]
pub struct UsInstrument {
    jmcode: String,
    stex_tp: UsExchange,
}

impl UsInstrument {
    /// 공백뿐인 값과 제어 문자를 거부하고 종목 코드·거래소를 구성합니다.
    pub fn new(ticker: &str, exchange: UsExchange) -> Result<Self, ClientError> {
        if ticker.trim().is_empty() || ticker.chars().any(char::is_control) {
            return Err(ClientError::InvalidRequest("invalid realtime ticker"));
        }
        Ok(Self {
            jmcode: ticker.into(),
            stex_tp: exchange,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum RegistrationElements {
    Single(String),
    All(Vec<String>),
    Us(Vec<UsInstrument>),
}

/// 피드별로 검증된 항목 형식과 단일 피드 ID를 묶은 등록 항목입니다.
#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<RegistrationElements>,
    #[serde(rename = "type")]
    types: Vec<RealtimeType>,
}

impl SubscriptionEntry {
    /// 국내 필드 표의 단일 String 형식입니다.
    /// 0B 또는 String으로 명시된 다른 피드에서 여러 종목 배열을 허용한다고 단정하지 않습니다.
    pub fn domestic(api: RealtimeType, item: Option<&str>) -> Result<Self, ClientError> {
        if matches!(
            api,
            RealtimeType::DomesticOrder
                | RealtimeType::MarketTime
                | RealtimeType::VolatilityInterruption
                | RealtimeType::UsOrderConfirmation
                | RealtimeType::UsExecution
                | RealtimeType::UsTradePrice
                | RealtimeType::UsOrderBook
        ) {
            return Err(ClientError::InvalidRequest(
                "realtime type requires a different item shape",
            ));
        }
        if let Some(v) = item {
            let max = if matches!(
                api,
                RealtimeType::StockInformation | RealtimeType::ElwIndicator
            ) {
                100
            } else {
                200
            };
            if v.trim().is_empty() || v.len() > max || v.chars().any(char::is_control) {
                return Err(ClientError::InvalidRequest("invalid realtime item length"));
            }
            if matches!(
                api,
                RealtimeType::ElwTheoretical | RealtimeType::ElwIndicator
            ) && (v.len() != 6 || !v.bytes().all(|b| b.is_ascii_alphanumeric()))
            {
                return Err(ClientError::InvalidRequest(
                    "ELW code must contain six characters",
                ));
            }
            if api == RealtimeType::IndustryIndex
                && (v.len() != 3 || !v.bytes().all(|b| b.is_ascii_digit()))
            {
                return Err(ClientError::InvalidRequest(
                    "industry code must contain three digits",
                ));
            }
            if api == RealtimeType::InternationalGold && !["MGD", "MGU"].contains(&v) {
                return Err(ClientError::InvalidRequest("undocumented gold item"));
            }
        }
        Ok(Self {
            item: item.map(|v| RegistrationElements::Single(v.into())),
            types: vec![api],
        })
    }

    /// 00, 0s, 1h, F4, F5의 설명에는 [""] 형식이 명시되어 있습니다.
    pub fn all_instruments(api: RealtimeType) -> Result<Self, ClientError> {
        if !matches!(
            api,
            RealtimeType::DomesticOrder
                | RealtimeType::MarketTime
                | RealtimeType::VolatilityInterruption
                | RealtimeType::UsOrderConfirmation
                | RealtimeType::UsExecution
        ) {
            return Err(ClientError::InvalidRequest(
                "realtime type has no verified all-instrument shape",
            ));
        }
        Ok(Self {
            item: Some(RegistrationElements::All(vec![String::new()])),
            types: vec![api],
        })
    }

    /// FE·FT 피드의 비어 있지 않은 미국 종목 객체 배열을 구성합니다.
    pub fn us_market(api: RealtimeType, items: Vec<UsInstrument>) -> Result<Self, ClientError> {
        if !matches!(api, RealtimeType::UsTradePrice | RealtimeType::UsOrderBook)
            || items.is_empty()
        {
            return Err(ClientError::InvalidRequest(
                "invalid US realtime item shape",
            ));
        }
        Ok(Self {
            item: Some(RegistrationElements::Us(items)),
            types: vec![api],
        })
    }
}

/// 그룹·항목 형식을 검증한 REG/REMOVE 요청입니다. 생성은 전송을 수행하지 않습니다.
#[derive(Debug, Clone, Serialize)]
pub struct RealtimeSubscription {
    trnm: SubscriptionAction,
    grp_no: String,
    refresh: RefreshMode,
    data: Vec<SubscriptionEntry>,
}

impl RealtimeSubscription {
    /// 복구용 읽기 전용 메타데이터입니다.
    /// 검증은 생성자가 담당합니다.
    pub fn action(&self) -> SubscriptionAction {
        self.trnm
    }

    /// 요청에 입력한 숫자 그룹 식별자입니다.
    pub fn group(&self) -> &str {
        &self.grp_no
    }

    /// 요청에 입력한 기존 그룹 교체·유지 선택입니다.
    pub fn refresh_mode(&self) -> RefreshMode {
        self.refresh
    }

    /// 각 항목은 문서에 명시된 형식을 보존합니다.
    /// 미국 객체는 서로 다른 형식을 합치지 않고 특정 종목을 정확히 제거할 수 있도록 나눕니다.
    pub fn atomic_entries(&self) -> Vec<SubscriptionEntry> {
        self.data
            .iter()
            .flat_map(|entry| match &entry.item {
                Some(RegistrationElements::Us(items)) => items
                    .iter()
                    .map(|item| SubscriptionEntry {
                        item: Some(RegistrationElements::Us(vec![item.clone()])),
                        types: entry.types.clone(),
                    })
                    .collect(),
                _ => vec![entry.clone()],
            })
            .collect()
    }

    /// 비어 있지 않은 숫자 그룹과 항목 목록을 검증합니다.
    /// 국내 피드가 포함되면 그룹 길이를 네 바이트로 제한합니다.
    pub fn new(
        action: SubscriptionAction,
        group: &str,
        refresh: RefreshMode,
        data: Vec<SubscriptionEntry>,
    ) -> Result<Self, ClientError> {
        if group.is_empty() || !group.bytes().all(|b| b.is_ascii_digit()) || data.is_empty() {
            return Err(ClientError::InvalidRequest("invalid realtime group/data"));
        }
        // 국내 표는 max4를 명시하지만 미국 표는 길이를 명시하지 않습니다.
        if group.len() > 4
            && data.iter().flat_map(|e| e.types.iter()).any(|api| {
                !matches!(
                    api,
                    RealtimeType::UsOrderConfirmation
                        | RealtimeType::UsExecution
                        | RealtimeType::UsTradePrice
                        | RealtimeType::UsOrderBook
                )
            })
        {
            return Err(ClientError::InvalidRequest(
                "domestic realtime group exceeds length four",
            ));
        }
        Ok(Self {
            trnm: action,
            grp_no: group.into(),
            refresh,
            data,
        })
    }
}

/// REG/REMOVE 확인 응답 필드입니다.
/// 결과 코드가 없으면 타입 디코딩이 실패합니다.
#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationAcknowledgement {
    /// 필수 증권사 결과 코드입니다. 0의 확인 여부는 세션·복구 계층이 판단합니다.
    pub return_code: i64,

    /// 증권사가 반환한 서비스 이름입니다.
    pub trnm: String,

    /// 증권사의 선택적 결과 메시지입니다.
    pub return_msg: Option<String>,

    /// 알려진 필드 이외의 확인 응답 JSON 값입니다.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl crate::ws::request::sealed::Sealed for RealtimeSubscription {}
impl crate::ws::request::WsRequest for RealtimeSubscription {
    type Response = RegistrationAcknowledgement;
    fn api_id(&self) -> &'static str {
        self.data
            .first()
            .and_then(|e| e.types.first())
            .map_or("", |api| api.api_id())
    }

    fn expected_services(&self) -> &'static [&'static str] {
        match self.trnm {
            SubscriptionAction::Register => &["REG"],
            SubscriptionAction::Remove => &["REMOVE"],
        }
    }

    fn validate(&self) -> Result<(), ClientError> {
        Self::new(self.trnm, &self.grp_no, self.refresh, self.data.clone()).map(|_| ())
    }
}
