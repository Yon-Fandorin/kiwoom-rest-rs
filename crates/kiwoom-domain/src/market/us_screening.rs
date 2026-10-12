//! 미국주식 조건·순위·계절별 조회 계약.
//!
//! 선택 입력의 생략을 보존하고, 각 API가 정한 선택값과 상한을 검사한다.

use crate::{ClientError, transactions::UsExchange};

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use super::{choice, integer_limit, nonempty};

macro_rules! optional_request {
    ($name:ident {$($field:ident),* $(,)?}) => {
        /// 선택 조건을 생략한 상태 그대로 직렬화하는 미국주식 조회 요청.
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                /// 선택 조건의 원본 문자열. 생략값은 전송하지 않는다.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,
            )*
        }
    }
}

// 같은 가격 범위 계약은 정확한 십진수와 음수 거부 규칙을 공유한다.
fn nonnegative_price(v: &Option<String>) -> Result<(), ClientError> {
    if let Some(s) = v {
        let n = rust_decimal::Decimal::from_str_exact(s)
            .map_err(|_| ClientError::InvalidRequest("invalid price filter"))?;
        if n < rust_decimal::Decimal::ZERO {
            return Err(ClientError::InvalidRequest("negative price filter"));
        }
    }
    Ok(())
}

// 십진수 조건은 표현 가능한 정밀도만 허용하고 반올림 없이 상한과 비교한다.
fn decimal_limit(v: &Option<String>, max: &str) -> Result<(), ClientError> {
    if let Some(s) = v {
        let n = rust_decimal::Decimal::from_str_exact(s)
            .map_err(|_| ClientError::InvalidRequest("invalid decimal filter"))?;
        let max = max
            .parse::<rust_decimal::Decimal>()
            .expect("constant decimal limit");
        if n < rust_decimal::Decimal::ZERO || n > max {
            return Err(ClientError::InvalidRequest(
                "decimal filter outside documented maximum",
            ));
        }
    }
    Ok(())
}

// 선택 연도가 있으면 네 자리 숫자 형식과 0000 금지 규칙을 검사한다.
fn year(v: &Option<String>) -> Result<(), ClientError> {
    if let Some(s) = v
        && (s.len() != 4 || !s.bytes().all(|b| b.is_ascii_digit()) || s == "0000")
    {
        return Err(ClientError::InvalidRequest("year must be YYYY"));
    }
    Ok(())
}

// 숫자 거래소 필터는 공개 페이지마다 설명이 달라 공통 거래소 이름을 부여하지 않는다.
// 각 API의 0/1/2/3 원본 선택값을 그대로 검증한다.
optional_request! {
    Usa20520Request {
        stex_tp,
        inds_cd,
        tm,
        stk_tp,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        trde_qty_tp,
    }
}

