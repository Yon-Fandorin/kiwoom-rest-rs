//! 현재 공개 웹 가이드를 바탕으로 독립적으로 작성한 차트·업종 계약입니다.
//! 가격·수량·수정주가·환율 적용 값은 증권사 문자열 그대로 보존합니다.
//! 과거 원장 환율이나 배당 금액으로 해석해서는 안 됩니다.

use crate::ClientError;

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

type UnknownFields = BTreeMap<String, serde_json::Value>;

fn check_field(
    value: Option<&str>,
    required: bool,
    max: usize,
    allowed: &[&str],
    date: bool,
) -> Result<(), ClientError> {
    let Some(value) = value else {
        return if required {
            Err(ClientError::InvalidRequest("missing required chart field"))
        } else {
            Ok(())
        };
    };
    if required && value.trim().is_empty() {
        return Err(ClientError::InvalidRequest("empty required chart field"));
    }
    if max > 0 && value.chars().count() > max {
        return Err(ClientError::InvalidRequest(
            "chart field exceeds documented length",
        ));
    }
    if !allowed.is_empty() && !allowed.contains(&value) {
        return Err(ClientError::InvalidRequest(
            "unsupported documented chart code",
        ));
    }
    if date
        && (value.len() != 8
            || !value.bytes().all(|c| c.is_ascii_digit())
            || chrono::NaiveDate::parse_from_str(value, "%Y%m%d").is_err())
    {
        return Err(ClientError::InvalidRequest("chart date must be YYYYMMDD"));
    }
    Ok(())
}

macro_rules! request {
    (
        $name:ident,
        {$($required:ident => ($rmax:expr, $rcodes:expr, $rdate:expr)),* $(,)?},
        {$($optional:ident => ($omax:expr, $ocodes:expr, $odate:expr)),* $(,)?}
    ) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $required: String,)*
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $optional: Option<String>,
            )*
        }

        impl $name {
            pub fn validate(&self) -> Result<(), ClientError> {
                $(check_field(Some(&self.$required), true, $rmax, $rcodes, $rdate)?;)*
                $(check_field(self.$optional.as_deref(), false, $omax, $ocodes, $odate)?;)*
                Ok(())
            }
        }
    }
}

macro_rules! raw_row {
    ($name:ident {$($field:ident => $wire:literal),* $(,)?}) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(rename=$wire, default, skip_serializing_if="Option::is_none")]
                pub $field: Option<String>,
            )*
            #[serde(flatten)]
            pub extra: UnknownFields,
        }
    }
}

macro_rules! response {
    ($name:ident {$($field:ident => $wire:literal),* $(,)?} {$($list:ident => $row:ident),* $(,)?}) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(rename=$wire, default, skip_serializing_if="Option::is_none")]
                pub $field: Option<String>,
            )*
            $(
                #[serde(default, skip_serializing_if="Option::is_none")]
                pub $list: Option<Vec<$row>>,
            )*
            #[serde(flatten)]
            pub extra: UnknownFields,
        }

        impl $name {
            /// 응답 최상위와 알려진 각 행에 포함된 알 수 없는 필드를 보존합니다.
            pub fn unknown_field_maps(&self) -> Vec<&BTreeMap<String, serde_json::Value>> {
                #[allow(unused_mut)]
                let mut maps = vec![&self.extra];
                $(
                    if let Some(rows) = &self.$list {
                        maps.extend(rows.iter().map(|row| &row.extra));
                    }
                )*
                maps
            }
        }
    }
}

