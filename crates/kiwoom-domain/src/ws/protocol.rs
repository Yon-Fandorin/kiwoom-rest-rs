//! 오프라인 프로토콜 경계만 정의하며 소켓·인증·재연결·재실행은 구현하지 않습니다.
//! 0B 가이드의 REG/REMOVE/REAL과 FID 중 일부만 지원합니다.
//! 기존 단일 종목·필수 응답 형식을 유지하는 호환 뷰이며, 전체 typed 계약은 `realtime`이 소유합니다.

use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PRODUCTION_URL: &str = "wss://api.kiwoom.com:10000/api/websocket";
#[derive(Debug, Serialize)]
pub enum RegistrationAction {
    #[serde(rename = "REG")]
    Register,
    #[serde(rename = "REMOVE")]
    Remove,
}

#[derive(Debug, Serialize)]
pub enum RegistrationMode {
    #[serde(rename = "0")]
    ReplaceGroup,
    #[serde(rename = "1")]
    KeepGroup,
}

#[derive(Debug, Serialize)]
pub struct StockExecutionSubscription {
    trnm: RegistrationAction,
    grp_no: String,
    refresh: RegistrationMode,
    data: Vec<RegistrationItem>,
}

#[derive(Debug, Serialize)]
struct RegistrationItem {
    item: String,
    #[serde(rename = "type")]
    types: Vec<&'static str>,
}

impl StockExecutionSubscription {
    /// 공개 표의 String 항목 형식을 따릅니다.
    /// 여러 항목을 담는 전송 형식은 실제 환경 검증이 필요합니다.
    pub fn new(
        action: RegistrationAction,
        group: &str,
        mode: RegistrationMode,
        instrument: &str,
    ) -> Result<Self, ClientError> {
        if group.is_empty()
            || group.len() > 4
            || !group.bytes().all(|b| b.is_ascii_digit())
            || instrument.is_empty()
            || instrument.len() > 200
        {
            return Err(ClientError::InvalidRequest("invalid WebSocket group/item"));
        }
        Ok(Self {
            trnm: action,
            grp_no: group.into(),
            refresh: mode,
            data: vec![RegistrationItem {
                item: instrument.into(),
                types: vec!["0B"],
            }],
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StockExecutionValues {
    #[serde(rename = "20")]
    pub trade_time: Option<String>,
    #[serde(rename = "10")]
    pub current_price: Option<String>,
    #[serde(rename = "15")]
    pub signed_trade_quantity: Option<String>,
    #[serde(flatten)]
    pub other_fids: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RealtimeItem {
    #[serde(rename = "type")]
    pub api_id: String,
    pub name: Option<String>,
    pub item: Option<String>,
    pub values: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Acknowledgement { action: String, code: i64 },
    Realtime { items: Vec<RealtimeItem> },
    Unknown { service: String },
}

pub fn decode_message(raw: &str) -> Result<ServerMessage, ClientError> {
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|_| ClientError::Protocol("invalid WebSocket JSON"))?;
    let service = value
        .get("trnm")
        .and_then(|v| v.as_str())
        .ok_or(ClientError::Protocol("missing WebSocket service"))?;
    match service {
        "REG" | "REMOVE" => {
            let code = value
                .get("return_code")
                .and_then(|v| v.as_i64())
                .ok_or(ClientError::Protocol("missing acknowledgement code"))?;
            if code != 0 {
                return Err(ClientError::Api(code));
            }
            Ok(ServerMessage::Acknowledgement {
                action: service.into(),
                code,
            })
        }
        "REAL" => {
            // 호환 뷰에서도 명시된 실패 코드를 성공 데이터로 반환하지 않습니다.
            if let Some(code) = value.get("return_code") {
                let code = code
                    .as_i64()
                    .ok_or(ClientError::Protocol("invalid realtime return_code"))?;
                if code != 0 {
                    return Err(ClientError::Api(code));
                }
            }
            let data = value
                .get("data")
                .ok_or(ClientError::Protocol("missing realtime data"))?;
            let array = data
                .as_array()
                .ok_or(ClientError::Protocol("invalid realtime items"))?;
            if array.iter().any(|item| !item.is_object()) {
                return Err(ClientError::Protocol("invalid realtime envelope"));
            }
            let items = serde_json::from_value(data.clone())
                .map_err(|_| ClientError::Protocol("invalid realtime items"))?;
            Ok(ServerMessage::Realtime { items })
        }
        _ => Ok(ServerMessage::Unknown {
            service: service.into(),
        }),
    }
}

impl RealtimeItem {
    pub fn stock_execution(&self) -> Result<StockExecutionValues, ClientError> {
        if self.api_id != "0B" {
            return Err(ClientError::UnsupportedApi);
        }
        serde_json::from_value(self.values.clone())
            .map_err(|_| ClientError::Protocol("invalid stock execution fields"))
    }
}
