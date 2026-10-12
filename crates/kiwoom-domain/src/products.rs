//! 현재 공개 HTML 필드 표를 바탕으로 독립적으로 작성한 계약입니다.
//! 소수 값을 담는 증권사 문자열의 부호·패딩·소수 자릿수를 보존하며 경제적 의미를 변환하지
//! 않습니다.

use crate::{ClientError, transactions::BrokerDate};

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

fn required(v: &str, max: usize) -> Result<(), ClientError> {
    if v.trim().is_empty() || v.len() > max {
        return Err(ClientError::InvalidRequest(
            "required field empty or exceeds documented length",
        ));
    }
    Ok(())
}

fn code(v: &str, allowed: &[&str]) -> Result<(), ClientError> {
    if !allowed.contains(&v) {
        return Err(ClientError::InvalidRequest("undocumented code"));
    }
    Ok(())
}

fn stock(v: &str) -> Result<(), ClientError> {
    if v.len() != 6 || !v.bytes().all(|x| x.is_ascii_alphanumeric()) {
        return Err(ClientError::InvalidRequest(
            "stock code must contain six ASCII alphanumeric characters",
        ));
    }
    Ok(())
}

fn digits(v: &str, max: usize) -> Result<(), ClientError> {
    required(v, max)?;
    if !v.bytes().all(|x| x.is_ascii_digit()) {
        return Err(ClientError::InvalidRequest("numeric code expected"));
    }
    Ok(())
}

fn days(v: &str, max: usize) -> Result<(), ClientError> {
    digits(v, max)?;
    if !matches!(v.parse::<u8>(), Ok(1..=99)) {
        return Err(ClientError::InvalidRequest("days outside documented 1..99"));
    }
    Ok(())
}

fn amount(v: &str) -> Result<(), ClientError> {
    required(v, 12)?;
    let mut split = v.split('.');
    let integer = split.next().unwrap_or("");
    let fractional = split.next();
    if integer.is_empty()
        || !integer.bytes().all(|b| b.is_ascii_digit())
        || fractional.is_some_and(|f| f.is_empty() || !f.bytes().all(|b| b.is_ascii_digit()))
        || split.next().is_some()
    {
        return Err(ClientError::InvalidRequest(
            "amount must be an unsigned decimal string",
        ));
    }
    Ok(())
}

macro_rules! fields {
    ($name:ident {$($f:ident),* $(,)?}) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(default, skip_serializing_if="Option::is_none")]
                pub $f: Option<String>,
            )*
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>,
        }
    };
}

macro_rules! response {
    ($name:ident {$($f:ident),*} $list:ident: $row:ident) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(default, skip_serializing_if="Option::is_none")]
                pub $f: Option<String>,
            )*
            #[serde(default, skip_serializing_if="Option::is_none")]
            pub $list: Option<Vec<$row>>,
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>,
        }
    };
}

macro_rules! request {
    ($name:ident {$($f:ident:$ty:ty),* $(,)?} $r:ident $body:block) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $f: $ty,)*
        }

        impl $name {
            pub fn validate(&self) -> Result<(), ClientError> {
                let $r = self;
                $body;
                Ok(())
            }
        }
    };
}

macro_rules! stock_request {
    ($($name:ident),+) => {
        $(
            request!($name { stk_cd: String } r {
                stock(&r.stk_cd)?;
            });
        )+
    };
}

stock_request!(Ka10048Request);

fields!(Ka10048Row {
    dt,
    iv,
    delta,
    gam,
    theta,
    vega,
    law,
    lp
});

response!(Ka10048Response {} elwdaly_snst_ix:Ka10048Row);

stock_request!(Ka10050Request);

fields!(Ka10050Row {
    cntr_tm,
    cur_prc,
    elwtheory_pric,
    iv,
    delta,
    gam,
    theta,
    vega,
    law,
    lp
});

response!(Ka10050Response {} elwsnst_ix_array:Ka10050Row);

stock_request!(Ka30011Request);

fields!(Ka30011Row {
    stk_cd,
    stk_nm,
    cur_prc,
    pre_sig,
    pred_pre,
    flu_rt,
    acc_trde_qty,
    alacc_rt
});

response!(Ka30011Response {} elwalacc_rt:Ka30011Row);

stock_request!(Ka30012Request);

