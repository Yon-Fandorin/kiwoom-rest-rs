//! 합성 전송으로 MCP 발견·호출과 SDK 계약·정책 보존을 검사합니다.
use async_trait::async_trait;
use kiwoom_client::{
    core::Secret,
    rest::{HttpRequest, HttpResponse, Transport, TransportError},
};
use kiwoom_mcp::{BackendTransport, DemoTransport, KiwoomMcp, ToolPolicy};
use rmcp::{
    RoleClient, ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
    service::RunningService,
};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .ok_or(TransportError)
    }
}

async fn connect(transport: impl Transport + 'static) -> RunningService<RoleClient, ClientConfig> {
    let server = KiwoomMcp::new(
        BackendTransport::new(transport),
        Secret::new("synthetic-token".into()),
        true,
    )
    .unwrap();
    connect_server(server).await
}

async fn connect_server(server: KiwoomMcp) -> RunningService<RoleClient, ClientConfig> {
    let (server_stream, client_stream) = tokio::io::duplex(512 * 1024);
    tokio::spawn(async move {
        server
            .serve(server_stream)
            .await
            .unwrap()
            .waiting()
            .await
            .unwrap();
    });
    ClientConfig::default().serve(client_stream).await.unwrap()
}

fn response(body: Value, headers: &[(&str, &str)]) -> HttpResponse {
    HttpResponse {
        status: 200,
        headers: headers
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect::<BTreeMap<_, _>>(),
        body,
    }
}

fn call(name: &str, args: Value) -> CallToolRequestParams {
    CallToolRequestParams::new(name.to_owned()).with_arguments(args.as_object().unwrap().clone())
}

// 도구 목록이 정확히 조회 다섯 개이며 쓰기·인증·WS 기능을 광고하지 않습니다.
#[tokio::test]
async fn lists_only_five_read_tools_with_constrained_inputs() {
    let client = connect(DemoTransport).await;
    let tools = client.list_all_tools().await.unwrap();
    let mut names = tools.iter().map(|t| t.name.as_ref()).collect::<Vec<_>>();
    names.sort();
    assert_eq!(
        names,
        [
            "kiwoom_account_evaluation",
            "kiwoom_daily_chart",
            "kiwoom_order_book",
            "kiwoom_stock_info",
            "kiwoom_stock_list"
        ]
    );
    for tool in &tools {
        assert_eq!(
            tool.annotations.as_ref().unwrap().read_only_hint,
            Some(true)
        );
        assert_eq!(tool.input_schema["additionalProperties"], false);
    }
    let list = tools
        .iter()
        .find(|t| t.name == "kiwoom_stock_list")
        .unwrap();
    assert_eq!(
        list.input_schema["properties"]["mrkt_tp"]["enum"]
            .as_array()
            .unwrap()
            .len(),
        16
    );
    client.cancel().await.unwrap();
}

// 모든 도구가 정확한 API ID·경로·본문으로 SDK typed 호출을 실행합니다.
#[tokio::test]
async fn routes_each_tool_through_named_typed_sdk_call() {
    let cases = [
        (
            "kiwoom_stock_info",
            "ka10001",
            "/api/dostk/stkinfo",
            json!({"stk_cd":"000001"}),
            "01",
        ),
        (
            "kiwoom_order_book",
            "ka10004",
            "/api/dostk/mrkcond",
            json!({"stk_cd":"000001"}),
            "02",
        ),
        (
            "kiwoom_stock_list",
            "ka10099",
            "/api/dostk/stkinfo",
            json!({"mrkt_tp":"10"}),
            "01",
        ),
        (
            "kiwoom_daily_chart",
            "ka10081",
            "/api/dostk/chart",
            json!({"stk_cd":"000001", "base_dt":"20261013", "upd_stkpc_tp":"1"}),
            "07",
        ),
        (
            "kiwoom_account_evaluation",
            "kt00018",
            "/api/dostk/acnt",
            json!({"qry_tp":"2", "dmst_stex_tp":"KRX"}),
            "08",
        ),
    ];
    for (tool, api, path, body, group) in cases {
        let mock = Mock::default();
        mock.responses.lock().unwrap().push_back(response(
            json!({"return_code":0,"future_field":{"raw":"+000001.20"}}),
            &[("cont-yn", "N")],
        ));
        let client = connect(mock.clone()).await;
        let result = client.call_tool(call(tool, body.clone())).await.unwrap();
        assert_eq!(result.is_error, Some(false));
        let content = result.structured_content.unwrap();
        assert_eq!(content["api_id"], api);
        assert_eq!(
            content["source_url"],
            format!("https://openapi.kiwoom.com/guide/apiGuideContents/{group}/{api}")
        );
        assert_eq!(content["backend"], "synthetic_demo");
        assert_eq!(content["live_verified"], false);
        assert_eq!(
            content["pages"][0]["raw"]["future_field"]["raw"],
            "+000001.20"
        );
        assert_eq!(
            content["pages"][0]["typed"]["future_field"]["raw"],
            "+000001.20"
        );
        {
            let requests = mock.requests.lock().unwrap();
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].api_id, api);
            assert_eq!(requests[0].path, path);
            assert_eq!(requests[0].body, body);
        }
        client.cancel().await.unwrap();
    }
}

