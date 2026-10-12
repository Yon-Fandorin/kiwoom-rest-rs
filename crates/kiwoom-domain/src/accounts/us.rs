//! 미국주식 계좌 조회 계약과 검증, 타입별 계약 검증.
//! 각 API의 요청·응답·목록 행과 순수 검증을 함께 유지합니다.

use super::validation::{validate_field, validate_range};
use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// `usa21670` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21670>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21670Request {
    /// `YYYYMMDD` 형식의 유효한 날짜.
    pub from: String,

    /// `YYYYMMDD` 형식의 유효한 날짜.
    pub to: String,
}

impl Usa21670Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "day", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "day", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21670` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21670Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21670Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21670` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21670Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acum_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dast: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exrt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21680` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21680>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21680Request {
    /// `YYYYMM` 형식의 유효한 월.
    pub from: String,

    /// `YYYYMM` 형식의 유효한 월.
    pub to: String,
}

impl Usa21680Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "month", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "month", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21680` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21680Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21680Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21680` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21680Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acum_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dast: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exrt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21690` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21690>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21690Request {
    /// `0000`을 제외한 네 자리 연도.
    pub from: String,

    /// `0000`을 제외한 네 자리 연도.
    pub to: String,
}

impl Usa21690Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "year", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "year", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21690` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21690Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21690Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21690` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21690Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acum_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dast: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exrt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21730` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21730>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21730Request {
    /// `YYYYMMDD` 형식의 유효한 날짜.
    pub from: String,

    /// `YYYYMMDD` 형식의 유효한 날짜.
    pub to: String,

    /// 허용값: `NA`, `ND`, `NY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    pub stk_cd: String,
}

impl Usa21730Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "day", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "day", &[])?;
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["NA", "ND", "NY"])?;
        }
        let value = self.stk_cd.as_str();
        validate_field(value, true, 0, "", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21730` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21730Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21730Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21730` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21730Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21731` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21731>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21731Request {
    /// `YYYYMM` 형식의 유효한 월.
    pub from: String,

    /// `YYYYMM` 형식의 유효한 월.
    pub to: String,

    /// 허용값: `NA`, `ND`, `NY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    pub stk_cd: String,
}

impl Usa21731Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "month", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "month", &[])?;
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["NA", "ND", "NY"])?;
        }
        let value = self.stk_cd.as_str();
        validate_field(value, true, 0, "", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21731` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21731Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21731Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21731` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21731Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21732` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21732>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa21732Request {
    /// `0000`을 제외한 네 자리 연도.
    pub from: String,

    /// `0000`을 제외한 네 자리 연도.
    pub to: String,

    /// 허용값: `NA`, `ND`, `NY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    pub stk_cd: String,
}

impl Usa21732Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.from.as_str();
        validate_field(value, true, 0, "year", &[])?;
        let value = self.to.as_str();
        validate_field(value, true, 0, "year", &[])?;
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["NA", "ND", "NY"])?;
        }
        let value = self.stk_cd.as_str();
        validate_field(value, true, 0, "", &[])?;
        validate_range(Some(self.from.as_str()), Some(self.to.as_str()))?;
        Ok(())
    }
}

/// `usa21732` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21732Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Usa21732Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa21732` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa21732Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_base_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_stk_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvid_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21050` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21050>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21050Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp: Option<String>,

    /// 최대 2바이트 · 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Ust21050Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.ord_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.slby_tp {
            validate_field(value, false, 1, "", &["0", "1", "2"])?;
        }
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 2, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        Ok(())
    }
}

/// `ust21050` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21050Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21050Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21050` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21050Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_cntr_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_trde_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cncl_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_resp_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21070` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21070>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21070Request {
    /// 최대 2바이트 · 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Ust21070Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 2, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        Ok(())
    }
}

/// `ust21070` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21070Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_prch_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_book_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_prch_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_book_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21070Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21070` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21070Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poss_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_cntr_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_cntr_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_cntr_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_cntr_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_book_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_book_uv_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now_pric_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_book_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_book_amt_krw: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21110` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21110>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21110Request {}

