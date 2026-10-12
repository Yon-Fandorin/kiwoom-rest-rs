//! 독립적으로 작성한 주문 계약입니다.
//! 금융 메서드는 명시적 허용을 요구하며 재시도하지 않습니다.

use crate::ClientError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

pub(crate) const READ_CONTRACTS: &[(&str, &str)] = &[
    ("ust21204", "/api/us/ordr"),
    ("ust21205", "/api/us/ordr"),
    ("ust31490", "/api/us/ordr"),
];
pub(crate) const WRITE_CONTRACTS: &[(&str, &str)] = &[
    ("kt10000", "/api/dostk/ordr"),
    ("kt10001", "/api/dostk/ordr"),
    ("kt10002", "/api/dostk/ordr"),
    ("kt10003", "/api/dostk/ordr"),
    ("kt50000", "/api/dostk/ordr"),
    ("kt50001", "/api/dostk/ordr"),
    ("kt50002", "/api/dostk/ordr"),
    ("kt50003", "/api/dostk/ordr"),
    ("kt10006", "/api/dostk/crdordr"),
    ("kt10007", "/api/dostk/crdordr"),
    ("kt10008", "/api/dostk/crdordr"),
    ("kt10009", "/api/dostk/crdordr"),
    ("ust20000", "/api/us/ordr"),
    ("ust20001", "/api/us/ordr"),
    ("ust20002", "/api/us/ordr"),
    ("ust20003", "/api/us/ordr"),
    ("ust21200", "/api/us/ordr"),
    ("ust21201", "/api/us/ordr"),
    ("ust21203", "/api/us/ordr"),
    ("ust31302", "/api/us/exchange"),
];
fn text(value: &str, max: usize) -> Result<(), ClientError> {
    if value.trim().is_empty() || value.chars().count() > max {
        Err(ClientError::InvalidRequest(
            "required order field empty or too long",
        ))
    } else {
        Ok(())
    }
}

fn code(value: &str, values: &[&str]) -> Result<(), ClientError> {
    if values.contains(&value) {
        Ok(())
    } else {
        Err(ClientError::InvalidRequest("undocumented order code"))
    }
}

fn number(value: &str, allow_zero: bool, whole: bool) -> Result<(), ClientError> {
    let d = Decimal::from_str(value)
        .map_err(|_| ClientError::InvalidRequest("invalid decimal order value"))?;
    if d < Decimal::ZERO
        || (!allow_zero && d == Decimal::ZERO)
        || (whole && d.fract() != Decimal::ZERO)
    {
        return Err(ClientError::InvalidRequest("invalid order quantity/price"));
    }
    Ok(())
}

// 누락·빈 문자열은 선택 값으로 유지하고, 값이 있을 때만 양의 수인지 검증합니다.
fn optional_number(value: &Option<String>) -> Result<(), ClientError> {
    if let Some(v) = value
        && !v.is_empty()
    {
        number(v, false, false)?;
    }
    Ok(())
}

fn date(value: &str) -> Result<(), ClientError> {
    crate::transactions::BrokerDate::new(value).map(|_| ())
}

macro_rules! request {
    ($name:ident { $($required:ident:$max:literal),* ; $($optional:ident:$omax:literal),* } $validator:expr) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $required: String,)*
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $optional: Option<String>,
            )*
        }

        impl $name {
            pub fn validate(&self) -> Result<(), ClientError> {
                $(text(&self.$required, $max)?;)*
                $(
                    if let Some(v) = &self.$optional {
                        if v.chars().count() > $omax {
                            return Err(ClientError::InvalidRequest("optional order field too long"));
                        }
                    }
                )*
                ($validator)(self)
            }
        }
    };
}

macro_rules! response {
    ($name:ident { $($field:ident),* }) => {
        #[derive(Debug, Clone, Deserialize)]
        pub struct $name {
            $(pub $field: Option<String>,)*
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>
        }
    };
}

// 국내주식 현금·신용 주문 계약. 응답을 함께 쓰는 요청은 같은 계열에 둡니다.

const DOMESTIC_TYPES: &[&str] = &[
    "0", "3", "5", "81", "61", "6", "7", "10", "13", "16", "20", "23", "26", "28", "29", "30",
    "31", "40", "46", "47",
];

