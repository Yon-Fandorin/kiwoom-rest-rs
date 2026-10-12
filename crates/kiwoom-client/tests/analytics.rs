//! 이 구현을 위해 직접 만든 오프라인 합성 데이터이며, 실제 계좌 내역이 아니다.

use async_trait::async_trait;
use kiwoom_client::{analytics::*, transport::*};
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
        let id = request.api_id.to_owned();
        self.seen.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::from([("api-id".into(), id), ("cont-yn".into(), "N".into())]),
            body: self.body.clone(),
        })
    }
}

fn setup(body: Value) -> (Mock, Client<Mock>) {
    let mock = Mock {
        seen: Arc::default(),
        body,
    };
    let client = Client::new(
        mock.clone(),
        ClientOptions {
            min_request_interval: Duration::ZERO,
            ..Default::default()
        },
    )
    .unwrap();
    (mock, client)
}

macro_rules! contract {
    ($name:ident, $req:ident, $res:ident, $method:ident, $path:literal, $body:expr, $bad:expr, $list:ident, $first:ident, $first_wire:literal, $second:ident, $second_wire:literal) => {
        // 요청 왕복, 문자열·미지 필드 보존, 전송 경로와 잘못된 입력의 거부를 검증한다.
        #[tokio::test]
        async fn $name() {
            let body = $body;
            let request: $req = serde_json::from_value(body.clone()).unwrap();
            assert_eq!(serde_json::to_value(&request).unwrap(), body);

            let fixture = json!({"return_code":0,"return_msg":"offline fixture",stringify!($list):[{$first_wire:"+000123.4500",$second_wire:"","future_field":{"source":"synthetic"}}],"future_top":[1,2]});
            let (mock, client) = setup(fixture.clone());
            let pages = client
                .$method(&Secret::new("offline-analytics-token".into()), &request)
                .await
                .unwrap();
            assert_eq!(pages.len(), 1);
            let row = &pages[0].typed.$list.as_ref().unwrap()[0];
            assert_eq!(row.$first.as_deref(), Some("+000123.4500"));
            assert_eq!(row.$second.as_deref(), Some(""));
            assert_eq!(row.extra["future_field"], json!({"source":"synthetic"}));
            assert_eq!(pages[0].typed.extra["future_top"], json!([1, 2]));
            assert_eq!(pages[0].raw, fixture);

            {
                let seen = mock.seen.lock().unwrap();
                assert_eq!(seen.len(), 1);
                assert_eq!(seen[0].api_id, stringify!($method));
                assert_eq!(seen[0].path, $path);
                assert_eq!(seen[0].body, body);
                assert_eq!(
                    pages[0].response_headers["api-id"],
                    stringify!($method)
                );
            }

            let absent: $res = serde_json::from_value(json!({"return_code":0})).unwrap();
            assert!(absent.$list.is_none());
            let empty: $res =
                serde_json::from_value(json!({"return_code":0,stringify!($list):[]})).unwrap();
            assert!(empty.$list.unwrap().is_empty());
            // 문자열 필드에 JSON 숫자를 넣으면 암묵적으로 변환하지 않고 거부한다.
            assert!(
                serde_json::from_value::<$res>(
                    json!({stringify!($list):[{$first_wire:123}]})
                )
                .is_err()
            );
            let invalid: Result<$req, _> = serde_json::from_value($bad);
            if let Ok(invalid) = invalid {
                let (mock, client) = setup(json!({"return_code":0}));
                assert!(matches!(
                    client
                        .$method(&Secret::new("offline".into()), &invalid)
                        .await,
                    Err(ClientError::InvalidRequest(_))
                ));
                assert!(mock.seen.lock().unwrap().is_empty());
            }
        }
    }
}

contract!(
    ka10014_contract,
    Ka10014Request,
    Ka10014Response,
    ka10014,
    "/api/dostk/shsa",
    json!({"stk_cd":"000001","strt_dt":"20260101","end_dt":"20260131","tm_tp":"1"}),
    json!({"stk_cd":"000001","strt_dt":"20260230","end_dt":"20260101"}),
    shrts_trnsn,
    close_pric,
    "close_pric",
    shrts_qty,
    "shrts_qty"
);

contract!(
    ka01300_contract,
    Ka01300Request,
    Ka01300Response,
    ka01300,
    "/api/dostk/watchlist",
    json!({}),
    Value::Null,
    nofi,
    gcod,
    "gcod",
    name,
    "name"
);

contract!(
    ka01301_contract,
    Ka01301Request,
    Ka01301Response,
    ka01301,
    "/api/dostk/watchlist",
    json!({"arn_grp_id":"001"}),
    json!({"arn_grp_id":"1234"}),
    nofj,
    cod2,
    "cod2",
    bgb_clr,
    "bgb_clr"
);

