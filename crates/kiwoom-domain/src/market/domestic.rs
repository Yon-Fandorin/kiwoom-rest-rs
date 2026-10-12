//! 국내주식 조건·투자자·프로그램·금 조회 계약.
//!
//! API별 요청 검증과 응답 행, 읽기 전용 계약 검증를 함께 관리한다.

use crate::{
    ClientError,
    transactions::{BrokerDate, validate_dates},
};

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use super::{bounded, choice, integer_limit, nonempty, required_choice};

macro_rules! required_request {
    ($name:ident {$($field:ident:$ty:ty),* $(,)?}) => {
        /// API별 필수 입력을 지정하는 국내주식 조회 요청.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(
                /// 브로커 요청에 전달할 필수 입력값.
                pub $field: $ty,
            )*
        }
    }
}

// 국내 필수 정수 입력은 선택 정수 검증기의 동일한 범위 규칙을 사용한다.
fn required_integer(v: &str, max: u32) -> Result<(), ClientError> {
    integer_limit(&Some(v.to_owned()), max)
}

// 공백은 허용하는 계약을 유지하고, 값이 있으면 길이와 숫자 형식을 검사한다.
fn blank_numeric(v: &str, max: Option<u32>, length: usize) -> Result<(), ClientError> {
    if v.len() > length {
        return Err(ClientError::InvalidRequest(
            "numeric filter exceeds documented format",
        ));
    }
    if v.trim().is_empty() {
        return Ok(());
    }
    if !v.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ClientError::InvalidRequest(
            "numeric filter exceeds documented format",
        ));
    }
    if let Some(max) = max {
        required_integer(v, max)?;
    }
    Ok(())
}

// 공개 요청 예제는 길이 6 표기와 달리 NX/AL 접미사를 명시한다.
// 여섯 자리 기본 코드와 문서에 나온 두 접미사만 허용한다.
fn domestic_stock_with_suffix(code: &str) -> Result<(), ClientError> {
    let base = code
        .strip_suffix("_NX")
        .or_else(|| code.strip_suffix("_AL"))
        .unwrap_or(code);
    if base.len() != 6 || !base.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ClientError::InvalidRequest(
            "expected six-digit stock code with optional _NX or _AL",
        ));
    }
    Ok(())
}

// 종목 정보·기관·회원사·신용 조회의 요청과 응답.
required_request! {
    Ka00198Request {
        qry_tp: String,
    }
}

impl Ka00198Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.qry_tp, &["1", "2", "3", "4", "5"])?;
        Ok(())
    }
}

fields! {
    DomesticInquiryRank {
        stk_nm,
        bigd_rank,
        rank_chg,
        rank_chg_sign,
        past_curr_prc,
        base_comp_sign,
        base_comp_chgr,
        prev_base_sign,
        prev_base_chgr,
        dt,
        tm,
        stk_cd,
    }
}

collection!(Ka00198Response, item_inq_rank, DomesticInquiryRank);

required_request! {
    Ka10013Request {
        stk_cd: String,
        dt: BrokerDate,
        qry_tp: String,
    }
}

impl Ka10013Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.qry_tp, &["1", "2"])?;
        Ok(())
    }
}

fields! {
    DomesticCreditTrend {
        dt,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        trde_qty,
        new,
        rpya,
        remn,
        amt,
        pre,
        shr_rt,
        remn_rt,
    }
}

collection!(Ka10013Response, crd_trde_trend, DomesticCreditTrend);

required_request! {
    Ka10015Request {
        stk_cd: String,
        strt_dt: BrokerDate,
    }
}

impl Ka10015Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        Ok(())
    }
}

fields! {
    DomesticDailyTrade {
        dt,
        close_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        trde_prica,
        bf_mkrt_trde_qty,
        bf_mkrt_trde_wght,
        opmr_trde_qty,
        opmr_trde_wght,
        af_mkrt_trde_qty,
        af_mkrt_trde_wght,
        tot_3,
        prid_trde_qty,
        cntr_str,
        for_poss,
        for_wght,
        for_netprps,
        orgn_netprps,
        ind_netprps,
        frgn,
        crd_remn_rt,
        prm,
        bf_mkrt_trde_prica,
        bf_mkrt_trde_prica_wght,
        opmr_trde_prica,
        opmr_trde_prica_wght,
        af_mkrt_trde_prica,
        af_mkrt_trde_prica_wght,
    }
}

collection!(Ka10015Response, daly_trde_dtl, DomesticDailyTrade);

required_request! {
    Ka10055Request {
        stk_cd: String,
        tdy_pred: String,
    }
}

impl Ka10055Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.tdy_pred, &["1", "2"])?;
        Ok(())
    }
}

fields! {
    DomesticTodayPreviousExecution {
        cntr_tm,
        cntr_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        cntr_qty,
        acc_trde_qty,
        acc_trde_prica,
    }
}

collection!(
    Ka10055Response,
    tdy_pred_cntr_qty,
    DomesticTodayPreviousExecution
);

required_request! {
    Ka90003Request {
        trde_upper_tp: String,
        amt_qty_tp: String,
        mrkt_tp: String,
        stex_tp: String,
    }
}

