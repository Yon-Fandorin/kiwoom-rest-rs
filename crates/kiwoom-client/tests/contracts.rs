use async_trait::async_trait;
use kiwoom_client::{auth::*, transactions::*, transport::*, ws::protocol::*};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Default)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(r);
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request")
    }
}

fn response(
    status: u16,
    body: Value,
    headers: &[(&str, &str)],
) -> Result<HttpResponse, TransportError> {
    Ok(HttpResponse {
        status,
        body,
        headers: headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<BTreeMap<_, _>>(),
    })
}

fn setup(
    responses: Vec<Result<HttpResponse, TransportError>>,
    max_pages: usize,
) -> (Mock, Client<Mock>) {
    let mock = Mock::default();
    mock.responses.lock().unwrap().extend(responses);
    let options = ClientOptions {
        max_pages,
        max_read_retries: 2,
        min_request_interval: Duration::ZERO,
        retry_base_delay: Duration::ZERO,
        ..ClientOptions::default()
    };
    let client = Client::new(mock.clone(), options).unwrap();
    (mock, client)
}

fn request() -> Ust21100Request {
    Ust21100Request {
        strt_dt: BrokerDate::new("20250101").unwrap(),
        end_dt: BrokerDate::new("20251231").unwrap(),
        tp: UsTransactionType::All,
        stex_tp: None,
        stk_cd: None,
        krw_repl_skip_yn: YesNo::No,
    }
}

fn token() -> Secret {
    Secret::new("offline-fixture-token".into())
}

// 빈 OAuth 입력은 전송 전에 거부되어 요청을 남기지 않는다.
#[tokio::test]
async fn empty_oauth_values_are_rejected_before_transport() {
    let (mock, client) = setup(vec![], 2);
    for request in [
        TokenRequest::new(Secret::new("".into()), token()),
        TokenRequest::new(token(), Secret::new(" ".into())),
    ] {
        assert!(matches!(
            client.token(&request).await,
            Err(ClientError::InvalidRequest(_))
        ));
    }
    let request = RevokeRequest::new(token(), token(), Secret::new("".into()));
    assert!(matches!(
        client.revoke(&request).await,
        Err(ClientError::InvalidRequest(_))
    ));
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 다음 페이지 커서를 그대로 전송하고 응답의 원본 필드를 보존한다.
#[tokio::test]
async fn pagination_sends_exact_cursor_and_preserves_original_fields() {
    let (mock, client) = setup(
        vec![
            response(
                200,
                json!({"return_code":0,"result_list":[{"deal_no":"000000001","exct_amt":"+000001.20","future_field":{"x":1}}]}),
                &[("cont-yn", "Y"), ("next-key", "cursor-1")],
            ),
            response(
                200,
                json!({"return_code":0,"result_list":[{"deal_no":"000000002","crnc_code":"USD"}]}),
                &[("cont-yn", "N")],
            ),
        ],
        10,
    );
    let pages = client.ust21100(&token(), &request()).await.unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].response_headers["next-key"], "cursor-1");
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0]
            .exct_amt
            .as_deref(),
        Some("+000001.20")
    );
    assert_eq!(
        pages[0].typed.result_list.as_ref().unwrap()[0].extra["future_field"],
        json!({"x":1})
    );
    assert_eq!(pages[0].raw["result_list"][0]["future_field"]["x"], 1);
    let requests = mock.requests.lock().unwrap();
    assert_eq!(requests[0].path, "/api/us/acnt");
    assert_eq!(requests[0].api_id, "ust21100");
    assert_eq!(
        requests[0].authorization.as_ref().unwrap().expose(),
        "offline-fixture-token"
    );
    assert!(requests[0].continuation.is_none());
    assert_eq!(requests[1].continuation.as_deref(), Some("cursor-1"));
    assert_eq!(requests[0].body["tp"], "0");
    assert!(requests[0].body.get("stk_cd").is_none());
    assert!(requests[0].body.get("stex_tp").is_none());
}

// 반복된 커서는 부분 성공으로 반환하지 않고 오류로 처리한다.
#[tokio::test]
async fn repeated_cursor_is_rejected_without_returning_partial_success() {
    let page = || {
        response(
            200,
            json!({"return_code":0}),
            &[("cont-yn", "Y"), ("next-key", "same")],
        )
    };
    let (mock, client) = setup(vec![page(), page()], 10);
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Protocol("repeated pagination cursor"))
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 2);
}

