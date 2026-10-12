"""합성 저장소에서 목록 검증의 거부 경계와 CLI 보고서를 확인합니다."""
import contextlib
import copy
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock

SCRIPT = Path(__file__).resolve().parents[1] / "check_api_inventory.py"
SPEC = importlib.util.spec_from_file_location("check_api_inventory", SCRIPT)
checker = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checker)


class InventoryValidationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.source = "crates/kiwoom-client/src/auth.rs"
        self.tests = "crates/kiwoom-client/tests/contracts.rs"
        self.write(self.source, '''pub struct TokenRequest { pub grant_type: String, }
pub struct TokenResponse { pub token: String, }
pub struct RevokeRequest { pub token: String, }
pub struct RevokeResponse { pub return_code: i64, }
impl Client {
    pub async fn token(&self, r: &TokenRequest) -> Result<TokenResponse, ClientError> {
        self.send("au10001", "/oauth2/token", r).await
    }
    pub async fn revoke(&self, r: &RevokeRequest) -> Result<RevokeResponse, ClientError> {
        self.send("au10002", "/oauth2/revoke", r).await
    }
}
''')
        self.write(self.tests, '''#[tokio::test]
async fn token_contract() {
    let request = TokenRequest { grant_type: "synthetic".into() };
    client.token(&request).await.unwrap();
}
#[tokio::test]
async fn revoke_contract() { client.revoke(&request).await.unwrap(); }
''')
        self.write("docs/contracts/auth.md", "# OAuth\n\n| API | Contract |\n| --- | --- |\n| `au10001` | typed token |\n\n## Document inconsistency\n\nSynthetic note.\n")
        self.catalog = {
            "observed_at": "2026-10-12", "source": "https://openapi.kiwoom.com/m/guide/apiguide",
            "scope": "synthetic local contracts", "entries": [{
                "id": "au10001", "market": "oauth", "public_guide_group": "a1",
                "source": "https://openapi.kiwoom.com/m/guide/apiguide/a1/au10001",
                "status": "typed_mock_tested", "transport": "http", "safety": "oauth",
                "contract_evidence": [self.source + "#TokenRequest"],
                "test_evidence": [self.tests + "#token_contract"], "live_verified": False,
            }],
        }

    def test_domain_contract_and_client_call_can_live_in_separate_crates(self):
        path = "crates/kiwoom-domain/src/auth.rs"
        original = (self.root / self.source).read_text()
        dto, implementation = original.split("impl Client", 1)
        self.write(path, dto)
        self.write(self.source, "")
        self.write("crates/kiwoom-client/src/rest/oauth.rs", "impl Client" + implementation)
        self.catalog["entries"][0]["contract_evidence"] = [path + "#TokenRequest"]
        self.assertEqual(self.errors(), [])

    def test_domain_endpoint_dto_without_actual_call_cannot_claim_typed_support(self):
        path = "crates/kiwoom-domain/src/analytics.rs"
        self.write(path, 'endpoint!(ka10001, Ka10001Request, Ka10001Response, Ka10001Row, "ka10001", "/api/dostk/stkinfo", required { stk_cd: (6, &[], false) }, optional {}, scalars { stk_nm }, list { stk_cd });')
        self.write(self.tests, '#[tokio::test]\nasync fn domain_only() { client.ka10001(&request).await.unwrap(); }')
        row = self.catalog["entries"][0]
        row.update(id="ka10001", market="domestic", public_guide_group="01", transport="http", safety="read_only", source="https://openapi.kiwoom.com/m/guide/apiguide/01/ka10001", contract_evidence=[path + "#Ka10001Request"], test_evidence=[self.tests + "#domain_only"])
        self.assertTrue(any("lacks named typed method" in error for error in self.errors()))

    def test_unapproved_crate_is_not_an_evidence_root(self):
        path = "crates/unapproved/src/auth.rs"
        self.write(path, (self.root / self.source).read_text())
        self.catalog["entries"][0]["contract_evidence"] = [path + "#TokenRequest"]
        self.assertTrue(self.errors())

    def write(self, path, text):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")

    def errors(self, catalog=None):
        return checker.validate_catalog(self.catalog if catalog is None else catalog, self.root)["errors"]

    def test_valid_declarations_and_document_references(self):
        row = self.catalog["entries"][0]
        row["contract_evidence"].append("docs/contracts/auth.md#au10001")
        row["limitation_evidence"] = ["docs/contracts/auth.md", "docs/contracts/auth.md#document-inconsistency"]
        self.assertEqual(self.errors(), [])

    def test_audit_metadata_mutations_are_rejected(self):
        # 감사에서 통과했던 잘못된 메타데이터를 각각 독립적으로 거부합니다.
        cases = [("id", "invented_id"), ("market", "us"), ("public_guide_group", "evil"),
                 ("transport", "HTTP"), ("transport", "websocket"), ("safety", "mutation"),
                 ("status", "invented"), ("status", []), ("id", 1),
                 ("live_verified", []), ("live_verified", 0), ("live_verified", "false"),
                 ("live_verified", True), ("contract_evidence", {"README.md": ""}),
                 ("test_evidence", None), ("test_evidence", [7]), ("test_evidence", [""]),
                 ("contract_evidence", []), ("test_evidence", [])]
        for key, value in cases:
            with self.subTest(key=key, value=value):
                catalog = copy.deepcopy(self.catalog)
                catalog["entries"][0][key] = value
                self.assertTrue(self.errors(catalog))

    def test_missing_unknown_keys_and_wrong_shapes_are_reported(self):
        for key in checker.ROW_KEYS:
            with self.subTest(missing=key):
                catalog = copy.deepcopy(self.catalog)
                del catalog["entries"][0][key]
                self.assertTrue(self.errors(catalog))
        for catalog in ([], {}, {**self.catalog, "entries": None},
                        {**self.catalog, "entries": [None]}, {**self.catalog, "extra": 1},
                        {**self.catalog, "observed_at": "2026-99-99"}, {**self.catalog, "entries": []}):
            with self.subTest(catalog=catalog):
                self.assertTrue(self.errors(catalog))
        self.catalog["entries"][0]["extra"] = "unsupported"
        self.assertTrue(self.errors())

    def test_duplicate_ids_are_rejected(self):
        self.catalog["entries"].append(copy.deepcopy(self.catalog["entries"][0]))
        self.assertTrue(any("duplicate identifier" in error for error in self.errors()))

    def test_url_identity_and_escape_mutations_are_rejected(self):
        urls = ["https://openapi.kiwoom.com/m/guide/apiguide/evil/invented_id",
                "https://openapi.kiwoom.com/m/guide/apiguide/a2/au10001",
                "https://openapi.kiwoom.com/m/guide/apiguide/a1/au10002",
                "https://openapi.kiwoom.com/m/guide/apiguide/../../evil",
                "https://openapi.kiwoom.com/m/guide/apiguide/a1/%2e%2e/au10001",
                "https://openapi.kiwoom.com.evil/m/guide/apiguide/a1/au10001",
                "https://openapi.kiwoom.com@evil/m/guide/apiguide/a1/au10001",
                "https://openapi.kiwoom.com/m/guide/apiguide/a1/au10001?x=1",
                "https://openapi.kiwoom.com/m/guide/apiguide/a1/au10001#anchor"]
        for url in urls:
            with self.subTest(url=url):
                self.catalog["entries"][0]["source"] = url
                self.assertTrue(self.errors())

    def test_uncontained_and_unanchored_evidence_is_rejected(self):
        for evidence in ("/tmp/external.rs#TokenRequest", "../external.rs#TokenRequest",
                         "crates/kiwoom-client/src/../../external.rs#TokenRequest",
                         "local/a.rs#TokenRequest", ".aws/a.rs#TokenRequest", "target/a.rs#TokenRequest",
                         self.source, self.source + "#", self.source + "#quest",
                         self.source + "#RevokeRequest", self.source + "#TokenRequest#other",
                         "docs/contracts/auth.md#missing"):
            with self.subTest(evidence=evidence):
                self.catalog["entries"][0]["contract_evidence"] = [evidence]
                self.assertTrue(self.errors())

    def test_symlink_escape_is_rejected(self):
        with tempfile.TemporaryDirectory() as outside:
            external = Path(outside) / "external.rs"
            external.write_text("pub struct TokenRequest {}")
            (self.root / self.source).unlink()
            (self.root / self.source).symlink_to(external)
            self.assertTrue(any("escapes repository" in error for error in self.errors()))

    def test_symlink_to_local_record_is_rejected(self):
        self.write("local/record.rs", "pub struct TokenRequest {}")
        (self.root / self.source).unlink()
        (self.root / self.source).symlink_to(self.root / "local/record.rs")
        self.assertTrue(any("unsupported evidence location" in error for error in self.errors()))

    def test_comments_strings_and_reexports_do_not_declare_contracts(self):
        replacements = ["// pub struct TokenRequest { pub grant_type: String }\n",
                        "/* outer /* nested */ pub struct TokenRequest { pub grant_type: String } */\n",
                        'const FAKE: &str = r#"\npub struct TokenRequest { pub grant_type: String }\n"#;\n',
                        "pub use unrelated::TokenRequest;\n"]
        original = (self.root / self.source).read_text()
        for replacement in replacements:
            with self.subTest(replacement=replacement):
                self.write(self.source, original.replace("pub struct TokenRequest { pub grant_type: String, }", replacement))
                self.assertTrue(self.errors())

    def test_unrelated_test_and_substring_names_are_rejected(self):
        for symbol in ("token", "revoke_contract", "unrelated"):
            with self.subTest(symbol=symbol):
                self.catalog["entries"][0]["test_evidence"] = [self.tests + "#" + symbol]
                self.assertTrue(self.errors())
        self.write(self.tests, "// #[tokio::test]\n// async fn token_contract() { client.token(&request).await; }\n")
        self.catalog["entries"][0]["test_evidence"] = [self.tests + "#token_contract"]
        self.assertTrue(self.errors())
        self.write(self.tests, '#[tokio::test]\nasync fn token_contract() { let fake = "client.token(&request)"; }\n')
        self.assertTrue(self.errors())

    def test_generic_json_only_dto_is_rejected(self):
        original = (self.root / self.source).read_text()
        self.write(self.source, original.replace("pub struct TokenResponse { pub token: String, }",
                   "pub struct TokenResponse { pub extra: BTreeMap<String, serde_json::Value>, }"))
        self.assertTrue(any("concrete named" in error for error in self.errors()))

    def test_supported_endpoint_and_test_macro_linkage(self):
        self.write("crates/kiwoom-client/src/analytics.rs", '''endpoint!(
    ka10014, Ka10014Request, Ka10014Response, Ka10014Row,
    "ka10014", "/api/dostk/shsa",
    required { stk_cd: (20, &[], false) }, optional {}, scalars {}, rows { dt }
);
''')
        self.write(self.tests, '''contract!(
    ka10014_contract, Ka10014Request, Ka10014Response, ka10014,
    "/api/dostk/shsa", json!({}), json!({}), rows, dt, "dt", price, "price"
);
''')
        row = self.catalog["entries"][0]
        row.update(id="ka10014", market="domestic", public_guide_group="17", safety="read_only",
                   source="https://openapi.kiwoom.com/m/guide/apiguide/17/ka10014",
                   contract_evidence=["crates/kiwoom-client/src/analytics.rs#Ka10014Request"],
                   test_evidence=[self.tests + "#ka10014_contract"])
        self.assertEqual(self.errors(), [])
        valid_macro = (self.root / self.tests).read_text()
        # 잘못된 메서드 인자를 fixture 안의 이름으로 보완할 수 없습니다.
        for fixture in ('json!({"note":"ka10014"})', 'json!({"nested":[{"note":"ka10014"}]})',
                        'r#"ka10014"#'):
            with self.subTest(fixture=fixture):
                bad = valid_macro.replace("Ka10014Response, ka10014,", "Ka10014Response, unrelated_method,")
                self.write(self.tests, bad.replace("json!({})", fixture, 1))
                self.assertTrue(self.errors())
        self.write(self.tests, valid_macro.replace("Ka10014Response, ka10014,", "OtherResponse, ka10014,"))
        self.assertTrue(self.errors())
        # 상품 형식도 세 번째 메서드 위치를 검사합니다.
        self.write(self.tests, 'contract!(ka10014_contract, Ka10014Request, ka10014, "/api/dostk/shsa", json!({}), json!({}), json!({}), typed);')
        self.assertEqual(self.errors(), [])
        self.write(self.tests, 'contract!(ka10014_contract, Ka10014Request, unrelated_method, "/api/dostk/shsa", json!({"note":"ka10014"}), json!({}), json!({}), typed);')
        self.assertTrue(self.errors())
        self.write(self.tests, valid_macro)
        row["test_evidence"] = [self.tests + "#ka10014"]
        self.assertTrue(self.errors())

    def test_case_sensitive_feed_ids_and_named_rpc_gate(self):
        row = self.catalog["entries"][0]
        for api, market, group in (("0g", "domestic", "14"), ("FT", "us", "40")):
            row.update(id=api, market=market, public_guide_group=group, transport="websocket", safety="read_only",
                       status="planned", source=f"https://openapi.kiwoom.com/m/guide/apiguide/{group}/{api}",
                       contract_evidence=[], test_evidence=[])
            self.assertEqual(self.errors(), [])
            row["market"] = "us" if market == "domestic" else "domestic"
            self.assertTrue(self.errors())
        self.write("crates/kiwoom-client/src/ws/conditions/rpc.rs", '''pub struct Ka10171Request {}
pub struct Ka10171Response { pub return_code: i64, }
session_contract!(Ka10171Request, Ka10171Response, "ka10171", &["CNSRLST"]);
''')
        row.update(id="ka10171", market="domestic", public_guide_group="15", status="contract_verified",
                   source="https://openapi.kiwoom.com/m/guide/apiguide/15/ka10171",
                   contract_evidence=["crates/kiwoom-client/src/ws/conditions/rpc.rs#Ka10171Request"])
        self.assertTrue(any("lacks named typed method" in error for error in self.errors()))
        path = self.root / "crates/kiwoom-client/src/ws/conditions/rpc.rs"
        base = path.read_text()
        path.write_text(base + """impl<T: WsTransport> WsSession<T> {
    pub async fn ka10171(&mut self, request: &Ka10171Request) -> Result<WsReply<Ka10171Response>, ClientError> {
        self.call(request).await
    }
}
""")
        self.assertEqual(self.errors(), [])
        path.write_text(path.read_text().replace("WsReply<Ka10171Response>", "WsReply<OtherResponse>"))
        self.assertTrue(any("lacks named typed method" in error for error in self.errors()))
        path.write_text(path.read_text().replace("WsReply<OtherResponse>", "WsReply<Ka10171Response>"))
        row["status"] = "typed_mock_tested"
        row["test_evidence"] = [self.tests + "#mock_ka10171_contract"]
        valid = 'contract!(mock_ka10171_contract, Ka10171Request, decode_ka10171, json!({}), json!({}), true);'
        self.write(self.tests, valid)
        self.assertEqual(self.errors(), [])
        for fixture in ('json!({"note":"decode_ka10171"})', 'r#"decode_ka10171"#'):
            with self.subTest(fixture=fixture):
                bad = valid.replace("Ka10171Request, decode_ka10171,", "Ka10171Request, unrelated_decode,")
                self.write(self.tests, bad.replace("json!({})", fixture, 1))
                self.assertTrue(self.errors())

    def test_cli_passed_gate_and_mutation_rejection(self):
        catalog_path = self.root / "catalog.json"
        validate = checker.validate_catalog
        for valid in (True, False):
            if not valid:
                self.catalog["entries"][0]["transport"] = "websocket"
            catalog_path.write_text(json.dumps(self.catalog))
            with mock.patch.object(checker, "validate_catalog", side_effect=lambda value: validate(value, self.root)):
                with contextlib.redirect_stdout(io.StringIO()) as output:
                    code = checker.main(["--catalog", str(catalog_path), "--fail-on-unimplemented"])
            self.assertEqual(code, 0 if valid else 1)
            self.assertEqual(bool(json.loads(output.getvalue())["errors"]), not valid)

    def test_document_anchor_in_comment_or_code_is_rejected(self):
        for content in ("<!--\n## Hidden heading\n-->", "```markdown\n## Hidden heading\n```\n"):
            self.write("docs/contracts/hidden.md", content)
            self.catalog["entries"][0]["limitation_evidence"] = ["docs/contracts/hidden.md#hidden-heading"]
            self.assertTrue(self.errors())

    def test_public_html_drift_and_unimplemented_cli_gate(self):
        catalog_path = self.root / "catalog.json"
        html = self.root / "guide.html"
        validate = checker.validate_catalog
        for status, menu_id, expected in (("typed_mock_tested", "au10001", 0),
                                          ("typed_mock_tested", "au10002", 1),
                                          ("planned", "au10001", 1)):
            self.catalog["entries"][0]["status"] = status
            catalog_path.write_text(json.dumps(self.catalog))
            html.write_text(f"goToGuide('a1', '{menu_id}')")
            with mock.patch.object(checker, "validate_catalog", side_effect=lambda value: validate(value, self.root)):
                with contextlib.redirect_stdout(io.StringIO()):
                    code = checker.main(["--catalog", str(catalog_path), "--public-html", str(html), "--fail-on-unimplemented"])
            self.assertEqual(code, expected)

    def test_cli_rejects_duplicate_json_keys_and_malformed_catalog(self):
        for contents in ('{"entries": [], "entries": []}', "{", '{"entries": 0}'):
            catalog_path = self.root / "catalog.json"
            catalog_path.write_text(contents)
            with contextlib.redirect_stdout(io.StringIO()) as output:
                exit_code = checker.main(["--catalog", str(catalog_path), "--fail-on-unimplemented"])
            report = json.loads(output.getvalue())
            self.assertEqual(exit_code, 1)
            self.assertTrue(report["errors"])


if __name__ == "__main__":
    unittest.main()
