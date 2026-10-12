//! 국내주식 계좌 조회 계약과 검증, 타입별 계약 검증.
//! 각 API의 요청·응답·목록 행과 순수 검증을 함께 유지합니다.

use super::validation::{validate_field, validate_range};
use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// `ka00001` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka00001>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka00001Request {}

impl Ka00001Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `ka00001` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka00001Response {
    #[serde(rename = "acctNo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_no: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka01690` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka01690>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka01690Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub qry_dt: String,
}

impl Ka01690Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.qry_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        Ok(())
    }
}

/// `ka01690` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka01690Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evltv_prft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbst_bal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_stk_asst: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_wght: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_bal_rt: Option<Vec<Ka01690Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka01690` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka01690Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmnd_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_wght: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evltv_prft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_wght: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10072` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10072>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10072Request {
    /// 최대 6바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub strt_dt: String,
}

impl Ka10072Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 6, "", &[])?;
        }
        let value = self.strt_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        Ok(())
    }
}

/// `ka10072` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10072Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_stk_div_rlzt_pl: Option<Vec<Ka10072Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10072` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10072Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wthd_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl_1: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10073` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10073>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10073Request {
    /// 최대 6바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub strt_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub end_dt: String,
}

impl Ka10073Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 6, "", &[])?;
        }
        let value = self.strt_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.end_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        validate_range(Some(self.strt_dt.as_str()), Some(self.end_dt.as_str()))?;
        Ok(())
    }
}

/// `ka10073` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10073Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_stk_rlzt_pl: Option<Vec<Ka10073Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10073` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10073Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_htssel_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wthd_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10074` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10074>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10074Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub strt_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub end_dt: String,
}

impl Ka10074Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.strt_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.end_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        validate_range(Some(self.strt_dt.as_str()), Some(self.end_dt.as_str()))?;
        Ok(())
    }
}

/// `ka10074` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10074Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rlzt_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_rlzt_pl: Option<Vec<Ka10074Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10074` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10074Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10075` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10075>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10075Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub all_stk_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub trde_tp: String,

    /// 최대 6바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stex_tp: String,
}

impl Ka10075Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.all_stk_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        let value = self.trde_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 6, "", &[])?;
        }
        let value = self.stex_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        Ok(())
    }
}

/// `ka10075` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10075Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oso: Option<Vec<Ka10075Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10075` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10075Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mang_empno: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tsk_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oso_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_tot_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sel_bid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_bid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ind_invsr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp_txt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sor_yn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10076` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10076>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10076Request {
    /// 최대 6바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub qry_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub sell_tp: String,

    /// 최대 10바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stex_tp: String,
}

impl Ka10076Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 6, "", &[])?;
        }
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        let value = self.sell_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.ord_no {
            validate_field(value, false, 10, "", &[])?;
        }
        let value = self.stex_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        Ok(())
    }
}

/// `ka10076` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10076Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr: Option<Vec<Ka10076Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10076` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10076Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oso_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp_txt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sor_yn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_pric: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10077` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10077>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10077Request {
    /// 최대 6바이트.
    pub stk_cd: String,
}

impl Ka10077Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.stk_cd.as_str();
        validate_field(value, true, 6, "", &[])?;
        Ok(())
    }
}

/// `ka10077` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10077Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_rlzt_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_rlzt_pl_dtl: Option<Vec<Ka10077Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10077` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10077Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10085` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10085>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10085Request {
    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stex_tp: String,
}

impl Ka10085Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.stex_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        Ok(())
    }
}

/// `ka10085` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10085Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_prft_rt: Option<Vec<Ka10085Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10085` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10085Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmnd_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sel_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setl_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clrn_alow_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr_dt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10088` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10088>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10088Request {
    /// 최대 20바이트.
    pub ord_no: String,
}

impl Ka10088Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.ord_no.as_str();
        validate_field(value, true, 20, "", &[])?;
        Ok(())
    }
}

/// `ka10088` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10088Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub osop: Option<Vec<Ka10088Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10088` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10088Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub osop_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_stt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stex_tp_txt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10170` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10170>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10170Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `1`, `2`.
    pub ottks_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub ch_crd_tp: String,
}

impl Ka10170Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.base_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.ottks_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2"])?;
        let value = self.ch_crd_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        Ok(())
    }
}

