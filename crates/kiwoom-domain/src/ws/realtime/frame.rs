//! REAL 프레임의 봉투를 검증하고 항목·프레임 원문을 보존합니다.
//! FID 값의 계약 선택은 페이로드 소유자에게 위임합니다.

use super::payloads::{RealtimePayload, decode_payload};
use crate::ClientError;
use serde::Deserialize;
use serde_json::Value;

/// 단일 REAL 항목의 선택적 메타데이터, FID 페이로드와 원문 JSON입니다.
#[derive(Debug, Clone)]
pub struct RealtimeItem {
    /// 선택적 피드 ID입니다. 알 수 없는 ID도 원문 그대로 보존합니다.
    pub api_id: Option<String>,

    /// 증권사가 반환한 선택적 피드 이름입니다.
    pub name: Option<String>,

    /// 증권사가 반환한 선택적 종목 문자열입니다.
    pub item: Option<String>,

    /// 공개 응답의 `stexTp` 표기를 그대로 사용합니다.
    /// 요청의 `stex_tp`와는 다릅니다.
    pub stex_tp: Option<String>,

    /// 존재하는 값만 디코딩합니다. 누락되거나 null인 값은 `None`입니다.
    pub values: Option<RealtimePayload>,

    /// 알려진 필드와 알 수 없는 필드를 모두 포함하는 항목 JSON입니다.
    pub raw: Value,
}

/// REAL 프레임의 항목과 전체 JSON·입력 텍스트입니다.
#[derive(Debug, Clone)]
pub struct RealtimeFrame {
    /// 누락되거나 null인 `data`는 `None`이며 빈 배열은 빈 목록입니다.
    pub data: Option<Vec<RealtimeItem>>,

    /// 프레임 전체 JSON 값입니다. 원문 공백·키 순서는 `raw_text`에서 확인합니다.
    pub raw: Value,

    /// JSON 공백과 키 순서를 포함하여 전달받은 원문 텍스트입니다.
    pub raw_text: String,
}

#[derive(Deserialize)]
struct ItemEnvelope {
    #[serde(rename = "type")]
    api_id: Option<String>,
    name: Option<String>,
    item: Option<String>,
    #[serde(rename = "stexTp")]
    stex_tp: Option<String>,
    values: Option<Value>,
}

/// REAL 데이터만 디코딩합니다.
/// 세션의 LOGIN·PING·확인 응답 처리는 WebSocket 계층이 담당합니다.
pub fn decode_realtime(raw: &str) -> Result<RealtimeFrame, ClientError> {
    let value: Value =
        serde_json::from_str(raw).map_err(|_| ClientError::Protocol("invalid realtime JSON"))?;
    if value.get("trnm").and_then(Value::as_str) != Some("REAL") {
        return Err(ClientError::Protocol("expected REAL service"));
    }
    if let Some(code) = value.get("return_code") {
        let code = code
            .as_i64()
            .ok_or(ClientError::Protocol("invalid realtime return_code"))?;
        if code != 0 {
            return Err(ClientError::Api(code));
        }
    }
    let data = match value.get("data") {
        None | Some(Value::Null) => None,
        Some(v) => {
            let array = v
                .as_array()
                .ok_or(ClientError::Protocol("invalid realtime data array"))?;
            let mut result = Vec::with_capacity(array.len());
            for raw_item in array {
                if !raw_item.is_object() {
                    return Err(ClientError::Protocol("invalid realtime envelope"));
                }
                let e: ItemEnvelope = serde_json::from_value(raw_item.clone())
                    .map_err(|_| ClientError::Protocol("invalid realtime envelope"))?;
                let values = e
                    .values
                    .map(|v| decode_payload(e.api_id.as_deref(), v))
                    .transpose()?;
                result.push(RealtimeItem {
                    api_id: e.api_id,
                    name: e.name,
                    item: e.item,
                    stex_tp: e.stex_tp,
                    values,
                    raw: raw_item.clone(),
                });
            }
            Some(result)
        }
    };
    Ok(RealtimeFrame {
        data,
        raw: value,
        raw_text: raw.into(),
    })
}
