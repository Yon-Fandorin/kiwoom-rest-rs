use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use kiwoom_client::{
    ClientError,
    accounts::{UsTransactionType, Ust21100Request},
    auth::managed_transport::{AuthenticationRetry, ManagedTransport},
    auth::token_manager::{
        Clock, CompactExpiryWithOffset, TokenIssueError, TokenIssuer, TokenManager,
        TokenManagerOptions,
    },
    auth::{TokenRequest, TokenResponse},
    orders::Kt10000Request,
    transactions::{BrokerDate, YesNo},
    transport::{
        Client, ClientOptions, HttpRequest, HttpResponse, Secret, Transport, TransportError,
        WritePolicy,
    },
};
use serde_json::json;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        "2026-10-12T00:00:00Z".parse().unwrap()
    }
}

#[derive(Default)]
struct Issuer(AtomicUsize);

#[async_trait]
impl TokenIssuer for Issuer {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError> {
        let generation = self.0.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(TokenResponse {
            expires_dt: "20261013000000".into(),
            token_type: "Bearer".into(),
            token: Secret::new(format!("synthetic-{generation}")),
        })
    }
}

#[derive(Clone, Default)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    replies: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
}

#[async_trait]
impl Transport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        self.replies
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected replay")
    }
}

fn response(status: u16, code: i64) -> Result<HttpResponse, TransportError> {
    Ok(HttpResponse {
        status,
        headers: BTreeMap::from([("cont-yn".into(), "N".into())]),
        body: json!({"return_code":code,"result_list":[]}),
    })
}

fn setup(
    policy: AuthenticationRetry,
    writes: WritePolicy,
) -> (
    Mock,
    Arc<Issuer>,
    Arc<TokenManager>,
    Client<ManagedTransport<Mock>>,
) {
    let mock = Mock::default();
    let issuer = Arc::new(Issuer::default());
    let tokens = Arc::new(
        TokenManager::new(
            issuer.clone(),
            Arc::new(FixedClock),
            Arc::new(CompactExpiryWithOffset::new(
                FixedOffset::east_opt(0).unwrap(),
            )),
            TokenManagerOptions {
                refresh_skew: Duration::ZERO,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    let client = Client::new(
        ManagedTransport::new(mock.clone(), tokens.clone(), policy),
        ClientOptions {
            min_request_interval: Duration::ZERO,
            retry_base_delay: Duration::ZERO,
            max_read_retries: 2,
            writes,
            ..Default::default()
        },
    )
    .unwrap();
    (mock, issuer, tokens, client)
}

fn request() -> Ust21100Request {
    Ust21100Request {
        strt_dt: BrokerDate::new("20260101").unwrap(),
        end_dt: BrokerDate::new("20261012").unwrap(),
        tp: UsTransactionType::All,
        stex_tp: None,
        stk_cd: None,
        krw_repl_skip_yn: YesNo::No,
    }
}

fn order() -> Kt10000Request {
    serde_json::from_value(json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"100","cond_uv":"50"})).unwrap()
}

fn placeholder() -> Secret {
    Secret::new("caller-placeholder-must-not-be-sent".into())
}

// 인증 읽기는 토큰 관리자를 사용하고 명시된 401 정책에서 한 번만 갱신한다.
#[tokio::test]
async fn authenticated_read_uses_manager_and_explicit_401_policy_refreshes_once() {
    let (mock, issuer, _, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    mock.replies
        .lock()
        .unwrap()
        .extend([response(401, 0), response(200, 0)]);
    client.ust21100(&placeholder(), &request()).await.unwrap();
    assert_eq!(issuer.0.load(Ordering::SeqCst), 2);
    let requests = mock.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].authorization.as_ref().unwrap().expose(),
        "synthetic-1"
    );
    assert_eq!(
        requests[1].authorization.as_ref().unwrap().expose(),
        "synthetic-2"
    );
    assert_eq!(requests[0].body, requests[1].body);
}

