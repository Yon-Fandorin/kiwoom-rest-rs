//! 계좌 거래 이력의 선택값·요청·응답과 타입별 계약 검증.
//! 문자열의 부호·패딩·빈 값과 알 수 없는 행 필드를 보존합니다.
//! 필드의 존재만으로 총액·순액·세금 또는 과거 환율의 의미를 확정하지 않습니다.
//! 선택 값의 누락·빈 문자열은 수치 0으로 바꾸지 않습니다.

use crate::transactions::wire_enum;
use crate::{
    ClientError,
    transactions::{
        BrokerDate, DomesticExchange, ProductType, SortOrder, UsExchange, YesNo, validate_dates,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

macro_rules! row {
    ($name:ident {$($field:ident),+ $(,)?}) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct $name {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,
            )+
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>
        }
    }
}

wire_enum!(DomesticTransactionType {
    All => "0",
    DepositsWithdrawals => "1",
    SecuritiesTransfers => "2",
    Trades => "3",
    Buys => "4",
    Sells => "5",
    Deposits => "6",
    Withdrawals => "7",
    CollateralLoanDeposit => "A",
    SellCollateralLoanDeposit => "B",
    CashRepayment => "C",
    Fx => "F",
    DepositsWithdrawalsAndFx => "M",
    ForeignCurrencyBuy => "G",
    ForeignCurrencySell => "H",
    FxSettlementDeposit => "I",
    FxSettlementWithdrawal => "J"
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kt00015Request {
    pub strt_dt: BrokerDate,
    pub end_dt: BrokerDate,
    pub tp: DomesticTransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crnc_cd: Option<String>,
    pub gds_tp: ProductType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frgn_stex_code: Option<String>,
    pub dmst_stex_tp: DomesticExchange,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qry_sort_tp: Option<SortOrder>,
}

impl Kt00015Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        validate_dates(&self.strt_dt, &self.end_dt)?;
        if self.stk_cd.as_ref().is_some_and(|s| s.len() > 12)
            || self.crnc_cd.as_ref().is_some_and(|s| s.len() != 3)
            || self.frgn_stex_code.as_ref().is_some_and(|s| s.len() > 10)
        {
            return Err(ClientError::InvalidRequest(
                "field exceeds documented length",
            ));
        }
        Ok(())
    }
}

row!(DomesticTransaction {
    trde_dt,
    trde_no,
    rmrk_nm,
    crd_deal_tp_nm,
    exct_amt,
    loan_amt_rpya,
    fc_trde_amt,
    fc_exct_amt,
    entra_remn,
    crnc_cd,
    trde_ocr_tp,
    trde_kind_nm,
    stk_nm,
    trde_amt,
    trde_agri_tax,
    rpy_diffa,
    fc_trde_tax,
    dly_sum,
    fc_entra,
    mdia_tp_nm,
    io_tp,
    io_tp_nm,
    orig_deal_no,
    stk_cd,
    trde_qty_jwa_cnt,
    cmsn,
    int_ls_usfe,
    fc_cmsn,
    fc_dly_sum,
    vlbl_nowrm,
    proc_tm,
    isin_cd,
    stex_cd,
    stex_nm,
    trde_unit,
    incm_resi_tax,
    loan_dt,
    uncl_ocr,
    rpym_sum,
    cntr_dt,
    rcpy_no,
    prcsr,
    proc_brch,
    trde_stle,
    txon_base_pric,
    tax_sum_cmsn,
    frgn_pay_txam,
    fc_uncl_ocr,
    rpym_sum_fr,
    rcpmnyer,
    trde_prtc_tp
});

#[derive(Debug, Clone, Deserialize)]
pub struct Kt00015Response {
    pub trst_ovrl_trde_prps_array: Option<Vec<DomesticTransaction>>,
}

wire_enum!(UsTransactionType {
    All => "0",
    Trades => "3",
    Buys => "4",
    Sells => "5"
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ust21100Request {
    pub strt_dt: BrokerDate,
    pub end_dt: BrokerDate,
    pub tp: UsTransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stex_tp: Option<UsExchange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stk_cd: Option<String>,
    pub krw_repl_skip_yn: YesNo,
}

impl Ust21100Request {
    pub fn validate(&self) -> Result<(), ClientError> {
        validate_dates(&self.strt_dt, &self.end_dt)?;
        if self.stk_cd.as_ref().is_some_and(|s| s.len() > 12) {
            return Err(ClientError::InvalidRequest(
                "ticker exceeds documented length",
            ));
        }
        Ok(())
    }
}

row!(UsTransaction {
    deal_dt,
    deal_kind_nm,
    rmrk_nm,
    deal_amt,
    tax_tot_amt,
    exct_amt,
    uncl_ocr,
    fc_uncl_ocr,
    entra_remn,
    deal_no,
    stk_nm,
    deal_qty,
    fc_deal_tax,
    frgn_pay_txam,
    rpym_sum,
    fc_rpym_sum,
    fc_entra,
    mdia_nm,
    orig_deal_no,
    stk_cd,
    uv_exrt,
    fc_cmsn,
    fc_exct_amt,
    dly_sum,
    fc_dly_sum,
    vlbl_nowrm,
    stex_nm,
    fc_deal_amt,
    proc_time,
    crnc_code
});

#[derive(Debug, Clone, Deserialize)]
pub struct Ust21100Response {
    pub acnt_print: Option<String>,
    pub sell_sum: Option<String>,
    pub buy_sum: Option<String>,
    pub result_list: Option<Vec<UsTransaction>>,
}