pub const CONTRACTS: &[(&str, &str)] = &[
    ("ka10010", "/api/dostk/sect"),
    ("ka10051", "/api/dostk/sect"),
    ("ka20001", "/api/dostk/sect"),
    ("ka20002", "/api/dostk/sect"),
    ("ka20003", "/api/dostk/sect"),
    ("ka20009", "/api/dostk/sect"),
    ("ka10060", "/api/dostk/chart"),
    ("ka10064", "/api/dostk/chart"),
    ("ka10079", "/api/dostk/chart"),
    ("ka10080", "/api/dostk/chart"),
    ("ka10081", "/api/dostk/chart"),
    ("ka10082", "/api/dostk/chart"),
    ("ka10083", "/api/dostk/chart"),
    ("ka10094", "/api/dostk/chart"),
    ("ka11079", "/api/dostk/chart"),
    ("ka11080", "/api/dostk/chart"),
    ("ka11081", "/api/dostk/chart"),
    ("ka11082", "/api/dostk/chart"),
    ("ka11083", "/api/dostk/chart"),
    ("ka11094", "/api/dostk/chart"),
    ("ka20004", "/api/dostk/chart"),
    ("ka20005", "/api/dostk/chart"),
    ("ka20006", "/api/dostk/chart"),
    ("ka20007", "/api/dostk/chart"),
    ("ka20008", "/api/dostk/chart"),
    ("ka20019", "/api/dostk/chart"),
    ("ka50079", "/api/dostk/chart"),
    ("ka50080", "/api/dostk/chart"),
    ("ka50081", "/api/dostk/chart"),
    ("ka50082", "/api/dostk/chart"),
    ("ka50083", "/api/dostk/chart"),
    ("ka50091", "/api/dostk/chart"),
    ("ka50092", "/api/dostk/chart"),
    ("usa23000", "/api/us/sect"),
    ("usa23100", "/api/us/sect"),
    ("usa06010", "/api/us/chart"),
    ("usa06011", "/api/us/chart"),
    ("usa06012", "/api/us/chart"),
    ("usa06013", "/api/us/chart"),
    ("usa06014", "/api/us/chart"),
    ("usa06015", "/api/us/chart"),
    ("usa06016", "/api/us/chart"),
];

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka10010 (확인일: 2026-10-12).
request!(Ka10010Request, {
    stk_cd => (20, &[], false)
}, {

});

response!(Ka10010Response {
    dfrt_trst_sell_qty => "dfrt_trst_sell_qty",
    dfrt_trst_sell_amt => "dfrt_trst_sell_amt",
    dfrt_trst_buy_qty => "dfrt_trst_buy_qty",
    dfrt_trst_buy_amt => "dfrt_trst_buy_amt",
    dfrt_trst_netprps_qty => "dfrt_trst_netprps_qty",
    dfrt_trst_netprps_amt => "dfrt_trst_netprps_amt",
    ndiffpro_trst_sell_qty => "ndiffpro_trst_sell_qty",
    ndiffpro_trst_sell_amt => "ndiffpro_trst_sell_amt",
    ndiffpro_trst_buy_qty => "ndiffpro_trst_buy_qty",
    ndiffpro_trst_buy_amt => "ndiffpro_trst_buy_amt",
    ndiffpro_trst_netprps_qty => "ndiffpro_trst_netprps_qty",
    ndiffpro_trst_netprps_amt => "ndiffpro_trst_netprps_amt",
    all_dfrt_trst_sell_qty => "all_dfrt_trst_sell_qty",
    all_dfrt_trst_sell_amt => "all_dfrt_trst_sell_amt",
    all_dfrt_trst_buy_qty => "all_dfrt_trst_buy_qty",
    all_dfrt_trst_buy_amt => "all_dfrt_trst_buy_amt",
    all_dfrt_trst_netprps_qty => "all_dfrt_trst_netprps_qty",
    all_dfrt_trst_netprps_amt => "all_dfrt_trst_netprps_amt"
} {});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka10051 (확인일: 2026-10-12).
request!(Ka10051Request, {
    mrkt_tp => (1, &["0", "1"], false),
    amt_qty_tp => (1, &["0", "1"], false),
    stex_tp => (1, &["1", "2", "3"], false)
}, {
    base_dt => (8, &[], true)
});

raw_row!(SectorNetBuyingRow {
    inds_cd => "inds_cd",
    inds_nm => "inds_nm",
    cur_prc => "cur_prc",
    pre_smbol => "pre_smbol",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    trde_qty => "trde_qty",
    sc_netprps => "sc_netprps",
    insrnc_netprps => "insrnc_netprps",
    invtrt_netprps => "invtrt_netprps",
    bank_netprps => "bank_netprps",
    jnsinkm_netprps => "jnsinkm_netprps",
    endw_netprps => "endw_netprps",
    etc_corp_netprps => "etc_corp_netprps",
    ind_netprps => "ind_netprps",
    frgnr_netprps => "frgnr_netprps",
    native_trmt_frgnr_netprps => "native_trmt_frgnr_netprps",
    natn_netprps => "natn_netprps",
    samo_fund_netprps => "samo_fund_netprps",
    orgn_netprps => "orgn_netprps"
});