impl Usa20520Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.tm, &["5", "10", "20", "30"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

// 조건검색 응답 행과 원본 컬렉션.
fields! {
    UsVolumeSurge {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        sdnin_rt,
        sel_bid,
        buy_bid,
    }
}

collection!(Usa20520Response, result_list, UsVolumeSurge);

optional_request! {
    Usa20521Request {
        stex_tp,
        tm,
        etf_cat1,
        etf_cat2,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        trde_qty_tp,
    }
}

impl Usa20521Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.tm, &["5", "10", "20", "30"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

collection!(Usa20521Response, result_list, UsVolumeSurge);

optional_request! {
    Usa20570Request {
        stex_tp,
        stk_tp,
        stk_cnd,
        inds_cd,
        trde_qty_tp,
        pric_cnd1,
        pric_cnd2,
        trde_prica_cnd,
    }
}

impl Usa20570Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        decimal_limit(&self.pric_cnd1, "999999.99")?;
        decimal_limit(&self.pric_cnd2, "999999.99")?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

fields! {
    UsPriceBand {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_bid,
        buy_bid,
        open_pric,
        high_pric,
        low_pric,
    }
}

/// `usa20570`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa20570Response {
    /// `stk_num` 원본 응답 값. 생략값을 보존한다.
    pub stk_num: Option<String>,

    /// `rising_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub rising_stk_num: Option<String>,

    /// `flat_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub flat_stk_num: Option<String>,

    /// `fall_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub fall_stk_num: Option<String>,

    /// `result_list` 원본 응답 값. 생략값을 보존한다.
    pub result_list: Option<Vec<UsPriceBand>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

optional_request! {
    Usa20571Request {
        stex_tp,
        stk_cnd,
        etf_cat1,
        etf_cat2,
        trde_qty_tp,
        pric_cnd1,
        pric_cnd2,
        trde_prica_cnd,
    }
}

impl Usa20571Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        decimal_limit(&self.pric_cnd1, "999999.99")?;
        decimal_limit(&self.pric_cnd2, "999999.99")?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

/// `usa20571`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa20571Response {
    /// `stk_num` 원본 응답 값. 생략값을 보존한다.
    pub stk_num: Option<String>,

    /// `rising_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub rising_stk_num: Option<String>,

    /// `flat_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub flat_stk_num: Option<String>,

    /// `fall_stk_num` 원본 응답 값. 생략값을 보존한다.
    pub fall_stk_num: Option<String>,

    /// `result_list` 원본 응답 값. 생략값을 보존한다.
    pub result_list: Option<Vec<UsPriceBand>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

optional_request! {
    Usa20930Request {
        stex_tp,
        stk_tp,
        inds_cd,
        stk_cnd,
        flu_tp,
        tm_tp,
        tm,
        pric_cnd,
        trde_qty_tp,
        trde_prica_cnd,
    }
}

impl Usa20930Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(&self.flu_tp, &["1", "2"])?;
        choice(&self.tm_tp, &["1", "2", "3"])?;
        if self.tm_tp.as_deref() == Some("3") {
            choice(&self.tm, &["1", "2", "3", "4", "5", "6", "7", "8", "9"])?;
        } else {
            integer_limit(&self.tm, 30)?;
        }
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

fields! {
    UsPriceSurge {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        base_pric,
        base_pre,
        acc_trde_qty,
        sdnin_rt,
    }
}

collection!(Usa20930Response, result_list, UsPriceSurge);

optional_request! {
    Usa20931Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        stk_cnd,
        flu_tp,
        tm_tp,
        tm,
        pric_cnd,
        trde_qty_tp,
        trde_prica_cnd,
    }
}

impl Usa20931Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(&self.flu_tp, &["1", "2"])?;
        choice(&self.tm_tp, &["1", "2", "3"])?;
        if self.tm_tp.as_deref() == Some("3") {
            choice(&self.tm, &["1", "2", "3", "4", "5", "6", "7", "8", "9"])?;
        } else {
            integer_limit(&self.tm, 30)?;
        }
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

collection!(Usa20931Response, result_list, UsPriceSurge);

/// 미국 조건 조회에 전달하는 종목·거래소 입력 행.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsWatchItem {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchange>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

/// `usa20932`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa20932Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// `stk_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cnd: Option<String>,

    /// `flu_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flu_tp: Option<String>,

    /// `tm_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm_tp: Option<String>,

    /// `tm` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm: Option<String>,

    /// `pric_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pric_cnd: Option<String>,

    /// `trde_qty_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_qty_tp: Option<String>,

    /// `trde_prica_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_prica_cnd: Option<String>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<Vec<UsWatchItem>>,
}

impl Usa20932Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(&self.flu_tp, &["1", "2"])?;
        choice(&self.tm_tp, &["1", "2", "3"])?;
        if self.tm_tp.as_deref() == Some("3") {
            choice(&self.tm, &["1", "2", "3", "4", "5", "6", "7", "8", "9"])?;
        } else {
            integer_limit(&self.tm, 30)?;
        }
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        for row in self.stk_cd.iter().flatten() {
            if let Some(code) = &row.stk_cd {
                nonempty(code)?;
            }
        }
        Ok(())
    }
}

