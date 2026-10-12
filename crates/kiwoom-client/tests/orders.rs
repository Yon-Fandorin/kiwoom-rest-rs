use async_trait::async_trait;
use kiwoom_client::{Client, orders::*, transport::*};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Default)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<Vec<HttpResponse>>>,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(r);
        if self.responses.lock().unwrap().is_empty() {
            return Err(TransportError);
        }
        Ok(self.responses.lock().unwrap().remove(0))
    }
}

fn client(mock: Mock, writes: WritePolicy) -> Client<Mock> {
    Client::new(
        mock,
        ClientOptions {
            writes,
            min_request_interval: Duration::ZERO,
            retry_base_delay: Duration::ZERO,
            ..Default::default()
        },
    )
    .unwrap()
}

fn token() -> Secret {
    Secret::new("synthetic-only-no-network".into())
}

fn fixture(mock: &Mock, body: Value, status: u16) {
    mock.responses.lock().unwrap().push(HttpResponse {
        status,
        body,
        headers: BTreeMap::new(),
    });
}

macro_rules! write_case {
    ($id:ident, $request:ident, $body:expr, $path:literal, { $($field:ident: $value:literal),+ }) => {
        // 금융 요청은 기본 거부하며 명시적 허용 뒤에만 한 번 전송한다.
        #[tokio::test]
        async fn $id() {
            let body = $body;
            let request: $request = serde_json::from_value(body.clone()).unwrap();
            let mock = Mock::default();

            // 기본 쓰기 정책은 전송 전에 거부한다.
            let denied = client(mock.clone(), WritePolicy::Deny);
            assert!(matches!(
                denied.$id(&token(), &request).await,
                Err(kiwoom_client::ClientError::WritesDisabled)
            ));
            assert!(mock.requests.lock().unwrap().is_empty());

            // 명시적으로 쓰기를 허용한 경우에만 요청과 응답을 확인한다.
            let mut response = json!({"return_code":0,"future":"kept"});
            $(response[stringify!($field)] = json!($value);)+
            fixture(&mock, response, 200);
            let enabled = client(mock.clone(), WritePolicy::ExplicitOptIn);
            let page = enabled.$id(&token(), &request).await.unwrap();
            assert_eq!(page.raw["future"], "kept");
            $(assert_eq!(page.typed.$field.as_deref(), Some($value));)+
            assert_eq!(page.typed.extra["future"], "kept");

            // 허용된 요청이 정확히 한 번 전송되었는지 확인한다.
            let requests = mock.requests.lock().unwrap();
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].body, body);
            assert_eq!(requests[0].api_id, stringify!($id));
            assert_eq!(requests[0].path, $path);
        }
    };
}

write_case!(
    kt10000,
    Kt10000Request,
    json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"100","cond_uv":"50"}),
    "/api/dostk/ordr",
    { ord_no: "0002468", dmst_stex_tp: "KRX" }
);

write_case!(
    kt10001,
    Kt10001Request,
    json!({"dmst_stex_tp":"NXT","stk_cd":"005930","ord_qty":"1","trde_tp":"3"}),
    "/api/dostk/ordr",
    { ord_no: "0002468", dmst_stex_tp: "NXT" }
);

write_case!(
    kt10002,
    Kt10002Request,
    json!({"dmst_stex_tp":"SOR","orig_ord_no":"0000001","stk_cd":"005930","mdfy_qty":"0","mdfy_uv":"100","mdfy_cond_uv":"50"}),
    "/api/dostk/ordr",
    { ord_no: "0002469", base_orig_ord_no: "0002468", mdfy_qty: "+00000000003", dmst_stex_tp: "SOR" }
);

write_case!(
    kt10003,
    Kt10003Request,
    json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","cncl_qty":"0"}),
    "/api/dostk/ordr",
    { ord_no: "0002470", base_orig_ord_no: "0002468", cncl_qty: "+00000000002" }
);

write_case!(
    kt50000,
    Kt50000Request,
    json!({"stk_cd":"M04020000","ord_qty":"1","ord_uv":"100","trde_tp":"00"}),
    "/api/dostk/ordr",
    { ord_no: "0003571" }
);