contract!(
    ka10008_contract,
    Ka10008Request,
    Ka10008Response,
    ka10008,
    "/api/dostk/frgnistt",
    json!({"stk_cd":"000001_NX"}),
    json!({"stk_cd":""}),
    stk_frgnr,
    close_pric,
    "close_pric",
    chg_qty,
    "chg_qty"
);

contract!(
    ka10131_contract,
    Ka10131Request,
    Ka10131Response,
    ka10131,
    "/api/dostk/frgnistt",
    json!({"dt":"5","mrkt_tp":"001","netslmt_tp":"2","stk_inds_tp":"0","amt_qty_tp":"1","stex_tp":"3"}),
    json!({"dt":"5","mrkt_tp":"001","netslmt_tp":"1","stk_inds_tp":"0","amt_qty_tp":"1","stex_tp":"3"}),
    orgn_frgnr_cont_trde_prst,
    orgn_nettrde_qty,
    "orgn_nettrde_qty",
    frgnr_nettrde_amt,
    "frgnr_nettrde_amt"
);

contract!(
    ka52301_contract,
    Ka52301Request,
    Ka52301Response,
    ka52301,
    "/api/dostk/frgnistt",
    json!({}),
    Value::Null,
    inve_trad_stat,
    all_dfrt_trst_netprps_amt,
    "all_dfrt_trst_netprps_amt",
    sell_uv,
    "sell_uv"
);

contract!(
    ka10068_contract,
    Ka10068Request,
    Ka10068Response,
    ka10068,
    "/api/dostk/slb",
    json!({"all_tp":"1","strt_dt":"20260101","end_dt":"20260131"}),
    json!({"all_tp":"0"}),
    dbrt_trde_trnsn,
    dbrt_trde_irds,
    "dbrt_trde_irds",
    remn_amt,
    "remn_amt"
);

contract!(
    ka10069_contract,
    Ka10069Request,
    Ka10069Response,
    ka10069,
    "/api/dostk/slb",
    json!({"strt_dt":"20260101","mrkt_tp":"001"}),
    json!({"strt_dt":"20260230","mrkt_tp":"001"}),
    dbrt_trde_upper_10stk,
    dbrt_trde_cntrcnt,
    "dbrt_trde_cntrcnt",
    stk_nm,
    "stk_nm"
);

contract!(
    ka20068_contract,
    Ka20068Request,
    Ka20068Response,
    ka20068,
    "/api/dostk/slb",
    json!({"stk_cd":"000001","all_tp":"0"}),
    json!({"stk_cd":"000001","all_tp":"1"}),
    dbrt_trde_trnsn,
    dbrt_trde_rpy,
    "dbrt_trde_rpy",
    dt,
    "dt"
);

contract!(
    ka90012_contract,
    Ka90012Request,
    Ka90012Response,
    ka90012,
    "/api/dostk/slb",
    json!({"dt":"20260101","mrkt_tp":"101"}),
    json!({"dt":"20260101","mrkt_tp":"000"}),
    dbrt_trde_prps,
    rmnd,
    "rmnd",
    remn_amt,
    "remn_amt"
);

contract!(
    usa20200_contract,
    Usa20200Request,
    Usa20200Response,
    usa20200,
    "/api/us/watchlist",
    json!({}),
    Value::Null,
    nofi,
    gcod,
    "gcod",
    name,
    "name"
);

contract!(
    usa20201_contract,
    Usa20201Request,
    Usa20201Response,
    usa20201,
    "/api/us/watchlist",
    json!({"arn_grp_id":"003"}),
    json!({"arn_grp_id":"1234"}),
    nofj,
    cod2,
    "cod2",
    stex_tp,
    "stex_tp"
);

contract!(
    ka00190_contract,
    Ka00190Request,
    Ka00190Response,
    ka00190,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "sort_tp": "1", "case_pric_tp": "10", "trde_prica_tp": "0", "stk_tp": "0", "trde_qty_tp": "0", "crd_tp": "0", "pric_tp": "0", "mac_tp": "0", "stex_tp": "3"}),
    json!({"mrkt_tp": "INVALID", "sort_tp": "1", "case_pric_tp": "10", "trde_prica_tp": "0", "stk_tp": "0", "trde_qty_tp": "0", "crd_tp": "0", "pric_tp": "0", "mac_tp": "0", "stex_tp": "3"}),
    result_list,
    cur_prc,
    "cur_prc",
    pre,
    "pre"
);

contract!(
    ka00196_contract,
    Ka00196Request,
    Ka00196Response,
    ka00196,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001"}),
    json!({"stk_cd": ""}),
    result_list,
    tot_avg_pric,
    "tot_avg_pric",
    mont_cntr_amt,
    "mont_cntr_amt"
);

