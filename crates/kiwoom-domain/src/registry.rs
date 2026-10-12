//! 독립적으로 작성한 구체적 계약만 전송 경로 집합에 포함합니다.
const CORE_CONTRACTS: &[(&str, &str)] =
    &[("au10001", "/oauth2/token"), ("au10002", "/oauth2/revoke")];
fn http_contracts() -> impl Iterator<Item = (&'static str, &'static str)> {
    CORE_CONTRACTS
        .iter()
        .copied()
        .chain(crate::accounts::CONTRACTS.iter().copied())
        .chain(crate::market::CONTRACTS.iter().copied())
        .chain(crate::charts::CONTRACTS.iter().copied())
        .chain(crate::analytics::CONTRACTS.iter().copied())
        .chain(crate::products::CONTRACTS.iter().copied())
        .chain(crate::orders::READ_CONTRACTS.iter().copied())
        .chain(crate::orders::WRITE_CONTRACTS.iter().copied())
}

/// 독립 HTTP 계약에 정확한 API-ID·경로 쌍이 등록되어 있는지 확인합니다.
pub fn implemented_pair(id: &str, path: &str) -> bool {
    http_contracts().any(|pair| pair == (id, path))
}

/// HTTP 계약 식별자의 등록 여부를 확인하며 WebSocket 피드·RPC는 포함하지 않습니다.
pub fn implemented_id(id: &str) -> bool {
    http_contracts().any(|(api, _)| api == id)
}

/// 등록된 HTTP 계약이 OAuth·금융 변경을 제외한 조회인지 확인합니다.
pub fn read_only_pair(id: &str, path: &str) -> bool {
    !id.starts_with("au") && !financial_write_pair(id, path) && implemented_pair(id, path)
}

/// HTTP 계약의 금융 변경 분류를 확인하며 실행 허용 여부는 판단하지 않습니다.
pub fn financial_write_pair(id: &str, path: &str) -> bool {
    crate::orders::WRITE_CONTRACTS.contains(&(id, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn registry_and_inventory_have_exactly_same_current_contracts() {
        let inventory: serde_json::Value =
            serde_json::from_str(include_str!("../../../docs/api-inventory.json")).unwrap();
        let http: Vec<_> = http_contracts().collect();
        let http_ids: BTreeSet<_> = http.iter().map(|p| p.0).collect();
        assert_eq!(http.len(), http_ids.len());
        assert_eq!(http.len(), 320);
        let ws_ids: BTreeSet<_> = crate::ws::conditions::CONTRACT_IDS
            .iter()
            .chain(crate::ws::realtime::CONTRACT_IDS.iter())
            .copied()
            .collect();
        assert_eq!(ws_ids.len(), 31);
        assert!(http_ids.is_disjoint(&ws_ids));
        let known: BTreeSet<_> = http_ids.union(&ws_ids).copied().collect();
        let listed: BTreeSet<_> = inventory["entries"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["id"].as_str().unwrap())
            .collect();
        assert_eq!(known, listed);
        for e in inventory["entries"].as_array().unwrap() {
            assert_eq!(e["status"], "typed_mock_tested");
            assert_eq!(e["live_verified"], false);
            // 식별자 집합만 같아도 항목별 전송·안전 분류가 다르면 실패합니다.
            let id = e["id"].as_str().unwrap();
            let expected_safety = if http_ids.contains(id) {
                assert_eq!(e["transport"], "http");
                let path = http.iter().find(|p| p.0 == id).unwrap().1;
                if financial_write_pair(id, path) {
                    "mutation"
                } else if id.starts_with("au") {
                    "oauth"
                } else {
                    "read_only"
                }
            } else {
                assert!(ws_ids.contains(id));
                assert_eq!(e["transport"], "websocket");
                "read_only"
            };
            assert_eq!(e["safety"], expected_safety);
        }
        for (id, path) in crate::orders::WRITE_CONTRACTS {
            assert!(!read_only_pair(id, path));
        }
        for (id, path) in crate::orders::READ_CONTRACTS {
            assert!(read_only_pair(id, path));
        }
        assert!(!read_only_pair("au10001", "/oauth2/token"));
        assert!(!implemented_pair("kt10000", "/api/us/ordr"));
    }
}
