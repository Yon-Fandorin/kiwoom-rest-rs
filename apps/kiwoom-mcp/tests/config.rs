//! 사용자 프로필의 누락·오류·빈 허용 목록을 전체 허용으로 대체하지 않는지 검사합니다.
use kiwoom_mcp::ToolPolicy;

// 설정 버전·프로필·지원 목록을 검증하고 잘못된 설정은 실패합니다.
#[test]
fn validates_profiles_without_fail_open() {
    let valid = br#"{"version":1,"profiles":{"research":{"tools":["kiwoom_stock_info"]},"disabled":{"tools":[]}}}"#;
    assert!(ToolPolicy::from_json(valid, "research").is_ok());
    assert!(ToolPolicy::from_json(valid, "disabled").is_ok());
    assert!(ToolPolicy::from_json(valid, "unknown").is_err());
    for input in [
        r#"{"version":2,"profiles":{"default":{"tools":[]}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":["kt10000"]}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":["kiwoom_stock_info","kiwoom_stock_info"]}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":[],"token":"secret"}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":[]},"broken":{"tools":["unknown"]}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":"*"}}}"#,
        r#"{"version":1,"profiles":{"default":{}}}"#,
        r#"{"version":1,"profiles":{"default":{"tools":[]}},"extra":true}"#,
    ] {
        assert!(ToolPolicy::from_json(input.as_bytes(), "default").is_err());
    }
    assert!(ToolPolicy::from_json(&vec![b' '; 64 * 1024 + 1], "default").is_err());
}
