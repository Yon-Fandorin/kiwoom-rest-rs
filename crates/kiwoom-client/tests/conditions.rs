//! 직접 만든 합성 조건검색 RPC 프레임이며, 네트워크나 인증정보를 사용하지 않는다.

use async_trait::async_trait;
use kiwoom_client::{ClientError, ws::conditions::*};
use kiwoom_client::{
    Secret,
    ws::session::{SessionOptions, WsReply, WsRequest, WsSession, WsTransport},
};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

struct MockSocket {
    responses: VecDeque<String>,
    sent: Arc<Mutex<Vec<Value>>>,
}

#[async_trait]
impl WsTransport for MockSocket {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }
    async fn receive(&mut self) -> Result<String, ClientError> {
        self.responses.pop_front().ok_or(ClientError::Transport)
    }
    async fn close(&mut self) -> Result<(), ClientError> {
        Ok(())
    }
}

async fn session_round_trip<Q: WsRequest>(request: &Q, raw: Value) -> Q::Response {
    let sent = Arc::new(Mutex::new(Vec::new()));
    let socket = MockSocket {
        responses: VecDeque::from([
            json!({"trnm":"LOGIN","return_code":0}).to_string(),
            raw.to_string(),
        ]),
        sent: sent.clone(),
    };
    let mut session = WsSession::new(socket, SessionOptions::default()).unwrap();
    session
        .login(&Secret::new("synthetic-offline-token".into()))
        .await
        .unwrap();
    let response = session.call(request).await.unwrap();
    assert_eq!(
        sent.lock().unwrap()[1],
        serde_json::to_value(request).unwrap()
    );
    assert_eq!(response.raw, raw);
    assert!(response.acknowledged);
    response.typed
}

trait RowCoverage {
    fn assert_row_fields(&self);
}

macro_rules! row_coverage {
    ($($response:ident), *) => {
        $(
            impl RowCoverage for $response {
                fn assert_row_fields(&self) {
                    assert_eq!(self.data.as_ref().unwrap()[0].extra.len(), 1);
                }
            }
        )*
    }
}

row_coverage!(
    Ka10171Response,
    Ka10172Response,
    Ka10173Response,
    Usa20280Response,
    Usa20281Response,
    Usa20290Response
);

impl RowCoverage for Ka10174Response {
    fn assert_row_fields(&self) {}
}

impl RowCoverage for Usa20291Response {
    fn assert_row_fields(&self) {}
}

macro_rules! contract {
    ($test:ident, $req:ident, $decode:ident, $query:expr, $response:expr, $absent_ok:expr) => {
        // 조건검색 RPC의 원본 필드, 세션 왕복, 오류와 필수 입력 경계를 검증한다.
        #[tokio::test]
        async fn $test() {
            let wire: Value = $query;
            let request: $req = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(request.encode().unwrap(), wire);

            let raw: Value = $response;
            let frame = $decode(raw.clone()).unwrap();
            assert_eq!(frame.raw, raw);
            assert_eq!(serde_json::to_value(&frame.typed).unwrap(), raw);
            assert_eq!(frame.typed.extra.len(), 1);
            frame.typed.assert_row_fields();
            let session_response = session_round_trip(&request, raw.clone()).await;
            assert_eq!(serde_json::to_value(&session_response).unwrap(), raw);

            let mut failure = raw.clone();
            failure["return_code"] = json!(37);
            assert!(matches!($decode(failure), Err(ClientError::Api(37))));
            let mut mismatch = raw.clone();
            mismatch["trnm"] = json!("OTHER");
            assert!($decode(mismatch).is_err());
            assert_eq!($decode(json!({})).is_ok(), $absent_ok);

            // 연속조회 선택값 외의 필수 요청 필드는 누락을 거부한다.
            for field in wire.as_object().unwrap().keys() {
                if !matches!(field.as_str(), "cont_yn" | "next_key") {
                    let mut missing = wire.clone();
                    missing.as_object_mut().unwrap().remove(field);
                    assert!(
                        serde_json::from_value::<$req>(missing).is_err(),
                        "missing {field}"
                    );
                }
            }
        }
    };
}