contract!(
    ka04196_contract,
    Ka04196Request,
    Ka04196Response,
    ka04196,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001", "st_dt": "20260101", "ed_dt": "20260131", "qry_tp": "0", "stex_tp": "1"}),
    json!({"stk_cd": "000001", "st_dt": "20260101", "ed_dt": "20260131", "qry_tp": "INVALID", "stex_tp": "1"}),
    result_list,
    prft_rt,
    "prft_rt",
    netprps_amt,
    "netprps_amt"
);

contract!(
    ka10020_contract,
    Ka10020Request,
    Ka10020Response,
    ka10020,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "001", "sort_tp": "1", "trde_qty_tp": "00100", "stk_cnd": "0", "crd_cnd": "0", "stex_tp": "3"}),
    json!({"mrkt_tp": "001", "sort_tp": "INVALID", "trde_qty_tp": "00100", "stk_cnd": "0", "crd_cnd": "0", "stex_tp": "3"}),
    bid_req_upper,
    netprps_req,
    "netprps_req",
    buy_rt,
    "buy_rt"
);

contract!(
    ka10021_contract,
    Ka10021Request,
    Ka10021Response,
    ka10021,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "001", "trde_tp": "1", "sort_tp": "1", "tm_tp": "10", "trde_qty_tp": "100", "stk_cnd": "0", "stex_tp": "1"}),
    json!({"mrkt_tp": "001", "trde_tp": "1", "sort_tp": "INVALID", "tm_tp": "10", "trde_qty_tp": "100", "stk_cnd": "0", "stex_tp": "1"}),
    bid_req_sdnin,
    int,
    "int",
    sdnin_qty,
    "sdnin_qty"
);

contract!(
    ka10022_contract,
    Ka10022Request,
    Ka10022Response,
    ka10022,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "001", "rt_tp": "1", "tm_tp": "10", "trde_qty_tp": "100", "stk_cnd": "0", "stex_tp": "1"}),
    json!({"mrkt_tp": "001", "rt_tp": "INVALID", "tm_tp": "10", "trde_qty_tp": "100", "stk_cnd": "0", "stex_tp": "1"}),
    req_rt_sdnin,
    now_rt,
    "now_rt",
    sdnin_rt,
    "sdnin_rt"
);

contract!(
    ka10023_contract,
    Ka10023Request,
    Ka10023Response,
    ka10023,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "sort_tp": "1", "tm_tp": "1", "trde_qty_tp": "1000", "tm": "10", "stk_cnd": "20", "pric_tp": "0", "stex_tp": "3"}),
    json!({"mrkt_tp": "000", "sort_tp": "INVALID", "tm_tp": "1", "trde_qty_tp": "1000", "tm": "10", "stk_cnd": "20", "pric_tp": "0", "stex_tp": "3"}),
    trde_qty_sdnin,
    sdnin_qty,
    "sdnin_qty",
    flu_rt,
    "flu_rt"
);

contract!(
    ka10027_contract,
    Ka10027Request,
    Ka10027Response,
    ka10027,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "sort_tp": "1", "trde_qty_cnd": "0100", "stk_cnd": "16", "crd_cnd": "0", "updown_incls": "1", "pric_cnd": "10", "trde_prica_cnd": "100", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "sort_tp": "INVALID", "trde_qty_cnd": "0100", "stk_cnd": "16", "crd_cnd": "0", "updown_incls": "1", "pric_cnd": "10", "trde_prica_cnd": "100", "stex_tp": "1"}),
    pred_pre_flu_rt_upper,
    cntr_str,
    "cntr_str",
    cnt,
    "cnt"
);

contract!(
    ka10029_contract,
    Ka10029Request,
    Ka10029Response,
    ka10029,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "sort_tp": "8", "trde_qty_cnd": "100", "stk_cnd": "16", "crd_cnd": "5", "pric_cnd": "10", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "sort_tp": "INVALID", "trde_qty_cnd": "100", "stk_cnd": "16", "crd_cnd": "5", "pric_cnd": "10", "stex_tp": "1"}),
    exp_cntr_flu_rt_upper,
    exp_cntr_pric,
    "exp_cntr_pric",
    base_pric,
    "base_pric"
);

contract!(
    ka10030_contract,
    Ka10030Request,
    Ka10030Response,
    ka10030,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "sort_tp": "3", "mang_stk_incls": "16", "crd_tp": "8", "trde_qty_tp": "1000", "pric_tp": "10", "trde_prica_tp": "5000", "mrkt_open_tp": "0", "stex_tp": "3"}),
    json!({"mrkt_tp": "000", "sort_tp": "INVALID", "mang_stk_incls": "16", "crd_tp": "8", "trde_qty_tp": "1000", "pric_tp": "10", "trde_prica_tp": "5000", "mrkt_open_tp": "0", "stex_tp": "3"}),
    tdy_trde_qty_upper,
    bf_mkrt_trde_amt,
    "bf_mkrt_trde_amt",
    opmr_trde_qty,
    "opmr_trde_qty"
);