write_case!(
    kt50001,
    Kt50001Request,
    json!({"stk_cd":"M04020100","ord_qty":"1","ord_uv":"100","trde_tp":"20"}),
    "/api/dostk/ordr",
    { ord_no: "0003572" }
);

write_case!(
    kt50002,
    Kt50002Request,
    json!({"stk_cd":"M04020000","orig_ord_no":"0000001","mdfy_qty":"0","mdfy_uv":"100"}),
    "/api/dostk/ordr",
    { ord_no: "0002469", base_orig_ord_no: "0002468", mdfy_qty: "+00000000003" }
);

write_case!(
    kt50003,
    Kt50003Request,
    json!({"stk_cd":"M04020100","orig_ord_no":"0000001","cncl_qty":"0"}),
    "/api/dostk/ordr",
    { ord_no: "0002470", base_orig_ord_no: "0002468", cncl_qty: "+00000000002" }
);

write_case!(
    kt10006,
    Kt10006Request,
    json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"100"}),
    "/api/dostk/crdordr",
    { ord_no: "0002468", dmst_stex_tp: "KRX" }
);

write_case!(
    kt10007,
    Kt10007Request,
    json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"100","crd_deal_tp":"33","crd_loan_dt":"20261001"}),
    "/api/dostk/crdordr",
    { ord_no: "0002468", dmst_stex_tp: "KRX" }
);

write_case!(
    kt10008,
    Kt10008Request,
    json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","mdfy_qty":"0","mdfy_uv":"100"}),
    "/api/dostk/crdordr",
    { ord_no: "0002469", base_orig_ord_no: "0002468", mdfy_qty: "+00000000003", dmst_stex_tp: "SOR" }
);

write_case!(
    kt10009,
    Kt10009Request,
    json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","cncl_qty":"0"}),
    "/api/dostk/crdordr",
    { ord_no: "0002470", base_orig_ord_no: "0002468", cncl_qty: "+00000000002" }
);

write_case!(
    ust20000,
    Ust20000Request,
    json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00","ord_uv":"1.25"}),
    "/api/us/ordr",
    { stk_nm: "합성 주식", ord_no: "000024681", fc_entra: "00001234.50", tdy_rebuy_useda: "00000012.50", pred_rebuy_useda: "00000006.25", trst_prof_ch: "00000003.75" }
);

