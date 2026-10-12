//! 직접 만든 합성 데이터로 계약을 검증하며, 브로커 HTTP 요청이나 인증정보를 사용하지 않는다.

use async_trait::async_trait;
use kiwoom_client::transport::{HttpRequest, HttpResponse, TransportError};
use kiwoom_client::{Client, ClientOptions, Secret, Transport, accounts::*};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone)]
struct Mock {
    request: Arc<Mutex<Vec<HttpRequest>>>,
    response: Value,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.request.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::new(),
            body: self.response.clone(),
        })
    }
}

fn client(body: Value) -> (Client<Mock>, Arc<Mutex<Vec<HttpRequest>>>) {
    let recorded = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        request: recorded.clone(),
        response: body,
    };
    let options = ClientOptions {
        min_request_interval: Duration::ZERO,
        ..ClientOptions::default()
    };
    (Client::new(transport, options).unwrap(), recorded)
}

#[tokio::test]
async fn mock_ka00001_contract() {
    let wire_request = json!({});
    let request: Ka00001Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{"acctNo": "  -0000.00 ", "return_code": 0, "unreviewed_marker": "preserved"}"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka00001(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka00001");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_ka01690_contract() {
    let wire_request = json!({"qry_dt": "20261012"});
    let request: Ka01690Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "dt": "  -0000.00 ",
            "tot_buy_amt": "  -0001.00 ",
            "tot_evlt_amt": "  -0002.00 ",
            "tot_evltv_prft": "  -0003.00 ",
            "tot_prft_rt": "  -0004.00 ",
            "dbst_bal": "  -0005.00 ",
            "day_stk_asst": "  -0006.00 ",
            "buy_wght": "  -0007.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "day_bal_rt": [
                {
                    "cur_prc": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "rmnd_qty": "  +0003.00 ",
                    "buy_uv": "  +0004.00 ",
                    "buy_wght": "  +0005.00 ",
                    "evltv_prft": "  +0006.00 ",
                    "prft_rt": "  +0007.00 ",
                    "evlt_amt": "  +0008.00 ",
                    "evlt_wght": "  +0009.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka01690(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.day_bal_rt.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka01690");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.qry_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10072_contract() {
    let wire_request = json!({"stk_cd": "005930", "strt_dt": "20261012"});
    let request: Ka10072Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "dt_stk_div_rlzt_pl": [
                {
                    "stk_nm": "  +0000.00 ",
                    "cntr_qty": "  +0001.00 ",
                    "buy_uv": "  +0002.00 ",
                    "cntr_pric": "  +0003.00 ",
                    "tdy_sel_pl": "  +0004.00 ",
                    "pl_rt": "  +0005.00 ",
                    "stk_cd": "  +0006.00 ",
                    "tdy_trde_cmsn": "  +0007.00 ",
                    "tdy_trde_tax": "  +0008.00 ",
                    "wthd_alowa": "  +0009.00 ",
                    "loan_dt": "  +00010.00 ",
                    "crd_tp": "  +00011.00 ",
                    "stk_cd_1": "  +00012.00 ",
                    "tdy_sel_pl_1": "  +00013.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10072(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.dt_stk_div_rlzt_pl.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10072");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10073_contract() {
    let wire_request = json!({"stk_cd": "005930", "strt_dt": "20261012", "end_dt": "20261012"});
    let request: Ka10073Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "dt_stk_rlzt_pl": [
                {
                    "dt": "  +0000.00 ",
                    "tdy_htssel_cmsn": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "cntr_qty": "  +0003.00 ",
                    "buy_uv": "  +0004.00 ",
                    "cntr_pric": "  +0005.00 ",
                    "tdy_sel_pl": "  +0006.00 ",
                    "pl_rt": "  +0007.00 ",
                    "stk_cd": "  +0008.00 ",
                    "tdy_trde_cmsn": "  +0009.00 ",
                    "tdy_trde_tax": "  +00010.00 ",
                    "wthd_alowa": "  +00011.00 ",
                    "loan_dt": "  +00012.00 ",
                    "crd_tp": "  +00013.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10073(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.dt_stk_rlzt_pl.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10073");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10074_contract() {
    let wire_request = json!({"strt_dt": "20261012", "end_dt": "20261012"});
    let request: Ka10074Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_buy_amt": "  -0000.00 ",
            "tot_sell_amt": "  -0001.00 ",
            "rlzt_pl": "  -0002.00 ",
            "trde_cmsn": "  -0003.00 ",
            "trde_tax": "  -0004.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "dt_rlzt_pl": [
                {
                    "dt": "  +0000.00 ",
                    "buy_amt": "  +0001.00 ",
                    "sell_amt": "  +0002.00 ",
                    "tdy_sel_pl": "  +0003.00 ",
                    "tdy_trde_cmsn": "  +0004.00 ",
                    "tdy_trde_tax": "  +0005.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10074(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.dt_rlzt_pl.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10074");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10075_contract() {
    let wire_request =
        json!({"all_stk_tp": "0", "trde_tp": "0", "stk_cd": "005930", "stex_tp": "0"});
    let request: Ka10075Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "oso": [
                {
                    "acnt_no": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "mang_empno": "  +0002.00 ",
                    "stk_cd": "  +0003.00 ",
                    "tsk_tp": "  +0004.00 ",
                    "ord_stt": "  +0005.00 ",
                    "stk_nm": "  +0006.00 ",
                    "ord_qty": "  +0007.00 ",
                    "ord_pric": "  +0008.00 ",
                    "oso_qty": "  +0009.00 ",
                    "cntr_tot_amt": "  +00010.00 ",
                    "orig_ord_no": "  +00011.00 ",
                    "io_tp_nm": "  +00012.00 ",
                    "trde_tp": "  +00013.00 ",
                    "tm": "  +00014.00 ",
                    "cntr_no": "  +00015.00 ",
                    "cntr_pric": "  +00016.00 ",
                    "cntr_qty": "  +00017.00 ",
                    "cur_prc": "  +00018.00 ",
                    "sel_bid": "  +00019.00 ",
                    "buy_bid": "  +00020.00 ",
                    "unit_cntr_pric": "  +00021.00 ",
                    "unit_cntr_qty": "  +00022.00 ",
                    "tdy_trde_cmsn": "  +00023.00 ",
                    "tdy_trde_tax": "  +00024.00 ",
                    "ind_invsr": "  +00025.00 ",
                    "stex_tp": "  +00026.00 ",
                    "stex_tp_txt": "  +00027.00 ",
                    "sor_yn": "  +00028.00 ",
                    "stop_pric": "  +00029.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10075(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(pages[0].typed.oso.as_ref().unwrap()[0].extra.len(), 1);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10075");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.all_stk_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.all_stk_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.all_stk_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10076_contract() {
    let wire_request = json!({"stk_cd": "005930", "qry_tp": "0", "sell_tp": "0", "ord_no": "0000001", "stex_tp": "0"});
    let request: Ka10076Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "cntr": [
                {
                    "ord_no": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "io_tp_nm": "  +0002.00 ",
                    "ord_pric": "  +0003.00 ",
                    "ord_qty": "  +0004.00 ",
                    "cntr_pric": "  +0005.00 ",
                    "cntr_qty": "  +0006.00 ",
                    "oso_qty": "  +0007.00 ",
                    "tdy_trde_cmsn": "  +0008.00 ",
                    "tdy_trde_tax": "  +0009.00 ",
                    "ord_stt": "  +00010.00 ",
                    "trde_tp": "  +00011.00 ",
                    "orig_ord_no": "  +00012.00 ",
                    "ord_tm": "  +00013.00 ",
                    "stk_cd": "  +00014.00 ",
                    "stex_tp": "  +00015.00 ",
                    "stex_tp_txt": "  +00016.00 ",
                    "sor_yn": "  +00017.00 ",
                    "stop_pric": "  +00018.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10076(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(pages[0].typed.cntr.as_ref().unwrap()[0].extra.len(), 1);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10076");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_no = Some("X".repeat(11));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10077_contract() {
    let wire_request = json!({"stk_cd": "005930"});
    let request: Ka10077Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tdy_rlzt_pl": "  -0000.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "tdy_rlzt_pl_dtl": [
                {
                    "stk_nm": "  +0000.00 ",
                    "cntr_qty": "  +0001.00 ",
                    "buy_uv": "  +0002.00 ",
                    "cntr_pric": "  +0003.00 ",
                    "tdy_sel_pl": "  +0004.00 ",
                    "pl_rt": "  +0005.00 ",
                    "tdy_trde_cmsn": "  +0006.00 ",
                    "tdy_trde_tax": "  +0007.00 ",
                    "stk_cd": "  +0008.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10077(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.tdy_rlzt_pl_dtl.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10077");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = "X".repeat(7);
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10085_contract() {
    let wire_request = json!({"stex_tp": "0"});
    let request: Ka10085Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_prft_rt": [
                {
                    "dt": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "cur_prc": "  +0003.00 ",
                    "pur_pric": "  +0004.00 ",
                    "pur_amt": "  +0005.00 ",
                    "rmnd_qty": "  +0006.00 ",
                    "tdy_sel_pl": "  +0007.00 ",
                    "tdy_trde_cmsn": "  +0008.00 ",
                    "tdy_trde_tax": "  +0009.00 ",
                    "crd_tp": "  +00010.00 ",
                    "loan_dt": "  +00011.00 ",
                    "setl_remn": "  +00012.00 ",
                    "clrn_alow_qty": "  +00013.00 ",
                    "crd_amt": "  +00014.00 ",
                    "crd_int": "  +00015.00 ",
                    "expr_dt": "  +00016.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10085(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_prft_rt.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10085");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10088_contract() {
    let wire_request = json!({"ord_no": "0000001"});
    let request: Ka10088Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "osop": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "ord_no": "  +0002.00 ",
                    "ord_qty": "  +0003.00 ",
                    "ord_pric": "  +0004.00 ",
                    "osop_qty": "  +0005.00 ",
                    "io_tp_nm": "  +0006.00 ",
                    "trde_tp": "  +0007.00 ",
                    "sell_tp": "  +0008.00 ",
                    "cntr_qty": "  +0009.00 ",
                    "ord_stt": "  +00010.00 ",
                    "cur_prc": "  +00011.00 ",
                    "stex_tp": "  +00012.00 ",
                    "stex_tp_txt": "  +00013.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10088(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(pages[0].typed.osop.as_ref().unwrap()[0].extra.len(), 1);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10088");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_no = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_no = "X".repeat(21);
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ka10170_contract() {
    let wire_request = json!({"base_dt": "20261012", "ottks_tp": "1", "ch_crd_tp": "0"});
    let request: Ka10170Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_sell_amt": "  -0000.00 ",
            "tot_buy_amt": "  -0001.00 ",
            "tot_cmsn_tax": "  -0002.00 ",
            "tot_exct_amt": "  -0003.00 ",
            "tot_pl_amt": "  -0004.00 ",
            "tot_prft_rt": "  -0005.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "tdy_trde_diary": [
                {
                    "stk_nm": "  +0000.00 ",
                    "buy_avg_pric": "  +0001.00 ",
                    "buy_qty": "  +0002.00 ",
                    "sel_avg_pric": "  +0003.00 ",
                    "sell_qty": "  +0004.00 ",
                    "cmsn_alm_tax": "  +0005.00 ",
                    "pl_amt": "  +0006.00 ",
                    "sell_amt": "  +0007.00 ",
                    "buy_amt": "  +0008.00 ",
                    "prft_rt": "  +0009.00 ",
                    "stk_cd": "  +00010.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ka10170(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.tdy_trde_diary.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ka10170");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.base_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ottks_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ottks_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ottks_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ch_crd_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ch_crd_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ch_crd_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00001_contract() {
    let wire_request = json!({"qry_tp": "2"});
    let request: Kt00001Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "entr": "  -0000.00 ",
            "profa_ch": "  -0001.00 ",
            "bncr_profa_ch": "  -0002.00 ",
            "nxdy_bncr_sell_exct": "  -0003.00 ",
            "fc_stk_krw_repl_set_amt": "  -0004.00 ",
            "crd_grnta_ch": "  -0005.00 ",
            "crd_grnt_ch": "  -0006.00 ",
            "add_grnt_ch": "  -0007.00 ",
            "etc_profa": "  -0008.00 ",
            "uncl_stk_amt": "  -0009.00 ",
            "shrts_prica": "  -00010.00 ",
            "crd_set_grnta": "  -00011.00 ",
            "chck_ina_amt": "  -00012.00 ",
            "etc_chck_ina_amt": "  -00013.00 ",
            "crd_grnt_ruse": "  -00014.00 ",
            "knx_asset_evltv": "  -00015.00 ",
            "elwdpst_evlta": "  -00016.00 ",
            "crd_ls_rght_frcs_amt": "  -00017.00 ",
            "lvlh_join_amt": "  -00018.00 ",
            "lvlh_trns_alowa": "  -00019.00 ",
            "repl_amt": "  -00020.00 ",
            "remn_repl_evlta": "  -00021.00 ",
            "trst_remn_repl_evlta": "  -00022.00 ",
            "bncr_remn_repl_evlta": "  -00023.00 ",
            "profa_repl": "  -00024.00 ",
            "crd_grnta_repl": "  -00025.00 ",
            "crd_grnt_repl": "  -00026.00 ",
            "add_grnt_repl": "  -00027.00 ",
            "rght_repl_amt": "  -00028.00 ",
            "pymn_alow_amt": "  -00029.00 ",
            "wrap_pymn_alow_amt": "  -00030.00 ",
            "ord_alow_amt": "  -00031.00 ",
            "bncr_buy_alowa": "  -00032.00 ",
            "20stk_ord_alow_amt": "  -00033.00 ",
            "30stk_ord_alow_amt": "  -00034.00 ",
            "40stk_ord_alow_amt": "  -00035.00 ",
            "100stk_ord_alow_amt": "  -00036.00 ",
            "ch_uncla": "  -00037.00 ",
            "ch_uncla_dlfe": "  -00038.00 ",
            "ch_uncla_tot": "  -00039.00 ",
            "crd_int_npay": "  -00040.00 ",
            "int_npay_amt_dlfe": "  -00041.00 ",
            "int_npay_amt_tot": "  -00042.00 ",
            "etc_loana": "  -00043.00 ",
            "etc_loana_dlfe": "  -00044.00 ",
            "etc_loan_tot": "  -00045.00 ",
            "nrpy_loan": "  -00046.00 ",
            "loan_sum": "  -00047.00 ",
            "ls_sum": "  -00048.00 ",
            "crd_grnt_rt": "  -00049.00 ",
            "mdstrm_usfe": "  -00050.00 ",
            "min_ord_alow_yn": "  -00051.00 ",
            "loan_remn_evlt_amt": "  -00052.00 ",
            "dpst_grntl_remn": "  -00053.00 ",
            "sell_grntl_remn": "  -00054.00 ",
            "d1_entra": "  -00055.00 ",
            "d1_slby_exct_amt": "  -00056.00 ",
            "d1_buy_exct_amt": "  -00057.00 ",
            "d1_out_rep_mor": "  -00058.00 ",
            "d1_sel_exct_amt": "  -00059.00 ",
            "d1_pymn_alow_amt": "  -00060.00 ",
            "d2_entra": "  -00061.00 ",
            "d2_slby_exct_amt": "  -00062.00 ",
            "d2_buy_exct_amt": "  -00063.00 ",
            "d2_out_rep_mor": "  -00064.00 ",
            "d2_sel_exct_amt": "  -00065.00 ",
            "d2_pymn_alow_amt": "  -00066.00 ",
            "50stk_ord_alow_amt": "  -00067.00 ",
            "60stk_ord_alow_amt": "  -00068.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "stk_entr_prst": [
                {
                    "crnc_cd": "  +0000.00 ",
                    "fx_entr": "  +0001.00 ",
                    "fc_krw_repl_evlta": "  +0002.00 ",
                    "fc_trst_profa": "  +0003.00 ",
                    "pymn_alow_amt": "  +0004.00 ",
                    "pymn_alow_amt_entr": "  +0005.00 ",
                    "ord_alow_amt_entr": "  +0006.00 ",
                    "fc_uncla": "  +0007.00 ",
                    "fc_ch_uncla": "  +0008.00 ",
                    "dly_amt": "  +0009.00 ",
                    "d1_fx_entr": "  +00010.00 ",
                    "d2_fx_entr": "  +00011.00 ",
                    "d3_fx_entr": "  +00012.00 ",
                    "d4_fx_entr": "  +00013.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00001(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.stk_entr_prst.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00001");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00002_contract() {
    let wire_request = json!({"start_dt": "20261012", "end_dt": "20261012"});
    let request: Kt00002Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "daly_prsm_dpst_aset_amt_prst": [
                {
                    "dt": "  +0000.00 ",
                    "entr": "  +0001.00 ",
                    "grnt_use_amt": "  +0002.00 ",
                    "crd_loan": "  +0003.00 ",
                    "ls_grnt": "  +0004.00 ",
                    "repl_amt": "  +0005.00 ",
                    "prsm_dpst_aset_amt": "  +0006.00 ",
                    "prsm_dpst_aset_amt_bncr_skip": "  +0007.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00002(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0]
            .typed
            .daly_prsm_dpst_aset_amt_prst
            .as_ref()
            .unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00002");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.start_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.start_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.start_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00003_contract() {
    let wire_request = json!({"qry_tp": "0"});
    let request: Kt00003Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(r#"{"prsm_dpst_aset_amt": "  -0000.00 ", "return_code": 0, "unreviewed_marker": "preserved"}"#).unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00003(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00003");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00004_contract() {
    let wire_request = json!({"qry_tp": "0", "dmst_stex_tp": "KRX"});
    let request: Kt00004Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "acnt_nm": "  -0000.00 ",
            "brch_nm": "  -0001.00 ",
            "entr": "  -0002.00 ",
            "d2_entra": "  -0003.00 ",
            "tot_est_amt": "  -0004.00 ",
            "aset_evlt_amt": "  -0005.00 ",
            "tot_pur_amt": "  -0006.00 ",
            "prsm_dpst_aset_amt": "  -0007.00 ",
            "tot_grnt_sella": "  -0008.00 ",
            "tdy_lspft_amt": "  -0009.00 ",
            "invt_bsamt": "  -00010.00 ",
            "lspft_amt": "  -00011.00 ",
            "tdy_lspft": "  -00012.00 ",
            "lspft2": "  -00013.00 ",
            "lspft": "  -00014.00 ",
            "tdy_lspft_rt": "  -00015.00 ",
            "lspft_ratio": "  -00016.00 ",
            "lspft_rt": "  -00017.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "stk_acnt_evlt_prst": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "rmnd_qty": "  +0002.00 ",
                    "avg_prc": "  +0003.00 ",
                    "cur_prc": "  +0004.00 ",
                    "evlt_amt": "  +0005.00 ",
                    "pl_amt": "  +0006.00 ",
                    "pl_rt": "  +0007.00 ",
                    "loan_dt": "  +0008.00 ",
                    "pur_amt": "  +0009.00 ",
                    "setl_remn": "  +00010.00 ",
                    "pred_buyq": "  +00011.00 ",
                    "pred_sellq": "  +00012.00 ",
                    "tdy_buyq": "  +00013.00 ",
                    "tdy_sellq": "  +00014.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00004(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.stk_acnt_evlt_prst.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00004");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00005_contract() {
    let wire_request = json!({"dmst_stex_tp": "KRX"});
    let request: Kt00005Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "entr": "  -0000.00 ",
            "entr_d1": "  -0001.00 ",
            "entr_d2": "  -0002.00 ",
            "pymn_alow_amt": "  -0003.00 ",
            "uncl_stk_amt": "  -0004.00 ",
            "repl_amt": "  -0005.00 ",
            "rght_repl_amt": "  -0006.00 ",
            "ord_alowa": "  -0007.00 ",
            "ch_uncla": "  -0008.00 ",
            "crd_int_npay_gold": "  -0009.00 ",
            "etc_loana": "  -00010.00 ",
            "nrpy_loan": "  -00011.00 ",
            "profa_ch": "  -00012.00 ",
            "repl_profa": "  -00013.00 ",
            "stk_buy_tot_amt": "  -00014.00 ",
            "evlt_amt_tot": "  -00015.00 ",
            "tot_pl_tot": "  -00016.00 ",
            "tot_pl_rt": "  -00017.00 ",
            "tot_re_buy_alowa": "  -00018.00 ",
            "20ord_alow_amt": "  -00019.00 ",
            "30ord_alow_amt": "  -00020.00 ",
            "40ord_alow_amt": "  -00021.00 ",
            "50ord_alow_amt": "  -00022.00 ",
            "60ord_alow_amt": "  -00023.00 ",
            "100ord_alow_amt": "  -00024.00 ",
            "crd_loan_tot": "  -00025.00 ",
            "crd_loan_ls_tot": "  -00026.00 ",
            "crd_grnt_rt": "  -00027.00 ",
            "dpst_grnt_use_amt_amt": "  -00028.00 ",
            "grnt_loan_amt": "  -00029.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "stk_cntr_remn": [
                {
                    "crd_tp": "  +0000.00 ",
                    "loan_dt": "  +0001.00 ",
                    "expr_dt": "  +0002.00 ",
                    "stk_cd": "  +0003.00 ",
                    "stk_nm": "  +0004.00 ",
                    "setl_remn": "  +0005.00 ",
                    "cur_qty": "  +0006.00 ",
                    "cur_prc": "  +0007.00 ",
                    "buy_uv": "  +0008.00 ",
                    "pur_amt": "  +0009.00 ",
                    "evlt_amt": "  +00010.00 ",
                    "evltv_prft": "  +00011.00 ",
                    "pl_rt": "  +00012.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00005(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.stk_cntr_remn.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00005");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00007_contract() {
    let wire_request = json!({"ord_dt": "20261012", "qry_tp": "1", "stk_bond_tp": "0", "sell_tp": "0", "stk_cd": "005930", "fr_ord_no": "1", "dmst_stex_tp": "%"});
    let request: Kt00007Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_ord_cntr_prps_dtl": [
                {
                    "ord_no": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "trde_tp": "  +0002.00 ",
                    "crd_tp": "  +0003.00 ",
                    "ord_qty": "  +0004.00 ",
                    "ord_uv": "  +0005.00 ",
                    "cnfm_qty": "  +0006.00 ",
                    "acpt_tp": "  +0007.00 ",
                    "rsrv_tp": "  +0008.00 ",
                    "ord_tm": "  +0009.00 ",
                    "ori_ord": "  +00010.00 ",
                    "stk_nm": "  +00011.00 ",
                    "io_tp_nm": "  +00012.00 ",
                    "loan_dt": "  +00013.00 ",
                    "cntr_qty": "  +00014.00 ",
                    "cntr_uv": "  +00015.00 ",
                    "ord_remnq": "  +00016.00 ",
                    "comm_ord_tp": "  +00017.00 ",
                    "mdfy_cncl": "  +00018.00 ",
                    "cnfm_tm": "  +00019.00 ",
                    "dmst_stex_tp": "  +00020.00 ",
                    "cond_uv": "  +00021.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00007(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_ord_cntr_prps_dtl.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00007");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00008_contract() {
    let wire_request = json!({"strt_dcd_seq": "1"});
    let request: Kt00008Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "trde_dt": "  -0000.00 ",
            "setl_dt": "  -0001.00 ",
            "sell_amt_sum": "  -0002.00 ",
            "buy_amt_sum": "  -0003.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_nxdy_setl_frcs_prps_array": [
                {
                    "seq": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "loan_dt": "  +0002.00 ",
                    "qty": "  +0003.00 ",
                    "engg_amt": "  +0004.00 ",
                    "cmsn": "  +0005.00 ",
                    "incm_tax": "  +0006.00 ",
                    "rstx": "  +0007.00 ",
                    "stk_nm": "  +0008.00 ",
                    "sell_tp": "  +0009.00 ",
                    "unp": "  +00010.00 ",
                    "exct_amt": "  +00011.00 ",
                    "trde_tax": "  +00012.00 ",
                    "resi_tax": "  +00013.00 ",
                    "crd_tp": "  +00014.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00008(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0]
            .typed
            .acnt_nxdy_setl_frcs_prps_array
            .as_ref()
            .unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00008");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dcd_seq = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00009_contract() {
    let wire_request = json!({"ord_dt": "20261012", "stk_bond_tp": "0", "mrkt_tp": "0", "sell_tp": "0", "qry_tp": "0", "stk_cd": "005930", "fr_ord_no": "1", "dmst_stex_tp": "%"});
    let request: Kt00009Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "sell_grntl_engg_amt": "  -0000.00 ",
            "buy_engg_amt": "  -0001.00 ",
            "engg_amt": "  -0002.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_ord_cntr_prst_array": [
                {
                    "stk_bond_tp": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "trde_tp": "  +0003.00 ",
                    "io_tp_nm": "  +0004.00 ",
                    "ord_qty": "  +0005.00 ",
                    "ord_uv": "  +0006.00 ",
                    "cnfm_qty": "  +0007.00 ",
                    "rsrv_oppo": "  +0008.00 ",
                    "cntr_no": "  +0009.00 ",
                    "acpt_tp": "  +00010.00 ",
                    "orig_ord_no": "  +00011.00 ",
                    "stk_nm": "  +00012.00 ",
                    "setl_tp": "  +00013.00 ",
                    "crd_deal_tp": "  +00014.00 ",
                    "cntr_qty": "  +00015.00 ",
                    "cntr_uv": "  +00016.00 ",
                    "comm_ord_tp": "  +00017.00 ",
                    "mdfy_cncl_tp": "  +00018.00 ",
                    "cntr_tm": "  +00019.00 ",
                    "dmst_stex_tp": "  +00020.00 ",
                    "cond_uv": "  +00021.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00009(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_ord_cntr_prst_array.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00009");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00010_contract() {
    let wire_request = json!({"io_amt": "1", "stk_cd": "005930", "trde_tp": "1", "trde_qty": "1", "uv": "1", "exp_buy_unp": "1"});
    let request: Kt00010Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "profa_20ord_alow_amt": "  -0000.00 ",
            "profa_20ord_alowq": "  -0001.00 ",
            "profa_30ord_alow_amt": "  -0002.00 ",
            "profa_30ord_alowq": "  -0003.00 ",
            "profa_40ord_alow_amt": "  -0004.00 ",
            "profa_40ord_alowq": "  -0005.00 ",
            "profa_50ord_alow_amt": "  -0006.00 ",
            "profa_50ord_alowq": "  -0007.00 ",
            "profa_60ord_alow_amt": "  -0008.00 ",
            "profa_60ord_alowq": "  -0009.00 ",
            "profa_rdex_60ord_alow_amt": "  -00010.00 ",
            "profa_rdex_60ord_alowq": "  -00011.00 ",
            "profa_100ord_alow_amt": "  -00012.00 ",
            "profa_100ord_alowq": "  -00013.00 ",
            "pred_reu_alowa": "  -00014.00 ",
            "tdy_reu_alowa": "  -00015.00 ",
            "entr": "  -00016.00 ",
            "repl_amt": "  -00017.00 ",
            "uncla": "  -00018.00 ",
            "ord_pos_repl": "  -00019.00 ",
            "ord_alowa": "  -00020.00 ",
            "wthd_alowa": "  -00021.00 ",
            "nxdy_wthd_alowa": "  -00022.00 ",
            "pur_amt": "  -00023.00 ",
            "cmsn": "  -00024.00 ",
            "pur_exct_amt": "  -00025.00 ",
            "d2entra": "  -00026.00 ",
            "profa_rdex_aplc_tp": "  -00027.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00010(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00010");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.io_amt = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = "X".repeat(13);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.trde_qty = Some("X".repeat(11));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.uv = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.uv = "X".repeat(11);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.exp_buy_unp = Some("X".repeat(11));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00011_contract() {
    let wire_request = json!({"stk_cd": "005930", "uv": "1"});
    let request: Kt00011Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "stk_profa_rt": "  -0000.00 ",
            "profa_rt": "  -0001.00 ",
            "aplc_rt": "  -0002.00 ",
            "profa_20ord_alow_amt": "  -0003.00 ",
            "profa_20ord_alowq": "  -0004.00 ",
            "profa_20pred_reu_amt": "  -0005.00 ",
            "profa_20tdy_reu_amt": "  -0006.00 ",
            "profa_30ord_alow_amt": "  -0007.00 ",
            "profa_30ord_alowq": "  -0008.00 ",
            "profa_30pred_reu_amt": "  -0009.00 ",
            "profa_30tdy_reu_amt": "  -00010.00 ",
            "profa_40ord_alow_amt": "  -00011.00 ",
            "profa_40ord_alowq": "  -00012.00 ",
            "profa_40pred_reu_amt": "  -00013.00 ",
            "profa_40tdy_reu_amt": "  -00014.00 ",
            "profa_50ord_alow_amt": "  -00015.00 ",
            "profa_50ord_alowq": "  -00016.00 ",
            "profa_50pred_reu_amt": "  -00017.00 ",
            "profa_50tdy_reu_amt": "  -00018.00 ",
            "profa_60ord_alow_amt": "  -00019.00 ",
            "profa_60ord_alowq": "  -00020.00 ",
            "profa_60pred_reu_amt": "  -00021.00 ",
            "profa_60tdy_reu_amt": "  -00022.00 ",
            "profa_100ord_alow_amt": "  -00023.00 ",
            "profa_100ord_alowq": "  -00024.00 ",
            "profa_100pred_reu_amt": "  -00025.00 ",
            "profa_100tdy_reu_amt": "  -00026.00 ",
            "min_ord_alow_amt": "  -00027.00 ",
            "min_ord_alowq": "  -00028.00 ",
            "min_pred_reu_amt": "  -00029.00 ",
            "min_tdy_reu_amt": "  -00030.00 ",
            "entr": "  -00031.00 ",
            "repl_amt": "  -00032.00 ",
            "uncla": "  -00033.00 ",
            "ord_pos_repl": "  -00034.00 ",
            "ord_alowa": "  -00035.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00011(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00011");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = "X".repeat(13);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.uv = Some("X".repeat(11));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00012_contract() {
    let wire_request = json!({"stk_cd": "005930", "uv": "1"});
    let request: Kt00012Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "stk_assr_rt": "  -0000.00 ",
            "stk_assr_rt_nm": "  -0001.00 ",
            "assr_30ord_alow_amt": "  -0002.00 ",
            "assr_30ord_alowq": "  -0003.00 ",
            "assr_30pred_reu_amt": "  -0004.00 ",
            "assr_30tdy_reu_amt": "  -0005.00 ",
            "assr_40ord_alow_amt": "  -0006.00 ",
            "assr_40ord_alowq": "  -0007.00 ",
            "assr_40pred_reu_amt": "  -0008.00 ",
            "assr_40tdy_reu_amt": "  -0009.00 ",
            "assr_50ord_alow_amt": "  -00010.00 ",
            "assr_50ord_alowq": "  -00011.00 ",
            "assr_50pred_reu_amt": "  -00012.00 ",
            "assr_50tdy_reu_amt": "  -00013.00 ",
            "assr_60ord_alow_amt": "  -00014.00 ",
            "assr_60ord_alowq": "  -00015.00 ",
            "assr_60pred_reu_amt": "  -00016.00 ",
            "assr_60tdy_reu_amt": "  -00017.00 ",
            "entr": "  -00018.00 ",
            "repl_amt": "  -00019.00 ",
            "uncla": "  -00020.00 ",
            "ord_pos_repl": "  -00021.00 ",
            "ord_alowa": "  -00022.00 ",
            "out_alowa": "  -00023.00 ",
            "out_pos_qty": "  -00024.00 ",
            "min_amt": "  -00025.00 ",
            "min_qty": "  -00026.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00012(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00012");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = "X".repeat(13);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.uv = Some("X".repeat(11));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00013_contract() {
    let wire_request = json!({});
    let request: Kt00013Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tdy_reu_objt_amt": "  -0000.00 ",
            "tdy_reu_use_amt": "  -0001.00 ",
            "tdy_reu_alowa": "  -0002.00 ",
            "tdy_reu_lmtt_amt": "  -0003.00 ",
            "tdy_reu_alowa_fin": "  -0004.00 ",
            "pred_reu_objt_amt": "  -0005.00 ",
            "pred_reu_use_amt": "  -0006.00 ",
            "pred_reu_alowa": "  -0007.00 ",
            "pred_reu_lmtt_amt": "  -0008.00 ",
            "pred_reu_alowa_fin": "  -0009.00 ",
            "ch_amt": "  -00010.00 ",
            "ch_profa": "  -00011.00 ",
            "use_pos_ch": "  -00012.00 ",
            "ch_use_lmtt_amt": "  -00013.00 ",
            "use_pos_ch_fin": "  -00014.00 ",
            "repl_amt_amt": "  -00015.00 ",
            "repl_profa": "  -00016.00 ",
            "use_pos_repl": "  -00017.00 ",
            "repl_use_lmtt_amt": "  -00018.00 ",
            "use_pos_repl_fin": "  -00019.00 ",
            "crd_grnta_ch": "  -00020.00 ",
            "crd_grnta_repl": "  -00021.00 ",
            "crd_grnt_ch": "  -00022.00 ",
            "crd_grnt_repl": "  -00023.00 ",
            "uncla": "  -00024.00 ",
            "ls_grnt_reu_gold": "  -00025.00 ",
            "20ord_alow_amt": "  -00026.00 ",
            "30ord_alow_amt": "  -00027.00 ",
            "40ord_alow_amt": "  -00028.00 ",
            "50ord_alow_amt": "  -00029.00 ",
            "60ord_alow_amt": "  -00030.00 ",
            "100ord_alow_amt": "  -00031.00 ",
            "tdy_crd_rpya_loss_amt": "  -00032.00 ",
            "pred_crd_rpya_loss_amt": "  -00033.00 ",
            "tdy_ls_rpya_loss_repl_profa": "  -00034.00 ",
            "pred_ls_rpya_loss_repl_profa": "  -00035.00 ",
            "evlt_repl_amt_spg_use_skip": "  -00036.00 ",
            "evlt_repl_rt": "  -00037.00 ",
            "crd_repl_profa": "  -00038.00 ",
            "ch_ord_repl_profa": "  -00039.00 ",
            "crd_ord_repl_profa": "  -00040.00 ",
            "crd_repl_conv_gold": "  -00041.00 ",
            "repl_alowa": "  -00042.00 ",
            "repl_alowa_2": "  -00043.00 ",
            "ch_repl_lck_gold": "  -00044.00 ",
            "crd_repl_lck_gold": "  -00045.00 ",
            "ch_ord_alow_repla": "  -00046.00 ",
            "crd_ord_alow_repla": "  -00047.00 ",
            "d2vexct_entr": "  -00048.00 ",
            "d2ch_ord_alow_amt": "  -00049.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00013(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00013");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_kt00016_contract() {
    let wire_request = json!({"fr_dt": "20261012", "to_dt": "20261012"});
    let request: Kt00016Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "mang_empno": "  -0000.00 ",
            "mngr_nm": "  -0001.00 ",
            "dept_nm": "  -0002.00 ",
            "entr_fr": "  -0003.00 ",
            "entr_to": "  -0004.00 ",
            "scrt_evlt_amt_fr": "  -0005.00 ",
            "scrt_evlt_amt_to": "  -0006.00 ",
            "ls_grnt_fr": "  -0007.00 ",
            "ls_grnt_to": "  -0008.00 ",
            "crd_loan_fr": "  -0009.00 ",
            "crd_loan_to": "  -00010.00 ",
            "ch_uncla_fr": "  -00011.00 ",
            "ch_uncla_to": "  -00012.00 ",
            "krw_asgna_fr": "  -00013.00 ",
            "krw_asgna_to": "  -00014.00 ",
            "ls_evlta_fr": "  -00015.00 ",
            "ls_evlta_to": "  -00016.00 ",
            "rght_evlta_fr": "  -00017.00 ",
            "rght_evlta_to": "  -00018.00 ",
            "loan_amt_fr": "  -00019.00 ",
            "loan_amt_to": "  -00020.00 ",
            "etc_loana_fr": "  -00021.00 ",
            "etc_loana_to": "  -00022.00 ",
            "crd_int_npay_gold_fr": "  -00023.00 ",
            "crd_int_npay_gold_to": "  -00024.00 ",
            "crd_int_fr": "  -00025.00 ",
            "crd_int_to": "  -00026.00 ",
            "tot_amt_fr": "  -00027.00 ",
            "tot_amt_to": "  -00028.00 ",
            "invt_bsamt": "  -00029.00 ",
            "evltv_prft": "  -00030.00 ",
            "prft_rt": "  -00031.00 ",
            "tern_rt": "  -00032.00 ",
            "termin_tot_trns": "  -00033.00 ",
            "termin_tot_pymn": "  -00034.00 ",
            "termin_tot_inq": "  -00035.00 ",
            "termin_tot_outq": "  -00036.00 ",
            "futr_repl_sella": "  -00037.00 ",
            "trst_repl_sella": "  -00038.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00016(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00016");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.fr_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt00017_contract() {
    let wire_request = json!({});
    let request: Kt00017Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "d2_entra": "  -0000.00 ",
            "crd_int_npay_gold": "  -0001.00 ",
            "etc_loana": "  -0002.00 ",
            "gnrl_stk_evlt_amt_d2": "  -0003.00 ",
            "dpst_grnt_use_amt_d2": "  -0004.00 ",
            "crd_stk_evlt_amt_d2": "  -0005.00 ",
            "crd_loan_d2": "  -0006.00 ",
            "crd_loan_evlta_d2": "  -0007.00 ",
            "crd_ls_grnt_d2": "  -0008.00 ",
            "crd_ls_evlta_d2": "  -0009.00 ",
            "ina_amt": "  -00010.00 ",
            "outa": "  -00011.00 ",
            "inq_amt": "  -00012.00 ",
            "outq_amt": "  -00013.00 ",
            "sell_amt": "  -00014.00 ",
            "buy_amt": "  -00015.00 ",
            "cmsn": "  -00016.00 ",
            "tax": "  -00017.00 ",
            "stk_pur_cptal_loan_amt": "  -00018.00 ",
            "rp_evlt_amt": "  -00019.00 ",
            "bd_evlt_amt": "  -00020.00 ",
            "elsevlt_amt": "  -00021.00 ",
            "crd_int_amt": "  -00022.00 ",
            "sel_prica_grnt_loan_int_amt_amt": "  -00023.00 ",
            "dvida_amt": "  -00024.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00017(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00017");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_kt00018_contract() {
    let wire_request = json!({"qry_tp": "1", "dmst_stex_tp": "KRX"});
    let request: Kt00018Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_pur_amt": "  -0000.00 ",
            "tot_evlt_amt": "  -0001.00 ",
            "tot_evlt_pl": "  -0002.00 ",
            "tot_prft_rt": "  -0003.00 ",
            "prsm_dpst_aset_amt": "  -0004.00 ",
            "tot_loan_amt": "  -0005.00 ",
            "tot_crd_loan_amt": "  -0006.00 ",
            "tot_crd_ls_amt": "  -0007.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_evlt_remn_indv_tot": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "evltv_prft": "  +0002.00 ",
                    "prft_rt": "  +0003.00 ",
                    "pur_pric": "  +0004.00 ",
                    "pred_close_pric": "  +0005.00 ",
                    "rmnd_qty": "  +0006.00 ",
                    "trde_able_qty": "  +0007.00 ",
                    "cur_prc": "  +0008.00 ",
                    "pred_buyq": "  +0009.00 ",
                    "pred_sellq": "  +00010.00 ",
                    "tdy_buyq": "  +00011.00 ",
                    "tdy_sellq": "  +00012.00 ",
                    "pur_amt": "  +00013.00 ",
                    "pur_cmsn": "  +00014.00 ",
                    "evlt_amt": "  +00015.00 ",
                    "sell_cmsn": "  +00016.00 ",
                    "tax": "  +00017.00 ",
                    "sum_cmsn": "  +00018.00 ",
                    "poss_rt": "  +00019.00 ",
                    "crd_tp": "  +00020.00 ",
                    "crd_tp_nm": "  +00021.00 ",
                    "crd_loan_dt": "  +00022.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt00018(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_evlt_remn_indv_tot.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt00018");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt50020_contract() {
    let wire_request = json!({});
    let request: Kt50020Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_entr": "  -0000.00 ",
            "net_entr": "  -0001.00 ",
            "tot_est_amt": "  -0002.00 ",
            "net_amt": "  -0003.00 ",
            "tot_book_amt2": "  -0004.00 ",
            "tot_dep_amt": "  -0005.00 ",
            "paym_alowa": "  -0006.00 ",
            "pl_amt": "  -0007.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "gold_acnt_evlt_prst": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "real_qty": "  +0002.00 ",
                    "avg_prc": "  +0003.00 ",
                    "cur_prc": "  +0004.00 ",
                    "est_amt": "  +0005.00 ",
                    "est_lspft": "  +0006.00 ",
                    "est_ratio": "  +0007.00 ",
                    "cmsn": "  +0008.00 ",
                    "vlad_tax": "  +0009.00 ",
                    "book_amt2": "  +00010.00 ",
                    "pl_prch_prc": "  +00011.00 ",
                    "qty": "  +00012.00 ",
                    "buy_qty": "  +00013.00 ",
                    "sell_qty": "  +00014.00 ",
                    "able_qty": "  +00015.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50020(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.gold_acnt_evlt_prst.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50020");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_kt50021_contract() {
    let wire_request = json!({});
    let request: Kt50021Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "entra": "  -0000.00 ",
            "profa_ch": "  -0001.00 ",
            "chck_ina_amt": "  -0002.00 ",
            "etc_loan": "  -0003.00 ",
            "etc_loan_dlfe": "  -0004.00 ",
            "etc_loan_tot": "  -0005.00 ",
            "prsm_entra": "  -0006.00 ",
            "buy_exct_amt": "  -0007.00 ",
            "sell_exct_amt": "  -0008.00 ",
            "sell_buy_exct_amt": "  -0009.00 ",
            "dly_amt": "  -00010.00 ",
            "prsm_pymn_alow_amt": "  -00011.00 ",
            "pymn_alow_amt": "  -00012.00 ",
            "ord_alow_amt": "  -00013.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50021(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50021");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_kt50030_contract() {
    let wire_request = json!({"ord_dt": "20261012", "qry_tp": "1", "mrkt_deal_tp": "5", "stk_bond_tp": "0", "slby_tp": "0", "stk_cd": "005930", "fr_ord_no": "1", "dmst_stex_tp": "%"});
    let request: Kt50030Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_ord_cntr_prst": [
                {
                    "stk_bond_tp": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "trde_tp": "  +0003.00 ",
                    "io_tp_nm": "  +0004.00 ",
                    "ord_qty": "  +0005.00 ",
                    "ord_uv": "  +0006.00 ",
                    "cnfm_qty": "  +0007.00 ",
                    "data_send_end_tp": "  +0008.00 ",
                    "mrkt_deal_tp": "  +0009.00 ",
                    "rsrv_tp": "  +00010.00 ",
                    "orig_ord_no": "  +00011.00 ",
                    "stk_nm": "  +00012.00 ",
                    "dcd_tp_nm": "  +00013.00 ",
                    "crd_deal_tp": "  +00014.00 ",
                    "cntr_qty": "  +00015.00 ",
                    "cntr_uv": "  +00016.00 ",
                    "ord_remnq": "  +00017.00 ",
                    "comm_ord_tp": "  +00018.00 ",
                    "mdfy_cncl_tp": "  +00019.00 ",
                    "dmst_stex_tp": "  +00020.00 ",
                    "cond_uv": "  +00021.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50030(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_ord_cntr_prst.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50030");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt50031_contract() {
    let wire_request = json!({"ord_dt": "20261012", "qry_tp": "1", "stk_bond_tp": "0", "sell_tp": "0", "stk_cd": "005930", "fr_ord_no": "1", "dmst_stex_tp": "%"});
    let request: Kt50031Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_ord_cntr_prps_dtl": [
                {
                    "ord_no": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "trde_tp": "  +0002.00 ",
                    "crd_tp": "  +0003.00 ",
                    "ord_qty": "  +0004.00 ",
                    "ord_uv": "  +0005.00 ",
                    "cnfm_qty": "  +0006.00 ",
                    "acpt_tp": "  +0007.00 ",
                    "rsrv_tp": "  +0008.00 ",
                    "ord_tm": "  +0009.00 ",
                    "ori_ord": "  +00010.00 ",
                    "stk_nm": "  +00011.00 ",
                    "io_tp_nm": "  +00012.00 ",
                    "loan_dt": "  +00013.00 ",
                    "cntr_qty": "  +00014.00 ",
                    "cntr_uv": "  +00015.00 ",
                    "ord_remnq": "  +00016.00 ",
                    "comm_ord_tp": "  +00017.00 ",
                    "mdfy_cncl": "  +00018.00 ",
                    "cnfm_tm": "  +00019.00 ",
                    "dmst_stex_tp": "  +00020.00 ",
                    "cond_uv": "  +00021.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50031(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_ord_cntr_prps_dtl.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50031");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "X".repeat(7);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt50032_contract() {
    let wire_request =
        json!({"strt_dt": "20261012", "end_dt": "20261012", "tp": "0", "stk_cd": "005930"});
    let request: Kt50032Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "acnt_print": "  -0000.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "gold_trde_hist": [
                {
                    "deal_dt": "  +0000.00 ",
                    "deal_no": "  +0001.00 ",
                    "rmrk_nm": "  +0002.00 ",
                    "deal_qty": "  +0003.00 ",
                    "gold_spot_vat": "  +0004.00 ",
                    "exct_amt": "  +0005.00 ",
                    "dly_sum": "  +0006.00 ",
                    "entra_remn": "  +0007.00 ",
                    "mdia_nm": "  +0008.00 ",
                    "orig_deal_no": "  +0009.00 ",
                    "stk_nm": "  +00010.00 ",
                    "uv_exrt": "  +00011.00 ",
                    "cmsn": "  +00012.00 ",
                    "uncl_ocr": "  +00013.00 ",
                    "rpym_sum": "  +00014.00 ",
                    "spot_remn": "  +00015.00 ",
                    "proc_time": "  +00016.00 ",
                    "rcpy_no": "  +00017.00 ",
                    "stk_cd": "  +00018.00 ",
                    "deal_amt": "  +00019.00 ",
                    "tax_tot_amt": "  +00020.00 ",
                    "cntr_dt": "  +00021.00 ",
                    "proc_brch_nm": "  +00022.00 ",
                    "prcsr": "  +00023.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50032(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.gold_trde_hist.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50032");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_kt50075_contract() {
    let wire_request = json!({"ord_dt": "20261012", "qry_tp": "1", "mrkt_deal_tp": "5", "stk_bond_tp": "0", "sell_tp": "0", "stk_cd": "005930", "fr_ord_no": "1", "dmst_stex_tp": "%"});
    let request: Kt50075Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "acnt_ord_oso_prst": [
                {
                    "stk_bond_tp": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "trde_tp": "  +0003.00 ",
                    "io_tp_nm": "  +0004.00 ",
                    "ord_qty": "  +0005.00 ",
                    "ord_uv": "  +0006.00 ",
                    "cnfm_qty": "  +0007.00 ",
                    "data_send_end_tp": "  +0008.00 ",
                    "mrkt_deal_tp": "  +0009.00 ",
                    "rsrv_tp": "  +00010.00 ",
                    "orig_ord_no": "  +00011.00 ",
                    "stk_nm": "  +00012.00 ",
                    "dcd_tp_nm": "  +00013.00 ",
                    "crd_deal_tp": "  +00014.00 ",
                    "cntr_qty": "  +00015.00 ",
                    "cntr_uv": "  +00016.00 ",
                    "ord_remnq": "  +00017.00 ",
                    "comm_ord_tp": "  +00018.00 ",
                    "mdfy_cncl_tp": "  +00019.00 ",
                    "dmst_stex_tp": "  +00020.00 ",
                    "cond_uv": "  +00021.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .kt50075(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.acnt_ord_oso_prst.as_ref().unwrap()[0]
            .extra
            .len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "kt50075");
    assert_eq!(recorded[0].path, "/api/dostk/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.mrkt_deal_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_bond_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.sell_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(8));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.dmst_stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21670_contract() {
    let wire_request = json!({"from": "20261012", "to": "20261012"});
    let request: Usa21670Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "base_dt": "  +0000.00 ",
                    "stk_evlta": "  +0001.00 ",
                    "pl_amt": "  +0002.00 ",
                    "dvid_amt": "  +0003.00 ",
                    "cmsn_tax": "  +0004.00 ",
                    "acum_pl_amt": "  +0005.00 ",
                    "pymn_amt": "  +0006.00 ",
                    "dast": "  +0007.00 ",
                    "dly_amt": "  +0008.00 ",
                    "sell_amt": "  +0009.00 ",
                    "buy_amt": "  +00010.00 ",
                    "prft_rt": "  +00011.00 ",
                    "frgn_stk_outq_amt": "  +00012.00 ",
                    "frgn_stk_inq_amt": "  +00013.00 ",
                    "ina_amt": "  +00014.00 ",
                    "exrt": "  +00015.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21670(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21670");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21680_contract() {
    let wire_request = json!({"from": "202610", "to": "202610"});
    let request: Usa21680Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "base_dt": "  +0000.00 ",
                    "stk_evlta": "  +0001.00 ",
                    "pl_amt": "  +0002.00 ",
                    "dvid_amt": "  +0003.00 ",
                    "cmsn_tax": "  +0004.00 ",
                    "acum_pl_amt": "  +0005.00 ",
                    "pymn_amt": "  +0006.00 ",
                    "dast": "  +0007.00 ",
                    "dly_amt": "  +0008.00 ",
                    "sell_amt": "  +0009.00 ",
                    "buy_amt": "  +00010.00 ",
                    "prft_rt": "  +00011.00 ",
                    "frgn_stk_outq_amt": "  +00012.00 ",
                    "frgn_stk_inq_amt": "  +00013.00 ",
                    "ina_amt": "  +00014.00 ",
                    "exrt": "  +00015.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21680(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21680");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21690_contract() {
    let wire_request = json!({"from": "2026", "to": "2026"});
    let request: Usa21690Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "base_dt": "  +0000.00 ",
                    "stk_evlta": "  +0001.00 ",
                    "pl_amt": "  +0002.00 ",
                    "dvid_amt": "  +0003.00 ",
                    "cmsn_tax": "  +0004.00 ",
                    "acum_pl_amt": "  +0005.00 ",
                    "pymn_amt": "  +0006.00 ",
                    "dast": "  +0007.00 ",
                    "dly_amt": "  +0008.00 ",
                    "sell_amt": "  +0009.00 ",
                    "buy_amt": "  +00010.00 ",
                    "prft_rt": "  +00011.00 ",
                    "frgn_stk_outq_amt": "  +00012.00 ",
                    "frgn_stk_inq_amt": "  +00013.00 ",
                    "ina_amt": "  +00014.00 ",
                    "exrt": "  +00015.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21690(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21690");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21730_contract() {
    let wire_request =
        json!({"from": "20261012", "to": "20261012", "stex_tp": "NA", "stk_cd": "AAPL"});
    let request: Usa21730Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "frst_base_dt": "  +0002.00 ",
                    "last_base_dt": "  +0003.00 ",
                    "pl_amt": "  +0004.00 ",
                    "prft_rt": "  +0005.00 ",
                    "frst_stk_evlta": "  +0006.00 ",
                    "last_stk_evlta": "  +0007.00 ",
                    "frst_qty": "  +0008.00 ",
                    "last_qty": "  +0009.00 ",
                    "buy_qty": "  +00010.00 ",
                    "sell_qty": "  +00011.00 ",
                    "buy_amt": "  +00012.00 ",
                    "sell_amt": "  +00013.00 ",
                    "frgn_stk_inq_amt": "  +00014.00 ",
                    "frgn_stk_outq_amt": "  +00015.00 ",
                    "cmsn_tax": "  +00016.00 ",
                    "dvid_amt": "  +00017.00 ",
                    "crnc_code": "  +00018.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21730(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21730");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21731_contract() {
    let wire_request = json!({"from": "202610", "to": "202610", "stex_tp": "NA", "stk_cd": "AAPL"});
    let request: Usa21731Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "frst_base_dt": "  +0002.00 ",
                    "last_base_dt": "  +0003.00 ",
                    "pl_amt": "  +0004.00 ",
                    "prft_rt": "  +0005.00 ",
                    "frst_stk_evlta": "  +0006.00 ",
                    "last_stk_evlta": "  +0007.00 ",
                    "frst_qty": "  +0008.00 ",
                    "last_qty": "  +0009.00 ",
                    "buy_qty": "  +00010.00 ",
                    "sell_qty": "  +00011.00 ",
                    "buy_amt": "  +00012.00 ",
                    "sell_amt": "  +00013.00 ",
                    "frgn_stk_inq_amt": "  +00014.00 ",
                    "frgn_stk_outq_amt": "  +00015.00 ",
                    "cmsn_tax": "  +00016.00 ",
                    "dvid_amt": "  +00017.00 ",
                    "crnc_code": "  +00018.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21731(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21731");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_usa21732_contract() {
    let wire_request = json!({"from": "2026", "to": "2026", "stex_tp": "NA", "stk_cd": "AAPL"});
    let request: Usa21732Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stk_cd": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "frst_base_dt": "  +0002.00 ",
                    "last_base_dt": "  +0003.00 ",
                    "pl_amt": "  +0004.00 ",
                    "prft_rt": "  +0005.00 ",
                    "frst_stk_evlta": "  +0006.00 ",
                    "last_stk_evlta": "  +0007.00 ",
                    "frst_qty": "  +0008.00 ",
                    "last_qty": "  +0009.00 ",
                    "buy_qty": "  +00010.00 ",
                    "sell_qty": "  +00011.00 ",
                    "buy_amt": "  +00012.00 ",
                    "sell_amt": "  +00013.00 ",
                    "frgn_stk_inq_amt": "  +00014.00 ",
                    "frgn_stk_outq_amt": "  +00015.00 ",
                    "cmsn_tax": "  +00016.00 ",
                    "dvid_amt": "  +00017.00 ",
                    "crnc_code": "  +00018.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .usa21732(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "usa21732");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.from = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.from = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = " ".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21050_contract() {
    let wire_request =
        json!({"ord_dt": "20261012", "slby_tp": "0", "stex_tp": "ND", "stk_cd": "AAPL"});
    let request: Ust21050Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "ord_cntr_tp": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "orig_ord_no": "  +0002.00 ",
                    "stex_nm": "  +0003.00 ",
                    "crnc_code": "  +0004.00 ",
                    "stk_cd": "  +0005.00 ",
                    "frgn_stk_nm": "  +0006.00 ",
                    "frgn_trde_tp": "  +0007.00 ",
                    "frgn_trde_nm": "  +0008.00 ",
                    "slby_tp": "  +0009.00 ",
                    "slby_tp_nm": "  +00010.00 ",
                    "ord_qty": "  +00011.00 ",
                    "ord_uv": "  +00012.00 ",
                    "stop_pric": "  +00013.00 ",
                    "cntr_qty": "  +00014.00 ",
                    "cntr_uv": "  +00015.00 ",
                    "mdfy_qty": "  +00016.00 ",
                    "mdfy_uv": "  +00017.00 ",
                    "cncl_qty": "  +00018.00 ",
                    "ord_remnq": "  +00019.00 ",
                    "ord_time": "  +00020.00 ",
                    "ord_resp_time": "  +00021.00 ",
                    "ord_stat": "  +00022.00 ",
                    "rsrv_tp": "  +00023.00 ",
                    "natn_nm": "  +00024.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21050(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21050");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("X".repeat(3));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21070_contract() {
    let wire_request = json!({"stex_tp": "ND", "stk_cd": "AAPL"});
    let request: Ust21070Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "crnc_code": "  -0000.00 ",
            "tot_evlt_amt": "  -0001.00 ",
            "tot_prch_amt": "  -0002.00 ",
            "tot_pl_amt": "  -0003.00 ",
            "tot_pl_rt": "  -0004.00 ",
            "tdy_book_amt": "  -0005.00 ",
            "tdy_pl_amt": "  -0006.00 ",
            "tdy_pl_rt": "  -0007.00 ",
            "tot_evlt_amt_krw": "  -0008.00 ",
            "tot_prch_amt_krw": "  -0009.00 ",
            "tot_pl_amt_krw": "  -00010.00 ",
            "tdy_book_amt_krw": "  -00011.00 ",
            "tdy_pl_amt_krw": "  -00012.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "crnc_code": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "frgn_stk_nm": "  +0003.00 ",
                    "qty": "  +0004.00 ",
                    "poss_qty": "  +0005.00 ",
                    "sell_alowq": "  +0006.00 ",
                    "pred_cntr_sellq": "  +0007.00 ",
                    "pred_cntr_buyq": "  +0008.00 ",
                    "tdy_cntr_sellq": "  +0009.00 ",
                    "tdy_cntr_buyq": "  +00010.00 ",
                    "frgn_stk_book_uv": "  +00011.00 ",
                    "now_pric": "  +00012.00 ",
                    "evlt_amt": "  +00013.00 ",
                    "pl_amt": "  +00014.00 ",
                    "pl_rt": "  +00015.00 ",
                    "evlt_amt_krw": "  +00016.00 ",
                    "pl_amt_krw": "  +00017.00 ",
                    "natn_nm": "  +00018.00 ",
                    "exch_rate": "  +00019.00 ",
                    "frgn_stk_book_uv_krw": "  +00020.00 ",
                    "now_pric_krw": "  +00021.00 ",
                    "frgn_stk_book_amt": "  +00022.00 ",
                    "frgn_stk_book_amt_krw": "  +00023.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21070(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21070");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stex_tp = Some("X".repeat(3));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21110_contract() {
    let wire_request = json!({});
    let request: Ust21110Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "krw_entra": "  -0000.00 ",
            "ch_uncla": "  -0001.00 ",
            "etc_loana": "  -0002.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "crnc_code": "  +0000.00 ",
                    "crnc_nm": "  +0001.00 ",
                    "fc_entra": "  +0002.00 ",
                    "fc_pymn_alowa": "  +0003.00 ",
                    "futr_repl_profa": "  +0004.00 ",
                    "fc_booka": "  +0005.00 ",
                    "fc_ord_alowa": "  +0006.00 ",
                    "futr_profa_booka": "  +0007.00 ",
                    "fc_ch_uncla": "  +0008.00 ",
                    "fc_etc_loana": "  +0009.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21110(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21110");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_ust21111_contract() {
    let wire_request = json!({});
    let request: Ust21111Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(r#"{"rgst_abnd_tp": "  -0000.00 ", "krw_pymn_alow_amt": "  -0001.00 ", "krw_repl_abnd_amt": "  -0002.00 ", "return_code": 0, "unreviewed_marker": "preserved"}"#).unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21111(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21111");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_ust21120_contract() {
    let wire_request = json!({"cmsn_incl_tp": "0", "exrt_tp": "0"});
    let request: Ust21120Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "won_entr": "  -0000.00 ",
            "aset_evlt_amt": "  -0001.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "crnc_code": "  +0000.00 ",
                    "fx_entr": "  +0001.00 ",
                    "evlt_amt": "  +0002.00 ",
                    "crnc_rt": "  +0003.00 ",
                    "chg_entr": "  +0004.00 ",
                    "chg_evlt_amt": "  +0005.00 ",
                    "evlt_amt_wght": "  +0006.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21120(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21120");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.cmsn_incl_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.cmsn_incl_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.exrt_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.exrt_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21121_contract() {
    let wire_request = json!({"cmsn_incl_tp": "0", "exrt_tp": "0"});
    let request: Ust21121Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "natn_nm": "  +0000.00 ",
                    "stex_nm": "  +0001.00 ",
                    "crnc_code": "  +0002.00 ",
                    "crnc_nm": "  +0003.00 ",
                    "evlt_amt": "  +0004.00 ",
                    "chg_evlt_amt": "  +0005.00 ",
                    "pl_rt": "  +0006.00 ",
                    "pl_amt": "  +0007.00 ",
                    "chg_profit_amt": "  +0008.00 ",
                    "evlt_amt_wght": "  +0009.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21121(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21121");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.cmsn_incl_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.cmsn_incl_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.exrt_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.exrt_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21131_contract() {
    let wire_request = json!({"base_dt": "20261012"});
    let request: Ust21131Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "natn_nm": "  +0000.00 ",
                    "stex_nm": "  +0001.00 ",
                    "crnc_code": "  +0002.00 ",
                    "crnc_nm": "  +0003.00 ",
                    "evlt_amt": "  +0004.00 ",
                    "chg_evlt_amt": "  +0005.00 ",
                    "pl_rt": "  +0006.00 ",
                    "pl_amt": "  +0007.00 ",
                    "chg_profit_amt": "  +0008.00 ",
                    "evlt_amt_wght": "  +0009.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21131(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21131");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.base_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21132_contract() {
    let wire_request = json!({"base_dt": "20261012"});
    let request: Ust21132Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "won_entr": "  -0000.00 ",
            "tot_evlt_amt": "  -0001.00 ",
            "book_amt": "  -0002.00 ",
            "tot_pl": "  -0003.00 ",
            "tot_pl_rt": "  -0004.00 ",
            "aset_evlt_amt": "  -0005.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "crnc_code": "  +0000.00 ",
                    "fx_entr": "  +0001.00 ",
                    "evlt_amt": "  +0002.00 ",
                    "crnc_rt": "  +0003.00 ",
                    "chg_entr": "  +0004.00 ",
                    "chg_evlt_amt": "  +0005.00 ",
                    "evlt_amt_wght": "  +0006.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21132(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21132");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.base_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21150_contract() {
    let wire_request = json!({"ord_dt": "20261012", "query_tp": "1", "slby_tp": "0", "stex_tp": "ND", "stk_cd": "AAPL", "oppo_trde_tp": "%", "fr_ord_no": "1"});
    let request: Ust21150Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "ord_no": "  +0000.00 ",
                    "crnc_code": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "isin_code": "  +0003.00 ",
                    "frgn_trde_tp": "  +0004.00 ",
                    "ord_qty": "  +0005.00 ",
                    "cntr_qty": "  +0006.00 ",
                    "mdfy_qty": "  +0007.00 ",
                    "cncl_qty": "  +0008.00 ",
                    "frgn_msg_code": "  +0009.00 ",
                    "rsrv_tp": "  +00010.00 ",
                    "oppo_trde_tp_nm": "  +00011.00 ",
                    "comm_ord_tp_nm": "  +00012.00 ",
                    "ord_time": "  +00013.00 ",
                    "crnc_nm": "  +00014.00 ",
                    "stex_nm": "  +00015.00 ",
                    "frgn_stk_nm": "  +00016.00 ",
                    "slby_tp_nm": "  +00017.00 ",
                    "ord_uv": "  +00018.00 ",
                    "stop_pric": "  +00019.00 ",
                    "cntr_uv": "  +00020.00 ",
                    "mdfy_uv": "  +00021.00 ",
                    "ord_remnq": "  +00022.00 ",
                    "ord_stat_nm": "  +00023.00 ",
                    "text1": "  +00024.00 ",
                    "inpt_chnl_tp": "  +00025.00 ",
                    "ord_resp_time": "  +00026.00 ",
                    "cntr_time": "  +00027.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21150(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21150");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.ord_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.ord_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.query_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.query_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.query_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("X".repeat(3));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.oppo_trde_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.oppo_trde_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_ord_no = Some("X".repeat(10));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21160_contract() {
    let wire_request = json!({});
    let request: Ust21160Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "won_entr": "  -0000.00 ",
            "won_dfr_amt": "  -0001.00 ",
            "won_etc_loana": "  -0002.00 ",
            "krw_ord_set_amt": "  -0003.00 ",
            "usd_exch_rate": "  -0004.00 ",
            "d0_setl_dt": "  -0005.00 ",
            "d0_won_conv_alow_ch": "  -0006.00 ",
            "d0_usd_fx_entr": "  -0007.00 ",
            "d1_setl_dt": "  -0008.00 ",
            "d1_won_conv_alow_ch": "  -0009.00 ",
            "d1_usd_fx_entr": "  -00010.00 ",
            "d1_usd_exct_amt": "  -00011.00 ",
            "d1_usd_buy_excta": "  -00012.00 ",
            "d1_usd_sell_excta": "  -00013.00 ",
            "d2_setl_dt": "  -00014.00 ",
            "d2_won_conv_alow_ch": "  -00015.00 ",
            "d2_usd_fx_entr": "  -00016.00 ",
            "d2_usd_exct_amt": "  -00017.00 ",
            "d2_usd_buy_excta": "  -00018.00 ",
            "d2_usd_sell_excta": "  -00019.00 ",
            "d3_setl_dt": "  -00020.00 ",
            "d3_won_conv_alow_ch": "  -00021.00 ",
            "d3_usd_fx_entr": "  -00022.00 ",
            "d3_usd_exct_amt": "  -00023.00 ",
            "d3_usd_buy_excta": "  -00024.00 ",
            "d3_usd_sell_excta": "  -00025.00 ",
            "d4_setl_dt": "  -00026.00 ",
            "d4_won_conv_alow_ch": "  -00027.00 ",
            "d4_usd_fx_entr": "  -00028.00 ",
            "d4_usd_exct_amt": "  -00029.00 ",
            "d4_usd_buy_excta": "  -00030.00 ",
            "d4_usd_sell_excta": "  -00031.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21160(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21160");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
}

#[tokio::test]
async fn mock_ust21170_contract() {
    let wire_request = json!({"fc_krw_tp": "0"});
    let request: Ust21170Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "crnc_code": "  -0000.00 ",
            "tot_evlt_amt": "  -0001.00 ",
            "tot_prch_amt": "  -0002.00 ",
            "tot_pl_amt": "  -0003.00 ",
            "tot_pl_rt": "  -0004.00 ",
            "tdy_pl_amt": "  -0005.00 ",
            "prsm_aseta_krw": "  -0006.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "crnc_code": "  +0001.00 ",
                    "stk_cd": "  +0002.00 ",
                    "frgn_stk_nm": "  +0003.00 ",
                    "tdy_pl_amt": "  +0004.00 ",
                    "evlt_pl_amt": "  +0005.00 ",
                    "evlt_pl_rt": "  +0006.00 ",
                    "prch_uv": "  +0007.00 ",
                    "poss_qty": "  +0008.00 ",
                    "sell_alowq": "  +0009.00 ",
                    "now_pric": "  +00010.00 ",
                    "prch_amt": "  +00011.00 ",
                    "evlt_amt": "  +00012.00 ",
                    "prch_exrt": "  +00013.00 ",
                    "sell_exrt": "  +00014.00 ",
                    "krw_chg_dfrn_pl_amt": "  +00015.00 ",
                    "krw_chg_pl_amt": "  +00016.00 ",
                    "evlt_amt_cmsn": "  +00017.00 ",
                    "evlt_amt_tax": "  +00018.00 ",
                    "natn_nm": "  +00019.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21170(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21170");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.fc_krw_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21180_contract() {
    let wire_request = json!({"strt_dt": "20261012", "end_dt": "20261012", "slby_tp": "0", "stex_tp": "ND", "stk_cd": "AAPL", "oppo_trde_tp": "%"});
    let request: Ust21180Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "ord_dt": "  +0000.00 ",
                    "ord_no": "  +0001.00 ",
                    "crnc_code": "  +0002.00 ",
                    "stk_cd": "  +0003.00 ",
                    "trde_tp": "  +0004.00 ",
                    "ord_qty": "  +0005.00 ",
                    "cntr_qty": "  +0006.00 ",
                    "mdfy_qty": "  +0007.00 ",
                    "cncl_qty": "  +0008.00 ",
                    "rsrv_tp": "  +0009.00 ",
                    "oppo_trde_tp_nm": "  +00010.00 ",
                    "inpt_chnl_tp_nm": "  +00011.00 ",
                    "ord_time": "  +00012.00 ",
                    "crnc_nm": "  +00013.00 ",
                    "stex_nm": "  +00014.00 ",
                    "stk_nm": "  +00015.00 ",
                    "slby_tp_nm": "  +00016.00 ",
                    "ord_uv": "  +00017.00 ",
                    "stop_pric": "  +00018.00 ",
                    "cntr_uv": "  +00019.00 ",
                    "mdfy_uv": "  +00020.00 ",
                    "ord_remnq": "  +00021.00 ",
                    "cntr_amt": "  +00022.00 ",
                    "comm_ord_tp_nm": "  +00023.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21180(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21180");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.oppo_trde_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.oppo_trde_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21510_contract() {
    let wire_request = json!({"slby_tp": "0", "stex_tp": "1", "stk_cd": "AAPL"});
    let request: Ust21510Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "ord_no": "  +0000.00 ",
                    "orig_ord_no": "  +0001.00 ",
                    "stex_nm": "  +0002.00 ",
                    "crnc_code": "  +0003.00 ",
                    "stk_cd": "  +0004.00 ",
                    "frgn_stk_nm": "  +0005.00 ",
                    "frgn_trde_tp": "  +0006.00 ",
                    "frgn_trde_nm": "  +0007.00 ",
                    "slby_tp": "  +0008.00 ",
                    "slby_tp_nm": "  +0009.00 ",
                    "ord_qty": "  +00010.00 ",
                    "ord_uv": "  +00011.00 ",
                    "stop_pric": "  +00012.00 ",
                    "cntr_qty": "  +00013.00 ",
                    "cntr_uv": "  +00014.00 ",
                    "cnfm_qty": "  +00015.00 ",
                    "ord_remnq": "  +00016.00 ",
                    "ord_time": "  +00017.00 ",
                    "ord_resp_time": "  +00018.00 ",
                    "ord_stat": "  +00019.00 ",
                    "rsrv_tp": "  +00020.00 ",
                    "natn_nm": "  +00021.00 ",
                    "cntr_time": "  +00022.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21510(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21510");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.slby_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.slby_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("X".repeat(3));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21530_contract() {
    let wire_request = json!({"strt_dt": "20261012", "end_dt": "20261012", "fc_krw_tp": "0"});
    let request: Ust21530Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_sell_amt": "  -0000.00 ",
            "tot_buy_amt": "  -0001.00 ",
            "tot_cmsn_tax": "  -0002.00 ",
            "tot_exct_amt": "  -0003.00 ",
            "tot_pl_amt": "  -0004.00 ",
            "tot_pl_rt": "  -0005.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "sell_dt": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "frgn_stk_nm": "  +0002.00 ",
                    "sell_qty": "  +0003.00 ",
                    "avg_buy_uv": "  +0004.00 ",
                    "buy_amt": "  +0005.00 ",
                    "avg_sell_uv": "  +0006.00 ",
                    "sell_amt": "  +0007.00 ",
                    "cmsn_tax": "  +0008.00 ",
                    "pl_amt": "  +0009.00 ",
                    "pl_rt": "  +00010.00 ",
                    "prch_exrt": "  +00011.00 ",
                    "sell_exrt": "  +00012.00 ",
                    "krw_chg_dfrn_pl_amt": "  +00013.00 ",
                    "krw_chg_pl_amt": "  +00014.00 ",
                    "comm_ord_tp": "  +00015.00 ",
                    "stex_nm": "  +00016.00 ",
                    "natn_nm": "  +00017.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21530(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21530");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = Some("X".repeat(2));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = Some("?".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21610_contract() {
    let wire_request = json!({"base_dt": "20261012", "qry_tp": "0", "fc_krw_tp": "0"});
    let request: Ust21610Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_sell_amt": "  -0000.00 ",
            "tot_buy_amt": "  -0001.00 ",
            "tot_cmsn_altx": "  -0002.00 ",
            "tot_exct_amt": "  -0003.00 ",
            "tot_pl_amt": "  -0004.00 ",
            "tot_pl_rt": "  -0005.00 ",
            "exch_rate": "  -0006.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "tdy_avg_buy_uv": "  +0003.00 ",
                    "tdy_buyq": "  +0004.00 ",
                    "tdy_buy_amt": "  +0005.00 ",
                    "tdy_avg_sell_uv": "  +0006.00 ",
                    "tdy_sellq": "  +0007.00 ",
                    "tdy_sell_amt": "  +0008.00 ",
                    "cmsn_altx": "  +0009.00 ",
                    "pl_amt": "  +00010.00 ",
                    "pl_rt": "  +00011.00 ",
                    "bf_prch_uv": "  +00012.00 ",
                    "exch_rate": "  +00013.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21610(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21610");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.base_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.base_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.qry_tp = "?".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21620_contract() {
    let wire_request = json!({"stex_tp": "NA", "stk_cd": "AAPL", "fc_krw_tp": "0"});
    let request: Ust21620Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "slby_tp_nm": "  +0003.00 ",
                    "crnc_code": "  +0004.00 ",
                    "cntr_uv": "  +0005.00 ",
                    "cntr_qty": "  +0006.00 ",
                    "engg_amt": "  +0007.00 ",
                    "cmsn": "  +0008.00 ",
                    "altx": "  +0009.00 ",
                    "exct_amt": "  +00010.00 ",
                    "exch_rate": "  +00011.00 ",
                    "natn_nm": "  +00012.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21620(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21620");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stex_tp = Some("X".repeat(3));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21630_contract() {
    let wire_request = json!({"stex_tp": "ND", "stk_cd": "AAPL", "fc_krw_tp": "0"});
    let request: Ust21630Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_tdy_pl_amt": "  -0000.00 ",
            "tot_tdy_pl_amt_krw": "  -0001.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "stk_cd": "  +0001.00 ",
                    "stk_nm": "  +0002.00 ",
                    "crnc_code": "  +0003.00 ",
                    "cntr_sellq": "  +0004.00 ",
                    "avg_buy_uv": "  +0005.00 ",
                    "cntr_sella": "  +0006.00 ",
                    "tdy_pl_amt": "  +0007.00 ",
                    "pl_rt": "  +0008.00 ",
                    "cmsn": "  +0009.00 ",
                    "altx": "  +00010.00 ",
                    "exch_rate": "  +00011.00 ",
                    "natn_nm": "  +00012.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21630(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21630");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21640_contract() {
    let wire_request =
        json!({"stex_tp": "ND", "stk_cd": "AAPL", "cntr_dt": "20261012", "fc_krw_tp": "0"});
    let request: Ust21640Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_pl_amt": "  -0000.00 ",
            "tot_pl_amt_krw": "  -0001.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "stex_nm": "  +0000.00 ",
                    "stk_nm": "  +0001.00 ",
                    "crnc_code": "  +0002.00 ",
                    "cntr_sellq": "  +0003.00 ",
                    "avg_buy_uv": "  +0004.00 ",
                    "cntr_sella": "  +0005.00 ",
                    "pl_amt": "  +0006.00 ",
                    "pl_rt": "  +0007.00 ",
                    "stk_cd": "  +0008.00 ",
                    "cmsn": "  +0009.00 ",
                    "altx": "  +00010.00 ",
                    "exch_rate": "  +00011.00 ",
                    "natn_nm": "  +00012.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21640(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21640");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.stex_tp = Some("?".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.stk_cd = Some("X".repeat(13));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.cntr_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.cntr_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.cntr_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "X".repeat(2);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fc_krw_tp = "?".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21650_contract() {
    let wire_request = json!({"fr_dt": "20261012", "to_dt": "20261012"});
    let request: Ust21650Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "fr_entr": "  -0000.00 ",
            "fr_dfr_amt": "  -0001.00 ",
            "fr_etc_loana": "  -0002.00 ",
            "fr_fc_entr": "  -0003.00 ",
            "fr_fc_dfr_amt": "  -0004.00 ",
            "fr_fc_etc_loana": "  -0005.00 ",
            "fr_frgn_stk_evltv": "  -0006.00 ",
            "fr_tot_evltv": "  -0007.00 ",
            "to_entr": "  -0008.00 ",
            "to_dfr_amt": "  -0009.00 ",
            "to_etc_loana": "  -00010.00 ",
            "to_fc_entr": "  -00011.00 ",
            "to_fc_dfr_amt": "  -00012.00 ",
            "to_fc_etc_loana": "  -00013.00 ",
            "to_frgn_stk_evltv": "  -00014.00 ",
            "to_tot_evltv": "  -00015.00 ",
            "fc_rcpta": "  -00016.00 ",
            "fc_payma": "  -00017.00 ",
            "chg_rcpta": "  -00018.00 ",
            "chg_payma": "  -00019.00 ",
            "frgn_stk_inqa": "  -00020.00 ",
            "frgn_stk_outqa": "  -00021.00 ",
            "invt_bsamt": "  -00022.00 ",
            "evlt_profit": "  -00023.00 ",
            "profit_rate": "  -00024.00 ",
            "io_bsamt": "  -00025.00 ",
            "tern_rt": "  -00026.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved"
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21650(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21650");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.fr_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.fr_dt = "20261399".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = " ".into();
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = "X".repeat(9);
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.to_dt = "20261399".into();
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21660_contract() {
    let wire_request = json!({"strt_dt": "20261012", "end_dt": "20261012"});
    let request: Ust21660Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_buy_amt": "  -0000.00 ",
            "tot_sell_amt": "  -0001.00 ",
            "tot_pl_amt": "  -0002.00 ",
            "tot_cmsn": "  -0003.00 ",
            "tot_tax": "  -0004.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "trde_dt": "  +0000.00 ",
                    "buy_amt": "  +0001.00 ",
                    "sell_amt": "  +0002.00 ",
                    "pl_amt": "  +0003.00 ",
                    "cmsn": "  +0004.00 ",
                    "tax": "  +0005.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21660(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21660");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("X".repeat(9));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn mock_ust21661_contract() {
    let wire_request = json!({"strt_dt": "202610", "end_dt": "202610"});
    let request: Ust21661Request = serde_json::from_value(wire_request.clone()).unwrap();
    let body: Value = serde_json::from_str(
        r#"{
            "tot_buy_amt": "  -0000.00 ",
            "tot_sell_amt": "  -0001.00 ",
            "tot_pl_amt": "  -0002.00 ",
            "tot_cmsn": "  -0003.00 ",
            "tot_tax": "  -0004.00 ",
            "return_code": 0,
            "unreviewed_marker": "preserved",
            "result_list": [
                {
                    "trde_dt": "  +0000.00 ",
                    "buy_amt": "  +0001.00 ",
                    "sell_amt": "  +0002.00 ",
                    "pl_amt": "  +0003.00 ",
                    "cmsn": "  +0004.00 ",
                    "tax": "  +0005.00 ",
                    "unreviewed_row_marker": "preserved"
                }
            ]
        }"#,
    )
    .unwrap();
    let (client, recorded) = client(body.clone());
    let pages = client
        .ust21661(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(pages[0].typed.extra.len(), 2);
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra.len(),
        1
    );
    let recorded = recorded.lock().unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].api_id, "ust21661");
    assert_eq!(recorded[0].path, "/api/us/acnt");
    assert_eq!(recorded[0].body, wire_request);
    drop(recorded);
    let mut invalid = request.clone();
    invalid.strt_dt = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.strt_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("X".repeat(7));
    assert!(invalid.validate().is_err());
    let mut invalid = request.clone();
    invalid.end_dt = Some("20261399".into());
    assert!(invalid.validate().is_err());
}

// 잘못된 계좌 요청은 전송 전에 거부한다.
#[tokio::test]
async fn invalid_account_request_is_rejected_before_transport() {
    let (client, recorded) = client(json!({"return_code": 0}));
    let request = Ka10077Request { stk_cd: "".into() };
    assert!(
        client
            .ka10077(&Secret::new("synthetic-token".into()), &request)
            .await
            .is_err()
    );
    assert!(recorded.lock().unwrap().is_empty());
}

// 계좌 응답의 누락된 선택 필드는 만들어내지 않는다.
#[tokio::test]
async fn missing_optional_account_fields_are_not_fabricated() {
    let body: Value =
        serde_json::from_str(r#"{"return_code": 0, "unreviewed_marker": "retained"}"#).unwrap();
    let (client, recorded) = client(body.clone());
    let request = Ust21070Request {
        stex_tp: None,
        stk_cd: None,
    };
    let pages = client
        .ust21070(&Secret::new("synthetic-token".into()), &request)
        .await
        .unwrap();
    assert!(pages[0].typed.tot_evlt_amt.is_none());
    assert!(pages[0].typed.result_list.is_none());
    assert_eq!(pages[0].raw, body);
    assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), body);
    assert_eq!(recorded.lock().unwrap()[0].body, json!({}));
}

// 계좌 조회의 달력 기간과 범위 경계를 검증한다.
#[test]
fn account_calendar_periods_and_range_boundaries_are_checked() {
    let mut monthly = Usa21680Request {
        from: "202602".into(),
        to: "202601".into(),
    };
    assert!(monthly.validate().is_err());
    monthly.to = "202612".into();
    assert!(monthly.validate().is_ok());
    monthly.to = "202613".into();
    assert!(monthly.validate().is_err());
    let mut daily = Kt00002Request {
        start_dt: "20240229".into(),
        end_dt: "20240301".into(),
    };
    assert!(daily.validate().is_ok());
    daily.start_dt = "20230229".into();
    assert!(daily.validate().is_err());
    let blank_today = Ka10170Request {
        base_dt: Some("".into()),
        ottks_tp: "1".into(),
        ch_crd_tp: "0".into(),
    };
    assert!(blank_today.validate().is_ok());
}