contract!(
    ka10031_contract,
    Ka10031Request,
    Ka10031Response,
    ka10031,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "001", "qry_tp": "1", "rank_strt": "0", "rank_end": "100", "stex_tp": "1"}),
    json!({"mrkt_tp": "001", "qry_tp": "1", "rank_strt": "INVALID", "rank_end": "100", "stex_tp": "1"}),
    pred_trde_qty_upper,
    cur_prc,
    "cur_prc",
    pred_pre,
    "pred_pre"
);

contract!(
    ka10032_contract,
    Ka10032Request,
    Ka10032Response,
    ka10032,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "mang_stk_incls": "0", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "mang_stk_incls": "INVALID", "stex_tp": "1"}),
    trde_prica_upper,
    now_rank,
    "now_rank",
    pred_rank,
    "pred_rank"
);

contract!(
    ka10033_contract,
    Ka10033Request,
    Ka10033Response,
    ka10033,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "trde_qty_tp": "1000", "stk_cnd": "0", "updown_incls": "1", "crd_cnd": "0", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "trde_qty_tp": "1000", "stk_cnd": "0", "updown_incls": "1", "crd_cnd": "INVALID", "stex_tp": "1"}),
    crd_rt_upper,
    crd_rt,
    "crd_rt",
    stk_infr,
    "stk_infr"
);

contract!(
    ka10034_contract,
    Ka10034Request,
    Ka10034Response,
    ka10034,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "trde_tp": "3", "dt": "60", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "trde_tp": "3", "dt": "INVALID", "stex_tp": "1"}),
    for_dt_trde_upper,
    netprps_qty,
    "netprps_qty",
    gain_pos_stkcnt,
    "gain_pos_stkcnt"
);

contract!(
    ka10035_contract,
    Ka10035Request,
    Ka10035Response,
    ka10035,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "trde_tp": "2", "base_dt_tp": "1", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "trde_tp": "INVALID", "base_dt_tp": "1", "stex_tp": "1"}),
    for_cont_nettrde_upper,
    dm1,
    "dm1",
    tot,
    "tot"
);

contract!(
    ka10036_contract,
    Ka10036Request,
    Ka10036Response,
    ka10036,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "dt": "20", "stex_tp": "3"}),
    json!({"mrkt_tp": "000", "dt": "INVALID", "stex_tp": "3"}),
    for_limit_exh_rt_incrs_upper,
    base_limit_exh_rt,
    "base_limit_exh_rt",
    exh_rt_incrs,
    "exh_rt_incrs"
);

contract!(
    ka10037_contract,
    Ka10037Request,
    Ka10037Response,
    ka10037,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "dt": "60", "trde_tp": "4", "sort_tp": "2", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "dt": "60", "trde_tp": "4", "sort_tp": "INVALID", "stex_tp": "1"}),
    frgn_wicket_trde_upper,
    netprps_prica,
    "netprps_prica",
    trde_prica,
    "trde_prica"
);

contract!(
    ka10038_contract,
    Ka10038Request,
    Ka10038Response,
    ka10038,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001", "qry_tp": "2", "strt_dt": "20260101", "end_dt": "20260131", "dt": ""}),
    json!({"stk_cd": "000001", "qry_tp": "INVALID", "strt_dt": "20260101", "end_dt": "20260131", "dt": ""}),
    stk_sec_rank,
    acc_netprps_qty,
    "acc_netprps_qty",
    mmcm_nm,
    "mmcm_nm"
);

contract!(
    ka10039_contract,
    Ka10039Request,
    Ka10039Response,
    ka10039,
    "/api/dostk/rkinfo",
    json!({"mmcm_cd": "001", "trde_qty_tp": "1000", "trde_tp": "2", "dt": "60", "stex_tp": "1"}),
    json!({"mmcm_cd": "001", "trde_qty_tp": "1000", "trde_tp": "2", "dt": "INVALID", "stex_tp": "1"}),
    sec_trde_upper,
    prid_stkpc_flu,
    "prid_stkpc_flu",
    netprps_amt,
    "netprps_amt"
);

contract!(
    ka10040_contract,
    Ka10040Request,
    Ka10040Response,
    ka10040,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001"}),
    json!({"stk_cd": "INVALID"}),
    tdy_main_trde_ori,
    sell_qty,
    "sell_qty",
    qry_dt,
    "qry_dt"
);

contract!(
    ka10042_contract,
    Ka10042Request,
    Ka10042Response,
    ka10042,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001", "qry_dt_tp": "0", "pot_tp": "1", "dt": "120", "sort_base": "2"}),
    json!({"stk_cd": "000001", "qry_dt_tp": "0", "pot_tp": "1", "dt": "120", "sort_base": "INVALID"}),
    netprps_trde_ori_rank,
    rank,
    "rank",
    mmcm_cd,
    "mmcm_cd"
);

