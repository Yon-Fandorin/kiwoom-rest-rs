use async_trait::async_trait;
use kiwoom_client::transport::{HttpRequest, HttpResponse, TransportError};
use kiwoom_client::{Client, ClientOptions, Secret, Transport, charts::*};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone)]
struct Mock {
    seen: Arc<Mutex<Vec<HttpRequest>>>,
    body: Value,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.seen.lock().unwrap().push(request);
        let headers = if self.seen.lock().unwrap().len() == 1 {
            BTreeMap::from([
                ("cont-yn".into(), "Y".into()),
                ("next-key".into(), "offline-next".into()),
            ])
        } else {
            BTreeMap::from([("cont-yn".into(), "N".into())])
        };
        Ok(HttpResponse {
            status: 200,
            headers,
            body: self.body.clone(),
        })
    }
}

fn options() -> ClientOptions {
    ClientOptions {
        min_request_interval: Duration::ZERO,
        retry_base_delay: Duration::ZERO,
        ..Default::default()
    }
}

// 각 공개 키의 값을 이름이 있는 Rust 필드로 읽어 rename 교환도 검출한다.
// 기대값은 아래 독립 합성 JSON에서 읽으며 직렬화 결과에서 만들지 않는다.
trait WireBindings {
    fn assert_wire_bindings(&self, wire: &Value);
}

macro_rules! wire_bindings {
    ($ty:ident { $($field:ident => $key:literal),* $(,)? } { $($list:ident => $list_key:literal),* $(,)? }) => {
        impl WireBindings for $ty {
            fn assert_wire_bindings(&self, wire: &Value) {
                $(assert_eq!(self.$field.as_deref(), wire[$key].as_str(), $key);)*
                $(
                    let rows = self.$list.as_ref().unwrap();
                    let expected_rows = wire[$list_key].as_array().unwrap();
                    assert_eq!(rows.len(), expected_rows.len(), $list_key);
                    for (row, expected) in rows.iter().zip(expected_rows) {
                        row.assert_wire_bindings(expected);
                    }
                )*
            }
        }
    };
}

