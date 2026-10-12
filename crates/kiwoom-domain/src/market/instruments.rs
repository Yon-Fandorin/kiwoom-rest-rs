//! 국내·미국 종목 목록, 메모와 기본 시세 조회 계약.
//!
//! 숫자처럼 보이는 응답도 원본 문자열로 보존하며 금융 값으로 환산하지 않는다.

use crate::{
    ClientError,
    transactions::{BrokerDate, UsExchange},
};

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use super::{bounded, nonempty, required_choice};

// 생략값과 빈 문자열은 허용하고, 값이 있으면 HHMM 형식을 검사한다.
fn hour_minute(v: &Option<String>) -> Result<(), ClientError> {
    if let Some(s) = v
        && !s.is_empty()
        && (s.len() != 4
            || !s.bytes().all(|b| b.is_ascii_digit())
            || chrono::NaiveTime::parse_from_str(s, "%H%M").is_err())
    {
        return Err(ClientError::InvalidRequest("time must be empty or HHMM"));
    }
    Ok(())
}

// 종목·메모·시세 요청과 API별 입력 검증.
stock_request!(Ka10001Request, 20);

// 응답 필드는 숫자처럼 보이는 값도 원본 문자열로 유지한다.
fields! {
    Ka10001Response {
        stk_cd,
        stk_nm,
        setl_mm,
        fav,
        cap,
        flo_stk,
        crd_rt,
        oyr_hgst,
        oyr_lwst,
        mac,
        mac_wght,
        for_exh_rt,
        repl_pric,
        per,
        eps,
        roe,
        pbr,
        ev,
        bps,
        sale_amt,
        bus_pro,
        cup_nga,
        h250 => "250hgst",
        l250 => "250lwst",
        high_pric,
        open_pric,
        low_pric,
        upl_pric,
        lst_pric,
        base_pric,
        exp_cntr_pric,
        exp_cntr_qty,
        h250_dt => "250hgst_pric_dt",
        h250_pre_rt => "250hgst_pric_pre_rt",
        l250_dt => "250lwst_pric_dt",
        l250_pre_rt => "250lwst_pric_pre_rt",
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        trde_pre,
        fav_unit,
        dstr_stk,
        dstr_rt,
    }
}

stock_request!(Ka10002Request, 20);

fields! {
    Ka10002Response {
        stk_cd,
        stk_nm,
        cur_prc,
        flu_smbol,
        base_pric,
        pred_pre,
        flu_rt,
        sel_trde_ori_nm_1,
        sel_trde_ori_1,
        sel_trde_qty_1,
        buy_trde_ori_nm_1,
        buy_trde_ori_1,
        buy_trde_qty_1,
        sel_trde_ori_nm_2,
        sel_trde_ori_2,
        sel_trde_qty_2,
        buy_trde_ori_nm_2,
        buy_trde_ori_2,
        buy_trde_qty_2,
        sel_trde_ori_nm_3,
        sel_trde_ori_3,
        sel_trde_qty_3,
        buy_trde_ori_nm_3,
        buy_trde_ori_3,
        buy_trde_qty_3,
        sel_trde_ori_nm_4,
        sel_trde_ori_4,
        sel_trde_qty_4,
        buy_trde_ori_nm_4,
        buy_trde_ori_4,
        buy_trde_qty_4,
        sel_trde_ori_nm_5,
        sel_trde_ori_5,
        sel_trde_qty_5,
        buy_trde_ori_nm_5,
        buy_trde_ori_5,
        buy_trde_qty_5,
    }
}

stock_request!(Ka10003Request, 20);

fields! {
    DomesticExecution {
        tm,
        cur_prc,
        pred_pre,
        pre_rt,
        pri_sel_bid_unit,
        pri_buy_bid_unit,
        cntr_trde_qty,
        sign,
        acc_trde_qty,
        acc_trde_prica,
        cntr_str,
        stex_tp,
    }
}

collection!(Ka10003Response, cntr_infr, DomesticExecution);

/// `ka10084`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10084Request {
    pub stk_cd: String,

    pub tdy_pred: String,

    pub tic_min: String,

    /// `tm` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm: Option<String>,
}

impl Ka10084Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.tdy_pred, &["1", "2"])?;
        required_choice(&self.tic_min, &["0", "1"])?;
        hour_minute(&self.tm)?;
        Ok(())
    }
}

collection!(Ka10084Response, tdy_pred_cntr, DomesticExecution);

stock_request!(Ka10004Request, 20);