// 기본 정책은 401을 그대로 반환하며 추가 발급이나 요청 재전송을 하지 않는다.
#[tokio::test]
async fn default_policy_returns_401_without_issuing_or_replaying_again() {
    let (mock, issuer, _, client) = setup(AuthenticationRetry::Never, WritePolicy::Deny);
    mock.replies.lock().unwrap().push_back(response(401, 0));
    assert!(matches!(
        client.ust21100(&placeholder(), &request()).await,
        Err(ClientError::Http { status: 401, .. })
    ));
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// 반복된 401은 재전송 한도를 지키고 두 번째 토큰을 무효화한다.
#[tokio::test]
async fn repeated_401_is_bounded_and_invalidates_the_second_token() {
    let (mock, issuer, tokens, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    mock.replies
        .lock()
        .unwrap()
        .extend([response(401, 0), response(401, 0)]);
    assert!(matches!(
        client.ust21100(&placeholder(), &request()).await,
        Err(ClientError::Http { status: 401, .. })
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 2);
    assert_eq!(issuer.0.load(Ordering::SeqCst), 2);
    assert_eq!(tokens.lease().await.unwrap().generation(), 3);
}

// 금융 요청 실패는 인증 계층과 바깥 재시도 계층 모두에서 재전송하지 않는다.
#[tokio::test]
async fn financial_failures_never_replay_in_authentication_or_outer_retry_layer() {
    for failure in [
        response(401, 0),
        response(503, 0),
        Err(TransportError),
        response(200, 999),
    ] {
        let (mock, issuer, _, client) = setup(
            AuthenticationRetry::SafeReadHttp401Once,
            WritePolicy::ExplicitOptIn,
        );
        mock.replies.lock().unwrap().push_back(failure);
        assert!(client.kt10000(&placeholder(), &order()).await.is_err());
        assert_eq!(mock.requests.lock().unwrap().len(), 1);
        assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    }
}

// 거부된 금융 요청은 토큰 발급이나 자원 요청을 발생시키지 않는다.
#[tokio::test]
async fn denied_financial_operation_has_no_token_or_resource_side_effect() {
    let (mock, issuer, _, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    assert!(matches!(
        client.kt10000(&placeholder(), &order()).await,
        Err(ClientError::WritesDisabled)
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 0);
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
}

// 브로커 업무 오류를 인증 갱신 사유로 바꾸지 않는다.
#[tokio::test]
async fn broker_business_error_is_not_invented_into_an_authentication_retry() {
    let (mock, issuer, _, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    mock.replies.lock().unwrap().push_back(response(200, 999));
    assert!(matches!(
        client.ust21100(&placeholder(), &request()).await,
        Err(ClientError::Api(999))
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
}

// OAuth 요청은 토큰 관리자의 재귀 발급 없이 한 번만 시도한다.
#[tokio::test]
async fn oauth_is_single_attempt_without_recursive_manager_issuance() {
    let (mock, issuer, _, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    mock.replies.lock().unwrap().push_back(response(401, 0));
    assert!(
        client
            .token(&TokenRequest::new(placeholder(), placeholder()))
            .await
            .is_err()
    );
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
}

// 위장한 금융 경로와 알 수 없는 경로는 토큰 발급 전에 거부한다.
#[tokio::test]
async fn forged_raw_financial_and_unknown_routes_are_denied_before_token_issuance() {
    let (mock, issuer, tokens, _) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    let wrapper = ManagedTransport::new(
        mock.clone(),
        tokens,
        AuthenticationRetry::SafeReadHttp401Once,
    );
    for (path, api_id) in [
        ("/api/dostk/ordr", "kt10000"),
        ("/api/us/acnt", "unregistered"),
    ] {
        assert!(
            wrapper
                .execute(HttpRequest::read_only(path, api_id, None, json!({})))
                .await
                .is_err()
        );
    }
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
    assert!(mock.requests.lock().unwrap().is_empty());
    let debug = format!("{wrapper:?}");
    assert!(!debug.contains("synthetic"));
}

// 단일 페이지의 잘못된 연속조회 정보는 보존하고 후속 커서는 전송 전에 검증한다.
#[tokio::test]
async fn single_page_preserves_invalid_continuation_evidence_and_validates_cursor_before_io() {
    let (mock, _, _, client) = setup(AuthenticationRetry::Never, WritePolicy::Deny);
    let mut raw = response(200, 0).unwrap();
    raw.headers.insert("cont-yn".into(), "Y".into());
    mock.replies.lock().unwrap().push_back(Ok(raw));
    let page = client
        .ust21100_page(&placeholder(), &request(), None)
        .await
        .unwrap();
    assert_eq!(page.response_headers["cont-yn"], "Y");
    for invalid in [" ".to_owned(), "x".repeat(51)] {
        assert!(matches!(
            client
                .ust21100_page(&placeholder(), &request(), Some(&invalid))
                .await,
            Err(ClientError::InvalidRequest(_))
        ));
    }
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// 잘못된 금융 요청에서도 기본 거부가 검증보다 먼저 적용됩니다.
#[tokio::test]
async fn default_deny_precedes_invalid_order_validation_and_token_issuance() {
    let mock = Mock::default();
    let issuer = Arc::new(Issuer::default());
    let client = controlled_client(
        mock.clone(),
        issuer.clone(),
        ClientOptions {
            min_request_interval: Duration::ZERO,
            ..Default::default()
        },
    );
    let mut invalid = order();
    invalid.ord_qty = "-1".into();
    assert!(matches!(
        client.kt10000(&placeholder(), &invalid).await,
        Err(ClientError::WritesDisabled)
    ));
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 명시적 허용 뒤에도 입력 오류는 토큰 임대·발급이나 자원 전송 전에 거부됩니다.
#[tokio::test]
async fn opted_in_invalid_order_precedes_token_lease_and_resource_send() {
    let (mock, issuer, _, client) = setup(
        AuthenticationRetry::SafeReadHttp401Once,
        WritePolicy::ExplicitOptIn,
    );
    let mut invalid = order();
    invalid.ord_qty = "1.5".into();
    assert!(matches!(
        client.kt10000(&placeholder(), &invalid).await,
        Err(ClientError::InvalidRequest(_))
    ));
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
    assert!(mock.requests.lock().unwrap().is_empty());
}

fn controlled_client<T: Transport>(
    transport: T,
    issuer: Arc<dyn TokenIssuer>,
    options: ClientOptions,
) -> Client<ManagedTransport<T>> {
    let tokens = Arc::new(
        TokenManager::new(
            issuer,
            Arc::new(FixedClock),
            Arc::new(CompactExpiryWithOffset::new(
                FixedOffset::east_opt(0).unwrap(),
            )),
            TokenManagerOptions {
                refresh_skew: Duration::ZERO,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    Client::new(
        ManagedTransport::new(transport, tokens, AuthenticationRetry::SafeReadHttp401Once),
        options,
    )
    .unwrap()
}

fn financial_options(interval: Duration) -> ClientOptions {
    ClientOptions {
        writes: WritePolicy::ExplicitOptIn,
        min_request_interval: interval,
        retry_base_delay: Duration::ZERO,
        max_read_retries: 2,
        ..Default::default()
    }
}

#[derive(Default)]
struct PendingOperation {
    started: tokio::sync::Notify,
    release: tokio::sync::Notify,
    attempts: AtomicUsize,
    completed: AtomicUsize,
    cancelled: AtomicUsize,
}

struct PendingGuard {
    operation: Arc<PendingOperation>,
    finished: bool,
}

impl Drop for PendingGuard {
    fn drop(&mut self) {
        if !self.finished {
            self.operation.cancelled.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl PendingOperation {
    async fn wait(self: &Arc<Self>) {
        self.attempts.fetch_add(1, Ordering::SeqCst);
        let mut guard = PendingGuard {
            operation: self.clone(),
            finished: false,
        };
        self.started.notify_one();
        self.release.notified().await;
        guard.finished = true;
        self.completed.fetch_add(1, Ordering::SeqCst);
    }
}

struct PendingIssuer(Arc<PendingOperation>);

#[async_trait]
impl TokenIssuer for PendingIssuer {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError> {
        self.0.wait().await;
        Ok(TokenResponse {
            expires_dt: "20261013000000".into(),
            token_type: "Bearer".into(),
            token: Secret::new("synthetic-pending-issue".into()),
        })
    }
}

#[derive(Clone, Default)]
struct PendingResource {
    operation: Arc<PendingOperation>,
    requests: Arc<Mutex<Vec<HttpRequest>>>,
}

#[async_trait]
impl Transport for PendingResource {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        self.operation.wait().await;
        response(200, 0)
    }
}

// 처음 폴링하기 전 금융 future를 버리면 발급과 전송을 시작하지 않습니다.
#[tokio::test]
async fn dropping_unpolled_financial_future_has_no_side_effect() {
    let (mock, issuer, _, client) = setup(
        AuthenticationRetry::SafeReadHttp401Once,
        WritePolicy::ExplicitOptIn,
    );
    let request = order();
    let token = placeholder();
    let future = client.kt10000(&token, &request);
    drop(future);
    tokio::task::yield_now().await;
    assert_eq!(issuer.0.load(Ordering::SeqCst), 0);
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 발급 시작을 Notify로 확인한 뒤 취소하면 발급 future 자체가 종료되고 전송은 없습니다.
#[tokio::test]
async fn cancelling_financial_future_during_token_issuance_stops_without_background_send() {
    let mock = Mock::default();
    let pending = Arc::new(PendingOperation::default());
    let client = controlled_client(
        mock.clone(),
        Arc::new(PendingIssuer(pending.clone())),
        financial_options(Duration::ZERO),
    );
    let token = placeholder();
    let request = order();
    let mut future = Box::pin(client.kt10000(&token, &request));
    tokio::time::timeout(Duration::from_secs(1), async {
        tokio::select! {
            result = future.as_mut() => panic!("발급 대기 전에 완료됨: {result:?}"),
            _ = pending.started.notified() => {},
        }
    })
    .await
    .expect("토큰 발급 시작 대기 초과");
    drop(future);
    assert_eq!(pending.cancelled.load(Ordering::SeqCst), 1);
    pending.release.notify_one();
    tokio::task::yield_now().await;
    assert_eq!(pending.attempts.load(Ordering::SeqCst), 1);
    assert_eq!(pending.completed.load(Ordering::SeqCst), 0);
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 간격 대기 중 취소된 주문은 대기 시간이 지나도 토큰 발급이나 전송을 재개하지 않습니다.
#[tokio::test]
async fn cancelling_financial_future_during_rate_limit_wait_never_replays() {
    let mock = Mock::default();
    let issuer = Arc::new(Issuer::default());
    let client = controlled_client(
        mock.clone(),
        issuer.clone(),
        financial_options(Duration::from_millis(100)),
    );
    mock.replies.lock().unwrap().push_back(response(200, 0));
    let token = placeholder();
    let request = order();
    client.kt10000(&token, &request).await.unwrap();
    let mut future = Box::pin(client.kt10000(&token, &request));
    assert!(futures_util::poll!(future.as_mut()).is_pending());
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    drop(future);
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}

// 단일 자원 전송 시작 뒤 취소하면 해당 future가 종료되고 숨은 완료·재전송이 없습니다.
#[tokio::test]
async fn cancelling_financial_future_during_resource_send_never_replays() {
    let resource = PendingResource::default();
    let issuer = Arc::new(Issuer::default());
    let client = controlled_client(
        resource.clone(),
        issuer.clone(),
        financial_options(Duration::ZERO),
    );
    let token = placeholder();
    let request = order();
    let mut future = Box::pin(client.kt10000(&token, &request));
    tokio::time::timeout(Duration::from_secs(1), async {
        tokio::select! {
            result = future.as_mut() => panic!("자원 대기 전에 완료됨: {result:?}"),
            _ = resource.operation.started.notified() => {},
        }
    })
    .await
    .expect("자원 전송 시작 대기 초과");
    drop(future);
    assert_eq!(resource.operation.cancelled.load(Ordering::SeqCst), 1);
    resource.operation.release.notify_one();
    tokio::task::yield_now().await;
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert_eq!(resource.operation.attempts.load(Ordering::SeqCst), 1);
    assert_eq!(resource.operation.completed.load(Ordering::SeqCst), 0);
    let sent = resource.requests.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].api_id, "kt10000");
    assert_eq!(sent[0].path, "/api/dostk/ordr");
    assert_eq!(
        sent[0].body,
        json!({"dmst_stex_tp":"KRX","stk_cd":"005930","ord_qty":"1","trde_tp":"0","ord_uv":"100","cond_uv":"50"})
    );
}

#[derive(Default)]
struct FailingIssuer(AtomicUsize);

#[async_trait]
impl TokenIssuer for FailingIssuer {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Err(TokenIssueError)
    }
}

// 읽기 재시도 2회가 설정되어도 금융 작업의 임대 실패는 발급 1회·자원 전송 0회입니다.
#[tokio::test]
async fn financial_token_issuer_failure_is_single_attempt_without_resource_send() {
    let mock = Mock::default();
    let issuer = Arc::new(FailingIssuer::default());
    let client = controlled_client(
        mock.clone(),
        issuer.clone(),
        financial_options(Duration::ZERO),
    );
    assert!(matches!(
        client.kt10000(&placeholder(), &order()).await,
        Err(ClientError::Transport)
    ));
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert!(mock.requests.lock().unwrap().is_empty());
}

// 잘못된 제어 헤더는 관리형 인증을 거쳐도 재시도 가능한 전송 오류로 바뀌지 않습니다.
#[tokio::test]
async fn managed_control_header_error_is_terminal_without_auth_or_read_replay() {
    let (mock, issuer, _, client) =
        setup(AuthenticationRetry::SafeReadHttp401Once, WritePolicy::Deny);
    mock.replies.lock().unwrap().push_back(Ok(HttpResponse {
        status: 200,
        headers: BTreeMap::from([
            ("cont-yn".into(), "N".into()),
            ("Cont-Yn".into(), "Y".into()),
        ]),
        body: json!({"return_code": 0, "result_list": []}),
    }));

    assert!(matches!(
        client.ust21100(&placeholder(), &request()).await,
        Err(ClientError::Protocol("duplicate HTTP control header"))
    ));
    assert_eq!(issuer.0.load(Ordering::SeqCst), 1);
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}
