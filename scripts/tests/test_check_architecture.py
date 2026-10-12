"""합성 크레이트에서 레이어 경계의 위반과 정상 계약을 검증합니다."""
import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "check_architecture.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("check_architecture", SCRIPT)
checker = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checker)


class ArchitectureTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.manifests = {}
        for name, edges in checker.WORKSPACE_EDGES.items():
            dependencies = checker.DEPENDENCIES.get(name, edges)
            content = f'[package]\nname = "{name}"\n[dependencies]\n'
            for dep in sorted(dependencies):
                content += f'{dep} = {{ path = "../{dep}" }}\n' if dep in checker.WORKSPACE_EDGES else f'{dep}.workspace = true\n'
            self.manifests[name] = content
            self.write(f"{checker.PACKAGE_PATHS[name]}/Cargo.toml", content)
            self.write(f"{checker.PACKAGE_PATHS[name]}/src/lib.rs", "pub struct Value;\n")

    def write(self, path, content):
        p = self.root / path
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(content)

    def errors(self):
        return checker.validate_architecture(self.root)["errors"]

    def test_valid_dependency_direction(self):
        self.assertEqual(self.errors(), [])

    def test_sdk_cannot_depend_on_application(self):
        self.write("crates/kiwoom-client/Cargo.toml", self.manifests["kiwoom-client"] + 'kiwoom-mcp = { path = "../../apps/kiwoom-mcp" }\n')
        self.assertTrue(self.errors())

    def test_sdk_cannot_acquire_mcp_framework(self):
        self.write("crates/kiwoom-client/Cargo.toml", self.manifests["kiwoom-client"] + 'rmcp = "3.5"\n')
        self.assertTrue(self.errors())

    def test_app_only_consumes_public_sdk(self):
        self.write("apps/kiwoom-mcp/src/lib.rs", "use kiwoom_client::{rest::Client, domain::market};")
        self.assertEqual(self.errors(), [])
        self.write("apps/kiwoom-mcp/src/lib.rs", "use kiwoom_client::{runtime::http::Client};")
        self.assertTrue(self.errors())

    def test_resolved_dev_dependency_cannot_reverse_app_direction(self):
        packages = [
            {"name": name, "dependencies": [{"name": dep} for dep in checker.DEPENDENCIES.get(name, edges)]}
            for name, edges in checker.WORKSPACE_EDGES.items()
        ]
        client = next(p for p in packages if p["name"] == "kiwoom-client")
        client["dependencies"].append({"name": "kiwoom-mcp", "kind": "dev"})
        self.assertTrue(checker.validate_architecture(self.root, {"packages": packages})["errors"])
        client["dependencies"][-1] = {"name": "rmcp", "kind": None, "target": "cfg(unix)"}
        self.assertTrue(checker.validate_architecture(self.root, {"packages": packages})["errors"])

    def test_domain_cannot_depend_on_client(self):
        self.write("crates/kiwoom-domain/Cargo.toml", self.manifests["kiwoom-domain"] + 'kiwoom-client = { path = "../kiwoom-client" }\n')
        self.assertTrue(self.errors())

    def test_renamed_network_dependency_cannot_hide_in_core(self):
        self.write("crates/kiwoom-core/Cargo.toml", self.manifests["kiwoom-core"].replace('serde.workspace = true', 'serde = { package = "reqwest", version = "0.12" }'))
        self.assertTrue(self.errors())

    def test_target_dependencies_cannot_bypass_domain_boundary(self):
        self.write("crates/kiwoom-domain/Cargo.toml", self.manifests["kiwoom-domain"] + '\n[target.\'cfg(unix)\'.dependencies]\ntokio = "1"\n')
        self.assertTrue(self.errors())

    def test_domain_cannot_add_async_io(self):
        self.write("crates/kiwoom-domain/src/lib.rs", 'async fn connect() { tokio::net::TcpStream::connect("localhost").await; }')
        self.assertTrue(self.errors())

    def test_grouped_standard_library_io_import_is_rejected(self):
        self.write("crates/kiwoom-domain/src/lib.rs", "use std::{collections::BTreeMap, net::TcpStream};")
        self.assertTrue(self.errors())

    def test_runtime_cannot_import_outer_facade(self):
        self.write("crates/kiwoom-client/src/runtime/ws.rs", 'use crate::ws::Client;')
        self.assertTrue(self.errors())

    def test_runtime_parent_cannot_import_outer_facade(self):
        # 부모 모듈의 호출 인터페이스 역의존도 하위 모듈과 같이 거절합니다.
        self.write("crates/kiwoom-client/src/runtime.rs", 'use crate::rest::Client;')
        self.assertTrue(self.errors())

    def test_grouped_outer_imports_are_rejected(self):
        for source in (
            "use crate::{ws::Client};",
            "use crate::{runtime::http::Client, auth::Token};",
            "use {crate::{rest::Client as SdkClient}};",
        ):
            with self.subTest(source=source):
                self.write("crates/kiwoom-client/src/runtime/ws.rs", source)
                self.assertTrue(self.errors())

    def test_grouped_contract_runtime_import_is_rejected(self):
        self.write("crates/kiwoom-domain/src/lib.rs", "use crate::{runtime::Client};")
        self.assertTrue(self.errors())

    def test_grouped_internal_runtime_imports_remain_allowed(self):
        self.write("crates/kiwoom-client/src/runtime/http.rs",
                   "use crate::{runtime::{ws::Session, auth::Token}, core::Secret};")
        self.assertEqual(self.errors(), [])

    def test_grouped_import_comments_and_strings_are_ignored(self):
        self.write("crates/kiwoom-client/src/runtime/ws.rs", '''// use crate::{rest::Client};
pub const MESSAGE: &str = "use crate::{ws::Client};";
''')
        self.assertEqual(self.errors(), [])

    def test_call_owner_cannot_define_request_dto(self):
        self.write("crates/kiwoom-client/src/rest/accounts.rs", 'pub struct AccountRequest { pub id: String }')
        self.assertTrue(self.errors())

    def test_rest_parent_cannot_define_request_dto(self):
        # REST 진입 파일에 계약 DTO를 선언해도 경계 위반을 검출합니다.
        self.write("crates/kiwoom-client/src/rest.rs", 'pub struct AccountRequest { pub id: String }')
        self.assertTrue(self.errors())

    def test_ws_parent_cannot_define_response_dto(self):
        # WS 진입 파일에 계약 DTO를 선언해도 경계 위반을 검출합니다.
        self.write("crates/kiwoom-client/src/ws.rs", 'pub struct FeedResponse { pub id: String }')
        self.assertTrue(self.errors())

    def test_comments_and_wire_strings_are_not_dependency_evidence(self):
        self.write("crates/kiwoom-domain/src/lib.rs", '''// async fn example() { tokio::net::TcpStream }
pub const WIRE: &str = "tokio::net::TcpStream and async fn";
''')
        self.assertEqual(self.errors(), [])


if __name__ == "__main__":
    unittest.main()