fields! {
    UsWatchPriceSurge {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        base_pric,
        cur_prc,
        base_pre,
        acc_trde_qty,
        sdnin_rt,
    }
}

collection!(Usa20932Response, result_list, UsWatchPriceSurge);

optional_request! {
    Usa20970Request {
        stex_tp,
        inds_cd,
        stk_tp,
        high_low_tp,
        alacc_rt,
        stk_cnd,
        pric_cnd_st,
        pric_cnd_ed,
        trde_pric_cnd_st,
        trde_qty_cnd_fr,
    }
}

impl Usa20970Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.high_low_tp, &["1", "2"])?;
        choice(&self.alacc_rt, &["0.5", "1.0", "1.5", "2.0", "2.5", "3.5"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        nonnegative_price(&self.pric_cnd_st)?;
        nonnegative_price(&self.pric_cnd_ed)?;
        choice(
            &self.trde_pric_cnd_st,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_cnd_fr,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

fields! {
    UsNearHighLow {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        sel_bid,
        buy_bid,
        high_pric,
        low_pric,
    }
}

collection!(Usa20970Response, result_list, UsNearHighLow);

optional_request! {
    Usa20971Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        high_low_tp,
        alacc_rt,
        stk_cnd,
        pric_cnd_st,
        pric_cnd_ed,
        trde_pric_cnd_st,
        trde_qty_cnd_fr,
    }
}

impl Usa20971Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.high_low_tp, &["1", "2"])?;
        choice(&self.alacc_rt, &["0.5", "1.0", "1.5", "2.0", "2.5", "3.5"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        nonnegative_price(&self.pric_cnd_st)?;
        nonnegative_price(&self.pric_cnd_ed)?;
        choice(
            &self.trde_pric_cnd_st,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_cnd_fr,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

collection!(Usa20971Response, result_list, UsNearHighLow);

/// `usa20972`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa20972Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// `high_low_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_low_tp: Option<String>,

    /// `alacc_rt` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alacc_rt: Option<String>,

    /// `stk_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cnd: Option<String>,

    /// `pric_cnd_st` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pric_cnd_st: Option<String>,

    /// `pric_cnd_ed` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pric_cnd_ed: Option<String>,

    /// `trde_pric_cnd_st` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_pric_cnd_st: Option<String>,

    /// `trde_qty_cnd_fr` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_qty_cnd_fr: Option<String>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<Vec<UsWatchItem>>,
}

impl Usa20972Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.high_low_tp, &["1", "2"])?;
        choice(&self.alacc_rt, &["0.5", "1.0", "1.5", "2.0", "2.5", "3.5"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        nonnegative_price(&self.pric_cnd_st)?;
        nonnegative_price(&self.pric_cnd_ed)?;
        choice(
            &self.trde_pric_cnd_st,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_cnd_fr,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        for row in self.stk_cd.iter().flatten() {
            if let Some(code) = &row.stk_cd {
                nonempty(code)?;
            }
        }
        Ok(())
    }
}

fields! {
    UsWatchNearHighLow {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        acc_trde_qty,
        tp,
        flu_rt,
        sel_bid,
        buy_bid,
        high_pric,
        low_pric,
    }
}

collection!(Usa20972Response, result_list, UsWatchNearHighLow);

optional_request! {
    Usa23400Request {
        stex_tp,
        stk_cd,
        trde_qty_tp,
        stk_tp,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        dt_tp,
    }
}

impl Usa23400Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(&self.dt_tp, &["5", "10", "20", "60", "250"])?;
        Ok(())
    }
}

fields! {
    UsLowVolume {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        prev_trde_qty,
        acc_trde_qty,
        sel_bid,
        buy_bid,
    }
}

collection!(Usa23400Response, result_list, UsLowVolume);

optional_request! {
    Usa23401Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        trde_qty_tp,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        dt_tp,
    }
}