fields! {
    Ka10004Response {
        bid_req_base_tm,
        sel_10th_pre_req_pre,
        sel_10th_pre_req,
        sel_10th_pre_bid,
        sel_9th_pre_req_pre,
        sel_9th_pre_req,
        sel_9th_pre_bid,
        sel_8th_pre_req_pre,
        sel_8th_pre_req,
        sel_8th_pre_bid,
        sel_7th_pre_req_pre,
        sel_7th_pre_req,
        sel_7th_pre_bid,
        sel_6th_pre_req_pre,
        sel_6th_pre_req,
        sel_6th_pre_bid,
        sel_5th_pre_req_pre,
        sel_5th_pre_req,
        sel_5th_pre_bid,
        sel_4th_pre_req_pre,
        sel_4th_pre_req,
        sel_4th_pre_bid,
        sel_3th_pre_req_pre,
        sel_3th_pre_req,
        sel_3th_pre_bid,
        sel_2th_pre_req_pre,
        sel_2th_pre_req,
        sel_2th_pre_bid,
        sel_1th_pre_req_pre,
        sel_fpr_req,
        sel_fpr_bid,
        buy_fpr_bid,
        buy_fpr_req,
        buy_1th_pre_req_pre,
        buy_2th_pre_bid,
        buy_2th_pre_req,
        buy_2th_pre_req_pre,
        buy_3th_pre_bid,
        buy_3th_pre_req,
        buy_3th_pre_req_pre,
        buy_4th_pre_bid,
        buy_4th_pre_req,
        buy_4th_pre_req_pre,
        buy_5th_pre_bid,
        buy_5th_pre_req,
        buy_5th_pre_req_pre,
        buy_6th_pre_bid,
        buy_6th_pre_req,
        buy_6th_pre_req_pre,
        buy_7th_pre_bid,
        buy_7th_pre_req,
        buy_7th_pre_req_pre,
        buy_8th_pre_bid,
        buy_8th_pre_req,
        buy_8th_pre_req_pre,
        buy_9th_pre_bid,
        buy_9th_pre_req,
        buy_9th_pre_req_pre,
        buy_10th_pre_bid,
        buy_10th_pre_req,
        buy_10th_pre_req_pre,
        tot_sel_req_jub_pre,
        tot_sel_req,
        tot_buy_req,
        tot_buy_req_jub_pre,
        ovt_sel_req_pre,
        ovt_sel_req,
        ovt_buy_req,
        ovt_buy_req_pre,
    }
}

stock_request!(Ka10005Request, 20);

fields! {
    DomesticPeriod {
        date,
        open_pric,
        high_pric,
        low_pric,
        close_pric,
        pre,
        flu_rt,
        trde_qty,
        trde_prica,
        for_poss,
        for_wght,
        for_netprps,
        orgn_netprps,
        ind_netprps,
        crd_remn_rt,
        frgn,
        prm,
    }
}

collection!(Ka10005Response, stk_ddwkmm, DomesticPeriod);

stock_request!(Ka10006Request, 20);

fields! {
    Ka10006Response {
        date,
        open_pric,
        high_pric,
        low_pric,
        close_pric,
        pre,
        flu_rt,
        trde_qty,
        trde_prica,
        cntr_str,
    }
}

/// `ka10100`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10100Request {
    pub stk_cd: String,
}

impl Ka10100Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if self.stk_cd.len() != 6 || !self.stk_cd.bytes().all(|b| b.is_ascii_digit()) {
            return Err(ClientError::InvalidRequest("stock code must be six digits"));
        }
        Ok(())
    }
}

fields! {
    Ka10100Response {
        code,
        name,
        list_count => "listCount",
        audit_info => "auditInfo",
        reg_day => "regDay",
        last_price => "lastPrice",
        state,
        market_code => "marketCode",
        market_name => "marketName",
        up_name => "upName",
        up_size_name => "upSizeName",
        company_class_name => "companyClassName",
        order_warning => "orderWarning",
        nxt_enable => "nxtEnable",
    }
}

// 목록 조회의 선택값은 공개 문자열과 일대일로 대응한다.
wire_enum! {
    StockListMarket {
        Kospi => "0",
        Kosdaq => "10",
        Kotc => "30",
        Konex => "50",
        Etn => "60",
        LimitedLossEtn => "70",
        Gold => "80",
        VolatilityEtn => "90",
        Infrastructure => "2",
        Elw => "3",
        MutualFund => "4",
        SubscriptionRights => "5",
        Reit => "6",
        SubscriptionCertificate => "7",
        Etf => "8",
        HighYieldFund => "9",
    }
}

/// `ka10099`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10099Request {
    pub mrkt_tp: StockListMarket,
}

fields! {
    DomesticStock {
        code,
        name,
        list_count => "listCount",
        audit_info => "auditInfo",
        reg_day => "regDay",
        last_price => "lastPrice",
        state,
        market_code => "marketCode",
        market_name => "marketName",
        up_name => "upName",
        up_size_name => "upSizeName",
        company_class_name => "companyClassName",
        order_warning => "orderWarning",
        nxt_enable => "nxtEnable",
    }
}