impl Ka90003Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.trde_upper_tp, &["1", "2"])?;
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        required_choice(&self.mrkt_tp, &["P00101", "P10102"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramRank {
        rank,
        stk_cd,
        stk_nm,
        cur_prc,
        flu_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        prm_sell_amt,
        prm_buy_amt,
        prm_netprps_amt,
    }
}

collection!(Ka90003Response, prm_netprps_upper_50, DomesticProgramRank);

required_request! {
    Ka90004Request {
        dt: BrokerDate,
        mrkt_tp: String,
        stex_tp: String,
    }
}

impl Ka90004Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["P00101", "P10102"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramStock {
        stk_cd,
        stk_nm,
        cur_prc,
        flu_sig,
        pred_pre,
        buy_cntr_qty,
        buy_cntr_amt,
        sel_cntr_qty,
        sel_cntr_amt,
        netprps_prica,
        all_trde_rt,
    }
}

/// `ka90004`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka90004Response {
    /// `tot_1` 원본 응답 값. 생략값을 보존한다.
    pub tot_1: Option<String>,

    /// `tot_2` 원본 응답 값. 생략값을 보존한다.
    pub tot_2: Option<String>,

    /// `tot_3` 원본 응답 값. 생략값을 보존한다.
    pub tot_3: Option<String>,

    /// `tot_4` 원본 응답 값. 생략값을 보존한다.
    pub tot_4: Option<String>,

    /// `tot_5` 원본 응답 값. 생략값을 보존한다.
    pub tot_5: Option<String>,

    /// `tot_6` 원본 응답 값. 생략값을 보존한다.
    pub tot_6: Option<String>,

    /// `stk_prm_trde_prst` 원본 응답 값. 생략값을 보존한다.
    pub stk_prm_trde_prst: Option<Vec<DomesticProgramStock>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// `kt20016`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt20016Request {
    pub mrkt_deal_tp: String,

    /// `crd_stk_grde_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crd_stk_grde_tp: Option<String>,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Kt20016Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_deal_tp, &["%", "1", "0"])?;
        choice(&self.crd_stk_grde_tp, &["%", "A", "B", "C", "D", "E"])?;
        if let Some(c) = &self.stk_cd {
            bounded(c, 12)?;
        }
        Ok(())
    }
}

fields! {
    DomesticCreditAvailable {
        stk_cd,
        stk_nm,
        crd_assr_rt,
        repl_pric,
        pred_close_pric,
        crd_limit_over_yn,
        crd_limit_over_txt,
    }
}

/// `kt20016`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Kt20016Response {
    /// `crd_loan_able` 원본 응답 값. 생략값을 보존한다.
    pub crd_loan_able: Option<String>,

    /// `crd_loan_pos_stk` 원본 응답 값. 생략값을 보존한다.
    pub crd_loan_pos_stk: Option<Vec<DomesticCreditAvailable>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

required_request! {
    Kt20017Request {
        stk_cd: String,
    }
}

impl Kt20017Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 12)?;
        Ok(())
    }
}

fields! {
    Kt20017Response {
        crd_alow_yn,
    }
}

required_request! {
    Ka10011Request {
        newstk_recvrht_tp: String,
    }
}

impl Ka10011Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.newstk_recvrht_tp, &["00", "05", "07"])?;
        Ok(())
    }
}

fields! {
    DomesticRightsQuote {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        fpr_sel_bid,
        fpr_buy_bid,
        acc_trde_qty,
        open_pric,
        high_pric,
        low_pric,
    }
}

collection!(Ka10011Response, newstk_recvrht_mrpr, DomesticRightsQuote);

required_request! {
    Ka10044Request {
        strt_dt: BrokerDate,
        end_dt: BrokerDate,
        trde_tp: String,
        mrkt_tp: String,
        stex_tp: String,
    }
}

impl Ka10044Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.trde_tp, &["1", "2"])?;
        required_choice(&self.mrkt_tp, &["001", "101"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticInstitutionalStock {
        stk_cd,
        stk_nm,
        netprps_qty,
        netprps_amt,
    }
}

collection!(
    Ka10044Response,
    daly_orgn_trde_stk,
    DomesticInstitutionalStock
);

required_request! {
    Ka10045Request {
        stk_cd: String,
        strt_dt: BrokerDate,
        end_dt: BrokerDate,
        orgn_prsm_unp_tp: String,
        for_prsm_unp_tp: String,
    }
}

impl Ka10045Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.orgn_prsm_unp_tp, &["1", "2"])?;
        required_choice(&self.for_prsm_unp_tp, &["1", "2"])?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticInstitutionalTrend {
        dt,
        close_pric,
        pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        orgn_dt_acc,
        orgn_daly_nettrde_qty,
        for_dt_acc,
        for_daly_nettrde_qty,
        limit_exh_rt,
    }
}

/// `ka10045`의 원본 응답 필드를 보존하는 타입.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ka10045Response {
    /// `orgn_prsm_avg_pric` 원본 응답 값. 생략값을 보존한다.
    pub orgn_prsm_avg_pric: Option<String>,

    /// `for_prsm_avg_pric` 원본 응답 값. 생략값을 보존한다.
    pub for_prsm_avg_pric: Option<String>,

    /// `stk_orgn_trde_trnsn` 원본 응답 값. 생략값을 보존한다.
    pub stk_orgn_trde_trnsn: Option<Vec<DomesticInstitutionalTrend>>,

    /// 타입에 선언되지 않은 브로커 응답 필드.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

