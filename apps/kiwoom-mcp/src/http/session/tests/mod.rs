use super::*;

// 세션 상한·만료·종료가 취소 신호와 map 제거에 함께 적용됩니다.
#[tokio::test(start_paused = true)]
async fn capacity_expiry_and_shutdown_remove_sessions() {
    let sessions = BoundedSessions::new(1, Duration::from_secs(10));
    let (first, transport) = sessions.create_session().await.unwrap();
    assert!(
        sessions.create_session().await.is_err(),
        "세션 개수 한도를 적용해야 합니다."
    );
    assert!(sessions.has_session(&first).await.unwrap());
    let cancellation = sessions.cancellation_for(&first).await.unwrap();
    tokio::time::advance(Duration::from_secs(11)).await;
    assert!(
        !sessions.has_session(&first).await.unwrap(),
        "만료 ID는 다시 사용할 수 없어야 합니다."
    );
    assert!(
        sessions.inner.sessions.read().await.is_empty(),
        "만료된 세션 map 항목도 제거해야 합니다."
    );
    assert!(
        cancellation.is_cancelled(),
        "만료된 세션의 진행 중인 조회도 취소합니다."
    );
    drop(transport);
    let (second, transport) = sessions.create_session().await.unwrap();
    let cancellation = sessions.cancellation_for(&second).await.unwrap();
    sessions.shutdown().await;
    assert!(
        cancellation.is_cancelled(),
        "서버 종료 시 진행 중인 조회도 취소합니다."
    );
    assert!(sessions.inner.sessions.read().await.is_empty());
    assert!(
        sessions.create_session().await.is_err(),
        "종료 후 새 세션을 만들 수 없어야 합니다."
    );
    drop(transport);
}

// 후속 HTTP 요청이 없어도 만료된 세션을 주기적으로 제거합니다.
#[tokio::test(start_paused = true)]
async fn background_reaper_removes_abandoned_sessions() {
    let sessions = Arc::new(BoundedSessions::new(2, Duration::from_secs(10)));
    let token = CancellationToken::new();
    sessions.spawn_reaper(token.clone());
    let (_, transport) = sessions.create_session().await.unwrap();
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(11)).await;
    tokio::task::yield_now().await;
    assert!(
        sessions.inner.sessions.read().await.is_empty(),
        "새 HTTP 요청이 없어도 만료 항목을 제거해야 합니다."
    );
    token.cancel();
    drop(transport);
}