contract!(
    mock_ka10171_contract,
    Ka10171Request,
    decode_ka10171,
    json!({"trnm":"CNSRLST"}),
    json!({"return_code":0,"return_msg":"","trnm":"CNSRLST","data":[{"seq":"001","name":"synthetic domestic condition","unknown_row":true}],"unknown":true}),
    true
);

contract!(
    mock_usa20280_contract,
    Usa20280Request,
    decode_usa20280,
    json!({"trnm":"GCNSRLST"}),
    json!({"return_code":0,"return_msg":"","trnm":"GCNSRLT","data":[{"seq":"002","name":"synthetic US condition","unknown_row":true}],"unknown":true}),
    true
);

contract!(
    mock_ka10172_contract,
    Ka10172Request,
    decode_ka10172,
    json!({"trnm":"CNSRREQ","seq":"001","search_type":"0","stex_tp":"K","cont_yn":"Y","next_key":"synthetic-cursor"}),
    json!({"return_code":0,"return_msg":"","trnm":"CNSRREQ","seq":"001","cont_yn":"Y","next_key":"next-synthetic","data":[{"9001":"A005930","302":"synthetic domestic security","10":"+00000100","25":"2","11":"+00000001","12":"00001500","13":"0000000001","16":"000000100","17":"000000101","18":"000000099","unknown_row":null}],"unknown":true}),
    true
);

contract!(
    mock_usa20281_contract,
    Usa20281Request,
    decode_usa20281,
    json!({"trnm":"GCNSRREQ","seq":"002","search_type":"0","cont_yn":"N"}),
    json!({"return_code":0,"return_msg":"","trnm":"GCNSRREQ","seq":"002","cont_yn":"N","next_key":"","data":[{"9001":"SYNTHETIC","302":"synthetic US security","10":"+00100.0000","25":"2","11":"+00001.0000","12":"+0001.50","13":"0000000001","16":"00100.0000","17":"00101.0000","18":"00099.0000","318":"synthetic industry","stex_tp":"ND","unknown_row":true}],"unknown":true}),
    true
);

contract!(
    mock_ka10173_contract,
    Ka10173Request,
    decode_ka10173,
    json!({"trnm":"CNSRREQ","seq":"001","search_type":"1","stex_tp":"A"}),
    json!({"return_code":0,"return_msg":"","trnm":"CNSRREQ","seq":"001","data":[{"jmcode":"A005930","unknown_row":true}],"unknown":true}),
    true
);

contract!(
    mock_usa20290_contract,
    Usa20290Request,
    decode_usa20290,
    json!({"trnm":"GCNSRREQ","seq":"002","search_type":"1"}),
    json!({"return_code":0,"return_msg":"","trnm":"GCNSRREQ","seq":"002","data":[{"jmcode":"SYNTHETIC","stexTp":"NY","unknown_row":true}],"unknown":true}),
    true
);

contract!(
    mock_ka10174_contract,
    Ka10174Request,
    decode_ka10174,
    json!({"trnm":"CNSRCLR","seq":"001"}),
    json!({"return_code":0,"return_msg":"","trnm":"CNSRCLR","seq":"001","unknown":true}),
    false
);

contract!(
    mock_usa20291_contract,
    Usa20291Request,
    decode_usa20291,
    json!({"trnm":"GCNSRCLR","seq":"002"}),
    json!({"return_code":0,"return_msg":"","trnm":"GCNSRCLR","seq":"002","unknown":true}),
    false
);