// 부적합한 코드·날짜·선택값·추가 입력·미지원 주문은 전송 전에 거절됩니다.
#[tokio::test]
async fn rejects_invalid_arguments_and_unsupported_operations_before_io() {
    let mock = Mock::default();
    let client = connect(mock.clone()).await;
    let cases = [
        ("kiwoom_stock_info", json!({"stk_cd":""})),
        (
            "kiwoom_stock_info",
            json!({"stk_cd":"000001", "token":"untrusted"}),
        ),
        ("kiwoom_stock_list", json!({"mrkt_tp":"99"})),
        (
            "kiwoom_daily_chart",
            json!({"stk_cd":"000001", "base_dt":"20260230", "upd_stkpc_tp":"1"}),
        ),
        (
            "kiwoom_daily_chart",
            json!({"stk_cd":"000001", "base_dt":"20261013", "upd_stkpc_tp":"9"}),
        ),
        (
            "kiwoom_account_evaluation",
            json!({"qry_tp":"2", "dmst_stex_tp":"SOR"}),
        ),
        ("kt10000", json!({"stk_cd":"000001"})),
    ];
    for (name, args) in cases {
        if let Ok(result) = client.call_tool(call(name, args)).await {
            assert_eq!(result.is_error, Some(true));
        }
    }
    assert!(mock.requests.lock().unwrap().is_empty());
    client.cancel().await.unwrap();
}

// 연속 조회 헤더와 원문을 보존하며 페이지 상한을 넘으면 일부 결과를 성공으로 내보내지 않습니다.
#[tokio::test]
async fn preserves_pagination_and_refuses_incomplete_results() {
    let mock = Mock::default();
    mock.responses.lock().unwrap().extend([
        response(
            json!({"return_code":0,"cur_prc":"+000001.20"}),
            &[("cont-yn", "Y"), ("next-key", "cursor-1")],
        ),
        response(
            json!({"return_code":0,"cur_prc":"-000001.30"}),
            &[("cont-yn", "N")],
        ),
    ]);
    let client = connect(mock.clone()).await;
    let result = client
        .call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))
        .await
        .unwrap();
    let content = result.structured_content.unwrap();
    assert_eq!(content["complete"], true);
    assert_eq!(content["pages"].as_array().unwrap().len(), 2);
    assert_eq!(
        content["pages"][0]["response_headers"]["next-key"],
        "cursor-1"
    );
    assert_eq!(content["pages"][1]["typed"]["cur_prc"], "-000001.30");
    assert_eq!(
        mock.requests.lock().unwrap()[1].continuation.as_deref(),
        Some("cursor-1")
    );
    for n in 0..4 {
        mock.responses.lock().unwrap().push_back(response(
            json!({"return_code":0}),
            &[("cont-yn", "Y"), ("next-key", &format!("limit-{n}"))],
        ));
    }
    let result = client
        .call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(mock.requests.lock().unwrap().len(), 6);
    client.cancel().await.unwrap();
}

// 응답 크기 상한과 브로커 실패는 원문 오류 본문·토큰을 노출하지 않습니다.
#[tokio::test]
async fn bounds_results_and_sanitizes_broker_errors_without_retry() {
    let mock = Mock::default();
    mock.responses.lock().unwrap().push_back(response(
        json!({"return_code":0,"unknown": "x".repeat(128*1024)}),
        &[],
    ));
    mock.responses.lock().unwrap().push_back(response(
        json!({"return_code":9,"return_msg":"sensitive-upstream-details"}),
        &[],
    ));
    mock.responses.lock().unwrap().push_back(HttpResponse {
        status: 503,
        headers: BTreeMap::new(),
        body: json!({"secret":"sensitive-upstream-details"}),
    });
    let client = connect(mock.clone()).await;
    for _ in 0..3 {
        let result = client
            .call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))
            .await
            .unwrap();
        assert_eq!(result.is_error, Some(true));
        let text = serde_json::to_string(&result).unwrap();
        assert!(!text.contains("sensitive-upstream-details"));
        assert!(!text.contains("synthetic-token"));
    }
    assert_eq!(mock.requests.lock().unwrap().len(), 3);
    client.cancel().await.unwrap();
}