response!(Ka10051Response {} {inds_netprps => SectorNetBuyingRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20001 (확인일: 2026-10-12).
request!(Ka20001Request, {
    mrkt_tp => (1, &["0", "1", "2"], false),
    inds_cd => (3, &[], false)
}, {

});

raw_row!(SectorIntradayRow {
    tm_n => "tm_n",
    cur_prc_n => "cur_prc_n",
    pred_pre_sig_n => "pred_pre_sig_n",
    pred_pre_n => "pred_pre_n",
    flu_rt_n => "flu_rt_n",
    trde_qty_n => "trde_qty_n",
    acc_trde_qty_n => "acc_trde_qty_n"
});

response!(Ka20001Response {
    cur_prc => "cur_prc",
    pred_pre_sig => "pred_pre_sig",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    trde_qty => "trde_qty",
    trde_prica => "trde_prica",
    trde_frmatn_stk_num => "trde_frmatn_stk_num",
    trde_frmatn_rt => "trde_frmatn_rt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    upl => "upl",
    rising => "rising",
    stdns => "stdns",
    fall => "fall",
    lst => "lst",
    week_wk_hgst_pric => "52wk_hgst_pric",
    week_wk_hgst_pric_dt => "52wk_hgst_pric_dt",
    week_wk_hgst_pric_pre_rt => "52wk_hgst_pric_pre_rt",
    week_wk_lwst_pric => "52wk_lwst_pric",
    week_wk_lwst_pric_dt => "52wk_lwst_pric_dt",
    week_wk_lwst_pric_pre_rt => "52wk_lwst_pric_pre_rt"
} {inds_cur_prc_tm => SectorIntradayRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20002 (확인일: 2026-10-12).
request!(Ka20002Request, {
    mrkt_tp => (1, &["0", "1", "2"], false),
    inds_cd => (3, &[], false),
    stex_tp => (1, &["1", "2", "3"], false)
}, {

});

raw_row!(SectorStockRow {
    stk_cd => "stk_cd",
    stk_nm => "stk_nm",
    cur_prc => "cur_prc",
    pred_pre_sig => "pred_pre_sig",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    now_trde_qty => "now_trde_qty",
    sel_bid => "sel_bid",
    buy_bid => "buy_bid",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric"
});

response!(Ka20002Response {} {inds_stkpc => SectorStockRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20003 (확인일: 2026-10-12).
request!(Ka20003Request, {
    inds_cd => (3, &["001", "101"], false)
}, {

});

raw_row!(AllSectorIndexRow {
    stk_cd => "stk_cd",
    stk_nm => "stk_nm",
    cur_prc => "cur_prc",
    pre_sig => "pre_sig",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    trde_qty => "trde_qty",
    wght => "wght",
    trde_prica => "trde_prica",
    upl => "upl",
    rising => "rising",
    stdns => "stdns",
    fall => "fall",
    lst => "lst",
    flo_stk_num => "flo_stk_num"
});

response!(Ka20003Response {} {all_inds_idex => AllSectorIndexRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20009 (확인일: 2026-10-12).
request!(Ka20009Request, {
    mrkt_tp => (1, &["0", "1", "2"], false),
    inds_cd => (3, &[], false)
}, {

});

raw_row!(SectorDailyPriceRow {
    dt_n => "dt_n",
    cur_prc_n => "cur_prc_n",
    pred_pre_sig_n => "pred_pre_sig_n",
    pred_pre_n => "pred_pre_n",
    flu_rt_n => "flu_rt_n",
    acc_trde_qty_n => "acc_trde_qty_n"
});

response!(Ka20009Response {
    cur_prc => "cur_prc",
    pred_pre_sig => "pred_pre_sig",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    trde_qty => "trde_qty",
    trde_prica => "trde_prica",
    trde_frmatn_stk_num => "trde_frmatn_stk_num",
    trde_frmatn_rt => "trde_frmatn_rt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    upl => "upl",
    rising => "rising",
    stdns => "stdns",
    fall => "fall",
    lst => "lst",
    week_wk_hgst_pric => "52wk_hgst_pric",
    week_wk_hgst_pric_dt => "52wk_hgst_pric_dt",
    week_wk_hgst_pric_pre_rt => "52wk_hgst_pric_pre_rt",
    week_wk_lwst_pric => "52wk_lwst_pric",
    week_wk_lwst_pric_dt => "52wk_lwst_pric_dt",
    week_wk_lwst_pric_pre_rt => "52wk_lwst_pric_pre_rt"
} {inds_cur_prc_daly_rept => SectorDailyPriceRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10060 (확인일: 2026-10-12).
request!(Ka10060Request, {
    dt => (8, &[], true),
    stk_cd => (20, &[], false),
    amt_qty_tp => (1, &["1", "2"], false),
    trde_tp => (1, &["0", "1", "2"], false),
    unit_tp => (4, &["1000", "1"], false)
}, {

});

raw_row!(InvestorDailyRow {
    dt => "dt",
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    acc_trde_prica => "acc_trde_prica",
    ind_invsr => "ind_invsr",
    frgnr_invsr => "frgnr_invsr",
    orgn => "orgn",
    fnnc_invt => "fnnc_invt",
    insrnc => "insrnc",
    invtrt => "invtrt",
    etc_fnnc => "etc_fnnc",
    bank => "bank",
    penfnd_etc => "penfnd_etc",
    samo_fund => "samo_fund",
    natn => "natn",
    etc_corp => "etc_corp",
    natfor => "natfor"
});

response!(Ka10060Response {} {stk_invsr_orgn_chart => InvestorDailyRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10064 (확인일: 2026-10-12).
request!(Ka10064Request, {
    mrkt_tp => (3, &["000", "001", "101"], false),
    amt_qty_tp => (1, &["1", "2"], false),
    trde_tp => (1, &["0", "1", "2"], false),
    stk_cd => (20, &[], false)
}, {

});

raw_row!(InvestorIntradayRow {
    tm => "tm",
    frgnr_invsr => "frgnr_invsr",
    orgn => "orgn",
    invtrt => "invtrt",
    insrnc => "insrnc",
    bank => "bank",
    penfnd_etc => "penfnd_etc",
    etc_corp => "etc_corp",
    natn => "natn"
});

response!(Ka10064Response {} {opmr_invsr_trde_chart => InvestorIntradayRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10079 (확인일: 2026-10-12).
request!(Ka10079Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "30"], false),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(DomesticTickRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Ka10079Response {
    stk_cd => "stk_cd",
    last_tic_cnt => "last_tic_cnt"
} {stk_tic_chart_qry => DomesticTickRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11079 (확인일: 2026-10-12).
request!(Ka11079Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "30"], false),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka11079Response {
    stk_cd => "stk_cd",
    last_tic_cnt => "last_tic_cnt"
} {result_list => DomesticTickRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10080 (확인일: 2026-10-12).
request!(Ka10080Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "15", "30", "45", "60"], false),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {
    base_dt => (8, &[], true)
});

raw_row!(DomesticMinuteRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    acc_trde_qty => "acc_trde_qty",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Ka10080Response {
    stk_cd => "stk_cd"
} {stk_min_pole_chart_qry => DomesticMinuteRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11080 (확인일: 2026-10-12).
request!(Ka11080Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "15", "30", "45", "60"], false),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {
    base_dt => (8, &[], true)
});

response!(Ka11080Response {
    stk_cd => "stk_cd"
} {result_list => DomesticMinuteRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10081 (확인일: 2026-10-12).
request!(Ka10081Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(DomesticPeriodRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    trde_prica => "trde_prica",
    dt => "dt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
    trde_tern_rt => "trde_tern_rt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Ka10081Response {
    stk_cd => "stk_cd"
} {stk_dt_pole_chart_qry => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10082 (확인일: 2026-10-12).
request!(Ka10082Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka10082Response {
    stk_cd => "stk_cd"
} {stk_stk_pole_chart_qry => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10083 (확인일: 2026-10-12).
request!(Ka10083Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka10083Response {
    stk_cd => "stk_cd"
} {stk_mth_pole_chart_qry => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11081 (확인일: 2026-10-12).
request!(Ka11081Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka11081Response {
    stk_cd => "stk_cd"
} {result_list => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11082 (확인일: 2026-10-12).
request!(Ka11082Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka11082Response {
    stk_cd => "stk_cd"
} {result_list => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11083 (확인일: 2026-10-12).
request!(Ka11083Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka11083Response {
    stk_cd => "stk_cd"
} {result_list => DomesticPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10094 (확인일: 2026-10-12).
request!(Ka10094Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(DomesticYearRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    trde_prica => "trde_prica",
    dt => "dt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Ka10094Response {
    stk_cd => "stk_cd"
} {stk_yr_pole_chart_qry => DomesticYearRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11094 (확인일: 2026-10-12).
request!(Ka11094Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka11094Response {
    stk_cd => "stk_cd"
} {result_list => DomesticYearRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20004 (확인일: 2026-10-12).
request!(Ka20004Request, {
    inds_cd => (3, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "30"], false)
}, {

});

raw_row!(SectorTickRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka20004Response {
    inds_cd => "inds_cd"
} {inds_tic_chart_qry => SectorTickRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20005 (확인일: 2026-10-12).
request!(Ka20005Request, {
    inds_cd => (3, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "15", "30", "45", "60"], false)
}, {
    base_dt => (8, &[], true)
});

raw_row!(SectorMinuteRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    acc_trde_qty => "acc_trde_qty",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka20005Response {
    inds_cd => "inds_cd"
} {inds_min_pole_qry => SectorMinuteRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20006 (확인일: 2026-10-12).
request!(Ka20006Request, {
    inds_cd => (3, &[], false),
    base_dt => (8, &[], true)
}, {

});

raw_row!(SectorPeriodRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    dt => "dt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    trde_prica => "trde_prica"
});

response!(Ka20006Response {
    inds_cd => "inds_cd"
} {inds_dt_pole_qry => SectorPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20007 (확인일: 2026-10-12).
// 문서는 inds_cd 길이를 8, base_dt 길이를 3으로 적으면서 날짜를 YYYYMMDD로 설명합니다.
// 클라이언트는 YYYYMMDD를 검증하며, 설명과 모순되는 base_dt 길이 3은 적용하지 않습니다.
request!(Ka20007Request, {
    inds_cd => (8, &[], false),
    base_dt => (0, &[], true)
}, {

});

response!(Ka20007Response {
    inds_cd => "inds_cd"
} {inds_stk_pole_qry => SectorPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20008 (확인일: 2026-10-12).
request!(Ka20008Request, {
    inds_cd => (3, &[], false),
    base_dt => (8, &[], true)
}, {

});

response!(Ka20008Response {
    inds_cd => "inds_cd"
} {inds_mth_pole_qry => SectorPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20019 (확인일: 2026-10-12).
request!(Ka20019Request, {
    inds_cd => (3, &[], false),
    base_dt => (8, &[], true)
}, {

});

response!(Ka20019Response {
    inds_cd => "inds_cd"
} {inds_yr_pole_qry => SectorPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50079 (확인일: 2026-10-12).
request!(Ka50079Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "30"], false),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(GoldTickRow {
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka50079Response {} {gds_tic_chart_qry => GoldTickRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50080 (확인일: 2026-10-12).
request!(Ka50080Request, {
    stk_cd => (20, &[], false),
    tic_scope => (3, &["1", "3", "5", "10", "15", "30", "45", "60"], false)
}, {
    upd_stkpc_tp => (1, &["0", "1"], false)
});

raw_row!(GoldMinuteRow {
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    acc_trde_qty => "acc_trde_qty",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka50080Response {} {gds_min_chart_qry => GoldMinuteRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50081 (확인일: 2026-10-12).
request!(Ka50081Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(GoldDailyRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka50081Response {} {gds_day_chart_qry => GoldDailyRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50082 (확인일: 2026-10-12).
request!(Ka50082Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

raw_row!(GoldPeriodRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt"
});

response!(Ka50082Response {} {gds_week_chart_qry => GoldPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50083 (확인일: 2026-10-12).
request!(Ka50083Request, {
    stk_cd => (20, &[], false),
    base_dt => (8, &[], true),
    upd_stkpc_tp => (1, &["0", "1"], false)
}, {

});

response!(Ka50083Response {} {gds_month_chart_qry => GoldPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50091 (확인일: 2026-10-12).
request!(Ka50091Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "30"], false)
}, {

});

raw_row!(GoldTickDetailRow {
    cntr_pric => "cntr_pric",
    pred_pre => "pred_pre",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka50091Response {} {gds_tic_chart_qry => GoldTickDetailRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50092 (확인일: 2026-10-12).
request!(Ka50092Request, {
    stk_cd => (20, &[], false),
    tic_scope => (2, &["1", "3", "5", "10", "15", "30", "45", "60"], false)
}, {

});

raw_row!(GoldMinuteDetailRow {
    cntr_pric => "cntr_pric",
    pred_pre => "pred_pre",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig"
});

response!(Ka50092Response {} {gds_min_chart_qry => GoldMinuteDetailRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/34/usa23000 (확인일: 2026-10-12).
request!(Usa23000Request, {

}, {
    stex_tp => (0, &["0", "1", "2", "3"], false),
    inds_cd => (0, &[], false)
});

raw_row!(UsSectorPerformanceRow {
    inds_cd => "inds_cd",
    inds_nm => "inds_nm",
    perf_1d => "perf_1d",
    perf_5d => "perf_5d",
    perf_1m => "perf_1m",
    perf_3m => "perf_3m",
    perf_6m => "perf_6m",
    perf_ytd => "perf_ytd",
    perf_1y => "perf_1y"
});

response!(Usa23000Response {} {result_list => UsSectorPerformanceRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/34/usa23100 (확인일: 2026-10-12).
request!(Usa23100Request, {

}, {
    stex_tp => (0, &["0", "1", "2", "3"], false),
    sort_tp => (0, &["1", "2"], false),
    inds_cd => (0, &[], false)
});

raw_row!(UsSectorStockRow {
    stex_tp => "stex_tp",
    stk_cd => "stk_cd",
    stk_nm => "stk_nm",
    stk_enm => "stk_enm",
    cur_prc => "cur_prc",
    pred_pre_sig => "pred_pre_sig",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    acc_trde_qty => "acc_trde_qty",
    sel_bid => "sel_bid",
    buy_bid => "buy_bid",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm"
});

response!(Usa23100Response {} {result_list => UsSectorStockRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06010 (확인일: 2026-10-12).
request!(Usa06010Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false),
    stk_cd => (0, &[], false)
}, {
    tic_scope => (0, &["1", "3", "5", "10", "20", "30", "60", "120"], false),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

raw_row!(UsIntradayRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    bus_dt => "bus_dt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Usa06010Response {} {result_list => UsIntradayRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06011 (확인일: 2026-10-12).
request!(Usa06011Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    tic_scope => (0, &["1", "3", "5", "10", "15", "30", "40", "60", "120"], false),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

response!(Usa06011Response {} {result_list => UsIntradayRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06012 (확인일: 2026-10-12).
request!(Usa06012Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

raw_row!(UsDailyRow {
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    flu_rt => "flu_rt",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Usa06012Response {} {result_list => UsDailyRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06013 (확인일: 2026-10-12).
request!(Usa06013Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

raw_row!(UsPeriodRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt"
});

response!(Usa06013Response {} {result_list => UsPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06014 (확인일: 2026-10-12).
request!(Usa06014Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

response!(Usa06014Response {} {result_list => UsPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06015 (확인일: 2026-10-12).
request!(Usa06015Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

response!(Usa06015Response {} {result_list => UsPeriodRow});

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06016 (확인일: 2026-10-12).
request!(Usa06016Request, {
    stex_tp => (0, &["NA", "ND", "NY"], false)
}, {
    stk_cd => (0, &[], false),
    strt_dt => (0, &[], true),
    upd_stkpc_tp => (0, &["0", "1"], false),
    exrt_appl_tp => (0, &["0", "1"], false)
});

response!(Usa06016Response {} {result_list => UsPeriodRow});
