#!/usr/bin/env python3
"""SDK의 단방향 크레이트 의존과 계약·실행·호출 경계를 검사합니다."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tomllib

from check_api_inventory import _strip_comments

ROOT = Path(__file__).resolve().parents[1]
DEPENDENCIES = {
    "kiwoom-core": {"serde", "serde_json", "thiserror"},
    "kiwoom-domain": {"kiwoom-core", "serde", "serde_json", "chrono", "rust_decimal"},
}
WORKSPACE_EDGES = {
    "kiwoom-core": set(),
    "kiwoom-domain": {"kiwoom-core"},
    "kiwoom-client": {"kiwoom-core", "kiwoom-domain"},
    "kiwoom-mcp": {"kiwoom-client"},
}
PACKAGE_PATHS = {name: f"crates/{name}" for name in WORKSPACE_EDGES if name != "kiwoom-mcp"}
PACKAGE_PATHS["kiwoom-mcp"] = "apps/kiwoom-mcp"

STRINGS = re.compile(r'r(?P<h>\#*)"[\s\S]*?"(?P=h)|"(?:\\.|[^"\\])*"')


def _import_paths(code):
    """중첩 use 트리에서 경로를 펼칩니다. alias의 사용처나 매크로는 해석하지 않습니다."""
    paths = set()
    for body in re.findall(r"\buse\s+([^;]+);", code):
        parents, path, alias = [[]], [], False
        for token in re.findall(r"[A-Za-z_]\w*|::|[{},*]", body):
            if alias:
                alias = False
            elif token == "as":
                alias = True
            elif token == "{":
                parents.append(path.copy())
            elif token in (",", "}"):
                if path:
                    paths.add(tuple(path))
                if token == "}" and len(parents) > 1:
                    parents.pop()
                path = parents[-1].copy()
            elif token != "::":
                path.append(token)
        if path:
            paths.add(tuple(path))
    return paths


def validate_architecture(root=ROOT, metadata=None):
    errors, files = [], []
    for name in WORKSPACE_EDGES:
        path = root / PACKAGE_PATHS[name] / "Cargo.toml"
        try:
            manifest = tomllib.loads(path.read_text())
            if manifest.get("package", {}).get("name") != name:
                errors.append(f"{name}: package name does not match its layer")
            dependencies = manifest.get("dependencies", {})
            workspace_edges = set(dependencies) & WORKSPACE_EDGES.keys()
            if workspace_edges != WORKSPACE_EDGES[name]:
                errors.append(f"{name}: invalid workspace dependency direction")
            if name != "kiwoom-mcp" and set(dependencies) & {"rmcp", "rmcp-macros", "axum", "clap"}:
                errors.append(f"{name}: SDK cannot depend on the MCP application framework")
            if name in DEPENDENCIES and set(dependencies) != DEPENDENCIES[name]:
                errors.append(f"{name}: contracts cannot acquire execution dependencies")
            if name in DEPENDENCIES:
                for section in ("dev-dependencies", "build-dependencies", "target", "features"):
                    if manifest.get(section):
                        errors.append(f"{name}: unexpected {section} can bypass the layer boundary")
            for dependency, settings in dependencies.items():
                if isinstance(settings, dict) and settings.get("package", dependency) != dependency:
                    errors.append(f"{name}: renamed dependency hides its actual layer")
        except (OSError, ValueError) as error:
            errors.append(f"{name}: {error}")
    if metadata is not None:
        packages = {p["name"]: p for p in metadata.get("packages", [])}
        if set(packages) != set(WORKSPACE_EDGES):
            errors.append("cargo metadata: unexpected workspace packages")
        for name, allowed in WORKSPACE_EDGES.items():
            dependencies = {d["name"] for d in packages.get(name, {}).get("dependencies", [])}
            edges = dependencies & WORKSPACE_EDGES.keys()
            if name != "kiwoom-mcp" and dependencies & {"rmcp", "rmcp-macros", "axum", "clap"}:
                errors.append(f"cargo metadata {name}: SDK cannot acquire the MCP framework")
            if edges != allowed:
                errors.append(f"cargo metadata {name}: resolved workspace dependencies violate direction")
        for name, allowed in DEPENDENCIES.items():
            dependencies = {d["name"] for d in packages.get(name, {}).get("dependencies", [])}
            if dependencies != allowed:
                errors.append(f"cargo metadata {name}: resolved dependencies violate the layer boundary")
    for name in WORKSPACE_EDGES:
        base = root / PACKAGE_PATHS[name] / "src"
        for path in sorted(base.rglob("*.rs")):
            relative = path.relative_to(root).as_posix()
            files.append(relative)
            code = STRINGS.sub(lambda m: " " * len(m[0]), _strip_comments(path.read_text()))
            imports = _import_paths(code)
            if name == "kiwoom-mcp":
                if any(path[:2] == ("kiwoom_client", "runtime") for path in imports):
                    errors.append(f"{relative}: application imports private SDK runtime")
            elif name in DEPENDENCIES:
                forbidden = [
                    r"\b(?:reqwest|tokio|tokio_tungstenite|futures_util|kiwoom_client)\s*::",
                    r"\b(?:async\s+fn|await)\b",
                    r"\b(?:Client|WsSession|TokenManager|ManagedTransport)\s*(?:<|\{)",
                    r"\bstd\s*::\s*(?:net|fs|process|thread)\b",
                    r"\buse\s+std\s*::\s*\{[^;]*\b(?:net|fs|process|thread)\b",
                    r"\bcrate\s*::\s*(?:rest|runtime|transport)\b",
                ]
                forbidden_import = any(
                    path[:2] in {("crate", "rest"), ("crate", "runtime"),
                                 ("crate", "transport"), ("std", "net"),
                                 ("std", "fs"), ("std", "process"), ("std", "thread")}
                    for path in imports
                )
                if forbidden_import or any(re.search(pattern, code) for pattern in forbidden):
                    errors.append(f"{relative}: pure contracts refer to execution or I/O")
            elif "/src/runtime/" in relative or relative.endswith("/src/runtime.rs"):
                outer_import = any(
                    path[:2] in {("crate", "rest"), ("crate", "ws"), ("crate", "auth")}
                    for path in imports
                )
                if outer_import or re.search(r"\bcrate\s*::\s*(?:rest|ws|auth)\b", code):
                    errors.append(f"{relative}: runtime depends on an outer call interface")
            elif "/src/rest/" in relative or relative.endswith(("/src/rest.rs", "/src/ws.rs", "/src/ws/calls.rs")):
                if re.search(r"\bpub\s+struct\s+\w*(?:Request|Response|Row)\b", code):
                    errors.append(f"{relative}: call interface defines a domain wire DTO")
    return {"workspace_edges": {k: sorted(v) for k, v in WORKSPACE_EDGES.items()},
            "checked_source_files": len(files), "errors": errors}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    try:
        process = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked", "--offline"],
            cwd=args.root, text=True, capture_output=True, check=True,
        )
        result = validate_architecture(args.root, json.loads(process.stdout))
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        result = {"errors": [f"architecture metadata: {error}"]}
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return int(bool(result["errors"]))


if __name__ == "__main__":
    raise SystemExit(main())