contract!(
    ka10053_contract,
    Ka10053Request,
    Ka10053Response,
    ka10053,
    "/api/dostk/rkinfo",
    json!({"stk_cd": "000001"}),
    json!({"stk_cd": "INVALID"}),
    tdy_upper_scesn_ori,
    sell_qty,
    "sell_qty",
    qry_tm,
    "qry_tm"
);

contract!(
    ka10062_contract,
    Ka10062Request,
    Ka10062Response,
    ka10062,
    "/api/dostk/rkinfo",
    json!({"strt_dt": "20260101", "end_dt": "20260131", "mrkt_tp": "000", "trde_tp": "2", "sort_cnd": "1", "unit_tp": "1000", "stex_tp": "1"}),
    json!({"strt_dt": "20260101", "end_dt": "20260131", "mrkt_tp": "000", "trde_tp": "2", "sort_cnd": "1", "unit_tp": "INVALID", "stex_tp": "1"}),
    eql_nettrde_rank,
    orgn_nettrde_avg_pric,
    "orgn_nettrde_avg_pric",
    for_nettrde_amt,
    "for_nettrde_amt"
);

contract!(
    ka10065_contract,
    Ka10065Request,
    Ka10065Response,
    ka10065,
    "/api/dostk/rkinfo",
    json!({"trde_tp": "1", "mrkt_tp": "001", "orgn_tp": "9999", "amt_qty_tp": "2"}),
    json!({"trde_tp": "1", "mrkt_tp": "001", "orgn_tp": "INVALID", "amt_qty_tp": "2"}),
    opmr_invsr_trde_upper,
    netslmt,
    "netslmt",
    sel_qty,
    "sel_qty"
);

contract!(
    ka90009_contract,
    Ka90009Request,
    Ka90009Response,
    ka90009,
    "/api/dostk/rkinfo",
    json!({"mrkt_tp": "000", "amt_qty_tp": "1", "qry_dt_tp": "1", "date": "20260101", "stex_tp": "1"}),
    json!({"mrkt_tp": "000", "amt_qty_tp": "1", "qry_dt_tp": "1", "date": "INVALID", "stex_tp": "1"}),
    frgnr_orgn_trde_upper,
    orgn_netprps_amt,
    "orgn_netprps_amt",
    for_netslmt_qty,
    "for_netslmt_qty"
);

contract!(
    usa01980_contract,
    Usa01980Request,
    Usa01980Response,
    usa01980,
    "/api/us/rkinfo",
    json!({"svc_type": "B286"}),
    json!({"svc_type": "INVALID"}),
    result_list,
    curr_pric,
    "curr_pric",
    diff_rate_for_prev,
    "diff_rate_for_prev"
);

contract!(
    usa01990_contract,
    Usa01990Request,
    Usa01990Response,
    usa01990,
    "/api/us/rkinfo",
    json!({"dt_unit_tp": "D", "stk_tp": "A"}),
    json!({"dt_unit_tp": "INVALID", "stk_tp": "A"}),
    result_list,
    rank_flu_sig,
    "rank_flu_sig",
    stk_enm,
    "stk_enm"
);

contract!(
    usa20510_contract,
    Usa20510Request,
    Usa20510Response,
    usa20510,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "5"}),
    json!({"stex_tp": "INVALID", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "5"}),
    result_list,
    stdt_base_pric,
    "stdt_base_pric",
    endt_trde_qty,
    "endt_trde_qty"
);

contract!(
    usa20511_contract,
    Usa20511Request,
    Usa20511Response,
    usa20511,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "10"}),
    json!({"stex_tp": "INVALID", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "10"}),
    result_list,
    endt_base_pric,
    "endt_base_pric",
    stdt_trde_qty,
    "stdt_trde_qty"
);

contract!(
    usa20512_contract,
    Usa20512Request,
    Usa20512Response,
    usa20512,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "30"}),
    json!({"stex_tp": "INVALID", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "tm": "30"}),
    result_list,
    stdt_base_pric,
    "stdt_base_pric",
    endt_base_pric,
    "endt_base_pric"
);

contract!(
    usa20530_contract,
    Usa20530Request,
    Usa20530Response,
    usa20530,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "qry_tp": "2"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "qry_tp": "INVALID"}),
    result_list,
    trde_prica,
    "trde_prica",
    pred_rt,
    "pred_rt"
);

contract!(
    usa20531_contract,
    Usa20531Request,
    Usa20531Response,
    usa20531,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "qry_tp": "0"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "qry_tp": "INVALID"}),
    result_list,
    acc_trde_qty,
    "acc_trde_qty",
    cur_prc,
    "cur_prc"
);