request!(
    Kt10000Request {
        dmst_stex_tp: 3,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12,
        cond_uv: 12
    }
    |r: &Kt10000Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        code(&r.trde_tp, DOMESTIC_TYPES)?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)?;
        optional_number(&r.cond_uv)
    }
);

response!(DomesticOrderResponse {
    ord_no,
    dmst_stex_tp
});

request!(
    Kt10001Request {
        dmst_stex_tp: 3,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12,
        cond_uv: 12
    }
    |r: &Kt10001Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        code(&r.trde_tp, DOMESTIC_TYPES)?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)?;
        optional_number(&r.cond_uv)
    }
);

request!(
    Kt10002Request {
        dmst_stex_tp: 3,
        orig_ord_no: 7,
        stk_cd: 12,
        mdfy_qty: 12,
        mdfy_uv: 12;
        mdfy_cond_uv: 12
    }
    |r: &Kt10002Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        number(&r.mdfy_qty, true, true)?;
        number(&r.mdfy_uv, false, false)?;
        optional_number(&r.mdfy_cond_uv)
    }
);

response!(DomesticModifyResponse {
    ord_no,
    base_orig_ord_no,
    mdfy_qty,
    dmst_stex_tp
});

request!(
    Kt10003Request {
        dmst_stex_tp: 3,
        orig_ord_no: 7,
        stk_cd: 12,
        cncl_qty: 12;
    }
    |r: &Kt10003Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        number(&r.cncl_qty, true, true)
    }
);

response!(CancelResponse {
    ord_no,
    base_orig_ord_no,
    cncl_qty
});

request!(
    Kt10006Request {
        dmst_stex_tp: 3,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12,
        cond_uv: 12
    }
    |r: &Kt10006Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        code(&r.trde_tp, DOMESTIC_TYPES)?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)?;
        optional_number(&r.cond_uv)
    }
);

request!(
    Kt10007Request {
        dmst_stex_tp: 3,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2,
        crd_deal_tp: 2;
        ord_uv: 12,
        crd_loan_dt: 8,
        cond_uv: 12
    }
    |r: &Kt10007Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        code(&r.trde_tp, DOMESTIC_TYPES)?;
        code(&r.crd_deal_tp, &["33", "99"])?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)?;
        optional_number(&r.cond_uv)?;
        if r.crd_deal_tp == "33" {
            date(
                r.crd_loan_dt
                    .as_deref()
                    .ok_or(ClientError::InvalidRequest("credit loan date required"))?
            )?;
        } else if let Some(d) = &r.crd_loan_dt
            && !d.is_empty()
        {
            date(d)?;
        }
        Ok(())
    }
);

request!(
    Kt10008Request {
        dmst_stex_tp: 3,
        orig_ord_no: 7,
        stk_cd: 12,
        mdfy_qty: 12,
        mdfy_uv: 12;
        mdfy_cond_uv: 12
    }
    |r: &Kt10008Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        number(&r.mdfy_qty, true, true)?;
        number(&r.mdfy_uv, false, false)?;
        optional_number(&r.mdfy_cond_uv)
    }
);

request!(
    Kt10009Request {
        dmst_stex_tp: 3,
        orig_ord_no: 7,
        stk_cd: 12,
        cncl_qty: 12;
    }
    |r: &Kt10009Request| {
        code(&r.dmst_stex_tp, &["KRX", "NXT", "SOR"])?;
        number(&r.cncl_qty, true, true)
    }
);

response!(CreditCancelResponse {
    ord_no,
    base_orig_ord_no,
    cncl_qty
});

// 금현물 주문·정정·취소 계약.

request!(
    Kt50000Request {
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12
    }
    |r: &Kt50000Request| {
        code(&r.stk_cd, &["M04020000", "M04020100"])?;
        code(&r.trde_tp, &["00", "10", "20"])?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)
    }
);

response!(GoldOrderResponse { ord_no });

request!(
    Kt50001Request {
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12
    }
    |r: &Kt50001Request| {
        code(&r.stk_cd, &["M04020000", "M04020100"])?;
        code(&r.trde_tp, &["00", "10", "20"])?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)
    }
);