/// `ka10170` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10170Response {
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
    pub tot_prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_trde_diary: Option<Vec<Ka10170Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `ka10170` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10170Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_avg_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sel_avg_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn_alm_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00001` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00001>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00001Request {
    /// 최대 1바이트 · 허용값: `2`, `3`.
    pub qry_tp: String,
}

impl Kt00001Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["2", "3"])?;
        Ok(())
    }
}

/// `kt00001` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00001Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bncr_profa_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nxdy_bncr_sell_exct: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_stk_krw_repl_set_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnta_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_grnt_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncl_stk_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shrts_prica: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_set_grnta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chck_ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_chck_ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_ruse: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knx_asset_evltv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elwdpst_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_ls_rght_frcs_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvlh_join_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lvlh_trns_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remn_repl_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trst_remn_repl_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bncr_remn_repl_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnta_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_grnt_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rght_repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrap_pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bncr_buy_alowa: Option<String>,
    #[serde(rename = "20stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_20stk_ord_alow_amt: Option<String>,
    #[serde(rename = "30stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_30stk_ord_alow_amt: Option<String>,
    #[serde(rename = "40stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_40stk_ord_alow_amt: Option<String>,
    #[serde(rename = "100stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_100stk_ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla_dlfe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_npay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int_npay_amt_dlfe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int_npay_amt_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana_dlfe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loan_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nrpy_loan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdstrm_usfe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ord_alow_yn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_remn_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dpst_grntl_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_grntl_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_slby_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_buy_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_out_rep_mor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_sel_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_slby_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_buy_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_out_rep_mor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_sel_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_pymn_alow_amt: Option<String>,
    #[serde(rename = "50stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_50stk_ord_alow_amt: Option<String>,
    #[serde(rename = "60stk_ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_60stk_ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_entr_prst: Option<Vec<Kt00001Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00001` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00001Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crnc_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_krw_repl_evlta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_trst_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_alow_amt_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alow_amt_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc_ch_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d1_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d3_fx_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d4_fx_entr: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00002` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00002>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00002Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub start_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub end_dt: String,
}

impl Kt00002Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.start_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        let value = self.end_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        validate_range(Some(self.start_dt.as_str()), Some(self.end_dt.as_str()))?;
        Ok(())
    }
}

/// `kt00002` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00002Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daly_prsm_dpst_aset_amt_prst: Option<Vec<Kt00002Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00002` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00002Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grnt_use_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_grnt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_dpst_aset_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_dpst_aset_amt_bncr_skip: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00003` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00003>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00003Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub qry_tp: String,
}

impl Kt00003Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        Ok(())
    }
}

/// `kt00003` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00003Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_dpst_aset_amt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00004` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00004>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00004Request {
    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub qry_tp: String,

    /// 최대 6바이트 · 허용값: `KRX`, `NXT`.
    pub dmst_stex_tp: String,
}

impl Kt00004Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["KRX", "NXT"])?;
        Ok(())
    }
}

/// `kt00004` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00004Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brch_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_est_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aset_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_dpst_aset_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_grnt_sella: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_lspft_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invt_bsamt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lspft_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_lspft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lspft2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lspft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_lspft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lspft_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lspft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_acnt_evlt_prst: Option<Vec<Kt00004Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00004` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00004Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmnd_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setl_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sellq: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00005` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00005>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00005Request {
    /// 최대 6바이트 · 허용값: `KRX`, `NXT`.
    pub dmst_stex_tp: String,
}

impl Kt00005Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["KRX", "NXT"])?;
        Ok(())
    }
}

/// `kt00005` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00005Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr_d1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncl_stk_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rght_repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_npay_gold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nrpy_loan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_buy_tot_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_re_buy_alowa: Option<String>,
    #[serde(rename = "20ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_20ord_alow_amt: Option<String>,
    #[serde(rename = "30ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_30ord_alow_amt: Option<String>,
    #[serde(rename = "40ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_40ord_alow_amt: Option<String>,
    #[serde(rename = "50ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_50ord_alow_amt: Option<String>,
    #[serde(rename = "60ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_60ord_alow_amt: Option<String>,
    #[serde(rename = "100ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_100ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_ls_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dpst_grnt_use_amt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grnt_loan_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cntr_remn: Option<Vec<Kt00005Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00005` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00005Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setl_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evltv_prft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_rt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00007` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00007>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00007Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `1`, `2`, `3`, `4`.
    pub qry_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stk_bond_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub sell_tp: String,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,

    /// 최대 6바이트 · 허용값: `%`, `KRX`, `NXT`, `SOR`.
    pub dmst_stex_tp: String,
}