fields!(Ka30012Response {
    aset_cd,
    cur_prc,
    pred_pre_sig,
    pred_pre,
    flu_rt,
    lpmmcm_nm,
    lpmmcm_nm_1,
    lpmmcm_nm_2,
    elwrght_cntn,
    elwexpr_evlt_pric,
    elwtheory_pric,
    dispty_rt,
    elwinnr_vltl,
    exp_rght_pric,
    elwpl_qutr_rt,
    elwexec_pric,
    elwcnvt_rt,
    elwcmpn_rt,
    elwpric_rising_part_rt,
    elwrght_type,
    elwsrvive_dys,
    stkcnt,
    elwlpord_pos,
    lpposs_rt,
    lprmnd_qty,
    elwspread,
    elwprty,
    elwgear,
    elwflo_dt,
    elwfin_trde_dt,
    expr_dt,
    exec_dt,
    lpsuply_end_dt,
    elwpay_dt,
    elwinvt_ix_comput,
    elwpay_agnt,
    elwappr_way,
    elwrght_exec_way,
    elwpblicte_orgn,
    dcsn_pay_amt,
    kobarr,
    iv,
    clsprd_end_elwocr,
    bsis_aset_1,
    bsis_aset_comp_rt_1,
    bsis_aset_2,
    bsis_aset_comp_rt_2,
    bsis_aset_3,
    bsis_aset_comp_rt_3,
    bsis_aset_4,
    bsis_aset_comp_rt_4,
    bsis_aset_5,
    bsis_aset_comp_rt_5,
    fr_dt,
    to_dt,
    fr_tm,
    evlt_end_tm,
    evlt_pric,
    evlt_fnsh_yn,
    all_hgst_pric,
    all_lwst_pric,
    imaf_hgst_pric,
    imaf_lwst_pric,
    sndhalf_mrkt_hgst_pric,
    sndhalf_mrkt_lwst_pric
});

stock_request!(Ka40002Request);

fields!(Ka40002Response {
    stk_nm,
    etfobjt_idex_nm,
    wonju_pric,
    etftxon_type,
    etntxon_type
});

stock_request!(Ka40003Request);

fields!(Ka40003Row {
    cntr_dt,
    cur_prc,
    pre_sig,
    pred_pre,
    pre_rt,
    trde_qty,
    nav,
    acc_trde_prica,
    navidex_dispty_rt,
    navetfdispty_rt,
    trace_eor_rt,
    trace_cur_prc,
    trace_pred_pre,
    trace_pre_sig
});

response!(Ka40003Response {} etfdaly_trnsn:Ka40003Row);

stock_request!(Ka40006Request);

fields!(Ka40006Row {
    tm,
    close_pric,
    pre_sig,
    pred_pre,
    flu_rt,
    trde_qty,
    nav,
    trde_prica,
    navidex,
    navetf,
    trace,
    trace_idex,
    trace_idex_pred_pre,
    trace_idex_pred_pre_sig
});

response!(Ka40006Response {stk_nm,etfobjt_idex_nm,wonju_pric,etftxon_type,etntxon_type} etftisl_trnsn:Ka40006Row);

stock_request!(Ka40007Request);

fields!(Ka40007Row {
    cntr_tm,
    cur_prc,
    pre_sig,
    pred_pre,
    trde_qty,
    stex_tp
});

response!(Ka40007Response {
    stk_cls,
    stk_nm,
    etfobjt_idex_nm,
    etfobjt_idex_cd,
    objt_idex_pre_rt,
    wonju_pric
} etftisl_cntr_array:Ka40007Row);

stock_request!(Ka40008Request);

fields!(Ka40008Row {
    dt,
    cur_prc_n,
    pre_sig_n,
    pred_pre_n,
    acc_trde_qty,
    for_netprps_qty,
    orgn_netprps_qty
});

response!(Ka40008Response {cntr_tm,cur_prc,pre_sig,pred_pre,trde_qty} etfnetprps_qty_array:Ka40008Row);

stock_request!(Ka40009Request);

fields!(Ka40009Row {
    nav,
    navpred_pre,
    navflu_rt,
    trace_eor_rt,
    dispty_rt,
    stkcnt,
    base_pric,
    for_rmnd_qty,
    repl_pric,
    conv_pric,
    drstk,
    wonju_pric
});

response!(Ka40009Response {} etfnavarray:Ka40009Row);

stock_request!(Ka40010Request);

fields!(Ka40010Row {
    cur_prc,
    pre_sig,
    pred_pre,
    trde_qty,
    for_netprps
});

response!(Ka40010Response {} etftisl_trnsn:Ka40010Row);

