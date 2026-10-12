use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use kiwoom_client::{
    Client, ClientOptions, Secret, Transport,
    auth::TokenResponse,
    auth::token_manager::{
        ClientCredentialsIssuer, Clock, CompactExpiryWithOffset, ExpiryInterpreter,
        TokenIssueError, TokenIssuer, TokenManager, TokenManagerError, TokenManagerOptions,
    },
};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;

struct FakeClock(Mutex<DateTime<Utc>>);

impl Clock for FakeClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

impl FakeClock {
    fn set(&self, value: DateTime<Utc>) {
        *self.0.lock().unwrap() = value;
    }
}

struct Issuer {
    calls: AtomicUsize,
    response: Mutex<(String, String, String)>,
    fail: bool,
    wait: Option<Arc<Notify>>,
    started: Notify,
}

#[async_trait]
impl TokenIssuer for Issuer {
    async fn issue(&self) -> Result<TokenResponse, TokenIssueError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.started.notify_one();
        if let Some(wait) = &self.wait {
            wait.notified().await;
        }
        if self.fail {
            return Err(TokenIssueError);
        }
        let response = self.response.lock().unwrap();
        Ok(TokenResponse {
            expires_dt: response.0.clone(),
            token_type: response.1.clone(),
            token: Secret::new(response.2.clone()),
        })
    }
}

fn now() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 12, 0, 0, 0).unwrap()
}

fn issuer(raw: &str, fail: bool, wait: Option<Arc<Notify>>) -> Arc<Issuer> {
    Arc::new(Issuer {
        calls: AtomicUsize::new(0),
        response: Mutex::new((raw.into(), "Bearer".into(), "FAKE-TOKEN-SECRET".into())),
        fail,
        wait,
        started: Notify::new(),
    })
}

fn options() -> TokenManagerOptions {
    TokenManagerOptions {
        refresh_skew: Duration::from_secs(10),
        issuance_timeout: Duration::from_millis(100),
        failure_cooldown: Duration::from_millis(40),
    }
}

fn manager(
    issuer: Arc<Issuer>,
    clock: Arc<FakeClock>,
    options: TokenManagerOptions,
) -> Arc<TokenManager> {
    Arc::new(
        TokenManager::new(
            issuer,
            clock,
            Arc::new(CompactExpiryWithOffset::new(
                FixedOffset::east_opt(0).unwrap(),
            )),
            options,
        )
        .unwrap(),
    )
}

// 명시된 시차로 만료 시각을 해석하고 잘못된 타임스탬프를 거부한다.
#[test]
fn explicit_offset_and_strict_timestamp_validation() {
    let policy = CompactExpiryWithOffset::new(FixedOffset::east_opt(9 * 3600).unwrap());
    assert_eq!(policy.interpret("20261012090000").unwrap(), now());
    for raw in [
        "",
        "2026101209000",
        "202610120900000",
        "20261312090000",
        "20260230090000",
        "20261012240000",
        "20261012000060",
        "20261012T000000",
        "20261012000000Z",
        " 20261012000000",
        "２０２６１０１２００００００",
    ] {
        assert!(
            policy.interpret(raw).is_err(),
            "unexpected acceptance: {raw}"
        );
    }
}

// 만료 경계에서 토큰을 갱신하고 원본 만료 문자열을 보존한다.
#[tokio::test]
async fn expiry_boundary_refreshes_and_raw_expiration_is_retained() {
    let provider = issuer("20261012000100", false, None);
    let clock = Arc::new(FakeClock(Mutex::new(now())));
    let manager = manager(provider.clone(), clock.clone(), options());
    let first = manager.lease().await.unwrap();
    assert_eq!(first.raw_expires_dt(), "20261012000100");
    assert_eq!(first.generation(), 1);
    clock.set(now() + chrono::Duration::seconds(49));
    assert_eq!(manager.lease().await.unwrap().generation(), 1);
    clock.set(now() + chrono::Duration::seconds(50));
    provider.response.lock().unwrap().0 = "20261012000200".into();
    assert_eq!(manager.lease().await.unwrap().generation(), 2);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
}