contract!(
    usa20540_contract,
    Usa20540Request,
    Usa20540Response,
    usa20540,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    pred_trde_qty,
    "pred_trde_qty",
    trde_prica,
    "trde_prica"
);

contract!(
    usa20541_contract,
    Usa20541Request,
    Usa20541Response,
    usa20541,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    cur_prc,
    "cur_prc",
    pred_pre,
    "pred_pre"
);

contract!(
    usa20550_contract,
    Usa20550Request,
    Usa20550Response,
    usa20550,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    mac,
    "mac",
    mac_wght,
    "mac_wght"
);

contract!(
    usa20551_contract,
    Usa20551Request,
    Usa20551Response,
    usa20551,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    trde_pre_rt,
    "trde_pre_rt",
    mgn_type,
    "mgn_type"
);

contract!(
    usa20880_contract,
    Usa20880Request,
    Usa20880Response,
    usa20880,
    "/api/us/rkinfo",
    json!({"qry_tp": "6", "dt_unit_tp": "5"}),
    json!({"qry_tp": "INVALID", "dt_unit_tp": "5"}),
    result_list,
    kw_high_rank,
    "kw_high_rank",
    kw_high_rank_sig,
    "kw_high_rank_sig"
);

contract!(
    usa20881_contract,
    Usa20881Request,
    Usa20881Response,
    usa20881,
    "/api/us/rkinfo",
    json!({"qry_tp": "7", "dt_unit_tp": "8"}),
    json!({"qry_tp": "7", "dt_unit_tp": "INVALID"}),
    result_list,
    kw_high_rank_hl,
    "kw_high_rank_hl",
    flu_rt,
    "flu_rt"
);

contract!(
    usa20910_contract,
    Usa20910Request,
    Usa20910Response,
    usa20910,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "inds_cls_tp": "3", "sort_tp": "5"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "inds_cls_tp": "3", "sort_tp": "INVALID"}),
    result_list,
    cnt,
    "cnt",
    sel_req,
    "sel_req"
);

contract!(
    usa20911_contract,
    Usa20911Request,
    Usa20911Response,
    usa20911,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "1"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    buy_req,
    "buy_req",
    trde_qty,
    "trde_qty"
);

contract!(
    usa20920_contract,
    Usa20920Request,
    Usa20920Response,
    usa20920,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "2"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    open_pric_pre,
    "open_pric_pre",
    cntr_tm,
    "cntr_tm"
);

contract!(
    usa20921_contract,
    Usa20921Request,
    Usa20921Response,
    usa20921,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "1"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    high_pric,
    "high_pric",
    low_pric,
    "low_pric"
);

contract!(
    usa20922_contract,
    Usa20922Request,
    Usa20922Response,
    usa20922,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "2", "stk_tp": "2"}),
    json!({"stex_tp": "0", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID", "stk_tp": "2"}),
    result_list,
    open_pric,
    "open_pric",
    open_pric_pre,
    "open_pric_pre"
);

contract!(
    usa20940_contract,
    Usa20940Request,
    Usa20940Response,
    usa20940,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "0", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "4", "pric_cnd1": "-100", "pric_cnd2": "100", "base_dt": "20260101"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "0", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID", "pric_cnd1": "-100", "pric_cnd2": "100", "base_dt": "20260101"}),
    result_list,
    acc_flu_rt,
    "acc_flu_rt",
    stk_high_dt,
    "stk_high_dt"
);

contract!(
    usa20941_contract,
    Usa20941Request,
    Usa20941Response,
    usa20941,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "0", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "0", "pric_cnd1": "-100", "pric_cnd2": "100", "base_dt": "20260101"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "0", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID", "pric_cnd1": "-100", "pric_cnd2": "100", "base_dt": "20260101"}),
    result_list,
    acc_flu_amt,
    "acc_flu_amt",
    stk_low_dt,
    "stk_low_dt"
);

contract!(
    usa20960_contract,
    Usa20960Request,
    Usa20960Response,
    usa20960,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "qry_tp": "1"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "qry_tp": "INVALID"}),
    result_list,
    pred_rt,
    "pred_rt",
    acc_trde_qty,
    "acc_trde_qty"
);

contract!(
    usa20961_contract,
    Usa20961Request,
    Usa20961Response,
    usa20961,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "qry_tp": "0"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "qry_tp": "INVALID"}),
    result_list,
    cur_prc,
    "cur_prc",
    pred_pre,
    "pred_pre"
);

contract!(
    usa24110_contract,
    Usa24110Request,
    Usa24110Response,
    usa24110,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "0", "dt_tp": "1"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID", "dt_tp": "1"}),
    result_list,
    week52_hgst_pric,
    "52wk_hgst_pric",
    week52_lwst_pric,
    "52wk_lwst_pric"
);