required_request! {
    Ka10078Request {
        mmcm_cd: String,
        stk_cd: String,
        strt_dt: BrokerDate,
        end_dt: BrokerDate,
    }
}

impl Ka10078Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.mmcm_cd, 3)?;
        bounded(&self.stk_cd, 20)?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticMemberStockTrend {
        dt,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        netprps_qty,
        buy_qty,
        sell_qty,
    }
}

collection!(
    Ka10078Response,
    sec_stk_trde_trend,
    DomesticMemberStockTrend
);

required_request! {
    Ka10086Request {
        stk_cd: String,
        qry_dt: BrokerDate,
        indc_tp: String,
    }
}

impl Ka10086Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.indc_tp, &["0", "1"])?;
        Ok(())
    }
}

fields! {
    DomesticDailyStockPrice {
        date,
        open_pric,
        high_pric,
        low_pric,
        close_pric,
        pred_rt,
        flu_rt,
        trde_qty,
        amt_mn,
        crd_rt,
        ind,
        orgn,
        for_qty,
        frgn,
        prm,
        for_rt,
        for_poss,
        for_wght,
        for_netprps,
        orgn_netprps,
        ind_netprps,
        crd_remn_rt,
    }
}

collection!(Ka10086Response, daly_stkpc, DomesticDailyStockPrice);

// 투자자·금·프로그램 조회의 요청과 응답.
required_request! {
    Ka10063Request {
        mrkt_tp: String,
        amt_qty_tp: String,
        invsr: String,
        frgn_all: String,
        smtm_netprps_tp: String,
        stex_tp: String,
    }
}

impl Ka10063Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.amt_qty_tp, &["1"])?;
        required_choice(&self.invsr, &["0", "1", "2", "3", "4", "5", "6", "7"])?;
        required_choice(&self.frgn_all, &["0", "1"])?;
        required_choice(&self.smtm_netprps_tp, &["0", "1"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticIntradayInvestor {
        stk_cd,
        stk_nm,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        netprps_amt,
        prev_netprps_amt,
        buy_amt,
        netprps_amt_irds,
        buy_amt_irds,
        sell_amt,
        sell_amt_irds,
        netprps_qty,
        prev_pot_netprps_qty,
        netprps_irds,
        buy_qty,
        buy_qty_irds,
        sell_qty,
        sell_qty_irds,
    }
}

collection!(Ka10063Response, opmr_invsr_trde, DomesticIntradayInvestor);

required_request! {
    Ka10066Request {
        mrkt_tp: String,
        amt_qty_tp: String,
        trde_tp: String,
        stex_tp: String,
    }
}

impl Ka10066Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        required_choice(&self.trde_tp, &["0", "1", "2"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticInvestorAfterClose {
        stk_cd,
        stk_nm,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        ind_invsr,
        frgnr_invsr,
        orgn,
        fnnc_invt,
        insrnc,
        invtrt,
        etc_fnnc,
        bank,
        penfnd_etc,
        samo_fund,
        natn,
        etc_corp,
    }
}

collection!(Ka10066Response, opaf_invsr_trde, DomesticInvestorAfterClose);

required_request! {
    Ka50010Request {
        stk_cd: String,
    }
}

impl Ka50010Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        Ok(())
    }
}

fields! {
    DomesticGoldExecution {
        cntr_pric,
        pred_pre,
        flu_rt,
        trde_qty,
        acc_trde_prica,
        cntr_trde_qty,
        tm,
        pre_sig,
        pri_sel_bid_unit,
        pri_buy_bid_unit,
        trde_pre,
        trde_tern_rt,
        cntr_str,
    }
}

collection!(Ka50010Response, gold_cntr, DomesticGoldExecution);

required_request! {
    Ka50012Request {
        stk_cd: String,
        base_dt: BrokerDate,
    }
}

impl Ka50012Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        Ok(())
    }
}

fields! {
    DomesticGoldDaily {
        cur_prc,
        pred_pre,
        flu_rt,
        trde_qty,
        acc_trde_prica,
        open_pric,
        high_pric,
        low_pric,
        dt,
        pre_sig,
        orgn_netprps,
        for_netprps,
        ind_netprps,
    }
}

collection!(Ka50012Response, gold_daly_trnsn, DomesticGoldDaily);

required_request! {
    Ka50087Request {
        stk_cd: String,
    }
}

impl Ka50087Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        Ok(())
    }
}

fields! {
    DomesticGoldExpected {
        exp_cntr_pric,
        exp_pred_pre,
        exp_flu_rt,
        exp_acc_trde_qty,
        exp_cntr_trde_qty,
        exp_tm,
        exp_pre_sig,
        stex_tp,
    }
}

collection!(Ka50087Response, gold_expt_exec, DomesticGoldExpected);

required_request! {
    Ka50100Request {
        stk_cd: String,
    }
}

impl Ka50100Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        Ok(())
    }
}

fields! {
    Ka50100Response {
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        open_pric,
        high_pric,
        low_pric,
        pred_rt,
        upl_pric,
        lst_pric,
        pred_close_pric,
    }
}

required_request! {
    Ka50101Request {
        stk_cd: String,
        tic_scope: String,
    }
}

impl Ka50101Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.tic_scope, &["1", "3", "5", "10", "30"])?;
        Ok(())
    }
}