// 조건검색 입력과 연속조회 값은 명시된 형식으로 검증한다.
#[test]
fn condition_request_validation_and_continuation_are_explicit() {
    let mut domestic = Ka10172Request {
        trnm: ConditionService::DomesticSearch,
        seq: "001".into(),
        search_type: "0".into(),
        stex_tp: "K".into(),
        cont_yn: None,
        next_key: None,
    };
    let mut us = Usa20281Request {
        trnm: ConditionService::UsSearch,
        seq: "002".into(),
        search_type: "0".into(),
        cont_yn: None,
        next_key: None,
    };
    assert_eq!(
        domestic.encode().unwrap(),
        json!({"trnm":"CNSRREQ","seq":"001","search_type":"0","stex_tp":"K"})
    );
    assert_eq!(
        us.encode().unwrap(),
        json!({"trnm":"GCNSRREQ","seq":"002","search_type":"0"})
    );
    domestic.cont_yn = Some("Y".into());
    us.cont_yn = Some("Y".into());
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.next_key = Some(" ".into());
    us.next_key = Some("".into());
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.next_key = Some("x".repeat(21));
    us.next_key = Some("x".repeat(21));
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.next_key = Some("x".repeat(20));
    us.next_key = Some("x".repeat(20));
    assert!(domestic.encode().is_ok());
    assert!(us.encode().is_ok());
    domestic.cont_yn = Some("?".into());
    us.cont_yn = Some("?".into());
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.cont_yn = None;
    us.cont_yn = None;
    domestic.seq = "0001".into();
    us.seq = "0001".into();
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.seq = " ".into();
    us.seq = " ".into();
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.seq = "001".into();
    us.seq = "002".into();
    domestic.search_type = "1".into();
    us.search_type = "1".into();
    assert!(domestic.encode().is_err());
    assert!(us.encode().is_err());
    domestic.search_type = "0".into();
    domestic.stex_tp = "N".into();
    assert!(domestic.encode().is_err());
    let realtime = Ka10173Request {
        trnm: ConditionService::DomesticSearch,
        seq: "001".into(),
        search_type: "1".into(),
        stex_tp: "K,A".into(),
    };
    assert!(realtime.encode().is_err());
    let realtime_us = Usa20290Request {
        trnm: ConditionService::UsSearch,
        seq: "002".into(),
        search_type: "0".into(),
    };
    assert!(realtime_us.encode().is_err());
    let clear = Ka10174Request {
        trnm: ConditionService::DomesticClear,
        seq: "".into(),
    };
    let clear_us = Usa20291Request {
        trnm: ConditionService::UsClear,
        seq: " ".into(),
    };
    assert!(clear.encode().is_err());
    assert!(clear_us.encode().is_err());
    assert!(
        Ka10171Request {
            trnm: ConditionService::UsList
        }
        .encode()
        .is_err()
    );
    assert!(
        Usa20280Request {
            trnm: ConditionService::DomesticList
        }
        .encode()
        .is_err()
    );
}

// 실시간 조건검색의 값과 미지의 필드를 원형대로 보존한다.
#[test]
fn condition_realtime_values_and_unknown_fields_are_preserved() {
    for exchange in ["K", "ND"] {
        let raw = json!({"trnm":"REAL","data":[{"type":"??","name":"synthetic condition result","values":{"841":"001","9001":"SYNTHETIC","843":"I","20":"120001","907":"2","9081":exchange,"unreviewed_fid":"  +0007 "},"unknown_item":true}],"unknown_frame":true});
        let frame = decode_condition_realtime(raw.clone()).unwrap();
        assert_eq!(frame.raw, raw);
        assert_eq!(serde_json::to_value(&frame.typed).unwrap(), raw);
        assert_eq!(frame.typed.data[0].values.extra.len(), 1);
        let mut invalid = raw.clone();
        invalid["data"][0]["values"]["843"] = json!("?");
        assert!(decode_condition_realtime(invalid).is_err());
        let mut absent = raw;
        absent["data"][0]["values"]
            .as_object_mut()
            .unwrap()
            .remove("841");
        assert!(decode_condition_realtime(absent).is_err());
    }
}

