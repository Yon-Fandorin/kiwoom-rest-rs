//! 합성 모의 값으로 계약을 검증하며, 과거 또는 실시간 시세가 아니다.

use async_trait::async_trait;
use kiwoom_client::{
    Secret,
    market::*,
    transactions::{BrokerDate, UsExchange},
    transport::*,
};
use serde_json::json;
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
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        Ok(self.responses.lock().unwrap().remove(0))
    }
}

fn setup(body: serde_json::Value) -> (Mock, kiwoom_client::Client<Mock>) {
    let mock = Mock::default();
    mock.responses.lock().unwrap().extend([
        HttpResponse {
            status: 200,
            body,
            headers: BTreeMap::from([("cont-yn".into(), "N".into())]),
        },
        HttpResponse {
            status: 200,
            body: json!({"return_code":321}),
            headers: BTreeMap::new(),
        },
    ]);
    let c = kiwoom_client::Client::new(
        mock.clone(),
        ClientOptions {
            min_request_interval: Duration::ZERO,
            retry_base_delay: Duration::ZERO,
            ..ClientOptions::default()
        },
    )
    .unwrap();
    (mock, c)
}

macro_rules! contract {
    ($id:ident, $path:literal, $request:expr, $wire:expr, $response:expr, $check:expr) => {
        // 시세 조회의 원본 응답과 요청 경로를 보존하고 업무 오류를 그대로 전달한다.
        #[tokio::test]
        async fn $id() {
            let request = $request;
            let body = $response;
            let (mock, client) = setup(body.clone());
            let token = Secret::new("offline-market-fixture".into());
            let pages = client.$id(&token, &request).await.unwrap();
            assert_eq!(pages.len(), 1);
            assert_eq!(pages[0].raw, body);
            assert_eq!(
                pages[0].typed.extra["unknown_top"],
                json!({"untouched":true})
            );
            ($check)(&pages[0].typed);

            // 성공 이후의 업무 오류도 임의로 해석하지 않고 전달한다.
            assert!(matches!(
                client.$id(&token, &request).await,
                Err(kiwoom_client::ClientError::Api(321))
            ));

            // 기록된 요청의 경로, API 식별자, 본문을 확인한다.
            let requests = mock.requests.lock().unwrap();
            assert_eq!(requests.len(), 2);
            assert_eq!(requests[0].path, $path);
            assert_eq!(requests[0].api_id, stringify!($id));
            assert_eq!(requests[0].body, $wire);
            assert!(requests[0].continuation.is_none());
        }
    }
}

contract!(
    ka10001,
    "/api/dostk/stkinfo",
    Ka10001Request {
        stk_cd: "039490_NX".into()
    },
    json!({"stk_cd":"039490_NX"}),
    json!({"return_code":0,"cur_prc":"+0001200","250hgst":"+0009000","250lwst_pric_dt":"20260101","unknown_top":{"untouched":true}}),
    |r: &Ka10001Response| {
        assert_eq!(r.cur_prc.as_deref(), Some("+0001200"));
        assert_eq!(r.h250.as_deref(), Some("+0009000"));
        assert_eq!(r.l250_dt.as_deref(), Some("20260101"));
        assert!(r.per.is_none());
    }
);

contract!(
    ka10002,
    "/api/dostk/stkinfo",
    Ka10002Request {
        stk_cd: "039490_AL".into()
    },
    json!({"stk_cd":"039490_AL"}),
    json!({"return_code":0,"buy_trde_qty_5":"-00012","sel_trde_ori_nm_1":"fixture member","unknown_top":{"untouched":true}}),
    |r: &Ka10002Response| {
        assert_eq!(r.buy_trde_qty_5.as_deref(), Some("-00012"));
        assert_eq!(r.sel_trde_ori_nm_1.as_deref(), Some("fixture member"));
    }
);

