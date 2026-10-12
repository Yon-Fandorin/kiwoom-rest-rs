//! 조건검색 REAL 알림의 필수 FID와 선택값을 검증합니다.

use super::rpc::{ConditionFrame, decode};
use crate::ClientError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// 국내·미국 조건 검색 페이지에 공통으로 명시된 REAL 값입니다.
/// 가격·수량·거래 의미를 추론하지 않고 문자열을 보존합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRealtimeValues {
    /// 조건 순번: FID 841
    #[serde(rename = "841")]
    pub seq: String,

    /// 종목 코드: FID 9001
    #[serde(rename = "9001")]
    pub instrument: String,

    /// 편입 I 또는 이탈 D: FID 843
    #[serde(rename = "843")]
    pub change: String,

    /// 원본 체결 시간: FID 20
    #[serde(rename = "20")]
    pub execution_time: String,

    /// 원본 매도·매수 구분: FID 907
    #[serde(rename = "907")]
    pub sell_buy_type: String,

    /// 원본 거래소 구분: FID 9081
    #[serde(rename = "9081")]
    pub exchange: String,

    /// 알 수 없는 원본 필드
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// 조건검색 REAL 알림의 개별 항목입니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRealtimeItem {
    /// 원본 실시간 타입: 2바이트
    #[serde(rename = "type")]
    pub api_type: String,

    /// 원본 실시간 항목명
    pub name: String,

    /// 필수 FID 값
    pub values: ConditionRealtimeValues,

    /// 알 수 없는 원본 필드
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// 조건검색 REAL 알림의 전체 응답입니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRealtimeResponse {
    /// 원본 서비스명 REAL
    pub trnm: String,

    /// 조건검색 알림 항목
    pub data: Vec<ConditionRealtimeItem>,

    /// 알 수 없는 원본 필드
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// REAL 서비스와 필수 FID를 디코딩한 뒤 타입의 바이트 길이와 I/D 선택값을 검사합니다.
/// 원본 JSON을 보존하며 특정 실시간 타입이나 구독 등록을 추론하지 않습니다.
pub fn decode_condition_realtime(
    raw: Value,
) -> Result<ConditionFrame<ConditionRealtimeResponse>, ClientError> {
    let frame: ConditionFrame<ConditionRealtimeResponse> = decode(raw, &["REAL"])?;
    for item in &frame.typed.data {
        if item.api_type.len() != 2 || !matches!(item.values.change.as_str(), "I" | "D") {
            return Err(ClientError::Protocol(
                "invalid documented condition realtime selector",
            ));
        }
    }
    Ok(frame)
}
