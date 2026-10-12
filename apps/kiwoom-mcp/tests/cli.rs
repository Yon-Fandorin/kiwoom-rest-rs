//! 실제 실행 파일을 비밀 값 없는 환경에서 실행하고 stdio와 실행 옵션을 확인합니다.
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
};
use serde_json::json;
use std::{
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const BIN: &str = env!("CARGO_BIN_EXE_kiwoom-mcp");

// 명시적인 허용 없이 broker 실행을 막고 무효·혼동되는 설정도 실패합니다.
#[test]
fn cli_rejects_network_and_invalid_modes_before_execution() {
    for args in [
        vec!["--backend", "broker"],
        vec!["--transport", "stdio", "--session-mode", "stateful"],
        vec!["--transport", "http", "--bind", "0.0.0.0:8080"],
        vec!["--transport", "http", "--bind", "127.0.0.1:0"],
        vec!["--allow-broker-network"],
        vec!["--environment", "production"],
        vec!["--profile", "research"],
        vec!["--transport", "http"],
    ] {
        let output = Command::new(BIN).env_clear().args(&args).output().unwrap();
        assert!(!output.status.success(), "설정이 거부되어야 함: {args:?}");
        assert!(output.stdout.is_empty());
    }
    assert!(
        Command::new(BIN)
            .env_clear()
            .arg("--help")
            .output()
            .unwrap()
            .status
            .success()
    );
}

// 실제 프로세스가 설정 프로필로 제한된 MCP 도구만 제공하고 stdout에는 프로토콜만 씁니다.
#[tokio::test]
async fn stdio_binary_uses_explicit_profile_and_synthetic_backend() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "kiwoom-mcp-profile-{}-{nonce}.json",
        std::process::id()
    ));
    std::fs::write(
        &path,
        r#"{"version":1,"profiles":{"research":{"tools":["kiwoom_stock_info"]}}}"#,
    )
    .unwrap();
    let mut child = tokio::process::Command::new(BIN)
        .env_clear()
        .args(["--config", path.to_str().unwrap(), "--profile", "research"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let client = ClientConfig::default()
        .serve((child.stdout.take().unwrap(), child.stdin.take().unwrap()))
        .await
        .unwrap();
    let tools = client.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "kiwoom_stock_info");
    let result = client
        .call_tool(
            CallToolRequestParams::new("kiwoom_stock_info")
                .with_arguments(json!({"stk_cd":"000001"}).as_object().unwrap().clone()),
        )
        .await
        .unwrap();
    assert_eq!(
        result.structured_content.as_ref().unwrap()["backend"],
        "synthetic_demo"
    );
    client.cancel().await.unwrap();
    let status = tokio::time::timeout(std::time::Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap();
    assert!(status.success());
    std::fs::remove_file(path).unwrap();
}