// 미국 조건 목록의 문서 간 서비스명 불일치를 그대로 유지한다.
#[test]
fn us_list_service_document_disagreement_is_retained() {
    for trnm in ["GCNSRLT", "GCNSRLST"] {
        let raw = json!({"return_code":0,"trnm":trnm});
        let frame = decode_usa20280(raw.clone()).unwrap();
        assert_eq!(frame.typed.trnm.as_deref(), Some(trnm));
        assert_eq!(frame.raw, raw);
    }
}

// 선택 결과 코드가 없으면 확인 응답을 만들어내지 않는다.
#[tokio::test]
async fn optional_result_code_does_not_fabricate_an_acknowledgement() {
    let raw = json!({"trnm":"GCNSRLST", "data":null, "unknown":"preserved"});
    let sent = Arc::new(Mutex::new(Vec::new()));
    let socket = MockSocket {
        responses: VecDeque::from([
            json!({"trnm":"LOGIN","return_code":0}).to_string(),
            raw.to_string(),
        ]),
        sent,
    };
    let mut session = WsSession::new(socket, SessionOptions::default()).unwrap();
    session
        .login(&Secret::new("synthetic-offline-token".into()))
        .await
        .unwrap();
    let response = session.call(&Usa20280Request::default()).await.unwrap();
    assert!(!response.acknowledged);
    assert!(response.typed.return_code.is_none());
    assert!(response.typed.data.is_none());
    assert_eq!(response.raw, raw);
    assert!(response.raw.get("data").unwrap().is_null());
}

// 아래 기대값은 2026-10-12 공개 HTML 필드 표로 확인했다. 값은 모두 직접 만든 구별 문자열이다.
// 15/ka10171 SHA256: 8f74ac777f77e5944a6331831e47dd6094a9d81ec1fdc19c1467a3681beb809e
// 15/ka10172 SHA256: bb5807f91bd11a5cf60ba50dbf75ed95213e791143032ff00995f126d39bf972
// 15/ka10173 SHA256: a15ec3e9abd08c9f8d49894557a0d1d919774b1e13ad0bf707017278a62ea7ac
// 39/usa20280 SHA256: e8d988fdd030ffcd81819a52219b3fe8ce262a55473a6707cd3959a917eeae69
// 39/usa20281 SHA256: 07243b0c2668a0d6714d3bffc33a8abeba6d6bc12b0f5f0886415bc00e581649
// 39/usa20290 SHA256: efaa7c444c2eb2ad818d7d448c027e98069f4e308868aec42f63ac1674b05e22
// URL 접두어: https://openapi.kiwoom.com/guide/apiGuideContents/
// 생산 코드 필드 목록이나 공식 예제에서 값을 생성하지 않으므로 이름 매핑 교환을 검출한다.
#[test]
fn condition_rows_have_independent_named_bindings() {
    macro_rules! named_row {
        ($decode:ident, $service:literal, {$($field:ident => $wire:literal),+ $(,)?}) => {{
            let raw = json!({"trnm":$service,"data":[{
                $($wire: concat!("synthetic-row-", $wire, ": +000.2500 "),)+
                "unknown_row": {"preserved":true}
            }]});
            let frame = $decode(raw.clone()).unwrap();
            let row = &frame.typed.data.as_ref().unwrap()[0];
            $(assert_eq!(row.$field.as_deref(), Some(concat!("synthetic-row-", $wire, ": +000.2500 ")),
                concat!(stringify!($field), " <- ", $wire));)+
            assert_eq!(row.extra.len(), 1);
            assert_eq!(row.extra["unknown_row"], json!({"preserved":true}));
            assert_eq!(frame.raw, raw);
        }};
    }
    named_row!(decode_ka10171, "CNSRLST", {seq => "seq", name => "name"});
    named_row!(decode_usa20280, "GCNSRLST", {seq => "seq", name => "name"});
    named_row!(decode_ka10172, "CNSRREQ", {
        instrument => "9001", name => "302", current_price => "10", change_sign => "25",
        change => "11", change_percent => "12", cumulative_volume => "13",
        open => "16", high => "17", low => "18"
    });
    named_row!(decode_usa20281, "GCNSRREQ", {
        instrument => "9001", name => "302", current_price => "10", change_sign => "25",
        change => "11", change_percent => "12", cumulative_volume => "13",
        open => "16", high => "17", low => "18", industry => "318", exchange => "stex_tp"
    });
    named_row!(decode_ka10173, "CNSRREQ", {jmcode => "jmcode"});
    named_row!(decode_usa20290, "GCNSRREQ", {jmcode => "jmcode", stex_tp => "stexTp"});
}