request!(
    Kt50002Request {
        stk_cd: 12,
        orig_ord_no: 7,
        mdfy_qty: 12,
        mdfy_uv: 12;
    }
    |r: &Kt50002Request| {
        code(&r.stk_cd, &["M04020000", "M04020100"])?;
        number(&r.mdfy_qty, true, true)?;
        number(&r.mdfy_uv, false, false)
    }
);

response!(GoldModifyResponse {
    ord_no,
    base_orig_ord_no,
    mdfy_qty
});

request!(
    Kt50003Request {
        orig_ord_no: 7,
        stk_cd: 12,
        cncl_qty: 12;
    }
    |r: &Kt50003Request| {
        code(&r.stk_cd, &["M04020000", "M04020100"])?;
        number(&r.cncl_qty, true, true)
    }
);

// 미국주식 일반 주문·정정·취소 계약.

const US_BUY_TYPES: &[&str] = &["00", "03", "26", "27", "30", "36", "37"];
const US_SELL_TYPES: &[&str] = &["00", "03", "26", "27", "30", "33", "36", "37", "35", "34"];
fn us_price(
    tp: &str,
    price: &Option<String>,
    stop: Option<&Option<String>>,
) -> Result<(), ClientError> {
    optional_number(price)?;
    if ["00", "26", "27", "30", "34"].contains(&tp) && price.as_deref().is_none_or(str::is_empty) {
        return Err(ClientError::InvalidRequest(
            "documented limit type requires price",
        ));
    }
    if ["03", "33", "35", "36", "37"].contains(&tp)
        && price.as_deref().is_some_and(|s| !s.is_empty())
    {
        return Err(ClientError::InvalidRequest(
            "market type requires empty price",
        ));
    }
    if let Some(s) = stop {
        optional_number(s)?;
        if ["34", "35"].contains(&tp) && s.as_deref().is_none_or(str::is_empty) {
            return Err(ClientError::InvalidRequest("STOP type requires stop price"));
        }
    }
    Ok(())
}

request!(
    Ust20000Request {
        stex_tp: 2,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12
    }
    |r: &Ust20000Request| {
        code(&r.stex_tp, &["NA", "ND", "NY"])?;
        code(&r.trde_tp, US_BUY_TYPES)?;
        number(&r.ord_qty, false, true)?;
        us_price(&r.trde_tp, &r.ord_uv, None)
    }
);

response!(UsBuyResponse {
    stk_nm,
    ord_no,
    fc_entra,
    tdy_rebuy_useda,
    pred_rebuy_useda,
    trst_prof_ch
});

request!(
    Ust20001Request {
        stk_cd: 12,
        stex_tp: 2,
        ord_qty: 12,
        trde_tp: 2;
        ord_uv: 12,
        stop_pric: 12
    }
    |r: &Ust20001Request| {
        code(&r.stex_tp, &["NA", "ND", "NY"])?;
        code(&r.trde_tp, US_SELL_TYPES)?;
        number(&r.ord_qty, false, true)?;
        us_price(&r.trde_tp, &r.ord_uv, Some(&r.stop_pric))
    }
);

response!(UsSellResponse {
    stk_nm,
    ord_no,
    poss_qty,
    tdy_resel_usedq,
    pred_resel_usedq
});

request!(
    Ust20002Request {
        orig_ord_no: 9,
        stex_tp: 2,
        stk_cd: 12;
        mdfy_uv: 12,
        stop_pric: 12
    }
    |r: &Ust20002Request| {
        code(&r.stex_tp, &["NA", "ND", "NY"])?;
        optional_number(&r.mdfy_uv)?;
        optional_number(&r.stop_pric)
    }
);

response!(UsModifyResponse {
    stk_nm,
    ord_no,
    fc_entra,
    tdy_rebuy_useda,
    pred_rebuy_useda,
    trst_prof_ch,
    mdfy_ord_qty
});

request!(
    Ust20003Request {
        orig_ord_no: 9,
        stex_tp: 2,
        stk_cd: 12;
    }
    |r: &Ust20003Request| code(&r.stex_tp, &["NA", "ND", "NY"])
);

response!(UsCancelResponse {
    stk_nm,
    ord_no,
    cncl_ord_qty
});

// 미국주식 예약 주문과 예약 내역 조회 계약.

