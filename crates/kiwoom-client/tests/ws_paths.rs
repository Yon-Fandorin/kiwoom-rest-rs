use async_trait::async_trait;
use kiwoom_client::{ClientError, Secret, ws};
use serde_json::{Value, json};
use std::{
    any::TypeId,
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

struct OfflineSocket {
    incoming: VecDeque<String>,
    sent: Arc<Mutex<Vec<Value>>>,
}

// 기존 포트로 작성한 구현이 새 세션에서도 그대로 사용할 수 있어야 합니다.
#[async_trait]
impl kiwoom_client::ws_session::WsTransport for OfflineSocket {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }

    async fn receive(&mut self) -> Result<String, ClientError> {
        self.incoming.pop_front().ok_or(ClientError::Transport)
    }

    async fn close(&mut self) -> Result<(), ClientError> {
        Ok(())
    }
}

#[test]
fn canonical_and_existing_paths_name_the_same_types() {
    fn same<A: 'static, B: 'static>() {
        assert_eq!(TypeId::of::<A>(), TypeId::of::<B>());
    }
    same::<
        ws::session::WsSession<OfflineSocket>,
        kiwoom_client::ws_session::WsSession<OfflineSocket>,
    >();
    same::<ws::session::WsReply<Value>, kiwoom_client::ws_session::WsReply<Value>>();
    same::<ws::session::SessionOptions, kiwoom_client::ws_session::SessionOptions>();
    same::<ws::transport::TungsteniteFactory, kiwoom_client::ws_transport::TungsteniteFactory>();
    same::<ws::transport::TungsteniteTransport, kiwoom_client::ws_transport::TungsteniteTransport>(
    );
    same::<ws::recovery::RecoveryOptions, kiwoom_client::ws_recovery::RecoveryOptions>();
    same::<ws::conditions::Ka10171Request, kiwoom_client::conditions::Ka10171Request>();
    same::<ws::realtime::StockExecutionValues, kiwoom_client::realtime::StockExecutionValues>();
    same::<ws::subscription::RealtimeSubscription, kiwoom_client::realtime::RealtimeSubscription>();
    same::<
        ws::protocol::StockExecutionSubscription,
        kiwoom_client::websocket::StockExecutionSubscription,
    >();
}

#[tokio::test]
async fn canonical_session_accepts_existing_port_and_contract_paths() {
    let sent = Arc::new(Mutex::new(Vec::new()));
    let socket = OfflineSocket {
        sent: sent.clone(),
        incoming: VecDeque::from([
            json!({"trnm":"LOGIN","return_code":0}).to_string(),
            json!({"trnm":"CNSRLST","return_code":0,"data":[{"seq":"001","name":"synthetic","future":true}]}).to_string(),
        ]),
    };
    let mut session = ws::Client::new(
        socket,
        ws::session::SessionOptions {
            timeout: Duration::from_secs(1),
            ..Default::default()
        },
    )
    .unwrap();
    session
        .login(&Secret::new("offline-path-test".into()))
        .await
        .unwrap();
    let request = kiwoom_client::conditions::Ka10171Request::default();
    let reply: kiwoom_client::ws_session::WsReply<ws::conditions::Ka10171Response> =
        session.ka10171(&request).await.unwrap();
    assert!(reply.acknowledged);
    assert_eq!(
        reply.typed.data.as_ref().unwrap()[0].name.as_deref(),
        Some("synthetic")
    );
    assert_eq!(reply.raw["data"][0]["future"], true);
    assert_eq!(
        *sent.lock().unwrap(),
        vec![
            json!({"trnm":"LOGIN","token":"offline-path-test"}),
            json!({"trnm":"CNSRLST"}),
        ]
    );
}