collection!(Ka10099Response, list, DomesticStock);

/// `ka10102`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10102Request {}

fields! {
    DomesticMember {
        code,
        name,
        gb,
    }
}

collection!(Ka10102Response, list, DomesticMember);

/// 국내 종목 메모 조회에 전달하는 종목 입력 행.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DomesticMemoInput {
    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

/// `ka10103`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10103Request {
    /// `input_list` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_list: Option<Vec<DomesticMemoInput>>,
}

impl Ka10103Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        for row in self.input_list.iter().flatten() {
            if let Some(code) = &row.stk_cd {
                bounded(code, 12)?;
            }
        }
        Ok(())
    }
}

fields! {
    DomesticMemo {
        stk_cd,
        memo,
    }
}

/// `ka10103`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10103Response {
    /// `rtcd` 원본 응답 값. 생략값을 보존한다.
    pub rtcd: Option<String>,

    /// `result_list` 원본 응답 값. 생략값을 보존한다.
    pub result_list: Option<Vec<DomesticMemo>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa10098`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa10098Request {
    pub stk_cd: String,
}

impl Usa10098Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        nonempty(&self.stk_cd)
    }
}

fields! {
    UsStock {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        mkgb,
        upgb,
        is_etf => "isEtf",
    }
}

collection!(Usa10098Response, list, UsStock);

wire_enum! {
    UsExchangeFilter {
        All => "%",
        Amex => "NA",
        Nasdaq => "ND",
        Nyse => "NY",
    }
}

/// `usa10099`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa10099Request {
    pub stex_tp: UsExchangeFilter,
}

collection!(Usa10099Response, list, UsStock);

/// `usa10100`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa10100Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchange>,

    pub stk_cd: String,
}

impl Usa10100Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        nonempty(&self.stk_cd)
    }
}

fields! {
    Usa10100Response {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        mkgb,
        upgb,
        is_etf => "isEtf",
    }
}

wire_enum! {
    UsRegionFilter {
        All => "%",
        UnitedStates => "1",
    }
}

/// `usa10101`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10101Request {
    /// `gubun` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gubun: Option<UsRegionFilter>,
}

fields! {
    UsIndustry {
        inds_cd,
        inds_nm,
        inds_enm,
    }
}

collection!(Usa10101Response, list, UsIndustry);

wire_enum! {
    UsIndexFilter {
        All => "%",
        Nasdaq => "NQ",
        Sp500 => "NS",
        Dow => "NW",
    }
}

/// `usa10102`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10102Request {
    /// `index_qry_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_qry_tp: Option<UsIndexFilter>,
}

fields! {
    UsIndex {
        index_cd,
        index_nm,
        index_enm,
    }
}

collection!(Usa10102Response, list, UsIndex);

/// 미국 종목 메모 조회에 전달하는 종목·거래소 입력 행.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsMemoInput {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchange>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

/// `usa10103`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10103Request {
    /// `input_list` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_list: Option<Vec<UsMemoInput>>,
}

impl Usa10103Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        for row in self.input_list.iter().flatten() {
            if let Some(code) = &row.stk_cd {
                bounded(code, 18)?;
            }
        }
        Ok(())
    }
}

fields! {
    UsMemo {
        stex_tp,
        stk_cd,
        memo,
    }
}

/// `usa10103`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10103Response {
    /// `rtcd` 원본 응답 값. 생략값을 보존한다.
    pub rtcd: Option<String>,

    /// `result_list` 원본 응답 값. 생략값을 보존한다.
    pub result_list: Option<Vec<UsMemo>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `usa10104`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10104Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchangeFilter>,
}

fields! {
    UsEtf {
        stex_tp,
        code,
        cate1,
        cate2,
        etn,
        mkgb,
        upnm,
    }
}

collection!(Usa10104Response, list, UsEtf);

wire_enum! {
    UsCategoryFilter {
        All => "%",
        First => "1",
        Second => "2",
    }
}

/// `usa10105`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa10105Request {
    /// `gubun` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gubun: Option<UsCategoryFilter>,
}

fields! {
    UsEtfCategory {
        gubun,
        cate1,
        cate1nam,
        cate2,
        cate2nam,
    }
}

collection!(Usa10105Response, list, UsEtfCategory);

us_quote_request!(Usa20100Request);