// 페이지 한도 초과와 누락된 커서는 성공으로 처리하지 않는다.
#[tokio::test]
async fn bounded_pages_and_missing_cursor_fail_closed() {
    let (_, client) = setup(
        vec![response(
            200,
            json!({"return_code":0}),
            &[("cont-yn", "Y"), ("next-key", "next")],
        )],
        1,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::PageLimit)
    ));
    let (_, client) = setup(
        vec![response(200, json!({"return_code":0}), &[("cont-yn", "Y")])],
        2,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Protocol(_))
    ));
}

// 읽기 요청은 속도 제한과 일시적인 전송 실패에 한해 재시도한다.
#[tokio::test]
async fn read_only_retries_rate_limits_and_transient_failure() {
    let (mock, client) = setup(
        vec![
            Err(TransportError),
            response(429, Value::Null, &[("retry-after", "0")]),
            response(200, json!({"return_code":0}), &[]),
        ],
        2,
    );
    assert_eq!(
        client.ust21100(&token(), &request()).await.unwrap().len(),
        1
    );
    assert_eq!(mock.requests.lock().unwrap().len(), 3);
}

// 재시도를 소진하면 마지막 상태를 반환하고 성공 본문을 만들지 않는다.
#[tokio::test]
async fn exhausted_retry_returns_status_and_no_body() {
    let (mock, client) = setup(
        (0..3)
            .map(|_| response(503, json!({"message":"sensitive broker body"}), &[]))
            .collect(),
        2,
    );
    let error = client.ust21100(&token(), &request()).await.unwrap_err();
    assert!(matches!(error, ClientError::Http { status: 503, .. }));
    assert!(!error.to_string().contains("sensitive"));
    assert_eq!(mock.requests.lock().unwrap().len(), 3);
}

// 긴 Retry-After를 기다릴 수 없으면 조기에 재시도하지 않는다.
#[tokio::test]
async fn long_retry_after_is_not_retried_early() {
    let (mock, client) = setup(
        vec![response(429, Value::Null, &[("retry-after", "120")])],
        2,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Http {
            status: 429,
            retry_after: Some(120)
        })
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// 업무 오류와 잘못된 성공 본문은 재시도하지 않는다.
#[tokio::test]
async fn business_error_and_malformed_success_are_not_retried() {
    let (mock, client) = setup(
        vec![response(
            200,
            json!({"return_code":123,"return_msg":"broker detail"}),
            &[],
        )],
        2,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Api(123))
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    let (_, client) = setup(vec![response(200, json!({}), &[])], 2);
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Protocol("missing integer return_code"))
    ));
}