request!(
    Ka30001Request {
        flu_tp: String,
        tm_tp: String,
        tm: String,
        trde_qty_tp: String,
        isscomp_cd: String,
        bsis_aset_cd: String,
        rght_tp: String,
        lpcd: String,
        trde_end_elwskip: String
    }
    r {
        code(&r.flu_tp, &["1", "2"])?;
        code(&r.tm_tp, &["1", "2"])?;
        days(&r.tm, 2)?;
        code(
            &r.trde_qty_tp,
            &["0", "10", "50", "100", "300", "500", "1000"]
        )?;
        digits(&r.isscomp_cd, 12)?;
        digits(&r.bsis_aset_cd, 12)?;
        digits(&r.lpcd, 12)?;
        code(
            &r.rght_tp,
            &["000", "001", "002", "003", "004", "005", "006", "007"]
        )?;
        code(&r.trde_end_elwskip, &["0", "1"])?;
    }
);

fields!(Ka30001Row {
    stk_cd,
    rank,
    stk_nm,
    pre_sig,
    pred_pre,
    trde_end_elwbase_pric,
    cur_prc,
    base_pre,
    trde_qty,
    jmp_rt
});

response!(Ka30001Response {base_pric_tm} elwpric_jmpflu:Ka30001Row);

request!(
    Ka30002Request {
        isscomp_cd: String,
        trde_qty_tp: String,
        trde_tp: String,
        dt: String,
        trde_end_elwskip: String
    }
    r {
        digits(&r.isscomp_cd, 3)?;
        if r.isscomp_cd.len() != 3 {
            return Err(ClientError::InvalidRequest(
                "issuer code must be three digits"
            ));
        }
        code(
            &r.trde_qty_tp,
            &["0", "5", "10", "50", "100", "500", "1000"]
        )?;
        code(&r.trde_tp, &["1", "2"])?;
        code(&r.dt, &["1", "5", "10", "40", "60"])?;
        code(&r.trde_end_elwskip, &["0", "1"])?;
    }
);

fields!(Ka30002Row {
    stk_cd,
    stk_nm,
    stkpc_flu,
    flu_rt,
    trde_qty,
    netprps,
    buy_trde_qty,
    sel_trde_qty
});

response!(Ka30002Response {} trde_ori_elwnettrde_upper:Ka30002Row);

request!(
    Ka30003Request {
        bsis_aset_cd: String,
        base_dt: BrokerDate
    }
    r {
        stock(&r.bsis_aset_cd)?;
    }
);

fields!(Ka30003Row {
    dt,
    cur_prc,
    pre_tp,
    pred_pre,
    flu_rt,
    trde_qty,
    trde_prica,
    chg_qty,
    lprmnd_qty,
    wght
});

response!(Ka30003Response {} elwlpposs_daly_trnsn:Ka30003Row);

request!(
    Ka30004Request {
        isscomp_cd: String,
        bsis_aset_cd: String,
        rght_tp: String,
        lpcd: String,
        trde_end_elwskip: String
    }
    r {
        digits(&r.isscomp_cd, 12)?;
        digits(&r.bsis_aset_cd, 12)?;
        digits(&r.lpcd, 12)?;
        code(
            &r.rght_tp,
            &["000", "001", "002", "003", "004", "005", "006", "007"]
        )?;
        code(&r.trde_end_elwskip, &["0", "1"])?;
    }
);

fields!(Ka30004Row {
    stk_cd,
    isscomp_nm,
    sqnc,
    base_aset_nm,
    rght_tp,
    dispty_rt,
    basis,
    srvive_dys,
    theory_pric,
    cur_prc,
    pre_tp,
    pred_pre,
    flu_rt,
    trde_qty,
    stk_nm
});

response!(Ka30004Response {} elwdispty_rt:Ka30004Row);

request!(
    Ka30005Request {
        isscomp_cd: String,
        bsis_aset_cd: String,
        rght_tp: String,
        lpcd: String,
        sort_tp: String
    }
    r {
        digits(&r.isscomp_cd, 12)?;
        if r.isscomp_cd.len() != 12 {
            return Err(ClientError::InvalidRequest(
                "issuer code must be twelve digits"
            ));
        }
        digits(&r.bsis_aset_cd, 12)?;
        digits(&r.lpcd, 12)?;
        code(&r.rght_tp, &["0", "1", "2", "3", "4", "5", "6", "7"])?;
        code(&r.sort_tp, &["0", "1", "2", "3", "4", "5", "6", "7"])?;
    }
);