impl Kt00007Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.ord_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2", "3", "4"])?;
        let value = self.stk_bond_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.sell_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 7, "", &[])?;
        }
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["%", "KRX", "NXT", "SOR"])?;
        Ok(())
    }
}

/// `kt00007` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00007Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_ord_cntr_prps_dtl: Option<Vec<Kt00007Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00007` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00007Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acpt_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ori_ord: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_cncl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond_uv: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00008` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00008>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00008Request {
    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_dcd_seq: Option<String>,
}

impl Kt00008Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.strt_dcd_seq {
            validate_field(value, false, 7, "", &[])?;
        }
        Ok(())
    }
}

/// `kt00008` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00008Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setl_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_nxdy_setl_frcs_prps_array: Option<Vec<Kt00008Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00008` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00008Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engg_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incm_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rstx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resi_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00009` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00009>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00009Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stk_bond_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`, `3`, `4`.
    pub mrkt_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub sell_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`.
    pub qry_tp: String,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,

    /// 최대 6바이트 · 허용값: `%`, `KRX`, `NXT`, `SOR`.
    pub dmst_stex_tp: String,
}

impl Kt00009Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.ord_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.stk_bond_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.mrkt_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2", "3", "4"])?;
        let value = self.sell_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 7, "", &[])?;
        }
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["%", "KRX", "NXT", "SOR"])?;
        Ok(())
    }
}

/// `kt00009` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00009Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_grntl_engg_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_engg_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engg_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_ord_cntr_prst_array: Option<Vec<Kt00009Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00009` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00009Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_bond_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_oppo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acpt_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setl_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_deal_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_cncl_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond_uv: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00010` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00010>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00010Request {
    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_amt: Option<String>,

    /// 최대 12바이트.
    pub stk_cd: String,

    /// 최대 1바이트 · 허용값: `1`, `2`.
    pub trde_tp: String,

    /// 최대 10바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_qty: Option<String>,

    /// 최대 10바이트.
    pub uv: String,

    /// 최대 10바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_buy_unp: Option<String>,
}

impl Kt00010Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.io_amt {
            validate_field(value, false, 12, "", &[])?;
        }
        let value = self.stk_cd.as_str();
        validate_field(value, true, 12, "", &[])?;
        let value = self.trde_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2"])?;
        if let Some(value) = &self.trde_qty {
            validate_field(value, false, 10, "", &[])?;
        }
        let value = self.uv.as_str();
        validate_field(value, true, 10, "", &[])?;
        if let Some(value) = &self.exp_buy_unp {
            validate_field(value, false, 10, "", &[])?;
        }
        Ok(())
    }
}

/// `kt00010` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00010Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_rdex_60ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_rdex_60ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pos_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wthd_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nxdy_wthd_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_rdex_aplc_tp: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00011` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00011>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00011Request {
    /// 최대 12바이트.
    pub stk_cd: String,

    /// 최대 10바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<String>,
}

impl Kt00011Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.stk_cd.as_str();
        validate_field(value, true, 12, "", &[])?;
        if let Some(value) = &self.uv {
            validate_field(value, false, 10, "", &[])?;
        }
        Ok(())
    }
}

/// `kt00011` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00011Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_profa_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aplc_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_20tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_30tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_40tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_50tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_60tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_100tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pos_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alowa: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00012` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00012>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00012Request {
    /// 최대 12바이트.
    pub stk_cd: String,

    /// 최대 10바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<String>,
}

impl Kt00012Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.stk_cd.as_str();
        validate_field(value, true, 12, "", &[])?;
        if let Some(value) = &self.uv {
            validate_field(value, false, 10, "", &[])?;
        }
        Ok(())
    }
}

/// `kt00012` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00012Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_assr_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_assr_rt_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_30ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_30ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_30pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_30tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_40ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_40ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_40pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_40tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_50ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_50ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_50pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_50tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_60ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_60ord_alowq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_60pred_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assr_60tdy_reu_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_pos_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_pos_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00013` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00013>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00013Request {}