fields! {
    DomesticGoldBook {
        cntr_pric,
        pred_pre,
        flu_rt,
        trde_qty,
        acc_trde_prica,
        cntr_trde_qty,
        tm,
        pre_sig,
        pri_sel_bid_unit,
        pri_buy_bid_unit,
        trde_pre,
        trde_tern_rt,
        cntr_str,
        lpmmcm_nm_1,
        stex_tp,
    }
}

collection!(Ka50101Response, gold_bid, DomesticGoldBook);

required_request! {
    Ka90005Request {
        date: BrokerDate,
        amt_qty_tp: String,
        mrkt_tp: String,
        min_tic_tp: String,
        stex_tp: String,
    }
}

impl Ka90005Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        match self.stex_tp.as_str() {
            "1" => required_choice(&self.mrkt_tp, &["P00101", "P10102"])?,
            "2" => required_choice(&self.mrkt_tp, &["P001_NX01", "P101_NX02"])?,
            "3" => required_choice(&self.mrkt_tp, &["P001_AL01", "P101_AL02"])?,
            _ => return Err(ClientError::InvalidRequest("undocumented exchange")),
        }
        required_choice(&self.min_tic_tp, &["0", "1"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramTrend {
        cntr_tm,
        dfrt_trde_sel,
        dfrt_trde_buy,
        dfrt_trde_netprps,
        ndiffpro_trde_sel,
        ndiffpro_trde_buy,
        ndiffpro_trde_netprps,
        dfrt_trde_sell_qty,
        dfrt_trde_buy_qty,
        dfrt_trde_netprps_qty,
        ndiffpro_trde_sell_qty,
        ndiffpro_trde_buy_qty,
        ndiffpro_trde_netprps_qty,
        all_sel,
        all_buy,
        all_netprps,
        kospi200,
        basis,
    }
}

collection!(Ka90005Response, prm_trde_trnsn, DomesticProgramTrend);

required_request! {
    Ka90010Request {
        date: BrokerDate,
        amt_qty_tp: String,
        mrkt_tp: String,
        min_tic_tp: String,
        stex_tp: String,
    }
}

impl Ka90010Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        match self.stex_tp.as_str() {
            "1" => required_choice(&self.mrkt_tp, &["P00101", "P10102"])?,
            "2" => required_choice(&self.mrkt_tp, &["P001_NX01", "P101_NX02"])?,
            "3" => required_choice(&self.mrkt_tp, &["P001_AL01", "P001_AL02"])?,
            _ => return Err(ClientError::InvalidRequest("undocumented exchange")),
        }
        required_choice(&self.min_tic_tp, &["0", "1"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

collection!(Ka90010Response, prm_trde_trnsn, DomesticProgramTrend);

required_request! {
    Ka90006Request {
        date: BrokerDate,
        stex_tp: String,
    }
}

impl Ka90006Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramArbitrageBalance {
        dt,
        buy_dfrt_trde_qty,
        buy_dfrt_trde_amt,
        buy_dfrt_trde_irds_amt,
        sel_dfrt_trde_qty,
        sel_dfrt_trde_amt,
        sel_dfrt_trde_irds_amt,
    }
}

collection!(
    Ka90006Response,
    prm_trde_dfrt_remn_trnsn,
    DomesticProgramArbitrageBalance
);

required_request! {
    Ka90007Request {
        date: BrokerDate,
        amt_qty_tp: String,
        mrkt_tp: String,
        stex_tp: String,
    }
}

impl Ka90007Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        required_choice(&self.mrkt_tp, &["0", "1"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramCumulative {
        dt,
        kospi200,
        basis,
        dfrt_trde_tdy,
        dfrt_trde_acc,
        ndiffpro_trde_tdy,
        ndiffpro_trde_acc,
        all_tdy,
        all_acc,
    }
}

collection!(
    Ka90007Response,
    prm_trde_acc_trnsn,
    DomesticProgramCumulative
);

/// `ka90013`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka90013Request {
    pub stk_cd: String,

    /// `amt_qty_tp` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt_qty_tp: Option<String>,

    /// `date` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<BrokerDate>,
}

impl Ka90013Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        choice(&self.amt_qty_tp, &["1", "2"])?;
        Ok(())
    }
}

fields! {
    DomesticProgramDailyStock {
        dt,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        prm_sell_amt,
        prm_buy_amt,
        prm_netprps_amt,
        prm_netprps_amt_irds,
        prm_sell_qty,
        prm_buy_qty,
        prm_netprps_qty,
        prm_netprps_qty_irds,
        base_pric_tm,
        dbrt_trde_rpy_sum,
        remn_rcvord_sum,
        stex_tp,
    }
}

collection!(
    Ka90013Response,
    stk_daly_prm_trde_trnsn,
    DomesticProgramDailyStock
);

/// `ka10016`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10016Request {
    pub mrkt_tp: String,

    pub ntl_tp: String,

    pub high_low_close_tp: String,

    pub stk_cnd: String,

    pub trde_qty_tp: String,

    pub crd_cnd: String,

    pub updown_incls: String,

    pub dt: String,

    pub stex_tp: String,
}