wire_bindings!(SectorNetBuyingRow {
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
    orgn_netprps => "orgn_netprps",
} {});
wire_bindings!(SectorIntradayRow {
    tm_n => "tm_n",
    cur_prc_n => "cur_prc_n",
    pred_pre_sig_n => "pred_pre_sig_n",
    pred_pre_n => "pred_pre_n",
    flu_rt_n => "flu_rt_n",
    trde_qty_n => "trde_qty_n",
    acc_trde_qty_n => "acc_trde_qty_n",
} {});
wire_bindings!(SectorDailyPriceRow {
    dt_n => "dt_n",
    cur_prc_n => "cur_prc_n",
    pred_pre_sig_n => "pred_pre_sig_n",
    pred_pre_n => "pred_pre_n",
    flu_rt_n => "flu_rt_n",
    acc_trde_qty_n => "acc_trde_qty_n",
} {});
wire_bindings!(SectorStockRow {
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
    low_pric => "low_pric",
} {});
wire_bindings!(AllSectorIndexRow {
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
    flo_stk_num => "flo_stk_num",
} {});
wire_bindings!(InvestorDailyRow {
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
    natfor => "natfor",
} {});
wire_bindings!(InvestorIntradayRow {
    tm => "tm",
    frgnr_invsr => "frgnr_invsr",
    orgn => "orgn",
    invtrt => "invtrt",
    insrnc => "insrnc",
    bank => "bank",
    penfnd_etc => "penfnd_etc",
    etc_corp => "etc_corp",
    natn => "natn",
} {});
wire_bindings!(DomesticTickRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt",
} {});
wire_bindings!(DomesticMinuteRow {
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
    upd_rt => "upd_rt",
} {});
wire_bindings!(DomesticPeriodRow {
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
    upd_rt => "upd_rt",
} {});
wire_bindings!(DomesticYearRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    trde_prica => "trde_prica",
    dt => "dt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt",
} {});
wire_bindings!(SectorTickRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(SectorMinuteRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    cntr_tm => "cntr_tm",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    acc_trde_qty => "acc_trde_qty",
    pred_pre => "pred_pre",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(SectorPeriodRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    dt => "dt",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    trde_prica => "trde_prica",
} {});
wire_bindings!(GoldTickRow {
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(GoldMinuteRow {
    cur_prc => "cur_prc",
    pred_pre => "pred_pre",
    acc_trde_qty => "acc_trde_qty",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(GoldDailyRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(GoldPeriodRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
} {});
wire_bindings!(GoldTickDetailRow {
    cntr_pric => "cntr_pric",
    pred_pre => "pred_pre",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    dt => "dt",
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(GoldMinuteDetailRow {
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
    pred_pre_sig => "pred_pre_sig",
} {});
wire_bindings!(UsSectorPerformanceRow {
    inds_cd => "inds_cd",
    inds_nm => "inds_nm",
    perf_1d => "perf_1d",
    perf_5d => "perf_5d",
    perf_1m => "perf_1m",
    perf_3m => "perf_3m",
    perf_6m => "perf_6m",
    perf_ytd => "perf_ytd",
    perf_1y => "perf_1y",
} {});
wire_bindings!(UsSectorStockRow {
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
    cntr_tm => "cntr_tm",
} {});
wire_bindings!(UsIntradayRow {
    cur_prc => "cur_prc",
    trde_qty => "trde_qty",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    cntr_tm => "cntr_tm",
    bus_dt => "bus_dt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt",
} {});
wire_bindings!(UsDailyRow {
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
    upd_rt => "upd_rt",
} {});
wire_bindings!(UsPeriodRow {
    cur_prc => "cur_prc",
    acc_trde_qty => "acc_trde_qty",
    acc_trde_prica => "acc_trde_prica",
    open_pric => "open_pric",
    high_pric => "high_pric",
    low_pric => "low_pric",
    dt => "dt",
    upd_stkpc_tp => "upd_stkpc_tp",
    upd_rt => "upd_rt",
} {});
wire_bindings!(Ka10010Response {
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
    all_dfrt_trst_netprps_amt => "all_dfrt_trst_netprps_amt",
} {});
wire_bindings!(Ka10051Response {
} { inds_netprps => "inds_netprps" });
wire_bindings!(Ka20001Response {
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
    week_wk_lwst_pric_pre_rt => "52wk_lwst_pric_pre_rt",
} { inds_cur_prc_tm => "inds_cur_prc_tm" });
wire_bindings!(Ka20002Response {
} { inds_stkpc => "inds_stkpc" });
wire_bindings!(Ka20003Response {
} { all_inds_idex => "all_inds_idex" });
wire_bindings!(Ka20009Response {
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
    week_wk_lwst_pric_pre_rt => "52wk_lwst_pric_pre_rt",
} { inds_cur_prc_daly_rept => "inds_cur_prc_daly_rept" });
wire_bindings!(Ka10060Response {
} { stk_invsr_orgn_chart => "stk_invsr_orgn_chart" });
wire_bindings!(Ka10064Response {
} { opmr_invsr_trde_chart => "opmr_invsr_trde_chart" });
wire_bindings!(Ka10079Response {
    stk_cd => "stk_cd",
    last_tic_cnt => "last_tic_cnt",
} { stk_tic_chart_qry => "stk_tic_chart_qry" });
wire_bindings!(Ka10080Response {
    stk_cd => "stk_cd",
} { stk_min_pole_chart_qry => "stk_min_pole_chart_qry" });
wire_bindings!(Ka10081Response {
    stk_cd => "stk_cd",
} { stk_dt_pole_chart_qry => "stk_dt_pole_chart_qry" });
wire_bindings!(Ka10082Response {
    stk_cd => "stk_cd",
} { stk_stk_pole_chart_qry => "stk_stk_pole_chart_qry" });
wire_bindings!(Ka10083Response {
    stk_cd => "stk_cd",
} { stk_mth_pole_chart_qry => "stk_mth_pole_chart_qry" });
wire_bindings!(Ka10094Response {
    stk_cd => "stk_cd",
} { stk_yr_pole_chart_qry => "stk_yr_pole_chart_qry" });
wire_bindings!(Ka11079Response {
    stk_cd => "stk_cd",
    last_tic_cnt => "last_tic_cnt",
} { result_list => "result_list" });
wire_bindings!(Ka11080Response {
    stk_cd => "stk_cd",
} { result_list => "result_list" });
wire_bindings!(Ka11081Response {
    stk_cd => "stk_cd",
} { result_list => "result_list" });
wire_bindings!(Ka11082Response {
    stk_cd => "stk_cd",
} { result_list => "result_list" });
wire_bindings!(Ka11083Response {
    stk_cd => "stk_cd",
} { result_list => "result_list" });
wire_bindings!(Ka11094Response {
    stk_cd => "stk_cd",
} { result_list => "result_list" });
wire_bindings!(Ka20004Response {
    inds_cd => "inds_cd",
} { inds_tic_chart_qry => "inds_tic_chart_qry" });
wire_bindings!(Ka20005Response {
    inds_cd => "inds_cd",
} { inds_min_pole_qry => "inds_min_pole_qry" });
wire_bindings!(Ka20006Response {
    inds_cd => "inds_cd",
} { inds_dt_pole_qry => "inds_dt_pole_qry" });
wire_bindings!(Ka20007Response {
    inds_cd => "inds_cd",
} { inds_stk_pole_qry => "inds_stk_pole_qry" });
wire_bindings!(Ka20008Response {
    inds_cd => "inds_cd",
} { inds_mth_pole_qry => "inds_mth_pole_qry" });
wire_bindings!(Ka20019Response {
    inds_cd => "inds_cd",
} { inds_yr_pole_qry => "inds_yr_pole_qry" });
wire_bindings!(Ka50079Response {
} { gds_tic_chart_qry => "gds_tic_chart_qry" });
wire_bindings!(Ka50080Response {
} { gds_min_chart_qry => "gds_min_chart_qry" });
wire_bindings!(Ka50081Response {
} { gds_day_chart_qry => "gds_day_chart_qry" });
wire_bindings!(Ka50082Response {
} { gds_week_chart_qry => "gds_week_chart_qry" });
wire_bindings!(Ka50083Response {
} { gds_month_chart_qry => "gds_month_chart_qry" });
wire_bindings!(Ka50091Response {
} { gds_tic_chart_qry => "gds_tic_chart_qry" });
wire_bindings!(Ka50092Response {
} { gds_min_chart_qry => "gds_min_chart_qry" });
wire_bindings!(Usa23000Response {
} { result_list => "result_list" });
wire_bindings!(Usa23100Response {
} { result_list => "result_list" });
wire_bindings!(Usa06010Response {
} { result_list => "result_list" });
wire_bindings!(Usa06011Response {
} { result_list => "result_list" });
wire_bindings!(Usa06012Response {
} { result_list => "result_list" });
wire_bindings!(Usa06013Response {
} { result_list => "result_list" });
wire_bindings!(Usa06014Response {
} { result_list => "result_list" });
wire_bindings!(Usa06015Response {
} { result_list => "result_list" });
wire_bindings!(Usa06016Response {
} { result_list => "result_list" });

// 날짜와 시각은 별도의 합성 달력 값을 사용하고 숫자 문자열은 부호·패딩을 유지한다.
fn fixture_value(key: &str, index: usize, row: bool) -> String {
    match key {
        "dt" => "20261012".into(),
        "dt_n" => "20261008".into(),
        "bus_dt" => "20261009".into(),
        "52wk_hgst_pric_dt" => "20261001".into(),
        "52wk_lwst_pric_dt" => "20260102".into(),
        "cntr_tm" => "093001".into(),
        "tm_n" => "093002".into(),
        "tm" => "093003".into(),
        _ if row => format!("-{:06}.2500", index + 101),
        _ => format!("+{:06}.4500", index + 1),
    }
}

// 원본의 부호와 패딩을 유지하면서 각 키에 서로 다른 합성 값을 넣는다.
fn fixture(top: &[&str], list: Option<(&str, &[&str])>) -> Value {
    let mut body = serde_json::Map::new();
    body.insert("return_code".into(), json!(0));
    body.insert("unknown_chart_field".into(), json!({"unchanged":true}));
    for (index, key) in top.iter().enumerate() {
        body.insert((*key).into(), json!(fixture_value(key, index, false)));
    }
    if let Some((name, fields)) = list {
        let mut row = serde_json::Map::new();
        for (index, key) in fields.iter().enumerate() {
            row.insert((*key).into(), json!(fixture_value(key, index, true)));
        }
        row.insert("unknown_row_field".into(), json!(null));
        body.insert(name.into(), json!([row]));
    }
    Value::Object(body)
}

macro_rules! contract_test {
    ($test:ident, $method:ident, $request:ident, $path:literal, $wire:expr, $body:expr) => {
        // 두 페이지의 차트 원본과 타입 필드를 보존하고 후속 커서를 정확히 전송한다.
        #[tokio::test]
        async fn $test() {
            let wire = $wire;
            let request: $request = serde_json::from_value(wire.clone()).unwrap();
            request.validate().unwrap();
            assert_eq!(serde_json::to_value(&request).unwrap(), wire);

            let body = $body;
            let mock = Mock {
                seen: Arc::new(Mutex::new(vec![])),
                body: body.clone(),
            };
            let seen = mock.seen.clone();
            let client = Client::new(mock, options()).unwrap();
            let pages = client
                .$method(&Secret::new("offline-only".into()), &request)
                .await
                .unwrap();
            assert_eq!(pages.len(), 2);
            for page in &pages {
                assert_eq!(page.raw, body);
                page.typed.assert_wire_bindings(&body);
                assert_eq!(serde_json::to_value(&page.typed).unwrap(), body);
                let unknowns = page.typed.unknown_field_maps();
                assert_eq!(unknowns[0].len(), 2, "known root fields must be typed");
                assert_eq!(unknowns[0]["return_code"], 0);
                for row in unknowns.iter().skip(1) {
                    assert_eq!(row.len(), 1, "all documented row fields must be typed");
                    assert_eq!(row["unknown_row_field"], Value::Null);
                }
            }

            // 첫 요청과 후속 요청의 커서 차이를 확인한다.
            let seen = seen.lock().unwrap();
            assert_eq!(seen.len(), 2);
            for sent in seen.iter() {
                assert_eq!(sent.api_id, stringify!($method));
                assert_eq!(sent.path, $path);
                assert_eq!(sent.body, wire);
                assert!(sent.authorization.is_some());
            }
            assert!(seen[0].continuation.is_none());
            assert_eq!(seen[1].continuation.as_deref(), Some("offline-next"));
        }
    };
}

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka10010 (2026-10-12).
contract_test!(
    mock_ka10010,
    ka10010,
    Ka10010Request,
    "/api/dostk/sect",
    json!({"stk_cd": "039490"}),
    fixture(
        &[
            "dfrt_trst_sell_qty",
            "dfrt_trst_sell_amt",
            "dfrt_trst_buy_qty",
            "dfrt_trst_buy_amt",
            "dfrt_trst_netprps_qty",
            "dfrt_trst_netprps_amt",
            "ndiffpro_trst_sell_qty",
            "ndiffpro_trst_sell_amt",
            "ndiffpro_trst_buy_qty",
            "ndiffpro_trst_buy_amt",
            "ndiffpro_trst_netprps_qty",
            "ndiffpro_trst_netprps_amt",
            "all_dfrt_trst_sell_qty",
            "all_dfrt_trst_sell_amt",
            "all_dfrt_trst_buy_qty",
            "all_dfrt_trst_buy_amt",
            "all_dfrt_trst_netprps_qty",
            "all_dfrt_trst_netprps_amt"
        ],
        None
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka10051 (2026-10-12).
contract_test!(
    mock_ka10051,
    ka10051,
    Ka10051Request,
    "/api/dostk/sect",
    json!({"mrkt_tp": "0", "amt_qty_tp": "0", "stex_tp": "1", "base_dt": "20261012"}),
    fixture(
        &[],
        Some((
            "inds_netprps",
            &[
                "inds_cd",
                "inds_nm",
                "cur_prc",
                "pre_smbol",
                "pred_pre",
                "flu_rt",
                "trde_qty",
                "sc_netprps",
                "insrnc_netprps",
                "invtrt_netprps",
                "bank_netprps",
                "jnsinkm_netprps",
                "endw_netprps",
                "etc_corp_netprps",
                "ind_netprps",
                "frgnr_netprps",
                "native_trmt_frgnr_netprps",
                "natn_netprps",
                "samo_fund_netprps",
                "orgn_netprps"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20001 (2026-10-12).
contract_test!(
    mock_ka20001,
    ka20001,
    Ka20001Request,
    "/api/dostk/sect",
    json!({"mrkt_tp": "0", "inds_cd": "001"}),
    fixture(
        &[
            "cur_prc",
            "pred_pre_sig",
            "pred_pre",
            "flu_rt",
            "trde_qty",
            "trde_prica",
            "trde_frmatn_stk_num",
            "trde_frmatn_rt",
            "open_pric",
            "high_pric",
            "low_pric",
            "upl",
            "rising",
            "stdns",
            "fall",
            "lst",
            "52wk_hgst_pric",
            "52wk_hgst_pric_dt",
            "52wk_hgst_pric_pre_rt",
            "52wk_lwst_pric",
            "52wk_lwst_pric_dt",
            "52wk_lwst_pric_pre_rt"
        ],
        Some((
            "inds_cur_prc_tm",
            &[
                "tm_n",
                "cur_prc_n",
                "pred_pre_sig_n",
                "pred_pre_n",
                "flu_rt_n",
                "trde_qty_n",
                "acc_trde_qty_n"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20002 (2026-10-12).
contract_test!(
    mock_ka20002,
    ka20002,
    Ka20002Request,
    "/api/dostk/sect",
    json!({"mrkt_tp": "0", "inds_cd": "001", "stex_tp": "1"}),
    fixture(
        &[],
        Some((
            "inds_stkpc",
            &[
                "stk_cd",
                "stk_nm",
                "cur_prc",
                "pred_pre_sig",
                "pred_pre",
                "flu_rt",
                "now_trde_qty",
                "sel_bid",
                "buy_bid",
                "open_pric",
                "high_pric",
                "low_pric"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20003 (2026-10-12).
contract_test!(
    mock_ka20003,
    ka20003,
    Ka20003Request,
    "/api/dostk/sect",
    json!({"inds_cd": "001"}),
    fixture(
        &[],
        Some((
            "all_inds_idex",
            &[
                "stk_cd",
                "stk_nm",
                "cur_prc",
                "pre_sig",
                "pred_pre",
                "flu_rt",
                "trde_qty",
                "wght",
                "trde_prica",
                "upl",
                "rising",
                "stdns",
                "fall",
                "lst",
                "flo_stk_num"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/04/ka20009 (2026-10-12).
contract_test!(
    mock_ka20009,
    ka20009,
    Ka20009Request,
    "/api/dostk/sect",
    json!({"mrkt_tp": "0", "inds_cd": "001"}),
    fixture(
        &[
            "cur_prc",
            "pred_pre_sig",
            "pred_pre",
            "flu_rt",
            "trde_qty",
            "trde_prica",
            "trde_frmatn_stk_num",
            "trde_frmatn_rt",
            "open_pric",
            "high_pric",
            "low_pric",
            "upl",
            "rising",
            "stdns",
            "fall",
            "lst",
            "52wk_hgst_pric",
            "52wk_hgst_pric_dt",
            "52wk_hgst_pric_pre_rt",
            "52wk_lwst_pric",
            "52wk_lwst_pric_dt",
            "52wk_lwst_pric_pre_rt"
        ],
        Some((
            "inds_cur_prc_daly_rept",
            &[
                "dt_n",
                "cur_prc_n",
                "pred_pre_sig_n",
                "pred_pre_n",
                "flu_rt_n",
                "acc_trde_qty_n"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10060 (2026-10-12).
contract_test!(
    mock_ka10060,
    ka10060,
    Ka10060Request,
    "/api/dostk/chart",
    json!({"dt": "20261012", "stk_cd": "039490", "amt_qty_tp": "1", "trde_tp": "0", "unit_tp": "1000"}),
    fixture(
        &[],
        Some((
            "stk_invsr_orgn_chart",
            &[
                "dt",
                "cur_prc",
                "pred_pre",
                "acc_trde_prica",
                "ind_invsr",
                "frgnr_invsr",
                "orgn",
                "fnnc_invt",
                "insrnc",
                "invtrt",
                "etc_fnnc",
                "bank",
                "penfnd_etc",
                "samo_fund",
                "natn",
                "etc_corp",
                "natfor"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10064 (2026-10-12).
contract_test!(
    mock_ka10064,
    ka10064,
    Ka10064Request,
    "/api/dostk/chart",
    json!({"mrkt_tp": "000", "amt_qty_tp": "1", "trde_tp": "0", "stk_cd": "039490"}),
    fixture(
        &[],
        Some((
            "opmr_invsr_trde_chart",
            &[
                "tm",
                "frgnr_invsr",
                "orgn",
                "invtrt",
                "insrnc",
                "bank",
                "penfnd_etc",
                "etc_corp",
                "natn"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10079 (2026-10-12).
contract_test!(
    mock_ka10079,
    ka10079,
    Ka10079Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "tic_scope": "1", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd", "last_tic_cnt"],
        Some((
            "stk_tic_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10080 (2026-10-12).
contract_test!(
    mock_ka10080,
    ka10080,
    Ka10080Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "tic_scope": "1", "upd_stkpc_tp": "0", "base_dt": "20261012"}),
    fixture(
        &["stk_cd"],
        Some((
            "stk_min_pole_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "acc_trde_qty",
                "pred_pre",
                "pred_pre_sig",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10081 (2026-10-12).
contract_test!(
    mock_ka10081,
    ka10081,
    Ka10081Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "stk_dt_pole_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10082 (2026-10-12).
contract_test!(
    mock_ka10082,
    ka10082,
    Ka10082Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "stk_stk_pole_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10083 (2026-10-12).
contract_test!(
    mock_ka10083,
    ka10083,
    Ka10083Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "stk_mth_pole_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10094 (2026-10-12).
contract_test!(
    mock_ka10094,
    ka10094,
    Ka10094Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "stk_yr_pole_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11079 (2026-10-12).
contract_test!(
    mock_ka11079,
    ka11079,
    Ka11079Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "tic_scope": "1", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd", "last_tic_cnt"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11080 (2026-10-12).
contract_test!(
    mock_ka11080,
    ka11080,
    Ka11080Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "tic_scope": "1", "upd_stkpc_tp": "0", "base_dt": "20261012"}),
    fixture(
        &["stk_cd"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "acc_trde_qty",
                "pred_pre",
                "pred_pre_sig",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11081 (2026-10-12).
contract_test!(
    mock_ka11081,
    ka11081,
    Ka11081Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11082 (2026-10-12).
contract_test!(
    mock_ka11082,
    ka11082,
    Ka11082Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11083 (2026-10-12).
contract_test!(
    mock_ka11083,
    ka11083,
    Ka11083Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig",
                "trde_tern_rt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka11094 (2026-10-12).
contract_test!(
    mock_ka11094,
    ka11094,
    Ka11094Request,
    "/api/dostk/chart",
    json!({"stk_cd": "039490", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &["stk_cd"],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "trde_prica",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20004 (2026-10-12).
contract_test!(
    mock_ka20004,
    ka20004,
    Ka20004Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "tic_scope": "1"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_tic_chart_qry",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "pred_pre",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20005 (2026-10-12).
contract_test!(
    mock_ka20005,
    ka20005,
    Ka20005Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "tic_scope": "1", "base_dt": "20261012"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_min_pole_qry",
            &[
                "cur_prc",
                "trde_qty",
                "cntr_tm",
                "open_pric",
                "high_pric",
                "low_pric",
                "acc_trde_qty",
                "pred_pre",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20006 (2026-10-12).
contract_test!(
    mock_ka20006,
    ka20006,
    Ka20006Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "base_dt": "20261012"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_dt_pole_qry",
            &[
                "cur_prc",
                "trde_qty",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "trde_prica"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20007 (2026-10-12).
contract_test!(
    mock_ka20007,
    ka20007,
    Ka20007Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "base_dt": "20261012"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_stk_pole_qry",
            &[
                "cur_prc",
                "trde_qty",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "trde_prica"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20008 (2026-10-12).
contract_test!(
    mock_ka20008,
    ka20008,
    Ka20008Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "base_dt": "20261012"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_mth_pole_qry",
            &[
                "cur_prc",
                "trde_qty",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "trde_prica"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka20019 (2026-10-12).
contract_test!(
    mock_ka20019,
    ka20019,
    Ka20019Request,
    "/api/dostk/chart",
    json!({"inds_cd": "001", "base_dt": "20261012"}),
    fixture(
        &["inds_cd"],
        Some((
            "inds_yr_pole_qry",
            &[
                "cur_prc",
                "trde_qty",
                "dt",
                "open_pric",
                "high_pric",
                "low_pric",
                "trde_prica"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50079 (2026-10-12).
contract_test!(
    mock_ka50079,
    ka50079,
    Ka50079Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "tic_scope": "1", "upd_stkpc_tp": "0"}),
    fixture(
        &[],
        Some((
            "gds_tic_chart_qry",
            &[
                "cur_prc",
                "pred_pre",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "dt",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50080 (2026-10-12).
contract_test!(
    mock_ka50080,
    ka50080,
    Ka50080Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "tic_scope": "1", "upd_stkpc_tp": "0"}),
    fixture(
        &[],
        Some((
            "gds_min_chart_qry",
            &[
                "cur_prc",
                "pred_pre",
                "acc_trde_qty",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "dt",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50081 (2026-10-12).
contract_test!(
    mock_ka50081,
    ka50081,
    Ka50081Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &[],
        Some((
            "gds_day_chart_qry",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50082 (2026-10-12).
contract_test!(
    mock_ka50082,
    ka50082,
    Ka50082Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &[],
        Some((
            "gds_week_chart_qry",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50083 (2026-10-12).
contract_test!(
    mock_ka50083,
    ka50083,
    Ka50083Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "base_dt": "20261012", "upd_stkpc_tp": "0"}),
    fixture(
        &[],
        Some((
            "gds_month_chart_qry",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50091 (2026-10-12).
contract_test!(
    mock_ka50091,
    ka50091,
    Ka50091Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "tic_scope": "1"}),
    fixture(
        &[],
        Some((
            "gds_tic_chart_qry",
            &[
                "cntr_pric",
                "pred_pre",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "dt",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/07/ka50092 (2026-10-12).
contract_test!(
    mock_ka50092,
    ka50092,
    Ka50092Request,
    "/api/dostk/chart",
    json!({"stk_cd": "M04020000", "tic_scope": "1"}),
    fixture(
        &[],
        Some((
            "gds_min_chart_qry",
            &[
                "cntr_pric",
                "pred_pre",
                "acc_trde_qty",
                "acc_trde_prica",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "dt",
                "pred_pre_sig"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/34/usa23000 (2026-10-12).
contract_test!(
    mock_usa23000,
    usa23000,
    Usa23000Request,
    "/api/us/sect",
    json!({"stex_tp": "0", "inds_cd": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "inds_cd", "inds_nm", "perf_1d", "perf_5d", "perf_1m", "perf_3m", "perf_6m",
                "perf_ytd", "perf_1y"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/34/usa23100 (2026-10-12).
contract_test!(
    mock_usa23100,
    usa23100,
    Usa23100Request,
    "/api/us/sect",
    json!({"stex_tp": "0", "sort_tp": "1", "inds_cd": "000"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "stex_tp",
                "stk_cd",
                "stk_nm",
                "stk_enm",
                "cur_prc",
                "pred_pre_sig",
                "pred_pre",
                "flu_rt",
                "acc_trde_qty",
                "sel_bid",
                "buy_bid",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06010 (2026-10-12).
contract_test!(
    mock_usa06010,
    usa06010,
    Usa06010Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "tic_scope": "1", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "bus_dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06011 (2026-10-12).
contract_test!(
    mock_usa06011,
    usa06011,
    Usa06011Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "tic_scope": "1", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "trde_qty",
                "open_pric",
                "high_pric",
                "low_pric",
                "cntr_tm",
                "bus_dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06012 (2026-10-12).
contract_test!(
    mock_usa06012,
    usa06012,
    Usa06012Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "pred_pre",
                "flu_rt",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06013 (2026-10-12).
contract_test!(
    mock_usa06013,
    usa06013,
    Usa06013Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06014 (2026-10-12).
contract_test!(
    mock_usa06014,
    usa06014,
    Usa06014Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06015 (2026-10-12).
contract_test!(
    mock_usa06015,
    usa06015,
    Usa06015Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/36/usa06016 (2026-10-12).
contract_test!(
    mock_usa06016,
    usa06016,
    Usa06016Request,
    "/api/us/chart",
    json!({"stex_tp": "NA", "stk_cd": "FIXTURE", "strt_dt": "20261012", "upd_stkpc_tp": "0", "exrt_appl_tp": "0"}),
    fixture(
        &[],
        Some((
            "result_list",
            &[
                "cur_prc",
                "acc_trde_qty",
                "acc_trde_prica",
                "open_pric",
                "high_pric",
                "low_pric",
                "dt",
                "upd_stkpc_tp",
                "upd_rt"
            ][..]
        ))
    )
);

// 잘못된 차트 입력은 전송 전에 거부한다.
#[tokio::test]
async fn invalid_chart_inputs_never_reach_transport() {
    let mock = Mock {
        seen: Arc::new(Mutex::new(vec![])),
        body: json!({"return_code":0}),
    };
    let seen = mock.seen.clone();
    let client = Client::new(mock, options()).unwrap();
    let token = Secret::new("offline-only".into());
    for request in [
        Ka10081Request {
            stk_cd: "".into(),
            base_dt: "20261012".into(),
            upd_stkpc_tp: "0".into(),
        },
        Ka10081Request {
            stk_cd: "X".repeat(21),
            base_dt: "20261012".into(),
            upd_stkpc_tp: "0".into(),
        },
        Ka10081Request {
            stk_cd: "039490".into(),
            base_dt: "20260230".into(),
            upd_stkpc_tp: "0".into(),
        },
        Ka10081Request {
            stk_cd: "039490".into(),
            base_dt: "20261012".into(),
            upd_stkpc_tp: "2".into(),
        },
    ] {
        assert!(client.ka10081(&token, &request).await.is_err());
    }
    let invalid = Usa06011Request {
        stex_tp: "ND".into(),
        stk_cd: None,
        strt_dt: None,
        tic_scope: Some("45".into()),
        upd_stkpc_tp: None,
        exrt_appl_tp: None,
    };
    assert!(client.usa06011(&token, &invalid).await.is_err());
    assert!(seen.lock().unwrap().is_empty());
}

// 선택 요청 필드는 생략하고 응답의 빈 문자열은 보존한다.
#[test]
fn optional_fields_are_omitted_and_response_empty_strings_are_preserved() {
    let request = Usa06011Request {
        stex_tp: "ND".into(),
        stk_cd: None,
        strt_dt: None,
        tic_scope: None,
        upd_stkpc_tp: None,
        exrt_appl_tp: None,
    };
    request.validate().unwrap();
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"stex_tp":"ND"})
    );
    let response: Ka10081Response = serde_json::from_value(
        json!({"stk_dt_pole_chart_qry":[{"cur_prc":"", "trde_qty":null,"new_field":1}]}),
    )
    .unwrap();
    let row = &response.stk_dt_pole_chart_qry.as_ref().unwrap()[0];
    assert_eq!(row.cur_prc.as_deref(), Some(""));
    assert!(row.trde_qty.is_none());
    assert!(row.dt.is_none());
    assert_eq!(row.extra["new_field"], 1);
}

// 주간 업종의 상충하는 길이 표기보다 명시된 날짜 설명을 따른다.
#[test]
fn contradictory_weekly_sector_length_uses_explicit_documented_date_description() {
    let request = Ka20007Request {
        inds_cd: "001".into(),
        base_dt: "20261012".into(),
    };
    request.validate().unwrap();
    let invalid = Ka20007Request {
        inds_cd: "001".into(),
        base_dt: "202".into(),
    };
    assert!(invalid.validate().is_err());
}