impl Kt00013Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `kt00013` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00013Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_objt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_use_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_lmtt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_reu_alowa_fin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_objt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_use_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_lmtt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_reu_alowa_fin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_pos_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_use_lmtt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_pos_ch_fin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_amt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_pos_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_use_lmtt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_pos_repl_fin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnta_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnta_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_grnt_repl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_grnt_reu_gold: Option<String>,
    #[serde(rename = "20ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_20ord_alow_amt: Option<String>,
    #[serde(rename = "30ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_30ord_alow_amt: Option<String>,
    #[serde(rename = "40ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_40ord_alow_amt: Option<String>,
    #[serde(rename = "50ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_50ord_alow_amt: Option<String>,
    #[serde(rename = "60ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_60ord_alow_amt: Option<String>,
    #[serde(rename = "100ord_alow_amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_100ord_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_crd_rpya_loss_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_crd_rpya_loss_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_ls_rpya_loss_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_ls_rpya_loss_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_repl_amt_spg_use_skip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_repl_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_ord_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_ord_repl_profa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_repl_conv_gold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repl_alowa_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_repl_lck_gold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_repl_lck_gold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_ord_alow_repla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_ord_alow_repla: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2vexct_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2ch_ord_alow_amt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00016` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00016>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00016Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub fr_dt: String,

    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub to_dt: String,
}

impl Kt00016Request {
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

/// `kt00016` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00016Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mang_empno: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mngr_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dept_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entr_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scrt_evlt_amt_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scrt_evlt_amt_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_grnt_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_grnt_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ch_uncla_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_asgna_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub krw_asgna_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_evlta_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ls_evlta_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rght_evlta_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rght_evlta_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_amt_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_amt_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_npay_gold_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_npay_gold_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_amt_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_amt_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invt_bsamt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evltv_prft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tern_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termin_tot_trns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termin_tot_pymn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termin_tot_inq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termin_tot_outq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub futr_repl_sella: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trst_repl_sella: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00017` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00017>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00017Request {}

impl Kt00017Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `kt00017` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00017Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d2_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_npay_gold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loana: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gnrl_stk_evlt_amt_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dpst_grnt_use_amt_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_stk_evlt_amt_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_evlta_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_ls_grnt_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_ls_evlta_d2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outq_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_pur_cptal_loan_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rp_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bd_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elsevlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_int_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sel_prica_grnt_loan_int_amt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dvida_amt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00018` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00018>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00018Request {
    /// 최대 1바이트 · 허용값: `1`, `2`.
    pub qry_tp: String,

    /// 최대 6바이트 · 허용값: `KRX`, `NXT`.
    pub dmst_stex_tp: String,
}

impl Kt00018Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2"])?;
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["KRX", "NXT"])?;
        Ok(())
    }
}

/// `kt00018` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00018Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_evlt_pl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_dpst_aset_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_loan_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_crd_loan_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_crd_ls_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_evlt_remn_indv_tot: Option<Vec<Kt00018Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt00018` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt00018Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evltv_prft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prft_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_close_pric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmnd_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_able_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pred_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_buyq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdy_sellq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pur_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evlt_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poss_rt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_loan_dt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50020` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50020>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50020Request {}

impl Kt50020Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `kt50020` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50020Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_entr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_est_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_book_amt2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tot_dep_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paym_alowa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gold_acnt_evlt_prst: Option<Vec<Kt50020Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50020` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50020Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub real_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cur_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub est_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub est_lspft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub est_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlad_tax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub book_amt2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pl_prch_prc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub able_qty: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50021` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50021>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50021Request {}

impl Kt50021Request {
    /// 검사할 요청 필드가 없어 성공을 반환합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

/// `kt50021` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50021Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profa_ch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chck_ina_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loan_dlfe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etc_loan_tot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_entra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_buy_exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prsm_pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pymn_alow_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_alow_amt: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50030` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50030>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50030Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub ord_dt: String,

    /// 최대 1바이트 · 허용값: `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<String>,

    /// 최대 1바이트 · 허용값: `5`.
    pub mrkt_deal_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stk_bond_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub slby_tp: String,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,

    /// 최대 6바이트 · 허용값: `%`, `KRX`, `NXT`, `SOR`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
}

impl Kt50030Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.ord_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        if let Some(value) = &self.qry_tp {
            validate_field(value, false, 1, "", &["1", "2"])?;
        }
        let value = self.mrkt_deal_tp.as_str();
        validate_field(value, true, 1, "", &["5"])?;
        let value = self.stk_bond_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.slby_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 7, "", &[])?;
        }
        if let Some(value) = &self.dmst_stex_tp {
            validate_field(value, false, 6, "", &["%", "KRX", "NXT", "SOR"])?;
        }
        Ok(())
    }
}

