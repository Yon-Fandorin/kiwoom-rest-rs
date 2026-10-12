use async_trait::async_trait;
use kiwoom_client::{products::*, transport::*};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Default)]
struct Mock {
    seen: Arc<Mutex<Vec<HttpRequest>>>,
    body: Value,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.seen.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::new(),
            body: self.body.clone(),
        })
    }
}

fn setup(body: Value) -> (Mock, Client<Mock>) {
    let mock = Mock {
        body,
        ..Default::default()
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

// 공개 HTML 응답 표의 키를 독립적으로 작성한 합성 값으로 검증한다.
// 각 계약 아래 출처의 응답 필드 표를 확인했다 (2026-10-12).
// 아래 기대값은 직렬화 결과나 생산 코드 선언에서 추출하지 않는다.
macro_rules! assert_fields {
    ($row:expr; $($field:ident => $expected:literal),+ $(,)?) => {
        $(assert_eq!($row.$field.as_deref(), Some($expected), stringify!($field));)+
    };
}

macro_rules! contract {
    ($name:ident, $request:ident, $method:ident, $path:literal, $body:expr, $bad:expr, $fixture:expr, $typed:expr) => {
        // 상품 계약의 요청·응답을 보존하고 잘못된 입력은 전송 전에 거부한다.
        #[tokio::test]
        async fn $name() {
            let request: $request = serde_json::from_value($body).unwrap();
            assert_eq!(serde_json::to_value(&request).unwrap(), $body);

            let fixture = $fixture;
            let (mock, client) = setup(fixture.clone());
            let pages = client
                .$method(&Secret::new("offline-products-token".into()), &request)
                .await
                .unwrap();
            assert_eq!(pages.len(), 1);
            ($typed)(&pages[0].typed);
            assert_eq!(pages[0].raw, fixture);
            assert_eq!(serde_json::to_value(&pages[0].typed).unwrap(), fixture);

            {
                let seen = mock.seen.lock().unwrap();
                assert_eq!(seen[0].api_id, stringify!($method));
                assert_eq!(seen[0].path, $path);
                assert_eq!(seen[0].body, $body);
            }

            let invalid: Result<$request, _> = serde_json::from_value($bad);
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
       };
}

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka10048.
contract!(
    ka10048_contract,
    Ka10048Request,
    ka10048,
    "/api/dostk/elw",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"123"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwdaly_snst_ix":[{"iv":"+000123.45","law":"-001","unknown_fid":"preserved","dt":"20261012","delta":"-000103.2500","gam":"-000104.2500","theta":"-000105.2500","vega":"-000106.2500","lp":"-000108.2500"}]}"#).unwrap(),
    |r: &Ka10048Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwdaly_snst_ix.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            dt => "20261012",
            iv => "+000123.45",
            delta => "-000103.2500",
            gam => "-000104.2500",
            theta => "-000105.2500",
            vega => "-000106.2500",
            law => "-001",
            lp => "-000108.2500",
        );
        assert_eq!(
            serde_json::to_value(&x.extra).unwrap(),
            json!({"unknown_fid":"preserved"})
        );
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka10050.
contract!(
    ka10050_contract,
    Ka10050Request,
    ka10050,
    "/api/dostk/elw",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":""}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwsnst_ix_array":[{"cntr_tm":"093001","theta":"-00001","cur_prc":"-000102.2500","elwtheory_pric":"-000103.2500","iv":"-000104.2500","delta":"-000105.2500","gam":"-000106.2500","vega":"-000108.2500","law":"-000109.2500","lp":"-000110.2500"}]}"#).unwrap(),
    |r: &Ka10050Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwsnst_ix_array.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            cntr_tm => "093001",
            cur_prc => "-000102.2500",
            elwtheory_pric => "-000103.2500",
            iv => "-000104.2500",
            delta => "-000105.2500",
            gam => "-000106.2500",
            theta => "-00001",
            vega => "-000108.2500",
            law => "-000109.2500",
            lp => "-000110.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30001.
contract!(
    ka30001_contract,
    Ka30001Request,
    ka30001,
    "/api/dostk/elw",
    json!({"flu_tp":"1","tm_tp":"1","tm":"3","trde_qty_tp":"0","isscomp_cd":"000000000000","bsis_aset_cd":"000000000000","rght_tp":"000","lpcd":"000000000000","trde_end_elwskip":"1"}),
    json!({"flu_tp":"8","tm_tp":"1","tm":"3","trde_qty_tp":"0","isscomp_cd":"000000000000","bsis_aset_cd":"000000000000","rght_tp":"000","lpcd":"000000000000","trde_end_elwskip":"1"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"base_pric_tm":"synthetic","elwpric_jmpflu":[{"jmp_rt":"-001.25","stk_cd":"-000101.2500","rank":"-000102.2500","stk_nm":"-000103.2500","pre_sig":"-000104.2500","pred_pre":"-000105.2500","trde_end_elwbase_pric":"-000106.2500","cur_prc":"-000107.2500","base_pre":"-000108.2500","trde_qty":"-000109.2500"}]}"#).unwrap(),
    |r: &Ka30001Response| {
        assert_fields!(r;
            base_pric_tm => "synthetic",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwpric_jmpflu.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            rank => "-000102.2500",
            stk_nm => "-000103.2500",
            pre_sig => "-000104.2500",
            pred_pre => "-000105.2500",
            trde_end_elwbase_pric => "-000106.2500",
            cur_prc => "-000107.2500",
            base_pre => "-000108.2500",
            trde_qty => "-000109.2500",
            jmp_rt => "-001.25",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30002.
contract!(
    ka30002_contract,
    Ka30002Request,
    ka30002,
    "/api/dostk/elw",
    json!({"isscomp_cd":"003","trde_qty_tp":"0","trde_tp":"1","dt":"5","trde_end_elwskip":"0"}),
    json!({"isscomp_cd":"003","trde_qty_tp":"0","trde_tp":"1","dt":"6","trde_end_elwskip":"0"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"trde_ori_elwnettrde_upper":[{"netprps":"-000020","stk_cd":"-000101.2500","stk_nm":"-000102.2500","stkpc_flu":"-000103.2500","flu_rt":"-000104.2500","trde_qty":"-000105.2500","buy_trde_qty":"-000107.2500","sel_trde_qty":"-000108.2500"}]}"#).unwrap(),
    |r: &Ka30002Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.trde_ori_elwnettrde_upper.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            stk_nm => "-000102.2500",
            stkpc_flu => "-000103.2500",
            flu_rt => "-000104.2500",
            trde_qty => "-000105.2500",
            netprps => "-000020",
            buy_trde_qty => "-000107.2500",
            sel_trde_qty => "-000108.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30003.
contract!(
    ka30003_contract,
    Ka30003Request,
    ka30003,
    "/api/dostk/elw",
    json!({"bsis_aset_cd":"500001","base_dt":"20261012"}),
    json!({"bsis_aset_cd":"500001","base_dt":"20260230"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwlpposs_daly_trnsn":[{"chg_qty":"-5","lprmnd_qty":"000090","dt":"20261012","cur_prc":"-000102.2500","pre_tp":"-000103.2500","pred_pre":"-000104.2500","flu_rt":"-000105.2500","trde_qty":"-000106.2500","trde_prica":"-000107.2500","wght":"-000110.2500"}]}"#).unwrap(),
    |r: &Ka30003Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwlpposs_daly_trnsn.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            dt => "20261012",
            cur_prc => "-000102.2500",
            pre_tp => "-000103.2500",
            pred_pre => "-000104.2500",
            flu_rt => "-000105.2500",
            trde_qty => "-000106.2500",
            trde_prica => "-000107.2500",
            chg_qty => "-5",
            lprmnd_qty => "000090",
            wght => "-000110.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30004.
contract!(
    ka30004_contract,
    Ka30004Request,
    ka30004,
    "/api/dostk/elw",
    json!({"isscomp_cd":"3","bsis_aset_cd":"201","rght_tp":"001","lpcd":"3","trde_end_elwskip":"1"}),
    json!({"isscomp_cd":"3","bsis_aset_cd":"201","rght_tp":"1","lpcd":"3","trde_end_elwskip":"1"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwdispty_rt":[{"dispty_rt":"-0012","theory_pric":"000058","stk_cd":"-000101.2500","isscomp_nm":"-000102.2500","sqnc":"-000103.2500","base_aset_nm":"-000104.2500","rght_tp":"-000105.2500","basis":"-000107.2500","srvive_dys":"-000108.2500","cur_prc":"-000110.2500","pre_tp":"-000111.2500","pred_pre":"-000112.2500","flu_rt":"-000113.2500","trde_qty":"-000114.2500","stk_nm":"-000115.2500"}]}"#).unwrap(),
    |r: &Ka30004Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwdispty_rt.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            isscomp_nm => "-000102.2500",
            sqnc => "-000103.2500",
            base_aset_nm => "-000104.2500",
            rght_tp => "-000105.2500",
            dispty_rt => "-0012",
            basis => "-000107.2500",
            srvive_dys => "-000108.2500",
            theory_pric => "000058",
            cur_prc => "-000110.2500",
            pre_tp => "-000111.2500",
            pred_pre => "-000112.2500",
            flu_rt => "-000113.2500",
            trde_qty => "-000114.2500",
            stk_nm => "-000115.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30005.
contract!(
    ka30005_contract,
    Ka30005Request,
    ka30005,
    "/api/dostk/elw",
    json!({"isscomp_cd":"000000000003","bsis_aset_cd":"201","rght_tp":"1","lpcd":"003","sort_tp":"5"}),
    json!({"isscomp_cd":"003","bsis_aset_cd":"201","rght_tp":"1","lpcd":"003","sort_tp":"5"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwcnd_qry":[{"cfp":"-000.5","xraymont_cntr_qty_profa_100tp":"fixture","bsis_aset_comp_rt_1":"unknown","stk_cd":"-000101.2500","isscomp_nm":"-000102.2500","sqnc":"-000103.2500","base_aset_nm":"-000104.2500","rght_tp":"-000105.2500","expr_dt":"20261231","cur_prc":"-000107.2500","pre_tp":"-000108.2500","pred_pre":"-000109.2500","flu_rt":"-000110.2500","trde_qty":"-000111.2500","trde_qty_pre":"-000112.2500","trde_prica":"-000113.2500","pred_trde_qty":"-000114.2500","sel_bid":"-000115.2500","buy_bid":"-000116.2500","prty":"-000117.2500","gear_rt":"-000118.2500","pl_qutr_rt":"-000119.2500","theory_pric":"-000121.2500","innr_vltl":"-000122.2500","delta":"-000123.2500","lvrg":"-000124.2500","exec_pric":"-000125.2500","cnvt_rt":"-000126.2500","lpposs_rt":"-000127.2500","pl_qutr_pt":"-000128.2500","fin_trde_dt":"20261229","flo_dt":"20260102","lpinitlast_suply_dt":"20261001","stk_nm":"-000132.2500","srvive_dys":"-000133.2500","dispty_rt":"-000134.2500","lpmmcm_nm":"-000135.2500","lpmmcm_nm_1":"-000136.2500","lpmmcm_nm_2":"-000137.2500","xraymont_cntr_qty_arng_trde_tp":"-000138.2500"}]}"#).unwrap(),
    |r: &Ka30005Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwcnd_qry.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            isscomp_nm => "-000102.2500",
            sqnc => "-000103.2500",
            base_aset_nm => "-000104.2500",
            rght_tp => "-000105.2500",
            expr_dt => "20261231",
            cur_prc => "-000107.2500",
            pre_tp => "-000108.2500",
            pred_pre => "-000109.2500",
            flu_rt => "-000110.2500",
            trde_qty => "-000111.2500",
            trde_qty_pre => "-000112.2500",
            trde_prica => "-000113.2500",
            pred_trde_qty => "-000114.2500",
            sel_bid => "-000115.2500",
            buy_bid => "-000116.2500",
            prty => "-000117.2500",
            gear_rt => "-000118.2500",
            pl_qutr_rt => "-000119.2500",
            cfp => "-000.5",
            theory_pric => "-000121.2500",
            innr_vltl => "-000122.2500",
            delta => "-000123.2500",
            lvrg => "-000124.2500",
            exec_pric => "-000125.2500",
            cnvt_rt => "-000126.2500",
            lpposs_rt => "-000127.2500",
            pl_qutr_pt => "-000128.2500",
            fin_trde_dt => "20261229",
            flo_dt => "20260102",
            lpinitlast_suply_dt => "20261001",
            stk_nm => "-000132.2500",
            srvive_dys => "-000133.2500",
            dispty_rt => "-000134.2500",
            lpmmcm_nm => "-000135.2500",
            lpmmcm_nm_1 => "-000136.2500",
            lpmmcm_nm_2 => "-000137.2500",
            xraymont_cntr_qty_arng_trde_tp => "-000138.2500",
            xraymont_cntr_qty_profa_100tp => "fixture",
        );
        assert_eq!(
            serde_json::to_value(&x.extra).unwrap(),
            json!({"bsis_aset_comp_rt_1":"unknown"})
        );
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30009.
contract!(
    ka30009_contract,
    Ka30009Request,
    ka30009,
    "/api/dostk/elw",
    json!({"sort_tp":"1","rght_tp":"001","trde_end_skip":"1"}),
    json!({"sort_tp":"1","rght_tp":"005","trde_end_skip":"1"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwflu_rt_rank":[{"rank":"0001","flu_rt":"+1.00","stk_cd":"-000102.2500","stk_nm":"-000103.2500","cur_prc":"-000104.2500","pre_sig":"-000105.2500","pred_pre":"-000106.2500","sel_req":"-000108.2500","buy_req":"-000109.2500","trde_qty":"-000110.2500","trde_prica":"-000111.2500"}]}"#).unwrap(),
    |r: &Ka30009Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwflu_rt_rank.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            rank => "0001",
            stk_cd => "-000102.2500",
            stk_nm => "-000103.2500",
            cur_prc => "-000104.2500",
            pre_sig => "-000105.2500",
            pred_pre => "-000106.2500",
            flu_rt => "+1.00",
            sel_req => "-000108.2500",
            buy_req => "-000109.2500",
            trde_qty => "-000110.2500",
            trde_prica => "-000111.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30010.
contract!(
    ka30010_contract,
    Ka30010Request,
    ka30010,
    "/api/dostk/elw",
    json!({"sort_tp":"2","rght_tp":"000","trde_end_skip":"0"}),
    json!({"sort_tp":"3","rght_tp":"000","trde_end_skip":"0"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwreq_rank":[{"netprps_req":"-000010","stk_cd":"-000101.2500","rank":"-000102.2500","stk_nm":"-000103.2500","cur_prc":"-000104.2500","pre_sig":"-000105.2500","pred_pre":"-000106.2500","flu_rt":"-000107.2500","trde_qty":"-000108.2500","sel_req":"-000109.2500","buy_req":"-000110.2500","trde_prica":"-000112.2500"}]}"#).unwrap(),
    |r: &Ka30010Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwreq_rank.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            rank => "-000102.2500",
            stk_nm => "-000103.2500",
            cur_prc => "-000104.2500",
            pre_sig => "-000105.2500",
            pred_pre => "-000106.2500",
            flu_rt => "-000107.2500",
            trde_qty => "-000108.2500",
            sel_req => "-000109.2500",
            buy_req => "-000110.2500",
            netprps_req => "-000010",
            trde_prica => "-000112.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30011.
contract!(
    ka30011_contract,
    Ka30011Request,
    ka30011,
    "/api/dostk/elw",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"한글한글한글"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwalacc_rt":[{"alacc_rt":"001.25","stk_cd":"-000101.2500","stk_nm":"-000102.2500","cur_prc":"-000103.2500","pre_sig":"-000104.2500","pred_pre":"-000105.2500","flu_rt":"-000106.2500","acc_trde_qty":"-000107.2500"}]}"#).unwrap(),
    |r: &Ka30011Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.elwalacc_rt.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            stk_nm => "-000102.2500",
            cur_prc => "-000103.2500",
            pre_sig => "-000104.2500",
            pred_pre => "-000105.2500",
            flu_rt => "-000106.2500",
            acc_trde_qty => "-000107.2500",
            alacc_rt => "001.25",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30012.
contract!(
    ka30012_contract,
    Ka30012Request,
    ka30012,
    "/api/dostk/elw",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"5000017"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"elwtheory_pric":"0000123","bsis_aset_comp_rt_5":"033.333","evlt_fnsh_yn":"N","future":"kept","aset_cd":"+000001.4500","cur_prc":"+000002.4500","pred_pre_sig":"+000003.4500","pred_pre":"+000004.4500","flu_rt":"+000005.4500","lpmmcm_nm":"+000006.4500","lpmmcm_nm_1":"+000007.4500","lpmmcm_nm_2":"+000008.4500","elwrght_cntn":"+000009.4500","elwexpr_evlt_pric":"+000010.4500","dispty_rt":"+000012.4500","elwinnr_vltl":"+000013.4500","exp_rght_pric":"+000014.4500","elwpl_qutr_rt":"+000015.4500","elwexec_pric":"+000016.4500","elwcnvt_rt":"+000017.4500","elwcmpn_rt":"+000018.4500","elwpric_rising_part_rt":"+000019.4500","elwrght_type":"+000020.4500","elwsrvive_dys":"+000021.4500","stkcnt":"+000022.4500","elwlpord_pos":"+000023.4500","lpposs_rt":"+000024.4500","lprmnd_qty":"+000025.4500","elwspread":"+000026.4500","elwprty":"+000027.4500","elwgear":"+000028.4500","elwflo_dt":"20260105","elwfin_trde_dt":"20261228","expr_dt":"20261231","exec_dt":"20261230","lpsuply_end_dt":"20261227","elwpay_dt":"20270105","elwinvt_ix_comput":"+000035.4500","elwpay_agnt":"+000036.4500","elwappr_way":"+000037.4500","elwrght_exec_way":"+000038.4500","elwpblicte_orgn":"+000039.4500","dcsn_pay_amt":"+000040.4500","kobarr":"+000041.4500","iv":"+000042.4500","clsprd_end_elwocr":"+000043.4500","bsis_aset_1":"+000044.4500","bsis_aset_comp_rt_1":"+000045.4500","bsis_aset_2":"+000046.4500","bsis_aset_comp_rt_2":"+000047.4500","bsis_aset_3":"+000048.4500","bsis_aset_comp_rt_3":"+000049.4500","bsis_aset_4":"+000050.4500","bsis_aset_comp_rt_4":"+000051.4500","bsis_aset_5":"+000052.4500","fr_dt":"20261008","to_dt":"20261009","fr_tm":"093002","evlt_end_tm":"153001","evlt_pric":"+000058.4500","all_hgst_pric":"+000060.4500","all_lwst_pric":"+000061.4500","imaf_hgst_pric":"+000062.4500","imaf_lwst_pric":"+000063.4500","sndhalf_mrkt_hgst_pric":"+000064.4500","sndhalf_mrkt_lwst_pric":"+000065.4500"}"#).unwrap(),
    |r: &Ka30012Response| {
        assert_fields!(r;
            aset_cd => "+000001.4500",
            cur_prc => "+000002.4500",
            pred_pre_sig => "+000003.4500",
            pred_pre => "+000004.4500",
            flu_rt => "+000005.4500",
            lpmmcm_nm => "+000006.4500",
            lpmmcm_nm_1 => "+000007.4500",
            lpmmcm_nm_2 => "+000008.4500",
            elwrght_cntn => "+000009.4500",
            elwexpr_evlt_pric => "+000010.4500",
            elwtheory_pric => "0000123",
            dispty_rt => "+000012.4500",
            elwinnr_vltl => "+000013.4500",
            exp_rght_pric => "+000014.4500",
            elwpl_qutr_rt => "+000015.4500",
            elwexec_pric => "+000016.4500",
            elwcnvt_rt => "+000017.4500",
            elwcmpn_rt => "+000018.4500",
            elwpric_rising_part_rt => "+000019.4500",
            elwrght_type => "+000020.4500",
            elwsrvive_dys => "+000021.4500",
            stkcnt => "+000022.4500",
            elwlpord_pos => "+000023.4500",
            lpposs_rt => "+000024.4500",
            lprmnd_qty => "+000025.4500",
            elwspread => "+000026.4500",
            elwprty => "+000027.4500",
            elwgear => "+000028.4500",
            elwflo_dt => "20260105",
            elwfin_trde_dt => "20261228",
            expr_dt => "20261231",
            exec_dt => "20261230",
            lpsuply_end_dt => "20261227",
            elwpay_dt => "20270105",
            elwinvt_ix_comput => "+000035.4500",
            elwpay_agnt => "+000036.4500",
            elwappr_way => "+000037.4500",
            elwrght_exec_way => "+000038.4500",
            elwpblicte_orgn => "+000039.4500",
            dcsn_pay_amt => "+000040.4500",
            kobarr => "+000041.4500",
            iv => "+000042.4500",
            clsprd_end_elwocr => "+000043.4500",
            bsis_aset_1 => "+000044.4500",
            bsis_aset_comp_rt_1 => "+000045.4500",
            bsis_aset_2 => "+000046.4500",
            bsis_aset_comp_rt_2 => "+000047.4500",
            bsis_aset_3 => "+000048.4500",
            bsis_aset_comp_rt_3 => "+000049.4500",
            bsis_aset_4 => "+000050.4500",
            bsis_aset_comp_rt_4 => "+000051.4500",
            bsis_aset_5 => "+000052.4500",
            bsis_aset_comp_rt_5 => "033.333",
            fr_dt => "20261008",
            to_dt => "20261009",
            fr_tm => "093002",
            evlt_end_tm => "153001",
            evlt_pric => "+000058.4500",
            evlt_fnsh_yn => "N",
            all_hgst_pric => "+000060.4500",
            all_lwst_pric => "+000061.4500",
            imaf_hgst_pric => "+000062.4500",
            imaf_lwst_pric => "+000063.4500",
            sndhalf_mrkt_hgst_pric => "+000064.4500",
            sndhalf_mrkt_lwst_pric => "+000065.4500",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0,"future":"kept"})
        );
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40001.
contract!(
    ka40001_contract,
    Ka40001Request,
    ka40001,
    "/api/dostk/etf",
    json!({"stk_cd":"500001","etfobjt_idex_cd":"201","dt":"0"}),
    json!({"stk_cd":"500001","etfobjt_idex_cd":"201","dt":"4"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etfprft_rt_lst":[{"etfprft_rt":"-01.50","cntr_prft_rt":"-000102.2500","for_netprps_qty":"-000103.2500","orgn_netprps_qty":"-000104.2500"}]}"#).unwrap(),
    |r: &Ka40001Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etfprft_rt_lst.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            etfprft_rt => "-01.50",
            cntr_prft_rt => "-000102.2500",
            for_netprps_qty => "-000103.2500",
            orgn_netprps_qty => "-000104.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40002.
contract!(
    ka40002_contract,
    Ka40002Request,
    ka40002,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etntxon_type":"fixture ETN type","etftxon_type":"fixture ETF type","stk_nm":"+000001.4500","etfobjt_idex_nm":"+000002.4500","wonju_pric":"+000003.4500"}"#).unwrap(),
    |r: &Ka40002Response| {
        assert_fields!(r;
            stk_nm => "+000001.4500",
            etfobjt_idex_nm => "+000002.4500",
            wonju_pric => "+000003.4500",
            etftxon_type => "fixture ETF type",
            etntxon_type => "fixture ETN type",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40003.
contract!(
    ka40003_contract,
    Ka40003Request,
    ka40003,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etfdaly_trnsn":[{"cntr_dt":"20261012","nav":"-00123.25","cur_prc":"-000102.2500","pre_sig":"-000103.2500","pred_pre":"-000104.2500","pre_rt":"-000105.2500","trde_qty":"-000106.2500","acc_trde_prica":"-000108.2500","navidex_dispty_rt":"-000109.2500","navetfdispty_rt":"-000110.2500","trace_eor_rt":"-000111.2500","trace_cur_prc":"-000112.2500","trace_pred_pre":"-000113.2500","trace_pre_sig":"-000114.2500"}]}"#).unwrap(),
    |r: &Ka40003Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etfdaly_trnsn.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            cntr_dt => "20261012",
            cur_prc => "-000102.2500",
            pre_sig => "-000103.2500",
            pred_pre => "-000104.2500",
            pre_rt => "-000105.2500",
            trde_qty => "-000106.2500",
            nav => "-00123.25",
            acc_trde_prica => "-000108.2500",
            navidex_dispty_rt => "-000109.2500",
            navetfdispty_rt => "-000110.2500",
            trace_eor_rt => "-000111.2500",
            trace_cur_prc => "-000112.2500",
            trace_pred_pre => "-000113.2500",
            trace_pre_sig => "-000114.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40004.
contract!(
    ka40004_contract,
    Ka40004Request,
    ka40004,
    "/api/dostk/etf",
    json!({"txon_type":"0","navpre":"0","mngmcomp":"0000","txon_yn":"0","trace_idex":"0","stex_tp":"3"}),
    json!({"txon_type":"0","navpre":"0","mngmcomp":"0000","txon_yn":"0","trace_idex":"1","stex_tp":"3"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etfall_mrpr":[{"dvid_bf_base":"000005","pred_dvida":"000002","trace_idex":"123","stk_cd":"-000101.2500","stk_cls":"-000102.2500","stk_nm":"-000103.2500","close_pric":"-000104.2500","pre_sig":"-000105.2500","pred_pre":"-000106.2500","pre_rt":"-000107.2500","trde_qty":"-000108.2500","nav":"-000109.2500","trace_eor_rt":"-000110.2500","txbs":"-000111.2500","trace_idex_nm":"-000114.2500","drng":"-000115.2500","trace_idex_cd":"-000116.2500","trace_flu_rt":"-000118.2500"}]}"#).unwrap(),
    |r: &Ka40004Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etfall_mrpr.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            stk_cls => "-000102.2500",
            stk_nm => "-000103.2500",
            close_pric => "-000104.2500",
            pre_sig => "-000105.2500",
            pred_pre => "-000106.2500",
            pre_rt => "-000107.2500",
            trde_qty => "-000108.2500",
            nav => "-000109.2500",
            trace_eor_rt => "-000110.2500",
            txbs => "-000111.2500",
            dvid_bf_base => "000005",
            pred_dvida => "000002",
            trace_idex_nm => "-000114.2500",
            drng => "-000115.2500",
            trace_idex_cd => "-000116.2500",
            trace_idex => "123",
            trace_flu_rt => "-000118.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40006.
contract!(
    ka40006_contract,
    Ka40006Request,
    ka40006,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"stk_nm":"synthetic ETF","etftisl_trnsn":[{"tm":"093003","trace_idex_pred_pre_sig":"5","close_pric":"-000102.2500","pre_sig":"-000103.2500","pred_pre":"-000104.2500","flu_rt":"-000105.2500","trde_qty":"-000106.2500","nav":"-000107.2500","trde_prica":"-000108.2500","navidex":"-000109.2500","navetf":"-000110.2500","trace":"-000111.2500","trace_idex":"-000112.2500","trace_idex_pred_pre":"-000113.2500"}],"etfobjt_idex_nm":"+000002.4500","wonju_pric":"+000003.4500","etftxon_type":"+000004.4500","etntxon_type":"+000005.4500"}"#).unwrap(),
    |r: &Ka40006Response| {
        assert_fields!(r;
            stk_nm => "synthetic ETF",
            etfobjt_idex_nm => "+000002.4500",
            wonju_pric => "+000003.4500",
            etftxon_type => "+000004.4500",
            etntxon_type => "+000005.4500",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etftisl_trnsn.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            tm => "093003",
            close_pric => "-000102.2500",
            pre_sig => "-000103.2500",
            pred_pre => "-000104.2500",
            flu_rt => "-000105.2500",
            trde_qty => "-000106.2500",
            nav => "-000107.2500",
            trde_prica => "-000108.2500",
            navidex => "-000109.2500",
            navetf => "-000110.2500",
            trace => "-000111.2500",
            trace_idex => "-000112.2500",
            trace_idex_pred_pre => "-000113.2500",
            trace_idex_pred_pre_sig => "5",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40007.
contract!(
    ka40007_contract,
    Ka40007Request,
    ka40007,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etfobjt_idex_cd":"201","etftisl_cntr_array":[{"stex_tp":"NXT","trde_qty":"-00005","cntr_tm":"093001","cur_prc":"-000102.2500","pre_sig":"-000103.2500","pred_pre":"-000104.2500"}],"stk_cls":"+000001.4500","stk_nm":"+000002.4500","etfobjt_idex_nm":"+000003.4500","objt_idex_pre_rt":"+000005.4500","wonju_pric":"+000006.4500"}"#).unwrap(),
    |r: &Ka40007Response| {
        assert_fields!(r;
            stk_cls => "+000001.4500",
            stk_nm => "+000002.4500",
            etfobjt_idex_nm => "+000003.4500",
            etfobjt_idex_cd => "201",
            objt_idex_pre_rt => "+000005.4500",
            wonju_pric => "+000006.4500",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etftisl_cntr_array.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            cntr_tm => "093001",
            cur_prc => "-000102.2500",
            pre_sig => "-000103.2500",
            pred_pre => "-000104.2500",
            trde_qty => "-00005",
            stex_tp => "NXT",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40008.
contract!(
    ka40008_contract,
    Ka40008Request,
    ka40008,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"cur_prc":"+00020","etfnetprps_qty_array":[{"cur_prc_n":"-00019","for_netprps_qty":"-5","dt":"20261012","pre_sig_n":"-000103.2500","pred_pre_n":"-000104.2500","acc_trde_qty":"-000105.2500","orgn_netprps_qty":"-000107.2500"}],"cntr_tm":"093001","pre_sig":"+000003.4500","pred_pre":"+000004.4500","trde_qty":"+000005.4500"}"#).unwrap(),
    |r: &Ka40008Response| {
        assert_fields!(r;
            cntr_tm => "093001",
            cur_prc => "+00020",
            pre_sig => "+000003.4500",
            pred_pre => "+000004.4500",
            trde_qty => "+000005.4500",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etfnetprps_qty_array.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            dt => "20261012",
            cur_prc_n => "-00019",
            pre_sig_n => "-000103.2500",
            pred_pre_n => "-000104.2500",
            acc_trde_qty => "-000105.2500",
            for_netprps_qty => "-5",
            orgn_netprps_qty => "-000107.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40009.
contract!(
    ka40009_contract,
    Ka40009Request,
    ka40009,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etfnavarray":[{"base_pric":"000020","stkcnt":"000100","nav":"-000101.2500","navpred_pre":"-000102.2500","navflu_rt":"-000103.2500","trace_eor_rt":"-000104.2500","dispty_rt":"-000105.2500","for_rmnd_qty":"-000108.2500","repl_pric":"-000109.2500","conv_pric":"-000110.2500","drstk":"-000111.2500","wonju_pric":"-000112.2500"}]}"#).unwrap(),
    |r: &Ka40009Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etfnavarray.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            nav => "-000101.2500",
            navpred_pre => "-000102.2500",
            navflu_rt => "-000103.2500",
            trace_eor_rt => "-000104.2500",
            dispty_rt => "-000105.2500",
            stkcnt => "000100",
            base_pric => "000020",
            for_rmnd_qty => "-000108.2500",
            repl_pric => "-000109.2500",
            conv_pric => "-000110.2500",
            drstk => "-000111.2500",
            wonju_pric => "-000112.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40010.
contract!(
    ka40010_contract,
    Ka40010Request,
    ka40010,
    "/api/dostk/etf",
    json!({"stk_cd":"500001"}),
    json!({"stk_cd":"bad"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"etftisl_trnsn":[{"for_netprps":"-00010","cur_prc":"-000101.2500","pre_sig":"-000102.2500","pred_pre":"-000103.2500","trde_qty":"-000104.2500"}]}"#).unwrap(),
    |r: &Ka40010Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.etftisl_trnsn.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            cur_prc => "-000101.2500",
            pre_sig => "-000102.2500",
            pred_pre => "-000103.2500",
            trde_qty => "-000104.2500",
            for_netprps => "-00010",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/11/ka90001.
contract!(
    ka90001_contract,
    Ka90001Request,
    ka90001,
    "/api/dostk/thme",
    json!({"qry_tp":"0","date_tp":"99","flu_pl_amt_tp":"1","stex_tp":"1"}),
    json!({"qry_tp":"2","date_tp":"99","flu_pl_amt_tp":"1","stex_tp":"1"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"thema_grp":[{"thema_grp_cd":"001","main_stk":"synthetic A,synthetic B","thema_nm":"-000102.2500","stk_num":"-000103.2500","flu_sig":"-000104.2500","flu_rt":"-000105.2500","rising_stk_num":"-000106.2500","fall_stk_num":"-000107.2500","dt_prft_rt":"-000108.2500"}]}"#).unwrap(),
    |r: &Ka90001Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.thema_grp.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            thema_grp_cd => "001",
            thema_nm => "-000102.2500",
            stk_num => "-000103.2500",
            flu_sig => "-000104.2500",
            flu_rt => "-000105.2500",
            rising_stk_num => "-000106.2500",
            fall_stk_num => "-000107.2500",
            dt_prft_rt => "-000108.2500",
            main_stk => "synthetic A,synthetic B",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/11/ka90002.
contract!(
    ka90002_contract,
    Ka90002Request,
    ka90002,
    "/api/dostk/thme",
    json!({"thema_grp_cd":"001","stex_tp":"3"}),
    json!({"date_tp":"100","thema_grp_cd":"001","stex_tp":"3"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"dt_prft_rt":"-001.25","thema_comp_stk":[{"dt_prft_rt_n":"+002.25","stk_cd":"-000101.2500","stk_nm":"-000102.2500","cur_prc":"-000103.2500","flu_sig":"-000104.2500","pred_pre":"-000105.2500","flu_rt":"-000106.2500","acc_trde_qty":"-000107.2500","sel_bid":"-000108.2500","sel_req":"-000109.2500","buy_bid":"-000110.2500","buy_req":"-000111.2500"}],"flu_rt":"+000001.4500"}"#).unwrap(),
    |r: &Ka90002Response| {
        assert_fields!(r;
            flu_rt => "+000001.4500",
            dt_prft_rt => "-001.25",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.thema_comp_stk.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stk_cd => "-000101.2500",
            stk_nm => "-000102.2500",
            cur_prc => "-000103.2500",
            flu_sig => "-000104.2500",
            pred_pre => "-000105.2500",
            flu_rt => "-000106.2500",
            acc_trde_qty => "-000107.2500",
            sel_bid => "-000108.2500",
            sel_req => "-000109.2500",
            buy_bid => "-000110.2500",
            buy_req => "-000111.2500",
            dt_prft_rt_n => "+002.25",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/37/usa24300.
contract!(
    usa24300_contract,
    Usa24300Request,
    usa24300,
    "/api/us/invtinfo",
    json!({}),
    json!({"qry_tp":"8"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"result_list":[{"stk_cd":"SYNTH","stex_tp":"ND","cur_prc":"+0001.2300","stk_nm":"-000103.2500","stk_enm":"-000104.2500","pred_pre_sig":"-000106.2500","pred_pre":"-000107.2500","flu_rt":"-000108.2500","acc_trde_qty":"-000109.2500"}]}"#).unwrap(),
    |r: &Usa24300Response| {
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
        let rows = r.result_list.as_ref().unwrap();
        assert_eq!(rows.len(), 1);
        let x = &rows[0];
        assert_fields!(x;
            stex_tp => "ND",
            stk_cd => "SYNTH",
            stk_nm => "-000103.2500",
            stk_enm => "-000104.2500",
            cur_prc => "+0001.2300",
            pred_pre_sig => "-000106.2500",
            pred_pre => "-000107.2500",
            flu_rt => "-000108.2500",
            acc_trde_qty => "-000109.2500",
        );
        assert_eq!(serde_json::to_value(&x.extra).unwrap(), json!({}));
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/41/ust31300.
contract!(
    ust31300_contract,
    Ust31300Request,
    ust31300,
    "/api/us/exchange",
    json!({"exch_tp":"1","fc_exmn_amt":"000100.00"}),
    json!({"exch_tp":"1","fc_exmn_amt":"1e5"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"aplc_exrt":"001300.25","krw_uncl_amt":"000000000000010","sell_expc_amt":"000100.00","sell_aplc_exrt":"+000001.4500","buy_aplc_exrt":"+000002.4500","sell_crnc_entra_nowrm":"+000004.4500","sell_crnc_ch_uncla_nowrm":"+000005.4500","sell_crnc_etc_loana_nowrm":"+000006.4500","sell_crnc_exmn_alow_amt":"+000007.4500","buy_crnc_entra_nowrm":"+000008.4500","buy_crnc_ch_uncla_nowrm":"+000009.4500","buy_crnc_etc_loana_nowrm":"+000010.4500","buy_crnc_exmn_alow_amt":"+000011.4500","buy_expc_amt":"+000014.4500"}"#).unwrap(),
    |r: &Ust31300Response| {
        assert_fields!(r;
            sell_aplc_exrt => "+000001.4500",
            buy_aplc_exrt => "+000002.4500",
            aplc_exrt => "001300.25",
            sell_crnc_entra_nowrm => "+000004.4500",
            sell_crnc_ch_uncla_nowrm => "+000005.4500",
            sell_crnc_etc_loana_nowrm => "+000006.4500",
            sell_crnc_exmn_alow_amt => "+000007.4500",
            buy_crnc_entra_nowrm => "+000008.4500",
            buy_crnc_ch_uncla_nowrm => "+000009.4500",
            buy_crnc_etc_loana_nowrm => "+000010.4500",
            buy_crnc_exmn_alow_amt => "+000011.4500",
            krw_uncl_amt => "000000000000010",
            sell_expc_amt => "000100.00",
            buy_expc_amt => "+000014.4500",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
    }
);

// 출처: https://openapi.kiwoom.com/guide/apiGuideContents/41/ust31301.
contract!(
    ust31301_contract,
    Ust31301Request,
    ust31301,
    "/api/us/exchange",
    json!({"exch_tp":"2"}),
    json!({"exch_tp":"3"}),
    serde_json::from_str::<Value>(r#"{"return_code":0,"aplc_exrt":"001300.25","spcl_bf_exrt":"001310.25","exrt_spcl_rt":"090.00","sell_aplc_exrt":"+000001.4500","buy_aplc_exrt":"+000002.4500","exrt_tp_nm":"+000004.4500"}"#).unwrap(),
    |r: &Ust31301Response| {
        assert_fields!(r;
            sell_aplc_exrt => "+000001.4500",
            buy_aplc_exrt => "+000002.4500",
            aplc_exrt => "001300.25",
            exrt_tp_nm => "+000004.4500",
            spcl_bf_exrt => "001310.25",
            exrt_spcl_rt => "090.00",
        );
        assert_eq!(
            serde_json::to_value(&r.extra).unwrap(),
            json!({"return_code":0})
        );
    }
);

// 상품 응답의 누락 목록, 빈 목록, 잘못된 목록 형식을 구분한다.
#[test]
fn products_missing_empty_and_malformed_collections_are_distinct() {
    let missing: Ka40009Response = serde_json::from_value(json!({})).unwrap();
    let empty: Ka40009Response = serde_json::from_value(json!({"etfnavarray":[]})).unwrap();
    assert!(missing.etfnavarray.is_none());
    let missing_fields: Ka40009Response =
        serde_json::from_value(json!({"etfnavarray":[{}]})).unwrap();
    assert!(missing_fields.etfnavarray.unwrap()[0].nav.is_none());
    let missing_fx: Ust31300Response = serde_json::from_value(json!({})).unwrap();
    assert!(missing_fx.buy_aplc_exrt.is_none());
    assert!(empty.etfnavarray.unwrap().is_empty());
    assert!(serde_json::from_value::<Ka40009Response>(json!({"etfnavarray":{}})).is_err());
    assert!(
        serde_json::from_value::<Ka40009Response>(json!({"etfnavarray":[{"nav":12.3}]})).is_err()
    );
}

// 상품 요청의 필수값과 선택값 경계를 검증한다.
#[test]
fn products_required_and_optional_boundaries() {
    assert!(serde_json::from_value::<Ka40004Request>(json!({"stex_tp":"1"})).is_err());
    assert!(
        Ka90001Request {
            qry_tp: "0".into(),
            stk_cd: None,
            date_tp: "0".into(),
            thema_nm: None,
            flu_pl_amt_tp: "1".into(),
            stex_tp: "1".into()
        }
        .validate()
        .is_err()
    );
    assert!(
        Ust31300Request {
            exch_tp: "1".into(),
            fc_exmn_amt: Some("1234567890123".into())
        }
        .validate()
        .is_err()
    );
    assert!(
        Ust31300Request {
            exch_tp: "1".into(),
            fc_exmn_amt: None
        }
        .validate()
        .is_ok()
    );
    assert!(!CONTRACTS.iter().any(|(id, _)| *id == "ust31302"));
}

// 테마 일수는 표 길이와 달라도 설명에 명시된 두 자리 값을 허용한다.
#[test]
fn theme_days_description_includes_two_digits_despite_table_length() {
    let request = Ka90002Request {
        date_tp: Some("99".into()),
        thema_grp_cd: "001".into(),
        stex_tp: "1".into(),
    };
    assert!(request.validate().is_ok());
    assert_eq!(serde_json::to_value(request).unwrap()["date_tp"], "99");
}