// ka10173/usa20290의 REAL 표에 공통으로 명시된 여섯 FID를 이름으로 각각 읽는다.
// I/D는 문서 선택값을 사용하며 나머지 문자열은 합성값이다. 실거래 의미는 변환하지 않는다.
#[test]
fn condition_notification_has_independent_named_bindings() {
    for change in ["I", "D"] {
        let raw = json!({"trnm":"REAL","data":[{
            "type":"??","name":"synthetic-notification-name",
            "values":{"841":"synthetic-sequence","9001":"synthetic-instrument",
                "843":change,"20":"synthetic-time","907":"synthetic-side",
                "9081":"synthetic-exchange","unknown_fid":"kept"}
        }]});
        let frame = decode_condition_realtime(raw.clone()).unwrap();
        let item = &frame.typed.data[0];
        assert_eq!(item.api_type, "??");
        assert_eq!(item.name, "synthetic-notification-name");
        assert_eq!(item.values.seq, "synthetic-sequence");
        assert_eq!(item.values.instrument, "synthetic-instrument");
        assert_eq!(item.values.change, change);
        assert_eq!(item.values.execution_time, "synthetic-time");
        assert_eq!(item.values.sell_buy_type, "synthetic-side");
        assert_eq!(item.values.exchange, "synthetic-exchange");
        assert_eq!(item.values.extra["unknown_fid"], "kept");
        assert_eq!(frame.raw, raw);
    }
}