impl Ka10016Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.ntl_tp, &["1", "2"])?;
        required_choice(&self.high_low_close_tp, &["1", "2"])?;
        required_choice(&self.stk_cnd, &["0", "1", "3", "5", "6", "7", "8"])?;
        required_choice(
            &self.trde_qty_tp,
            &[
                "00000", "00010", "00050", "00100", "00150", "00200", "00300", "00500", "01000",
            ],
        )?;
        required_choice(&self.crd_cnd, &["0", "1", "2", "3", "4", "7", "9"])?;
        required_choice(&self.updown_incls, &["0", "1"])?;
        if self.dt.len() > 3 {
            return Err(ClientError::InvalidRequest(
                "day count exceeds documented length",
            ));
        }
        required_integer(&self.dt, 250)?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticNewHighLow {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        pred_trde_qty_pre_rt,
        sel_bid,
        buy_bid,
        high_pric,
        low_pric,
    }
}

collection!(Ka10016Response, ntl_pric, DomesticNewHighLow);

/// `ka10017`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10017Request {
    pub mrkt_tp: String,

    pub updown_tp: String,

    pub sort_tp: String,

    pub stk_cnd: String,

    pub trde_qty_tp: String,

    pub crd_cnd: String,

    pub trde_gold_tp: String,

    pub stex_tp: String,
}

impl Ka10017Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.updown_tp, &["1", "2", "3", "4", "5", "6", "7"])?;
        required_choice(&self.sort_tp, &["1", "2", "3"])?;
        required_choice(
            &self.stk_cnd,
            &["0", "1", "3", "4", "5", "6", "7", "8", "9", "10"],
        )?;
        required_choice(
            &self.trde_qty_tp,
            &[
                "00000", "00010", "00050", "00100", "00150", "00200", "00300", "00500", "01000",
            ],
        )?;
        required_choice(&self.crd_cnd, &["0", "1", "2", "3", "4", "7", "9"])?;
        required_choice(&self.trde_gold_tp, &["0", "1", "2", "3", "4", "5", "8"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticLimitPrice {
        stk_cd,
        stk_infr,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        pred_trde_qty,
        sel_req,
        sel_bid,
        buy_bid,
        buy_req,
        cnt,
    }
}

collection!(Ka10017Response, updown_pric, DomesticLimitPrice);

/// `ka10018`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10018Request {
    pub high_low_tp: String,

    pub alacc_rt: String,

    pub mrkt_tp: String,

    pub trde_qty_tp: String,

    pub stk_cnd: String,

    pub crd_cnd: String,

    pub stex_tp: String,
}

impl Ka10018Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.high_low_tp, &["1", "2"])?;
        required_choice(&self.alacc_rt, &["05", "10", "15", "20", "25", "30"])?;
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(
            &self.trde_qty_tp,
            &[
                "00000", "00010", "00050", "00100", "00150", "00200", "00300", "00500", "01000",
            ],
        )?;
        required_choice(&self.stk_cnd, &["0", "1", "3", "5", "6", "7", "8"])?;
        required_choice(&self.crd_cnd, &["0", "1", "2", "3", "4", "7", "9"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticNearHighLow {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        sel_bid,
        buy_bid,
        tdy_high_pric,
        tdy_low_pric,
    }
}

collection!(Ka10018Response, high_low_pric_alacc, DomesticNearHighLow);

/// `ka10019`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10019Request {
    pub mrkt_tp: String,

    pub flu_tp: String,

    pub tm_tp: String,

    pub tm: String,

    pub trde_qty_tp: String,

    pub stk_cnd: String,

    pub crd_cnd: String,

    pub pric_cnd: String,

    pub updown_incls: String,

    pub stex_tp: String,
}

impl Ka10019Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101", "201"])?;
        required_choice(&self.flu_tp, &["1", "2"])?;
        required_choice(&self.tm_tp, &["1", "2"])?;
        required_integer(&self.tm, 99)?;
        if self.tm.len() > 2 {
            return Err(ClientError::InvalidRequest(
                "time exceeds documented length2",
            ));
        }
        required_choice(
            &self.trde_qty_tp,
            &[
                "00000", "00010", "00050", "00100", "00150", "00200", "00300", "00500", "01000",
            ],
        )?;
        required_choice(&self.stk_cnd, &["0", "1", "3", "5", "6", "7", "8"])?;
        required_choice(&self.crd_cnd, &["0", "1", "2", "3", "4", "7", "9"])?;
        required_choice(&self.pric_cnd, &["0", "1", "2", "3", "4", "5", "8"])?;
        required_choice(&self.updown_incls, &["0", "1"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticPriceSurge {
        stk_cd,
        stk_cls,
        stk_nm,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        base_pric,
        cur_prc,
        base_pre,
        trde_qty,
        jmp_rt,
    }
}

collection!(Ka10019Response, pric_jmpflu, DomesticPriceSurge);

/// `ka10024`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10024Request {
    pub mrkt_tp: String,

    pub cycle_tp: String,

    pub trde_qty_tp: String,

    pub stex_tp: String,
}

impl Ka10024Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.cycle_tp, &["5", "10", "20", "60", "250"])?;
        required_choice(
            &self.trde_qty_tp,
            &["5", "10", "50", "100", "200", "300", "500", "1000"],
        )?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticVolumeUpdate {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        prev_trde_qty,
        now_trde_qty,
        sel_bid,
        buy_bid,
    }
}

collection!(Ka10024Response, trde_qty_updt, DomesticVolumeUpdate);