// 증권사 연결 없이 전체 데모 도구를 호출하며 합성 출처와 금융 문자열이 유지됩니다.
#[tokio::test]
async fn demo_is_explicit_and_works_for_all_tools() {
    let client = connect(DemoTransport).await;
    for (name, args) in [
        ("kiwoom_stock_info", json!({"stk_cd":"000001"})),
        ("kiwoom_order_book", json!({"stk_cd":"000001"})),
        ("kiwoom_stock_list", json!({"mrkt_tp":"0"})),
        (
            "kiwoom_daily_chart",
            json!({"stk_cd":"000001", "base_dt":"20261013", "upd_stkpc_tp":"0"}),
        ),
        (
            "kiwoom_account_evaluation",
            json!({"qry_tp":"1", "dmst_stex_tp":"NXT"}),
        ),
    ] {
        let result = client.call_tool(call(name, args)).await.unwrap();
        assert_eq!(result.is_error, Some(false));
        assert_eq!(
            result.structured_content.as_ref().unwrap()["backend"],
            "synthetic_demo"
        );
    }
    client.cancel().await.unwrap();
}

// 프로필이 도구 발견과 직접 실행에 동일하게 적용되고 다른 서버로 전파되지 않습니다.
#[tokio::test]
async fn selected_profiles_filter_discovery_and_dispatch_without_cross_instance_leak() {
    let mock = Mock::default();
    let server = KiwoomMcp::new(
        BackendTransport::new(mock.clone()),
        Secret::new("synthetic-profile-token".into()),
        true,
    )
    .unwrap()
    .with_tool_policy(ToolPolicy::allow_only(vec!["kiwoom_stock_info".into()]).unwrap());
    let client = connect_server(server).await;
    let tools = client.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "kiwoom_stock_info");
    assert!(
        client
            .call_tool(call(
                "kiwoom_account_evaluation",
                json!({"qry_tp":"1", "dmst_stex_tp":"KRX"})
            ))
            .await
            .is_err()
    );
    assert!(mock.requests.lock().unwrap().is_empty());
    let independent = connect(DemoTransport).await;
    assert_eq!(independent.list_all_tools().await.unwrap().len(), 5);
    independent.cancel().await.unwrap();
    client.cancel().await.unwrap();
    let empty = KiwoomMcp::new(
        BackendTransport::new(DemoTransport),
        Secret::new("synthetic-token".into()),
        true,
    )
    .unwrap()
    .with_tool_policy(ToolPolicy::allow_only(vec![]).unwrap());
    let client = connect_server(empty).await;
    assert!(client.list_all_tools().await.unwrap().is_empty());
    assert!(
        client
            .call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))
            .await
            .is_err()
    );
    client.cancel().await.unwrap();
}

// 느린 조회는 전체 마감시간에 중단되며 대기 중인 전송 future가 폐기됩니다.
#[tokio::test(start_paused = true)]
async fn call_deadline_cancels_pending_transport() {
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Slow(Arc<AtomicBool>);
    struct Dropped(Arc<AtomicBool>);
    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    #[async_trait]
    impl Transport for Slow {
        async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, TransportError> {
            let _guard = Dropped(self.0.clone());
            std::future::pending().await
        }
    }
    let dropped = Arc::new(AtomicBool::new(false));
    let client = connect(Slow(dropped.clone())).await;
    let result = client
        .call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    assert!(dropped.load(Ordering::SeqCst));
    client.cancel().await.unwrap();
}

// 같은 서버의 동시 조회가 상한을 넘으면 추가 전송을 시작하지 않습니다.
#[tokio::test(start_paused = true)]
async fn concurrent_tools_share_a_finite_execution_limit() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Slow(Arc<AtomicUsize>);
    #[async_trait]
    impl Transport for Slow {
        async fn execute(&self, _: HttpRequest) -> Result<HttpResponse, TransportError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            std::future::pending().await
        }
    }
    let requests = Arc::new(AtomicUsize::new(0));
    let client = connect(Slow(requests.clone())).await;
    let results = futures::future::join_all(
        (0..9).map(|_| client.call_tool(call("kiwoom_stock_info", json!({"stk_cd":"000001"})))),
    )
    .await;
    assert_eq!(
        results
            .iter()
            .filter(|r| serde_json::to_string(r.as_ref().unwrap())
                .unwrap()
                .contains("동시 조회 한도"))
            .count(),
        1
    );
    assert_eq!(requests.load(Ordering::SeqCst), 8);
    client.cancel().await.unwrap();
}