// 여덟 이름 있는 세션 메서드의 요청·응답 타입을 컴파일하고 실제 MockSocket 왕복을 검증한다.
// 전송 기대값은 요청 재직렬화로 만들지 않는다. 목록·일회검색·실시간검색·해제를 명시한다.
#[tokio::test]
async fn named_condition_methods_send_typed_requests_and_return_wire_evidence() {
    let wires = [
        json!({"trnm":"CNSRLST"}),
        json!({"trnm":"CNSRREQ","seq":"101","search_type":"0","stex_tp":"K","cont_yn":"N"}),
        json!({"trnm":"CNSRREQ","seq":"102","search_type":"1","stex_tp":"A"}),
        json!({"trnm":"CNSRCLR","seq":"102"}),
        json!({"trnm":"GCNSRLST"}),
        json!({"trnm":"GCNSRREQ","seq":"201","search_type":"0","cont_yn":"Y","next_key":"synthetic-cursor"}),
        json!({"trnm":"GCNSRREQ","seq":"202","search_type":"1"}),
        json!({"trnm":"GCNSRCLR","seq":"202"}),
    ];
    let raw = [
        json!({"trnm":"CNSRLST","return_code":0,"data":[{"seq":"101","name":"synthetic domestic list"}],"future":"list-domestic"}),
        json!({"trnm":"CNSRREQ","return_code":0,"seq":"101","data":[{"9001":"A500001","10":"+00101100"}],"future":"once-domestic"}),
        json!({"trnm":"CNSRREQ","return_code":0,"seq":"102","data":[{"jmcode":"A500002"}],"future":"realtime-domestic"}),
        json!({"trnm":"CNSRCLR","return_code":0,"return_msg":"synthetic domestic cleared","seq":"102","future":"clear-domestic"}),
        json!({"trnm":"GCNSRLT","return_code":0,"data":[{"seq":"201","name":"synthetic US list"}],"future":"list-us"}),
        json!({"trnm":"GCNSRREQ","return_code":0,"seq":"201","data":[{"9001":"SYNTH1","318":"synthetic industry"}],"future":"once-us"}),
        json!({"trnm":"GCNSRREQ","return_code":0,"seq":"202","data":[{"jmcode":"SYNTH2","stexTp":"NY"}],"future":"realtime-us"}),
        json!({"trnm":"GCNSRCLR","return_code":0,"return_msg":"synthetic US cleared","seq":"202","future":"clear-us"}),
    ];
    let sent = Arc::new(Mutex::new(Vec::new()));
    let socket = MockSocket {
        responses: std::iter::once(json!({"trnm":"LOGIN","return_code":0}).to_string())
            .chain(raw.iter().map(Value::to_string))
            .collect(),
        sent: sent.clone(),
    };
    let mut session = WsSession::new(socket, SessionOptions::default()).unwrap();
    session
        .login(&Secret::new("synthetic-named-method-token".into()))
        .await
        .unwrap();

    macro_rules! reply_evidence {
        ($reply:ident, $index:literal) => {
            assert!($reply.acknowledged);
            assert_eq!($reply.raw, raw[$index]);
            assert_eq!($reply.typed.extra["future"], raw[$index]["future"]);
        };
    }
    let reply: WsReply<Ka10171Response> = session
        .ka10171(&serde_json::from_value::<Ka10171Request>(wires[0].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 0);
    assert_eq!(
        reply.typed.data.unwrap()[0].name.as_deref(),
        Some("synthetic domestic list")
    );
    let reply: WsReply<Ka10172Response> = session
        .ka10172(&serde_json::from_value::<Ka10172Request>(wires[1].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 1);
    assert_eq!(
        reply.typed.data.unwrap()[0].current_price.as_deref(),
        Some("+00101100")
    );
    let reply: WsReply<Ka10173Response> = session
        .ka10173(&serde_json::from_value::<Ka10173Request>(wires[2].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 2);
    assert_eq!(
        reply.typed.data.unwrap()[0].jmcode.as_deref(),
        Some("A500002")
    );
    let reply: WsReply<Ka10174Response> = session
        .ka10174(&serde_json::from_value::<Ka10174Request>(wires[3].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 3);
    assert_eq!(reply.typed.return_msg, "synthetic domestic cleared");
    assert_eq!(reply.typed.seq, "102");
    let reply: WsReply<Usa20280Response> = session
        .usa20280(&serde_json::from_value::<Usa20280Request>(wires[4].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 4);
    assert_eq!(
        reply.typed.data.unwrap()[0].name.as_deref(),
        Some("synthetic US list")
    );
    let reply: WsReply<Usa20281Response> = session
        .usa20281(&serde_json::from_value::<Usa20281Request>(wires[5].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 5);
    assert_eq!(
        reply.typed.data.unwrap()[0].industry.as_deref(),
        Some("synthetic industry")
    );
    let reply: WsReply<Usa20290Response> = session
        .usa20290(&serde_json::from_value::<Usa20290Request>(wires[6].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 6);
    assert_eq!(reply.typed.data.unwrap()[0].stex_tp.as_deref(), Some("NY"));
    let reply: WsReply<Usa20291Response> = session
        .usa20291(&serde_json::from_value::<Usa20291Request>(wires[7].clone()).unwrap())
        .await
        .unwrap();
    reply_evidence!(reply, 7);
    assert_eq!(reply.typed.return_msg, "synthetic US cleared");
    assert_eq!(reply.typed.seq, "202");
    let sent = sent.lock().unwrap();
    assert_eq!(sent.len(), 9);
    assert_eq!(&sent[1..], wires.as_slice());
}