/// `ka10025`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10025Request {
    pub mrkt_tp: String,

    pub prps_cnctr_rt: String,

    pub cur_prc_entry: String,

    pub prpscnt: String,

    pub cycle_tp: String,

    pub stex_tp: String,
}

impl Ka10025Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_integer(&self.prps_cnctr_rt, 100)?;
        if self.prps_cnctr_rt.len() > 3 {
            return Err(ClientError::InvalidRequest("concentration exceeds length3"));
        }
        required_choice(&self.cur_prc_entry, &["0", "1"])?;
        required_integer(&self.prpscnt, 99)?;
        if self.prpscnt.len() > 2 {
            return Err(ClientError::InvalidRequest("band count exceeds length2"));
        }
        required_choice(&self.cycle_tp, &["50", "100", "150", "200", "250", "300"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticConcentratedSupply {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        now_trde_qty,
        pric_strt,
        pric_end,
        prps_qty,
        prps_rt,
    }
}

collection!(Ka10025Response, prps_cnctr, DomesticConcentratedSupply);

/// `ka10026`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10026Request {
    pub pertp: String,

    pub stex_tp: String,
}

impl Ka10026Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.pertp, &["1", "2", "3", "4", "5", "6"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticValuationRank {
        stk_cd,
        stk_nm,
        per,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        now_trde_qty,
        sel_bid,
    }
}

collection!(Ka10026Response, high_low_per, DomesticValuationRank);

/// `ka10028`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10028Request {
    pub sort_tp: String,

    pub trde_qty_cnd: String,

    pub mrkt_tp: String,

    pub updown_incls: String,

    pub stk_cnd: String,

    pub crd_cnd: String,

    pub trde_prica_cnd: String,

    pub flu_cnd: String,

    pub stex_tp: String,
}

impl Ka10028Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.sort_tp, &["1", "2", "3", "4"])?;
        required_choice(
            &self.trde_qty_cnd,
            &["0000", "0010", "0050", "0100", "0500", "1000"],
        )?;
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.updown_incls, &["0", "1"])?;
        required_choice(
            &self.stk_cnd,
            &["0", "1", "3", "4", "5", "6", "7", "8", "9"],
        )?;
        required_choice(&self.crd_cnd, &["0", "1", "2", "3", "4", "7", "9"])?;
        required_choice(
            &self.trde_prica_cnd,
            &[
                "0", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000",
            ],
        )?;
        required_choice(&self.flu_cnd, &["1", "2"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        Ok(())
    }
}

fields! {
    DomesticRelativeOpen {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        open_pric,
        high_pric,
        low_pric,
        open_pric_pre,
        now_trde_qty,
        cntr_str,
    }
}

collection!(Ka10028Response, open_pric_pre_flu_rt, DomesticRelativeOpen);

/// `ka10043`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10043Request {
    pub stk_cd: String,

    pub strt_dt: BrokerDate,

    pub end_dt: BrokerDate,

    pub qry_dt_tp: String,

    pub pot_tp: String,

    pub dt: String,

    pub sort_base: String,

    pub mmcm_cd: String,

    pub stex_tp: String,
}

impl Ka10043Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.qry_dt_tp, &["0", "1"])?;
        required_choice(&self.pot_tp, &["0", "1"])?;
        required_choice(&self.dt, &["5", "10", "20", "40", "60", "120"])?;
        required_choice(&self.sort_base, &["1", "2"])?;
        bounded(&self.mmcm_cd, 3)?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticMemberSupply {
        dt,
        close_pric,
        pre_sig,
        pred_pre,
        sel_qty,
        buy_qty,
        netprps_qty,
        trde_qty_sum,
        trde_wght,
    }
}

collection!(Ka10043Response, trde_ori_prps_anly, DomesticMemberSupply);

/// `ka10052`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10052Request {
    pub mmcm_cd: String,

    pub mrkt_tp: String,

    pub qty_tp: String,

    pub pric_tp: String,

    pub stex_tp: String,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Ka10052Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.mmcm_cd, 3)?;
        required_choice(&self.mrkt_tp, &["0", "1", "2", "3"])?;
        required_choice(
            &self.qty_tp,
            &["0", "1", "2", "3", "5", "10", "30", "50", "100"],
        )?;
        required_choice(&self.pric_tp, &["0", "1", "2", "3", "4", "5", "8"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        if let Some(s) = &self.stk_cd {
            bounded(s, 20)?;
        }
        Ok(())
    }
}

fields! {
    DomesticMemberMoment {
        tm,
        stk_cd,
        stk_nm,
        trde_ori_nm,
        tp,
        mont_trde_qty,
        acc_netprps,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
    }
}

collection!(
    Ka10052Response,
    trde_ori_mont_trde_qty,
    DomesticMemberMoment
);

/// `ka10054`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10054Request {
    pub mrkt_tp: String,

    pub bf_mkrt_tp: String,

    pub motn_tp: String,

    pub skip_stk: String,

    pub trde_qty_tp: String,

    pub min_trde_qty: String,

    pub max_trde_qty: String,

    pub trde_prica_tp: String,

    pub min_trde_prica: String,

    pub max_trde_prica: String,

    pub motn_drc: String,

    pub stex_tp: String,

    /// `stk_cd` 선택 입력. 생략하면 요청에서 제외한다.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
}