impl Usa23401Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(&self.dt_tp, &["5", "10", "20", "60", "250"])?;
        Ok(())
    }
}

collection!(Usa23401Response, result_list, UsLowVolume);

/// `usa23402`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa23402Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<String>,

    /// `trde_qty_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_qty_tp: Option<String>,

    /// `stk_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cnd: Option<String>,

    /// `pric_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pric_cnd: Option<String>,

    /// `trde_prica_cnd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trde_prica_cnd: Option<String>,

    /// `dt_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_tp: Option<String>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<Vec<UsWatchItem>>,
}

impl Usa23402Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(&self.dt_tp, &["5", "10", "20", "60", "250"])?;
        for row in self.stk_cd.iter().flatten() {
            if let Some(code) = &row.stk_cd {
                nonempty(code)?;
            }
        }
        Ok(())
    }
}

collection!(Usa23402Response, result_list, UsLowVolume);

optional_request! {
    Usa24100Request {
        stex_tp,
        stk_tp,
        inds_cd,
        stk_cnd,
        ntl_tp,
        high_low_tp,
        dt,
        pric_cnd,
        trde_qty_tp,
        trde_prica_cnd,
    }
}

impl Usa24100Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(&self.ntl_tp, &["1", "2"])?;
        choice(&self.high_low_tp, &["1", "2"])?;
        integer_limit(&self.dt, 250)?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

fields! {
    UsNewHighLow {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pre_trde_rt,
        sel_bid,
        buy_bid,
        high_pric,
        low_pric,
        pred_trde_qty,
    }
}

collection!(Usa24100Response, result_list, UsNewHighLow);

optional_request! {
    Usa24101Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        stk_cnd,
        ntl_tp,
        high_low_tp,
        dt,
        pric_cnd,
        trde_qty_tp,
        trde_prica_cnd,
    }
}

impl Usa24101Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(&self.ntl_tp, &["1", "2"])?;
        choice(&self.high_low_tp, &["1", "2"])?;
        integer_limit(&self.dt, 250)?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

collection!(Usa24101Response, result_list, UsNewHighLow);

optional_request! {
    Usa24140Request {
        stex_tp,
        inds_cd,
        stk_tp,
        sort_tp,
        updown_tp,
        alacc_rt,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        trde_qty_tp,
    }
}

impl Usa24140Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        choice(&self.sort_tp, &["1", "2"])?;
        choice(&self.updown_tp, &["1", "2"])?;
        choice(&self.alacc_rt, &["3", "5", "10", "50", "100", "150", "200"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

fields! {
    UsGap {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        open_pric,
        base_close_pric,
        gap_rt,
        pre_high_pric,
        pred_trde_qty,
    }
}

collection!(Usa24140Response, result_list, UsGap);

optional_request! {
    Usa24141Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        sort_tp,
        updown_tp,
        alacc_rt,
        stk_cnd,
        pric_cnd,
        trde_prica_cnd,
        trde_qty_tp,
    }
}

impl Usa24141Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.sort_tp, &["1", "2"])?;
        choice(&self.updown_tp, &["1", "2"])?;
        choice(&self.alacc_rt, &["3", "5", "10", "50", "100", "150", "200"])?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        Ok(())
    }
}

collection!(Usa24141Response, result_list, UsGap);

optional_request! {
    Usa24210Request {
        stex_tp,
        inds_cd,
        rt_tp,
        stk_tp,
        tm,
        stk_cnd,
        trde_qty_tp,
        pric_cnd,
        trde_prica_cnd,
    }
}

impl Usa24210Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.rt_tp, &["0", "1"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        integer_limit(&self.tm, 30)?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

fields! {
    UsBookRatioSurge {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        int,
        now_rt,
        sdnin_rt,
        tot_sel_req,
        tot_buy_req,
    }
}

collection!(Usa24210Response, result_list, UsBookRatioSurge);

optional_request! {
    Usa24211Request {
        stex_tp,
        rt_tp,
        etf_cat1,
        etf_cat2,
        tm,
        stk_cnd,
        trde_qty_tp,
        pric_cnd,
        trde_prica_cnd,
    }
}

impl Usa24211Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.rt_tp, &["0", "1"])?;
        integer_limit(&self.tm, 30)?;
        choice(&self.stk_cnd, &["0", "1", "2"])?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