write_case!(
    ust20001,
    Ust20001Request,
    json!({"stex_tp":"NY","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"34","ord_uv":"1.25","stop_pric":"1.20"}),
    "/api/us/ordr",
    { stk_nm: "합성 주식", ord_no: "000024682", poss_qty: "000000000005", tdy_resel_usedq: "000000000002", pred_resel_usedq: "000000000001" }
);

write_case!(
    ust20002,
    Ust20002Request,
    json!({"orig_ord_no":"000000001","stex_tp":"NA","stk_cd":"SYNTH","mdfy_uv":"1.25","stop_pric":"1.2"}),
    "/api/us/ordr",
    { stk_nm: "합성 주식", ord_no: "000024683", fc_entra: "00001234.50", tdy_rebuy_useda: "00000012.50", pred_rebuy_useda: "00000006.25", trst_prof_ch: "00000003.75", mdfy_ord_qty: "000000000003" }
);

write_case!(
    ust20003,
    Ust20003Request,
    json!({"orig_ord_no":"000000001","stex_tp":"ND","stk_cd":"SYNTH"}),
    "/api/us/ordr",
    { stk_nm: "합성 주식", ord_no: "000024684", cncl_ord_qty: "000000000002" }
);

write_case!(
    ust21200,
    Ust21200Request,
    json!({"rsrv_ord_tp":"2","rsrv_strt_dt":"20261020","rsrv_end_dt":"20261021","stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00","ord_uv":"1.25"}),
    "/api/us/ordr",
    { rsrv_ord_no: "000000024681", frcs_dt: "20261020" }
);

write_case!(
    ust21201,
    Ust21201Request,
    json!({"rsrv_ord_tp":"1","stex_tp":"NY","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"35","stop_pric":"1.20"}),
    "/api/us/ordr",
    { rsrv_ord_no: "000000024682", frcs_dt: "20261021" }
);

write_case!(
    ust31302,
    Ust31302Request,
    json!({"exch_tp":"1","fc_exmn_amt":"100"}),
    "/api/us/exchange",
    { sell_aplc_exrt: "001350.25", buy_aplc_exrt: "001350.75", aplc_exrt: "001350.50", entra_prerm: "000000000200000", entra_nowrm: "000000000199900", fc_entra_prerm: "000000010.00", fc_entra_nowrm: "000000010.07", krw_exmn_amt: "000000000000100", sell_fc_amt: "00000000000100.00", buy_fc_amt: "00000000000000.07" }
);

// 명시적 쓰기 허용에서 예약주문 취소의 미지 응답 필드와 전송 본문을 검증한다.
#[tokio::test]
async fn ust21203() {
    let body =
        json!({"rsrv_dt":"20261020","rsrv_ord_no":"000000000001","stex_tp":"ND","stk_cd":"SYNTH"});
    let r: Ust21203Request = serde_json::from_value(body.clone()).unwrap();
    let m = Mock::default();
    assert!(matches!(
        client(m.clone(), WritePolicy::Deny)
            .ust21203(&token(), &r)
            .await,
        Err(kiwoom_client::ClientError::WritesDisabled)
    ));
    assert!(m.requests.lock().unwrap().is_empty());
    fixture(
        &m,
        json!({"return_code":0,"filler":" ","future":"retained"}),
        200,
    );
    let p = client(m.clone(), WritePolicy::ExplicitOptIn)
        .ust21203(&token(), &r)
        .await
        .unwrap();
    assert_eq!(p.typed.filler.as_deref(), Some(" "));
    assert_eq!(p.raw["future"], "retained");
    assert_eq!(p.typed.extra["future"], "retained");
    let sent = m.requests.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].body, body);
    assert_eq!(sent[0].path, "/api/us/ordr");
    assert_eq!(sent[0].api_id, "ust21203");
}

// 주문 조회는 읽기 정책에서 문서의 요청과 응답 필드를 사용한다.
#[tokio::test]
async fn read_only_order_queries() {
    let m = Mock::default();
    let c = client(m.clone(), WritePolicy::Deny);
    let r: Ust21204Request =
        serde_json::from_value(json!({"rsrv_ord_tp":"0","ord_send_dt":"20261020"})).unwrap();
    fixture(
        &m,
        json!({"return_code":0,"result_list":[{"rsrv_dt":"20261020","rsrv_ord_no":"000000024681","ord_qty":"000000000003","ord_uv":"+001.25","rsrv_strt_dt":"20261020","rsrv_end_dt":"20261021","future":"kept"}]}),
        200,
    );
    let p = c.ust21204(&token(), &r).await.unwrap();
    assert_eq!(
        p[0].typed.result_list.as_ref().unwrap()[0]
            .ord_uv
            .as_deref(),
        Some("+001.25")
    );
    assert_eq!(
        m.requests.lock().unwrap()[0].body,
        json!({"rsrv_ord_tp":"0","ord_send_dt":"20261020"})
    );
    let row = &p[0].typed.result_list.as_ref().unwrap()[0];
    assert_eq!(row.rsrv_dt.as_deref(), Some("20261020"));
    assert_eq!(row.rsrv_ord_no.as_deref(), Some("000000024681"));
    assert_eq!(row.ord_qty.as_deref(), Some("000000000003"));
    assert_eq!(row.rsrv_strt_dt.as_deref(), Some("20261020"));
    assert_eq!(row.rsrv_end_dt.as_deref(), Some("20261021"));
    assert_eq!(row.extra["future"], "kept");
    assert_eq!(p[0].raw["result_list"][0]["future"], "kept");
    {
        let sent = m.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].path, "/api/us/ordr");
        assert_eq!(sent[0].api_id, "ust21204");
    }
    let r:Ust21205Request=serde_json::from_value(json!({"fr_rsrv_dt":"20261001","to_rsrv_dt":"20261020","rsrv_ord_tp":"4","rsrv_cncl_yn":"N","rsrv_proc_tp":"%","slby_tp":"0","base_dt_tp":"1","stex_tp":"ND","stk_cd":"SYNTH"})).unwrap();
    fixture(
        &m,
        json!({"return_code":0,"result_list":[{"rsrv_ord_no":"000000024682","ord_qty":"000000000002","ord_uv":"001.25","stop_pric":"001.20","rsrv_cncl_yn":"N","proc_tp":"0","future":"kept"}]}),
        200,
    );
    let p = c.ust21205(&token(), &r).await.unwrap();
    assert_eq!(
        p[0].typed.result_list.as_ref().unwrap()[0]
            .stop_pric
            .as_deref(),
        Some("001.20")
    );
    let row = &p[0].typed.result_list.as_ref().unwrap()[0];
    assert_eq!(row.rsrv_ord_no.as_deref(), Some("000000024682"));
    assert_eq!(row.ord_qty.as_deref(), Some("000000000002"));
    assert_eq!(row.ord_uv.as_deref(), Some("001.25"));
    assert_eq!(row.rsrv_cncl_yn.as_deref(), Some("N"));
    assert_eq!(row.proc_tp.as_deref(), Some("0"));
    assert_eq!(row.extra["future"], "kept");
    {
        let sent = m.requests.lock().unwrap();
        assert_eq!(sent.len(), 2);
        assert_eq!(
            sent[1].body,
            json!({"fr_rsrv_dt":"20261001","to_rsrv_dt":"20261020","rsrv_ord_tp":"4","rsrv_cncl_yn":"N","rsrv_proc_tp":"%","slby_tp":"0","base_dt_tp":"1","stex_tp":"ND","stk_cd":"SYNTH"})
        );
        assert_eq!(sent[1].path, "/api/us/ordr");
        assert_eq!(sent[1].api_id, "ust21205");
    }
    let r: Ust31490Request =
        serde_json::from_value(json!({"stk_cd":"SYNTH","uv":"1.25","stex_tp":"ND"})).unwrap();
    fixture(
        &m,
        json!({"return_code":0,"stk_profa_rt":"50%","ord_alowq_50":"000000000001","ord_alowa_50":"001234.50","fc_entra":"000100.25","krw_ord_rqst_yn":"N","crnc_code":"USD","future":"kept"}),
        200,
    );
    let p = c.ust31490(&token(), &r).await.unwrap();
    assert_eq!(p[0].typed.stk_profa_rt.as_deref(), Some("50%"));
    assert_eq!(p[0].typed.ord_alowq_50.as_deref(), Some("000000000001"));
    assert_eq!(p[0].typed.ord_alowa_50.as_deref(), Some("001234.50"));
    assert_eq!(p[0].typed.fc_entra.as_deref(), Some("000100.25"));
    assert_eq!(p[0].typed.krw_ord_rqst_yn.as_deref(), Some("N"));
    assert_eq!(p[0].typed.crnc_code.as_deref(), Some("USD"));
    assert_eq!(p[0].typed.extra["future"], "kept");
    let sent = m.requests.lock().unwrap();
    assert_eq!(sent.len(), 3);
    assert_eq!(
        sent[2].body,
        json!({"stk_cd":"SYNTH","uv":"1.25","stex_tp":"ND"})
    );
    assert_eq!(sent[2].api_id, "ust31490");
    assert_eq!(sent[2].path, "/api/us/ordr");
}