impl Ka10054Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.mrkt_tp, &["000", "001", "101"])?;
        required_choice(&self.bf_mkrt_tp, &["0"])?;
        required_choice(&self.motn_tp, &["0", "1", "2", "3"])?;
        if self.skip_stk.len() != 9 || !self.skip_stk.bytes().all(|b| b == b'0' || b == b'1') {
            return Err(ClientError::InvalidRequest(
                "skip_stk must be nine binary flags",
            ));
        }
        required_choice(&self.trde_qty_tp, &["0", "1"])?;
        blank_numeric(&self.min_trde_qty, None, 12)?;
        blank_numeric(&self.max_trde_qty, Some(100000000), 12)?;
        required_choice(&self.trde_prica_tp, &["0", "1"])?;
        blank_numeric(&self.min_trde_prica, None, 10)?;
        blank_numeric(&self.max_trde_prica, Some(100000000), 10)?;
        required_choice(&self.motn_drc, &["0", "1", "2"])?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        if let Some(s) = &self.stk_cd {
            if s.len() > 20 {
                return Err(ClientError::InvalidRequest(
                    "stock code exceeds documented length",
                ));
            }
            if !s.trim().is_empty() {
                bounded(s, 20)?;
            }
        }
        Ok(())
    }
}

fields! {
    DomesticViActivation {
        stk_cd,
        stk_nm,
        acc_trde_qty,
        motn_pric,
        dynm_dispty_rt,
        trde_cntr_proc_time,
        virelis_time,
        viaplc_tp,
        dynm_stdpc,
        static_stdpc,
        static_dispty_rt,
        open_pric_pre_flu_rt,
        vimotn_cnt,
        stex_tp,
    }
}

collection!(Ka10054Response, motn_stk, DomesticViActivation);

/// `ka10058`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10058Request {
    pub strt_dt: BrokerDate,

    pub end_dt: BrokerDate,

    pub trde_tp: String,

    pub mrkt_tp: String,

    pub invsr_tp: String,

    pub stex_tp: String,
}

impl Ka10058Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.trde_tp, &["1", "2"])?;
        required_choice(&self.mrkt_tp, &["001", "101"])?;
        required_choice(
            &self.invsr_tp,
            &[
                "8000", "9000", "1000", "3000", "3100", "5000", "4000", "2000", "6000", "7000",
                "7100", "9999",
            ],
        )?;
        required_choice(&self.stex_tp, &["1", "2", "3"])?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticInvestorDailyStock {
        stk_cd,
        stk_nm,
        netslmt_qty,
        netslmt_amt,
        prsm_avg_pric,
        cur_prc,
        pre_sig,
        pred_pre,
        avg_pric_pre,
        pre_rt,
        dt_trde_qty,
    }
}

collection!(
    Ka10058Response,
    invsr_daly_trde_stk,
    DomesticInvestorDailyStock
);

/// `ka10059`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10059Request {
    pub dt: BrokerDate,

    pub stk_cd: String,

    pub amt_qty_tp: String,

    pub trde_tp: String,

    pub unit_tp: String,
}

impl Ka10059Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        required_choice(&self.trde_tp, &["0", "1", "2"])?;
        required_choice(&self.unit_tp, &["1000", "1"])?;
        Ok(())
    }
}

fields! {
    DomesticInvestorStock {
        dt,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        acc_trde_prica,
        ind_invsr,
        frgnr_invsr,
        orgn,
        fnnc_invt,
        insrnc,
        invtrt,
        etc_fnnc,
        bank,
        penfnd_etc,
        samo_fund,
        natn,
        etc_corp,
        natfor,
    }
}

collection!(Ka10059Response, stk_invsr_orgn, DomesticInvestorStock);

/// `ka10061`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10061Request {
    pub stk_cd: String,

    pub strt_dt: BrokerDate,

    pub end_dt: BrokerDate,

    pub amt_qty_tp: String,

    pub trde_tp: String,

    pub unit_tp: String,
}

impl Ka10061Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        required_choice(&self.trde_tp, &["0"])?;
        required_choice(&self.unit_tp, &["1000", "1"])?;
        validate_dates(&self.strt_dt, &self.end_dt)?;
        Ok(())
    }
}

fields! {
    DomesticInvestorTotals {
        ind_invsr,
        frgnr_invsr,
        orgn,
        fnnc_invt,
        insrnc,
        invtrt,
        etc_fnnc,
        bank,
        penfnd_etc,
        samo_fund,
        natn,
        etc_corp,
        natfor,
    }
}

collection!(Ka10061Response, stk_invsr_orgn_tot, DomesticInvestorTotals);

/// `ka10095`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10095Request {
    pub stk_cd: String,
}

impl Ka10095Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        bounded(&self.stk_cd, 20)?;
        for code in self.stk_cd.split("|") {
            nonempty(code)?;
        }
        Ok(())
    }
}