fields!(Ka30005Row {
    stk_cd,
    isscomp_nm,
    sqnc,
    base_aset_nm,
    rght_tp,
    expr_dt,
    cur_prc,
    pre_tp,
    pred_pre,
    flu_rt,
    trde_qty,
    trde_qty_pre,
    trde_prica,
    pred_trde_qty,
    sel_bid,
    buy_bid,
    prty,
    gear_rt,
    pl_qutr_rt,
    cfp,
    theory_pric,
    innr_vltl,
    delta,
    lvrg,
    exec_pric,
    cnvt_rt,
    lpposs_rt,
    pl_qutr_pt,
    fin_trde_dt,
    flo_dt,
    lpinitlast_suply_dt,
    stk_nm,
    srvive_dys,
    dispty_rt,
    lpmmcm_nm,
    lpmmcm_nm_1,
    lpmmcm_nm_2,
    xraymont_cntr_qty_arng_trde_tp,
    xraymont_cntr_qty_profa_100tp
});

response!(Ka30005Response {} elwcnd_qry:Ka30005Row);

request!(
    Ka30009Request {
        sort_tp: String,
        rght_tp: String,
        trde_end_skip: String
    }
    r {
        code(&r.sort_tp, &["1", "2", "3", "4"])?;
        code(
            &r.rght_tp,
            &["000", "001", "002", "003", "004", "006", "007"]
        )?;
        code(&r.trde_end_skip, &["0", "1"])?;
    }
);

fields!(Ka30009Row {
    rank,
    stk_cd,
    stk_nm,
    cur_prc,
    pre_sig,
    pred_pre,
    flu_rt,
    sel_req,
    buy_req,
    trde_qty,
    trde_prica
});

response!(Ka30009Response {} elwflu_rt_rank:Ka30009Row);

request!(
    Ka30010Request {
        sort_tp: String,
        rght_tp: String,
        trde_end_skip: String
    }
    r {
        code(&r.sort_tp, &["1", "2"])?;
        code(
            &r.rght_tp,
            &["000", "001", "002", "003", "004", "006", "007"]
        )?;
        code(&r.trde_end_skip, &["0", "1"])?;
    }
);

fields!(Ka30010Row {
    stk_cd,
    rank,
    stk_nm,
    cur_prc,
    pre_sig,
    pred_pre,
    flu_rt,
    trde_qty,
    sel_req,
    buy_req,
    netprps_req,
    trde_prica
});

response!(Ka30010Response {} elwreq_rank:Ka30010Row);

request!(
    Ka40001Request {
        stk_cd: String,
        etfobjt_idex_cd: String,
        dt: String
    }
    r {
        stock(&r.stk_cd)?;
        digits(&r.etfobjt_idex_cd, 3)?;
        if r.etfobjt_idex_cd.len() != 3 {
            return Err(ClientError::InvalidRequest(
                "ETF index code must be three digits"
            ));
        }
        code(&r.dt, &["0", "1", "2", "3"])?;
    }
);

fields!(Ka40001Row {
    etfprft_rt,
    cntr_prft_rt,
    for_netprps_qty,
    orgn_netprps_qty
});

response!(Ka40001Response {} etfprft_rt_lst:Ka40001Row);

request!(
    Ka40004Request {
        txon_type: String,
        navpre: String,
        mngmcomp: String,
        txon_yn: String,
        trace_idex: String,
        stex_tp: String
    }
    r {
        code(&r.txon_type, &["0", "1", "2", "3", "4", "5"])?;
        code(&r.navpre, &["0", "1", "2"])?;
        digits(&r.mngmcomp, 4)?;
        if r.mngmcomp.len() != 4 {
            return Err(ClientError::InvalidRequest(
                "manager code must be four digits"
            ));
        }
        code(&r.txon_yn, &["0", "1", "2"])?;
        code(&r.trace_idex, &["0"])?;
        code(&r.stex_tp, &["1", "2", "3"])?;
    }
);

fields!(Ka40004Row {
    stk_cd,
    stk_cls,
    stk_nm,
    close_pric,
    pre_sig,
    pred_pre,
    pre_rt,
    trde_qty,
    nav,
    trace_eor_rt,
    txbs,
    dvid_bf_base,
    pred_dvida,
    trace_idex_nm,
    drng,
    trace_idex_cd,
    trace_idex,
    trace_flu_rt
});

response!(Ka40004Response {} etfall_mrpr:Ka40004Row);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka90001Request {
    pub qry_tp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    pub date_tp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thema_nm: Option<String>,
    pub flu_pl_amt_tp: String,
    pub stex_tp: String,
}

impl Ka90001Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        code(&self.qry_tp, &["0", "2"])?;
        days(&self.date_tp, 2)?;
        code(&self.flu_pl_amt_tp, &["1", "2", "3", "4"])?;
        code(&self.stex_tp, &["1", "2", "3"])?;
        if let Some(s) = &self.stk_cd {
            stock(s)?;
        }
        if self.qry_tp == "2" && self.stk_cd.is_none() {
            return Err(ClientError::InvalidRequest(
                "stock search requires stock code",
            ));
        }
        if self.thema_nm.as_ref().is_some_and(|s| s.len() > 50) {
            return Err(ClientError::InvalidRequest(
                "theme exceeds documented length",
            ));
        }
        Ok(())
    }
}