// 명시적으로 허용한 금융 요청도 실패하면 자동 재시도하지 않는다.
#[tokio::test]
async fn opted_in_financial_failures_are_never_retried() {
    let r: Ust31302Request =
        serde_json::from_value(json!({"exch_tp":"1","fc_exmn_amt":"100"})).unwrap();
    let m = Mock::default();
    fixture(&m, Value::Null, 503);
    let error = client(m.clone(), WritePolicy::ExplicitOptIn)
        .ust31302(&token(), &r)
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        kiwoom_client::ClientError::Http { status: 503, .. }
    ));
    assert_eq!(m.requests.lock().unwrap().len(), 1);
    let m = Mock::default();
    assert!(matches!(
        client(m.clone(), WritePolicy::ExplicitOptIn)
            .ust31302(&token(), &r)
            .await,
        Err(kiwoom_client::ClientError::Transport)
    ));
    assert_eq!(m.requests.lock().unwrap().len(), 1);
}

macro_rules! rejects_before_io {
    ($name:ident, $method:ident, $request:ident, $bodies:expr) => {
        // 정상 경로의 단일 잘못된 입력은 클라이언트 호출에서 전송 전에 거부합니다.
        #[tokio::test]
        async fn $name() {
            let mock = Mock::default();
            let enabled = client(mock.clone(), WritePolicy::ExplicitOptIn);
            for body in $bodies {
                let request: $request = serde_json::from_value(body.clone()).unwrap();
                assert!(
                    matches!(
                        enabled.$method(&token(), &request).await,
                        Err(kiwoom_client::ClientError::InvalidRequest(_))
                    ),
                    "거부되지 않은 입력: {body}"
                );
                assert!(mock.requests.lock().unwrap().is_empty());
            }
        }
    };
}