contract!(
    usa24111_contract,
    Usa24111Request,
    Usa24111Response,
    usa24111,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "1", "dt_tp": "0"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID", "dt_tp": "0"}),
    result_list,
    hl_pre,
    "hl_pre",
    pre_flu_rt,
    "pre_flu_rt"
);

contract!(
    usa24120_contract,
    Usa24120Request,
    Usa24120Response,
    usa24120,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "base_dt": "20260101", "sort_tp": "2"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "base_dt": "20260101", "sort_tp": "INVALID"}),
    result_list,
    open_pric,
    "open_pric",
    base_dt,
    "base_dt"
);

contract!(
    usa24121_contract,
    Usa24121Request,
    Usa24121Response,
    usa24121,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "base_dt": "20260101", "sort_tp": "0"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "base_dt": "20260101", "sort_tp": "INVALID"}),
    result_list,
    trde_prica,
    "trde_prica",
    flu_rt,
    "flu_rt"
);

contract!(
    usa24150_contract,
    Usa24150Request,
    Usa24150Response,
    usa24150,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    day2_avg_tern_rt,
    "2d_avg_tern_rt",
    day20_avg_tern_rt,
    "20d_avg_tern_rt"
);

contract!(
    usa24151_contract,
    Usa24151Request,
    Usa24151Response,
    usa24151,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    json!({"stex_tp": "INVALID", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50"}),
    result_list,
    day10_avg_tern_rt,
    "10d_avg_tern_rt",
    trde_tern_rt,
    "trde_tern_rt"
);

contract!(
    usa24160_contract,
    Usa24160Request,
    Usa24160Response,
    usa24160,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "5"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    conti_dt,
    "conti_dt",
    base_close_pric,
    "base_close_pric"
);

contract!(
    usa24161_contract,
    Usa24161Request,
    Usa24161Response,
    usa24161,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "3"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    base_pre_rt,
    "base_pre_rt",
    acc_trde_qty,
    "acc_trde_qty"
);

contract!(
    usa24162_contract,
    Usa24162Request,
    Usa24162Response,
    usa24162,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "2"}),
    json!({"stex_tp": "0", "stk_cd": [{"stex_tp": "ND", "stk_cd": "TEST"}], "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    conti_dt,
    "conti_dt",
    base_pre_rt,
    "base_pre_rt"
);

contract!(
    usa24200_contract,
    Usa24200Request,
    Usa24200Response,
    usa24200,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "4"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    buy_rt,
    "buy_rt",
    sel_rt,
    "sel_rt"
);

contract!(
    usa24201_contract,
    Usa24201Request,
    Usa24201Response,
    usa24201,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "3"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    tot_sel_req,
    "tot_sel_req",
    cntr_tm,
    "cntr_tm"
);

contract!(
    usa24290_contract,
    Usa24290Request,
    Usa24290Response,
    usa24290,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "inds_cls_tp": "3", "sort_tp": "4"}),
    json!({"stex_tp": "0", "inds_cd": "000", "stk_tp": "1", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "inds_cls_tp": "3", "sort_tp": "INVALID"}),
    result_list,
    dispty_rt,
    "dispty_rt",
    cntr_str,
    "cntr_str"
);

contract!(
    usa24291_contract,
    Usa24291Request,
    Usa24291Response,
    usa24291,
    "/api/us/rkinfo",
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "2"}),
    json!({"stex_tp": "0", "etf_cat1": "10", "etf_cat2": "1010", "trde_qty_tp": "10", "stk_cnd": "2", "pric_cnd": "10", "trde_prica_cnd": "50", "sort_tp": "INVALID"}),
    result_list,
    sel_bid,
    "sel_bid",
    buy_req,
    "buy_req"
);