fields!(Ka90001Row {
    thema_grp_cd,
    thema_nm,
    stk_num,
    flu_sig,
    flu_rt,
    rising_stk_num,
    fall_stk_num,
    dt_prft_rt,
    main_stk
});

response!(Ka90001Response {} thema_grp:Ka90001Row);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka90002Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_tp: Option<String>,
    pub thema_grp_cd: String,
    pub stex_tp: String,
}

impl Ka90002Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        // 표의 길이는 1이지만 설명에 1..99 범위가 명시되어 있으므로 그 범위를 따릅니다.
        if let Some(v) = &self.date_tp {
            days(v, 2)?;
        }
        digits(&self.thema_grp_cd, 6)?;
        code(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields!(Ka90002Row {
    stk_cd,
    stk_nm,
    cur_prc,
    flu_sig,
    pred_pre,
    flu_rt,
    acc_trde_qty,
    sel_bid,
    sel_req,
    buy_bid,
    buy_req,
    dt_prft_rt_n
});

response!(Ka90002Response {flu_rt,dt_prft_rt} thema_comp_stk:Ka90002Row);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa24300Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<String>,
}

impl Usa24300Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(v) = &self.qry_tp {
            code(v, &["0", "1"])?;
        }
        Ok(())
    }
}

fields!(Usa24300Row {
    stex_tp,
    stk_cd,
    stk_nm,
    stk_enm,
    cur_prc,
    pred_pre_sig,
    pred_pre,
    flu_rt,
    acc_trde_qty
});

response!(Usa24300Response {} result_list:Usa24300Row);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust31300Request {
    pub exch_tp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_exmn_amt: Option<String>,
}

impl Ust31300Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        code(&self.exch_tp, &["1", "2"])?;
        if let Some(v) = &self.fc_exmn_amt {
            amount(v)?;
        }
        Ok(())
    }
}

// 현재 환율·가용성만 나타내며, 이 필드로 과거 원장 환율을 확정하지 않습니다.
fields!(Ust31300Response {
    sell_aplc_exrt,
    buy_aplc_exrt,
    aplc_exrt,
    sell_crnc_entra_nowrm,
    sell_crnc_ch_uncla_nowrm,
    sell_crnc_etc_loana_nowrm,
    sell_crnc_exmn_alow_amt,
    buy_crnc_entra_nowrm,
    buy_crnc_ch_uncla_nowrm,
    buy_crnc_etc_loana_nowrm,
    buy_crnc_exmn_alow_amt,
    krw_uncl_amt,
    sell_expc_amt,
    buy_expc_amt
});

request!(
    Ust31301Request {
        exch_tp: String
    }
    r {
        code(&r.exch_tp, &["1", "2"])?;
    }
);

fields!(Ust31301Response {
    sell_aplc_exrt,
    buy_aplc_exrt,
    aplc_exrt,
    exrt_tp_nm,
    spcl_bf_exrt,
    exrt_spcl_rt
});

/// 상품 조회 API의 식별자와 정확한 HTTP 경로.
pub const CONTRACTS: &[(&str, &str)] = &[
    ("ka10048", "/api/dostk/elw"),
    ("ka10050", "/api/dostk/elw"),
    ("ka30001", "/api/dostk/elw"),
    ("ka30002", "/api/dostk/elw"),
    ("ka30003", "/api/dostk/elw"),
    ("ka30004", "/api/dostk/elw"),
    ("ka30005", "/api/dostk/elw"),
    ("ka30009", "/api/dostk/elw"),
    ("ka30010", "/api/dostk/elw"),
    ("ka30011", "/api/dostk/elw"),
    ("ka30012", "/api/dostk/elw"),
    ("ka40001", "/api/dostk/etf"),
    ("ka40002", "/api/dostk/etf"),
    ("ka40003", "/api/dostk/etf"),
    ("ka40004", "/api/dostk/etf"),
    ("ka40006", "/api/dostk/etf"),
    ("ka40007", "/api/dostk/etf"),
    ("ka40008", "/api/dostk/etf"),
    ("ka40009", "/api/dostk/etf"),
    ("ka40010", "/api/dostk/etf"),
    ("ka90001", "/api/dostk/thme"),
    ("ka90002", "/api/dostk/thme"),
    ("usa24300", "/api/us/invtinfo"),
    ("ust31300", "/api/us/exchange"),
    ("ust31301", "/api/us/exchange"),
];