// 예약 계약이 공유하는 선택 날짜와 기간 순서를 기존 순서로 검사합니다.
fn reservation_dates(start: &Option<String>, end: &Option<String>) -> Result<(), ClientError> {
    for d in [start, end].into_iter().flatten() {
        if !d.is_empty() {
            date(d)?;
        }
    }
    if let (Some(s), Some(e)) = (start, end)
        && !s.is_empty()
        && !e.is_empty()
        && s > e
    {
        return Err(ClientError::InvalidRequest("reservation range reversed"));
    }
    Ok(())
}

fn reservation(tp: &str, start: &Option<String>, end: &Option<String>) -> Result<(), ClientError> {
    code(tp, &["1", "2", "3"])?;
    reservation_dates(start, end)
}

request!(
    Ust21200Request {
        rsrv_ord_tp: 1,
        stex_tp: 2,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        rsrv_strt_dt: 8,
        rsrv_end_dt: 8,
        ord_uv: 12
    }
    |r: &Ust21200Request| {
        reservation(&r.rsrv_ord_tp, &r.rsrv_strt_dt, &r.rsrv_end_dt)?;
        code(&r.stex_tp, &["ND", "NY", "NA"])?;
        code(&r.trde_tp, US_BUY_TYPES)?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)
    }
);

response!(ReservationResponse {
    rsrv_ord_no,
    frcs_dt
});

request!(
    Ust21201Request {
        rsrv_ord_tp: 1,
        stex_tp: 2,
        stk_cd: 12,
        ord_qty: 12,
        trde_tp: 2;
        rsrv_strt_dt: 8,
        rsrv_end_dt: 8,
        ord_uv: 12,
        stop_pric: 12
    }
    |r: &Ust21201Request| {
        reservation(&r.rsrv_ord_tp, &r.rsrv_strt_dt, &r.rsrv_end_dt)?;
        code(&r.stex_tp, &["ND", "NY", "NA"])?;
        code(&r.trde_tp, US_SELL_TYPES)?;
        number(&r.ord_qty, false, true)?;
        optional_number(&r.ord_uv)?;
        optional_number(&r.stop_pric)?;
        if ["34", "35"].contains(&r.trde_tp.as_str())
            && r.stop_pric.as_deref().is_none_or(str::is_empty)
        {
            return Err(ClientError::InvalidRequest(
                "STOP reservation requires stop price"
            ));
        }
        Ok(())
    }
);

request!(
    Ust21203Request {
        rsrv_dt: 8,
        rsrv_ord_no: 12,
        stex_tp: 2,
        stk_cd: 12;
    }
    |r: &Ust21203Request| {
        date(&r.rsrv_dt)?;
        code(&r.stex_tp, &["ND", "NY", "NA"])
    }
);

response!(ReservationCancelResponse { filler });

request!(
    Ust21204Request {
        rsrv_ord_tp: 1;
        ord_send_dt: 8
    }
    |r: &Ust21204Request| {
        code(&r.rsrv_ord_tp, &["0", "1", "2", "3"])?;
        if let Some(d) = &r.ord_send_dt
            && !d.is_empty()
        {
            date(d)?;
        }
        Ok(())
    }
);

response!(ReservationDayRow {
    rsrv_dt,
    ord_send_dt,
    stk_cd,
    rsrv_ord_no,
    rsrv_ord_tp,
    rsrv_ord_nm,
    stk_nm,
    ord_no,
    slby_tp,
    trde_tp,
    trde_nm,
    am_pm_tp,
    stex_nm,
    orig_ord_qty,
    accm_cntr_qty,
    ord_qty,
    ord_uv,
    rsrv_strt_dt,
    rsrv_end_dt,
    err_cntn
});