collection!(Usa24211Response, result_list, UsBookRatioSurge);

optional_request! {
    Usa24220Request {
        stex_tp,
        inds_cd,
        stk_tp,
        dt,
        prps_cnctr_rt,
        cond,
        prpscnt,
        trde_qty_tp,
        pric_cnd,
        trde_prica_cnd,
    }
}

impl Usa24220Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        choice(&self.stk_tp, &["0", "1"])?;
        integer_limit(&self.dt, 300)?;
        decimal_limit(&self.prps_cnctr_rt, "100")?;
        choice(&self.cond, &["0", "1", "2", "3"])?;
        integer_limit(&self.prpscnt, 99)?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

fields! {
    UsConcentratedSupply {
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pric_cnd_st,
        pric_cnd_ed,
        prps_qty,
        prps_rt,
    }
}

collection!(Usa24220Response, result_list, UsConcentratedSupply);

optional_request! {
    Usa24221Request {
        stex_tp,
        etf_cat1,
        etf_cat2,
        dt,
        prps_cnctr_rt,
        cond,
        prpscnt,
        trde_qty_tp,
        pric_cnd,
        trde_prica_cnd,
    }
}

impl Usa24221Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        choice(&self.stex_tp, &["0", "1", "2", "3"])?;
        integer_limit(&self.dt, 300)?;
        decimal_limit(&self.prps_cnctr_rt, "100")?;
        choice(&self.cond, &["0", "1", "2", "3"])?;
        integer_limit(&self.prpscnt, 99)?;
        choice(
            &self.trde_qty_tp,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
        )?;
        choice(
            &self.pric_cnd,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        choice(
            &self.trde_prica_cnd,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        Ok(())
    }
}

collection!(Usa24221Response, result_list, UsConcentratedSupply);

optional_request! {
    Usa26411Request {
        inds_cd,
        srch_yr,
    }
}

impl Usa26411Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        year(&self.srch_yr)?;
        Ok(())
    }
}

fields! {
    UsSeasonalStock {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        m01_prft_rt,
        m02_prft_rt,
        m03_prft_rt,
        m04_prft_rt,
        m05_prft_rt,
        m06_prft_rt,
        m07_prft_rt,
        m08_prft_rt,
        m09_prft_rt,
        m10_prft_rt,
        m11_prft_rt,
        m12_prft_rt,
    }
}

collection!(Usa26411Response, result_list, UsSeasonalStock);

optional_request! {
    Usa26412Request {
        etf_cat1,
        etf_cat2,
        srch_yr,
    }
}

impl Usa26412Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        year(&self.srch_yr)?;
        Ok(())
    }
}

collection!(Usa26412Response, result_list, UsSeasonalStock);

optional_request! {
    Usa26413Request {
        inds_cd,
    }
}

impl Usa26413Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

fields! {
    UsSeasonalYear {
        dt,
        m01_prft_rt,
        m02_prft_rt,
        m03_prft_rt,
        m04_prft_rt,
        m05_prft_rt,
        m06_prft_rt,
        m07_prft_rt,
        m08_prft_rt,
        m09_prft_rt,
        m10_prft_rt,
        m11_prft_rt,
        m12_prft_rt,
    }
}

collection!(Usa26413Response, result_list, UsSeasonalYear);

optional_request! {
    Usa26414Request {
        etf_cat1,
        etf_cat2,
    }
}

impl Usa26414Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        Ok(())
    }
}

collection!(Usa26414Response, result_list, UsSeasonalYear);

us_quote_request!(Usa26410Request);

collection!(Usa26410Response, result_list, UsSeasonalYear);