rejects_before_io!(
    domestic_quantity_is_rejected_independently_of_exchange,
    kt10000,
    Kt10000Request,
    ["-1", "1.5", "0", "invalid"].map(|qty| json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":qty,"trde_tp":"0","ord_uv":"100"}))
);
rejects_before_io!(
    domestic_supplied_price_must_be_positive,
    kt10000,
    Kt10000Request,
    ["0", "-1", "invalid"].map(|price| json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":price}))
);
rejects_before_io!(
    modification_quantity_rejects_fractional_and_negative,
    kt10002,
    Kt10002Request,
    ["-1", "0.5"].map(|qty| json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","mdfy_qty":qty,"mdfy_uv":"100"}))
);
rejects_before_io!(
    required_modification_price_rejects_zero,
    kt10002,
    Kt10002Request,
    [
        json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","mdfy_qty":"0","mdfy_uv":"0"})
    ]
);
rejects_before_io!(
    cancel_quantity_rejects_negative_fractional_and_overlong,
    kt10003,
    Kt10003Request,
    ["-1", "0.5", "1000000000000"].map(
        |qty| json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"005930","cncl_qty":qty})
    )
);
rejects_before_io!(
    required_text_and_optional_price_length_fail_before_io,
    kt10000,
    Kt10000Request,
    [
        json!({"dmst_stex_tp":"KRX","stk_cd":"","ord_qty":"1","trde_tp":"0"}),
        json!({"dmst_stex_tp":"KRX","stk_cd":" ","ord_qty":"1","trde_tp":"0"}),
        json!({"dmst_stex_tp":"KRX","stk_cd":"ABCDEFGHIJKLM","ord_qty":"1","trde_tp":"0"}),
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"1000000000000"}),
    ]
);
rejects_before_io!(
    live_us_limit_and_stop_conditions_reject_missing_inputs,
    ust20001,
    Ust20001Request,
    [
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00"}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00","ord_uv":""}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"03","ord_uv":"1.25"}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"35"}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"35","stop_pric":""}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"34","ord_uv":"1.25"}),
        json!({"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"35","stop_pric":"0"}),
    ]
);
rejects_before_io!(
    reservation_calendar_and_ordered_range_reject_before_io,
    ust21200,
    Ust21200Request,
    [
        ("20260229", "20260301"),
        ("20261001", "20261301"),
        ("20260431", "20260501"),
        ("20261022", "20261021"),
        ("2026A001", "20261021"),
    ].map(|(start,end)| json!({"rsrv_ord_tp":"2","rsrv_strt_dt":start,"rsrv_end_dt":end,"stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00"}))
);
rejects_before_io!(
    reservation_sell_stop_price_is_required,
    ust21201,
    Ust21201Request,
    [
        json!({"rsrv_ord_tp":"1","stex_tp":"NY","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"35"}),
        json!({"rsrv_ord_tp":"1","stex_tp":"NY","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"34","ord_uv":"1.25","stop_pric":""}),
    ]
);
rejects_before_io!(
    reservation_cancel_rejects_invalid_calendar_date,
    ust21203,
    Ust21203Request,
    [json!({"rsrv_dt":"20260229","rsrv_ord_no":"000000024681","stex_tp":"ND","stk_cd":"SYNTH"})]
);
rejects_before_io!(
    reservation_day_query_rejects_invalid_calendar_date,
    ust21204,
    Ust21204Request,
    [json!({"rsrv_ord_tp":"0","ord_send_dt":"20260229"})]
);
rejects_before_io!(
    reservation_history_query_rejects_invalid_dates_and_reversed_range,
    ust21205,
    Ust21205Request,
    [
        json!({"fr_rsrv_dt":"20260229","to_rsrv_dt":"20260301"}),
        json!({"fr_rsrv_dt":"20261001","to_rsrv_dt":"20260431"}),
        json!({"fr_rsrv_dt":"20261022","to_rsrv_dt":"20261021"}),
    ]
);
rejects_before_io!(
    credit_33_requires_a_real_loan_date_and_99_validates_supplied_date,
    kt10007,
    Kt10007Request,
    [
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","crd_deal_tp":"33"}),
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","crd_deal_tp":"33","crd_loan_dt":""}),
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","crd_deal_tp":"33","crd_loan_dt":"20260229"}),
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","crd_deal_tp":"99","crd_loan_dt":"20260229"}),
    ]
);

// 윤년·동일 시작종료일·선택값 누락과 빈 값은 실제 전송으로 유효 경계를 확인합니다.
#[tokio::test]
async fn reservation_valid_calendar_and_optional_boundaries_are_sent_once() {
    for dates in [
        json!({"rsrv_strt_dt":"20280229","rsrv_end_dt":"20280229"}),
        json!({}),
        json!({"rsrv_strt_dt":"","rsrv_end_dt":"","ord_uv":""}),
    ] {
        let mut body =
            json!({"rsrv_ord_tp":"2","stex_tp":"ND","stk_cd":"SYNTH","ord_qty":"1","trde_tp":"00"});
        body.as_object_mut()
            .unwrap()
            .extend(dates.as_object().unwrap().clone());
        let request: Ust21200Request = serde_json::from_value(body.clone()).unwrap();
        let mock = Mock::default();
        fixture(
            &mock,
            json!({"return_code":0,"rsrv_ord_no":"000000024681"}),
            200,
        );
        client(mock.clone(), WritePolicy::ExplicitOptIn)
            .ust21200(&token(), &request)
            .await
            .unwrap();
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].body, body);
    }
}

// 융자합은 대출일 누락·빈 값이 가능하며, 융자는 유효 윤년 날짜로 한 번 전송합니다.
#[tokio::test]
async fn credit_valid_loan_date_boundaries_are_sent_once() {
    for credit in [
        json!({"crd_deal_tp":"99"}),
        json!({"crd_deal_tp":"99","crd_loan_dt":""}),
        json!({"crd_deal_tp":"33","crd_loan_dt":"20280229"}),
    ] {
        let mut body = json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0"});
        body.as_object_mut()
            .unwrap()
            .extend(credit.as_object().unwrap().clone());
        let request: Kt10007Request = serde_json::from_value(body.clone()).unwrap();
        let mock = Mock::default();
        fixture(&mock, json!({"return_code":0,"ord_no":"0002468"}), 200);
        client(mock.clone(), WritePolicy::ExplicitOptIn)
            .kt10007(&token(), &request)
            .await
            .unwrap();
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].body, body);
    }
}

// 잔량 전체 취소의 0과 수량 필드의 12자리 경계는 전송하고 원래 문자열을 유지합니다.
#[tokio::test]
async fn cancel_zero_and_maximum_field_length_are_sent_once() {
    for qty in ["0", "999999999999"] {
        let body = json!({"dmst_stex_tp":"KRX","orig_ord_no":"0000001","stk_cd":"ABCDEFGHIJKL","cncl_qty":qty});
        let request: Kt10003Request = serde_json::from_value(body.clone()).unwrap();
        let mock = Mock::default();
        fixture(
            &mock,
            json!({"return_code":0,"cncl_qty":"+00000000002"}),
            200,
        );
        client(mock.clone(), WritePolicy::ExplicitOptIn)
            .kt10003(&token(), &request)
            .await
            .unwrap();
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].body, body);
    }
}