// 동시 호출 30개가 한 번의 발급과 동일한 토큰 세대를 공유한다.
#[tokio::test]
async fn thirty_parallel_callers_share_one_issuance_and_generation() {
    let release = Arc::new(Notify::new());
    let provider = issuer("20261012000100", false, Some(release.clone()));
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let mut tasks = Vec::new();
    for _ in 0..30 {
        let manager = manager.clone();
        tasks.push(tokio::spawn(async move {
            manager.lease().await.unwrap().generation()
        }));
    }
    provider.started.notified().await;
    release.notify_one();
    for task in tasks {
        assert_eq!(task.await.unwrap(), 1);
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
}

// 오래된 요청의 실패는 새 세대의 토큰을 무효화하지 않는다.
#[tokio::test]
async fn stale_failure_cannot_invalidate_new_generation() {
    let provider = issuer("20261012000100", false, None);
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let old = manager.lease().await.unwrap();
    assert!(manager.invalidate_if_generation(old.generation()).await);
    let fresh = manager.lease().await.unwrap();
    assert_eq!(fresh.generation(), 2);
    assert!(!manager.invalidate_if_generation(old.generation()).await);
    assert_eq!(
        manager.lease().await.unwrap().generation(),
        fresh.generation()
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
}

// 발급 주체의 실패와 냉각 기간을 공유하고 이후 명시적 호출에서 재발급한다.
#[tokio::test]
async fn failed_leader_shares_failure_and_cooldown_then_deliberate_call_retries() {
    let release = Arc::new(Notify::new());
    let provider = issuer("20261012000100", true, Some(release.clone()));
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let mut tasks = Vec::new();
    for _ in 0..20 {
        let manager = manager.clone();
        tasks.push(tokio::spawn(
            async move { manager.lease().await.unwrap_err() },
        ));
    }
    provider.started.notified().await;
    release.notify_one();
    for task in tasks {
        assert_eq!(task.await.unwrap(), TokenManagerError::IssuanceFailed);
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    tokio::time::sleep(Duration::from_millis(45)).await;
    release.notify_one();
    assert_eq!(
        manager.lease().await.unwrap_err(),
        TokenManagerError::IssuanceFailed
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
}

// 발급 주체가 취소되면 대기를 해제하고 냉각 기간 뒤 새 시도를 허용한다.
#[tokio::test]
async fn cancelled_leader_releases_waiters_and_allows_new_attempt_after_cooldown() {
    let release = Arc::new(Notify::new());
    let provider = issuer("20261012000100", false, Some(release.clone()));
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let leader = {
        let manager = manager.clone();
        tokio::spawn(async move { manager.lease().await })
    };
    provider.started.notified().await;
    // 최초 예약 이후 취소되어도 냉각 기간을 다시 전부 적용한다.
    // 대기 중인 호출이 곧바로 두 번째 발급을 시작해서는 안 된다.
    tokio::time::sleep(Duration::from_millis(45)).await;
    let waiter = {
        let manager = manager.clone();
        tokio::spawn(async move { manager.lease().await })
    };
    leader.abort();
    assert!(leader.await.unwrap_err().is_cancelled());
    assert_eq!(
        waiter.await.unwrap().unwrap_err(),
        TokenManagerError::IssuanceCancelled
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    tokio::time::sleep(Duration::from_millis(45)).await;
    release.notify_one();
    assert_eq!(manager.lease().await.unwrap().generation(), 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
}

// 발급 시간 초과를 한도 안에서 공유하며 자동으로 재시도하지 않는다.
#[tokio::test]
async fn timeout_is_bounded_and_shared_without_automatic_retry() {
    let provider = issuer("20261012000100", false, Some(Arc::new(Notify::new())));
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let first = {
        let manager = manager.clone();
        tokio::spawn(async move { manager.lease().await })
    };
    provider.started.notified().await;
    let second = {
        let manager = manager.clone();
        tokio::spawn(async move { manager.lease().await })
    };
    assert_eq!(
        first.await.unwrap().unwrap_err(),
        TokenManagerError::IssuanceTimedOut
    );
    assert_eq!(
        second.await.unwrap().unwrap_err(),
        TokenManagerError::IssuanceTimedOut
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
}

// 잘못된 만료 시각, 지나치게 짧은 유효기간, 잘못된 Bearer는 캐시하지 않는다.
#[tokio::test]
async fn invalid_and_too_short_expiry_or_wrong_bearer_are_never_cached() {
    for (raw, token_type, token, error) in [
        (
            "garbage",
            "Bearer",
            "fake",
            TokenManagerError::InvalidExpiry,
        ),
        (
            "20261012000010",
            "Bearer",
            "fake",
            TokenManagerError::UnusableToken,
        ),
        (
            "20261012000000",
            "Bearer",
            "fake",
            TokenManagerError::UnusableToken,
        ),
        (
            "20261012000100",
            "bearer",
            "fake",
            TokenManagerError::UnusableToken,
        ),
        (
            "20261012000100",
            "Bearer",
            " ",
            TokenManagerError::UnusableToken,
        ),
    ] {
        let provider = issuer(raw, false, None);
        *provider.response.lock().unwrap() = (raw.into(), token_type.into(), token.into());
        let manager = manager(
            provider.clone(),
            Arc::new(FakeClock(Mutex::new(now()))),
            options(),
        );
        assert_eq!(manager.lease().await.unwrap_err(), error);
        assert_eq!(manager.lease().await.unwrap_err(), error);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    }
}

// 디버그와 오류 출력에 토큰이나 인증정보가 포함되지 않는다.
#[tokio::test]
async fn debug_and_errors_never_contain_token_or_credentials() {
    let provider = issuer("20261012000100", false, None);
    let manager = manager(provider, Arc::new(FakeClock(Mutex::new(now()))), options());
    let lease = manager.lease().await.unwrap();
    for formatted in [
        format!("{manager:?}"),
        format!("{lease:?}"),
        format!("{}", TokenManagerError::IssuanceFailed),
    ] {
        assert!(!formatted.contains("FAKE-TOKEN-SECRET"));
    }
    assert_eq!(lease.token().expose(), "FAKE-TOKEN-SECRET");
}

// WebSocket 토큰 공급은 공유 토큰을 재사용하고 실패에서 비밀값을 가린다.
#[tokio::test]
async fn websocket_token_supplier_reuses_shared_lease_and_redacts_failure() {
    let provider = issuer("20261012000100", false, None);
    let manager = manager(
        provider.clone(),
        Arc::new(FakeClock(Mutex::new(now()))),
        options(),
    );
    let lease = manager.lease().await.unwrap();
    let token = kiwoom_client::ws::recovery::WsTokenProvider::token(&manager)
        .await
        .unwrap();
    assert_eq!(token.expose(), lease.token().expose());
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);

    let failure = issuer("SECRET-IN-BROKEN-EXPIRY", true, None);
    let failed_manager = TokenManager::new(
        failure,
        Arc::new(FakeClock(Mutex::new(now()))),
        Arc::new(CompactExpiryWithOffset::new(
            FixedOffset::east_opt(0).unwrap(),
        )),
        options(),
    )
    .unwrap();
    let error = kiwoom_client::ws::recovery::WsTokenProvider::token(&failed_manager)
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "invalid broker response: token acquisition failed"
    );
    assert!(!format!("{error:?}").contains("SECRET-IN-BROKEN-EXPIRY"));
}

// 잘못된 토큰 관리자 한도를 생성 시 거부한다.
#[test]
fn invalid_manager_limits_are_rejected() {
    let provider = issuer("20261012000100", false, None);
    for option in [
        TokenManagerOptions {
            issuance_timeout: Duration::ZERO,
            ..options()
        },
        TokenManagerOptions {
            issuance_timeout: Duration::from_secs(121),
            ..options()
        },
        TokenManagerOptions {
            failure_cooldown: Duration::ZERO,
            ..options()
        },
        TokenManagerOptions {
            failure_cooldown: Duration::from_secs(301),
            ..options()
        },
        TokenManagerOptions {
            refresh_skew: Duration::from_secs(86401),
            ..options()
        },
    ] {
        assert_eq!(
            TokenManager::new(
                provider.clone(),
                Arc::new(FakeClock(Mutex::new(now()))),
                Arc::new(CompactExpiryWithOffset::new(
                    FixedOffset::east_opt(0).unwrap()
                )),
                option
            )
            .unwrap_err(),
            TokenManagerError::InvalidOptions
        );
    }
}

// 현재 시각과 여유 시간의 합이 범위를 넘으면 사용할 수 있는 토큰을 발급하지 않는다.
#[tokio::test]
async fn overflowing_clock_plus_skew_never_issues_a_usable_lease() {
    let provider = issuer("20261012000100", false, None);
    let clock = Arc::new(FakeClock(Mutex::new(DateTime::<Utc>::MAX_UTC)));
    let manager = manager(provider, clock, options());
    assert_eq!(
        manager.lease().await.unwrap_err(),
        TokenManagerError::UnusableToken
    );
}

struct OAuthMock(AtomicUsize);

#[async_trait]
impl Transport for OAuthMock {
    async fn execute(
        &self,
        request: kiwoom_client::transport::HttpRequest,
    ) -> Result<kiwoom_client::transport::HttpResponse, kiwoom_client::transport::TransportError>
    {
        self.0.fetch_add(1, Ordering::SeqCst);
        assert_eq!(request.api_id, "au10001");
        assert_eq!(request.path, "/oauth2/token");
        assert_eq!(request.body["grant_type"], "client_credentials");
        assert_eq!(request.body["appkey"], "FAKE-APPKEY");
        assert_eq!(request.body["secretkey"], "FAKE-SECRETKEY");
        assert!(request.authorization.is_none());
        Ok(kiwoom_client::transport::HttpResponse {
            status: 200,
            headers: Default::default(),
            body: serde_json::json!({"return_code":0,
                "expires_dt":"20261012000100", "token_type":"Bearer", "token":"FAKE-TOKEN"}),
        })
    }
}

// 클라이언트 인증 어댑터는 주입된 키로 기존 타입 기반 발급을 사용한다.
#[tokio::test]
async fn client_credentials_adapter_uses_existing_typed_issuance_with_injected_keys_only() {
    let client = Client::new(
        OAuthMock(AtomicUsize::new(0)),
        ClientOptions {
            min_request_interval: Duration::ZERO,
            ..Default::default()
        },
    )
    .unwrap();
    let issuer = ClientCredentialsIssuer::new(
        client,
        kiwoom_client::auth::TokenRequest::new(
            Secret::new("FAKE-APPKEY".into()),
            Secret::new("FAKE-SECRETKEY".into()),
        ),
    );
    assert!(!format!("{issuer:?}").contains("FAKE-APPKEY"));
    let manager = TokenManager::new(
        Arc::new(issuer),
        Arc::new(FakeClock(Mutex::new(now()))),
        Arc::new(CompactExpiryWithOffset::new(
            FixedOffset::east_opt(0).unwrap(),
        )),
        options(),
    )
    .unwrap();
    assert_eq!(
        manager.lease().await.unwrap().token().expose(),
        "FAKE-TOKEN"
    );
}
