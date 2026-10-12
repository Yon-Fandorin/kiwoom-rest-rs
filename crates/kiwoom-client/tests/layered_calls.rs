use async_trait::async_trait;
use kiwoom_client::rest::{HttpRequest, HttpResponse, TransportError};
use kiwoom_client::{core::Secret, domain::accounts::Ka00001Request, rest};
use serde_json::json;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

struct OfflineHttp(Arc<Mutex<Vec<HttpRequest>>>);
#[async_trait]
impl rest::Transport for OfflineHttp {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.0.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::new(),
            body: json!({"return_code": 0, "acctNo": "synthetic-account", "future": true}),
        })
    }
}

#[tokio::test]
async fn rest_entry_calls_a_domain_contract_through_the_injected_port() {
    let sent = Arc::new(Mutex::new(Vec::new()));
    let client = rest::Client::new(
        OfflineHttp(sent.clone()),
        rest::ClientOptions {
            min_request_interval: Duration::ZERO,
            ..Default::default()
        },
    )
    .unwrap();
    let pages = client
        .ka00001(
            &Secret::new("offline-layer-test".into()),
            &Ka00001Request {},
        )
        .await
        .unwrap();
    assert_eq!(pages[0].typed.acct_no.as_deref(), Some("synthetic-account"));
    assert_eq!(pages[0].typed.extra["future"], true);
    assert_eq!(pages[0].raw["future"], true);
    let sent = sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].api_id, "ka00001");
    assert_eq!(sent[0].path, "/api/dostk/acnt");
    assert_eq!(sent[0].body, json!({}));
}