contract!(
    ka10003,
    "/api/dostk/stkinfo",
    Ka10003Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd":"039490"}),
    json!({"return_code":0,"cntr_infr":[{"tm":"093001","cntr_trde_qty":"-5","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10003Response| {
        let row = &r.cntr_infr.as_ref().unwrap()[0];
        assert_eq!(row.cntr_trde_qty.as_deref(), Some("-5"));
        assert_eq!(row.tm.as_deref(), Some("093001"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10004,
    "/api/dostk/mrkcond",
    Ka10004Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd":"039490"}),
    json!({"return_code":0,"sel_10th_pre_bid":"+1300","buy_fpr_bid":"+1200","ovt_buy_req_pre":"-2","unknown_top":{"untouched":true}}),
    |r: &Ka10004Response| {
        assert_eq!(r.sel_10th_pre_bid.as_deref(), Some("+1300"));
        assert_eq!(r.buy_fpr_bid.as_deref(), Some("+1200"));
        assert_eq!(r.ovt_buy_req_pre.as_deref(), Some("-2"));
    }
);

contract!(
    ka10005,
    "/api/dostk/mrkcond",
    Ka10005Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd":"039490"}),
    json!({"return_code":0,"stk_ddwkmm":[{"date":"20261012","trde_prica":"00012","frgn":"-5","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10005Response| {
        let row = &r.stk_ddwkmm.as_ref().unwrap()[0];
        assert_eq!(row.trde_prica.as_deref(), Some("00012"));
        assert_eq!(row.frgn.as_deref(), Some("-5"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10006,
    "/api/dostk/mrkcond",
    Ka10006Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd":"039490"}),
    json!({"return_code":0,"close_pric":"+1200","cntr_str":"100.00","unknown_top":{"untouched":true}}),
    |r: &Ka10006Response| {
        assert_eq!(r.close_pric.as_deref(), Some("+1200"));
        assert_eq!(r.cntr_str.as_deref(), Some("100.00"));
    }
);

contract!(
    ka10099,
    "/api/dostk/stkinfo",
    Ka10099Request {
        mrkt_tp: StockListMarket::Etf
    },
    json!({"mrkt_tp":"8"}),
    json!({"return_code":0,"list":[{"code":"039490","listCount":"+0000000000000010","nxtEnable":"","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10099Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.list_count.as_deref(), Some("+0000000000000010"));
        assert_eq!(row.nxt_enable.as_deref(), Some(""));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10100,
    "/api/dostk/stkinfo",
    Ka10100Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd":"039490"}),
    json!({"return_code":0,"lastPrice":"+00001200","marketCode":"0","orderWarning":"0","unknown_top":{"untouched":true}}),
    |r: &Ka10100Response| {
        assert_eq!(r.last_price.as_deref(), Some("+00001200"));
        assert_eq!(r.market_code.as_deref(), Some("0"));
        assert_eq!(r.order_warning.as_deref(), Some("0"));
    }
);

contract!(
    ka10102,
    "/api/dostk/stkinfo",
    Ka10102Request {},
    json!({}),
    json!({"return_code":0,"list":[{"code":"001","name":"fixture","gb":"2","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10102Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.code.as_deref(), Some("001"));
        assert_eq!(row.gb.as_deref(), Some("2"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10103,
    "/api/dostk/stkinfo",
    Ka10103Request {
        input_list: Some(vec![DomesticMemoInput {
            stk_cd: Some("039490".into())
        }])
    },
    json!({"input_list":[{"stk_cd":"039490"}]}),
    json!({"return_code":0,"rtcd":"S","result_list":[{"stk_cd":"039490","memo":"배당 확인","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10103Response| {
        assert_eq!(r.rtcd.as_deref(), Some("S"));
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.memo.as_deref(), Some("배당 확인"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10098,
    "/api/us/stkinfo",
    Usa10098Request {
        stk_cd: "AAPL".into()
    },
    json!({"stk_cd":"AAPL"}),
    json!({"return_code":0,"list":[{"stk_cd":"AAPL","isEtf":"N","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10098Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.stk_cd.as_deref(), Some("AAPL"));
        assert_eq!(row.is_etf.as_deref(), Some("N"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10099,
    "/api/us/stkinfo",
    Usa10099Request {
        stex_tp: UsExchangeFilter::All
    },
    json!({"stex_tp":"%"}),
    json!({"return_code":0,"list":[{"stex_tp":"ND","stk_enm":"fixture company","isEtf":"N","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10099Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.stex_tp.as_deref(), Some("ND"));
        assert_eq!(row.stk_enm.as_deref(), Some("fixture company"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10100,
    "/api/us/stkinfo",
    Usa10100Request {
        stex_tp: None,
        stk_cd: "AAPL".into()
    },
    json!({"stk_cd":"AAPL"}),
    json!({"return_code":0,"stk_nm":"fixture","isEtf":"N","unknown_top":{"untouched":true}}),
    |r: &Usa10100Response| {
        assert_eq!(r.stk_nm.as_deref(), Some("fixture"));
        assert_eq!(r.is_etf.as_deref(), Some("N"));
    }
);

contract!(
    usa10101,
    "/api/us/stkinfo",
    Usa10101Request {
        gubun: Some(UsRegionFilter::UnitedStates)
    },
    json!({"gubun":"1"}),
    json!({"return_code":0,"list":[{"inds_cd":"123","inds_enm":"fixture industry","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10101Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.inds_cd.as_deref(), Some("123"));
        assert_eq!(row.inds_enm.as_deref(), Some("fixture industry"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10102,
    "/api/us/stkinfo",
    Usa10102Request {
        index_qry_tp: Some(UsIndexFilter::Sp500)
    },
    json!({"index_qry_tp":"NS"}),
    json!({"return_code":0,"list":[{"index_cd":"fixture","index_enm":"fixture index","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10102Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.index_cd.as_deref(), Some("fixture"));
        assert_eq!(row.index_enm.as_deref(), Some("fixture index"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10103,
    "/api/us/stkinfo",
    Usa10103Request {
        input_list: Some(vec![UsMemoInput {
            stex_tp: Some(UsExchange::Nasdaq),
            stk_cd: Some("AAPL".into())
        }])
    },
    json!({"input_list":[{"stex_tp":"ND","stk_cd":"AAPL"}]}),
    json!({"return_code":0,"rtcd":"S","result_list":[{"stex_tp":"ND","stk_cd":"AAPL","memo":"fixture memo","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10103Response| {
        assert_eq!(r.rtcd.as_deref(), Some("S"));
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.memo.as_deref(), Some("fixture memo"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10104,
    "/api/us/stkinfo",
    Usa10104Request {
        stex_tp: Some(UsExchangeFilter::Nasdaq)
    },
    json!({"stex_tp":"ND"}),
    json!({"return_code":0,"list":[{"code":"fixture","cate1":"1","etn":"Y","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10104Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.cate1.as_deref(), Some("1"));
        assert_eq!(row.etn.as_deref(), Some("Y"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa10105,
    "/api/us/stkinfo",
    Usa10105Request {
        gubun: Some(UsCategoryFilter::Second)
    },
    json!({"gubun":"2"}),
    json!({"return_code":0,"list":[{"cate2":"2","cate2nam":"fixture category","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa10105Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.cate2.as_deref(), Some("2"));
        assert_eq!(row.cate2nam.as_deref(), Some("fixture category"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa20100,
    "/api/us/mrkcond",
    Usa20100Request {
        stex_tp: UsExchange::Nasdaq,
        stk_cd: "AAPL".into()
    },
    json!({"stex_tp":"ND","stk_cd":"AAPL"}),
    json!({"return_code":0,"52wk_hgst_pric":"+0999.1234","base_exrt":"1300.00","curr_unit":"USD","unknown_top":{"untouched":true}}),
    |r: &Usa20100Response| {
        assert_eq!(r.h52_pric.as_deref(), Some("+0999.1234"));
        assert_eq!(r.base_exrt.as_deref(), Some("1300.00"));
        assert_eq!(r.curr_unit.as_deref(), Some("USD"));
    }
);

contract!(
    usa20101,
    "/api/us/mrkcond",
    Usa20101Request {
        stex_tp: UsExchange::Nyse,
        stk_cd: "IBM".into()
    },
    json!({"stex_tp":"NY","stk_cd":"IBM"}),
    json!({"return_code":0,"sel_10bid_req":"0010","buy_1th_pre_req_pre":"-3","trde_prica":"00012","unknown_top":{"untouched":true}}),
    |r: &Usa20101Response| {
        assert_eq!(r.sel_10bid_req.as_deref(), Some("0010"));
        assert_eq!(r.buy_1th_pre_req_pre.as_deref(), Some("-3"));
        assert_eq!(r.trde_prica.as_deref(), Some("00012"));
    }
);

contract!(
    usa20150,
    "/api/us/mrkcond",
    Usa20150Request {
        stex_tp: UsExchange::Nasdaq,
        stk_cd: "AAPL".into()
    },
    json!({"stex_tp":"ND","stk_cd":"AAPL"}),
    json!({"return_code":0,"result_list":[{"cur_prc":"+0001.2345","cntr_tm":"093001","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa20150Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
        assert_eq!(row.cntr_tm.as_deref(), Some("093001"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa20151,
    "/api/us/mrkcond",
    Usa20151Request {
        stex_tp: UsExchange::Nasdaq,
        stk_cd: "AAPL".into(),
        base_dt: Some(BrokerDate::new("20261012").unwrap())
    },
    json!({"stex_tp":"ND","stk_cd":"AAPL","base_dt":"20261012"}),
    json!({"return_code":0,"result_list":[{"dt":"20261009","acc_trde_qty":"00010","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa20151Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.dt.as_deref(), Some("20261009"));
        assert_eq!(row.acc_trde_qty.as_deref(), Some("00010"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    usa20590,
    "/api/us/mrkcond",
    Usa20590Request {
        stex_tp: None,
        stk_cd: Some("AAPL".into()),
        base_dt: None
    },
    json!({"stk_cd":"AAPL"}),
    json!({"return_code":0,"result_list":[{"base_pric":"0010.0000","base_open_flu_rt":"-1.00","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Usa20590Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.base_pric.as_deref(), Some("0010.0000"));
        assert_eq!(row.base_open_flu_rt.as_deref(), Some("-1.00"));
        assert_eq!(row.extra["future"], 7);
    }
);

// 시세 입력은 전송 전에 검증한다.
#[tokio::test]
async fn market_validation_precedes_transport() {
    let (mock, c) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    assert!(
        c.ka10001(
            &token,
            &Ka10001Request {
                stk_cd: "X".repeat(21)
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10100(
            &token,
            &Ka10100Request {
                stk_cd: "ABCDEF".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa20100(
            &token,
            &Usa20100Request {
                stex_tp: UsExchange::Amex,
                stk_cd: " ".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10103(
            &token,
            &Ka10103Request {
                input_list: Some(vec![DomesticMemoInput {
                    stk_cd: Some("X".repeat(13))
                }])
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa10103(
            &token,
            &Usa10103Request {
                input_list: Some(vec![UsMemoInput {
                    stex_tp: None,
                    stk_cd: Some("X".repeat(19))
                }])
            }
        )
        .await
        .is_err()
    );
    assert!(mock.requests.lock().unwrap().is_empty());
    assert!(
        serde_json::from_value::<Usa20100Request>(json!({"stex_tp":"%","stk_cd":"AAPL"})).is_err()
    );
    assert!(serde_json::from_value::<Ka10099Request>(json!({"mrkt_tp":"1"})).is_err());
    assert!(
        serde_json::from_value::<Usa20151Request>(
            json!({"stex_tp":"ND","stk_cd":"AAPL","base_dt":"20260230"})
        )
        .is_err()
    );
}

// 시세 응답의 누락 목록과 빈 목록을 구분한다.
#[test]
fn omitted_and_empty_market_collections_remain_distinct() {
    let a: Usa10098Response = serde_json::from_value(json!({})).unwrap();
    let b: Usa10098Response = serde_json::from_value(json!({"list":[]})).unwrap();
    assert!(a.list.is_none());
    assert!(b.list.unwrap().is_empty());
    assert!(serde_json::from_value::<Usa10098Response>(json!({"list":[{"stk_cd":123}]})).is_err());
    assert_eq!(
        serde_json::to_value(Usa10101Request::default()).unwrap(),
        json!({})
    );
}

contract!(
    usa20520,
    "/api/us/stkinfo",
    Usa20520Request {
        stex_tp: Some("2".into()),
        tm: Some("20".into()),
        stk_tp: Some("1".into()),
        trde_prica_cnd: Some("50".into()),
        ..Default::default()
    },
    json!({"stex_tp": "2", "tm": "20", "stk_tp": "1", "trde_prica_cnd": "50"}),
    json!({"return_code": 0, "result_list": [{"sdnin_rt": "-1.23", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20520Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.sdnin_rt.as_deref(), Some("-1.23"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20521,
    "/api/us/stkinfo",
    Usa20521Request {
        stex_tp: Some("3".into()),
        tm: Some("5".into()),
        etf_cat1: Some("fixture-category".into()),
        trde_qty_tp: Some("15".into()),
        ..Default::default()
    },
    json!({"stex_tp": "3", "tm": "5", "etf_cat1": "fixture-category", "trde_qty_tp": "15"}),
    json!({"return_code": 0, "result_list": [{"rank": "0001", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20521Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.rank.as_deref(), Some("0001"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20570,
    "/api/us/stkinfo",
    Usa20570Request {
        pric_cnd1: Some("10.00".into()),
        pric_cnd2: Some("25.99".into()),
        stk_tp: Some("1".into()),
        ..Default::default()
    },
    json!({"pric_cnd1": "10.00", "pric_cnd2": "25.99", "stk_tp": "1"}),
    json!({"return_code": 0, "result_list": [{"open_pric": "+0012.3400", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}, "stk_num": "0010", "rising_stk_num": "0002", "flat_stk_num": "0003", "fall_stk_num": "0005"}),
    |r: &Usa20570Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.open_pric.as_deref(), Some("+0012.3400"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
        assert_eq!(r.stk_num.as_deref(), Some("0010"));
        assert_eq!(r.fall_stk_num.as_deref(), Some("0005"));
    }
);

contract!(
    usa20571,
    "/api/us/stkinfo",
    Usa20571Request {
        pric_cnd1: Some("0".into()),
        pric_cnd2: Some("999999.99".into()),
        etf_cat2: Some("fixture".into()),
        ..Default::default()
    },
    json!({"pric_cnd1": "0", "pric_cnd2": "999999.99", "etf_cat2": "fixture"}),
    json!({"return_code": 0, "result_list": [{"low_pric": "+0001.0000", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}, "stk_num": "0010", "rising_stk_num": "0002", "flat_stk_num": "0003", "fall_stk_num": "0005"}),
    |r: &Usa20571Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.low_pric.as_deref(), Some("+0001.0000"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
        assert_eq!(r.stk_num.as_deref(), Some("0010"));
        assert_eq!(r.fall_stk_num.as_deref(), Some("0005"));
    }
);

contract!(
    usa20930,
    "/api/us/stkinfo",
    Usa20930Request {
        flu_tp: Some("2".into()),
        tm_tp: Some("3".into()),
        tm: Some("9".into()),
        stk_cnd: Some("1".into()),
        ..Default::default()
    },
    json!({"flu_tp": "2", "tm_tp": "3", "tm": "9", "stk_cnd": "1"}),
    json!({"return_code": 0, "result_list": [{"base_pre": "-0001.2000", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20930Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.base_pre.as_deref(), Some("-0001.2000"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20931,
    "/api/us/stkinfo",
    Usa20931Request {
        flu_tp: Some("1".into()),
        tm_tp: Some("2".into()),
        tm: Some("30".into()),
        etf_cat1: Some("fixture".into()),
        ..Default::default()
    },
    json!({"flu_tp": "1", "tm_tp": "2", "tm": "30", "etf_cat1": "fixture"}),
    json!({"return_code": 0, "result_list": [{"sdnin_rt": "+1.25", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20931Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.sdnin_rt.as_deref(), Some("+1.25"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20932,
    "/api/us/stkinfo",
    Usa20932Request {
        stex_tp: Some("2".into()),
        flu_tp: Some("2".into()),
        tm_tp: Some("1".into()),
        tm: Some("5".into()),
        stk_cd: Some(vec![UsWatchItem {
            stex_tp: Some(UsExchange::Amex),
            stk_cd: Some("SPY".into())
        }]),
        ..Default::default()
    },
    json!({"stex_tp": "2", "flu_tp": "2", "tm_tp": "1", "tm": "5", "stk_cd": [{"stex_tp": "NA", "stk_cd": "SPY"}]}),
    json!({"return_code": 0, "result_list": [{"base_pric": "0010.0000", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20932Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.base_pric.as_deref(), Some("0010.0000"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20970,
    "/api/us/stkinfo",
    Usa20970Request {
        high_low_tp: Some("1".into()),
        alacc_rt: Some("0.5".into()),
        pric_cnd_st: Some("0".into()),
        pric_cnd_ed: Some("20".into()),
        ..Default::default()
    },
    json!({"high_low_tp": "1", "alacc_rt": "0.5", "pric_cnd_st": "0", "pric_cnd_ed": "20"}),
    json!({"return_code": 0, "result_list": [{"high_pric": "+0020.0000", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20970Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.high_pric.as_deref(), Some("+0020.0000"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20971,
    "/api/us/stkinfo",
    Usa20971Request {
        high_low_tp: Some("2".into()),
        alacc_rt: Some("3.5".into()),
        etf_cat2: Some("fixture".into()),
        ..Default::default()
    },
    json!({"high_low_tp": "2", "alacc_rt": "3.5", "etf_cat2": "fixture"}),
    json!({"return_code": 0, "result_list": [{"low_pric": "+0009.9999", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20971Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.low_pric.as_deref(), Some("+0009.9999"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa20972,
    "/api/us/stkinfo",
    Usa20972Request {
        high_low_tp: Some("2".into()),
        alacc_rt: Some("1.0".into()),
        stk_cd: Some(vec![UsWatchItem {
            stex_tp: Some(UsExchange::Amex),
            stk_cd: Some("SPY".into())
        }]),
        ..Default::default()
    },
    json!({"high_low_tp": "2", "alacc_rt": "1.0", "stk_cd": [{"stex_tp": "NA", "stk_cd": "SPY"}]}),
    json!({"return_code": 0, "result_list": [{"tp": "", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa20972Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.tp.as_deref(), Some(""));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa23400,
    "/api/us/stkinfo",
    Usa23400Request {
        stk_cd: Some("000".into()),
        dt_tp: Some("250".into()),
        trde_qty_tp: Some("10".into()),
        ..Default::default()
    },
    json!({"stk_cd": "000", "dt_tp": "250", "trde_qty_tp": "10"}),
    json!({"return_code": 0, "result_list": [{"prev_trde_qty": "000010", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa23400Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.prev_trde_qty.as_deref(), Some("000010"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa23401,
    "/api/us/stkinfo",
    Usa23401Request {
        etf_cat1: Some("fixture".into()),
        dt_tp: Some("60".into()),
        stk_cnd: Some("2".into()),
        ..Default::default()
    },
    json!({"etf_cat1": "fixture", "dt_tp": "60", "stk_cnd": "2"}),
    json!({"return_code": 0, "result_list": [{"acc_trde_qty": "000012", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa23401Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.acc_trde_qty.as_deref(), Some("000012"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa23402,
    "/api/us/stkinfo",
    Usa23402Request {
        dt_tp: Some("5".into()),
        stex_tp: Some("3".into()),
        stk_cd: Some(vec![UsWatchItem {
            stex_tp: Some(UsExchange::Amex),
            stk_cd: Some("SPY".into())
        }]),
        ..Default::default()
    },
    json!({"dt_tp": "5", "stex_tp": "3", "stk_cd": [{"stex_tp": "NA", "stk_cd": "SPY"}]}),
    json!({"return_code": 0, "result_list": [{"prev_trde_qty": "000099", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa23402Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.prev_trde_qty.as_deref(), Some("000099"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24100,
    "/api/us/stkinfo",
    Usa24100Request {
        ntl_tp: Some("1".into()),
        high_low_tp: Some("2".into()),
        dt: Some("250".into()),
        stk_tp: Some("1".into()),
        ..Default::default()
    },
    json!({"ntl_tp": "1", "high_low_tp": "2", "dt": "250", "stk_tp": "1"}),
    json!({"return_code": 0, "result_list": [{"pre_trde_rt": "+100.00", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24100Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.pre_trde_rt.as_deref(), Some("+100.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24101,
    "/api/us/stkinfo",
    Usa24101Request {
        ntl_tp: Some("2".into()),
        high_low_tp: Some("1".into()),
        dt: Some("100".into()),
        etf_cat1: Some("fixture".into()),
        ..Default::default()
    },
    json!({"ntl_tp": "2", "high_low_tp": "1", "dt": "100", "etf_cat1": "fixture"}),
    json!({"return_code": 0, "result_list": [{"pred_trde_qty": "000013", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24101Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.pred_trde_qty.as_deref(), Some("000013"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24140,
    "/api/us/stkinfo",
    Usa24140Request {
        sort_tp: Some("1".into()),
        updown_tp: Some("2".into()),
        alacc_rt: Some("150".into()),
        ..Default::default()
    },
    json!({"sort_tp": "1", "updown_tp": "2", "alacc_rt": "150"}),
    json!({"return_code": 0, "result_list": [{"gap_rt": "-3.00", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24140Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.gap_rt.as_deref(), Some("-3.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24141,
    "/api/us/stkinfo",
    Usa24141Request {
        sort_tp: Some("2".into()),
        updown_tp: Some("1".into()),
        alacc_rt: Some("3".into()),
        etf_cat2: Some("fixture".into()),
        ..Default::default()
    },
    json!({"sort_tp": "2", "updown_tp": "1", "alacc_rt": "3", "etf_cat2": "fixture"}),
    json!({"return_code": 0, "result_list": [{"base_close_pric": "0010.0000", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24141Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.base_close_pric.as_deref(), Some("0010.0000"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24210,
    "/api/us/stkinfo",
    Usa24210Request {
        rt_tp: Some("0".into()),
        tm: Some("30".into()),
        trde_prica_cnd: Some("5000".into()),
        ..Default::default()
    },
    json!({"rt_tp": "0", "tm": "30", "trde_prica_cnd": "5000"}),
    json!({"return_code": 0, "result_list": [{"int": "10.00", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24210Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.int.as_deref(), Some("10.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24211,
    "/api/us/stkinfo",
    Usa24211Request {
        rt_tp: Some("1".into()),
        tm: Some("0".into()),
        etf_cat1: Some("fixture".into()),
        ..Default::default()
    },
    json!({"rt_tp": "1", "tm": "0", "etf_cat1": "fixture"}),
    json!({"return_code": 0, "result_list": [{"now_rt": "50.00", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24211Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.now_rt.as_deref(), Some("50.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24220,
    "/api/us/stkinfo",
    Usa24220Request {
        dt: Some("300".into()),
        prps_cnctr_rt: Some("100".into()),
        prpscnt: Some("99".into()),
        cond: Some("2".into()),
        ..Default::default()
    },
    json!({"dt": "300", "prps_cnctr_rt": "100", "prpscnt": "99", "cond": "2"}),
    json!({"return_code": 0, "result_list": [{"prps_qty": "000005", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24220Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.prps_qty.as_deref(), Some("000005"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa24221,
    "/api/us/stkinfo",
    Usa24221Request {
        dt: Some("20".into()),
        prps_cnctr_rt: Some("2.50".into()),
        prpscnt: Some("5".into()),
        etf_cat1: Some("fixture".into()),
        ..Default::default()
    },
    json!({"dt": "20", "prps_cnctr_rt": "2.50", "prpscnt": "5", "etf_cat1": "fixture"}),
    json!({"return_code": 0, "result_list": [{"prps_rt": "2.50", "future": 7, "cur_prc": "+0001.2345"}], "unknown_top": {"untouched": true}}),
    |r: &Usa24221Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.prps_rt.as_deref(), Some("2.50"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.cur_prc.as_deref(), Some("+0001.2345"));
    }
);

contract!(
    usa26410,
    "/api/us/stkinfo",
    Usa26410Request {
        stex_tp: UsExchange::Nasdaq,
        stk_cd: "AAPL".into()
    },
    json!({"stex_tp": "ND", "stk_cd": "AAPL"}),
    json!({"return_code": 0, "result_list": [{"dt": "0", "future": 7, "m02_prft_rt": "-1.00"}], "unknown_top": {"untouched": true}}),
    |r: &Usa26410Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.dt.as_deref(), Some("0"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.m02_prft_rt.as_deref(), Some("-1.00"));
    }
);

contract!(
    usa26411,
    "/api/us/stkinfo",
    Usa26411Request {
        inds_cd: Some("000".into()),
        srch_yr: Some("2026".into())
    },
    json!({"inds_cd": "000", "srch_yr": "2026"}),
    json!({"return_code": 0, "result_list": [{"stk_enm": "fixture company", "future": 7, "m02_prft_rt": "-1.00"}], "unknown_top": {"untouched": true}}),
    |r: &Usa26411Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.stk_enm.as_deref(), Some("fixture company"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.m02_prft_rt.as_deref(), Some("-1.00"));
    }
);

contract!(
    usa26412,
    "/api/us/stkinfo",
    Usa26412Request {
        etf_cat1: Some("fixture".into()),
        etf_cat2: Some("fixture2".into()),
        srch_yr: Some("2025".into())
    },
    json!({"etf_cat1": "fixture", "etf_cat2": "fixture2", "srch_yr": "2025"}),
    json!({"return_code": 0, "result_list": [{"m12_prft_rt": "-1.23", "future": 7, "m02_prft_rt": "-1.00"}], "unknown_top": {"untouched": true}}),
    |r: &Usa26412Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.m12_prft_rt.as_deref(), Some("-1.23"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.m02_prft_rt.as_deref(), Some("-1.00"));
    }
);

contract!(
    usa26413,
    "/api/us/stkinfo",
    Usa26413Request {
        inds_cd: Some("000".into())
    },
    json!({"inds_cd": "000"}),
    json!({"return_code": 0, "result_list": [{"m01_prft_rt": "+2.00", "future": 7, "m02_prft_rt": "-1.00"}], "unknown_top": {"untouched": true}}),
    |r: &Usa26413Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.m01_prft_rt.as_deref(), Some("+2.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.m02_prft_rt.as_deref(), Some("-1.00"));
    }
);

contract!(
    usa26414,
    "/api/us/stkinfo",
    Usa26414Request {
        etf_cat2: Some("fixture".into()),
        ..Default::default()
    },
    json!({"etf_cat2": "fixture"}),
    json!({"return_code": 0, "result_list": [{"m06_prft_rt": "0.00", "future": 7, "m02_prft_rt": "-1.00"}], "unknown_top": {"untouched": true}}),
    |r: &Usa26414Response| {
        let row = &r.result_list.as_ref().unwrap()[0];
        assert_eq!(row.m06_prft_rt.as_deref(), Some("0.00"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(row.m02_prft_rt.as_deref(), Some("-1.00"));
    }
);

// 검색 한도와 선택 관심종목 행은 전송 전에 검증한다.
#[tokio::test]
async fn screening_limits_and_optional_watch_rows_validate_before_transport() {
    let (mock, c) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    let q = Usa20570Request {
        pric_cnd2: Some("1000000.00".into()),
        ..Default::default()
    };
    assert!(c.usa20570(&token, &q).await.is_err());
    assert!(
        c.usa20521(
            &token,
            &Usa20521Request {
                tm: Some("6".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa20930(
            &token,
            &Usa20930Request {
                tm_tp: Some("3".into()),
                tm: Some("10".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa24210(
            &token,
            &Usa24210Request {
                tm: Some("31".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa24100(
            &token,
            &Usa24100Request {
                dt: Some("251".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa24220(
            &token,
            &Usa24220Request {
                prps_cnctr_rt: Some("100.01".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa24221(
            &token,
            &Usa24221Request {
                prpscnt: Some("100".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa26411(
            &token,
            &Usa26411Request {
                srch_yr: Some("20261".into()),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.usa20972(
            &token,
            &Usa20972Request {
                stk_cd: Some(vec![UsWatchItem {
                    stk_cd: Some("".into()),
                    ..Default::default()
                }]),
                ..Default::default()
            }
        )
        .await
        .is_err()
    );
    assert!(mock.requests.lock().unwrap().is_empty());
    assert_eq!(
        serde_json::to_value(Usa24221Request::default()).unwrap(),
        json!({})
    );
}

contract!(
    ka00198,
    "/api/dostk/stkinfo",
    Ka00198Request { qry_tp: "5".into() },
    json!({"qry_tp": "5"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "item_inq_rank": [{"rank_chg": "", "future": 7}]}),
    |r: &Ka00198Response| {
        let row = &r.item_inq_rank.as_ref().unwrap()[0];
        assert_eq!(row.rank_chg.as_deref(), Some(""));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10013,
    "/api/dostk/stkinfo",
    Ka10013Request {
        stk_cd: "039490_NX".into(),
        dt: BrokerDate::new("20261012").unwrap(),
        qry_tp: "2".into()
    },
    json!({"stk_cd": "039490_NX", "dt": "20261012", "qry_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "crd_trde_trend": [{"rpya": "00012", "future": 7}]}),
    |r: &Ka10013Response| {
        let row = &r.crd_trde_trend.as_ref().unwrap()[0];
        assert_eq!(row.rpya.as_deref(), Some("00012"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10015,
    "/api/dostk/stkinfo",
    Ka10015Request {
        stk_cd: "039490".into(),
        strt_dt: BrokerDate::new("20261012").unwrap()
    },
    json!({"stk_cd": "039490", "strt_dt": "20261012"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "daly_trde_dtl": [{"af_mkrt_trde_prica_wght": "-2.00", "future": 7}]}),
    |r: &Ka10015Response| {
        let row = &r.daly_trde_dtl.as_ref().unwrap()[0];
        assert_eq!(row.af_mkrt_trde_prica_wght.as_deref(), Some("-2.00"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10055,
    "/api/dostk/stkinfo",
    Ka10055Request {
        stk_cd: "039490".into(),
        tdy_pred: "2".into()
    },
    json!({"stk_cd": "039490", "tdy_pred": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "tdy_pred_cntr_qty": [{"cntr_qty": "-5", "future": 7}]}),
    |r: &Ka10055Response| {
        let row = &r.tdy_pred_cntr_qty.as_ref().unwrap()[0];
        assert_eq!(row.cntr_qty.as_deref(), Some("-5"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10084,
    "/api/dostk/stkinfo",
    Ka10084Request {
        stk_cd: "039490".into(),
        tdy_pred: "1".into(),
        tic_min: "1".into(),
        tm: Some("".into())
    },
    json!({"stk_cd": "039490", "tdy_pred": "1", "tic_min": "1", "tm": ""}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "tdy_pred_cntr": [{"acc_trde_prica": "00012", "future": 7}]}),
    |r: &Ka10084Response| {
        let row = &r.tdy_pred_cntr.as_ref().unwrap()[0];
        assert_eq!(row.acc_trde_prica.as_deref(), Some("00012"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90003,
    "/api/dostk/stkinfo",
    Ka90003Request {
        trde_upper_tp: "1".into(),
        amt_qty_tp: "2".into(),
        mrkt_tp: "P10102".into(),
        stex_tp: "3".into()
    },
    json!({"trde_upper_tp": "1", "amt_qty_tp": "2", "mrkt_tp": "P10102", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prm_netprps_upper_50": [{"prm_netprps_amt": "-0010", "future": 7}]}),
    |r: &Ka90003Response| {
        let row = &r.prm_netprps_upper_50.as_ref().unwrap()[0];
        assert_eq!(row.prm_netprps_amt.as_deref(), Some("-0010"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90004,
    "/api/dostk/stkinfo",
    Ka90004Request {
        dt: BrokerDate::new("20261012").unwrap(),
        mrkt_tp: "P00101".into(),
        stex_tp: "1".into()
    },
    json!({"dt": "20261012", "mrkt_tp": "P00101", "stex_tp": "1"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "stk_prm_trde_prst": [{"buy_cntr_qty": "00005", "future": 7}], "tot_5": "-00010"}),
    |r: &Ka90004Response| {
        let row = &r.stk_prm_trde_prst.as_ref().unwrap()[0];
        assert_eq!(row.buy_cntr_qty.as_deref(), Some("00005"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(r.tot_5.as_deref(), Some("-00010"));
    }
);

contract!(
    kt20016,
    "/api/dostk/stkinfo",
    Kt20016Request {
        mrkt_deal_tp: "%".into(),
        crd_stk_grde_tp: Some("E".into()),
        stk_cd: Some("039490".into())
    },
    json!({"mrkt_deal_tp": "%", "crd_stk_grde_tp": "E", "stk_cd": "039490"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "crd_loan_pos_stk": [{"crd_limit_over_txt": "", "future": 7}], "crd_loan_able": "fixture availability"}),
    |r: &Kt20016Response| {
        let row = &r.crd_loan_pos_stk.as_ref().unwrap()[0];
        assert_eq!(row.crd_limit_over_txt.as_deref(), Some(""));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(r.crd_loan_able.as_deref(), Some("fixture availability"));
    }
);

contract!(
    kt20017,
    "/api/dostk/stkinfo",
    Kt20017Request {
        stk_cd: "039490".into()
    },
    json!({"stk_cd": "039490"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "crd_alow_yn": "fixture availability"}),
    |r: &Kt20017Response| {
        assert_eq!(r.crd_alow_yn.as_deref(), Some("fixture availability"));
    }
);

contract!(
    ka10011,
    "/api/dostk/mrkcond",
    Ka10011Request {
        newstk_recvrht_tp: "07".into()
    },
    json!({"newstk_recvrht_tp": "07"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "newstk_recvrht_mrpr": [{"fpr_sel_bid": "+0100", "future": 7}]}),
    |r: &Ka10011Response| {
        let row = &r.newstk_recvrht_mrpr.as_ref().unwrap()[0];
        assert_eq!(row.fpr_sel_bid.as_deref(), Some("+0100"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10044,
    "/api/dostk/mrkcond",
    Ka10044Request {
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        trde_tp: "2".into(),
        mrkt_tp: "001".into(),
        stex_tp: "2".into()
    },
    json!({"strt_dt": "20261001", "end_dt": "20261012", "trde_tp": "2", "mrkt_tp": "001", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "daly_orgn_trde_stk": [{"netprps_qty": "-0001", "future": 7}]}),
    |r: &Ka10044Response| {
        let row = &r.daly_orgn_trde_stk.as_ref().unwrap()[0];
        assert_eq!(row.netprps_qty.as_deref(), Some("-0001"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10045,
    "/api/dostk/mrkcond",
    Ka10045Request {
        stk_cd: "039490".into(),
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        orgn_prsm_unp_tp: "1".into(),
        for_prsm_unp_tp: "2".into()
    },
    json!({"stk_cd": "039490", "strt_dt": "20261001", "end_dt": "20261012", "orgn_prsm_unp_tp": "1", "for_prsm_unp_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "stk_orgn_trde_trnsn": [{"orgn_dt_acc": "-0010", "future": 7}], "orgn_prsm_avg_pric": "0001200"}),
    |r: &Ka10045Response| {
        let row = &r.stk_orgn_trde_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.orgn_dt_acc.as_deref(), Some("-0010"));
        assert_eq!(row.extra["future"], 7);
        assert_eq!(r.orgn_prsm_avg_pric.as_deref(), Some("0001200"));
    }
);

contract!(
    ka10078,
    "/api/dostk/mrkcond",
    Ka10078Request {
        mmcm_cd: "001".into(),
        stk_cd: "039490".into(),
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap()
    },
    json!({"mmcm_cd": "001", "stk_cd": "039490", "strt_dt": "20261001", "end_dt": "20261012"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "sec_stk_trde_trend": [{"sell_qty": "00010", "future": 7}]}),
    |r: &Ka10078Response| {
        let row = &r.sec_stk_trde_trend.as_ref().unwrap()[0];
        assert_eq!(row.sell_qty.as_deref(), Some("00010"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10086,
    "/api/dostk/mrkcond",
    Ka10086Request {
        stk_cd: "039490".into(),
        qry_dt: BrokerDate::new("20261012").unwrap(),
        indc_tp: "1".into()
    },
    json!({"stk_cd": "039490", "qry_dt": "20261012", "indc_tp": "1"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "daly_stkpc": [{"for_qty": "-0010", "future": 7}]}),
    |r: &Ka10086Response| {
        let row = &r.daly_stkpc.as_ref().unwrap()[0];
        assert_eq!(row.for_qty.as_deref(), Some("-0010"));
        assert_eq!(row.extra["future"], 7);
    }
);

// 국내 시세의 날짜, 시간, 열거 값은 로컬에서 검증한다.
#[tokio::test]
async fn domestic_dates_time_and_enum_validation_is_local() {
    let (mock, c) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    assert!(
        c.ka00198(&token, &Ka00198Request { qry_tp: "6".into() })
            .await
            .is_err()
    );
    assert!(
        c.ka10084(
            &token,
            &Ka10084Request {
                stk_cd: "039490".into(),
                tdy_pred: "1".into(),
                tic_min: "0".into(),
                tm: Some("2460".into())
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10044(
            &token,
            &Ka10044Request {
                strt_dt: BrokerDate::new("20261012").unwrap(),
                end_dt: BrokerDate::new("20261001").unwrap(),
                trde_tp: "1".into(),
                mrkt_tp: "001".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.kt20016(
            &token,
            &Kt20016Request {
                mrkt_deal_tp: "%".into(),
                crd_stk_grde_tp: Some("F".into()),
                stk_cd: None
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka90003(
            &token,
            &Ka90003Request {
                trde_upper_tp: "1".into(),
                amt_qty_tp: "2".into(),
                mrkt_tp: "001".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(mock.requests.lock().unwrap().is_empty());
}

contract!(
    ka10063,
    "/api/dostk/mrkcond",
    Ka10063Request {
        mrkt_tp: "101".into(),
        amt_qty_tp: "1".into(),
        invsr: "0".into(),
        frgn_all: "1".into(),
        smtm_netprps_tp: "0".into(),
        stex_tp: "3".into()
    },
    json!({"mrkt_tp": "101", "amt_qty_tp": "1", "invsr": "0", "frgn_all": "1", "smtm_netprps_tp": "0", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "opmr_invsr_trde": [{"prev_pot_netprps_qty": "-0001", "future": 7}]}),
    |r: &Ka10063Response| {
        let row = &r.opmr_invsr_trde.as_ref().unwrap()[0];
        assert_eq!(row.prev_pot_netprps_qty.as_deref(), Some("-0001"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10066,
    "/api/dostk/mrkcond",
    Ka10066Request {
        mrkt_tp: "001".into(),
        amt_qty_tp: "2".into(),
        trde_tp: "0".into(),
        stex_tp: "2".into()
    },
    json!({"mrkt_tp": "001", "amt_qty_tp": "2", "trde_tp": "0", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "opaf_invsr_trde": [{"samo_fund": "00005", "future": 7}]}),
    |r: &Ka10066Response| {
        let row = &r.opaf_invsr_trde.as_ref().unwrap()[0];
        assert_eq!(row.samo_fund.as_deref(), Some("00005"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka50010,
    "/api/dostk/mrkcond",
    Ka50010Request {
        stk_cd: "M04020000".into()
    },
    json!({"stk_cd": "M04020000"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "gold_cntr": [{"acc_trde_prica": "00012", "future": 7}]}),
    |r: &Ka50010Response| {
        let row = &r.gold_cntr.as_ref().unwrap()[0];
        assert_eq!(row.acc_trde_prica.as_deref(), Some("00012"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka50012,
    "/api/dostk/mrkcond",
    Ka50012Request {
        stk_cd: "M04020100".into(),
        base_dt: BrokerDate::new("20261012").unwrap()
    },
    json!({"stk_cd": "M04020100", "base_dt": "20261012"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "gold_daly_trnsn": [{"orgn_netprps": "-0010", "future": 7}]}),
    |r: &Ka50012Response| {
        let row = &r.gold_daly_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.orgn_netprps.as_deref(), Some("-0010"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka50087,
    "/api/dostk/mrkcond",
    Ka50087Request {
        stk_cd: "M04020000".into()
    },
    json!({"stk_cd": "M04020000"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "gold_expt_exec": [{"exp_cntr_trde_qty": "-0005", "future": 7}]}),
    |r: &Ka50087Response| {
        let row = &r.gold_expt_exec.as_ref().unwrap()[0];
        assert_eq!(row.exp_cntr_trde_qty.as_deref(), Some("-0005"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka50100,
    "/api/dostk/mrkcond",
    Ka50100Request {
        stk_cd: "M04020100".into()
    },
    json!({"stk_cd": "M04020100"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "pred_close_pric": "001000"}),
    |r: &Ka50100Response| {
        assert_eq!(r.pred_close_pric.as_deref(), Some("001000"));
    }
);

contract!(
    ka50101,
    "/api/dostk/mrkcond",
    Ka50101Request {
        stk_cd: "M04020000".into(),
        tic_scope: "30".into()
    },
    json!({"stk_cd": "M04020000", "tic_scope": "30"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "gold_bid": [{"lpmmcm_nm_1": "", "future": 7}]}),
    |r: &Ka50101Response| {
        let row = &r.gold_bid.as_ref().unwrap()[0];
        assert_eq!(row.lpmmcm_nm_1.as_deref(), Some(""));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90005,
    "/api/dostk/mrkcond",
    Ka90005Request {
        date: BrokerDate::new("20261012").unwrap(),
        amt_qty_tp: "1".into(),
        mrkt_tp: "P101_AL02".into(),
        min_tic_tp: "1".into(),
        stex_tp: "3".into()
    },
    json!({"date": "20261012", "amt_qty_tp": "1", "mrkt_tp": "P101_AL02", "min_tic_tp": "1", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prm_trde_trnsn": [{"cntr_tm": "093000", "future": 7}]}),
    |r: &Ka90005Response| {
        let row = &r.prm_trde_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.cntr_tm.as_deref(), Some("093000"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90006,
    "/api/dostk/mrkcond",
    Ka90006Request {
        date: BrokerDate::new("20261012").unwrap(),
        stex_tp: "2".into()
    },
    json!({"date": "20261012", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prm_trde_dfrt_remn_trnsn": [{"buy_dfrt_trde_irds_amt": "-00012", "future": 7}]}),
    |r: &Ka90006Response| {
        let row = &r.prm_trde_dfrt_remn_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.buy_dfrt_trde_irds_amt.as_deref(), Some("-00012"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90007,
    "/api/dostk/mrkcond",
    Ka90007Request {
        date: BrokerDate::new("20261012").unwrap(),
        amt_qty_tp: "2".into(),
        mrkt_tp: "1".into(),
        stex_tp: "3".into()
    },
    json!({"date": "20261012", "amt_qty_tp": "2", "mrkt_tp": "1", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prm_trde_acc_trnsn": [{"all_acc": "-00020", "future": 7}]}),
    |r: &Ka90007Response| {
        let row = &r.prm_trde_acc_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.all_acc.as_deref(), Some("-00020"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90010,
    "/api/dostk/mrkcond",
    Ka90010Request {
        date: BrokerDate::new("20261012").unwrap(),
        amt_qty_tp: "2".into(),
        mrkt_tp: "P001_AL02".into(),
        min_tic_tp: "0".into(),
        stex_tp: "3".into()
    },
    json!({"date": "20261012", "amt_qty_tp": "2", "mrkt_tp": "P001_AL02", "min_tic_tp": "0", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prm_trde_trnsn": [{"cntr_tm": "20261012093000", "future": 7}]}),
    |r: &Ka90010Response| {
        let row = &r.prm_trde_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.cntr_tm.as_deref(), Some("20261012093000"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90013,
    "/api/dostk/mrkcond",
    Ka90013Request {
        stk_cd: "039490_NX".into(),
        amt_qty_tp: Some("2".into()),
        date: Some(BrokerDate::new("20261012").unwrap())
    },
    json!({"stk_cd": "039490_NX", "amt_qty_tp": "2", "date": "20261012"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "stk_daly_prm_trde_trnsn": [{"prm_netprps_qty_irds": "-0010", "future": 7}]}),
    |r: &Ka90013Response| {
        let row = &r.stk_daly_prm_trde_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.prm_netprps_qty_irds.as_deref(), Some("-0010"));
        assert_eq!(row.extra["future"], 7);
    }
);

// 호가·프로그램 조회의 거래소와 시장 조합을 검증한다.
#[tokio::test]
async fn quote_program_exchange_market_relationships_validate() {
    let (mock, c) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    assert!(
        c.ka90005(
            &token,
            &Ka90005Request {
                date: BrokerDate::new("20261012").unwrap(),
                amt_qty_tp: "1".into(),
                mrkt_tp: "P00101".into(),
                min_tic_tp: "0".into(),
                stex_tp: "2".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka90010(
            &token,
            &Ka90010Request {
                date: BrokerDate::new("20261012").unwrap(),
                amt_qty_tp: "1".into(),
                mrkt_tp: "P101_AL02".into(),
                min_tic_tp: "0".into(),
                stex_tp: "3".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka50101(
            &token,
            &Ka50101Request {
                stk_cd: "M04020000".into(),
                tic_scope: "2".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10063(
            &token,
            &Ka10063Request {
                mrkt_tp: "001".into(),
                amt_qty_tp: "2".into(),
                invsr: "6".into(),
                frgn_all: "0".into(),
                smtm_netprps_tp: "0".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(mock.requests.lock().unwrap().is_empty());
}

contract!(
    ka10016,
    "/api/dostk/stkinfo",
    Ka10016Request {
        mrkt_tp: "101".into(),
        ntl_tp: "2".into(),
        high_low_close_tp: "1".into(),
        stk_cnd: "8".into(),
        trde_qty_tp: "00150".into(),
        crd_cnd: "7".into(),
        updown_incls: "1".into(),
        dt: "7".into(),
        stex_tp: "3".into()
    },
    json!({"mrkt_tp": "101", "ntl_tp": "2", "high_low_close_tp": "1", "stk_cnd": "8", "trde_qty_tp": "00150", "crd_cnd": "7", "updown_incls": "1", "dt": "7", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "ntl_pric": [{"pred_trde_qty_pre_rt": "-1.20", "future": 7}]}),
    |r: &Ka10016Response| {
        let row = &r.ntl_pric.as_ref().unwrap()[0];
        assert_eq!(row.pred_trde_qty_pre_rt.as_deref(), Some("-1.20"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10017,
    "/api/dostk/stkinfo",
    Ka10017Request {
        mrkt_tp: "000".into(),
        updown_tp: "7".into(),
        sort_tp: "2".into(),
        stk_cnd: "10".into(),
        trde_qty_tp: "00050".into(),
        crd_cnd: "9".into(),
        trde_gold_tp: "8".into(),
        stex_tp: "2".into()
    },
    json!({"mrkt_tp": "000", "updown_tp": "7", "sort_tp": "2", "stk_cnd": "10", "trde_qty_tp": "00050", "crd_cnd": "9", "trde_gold_tp": "8", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "updown_pric": [{"cnt": "0003", "future": 7}]}),
    |r: &Ka10017Response| {
        let row = &r.updown_pric.as_ref().unwrap()[0];
        assert_eq!(row.cnt.as_deref(), Some("0003"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10018,
    "/api/dostk/stkinfo",
    Ka10018Request {
        high_low_tp: "2".into(),
        alacc_rt: "05".into(),
        mrkt_tp: "001".into(),
        trde_qty_tp: "00000".into(),
        stk_cnd: "5".into(),
        crd_cnd: "0".into(),
        stex_tp: "1".into()
    },
    json!({"high_low_tp": "2", "alacc_rt": "05", "mrkt_tp": "001", "trde_qty_tp": "00000", "stk_cnd": "5", "crd_cnd": "0", "stex_tp": "1"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "high_low_pric_alacc": [{"tdy_low_pric": "+0100", "future": 7}]}),
    |r: &Ka10018Response| {
        let row = &r.high_low_pric_alacc.as_ref().unwrap()[0];
        assert_eq!(row.tdy_low_pric.as_deref(), Some("+0100"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10019,
    "/api/dostk/stkinfo",
    Ka10019Request {
        mrkt_tp: "201".into(),
        flu_tp: "1".into(),
        tm_tp: "1".into(),
        tm: "15".into(),
        trde_qty_tp: "01000".into(),
        stk_cnd: "0".into(),
        crd_cnd: "1".into(),
        pric_cnd: "8".into(),
        updown_incls: "0".into(),
        stex_tp: "3".into()
    },
    json!({"mrkt_tp": "201", "flu_tp": "1", "tm_tp": "1", "tm": "15", "trde_qty_tp": "01000", "stk_cnd": "0", "crd_cnd": "1", "pric_cnd": "8", "updown_incls": "0", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "pric_jmpflu": [{"jmp_rt": "+1.00", "future": 7}]}),
    |r: &Ka10019Response| {
        let row = &r.pric_jmpflu.as_ref().unwrap()[0];
        assert_eq!(row.jmp_rt.as_deref(), Some("+1.00"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10024,
    "/api/dostk/stkinfo",
    Ka10024Request {
        mrkt_tp: "101".into(),
        cycle_tp: "250".into(),
        trde_qty_tp: "1000".into(),
        stex_tp: "2".into()
    },
    json!({"mrkt_tp": "101", "cycle_tp": "250", "trde_qty_tp": "1000", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "trde_qty_updt": [{"prev_trde_qty": "000099", "future": 7}]}),
    |r: &Ka10024Response| {
        let row = &r.trde_qty_updt.as_ref().unwrap()[0];
        assert_eq!(row.prev_trde_qty.as_deref(), Some("000099"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10025,
    "/api/dostk/stkinfo",
    Ka10025Request {
        mrkt_tp: "000".into(),
        prps_cnctr_rt: "100".into(),
        cur_prc_entry: "1".into(),
        prpscnt: "99".into(),
        cycle_tp: "300".into(),
        stex_tp: "1".into()
    },
    json!({"mrkt_tp": "000", "prps_cnctr_rt": "100", "cur_prc_entry": "1", "prpscnt": "99", "cycle_tp": "300", "stex_tp": "1"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "prps_cnctr": [{"pric_strt": "001000", "future": 7}]}),
    |r: &Ka10025Response| {
        let row = &r.prps_cnctr.as_ref().unwrap()[0];
        assert_eq!(row.pric_strt.as_deref(), Some("001000"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10026,
    "/api/dostk/stkinfo",
    Ka10026Request {
        pertp: "6".into(),
        stex_tp: "3".into()
    },
    json!({"pertp": "6", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "high_low_per": [{"per": "0.00", "future": 7}]}),
    |r: &Ka10026Response| {
        let row = &r.high_low_per.as_ref().unwrap()[0];
        assert_eq!(row.per.as_deref(), Some("0.00"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10028,
    "/api/dostk/stkinfo",
    Ka10028Request {
        sort_tp: "4".into(),
        trde_qty_cnd: "0050".into(),
        mrkt_tp: "101".into(),
        updown_incls: "1".into(),
        stk_cnd: "9".into(),
        crd_cnd: "4".into(),
        trde_prica_cnd: "3000".into(),
        flu_cnd: "2".into(),
        stex_tp: "2".into()
    },
    json!({"sort_tp": "4", "trde_qty_cnd": "0050", "mrkt_tp": "101", "updown_incls": "1", "stk_cnd": "9", "crd_cnd": "4", "trde_prica_cnd": "3000", "flu_cnd": "2", "stex_tp": "2"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "open_pric_pre_flu_rt": [{"open_pric_pre": "-1.20", "future": 7}]}),
    |r: &Ka10028Response| {
        let row = &r.open_pric_pre_flu_rt.as_ref().unwrap()[0];
        assert_eq!(row.open_pric_pre.as_deref(), Some("-1.20"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10043,
    "/api/dostk/stkinfo",
    Ka10043Request {
        stk_cd: "039490_AL".into(),
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        qry_dt_tp: "1".into(),
        pot_tp: "0".into(),
        dt: "120".into(),
        sort_base: "2".into(),
        mmcm_cd: "001".into(),
        stex_tp: "3".into()
    },
    json!({"stk_cd": "039490_AL", "strt_dt": "20261001", "end_dt": "20261012", "qry_dt_tp": "1", "pot_tp": "0", "dt": "120", "sort_base": "2", "mmcm_cd": "001", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "trde_ori_prps_anly": [{"trde_wght": "0.50", "future": 7}]}),
    |r: &Ka10043Response| {
        let row = &r.trde_ori_prps_anly.as_ref().unwrap()[0];
        assert_eq!(row.trde_wght.as_deref(), Some("0.50"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10052,
    "/api/dostk/stkinfo",
    Ka10052Request {
        mmcm_cd: "001".into(),
        mrkt_tp: "3".into(),
        qty_tp: "3".into(),
        pric_tp: "8".into(),
        stex_tp: "1".into(),
        stk_cd: Some("039490".into())
    },
    json!({"mmcm_cd": "001", "mrkt_tp": "3", "qty_tp": "3", "pric_tp": "8", "stex_tp": "1", "stk_cd": "039490"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "trde_ori_mont_trde_qty": [{"tp": "-", "future": 7}]}),
    |r: &Ka10052Response| {
        let row = &r.trde_ori_mont_trde_qty.as_ref().unwrap()[0];
        assert_eq!(row.tp.as_deref(), Some("-"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10054,
    "/api/dostk/stkinfo",
    Ka10054Request {
        mrkt_tp: "001".into(),
        bf_mkrt_tp: "0".into(),
        motn_tp: "3".into(),
        skip_stk: "011111111".into(),
        trde_qty_tp: "0".into(),
        min_trde_qty: "".into(),
        max_trde_qty: "".into(),
        trde_prica_tp: "1".into(),
        min_trde_prica: "0".into(),
        max_trde_prica: "100000000".into(),
        motn_drc: "2".into(),
        stex_tp: "2".into(),
        stk_cd: Some(" ".into())
    },
    json!({"mrkt_tp": "001", "bf_mkrt_tp": "0", "motn_tp": "3", "skip_stk": "011111111", "trde_qty_tp": "0", "min_trde_qty": "", "max_trde_qty": "", "trde_prica_tp": "1", "min_trde_prica": "0", "max_trde_prica": "100000000", "motn_drc": "2", "stex_tp": "2", "stk_cd": " "}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "motn_stk": [{"viaplc_tp": "정적", "future": 7}]}),
    |r: &Ka10054Response| {
        let row = &r.motn_stk.as_ref().unwrap()[0];
        assert_eq!(row.viaplc_tp.as_deref(), Some("정적"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10058,
    "/api/dostk/stkinfo",
    Ka10058Request {
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        trde_tp: "1".into(),
        mrkt_tp: "101".into(),
        invsr_tp: "3100".into(),
        stex_tp: "3".into()
    },
    json!({"strt_dt": "20261001", "end_dt": "20261012", "trde_tp": "1", "mrkt_tp": "101", "invsr_tp": "3100", "stex_tp": "3"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "invsr_daly_trde_stk": [{"netslmt_amt": "-0100", "future": 7}]}),
    |r: &Ka10058Response| {
        let row = &r.invsr_daly_trde_stk.as_ref().unwrap()[0];
        assert_eq!(row.netslmt_amt.as_deref(), Some("-0100"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10059,
    "/api/dostk/stkinfo",
    Ka10059Request {
        dt: BrokerDate::new("20261012").unwrap(),
        stk_cd: "039490_NX".into(),
        amt_qty_tp: "2".into(),
        trde_tp: "2".into(),
        unit_tp: "1000".into()
    },
    json!({"dt": "20261012", "stk_cd": "039490_NX", "amt_qty_tp": "2", "trde_tp": "2", "unit_tp": "1000"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "stk_invsr_orgn": [{"natfor": "-0001", "future": 7}]}),
    |r: &Ka10059Response| {
        let row = &r.stk_invsr_orgn.as_ref().unwrap()[0];
        assert_eq!(row.natfor.as_deref(), Some("-0001"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10061,
    "/api/dostk/stkinfo",
    Ka10061Request {
        stk_cd: "039490_AL".into(),
        strt_dt: BrokerDate::new("20261001").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        amt_qty_tp: "1".into(),
        trde_tp: "0".into(),
        unit_tp: "1".into()
    },
    json!({"stk_cd": "039490_AL", "strt_dt": "20261001", "end_dt": "20261012", "amt_qty_tp": "1", "trde_tp": "0", "unit_tp": "1"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "stk_invsr_orgn_tot": [{"etc_corp": "00012", "future": 7}]}),
    |r: &Ka10061Response| {
        let row = &r.stk_invsr_orgn_tot.as_ref().unwrap()[0];
        assert_eq!(row.etc_corp.as_deref(), Some("00012"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10095,
    "/api/dostk/stkinfo",
    Ka10095Request {
        stk_cd: "039490|005930".into()
    },
    json!({"stk_cd": "039490|005930"}),
    json!({"return_code": 0, "unknown_top": {"untouched": true}, "atn_stk_infr": [{"sel_5th_bid": "+0012", "future": 7}]}),
    |r: &Ka10095Response| {
        let row = &r.atn_stk_infr.as_ref().unwrap()[0];
        assert_eq!(row.sel_5th_bid.as_deref(), Some("+0012"));
        assert_eq!(row.extra["future"], 7);
    }
);

// 국내 검색은 코드를 정규화하지 않고 문서에 없는 필터를 거부한다.
#[tokio::test]
async fn domestic_screening_rejects_undocumented_filters_without_normalizing_codes() {
    let (mock, c) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    assert!(
        c.ka10024(
            &token,
            &Ka10024Request {
                mrkt_tp: "001".into(),
                cycle_tp: "250".into(),
                trde_qty_tp: "01000".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10025(
            &token,
            &Ka10025Request {
                mrkt_tp: "001".into(),
                prps_cnctr_rt: "101".into(),
                cur_prc_entry: "1".into(),
                prpscnt: "10".into(),
                cycle_tp: "300".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10026(
            &token,
            &Ka10026Request {
                pertp: "0".into(),
                stex_tp: "1".into()
            }
        )
        .await
        .is_err()
    );
    assert!(
        c.ka10095(
            &token,
            &Ka10095Request {
                stk_cd: "039490||005930".into()
            }
        )
        .await
        .is_err()
    );
    let q = Ka10054Request {
        mrkt_tp: "001".into(),
        bf_mkrt_tp: "0".into(),
        motn_tp: "1".into(),
        skip_stk: "000000002".into(),
        trde_qty_tp: "0".into(),
        min_trde_qty: "".into(),
        max_trde_qty: "".into(),
        trde_prica_tp: "0".into(),
        min_trde_prica: "".into(),
        max_trde_prica: "".into(),
        motn_drc: "0".into(),
        stex_tp: "1".into(),
        stk_cd: None,
    };
    assert!(c.ka10054(&token, &q).await.is_err());
    assert!(mock.requests.lock().unwrap().is_empty());
}

contract!(
    ka10007,
    "/api/dostk/mrkcond",
    Ka10007Request {
        stk_cd: "039490_NX".into()
    },
    json!({"stk_cd":"039490_NX"}),
    json!({"return_code":0,"pred_trde_prica":"00012","buy_10bid_jub_pre":"-0001","lpbuy_10bid_req":"000005","sel_5bid_cnt":"","unknown_top":{"untouched":true}}),
    |r: &Ka10007Response| {
        assert_eq!(r.pred_trde_prica.as_deref(), Some("00012"));
        assert_eq!(r.buy_10bid_jub_pre.as_deref(), Some("-0001"));
        assert_eq!(r.lpbuy_10bid_req.as_deref(), Some("000005"));
        assert_eq!(r.sel_5bid_cnt.as_deref(), Some(""));
    }
);

// 검색 지정 기간은 문서에 정의된 코드를 요구한다.
#[test]
fn screening_designated_period_requires_a_documented_code() {
    let q = Usa20930Request {
        tm_tp: Some("3".into()),
        tm: Some("0".into()),
        ..Default::default()
    };
    assert!(q.validate().is_err());
    let q = Usa20931Request {
        tm_tp: Some("3".into()),
        tm: Some("01".into()),
        ..Default::default()
    };
    assert!(q.validate().is_err());
    let q = Usa20932Request {
        tm_tp: Some("3".into()),
        tm: Some("9".into()),
        ..Default::default()
    };
    assert!(q.validate().is_ok());
}

contract!(
    ka10101,
    "/api/dostk/stkinfo",
    Ka10101Request {
        mrkt_tp: DomesticIndustryMarket::Kosdaq
    },
    json!({"mrkt_tp":"1"}),
    json!({"return_code":0,"list":[{"marketCode":"1","code":"fixture","name":"fixture industry","group":"11","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10101Response| {
        let row = &r.list.as_ref().unwrap()[0];
        assert_eq!(row.market_code.as_deref(), Some("1"));
        assert_eq!(row.group.as_deref(), Some("11"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10046,
    "/api/dostk/mrkcond",
    Ka10046Request {
        stk_cd: "039490_NX".into()
    },
    json!({"stk_cd":"039490_NX"}),
    json!({"return_code":0,"cntr_str_tm":[{"cntr_tm":"093001","cntr_str_60min":"100.00","stex_tp":"NXT","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10046Response| {
        let row = &r.cntr_str_tm.as_ref().unwrap()[0];
        assert_eq!(row.cntr_str_60min.as_deref(), Some("100.00"));
        assert_eq!(row.stex_tp.as_deref(), Some("NXT"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka10047,
    "/api/dostk/mrkcond",
    Ka10047Request {
        stk_cd: "039490_AL".into()
    },
    json!({"stk_cd":"039490_AL"}),
    json!({"return_code":0,"cntr_str_daly":[{"dt":"20261012","cntr_str_5min":"1.00","cntr_str_20min":"20.00","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka10047Response| {
        let row = &r.cntr_str_daly.as_ref().unwrap()[0];
        assert_eq!(row.cntr_str_5min.as_deref(), Some("1.00"));
        assert_eq!(row.cntr_str_20min.as_deref(), Some("20.00"));
        assert_eq!(row.extra["future"], 7);
    }
);

contract!(
    ka90008,
    "/api/dostk/mrkcond",
    Ka90008Request {
        amt_qty_tp: "2".into(),
        stk_cd: "039490_NX".into(),
        date: BrokerDate::new("20261012").unwrap()
    },
    json!({"amt_qty_tp":"2","stk_cd":"039490_NX","date":"20261012"}),
    json!({"return_code":0,"stk_tm_prm_trde_trnsn":[{"tm":"093001","prm_netprps_qty_irds":"-10","stex_tp":"NXT","future":7}],"unknown_top":{"untouched":true}}),
    |r: &Ka90008Response| {
        let row = &r.stk_tm_prm_trde_trnsn.as_ref().unwrap()[0];
        assert_eq!(row.prm_netprps_qty_irds.as_deref(), Some("-10"));
        assert_eq!(row.stex_tp.as_deref(), Some("NXT"));
        assert_eq!(row.extra["future"], 7);
    }
);

// 공개 예제로 시장 입력 형태와 접미사를 확인하고 추정한 값은 거부한다.
#[test]
fn explicit_public_examples_resolve_market_shape_and_suffix_but_reject_guesses() {
    assert!(
        Ka10046Request {
            stk_cd: "039490".into()
        }
        .validate()
        .is_ok()
    );
    assert!(
        Ka10046Request {
            stk_cd: "039490_NX".into()
        }
        .validate()
        .is_ok()
    );
    assert!(
        Ka10047Request {
            stk_cd: "039490_AL".into()
        }
        .validate()
        .is_ok()
    );
    assert!(
        Ka10047Request {
            stk_cd: "039490_OTHER".into()
        }
        .validate()
        .is_err()
    );
    assert!(
        Ka10046Request {
            stk_cd: "ABCDEF".into()
        }
        .validate()
        .is_err()
    );
    assert!(serde_json::from_value::<Ka10101Request>(json!({"mrkt_tp":"3"})).is_err());
    assert!(
        serde_json::from_value::<Ka10101Response>(json!({"list":[{"marketCode":[{}]}]})).is_err()
    );
}

// 공백 필터도 문서의 길이 경계를 지키며 원본 공백을 그대로 전송한다.
#[tokio::test]
async fn ka10054_accepts_blank_and_numeric_length_boundaries_without_normalization() {
    for (qty, amount, stock) in [
        (" ".repeat(12), " ".repeat(10), " ".repeat(20)),
        (
            "000100000000".into(),
            "0100000000".into(),
            "039490_NX".into(),
        ),
    ] {
        let request = vi_length_fixture(qty, amount, Some(stock));
        let wire = serde_json::to_value(&request).unwrap();
        let (mock, client) = setup(json!({"return_code":0}));
        client
            .ka10054(&Secret::new("offline".into()), &request)
            .await
            .unwrap();
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].api_id, "ka10054");
        assert_eq!(sent[0].path, "/api/dostk/stkinfo");
        assert_eq!(sent[0].body, wire);
    }
}

fn vi_length_fixture(qty: String, amount: String, stock: Option<String>) -> Ka10054Request {
    Ka10054Request {
        mrkt_tp: "001".into(),
        bf_mkrt_tp: "0".into(),
        motn_tp: "1".into(),
        skip_stk: "000000000".into(),
        trde_qty_tp: "0".into(),
        min_trde_qty: qty.clone(),
        max_trde_qty: qty,
        trde_prica_tp: "0".into(),
        min_trde_prica: amount.clone(),
        max_trde_prica: amount,
        motn_drc: "0".into(),
        stex_tp: "1".into(),
        stk_cd: stock,
    }
}

// 수량과 금액의 공백 길이를 각각 검사하며 전송 전에 거절한다.
#[tokio::test]
async fn ka10054_rejects_overlong_blank_numeric_fields_before_transport() {
    let (mock, client) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    for field in [
        "min_trde_qty",
        "max_trde_qty",
        "min_trde_prica",
        "max_trde_prica",
    ] {
        let mut request = vi_length_fixture("".into(), "".into(), None);
        match field {
            "min_trde_qty" => request.min_trde_qty = " ".repeat(13),
            "max_trde_qty" => request.max_trde_qty = " ".repeat(13),
            "min_trde_prica" => request.min_trde_prica = " ".repeat(11),
            "max_trde_prica" => request.max_trde_prica = " ".repeat(11),
            _ => unreachable!(),
        }
        assert!(
            matches!(
                client.ka10054(&token, &request).await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "{field}"
        );
        assert!(mock.requests.lock().unwrap().is_empty(), "{field}");
    }
}

// 선택 종목 코드가 공백뿐이어도 최대 길이는 적용한다.
#[tokio::test]
async fn ka10054_rejects_overlong_blank_stock_code_before_transport() {
    let (mock, client) = setup(json!({"return_code":0}));
    let request = vi_length_fixture("".into(), "".into(), Some(" ".repeat(21)));
    assert!(matches!(
        client
            .ka10054(&Secret::new("offline".into()), &request)
            .await,
        Err(kiwoom_client::ClientError::InvalidRequest(_))
    ));
    assert!(mock.requests.lock().unwrap().is_empty());
}

fn new_high_low_fixture(dt: &str) -> Ka10016Request {
    Ka10016Request {
        mrkt_tp: "101".into(),
        ntl_tp: "2".into(),
        high_low_close_tp: "1".into(),
        stk_cnd: "8".into(),
        trde_qty_tp: "00150".into(),
        crd_cnd: "7".into(),
        updown_incls: "1".into(),
        dt: dt.into(),
        stex_tp: "3".into(),
    }
}

// 날짜 수는 세 자리와 250 상한을 함께 지키고 허용된 앞자리 0은 보존한다.
#[tokio::test]
async fn ka10016_enforces_day_count_length_and_value_before_transport() {
    for dt in ["5", "005", "250"] {
        let request = new_high_low_fixture(dt);
        let (mock, client) = setup(json!({"return_code":0}));
        client
            .ka10016(&Secret::new("offline".into()), &request)
            .await
            .unwrap();
        assert_eq!(mock.requests.lock().unwrap()[0].body["dt"], dt);
    }
    let (mock, client) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    for dt in ["0005", "251", "", "-1", " 5"] {
        assert!(
            matches!(
                client.ka10016(&token, &new_high_low_fixture(dt)).await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "{dt:?}"
        );
        assert!(mock.requests.lock().unwrap().is_empty(), "{dt:?}");
    }
}

// 두 집중매수 API의 문서상 조건 3을 허용하고 원본 요청과 경로를 확인한다.
#[tokio::test]
async fn us_concentrated_supply_accepts_documented_condition_three() {
    for cond in ["0", "1", "2", "3"] {
        assert!(
            Usa24220Request {
                cond: Some(cond.into()),
                ..Default::default()
            }
            .validate()
            .is_ok()
        );
        assert!(
            Usa24221Request {
                cond: Some(cond.into()),
                ..Default::default()
            }
            .validate()
            .is_ok()
        );
    }
    let token = Secret::new("offline".into());
    let request = Usa24220Request {
        dt: Some("300".into()),
        prps_cnctr_rt: Some("100".into()),
        prpscnt: Some("99".into()),
        cond: Some("3".into()),
        ..Default::default()
    };
    let (mock, client) = setup(json!({"return_code":0}));
    client.usa24220(&token, &request).await.unwrap();
    {
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].api_id, "usa24220");
        assert_eq!(sent[0].path, "/api/us/stkinfo");
        assert_eq!(
            sent[0].body,
            json!({"dt":"300","prps_cnctr_rt":"100","prpscnt":"99","cond":"3"})
        );
    }

    let request = Usa24221Request {
        etf_cat1: Some("fixture".into()),
        cond: Some("3".into()),
        ..Default::default()
    };
    let (mock, client) = setup(json!({"return_code":0}));
    client.usa24221(&token, &request).await.unwrap();
    let sent = mock.requests.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].api_id, "usa24221");
    assert_eq!(sent[0].path, "/api/us/stkinfo");
    assert_eq!(sent[0].body, json!({"etf_cat1":"fixture","cond":"3"}));
}

// 문서상 마지막 조건 다음 값과 정규화가 필요한 문자열은 전송 전에 거절한다.
#[tokio::test]
async fn us_concentrated_supply_rejects_undocumented_conditions_before_transport() {
    let (mock, client) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    for cond in ["4", "03", " 3"] {
        assert!(
            matches!(
                client
                    .usa24220(
                        &token,
                        &Usa24220Request {
                            cond: Some(cond.into()),
                            ..Default::default()
                        }
                    )
                    .await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "usa24220 {cond:?}"
        );
        assert!(
            matches!(
                client
                    .usa24221(
                        &token,
                        &Usa24221Request {
                            cond: Some(cond.into()),
                            ..Default::default()
                        }
                    )
                    .await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "usa24221 {cond:?}"
        );
        assert!(mock.requests.lock().unwrap().is_empty());
    }
}

// HHMM은 ASCII 네 자리이며 생략과 빈 문자열은 구별하여 보존한다.
#[tokio::test]
async fn ka10084_accepts_exact_hour_minute_boundaries_and_omission() {
    for tm in [None, Some(""), Some("0000"), Some("0930"), Some("2359")] {
        let request = Ka10084Request {
            stk_cd: "039490".into(),
            tdy_pred: "1".into(),
            tic_min: "0".into(),
            tm: tm.map(str::to_owned),
        };
        let (mock, client) = setup(json!({"return_code":0}));
        client
            .ka10084(&Secret::new("offline".into()), &request)
            .await
            .unwrap();
        let sent = mock.requests.lock().unwrap();
        assert_eq!(sent[0].body.get("tm"), tm.map(|s| json!(s)).as_ref());
    }
}

// 시간 파서가 허용하는 공백도 공개 HHMM 형식에서는 전송 전에 거절한다.
#[tokio::test]
async fn ka10084_rejects_whitespace_and_invalid_hour_minute_before_transport() {
    let (mock, client) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    for tm in ["9 30", " 930", "09 3", "2400", "2360", "930"] {
        let request = Ka10084Request {
            stk_cd: "039490".into(),
            tdy_pred: "1".into(),
            tic_min: "0".into(),
            tm: Some(tm.into()),
        };
        assert!(
            matches!(
                client.ka10084(&token, &request).await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "{tm:?}"
        );
        assert!(mock.requests.lock().unwrap().is_empty(), "{tm:?}");
    }
}

// 표현 가능한 정밀도 경계의 양수와 상한값은 원본 숫자 문자열로 전송한다.
#[tokio::test]
async fn us_concentration_decimal_accepts_exact_precision_boundaries() {
    for value in [
        "0",
        "100",
        "0.0000000000000000000000000001",
        "99.99999999999999999999999999",
    ] {
        let request = Usa24220Request {
            prps_cnctr_rt: Some(value.into()),
            ..Default::default()
        };
        assert!(
            Usa24221Request {
                prps_cnctr_rt: Some(value.into()),
                ..Default::default()
            }
            .validate()
            .is_ok()
        );
        let (mock, client) = setup(json!({"return_code":0}));
        client
            .usa24220(&Secret::new("offline".into()), &request)
            .await
            .unwrap();
        assert_eq!(
            mock.requests.lock().unwrap()[0].body,
            json!({"prps_cnctr_rt":value})
        );
    }
}

// 상한 초과와 음수를 반올림해 허용하지 않고 정밀도 손실 전에 거절한다.
#[tokio::test]
async fn us_concentration_decimal_rejects_rounding_across_limits_before_transport() {
    let (mock, client) = setup(json!({"return_code":0}));
    let token = Secret::new("offline".into());
    for value in [
        "100.00000000000000000000000000001",
        "-0.00000000000000000000000000001",
    ] {
        assert!(
            matches!(
                client
                    .usa24220(
                        &token,
                        &Usa24220Request {
                            prps_cnctr_rt: Some(value.into()),
                            ..Default::default()
                        }
                    )
                    .await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "usa24220 {value}"
        );
        assert!(
            matches!(
                client
                    .usa24221(
                        &token,
                        &Usa24221Request {
                            prps_cnctr_rt: Some(value.into()),
                            ..Default::default()
                        }
                    )
                    .await,
                Err(kiwoom_client::ClientError::InvalidRequest(_))
            ),
            "usa24221 {value}"
        );
        assert!(mock.requests.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn shared_screening_price_guard_preserves_each_callers_wire_and_error_order() {
    let token = Secret::new("offline-price-guard".into());

    macro_rules! check {
        ($request:ident, $method:ident) => {
            // 공통 검사로 옮겨도 세 조회가 정확한 문자열을 보내고 잘못된 값은 전송 전에 거절해야 합니다.
            for value in ["0", "0001.20", "0.0000000000000000000000000001"] {
                let (mock, client) = setup(json!({"return_code": 0}));
                client
                    .$method(
                        &token,
                        &$request {
                            pric_cnd_st: Some(value.into()),
                            pric_cnd_ed: Some(value.into()),
                            ..Default::default()
                        },
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    mock.requests.lock().unwrap()[0].body,
                    json!({"pric_cnd_st": value, "pric_cnd_ed": value}),
                );
            }

            for (start, end, reason) in [
                ("", "0", "invalid price filter"),
                ("0", "", "invalid price filter"),
                ("-1", "invalid", "negative price filter"),
                ("invalid", "-1", "invalid price filter"),
                ("0", "-1", "negative price filter"),
                ("0.00000000000000000000000000001", "0", "invalid price filter"),
            ] {
                let (mock, client) = setup(json!({"return_code": 0}));
                assert!(matches!(
                    client
                        .$method(
                            &token,
                            &$request {
                                pric_cnd_st: Some(start.into()),
                                pric_cnd_ed: Some(end.into()),
                                ..Default::default()
                            },
                        )
                        .await,
                    Err(kiwoom_client::ClientError::InvalidRequest(actual)) if actual == reason
                ));
                assert!(mock.requests.lock().unwrap().is_empty());
            }
        };
    }

    check!(Usa20970Request, usa20970);
    check!(Usa20971Request, usa20971);
    check!(Usa20972Request, usa20972);
}