/// `kt50030` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50030Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_ord_cntr_prst: Option<Vec<Kt50030Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50030` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50030Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_bond_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_send_end_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrkt_deal_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dcd_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_deal_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_cncl_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond_uv: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50031` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50031>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50031Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_dt: Option<String>,

    /// 최대 1바이트 · 허용값: `1`, `2`, `3`, `4`.
    pub qry_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stk_bond_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub sell_tp: String,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,

    /// 최대 6바이트 · 허용값: `%`, `KRX`, `NXT`, `SOR`.
    pub dmst_stex_tp: String,
}

impl Kt50031Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.ord_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        let value = self.qry_tp.as_str();
        validate_field(value, true, 1, "", &["1", "2", "3", "4"])?;
        let value = self.stk_bond_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.sell_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 7, "", &[])?;
        }
        let value = self.dmst_stex_tp.as_str();
        validate_field(value, true, 6, "", &["%", "KRX", "NXT", "SOR"])?;
        Ok(())
    }
}

/// `kt50031` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50031Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_ord_cntr_prps_dtl: Option<Vec<Kt50031Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50031` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50031Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acpt_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ori_ord: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_cncl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_tm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond_uv: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50032` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50032>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50032Request {
    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_dt: Option<String>,

    /// 최대 8바이트 · 비어 있지 않으면 `YYYYMMDD` 형식의 유효한 날짜.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp: Option<String>,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Kt50032Request {
    /// 문서에 근거한 필드 제약과 조회 기간 순서를 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(value) = &self.strt_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.end_dt {
            validate_field(value, false, 8, "day", &[])?;
        }
        if let Some(value) = &self.tp {
            validate_field(
                value,
                false,
                1,
                "",
                &["0", "1", "2", "3", "4", "5", "6", "7"],
            )?;
        }
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        validate_range(self.strt_dt.as_deref(), self.end_dt.as_deref())?;
        Ok(())
    }
}

/// `kt50032` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50032Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_print: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gold_trde_hist: Option<Vec<Kt50032Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50032` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50032Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deal_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deal_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmrk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deal_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gold_spot_vat: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exct_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dly_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entra_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdia_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_deal_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv_exrt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmsn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncl_ocr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpym_sum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spot_remn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcpy_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deal_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_tot_amt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_dt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_brch_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prcsr: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50075` 계좌 조회 요청.
///
/// 공개 가이드: <https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50075>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt50075Request {
    /// 최대 8바이트 · `YYYYMMDD` 형식의 유효한 날짜.
    pub ord_dt: String,

    /// 최대 1바이트 · 허용값: `1`, `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<String>,

    /// 최대 1바이트 · 허용값: `5`.
    pub mrkt_deal_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub stk_bond_tp: String,

    /// 최대 1바이트 · 허용값: `0`, `1`, `2`.
    pub sell_tp: String,

    /// 최대 12바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// 최대 7바이트.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_ord_no: Option<String>,

    /// 최대 6바이트 · 허용값: `%`, `KRX`, `NXT`, `SOR`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
}

impl Kt50075Request {
    /// 문서에 근거한 요청 필드 제약을 검사합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        let value = self.ord_dt.as_str();
        validate_field(value, true, 8, "day", &[])?;
        if let Some(value) = &self.qry_tp {
            validate_field(value, false, 1, "", &["1", "2"])?;
        }
        let value = self.mrkt_deal_tp.as_str();
        validate_field(value, true, 1, "", &["5"])?;
        let value = self.stk_bond_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        let value = self.sell_tp.as_str();
        validate_field(value, true, 1, "", &["0", "1", "2"])?;
        if let Some(value) = &self.stk_cd {
            validate_field(value, false, 12, "", &[])?;
        }
        if let Some(value) = &self.fr_ord_no {
            validate_field(value, false, 7, "", &[])?;
        }
        if let Some(value) = &self.dmst_stex_tp {
            validate_field(value, false, 6, "", &["%", "KRX", "NXT", "SOR"])?;
        }
        Ok(())
    }
}

/// `kt50075` 응답. 문자열의 부호·패딩·빈 값을 원본 그대로 보존합니다.
/// 목록의 생략은 `None`으로 보존하며 빈 배열과 구분합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50075Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acnt_ord_oso_prst: Option<Vec<Kt50075Row>>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt50075` 응답 목록의 한 행. 알 수 없는 필드는 `extra`에 보존합니다.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt50075Row {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_bond_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trde_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnfm_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_send_end_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrkt_deal_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsrv_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orig_ord_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stk_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dcd_tp_nm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd_deal_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_qty: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_uv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ord_remnq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comm_ord_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdfy_cncl_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dmst_stex_tp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond_uv: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