impl Ust21110Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `ust21110` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21110Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21110Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21110` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21110Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_pymn_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub futr_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_booka: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_ord_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub futr_profa_booka: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_ch_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_etc_loana: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21111` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21111>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21111Request {}

impl Ust21111Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `ust21111` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21111Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rgst_abnd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_repl_abnd_amt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21120` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21120>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21120Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_incl_tp: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exrt_tp: Option<String>,
}

impl Ust21120Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.cmsn_incl_tp {
            validate_field(value, false, 1, "", &["0", "1"])?;
        }
        if let Some(value) = &self.exrt_tp {
            validate_field(value, false, 1, "", &["0", "1", "2"])?;
        }
        Ok(())
    }
}

/// `ust21120` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21120Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub won_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aset_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21120Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21120` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21120Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_wght: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21121` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21121>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21121Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_incl_tp: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exrt_tp: Option<String>,
}

impl Ust21121Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.cmsn_incl_tp {
            validate_field(value, false, 1, "", &["0", "1"])?;
        }
        if let Some(value) = &self.exrt_tp {
            validate_field(value, false, 1, "", &["0", "1", "2"])?;
        }
        Ok(())
    }
}

/// `ust21121` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21121Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21121Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21121` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21121Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_profit_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_wght: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21131` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21131>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21131Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub base_dt: String,
}

impl Ust21131Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.base_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        Ok(())
    }
}

/// `ust21131` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21131Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21131Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21131` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21131Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_profit_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_wght: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21132` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21132>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21132Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub base_dt: String,
}

impl Ust21132Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.base_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        Ok(())
    }
}

/// `ust21132` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21132Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub won_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub book_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aset_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21132Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21132` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21132Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_wght: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21150` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21150>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21150Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `1`, `2`, `3`, `4`, `5`, `6`.
    pub query_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub slby_tp: String,

    /// 최대 2바이트 · 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `%`, `0`, `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oppo_trde_tp: Option<String>,

    /// 최대 9바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,
}

impl Ust21150Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.ord_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.query_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2", "3", "4", "5", "6"])?;
        let value = self.slby_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 2, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.oppo_trde_tp {
            validate_field(value, false, 1, "", &["%", "0", "1"])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 9, "", &[])?;
        }
        Ok(())
    }
}

/// `ust21150` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21150Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21150Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21150` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21150Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isin_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cncl_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_msg_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oppo_trde_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stat_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inpt_chnl_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_resp_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_time: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21160` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21160>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21160Request {}

impl Ust21160Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `ust21160` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21160Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub won_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub won_dfr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub won_etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_ord_set_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usd_exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d0_setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d0_won_conv_alow_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d0_usd_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_won_conv_alow_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_usd_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_usd_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_usd_buy_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_usd_sell_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_won_conv_alow_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_usd_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_usd_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_usd_buy_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_usd_sell_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_won_conv_alow_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_usd_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_usd_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_usd_buy_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_usd_sell_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_won_conv_alow_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_usd_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_usd_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_usd_buy_excta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_usd_sell_excta: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21170` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21170>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21170Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub fc_krw_tp: String,
}

impl Ust21170Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.fc_krw_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `ust21170` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21170Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_prch_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_aseta_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21170Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21170` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21170Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prch_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poss_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prch_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prch_exrt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_exrt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_chg_dfrn_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_chg_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21180` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21180>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21180Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub strt_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub end_dt: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp: Option<String>,

    /// 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `%`, `0`, `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oppo_trde_tp: Option<String>,
}

impl Ust21180Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.strt_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.end_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        if let Some(value) = &self.slby_tp {
            validate_field(value, false, 1, "", &["0", "1", "2"])?;
        }
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.oppo_trde_tp {
            validate_field(value, false, 1, "", &["%", "0", "1"])?;
        }
        validate_range(Some(self.strt_dt.as_str()), Some(self.end_dt.as_str()))?;
        Ok(())
    }
}