#[derive(Debug, Clone, Deserialize)]
pub struct Ust21204Response {
    pub result_list: Option<Vec<ReservationDayRow>>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

request!(
    Ust21205Request {
        ;
        fr_rsrv_dt: 8,
        to_rsrv_dt: 8,
        fr_rsrv_ord_no: 12,
        rsrv_ord_tp: 1,
        rsrv_cncl_yn: 1,
        rsrv_proc_tp: 1,
        slby_tp: 1,
        stex_tp: 2,
        stk_cd: 12,
        base_dt_tp: 1
    }
    |r: &Ust21205Request| {
        reservation_dates(&r.fr_rsrv_dt, &r.to_rsrv_dt)?;
        for (v, codes) in [
            (&r.rsrv_ord_tp, &["0", "1", "2", "3", "4"][..]),
            (&r.rsrv_cncl_yn, &["0", "Y", "N"][..]),
            (&r.rsrv_proc_tp, &["%", "0", "1"][..]),
            (&r.slby_tp, &["0", "1", "2"][..]),
            (&r.stex_tp, &["ND", "NY", "NA"][..]),
            (&r.base_dt_tp, &["0", "1"][..])
        ] {
            if let Some(v) = v
                && !v.is_empty()
            {
                code(v, codes)?;
            }
        }
        Ok(())
    }
);

response!(ReservationHistoryRow {
    rsrv_dt,
    rsrv_ord_no,
    rsrv_strt_dt,
    slby_tp,
    ord_qty,
    orig_ord_qty,
    rsrv_cncl_yn,
    stk_cd,
    stk_nm,
    stex_nm,
    rsrv_ord_tp,
    rsrv_ord_nm,
    rsrv_end_dt,
    trde_tp,
    trde_nm,
    ord_uv,
    stop_pric,
    accm_cntr_qty,
    am_pm_tp,
    proc_tp,
    err_cntn
});

#[derive(Debug, Clone, Deserialize)]
pub struct Ust21205Response {
    pub result_list: Option<Vec<ReservationHistoryRow>>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

// 미국주식 주문 가능 금액 조회 계약.

request!(
    Ust31490Request {
        stk_cd: 12,
        uv: 12;
        stex_tp: 2
    }
    |r: &Ust31490Request| {
        number(&r.uv, false, false)?;
        if let Some(v) = &r.stex_tp
            && !v.is_empty()
        {
            code(v, &["NA", "ND", "NY"])?;
        }
        Ok(())
    }
);

response!(Ust31490Response {
    stk_profa_rt,
    profa_rt,
    aplc_rt,
    krw_ord_rqst_yn,
    krw_ord_alowa_50,
    krw_ord_alowq_50,
    ord_alowa_50,
    ord_alowq_50,
    pred_rebuy_alowa_50,
    tdy_rebuy_alowa_50,
    krw_ord_alowa_100,
    krw_ord_alowq_100,
    ord_alowa_100,
    ord_alowq_100,
    pred_rebuy_alowa_100,
    tdy_rebuy_alowa_100,
    min_krw_ord_alowa,
    min_krw_ord_alowq,
    min_ord_alowa,
    min_ord_alowq,
    min_pred_rebuy_alowa,
    min_tdy_rebuy_alowa,
    krw_entra,
    fc_entra,
    fc_uncl_amt,
    ord_alowa,
    krw_ord_set_amt,
    krw_ord_evlt_amt,
    crnc_code
});

// 미국주식 환전 실행 계약.

request!(
    Ust31302Request {
        exch_tp: 1,
        fc_exmn_amt: 12;
    }
    |r: &Ust31302Request| {
        code(&r.exch_tp, &["1", "2"])?;
        number(&r.fc_exmn_amt, false, false)
    }
);

response!(Ust31302Response {
    sell_aplc_exrt,
    buy_aplc_exrt,
    aplc_exrt,
    entra_prerm,
    ch_uncla_prerm,
    etc_loana_prerm,
    entra_nowrm,
    ch_uncla_nowrm,
    etc_loana_nowrm,
    krw_exmn_alow_amt,
    ch_uncl_rpym_amt,
    ch_uncl_dlfe,
    etc_loan_npay_rpym_amt,
    etc_loan_npay_dlfe,
    fc_entra_prerm,
    fc_ch_uncla_prerm,
    fc_etc_loana_prerm,
    fc_entra_nowrm,
    fc_ch_uncla_nowrm,
    fc_etc_loana_nowrm,
    fc_exmn_alow_amt,
    fc_ch_uncl_rpym_amt,
    fc_ch_uncl_dlfe,
    fc_etc_loan_npay_rpym_amt,
    fc_etc_loan_npay_dlfe,
    krw_exmn_amt,
    sell_fc_amt,
    buy_fc_amt
});