fields! {
    DomesticSpecifiedStock {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        base_pric,
        trde_qty,
        trde_prica,
        cntr_qty,
        cntr_str,
        pred_trde_qty_pre,
        sel_bid,
        buy_bid,
        sel_1th_bid,
        sel_2th_bid,
        sel_3th_bid,
        sel_4th_bid,
        sel_5th_bid,
        buy_1th_bid,
        buy_2th_bid,
        buy_3th_bid,
        buy_4th_bid,
        buy_5th_bid,
        upl_pric,
        lst_pric,
        open_pric,
        high_pric,
        low_pric,
        close_pric,
        cntr_tm,
        exp_cntr_pric,
        exp_cntr_qty,
        cap,
        fav,
        mac,
        stkcnt,
        bid_tm,
        dt,
        pri_sel_req,
        pri_buy_req,
        pri_sel_cnt,
        pri_buy_cnt,
        tot_sel_req,
        tot_buy_req,
        tot_sel_cnt,
        tot_buy_cnt,
        prty,
        gear,
        pl_qutr,
        cap_support,
        elwexec_pric,
        cnvt_rt,
        elwexpr_dt,
        cntr_engg,
        cntr_pred_pre,
        theory_pric,
        innr_vltl,
        delta,
        gam,
        theta,
        vega,
        law,
    }
}

collection!(Ka10095Response, atn_stk_infr, DomesticSpecifiedStock);

// 시세표와 접미사가 있는 종목별 체결강도·프로그램 조회.
stock_request!(Ka10007Request, 20);

fields! {
    Ka10007Response {
        stk_nm,
        stk_cd,
        date,
        tm,
        pred_close_pric,
        pred_trde_qty,
        upl_pric,
        lst_pric,
        pred_trde_prica,
        flo_stkcnt,
        cur_prc,
        smbol,
        flu_rt,
        pred_rt,
        open_pric,
        high_pric,
        low_pric,
        cntr_qty,
        trde_qty,
        trde_prica,
        exp_cntr_pric,
        exp_cntr_qty,
        exp_sel_pri_bid,
        exp_buy_pri_bid,
        trde_strt_dt,
        exec_pric,
        hgst_pric,
        lwst_pric,
        hgst_pric_dt,
        lwst_pric_dt,
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
        sel_1bid_cnt,
        sel_2bid_cnt,
        sel_3bid_cnt,
        sel_4bid_cnt,
        sel_5bid_cnt,
        buy_1bid_cnt,
        buy_2bid_cnt,
        buy_3bid_cnt,
        buy_4bid_cnt,
        buy_5bid_cnt,
        lpsel_1bid_req,
        lpsel_2bid_req,
        lpsel_3bid_req,
        lpsel_4bid_req,
        lpsel_5bid_req,
        lpsel_6bid_req,
        lpsel_7bid_req,
        lpsel_8bid_req,
        lpsel_9bid_req,
        lpsel_10bid_req,
        lpbuy_1bid_req,
        lpbuy_2bid_req,
        lpbuy_3bid_req,
        lpbuy_4bid_req,
        lpbuy_5bid_req,
        lpbuy_6bid_req,
        lpbuy_7bid_req,
        lpbuy_8bid_req,
        lpbuy_9bid_req,
        lpbuy_10bid_req,
        tot_buy_req,
        tot_sel_req,
        tot_buy_cnt,
        tot_sel_cnt,
    }
}

/// `ka10046`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10046Request {
    pub stk_cd: String,
}

impl Ka10046Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        domestic_stock_with_suffix(&self.stk_cd)
    }
}

fields! {
    DomesticExecutionStrengthTime {
        cntr_tm,
        cur_prc,
        pred_pre,
        pred_pre_sig,
        flu_rt,
        trde_qty,
        acc_trde_prica,
        acc_trde_qty,
        cntr_str,
        cntr_str_5min,
        cntr_str_20min,
        cntr_str_60min,
        stex_tp,
    }
}

collection!(Ka10046Response, cntr_str_tm, DomesticExecutionStrengthTime);

/// `ka10047`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka10047Request {
    pub stk_cd: String,
}

impl Ka10047Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        domestic_stock_with_suffix(&self.stk_cd)
    }
}

fields! {
    DomesticExecutionStrengthDate {
        dt,
        cur_prc,
        pred_pre,
        pred_pre_sig,
        flu_rt,
        trde_qty,
        acc_trde_prica,
        acc_trde_qty,
        cntr_str,
        cntr_str_5min,
        cntr_str_20min,
        cntr_str_60min,
    }
}

collection!(
    Ka10047Response,
    cntr_str_daly,
    DomesticExecutionStrengthDate
);

/// `ka90008`의 typed 조회 요청.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ka90008Request {
    pub amt_qty_tp: String,

    pub stk_cd: String,

    pub date: BrokerDate,
}

impl Ka90008Request {
    /// 전송 전에 이 요청의 문서 제약을 검사한다.
    pub fn validate(&self) -> Result<(), ClientError> {
        required_choice(&self.amt_qty_tp, &["1", "2"])?;
        domestic_stock_with_suffix(&self.stk_cd)
    }
}

fields! {
    DomesticProgramTimeStock {
        tm,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        prm_sell_amt,
        prm_buy_amt,
        prm_netprps_amt,
        prm_netprps_amt_irds,
        prm_sell_qty,
        prm_buy_qty,
        prm_netprps_qty,
        prm_netprps_qty_irds,
        base_pric_tm,
        dbrt_trde_rpy_sum,
        remn_rcvord_sum,
        stex_tp,
    }
}

collection!(
    Ka90008Response,
    stk_tm_prm_trde_trnsn,
    DomesticProgramTimeStock
);