// 응답의 스칼라 필드와 배율 문자열을 별도 해석 없이 보존한다.
#[test]
fn response_scalar_fields_and_scaled_strings_remain_uninterpreted() {
    let lending: Ka10069Response=serde_json::from_value(json!({"dbrt_trde_cntrcnt_sum":"000010","dbrt_trde_rpy_sum":"000020","rmnd_sum":"000030","remn_amt_sum":"000040","dbrt_trde_cntrcnt_rt":"2658","dbrt_trde_rpy_rt":"-000123","rmnd_rt":"","remn_amt_rt":"0"})).unwrap();
    assert_eq!(lending.dbrt_trde_cntrcnt_sum.as_deref(), Some("000010"));
    assert_eq!(lending.dbrt_trde_rpy_sum.as_deref(), Some("000020"));
    assert_eq!(lending.rmnd_sum.as_deref(), Some("000030"));
    assert_eq!(lending.remn_amt_sum.as_deref(), Some("000040"));
    assert_eq!(lending.dbrt_trde_cntrcnt_rt.as_deref(), Some("2658"));
    assert_eq!(lending.dbrt_trde_rpy_rt.as_deref(), Some("-000123"));
    assert_eq!(lending.rmnd_rt.as_deref(), Some(""));
    assert_eq!(lending.remn_amt_rt.as_deref(), Some("0"));
    let groups: Ka01300Response = serde_json::from_value(json!({"rtcd":"F"})).unwrap();
    assert_eq!(groups.rtcd.as_deref(), Some("F"));
    let groups: Usa20200Response = serde_json::from_value(json!({"rtcd":"S"})).unwrap();
    assert_eq!(groups.rtcd.as_deref(), Some("S"));
    let rank: Ka10038Response = serde_json::from_value(
        json!({"rank_1":"+001","rank_2":"-002","rank_3":"-001","prid_trde_qty":"0003"}),
    )
    .unwrap();
    assert_eq!(rank.rank_1.as_deref(), Some("+001"));
    assert_eq!(rank.rank_2.as_deref(), Some("-002"));
    assert_eq!(rank.rank_3.as_deref(), Some("-001"));
    assert_eq!(rank.prid_trde_qty.as_deref(), Some("0003"));
    let popular: Usa01980Response =
        serde_json::from_value(json!({"base_date":"20261012","base_time":"120000"})).unwrap();
    assert_eq!(popular.base_date.as_deref(), Some("20261012"));
    assert_eq!(popular.base_time.as_deref(), Some("120000"));
    let trading: Ka10040Response=serde_json::from_value(json!({"sel_trde_ori_irds_1":"-001","buy_trde_ori_qty_5":"0003","frgn_sel_prsm_sum_chang":"+002","frgn_buy_prsm_sum":""})).unwrap();
    assert_eq!(trading.sel_trde_ori_irds_1.as_deref(), Some("-001"));
    assert_eq!(trading.buy_trde_ori_qty_5.as_deref(), Some("0003"));
    assert_eq!(trading.frgn_sel_prsm_sum_chang.as_deref(), Some("+002"));
    assert_eq!(trading.frgn_buy_prsm_sum.as_deref(), Some(""));
}

// 조회 한도, 날짜, 중첩 종목과 거래소가 붙은 티커를 검증한다.
#[tokio::test]
async fn query_bounds_dates_nested_symbols_and_qualified_tickers_are_guarded() {
    let bad: Ka10031Request = serde_json::from_value(
        json!({"mrkt_tp":"001","qry_tp":"1","rank_strt":"101","rank_end":"100","stex_tp":"1"}),
    )
    .unwrap();
    assert!(bad.validate().is_err());
    let reversed: Ka10031Request = serde_json::from_value(
        json!({"mrkt_tp":"001","qry_tp":"1","rank_strt":"100","rank_end":"10","stex_tp":"1"}),
    )
    .unwrap();
    assert!(reversed.validate().is_err());
    let date_query:Ka10131Request=serde_json::from_value(json!({"dt":"0","mrkt_tp":"001","netslmt_tp":"2","stk_inds_tp":"0","amt_qty_tp":"0","stex_tp":"1"})).unwrap();
    assert!(date_query.validate().is_err());
    let nested: Usa20512Request =
        serde_json::from_value(json!({"stk_cd":[{"stex_tp":"ND","stk_cd":""}]})).unwrap();
    let (mock, client) = setup(json!({"return_code":0}));
    assert!(matches!(
        client
            .usa20512(&Secret::new("offline".into()), &nested)
            .await,
        Err(ClientError::InvalidRequest(_))
    ));
    assert!(mock.seen.lock().unwrap().is_empty());
    let ticker: Ka10040Request = serde_json::from_value(json!({"stk_cd":"000001_NX"})).unwrap();
    assert!(ticker.validate().is_ok());
    let ticker: Ka10040Request = serde_json::from_value(json!({"stk_cd":"000001_BAD"})).unwrap();
    assert!(ticker.validate().is_err());
    let generic_lending: Ka20068Request =
        serde_json::from_value(json!({"stk_cd":"000001_NX"})).unwrap();
    assert!(generic_lending.validate().is_err());
    let request: Ka04196Request =
        serde_json::from_value(json!({"st_dt":"20260131","ed_dt":"20260101"})).unwrap();
    assert!(request.validate().is_err());
}

// 예제의 오타를 표에 정의된 필드의 별칭으로 취급하지 않는다.
#[test]
fn table_contract_does_not_silently_alias_sample_typo() {
    // usa24162 예제의 result_lsit과 Body 표의 result_list가 다르므로 표의 계약을 검증한다.
    let response: Usa24162Response =
        serde_json::from_value(json!({"result_lsit":[{"stk_cd":"TEST"}]})).unwrap();
    assert!(response.result_list.is_none());
    assert_eq!(response.extra["result_lsit"][0]["stk_cd"], "TEST");
}