fields! {
    Usa20100Response {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        h52_pric => "52wk_hgst_pric",
        h52_pre_rt => "52wk_hgst_pric_pre_rt",
        h52_dt => "52wk_hgst_pric_dt",
        l52_pric => "52wk_lwst_pric",
        l52_pre_rt => "52wk_lwst_pric_pre_rt",
        l52_dt => "52wk_lwst_pric_dt",
        stk_cnt,
        mac,
        setl_mm,
        lg_inds_cd,
        sm_inds_cd,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        oyr_hgst,
        oyr_hgst_dt,
        oyr_hgst_pre_rt,
        oyr_lwst,
        oyr_lwst_dt,
        oyr_lwst_pre_rt,
        pre_open_pric,
        pre_high_pric,
        pre_low_pric,
        base_close_pric,
        upl_pric,
        lst_pric,
        trde_qty_unit,
        uncert_lv,
        comp_adv_tp,
        curr_unit,
        open_pric,
        high_pric,
        low_pric,
        trd_susp_tp,
        base_exrt,
    }
}

us_quote_request!(Usa20101Request);

fields! {
    Usa20101Response {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre,
        flu_rt,
        trde_qty,
        trde_prica,
        open_pric,
        high_pric,
        low_pric,
        bid_tm,
        dt,
        pred_pre_sig,
        fpr_sel_bid,
        fpr_buy_bid,
        pre_trde_rt,
        trde_tern_rt,
        sel_1bid,
        sel_2bid,
        sel_3bid,
        sel_4bid,
        sel_5bid,
        sel_6bid,
        sel_7bid,
        sel_8bid,
        sel_9bid,
        sel_10bid,
        buy_1bid,
        buy_2bid,
        buy_3bid,
        buy_4bid,
        buy_5bid,
        buy_6bid,
        buy_7bid,
        buy_8bid,
        buy_9bid,
        buy_10bid,
        sel_1bid_req,
        sel_2bid_req,
        sel_3bid_req,
        sel_4bid_req,
        sel_5bid_req,
        sel_6bid_req,
        sel_7bid_req,
        sel_8bid_req,
        sel_9bid_req,
        sel_10bid_req,
        buy_1bid_req,
        buy_2bid_req,
        buy_3bid_req,
        buy_4bid_req,
        buy_5bid_req,
        buy_6bid_req,
        buy_7bid_req,
        buy_8bid_req,
        buy_9bid_req,
        buy_10bid_req,
        sel_1bid_jub_pre,
        sel_2bid_jub_pre,
        sel_3bid_jub_pre,
        sel_4bid_jub_pre,
        sel_5bid_jub_pre,
        sel_6bid_jub_pre,
        sel_7bid_jub_pre,
        sel_8bid_jub_pre,
        sel_9bid_jub_pre,
        sel_10bid_jub_pre,
        buy_1bid_jub_pre,
        buy_2bid_jub_pre,
        buy_3bid_jub_pre,
        buy_4bid_jub_pre,
        buy_5bid_jub_pre,
        buy_6bid_jub_pre,
        buy_7bid_jub_pre,
        buy_8bid_jub_pre,
        buy_9bid_jub_pre,
        buy_10bid_jub_pre,
        buy_1th_pre_req_pre,
        tot_sel_req,
        sel_bid_tot_req_jub_pre,
        tot_buy_req,
        buy_bid_tot_req_jub_pre,
        netprps_req,
        netslmt_req,
        upl_pric,
        base_pric,
    }
}

us_quote_request!(Usa20150Request);

fields! {
    UsExecution {
        cur_prc,
        pred_pre,
        flu_rt,
        trde_qty,
        cntr_tm,
        pred_pre_sig,
    }
}

collection!(Usa20150Response, result_list, UsExecution);

/// `usa20151`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usa20151Request {
    pub stex_tp: UsExchange,

    pub stk_cd: String,

    /// `base_dt` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<BrokerDate>,
}

impl Usa20151Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        nonempty(&self.stk_cd)
    }
}

fields! {
    UsDailyQuote {
        dt,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
    }
}

collection!(Usa20151Response, result_list, UsDailyQuote);

/// `usa20590`의 typed 조회 요청.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usa20590Request {
    /// `stex_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchange>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,

    /// `base_dt` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dt: Option<BrokerDate>,
}

impl Usa20590Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        if let Some(s) = &self.stk_cd {
            nonempty(s)?;
        }
        Ok(())
    }
}

fields! {
    UsDailyPrice {
        dt,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        trde_prica,
        open_pric,
        high_pric,
        low_pric,
        base_pric,
        base_open_flu_rt,
        base_high_flu_rt,
        base_low_flu_rt,
    }
}

collection!(Usa20590Response, result_list, UsDailyPrice);

wire_enum! {
    DomesticIndustryMarket {
        Kospi => "0",
        Kosdaq => "1",
        Kospi200 => "2",
        Kospi100 => "4",
        Krx100 => "7",
    }
}

/// `ka10101`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10101Request {
    pub mrkt_tp: DomesticIndustryMarket,
}

fields! {
    DomesticIndustry {
        market_code => "marketCode",
        code,
        name,
        group,
    }
}

collection!(Ka10101Response, list, DomesticIndustry);