// 토큰 발급과 폐기 응답을 타입으로 해석하며 자동 재시도하지 않는다.
#[tokio::test]
async fn oauth_token_and_revoke_are_typed_and_not_retried() {
    let (mock, client) = setup(
        vec![
            response(
                200,
                json!({"return_code":0,"expires_dt":"20261013000000","token_type":"Bearer","token":"offline-response-token"}),
                &[],
            ),
            response(200, json!({"return_code":0}), &[]),
        ],
        2,
    );
    let req = TokenRequest::new(
        Secret::new("offline-key".into()),
        Secret::new("offline-secret".into()),
    );
    let result = client.token(&req).await.unwrap();
    assert_eq!(result.token.expose(), "offline-response-token");
    assert!(!format!("{result:?}").contains("offline-response-token"));
    client
        .revoke(&RevokeRequest::new(
            Secret::new("offline-key".into()),
            Secret::new("offline-secret".into()),
            result.token,
        ))
        .await
        .unwrap();
    {
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests[0].path, "/oauth2/token");
        assert_eq!(requests[0].body["grant_type"], "client_credentials");
        assert_eq!(requests[1].path, "/oauth2/revoke");
    }
    let (mock, client) = setup(vec![response(429, Value::Null, &[])], 2);
    assert!(matches!(
        client.token(&req).await,
        Err(ClientError::Http { status: 429, .. })
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// 디버그 출력은 비밀값을 가리고 미지원 쓰기 요청은 거부한다.
#[test]
fn debug_redacts_and_unsupported_write_apis_fail_closed() {
    let (_, client) = setup(vec![], 2);
    assert!(matches!(
        client.require_write_opt_in(),
        Err(ClientError::WritesDisabled)
    ));
    assert!(WritePolicy::ExplicitOptIn.require().is_ok());
    assert!(matches!(
        client.require_implemented_api("unlisted"),
        Err(ClientError::UnsupportedApi)
    ));
    let secret = Secret::new("fixture-secret".into());
    assert_eq!(format!("{secret:?}"), "[REDACTED]");
    let mut req = HttpRequest::read_only(
        "/oauth2/token",
        "au10001",
        Some(secret),
        json!({"secretkey":"fixture-secret"}),
    );
    req.continuation = Some("fixture-cursor".into());
    assert!(!format!("{req:?}").contains("fixture-"));
}

// 문서에 없는 날짜와 열거 값을 추정해 허용하지 않는다.
#[test]
fn documented_dates_and_enum_values_reject_guesses() {
    assert!(BrokerDate::new("20250229").is_err());
    assert!(BrokerDate::new("2025-01-01").is_err());
    assert!(serde_json::from_value::<UsTransactionType>(json!("8")).is_err());
    assert_eq!(serde_json::to_value(UsTransactionType::Buys).unwrap(), "4");
    let row: UsTransaction =
        serde_json::from_value(json!({"exct_amt":"10.00","tax_tot_amt":"2.00","uv_exrt":"100.00"}))
            .unwrap();
    // 클라이언트는 2를 다시 차감하거나 단가 100으로 환산 평가액을 계산하지 않는다.
    assert_eq!(row.exct_amt.as_deref(), Some("10.00"));
    assert!(row.crnc_code.is_none());
}

// 국내 조회는 문서의 배열 필드와 요청 경로를 사용한다.
#[tokio::test]
async fn domestic_contract_uses_documented_array_and_path() {
    let (mock, client) = setup(
        vec![response(
            200,
            json!({"return_code":0,"trst_ovrl_trde_prps_array":[{"trde_no":"000000001","trde_qty_jwa_cnt":"0000010","orig_deal_no":"000000000"}]}),
            &[],
        )],
        2,
    );
    let r = Kt00015Request {
        strt_dt: BrokerDate::new("20250101").unwrap(),
        end_dt: BrokerDate::new("20250131").unwrap(),
        tp: DomesticTransactionType::Buys,
        stk_cd: None,
        crnc_cd: None,
        gds_tp: ProductType::DomesticStock,
        frgn_stex_code: None,
        dmst_stex_tp: DomesticExchange::All,
        qry_sort_tp: Some(SortOrder::Oldest),
    };
    let pages = client.kt00015(&token(), &r).await.unwrap();
    assert_eq!(
        pages[0].typed.trst_ovrl_trde_prps_array.as_ref().unwrap()[0]
            .trde_qty_jwa_cnt
            .as_deref(),
        Some("0000010")
    );
    let requests = mock.requests.lock().unwrap();
    assert_eq!(requests[0].path, "/api/dostk/acnt");
    assert_eq!(requests[0].api_id, "kt00015");
    assert_eq!(requests[0].body["dmst_stex_tp"], "%");
    assert!(requests[0].body.get("stk_cd").is_none());
}

// WebSocket 해석은 미지의 FID를 보존하고 오류 응답을 거부한다.
#[test]
fn websocket_boundary_preserves_unknown_fids_and_rejects_errors() {
    let req = StockExecutionSubscription::new(
        RegistrationAction::Register,
        "1",
        RegistrationMode::KeepGroup,
        "039490",
    )
    .unwrap();
    let value = serde_json::to_value(req).unwrap();
    assert_eq!(value["trnm"], "REG");
    assert_eq!(value["data"][0]["type"], json!(["0B"]));
    let message=decode_message(r#"{"trnm":"REAL","data":[{"type":"0B","item":"039490","values":{"20":"093000","10":"+1200","15":"-5","future":"untouched"}}]}"#).unwrap();
    if let ServerMessage::Realtime { items } = message {
        let tick = items[0].stock_execution().unwrap();
        assert_eq!(tick.signed_trade_quantity.as_deref(), Some("-5"));
        assert_eq!(tick.other_fids["future"], "untouched");
    } else {
        panic!("expected realtime")
    }
    assert!(matches!(
        decode_message(r#"{"trnm":"REG","return_code":1}"#),
        Err(ClientError::Api(1))
    ));
    assert!(matches!(
        decode_message(r#"{"trnm":"PING"}"#).unwrap(),
        ServerMessage::Unknown { .. }
    ));
}

// 기존 0B 호환 뷰에서도 명시된 실패·잘못된 결과 코드를 성공 데이터로 만들지 않는다.
#[test]
fn legacy_realtime_rejects_error_codes_and_preserves_valid_frames() {
    assert!(matches!(
        decode_message(r#"{"trnm":"REAL","return_code":7,"data":[]}"#),
        Err(ClientError::Api(7))
    ));
    for code in [json!("0"), json!(null), json!(0.5)] {
        let raw = json!({"trnm":"REAL","return_code":code,"data":[]}).to_string();
        assert!(matches!(
            decode_message(&raw),
            Err(ClientError::Protocol(_))
        ));
    }
    for raw in [
        r#"{"trnm":"REAL","return_code":0,"data":[]}"#,
        r#"{"trnm":"REAL","data":[]}"#,
    ] {
        assert!(matches!(
            decode_message(raw),
            Ok(ServerMessage::Realtime { .. })
        ));
    }
    assert!(matches!(
        decode_message(r#"{"trnm":"FUTURE","return_code":7}"#),
        Ok(ServerMessage::Unknown { .. })
    ));
}

// 누락된 목록과 명시적으로 비어 있는 목록을 구분한다.
#[test]
fn omitted_collection_is_distinct_from_confirmed_empty() {
    let omitted: Ust21100Response = serde_json::from_value(json!({})).unwrap();
    let empty: Ust21100Response = serde_json::from_value(json!({"result_list":[]})).unwrap();
    assert!(omitted.result_list.is_none());
    assert!(empty.result_list.unwrap().is_empty());
}

// 응답 API 불일치와 잘못된 날짜는 성공으로 처리하지 않는다.
#[tokio::test]
async fn response_api_mismatch_and_invalid_dates_fail_closed() {
    let (_, client) = setup(
        vec![response(
            200,
            json!({"return_code":0}),
            &[("api-id", "kt00015")],
        )],
        2,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Protocol("response API identifier mismatch"))
    ));
    let (mock, client) = setup(vec![], 2);
    let mut r = request();
    r.strt_dt = BrokerDate::new("20260101").unwrap();
    assert!(matches!(
        client.ust21100(&token(), &r).await,
        Err(ClientError::InvalidRequest(_))
    ));
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 헤더 이름의 대소문자는 연속 조회와 응답 식별자의 의미를 바꾸지 않는다.
#[tokio::test]
async fn mixed_case_headers_are_checked_and_pagination_is_not_truncated() {
    let (mock, client) = setup(
        vec![
            response(
                200,
                json!({"return_code":0}),
                &[
                    ("Cont-Yn", "Y"),
                    ("Next-Key", "cursor"),
                    ("Api-Id", "ust21100"),
                ],
            ),
            response(
                200,
                json!({"return_code":0}),
                &[("CONT-YN", "N"), ("API-ID", "ust21100")],
            ),
        ],
        3,
    );
    let pages = client.ust21100(&token(), &request()).await.unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].response_headers["next-key"], "cursor");
    assert_eq!(
        mock.requests.lock().unwrap()[1].continuation.as_deref(),
        Some("cursor")
    );
    for headers in [
        vec![("Api-Id", "wrong")],
        vec![("cont-yn", "N"), ("Cont-Yn", "N")],
        vec![("Cont-Yn", "é")],
    ] {
        let (mock, client) = setup(vec![response(200, json!({"return_code":0}), &headers)], 3);
        assert!(matches!(
            client.ust21100(&token(), &request()).await,
            Err(ClientError::Protocol(_))
        ));
        assert_eq!(mock.requests.lock().unwrap().len(), 1);
    }
    let (mock, client) = setup(
        vec![response(429, Value::Null, &[("Retry-After", "120")])],
        3,
    );
    assert!(matches!(
        client.ust21100(&token(), &request()).await,
        Err(ClientError::Http {
            retry_after: Some(120),
            ..
        })
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// REAL 행은 공식 Map 형태여야 하며 Serde가 허용하는 위치 기반 배열로 대신하지 않는다.
#[test]
fn legacy_realtime_requires_object_items_instead_of_positional_arrays() {
    for raw in [
        r#"{"trnm":"REAL","data":[["0B",null,null,{"15":"-5"}]]}"#,
        r#"{"trnm":"REAL","data":[null]}"#,
        r#"{"trnm":"REAL","data":[{"type":"0B","item":"039490","values":{}},["0B",null,null,{}]]}"#,
    ] {
        assert!(matches!(decode_message(raw), Err(ClientError::Protocol(_))));
    }
    assert!(matches!(
        decode_message(r#"{"trnm":"REAL","data":[{"type":"0B","values":{"15":"-5"}}]}"#),
        Ok(ServerMessage::Realtime { .. })
    ));
}