/// `ust21180` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21180Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21180Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21180` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21180Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cncl_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oppo_trde_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inpt_chnl_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21510` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21510>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21510Request {
    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp: Option<String>,

    /// 최대 2바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Ust21510Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.slby_tp {
            validate_field(value, false, 1, "", &["0", "1", "2"])?;
        }
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 2, "", &[])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        Ok(())
    }
}

/// `ust21510` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21510Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21510Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21510` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21510Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_trde_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_resp_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_time: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21530` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21530>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21530Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_dt: Option<String>,

    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_krw_tp: Option<String>,
}

impl Ust21530Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.strt_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.end_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.fc_krw_tp {
            validate_field(value, false, 1, "", &["0", "1"])?;
        }
        validate_range(self.strt_dt.as_deref(), self.end_dt.as_deref())?;
        Ok(())
    }
}

/// `ust21530` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21530Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21530Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21530` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21530Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_sell_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prch_exrt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_exrt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_chg_dfrn_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_chg_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21610` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21610>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21610Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub qry_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub fc_krw_tp: String,
}

impl Ust21610Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.base_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        let value = self.fc_krw_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `ust21610` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21610Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_cmsn_altx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21610Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21610` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21610Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_avg_buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_avg_sell_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_altx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bf_prch_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21620` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21620>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21620Request {
    /// 최대 2바이트 · 허용값: `NA`, `ND`, `NY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub fc_krw_tp: String,
}

impl Ust21620Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 2, "", &["NA", "ND", "NY"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        let value = self.fc_krw_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `ust21620` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21620Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21620Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21620` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21620Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slby_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engg_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub altx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21630` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21630>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21630Request {
    /// 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub fc_krw_tp: String,
}

impl Ust21630Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        let value = self.fc_krw_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `ust21630` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21630Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_tdy_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_tdy_pl_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21630Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21630` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21630Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_sella: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub altx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21640` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21640>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21640Request {
    /// 허용값: `ND`, `NY`, `NA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub cntr_dt: String,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub fc_krw_tp: String,
}

impl Ust21640Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stex_tp {
            validate_field(value, false, 0, "", &["ND", "NY", "NA"])?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        let value = self.cntr_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.fc_krw_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `ust21640` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21640Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt_krw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21640Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21640` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21640Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_sella: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub altx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exch_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub natn_nm: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21650` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21650>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21650Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub fr_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub to_dt: String,
}

impl Ust21650Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.fr_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.to_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        validate_range(Some(self.fr_dt.as_str()), Some(self.to_dt.as_str()))?;
        Ok(())
    }
}

/// `ust21650` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21650Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_dfr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_fc_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_fc_dfr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_fc_etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_frgn_stk_evltv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_tot_evltv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_dfr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_fc_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_fc_dfr_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_fc_etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_frgn_stk_evltv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_tot_evltv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_rcpta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_payma: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_rcpta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chg_payma: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_inqa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frgn_stk_outqa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invt_bsamt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_profit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profit_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_bsamt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tern_rt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21660` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21660>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21660Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_dt: Option<String>,

    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<String>,
}

impl Ust21660Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.strt_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.end_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        validate_range(self.strt_dt.as_deref(), self.end_dt.as_deref())?;
        Ok(())
    }
}

/// `ust21660` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21660Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21660Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21660` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21660Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21661` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21661>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21661Request {
    /// 최대 6바이트 · 비어 있지 않으면 `YYYYMM` 형식의 유효한 월.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_dt: Option<String>,

    /// 최대 6바이트 · 비어 있지 않으면 `YYYYMM` 형식의 유효한 월.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<String>,
}

impl Ust21661Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.strt_dt {
            validate_field(value, false, 6, "month", &[])?;
        }
        if let Some(value) = &self.end_dt {
            validate_field(value, false, 6, "month", &[])?;
        }
        validate_range(self.strt_dt.as_deref(), self.end_dt.as_deref())?;
        Ok(())
    }
}

/// `ust21661` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21661Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<Ust21661Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ust21661` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ust21661Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
