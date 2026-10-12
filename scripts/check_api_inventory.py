#!/usr/bin/env python3
"""독립 API 목록의 메타데이터와 로컬 근거 연결을 오프라인으로 검사합니다."""
import argparse
import collections
from dataclasses import dataclass, field
from datetime import date
import json
from pathlib import Path, PurePosixPath
import re
import sys
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOTS = tuple(f"crates/{name}/src" for name in ("kiwoom-core", "kiwoom-domain", "kiwoom-client"))


def _production_source(path):
    return any(path.startswith(root + "/") and path.endswith(".rs") for root in SOURCE_ROOTS)


ROOT_KEYS = {"observed_at", "source", "scope", "entries"}
ROW_KEYS = {"id", "market", "public_guide_group", "source", "status", "transport", "safety",
            "contract_evidence", "test_evidence", "live_verified"}
STATUSES = {"planned", "contract_verified", "typed_mock_tested", "partial_mock_tested"}
DOMESTIC_FEEDS = set("00 04 0A 0B 0C 0D 0E 0F 0G 0H 0I 0J 0U 0g 0m 0s 0u 0w 1h".split())
US_FEEDS = {"F4", "F5", "FE", "FT"}
RPC_IDS = {"ka10171", "ka10172", "ka10173", "ka10174", "usa20280", "usa20281", "usa20290", "usa20291"}
IDENT = r"[A-Za-z_][A-Za-z_0-9]*"


@dataclass
class Binding:
    transport: str
    safety: str
    request: str
    response: str
    method: str
    path: str = ""
    named: bool = False


@dataclass
class Source:
    text: str
    structs: dict[str, str] = field(default_factory=dict)
    functions: dict[str, str] = field(default_factory=dict)
    macros: list[tuple[str, str]] = field(default_factory=list)


def validate_catalog(catalog, root=ROOT):
    """검사 보고서를 반환합니다. 근거 검사는 선언 연결이며 실행·필드 의미의 증명이 아닙니다."""
    errors = []
    rows = []
    if not isinstance(catalog, dict):
        errors.append("catalog: expected object")
    else:
        _keys(catalog, ROOT_KEYS, set(), "catalog", errors)
        for key in ("observed_at", "source", "scope"):
            if not _nonempty(catalog.get(key)):
                errors.append(f"catalog.{key}: expected nonempty string")
        if _nonempty(catalog.get("observed_at")):
            try:
                if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", catalog["observed_at"]):
                    raise ValueError("invalid date format")
                date.fromisoformat(catalog["observed_at"])
            except ValueError:
                errors.append("catalog.observed_at: expected valid YYYY-MM-DD date")
        if _nonempty(catalog.get("source")) and catalog["source"] not in {
            "https://openapi.kiwoom.com/m/guide/apiguide",
            "https://openapi.kiwoom.com/m/guide/apiguide?dummyVal=0",
        }:
            errors.append("catalog.source: expected primary guide menu URL")
        if not isinstance(catalog.get("entries"), list):
            errors.append("catalog.entries: expected array")
        else:
            rows = catalog["entries"]
            if not rows:
                errors.append("catalog.entries: expected nonempty array")
    root = Path(root).resolve()
    sources, bindings, groups = _source_index(root, errors)
    seen = set()
    valid_rows = []
    for number, row in enumerate(rows):
        label = f"entries[{number}]"
        if not isinstance(row, dict):
            errors.append(f"{label}: expected object")
            continue
        if _nonempty(row.get("id")):
            label += f" ({row['id']})"
        _keys(row, ROW_KEYS, {"limitation_evidence"}, label, errors)
        shape_ok = True
        for key in ROW_KEYS - {"contract_evidence", "test_evidence", "live_verified"}:
            if not _nonempty(row.get(key)):
                errors.append(f"{label}.{key}: expected nonempty string")
                shape_ok = False
        for key in ("contract_evidence", "test_evidence", "limitation_evidence"):
            if key == "limitation_evidence" and key not in row:
                continue
            value = row.get(key)
            if not isinstance(value, list) or any(not _nonempty(item) for item in value):
                errors.append(f"{label}.{key}: expected array of nonempty strings")
                shape_ok = False
        if row.get("live_verified") is not False:
            errors.append(f"{label}.live_verified: must be boolean false")
        if not shape_ok:
            continue
        valid_rows.append(row)
        api = row["id"]
        if api in seen:
            errors.append(f"{label}: duplicate identifier")
        seen.add(api)
        for key, allowed in (("status", STATUSES), ("market", {"oauth", "domestic", "us"}),
                             ("transport", {"http", "websocket"}), ("safety", {"oauth", "read_only", "mutation"})):
            if row[key] not in allowed:
                errors.append(f"{label}.{key}: invalid value {row[key]!r}")
        expected_market = _market(api)
        if expected_market is None or row["market"] != expected_market:
            errors.append(f"{label}: ID namespace and market mismatch")
        expected_transport = "websocket" if api in DOMESTIC_FEEDS | US_FEEDS | RPC_IDS else "http"
        if row["transport"] != expected_transport:
            errors.append(f"{label}: ID and transport mismatch")
        allowed_groups = {"oauth": {"a1", "a2"}, "domestic": {f"{n:02}" for n in range(1, 19)},
                          "us": {str(n) for n in range(31, 43)}}
        group = row["public_guide_group"]
        special_group = {"au10001": "a1", "au10002": "a2"}.get(api)
        if api in DOMESTIC_FEEDS:
            special_group = "14"
        elif api in US_FEEDS:
            special_group = "40"
        elif api in RPC_IDS:
            special_group = "15" if expected_market == "domestic" else "39"
        if group not in allowed_groups.get(expected_market, set()) or (special_group and group != special_group):
            errors.append(f"{label}: ID and guide group mismatch")
        if api in groups and groups[api] != {group}:
            errors.append(f"{label}: guide group differs from maintained primary-source links")
        if not _source_url(row["source"], group, api):
            errors.append(f"{label}.source: expected exact primary guide URL for group/ID")
        binding = bindings.get(api)
        if binding:
            if row["transport"] != binding.transport or row["safety"] != binding.safety:
                errors.append(f"{label}: classification differs from concrete API binding")
        elif row["status"] != "planned":
            errors.append(f"{label}: no supported concrete API binding")
        if row["status"] != "planned":
            if binding and (not _concrete(sources, binding.request, request=True)
                            or not _concrete(sources, binding.response, request=False)):
                errors.append(f"{label}: binding lacks concrete named request/response or payload declarations")
            if binding and binding.transport == "http" and not binding.named:
                errors.append(f"{label}: HTTP binding lacks named typed method")
            if api in RPC_IDS and binding and not binding.named:
                errors.append(f"{label}: RPC binding lacks named typed method")
        for kind in ("contract_evidence", "test_evidence", "limitation_evidence"):
            if kind not in row:
                continue
            required = row["status"] != "planned" and (kind == "contract_evidence" or
                        kind == "test_evidence" and row["status"] != "contract_verified")
            if required and not row[kind]:
                errors.append(f"{label}.{kind}: missing evidence")
            code_found = False
            for evidence in row[kind]:
                valid, code = _evidence(root, sources, row, binding, kind, evidence, errors, label)
                code_found |= valid and code
            if required and not code_found:
                errors.append(f"{label}.{kind}: needs concrete code evidence linked to API")
    return {"total": len(rows),
            "markets": dict(collections.Counter(row["market"] for row in valid_rows)),
            "status": dict(collections.Counter(row["status"] for row in valid_rows)),
            "websocket_catalog_entries": sum(row["transport"] == "websocket" for row in valid_rows),
            "errors": errors}


def _keys(value, required, optional, label, errors):
    missing, unknown = required - value.keys(), value.keys() - required - optional
    if missing:
        errors.append(f"{label}: missing keys {sorted(map(str, missing))}")
    if unknown:
        errors.append(f"{label}: unknown keys {sorted(map(str, unknown))}")


def _nonempty(value):
    return isinstance(value, str) and bool(value.strip())


def _market(api):
    if api in {"au10001", "au10002"}:
        return "oauth"
    if api in DOMESTIC_FEEDS or re.fullmatch(r"(?:ka|kt)\d{5}", api):
        return "domestic"
    if api in US_FEEDS or re.fullmatch(r"(?:usa|ust)\d{5}", api):
        return "us"
    return None


def _source_url(value, group, api):
    try:
        url = urlsplit(value)
        return (url.scheme == "https" and url.netloc == "openapi.kiwoom.com" and
                url.path == f"/m/guide/apiguide/{group}/{api}" and not url.query and not url.fragment and
                value == f"https://openapi.kiwoom.com/m/guide/apiguide/{group}/{api}")
    except ValueError:
        return False


def _safe_file(root, path):
    parts = PurePosixPath(path).parts
    if (not parts or path.startswith("/") or "\\" in path or any(p in {".", ".."} or p.startswith(".") for p in parts)
            or path != str(PurePosixPath(path))):
        raise ValueError("expected canonical relative path")
    if not (path.startswith("docs/") and path.endswith(".md") or
            _production_source(path) or
            path.startswith("crates/kiwoom-client/tests/") and path.endswith(".rs")):
        raise ValueError("unsupported evidence location")
    target = (root / path).resolve()
    if not target.is_relative_to(root):
        raise ValueError("resolved path escapes repository")
    resolved_path = target.relative_to(root).as_posix()
    if not ((resolved_path.startswith("docs/") and resolved_path.endswith(".md")) or
            (_production_source(resolved_path)) or
            (resolved_path.startswith("crates/kiwoom-client/tests/") and resolved_path.endswith(".rs"))) or any(part.startswith(".") for part in target.relative_to(root).parts):
        raise ValueError("resolved path has unsupported evidence location")
    if not target.is_file():
        raise ValueError("missing evidence file")
    return target


# 문자열을 보존하고 중첩 블록 주석을 제거해 주석 안의 선언을 근거로 인정하지 않습니다.
_TOKEN = re.compile(r'r(?P<hash>\#*)"[\s\S]*?"(?P=hash)|"(?:\\.|[^"\\])*"|/\*|//[^\n]*')


def _strip_comments(text):
    output, pos = [], 0
    while match := _TOKEN.search(text, pos):
        output.append(text[pos:match.start()])
        end = match.end()
        if match.group() == "/*":
            depth = 1
            while depth and end < len(text):
                nested = re.search(r"/\*|\*/", text[end:])
                if not nested:
                    end = len(text)
                    break
                end += nested.end()
                depth += 1 if nested.group() == "/*" else -1
            output.append(re.sub(r"[^\n]", " ", text[match.start():end]))
        elif match.group().startswith("//"):
            output.append(" " * len(match.group()))
        else:
            output.append(match.group())
        pos = end
    output.append(text[pos:])
    return "".join(output)


def _closing(text, start):
    stack = []
    strings = re.compile(r'r(?P<h>\#*)"[\s\S]*?"(?P=h)|"(?:\\.|[^"\\])*"')
    index = start
    while index < len(text):
        string = strings.match(text, index)
        if string:
            index = string.end()
            continue
        char = text[index]
        if char in "({[":
            stack.append(char)
        elif char in ")}]":
            if not stack or "({["[")}]".index(char)] != stack.pop():
                return None
            if not stack:
                return index
        index += 1
    return None


def _rust_source(text):
    text = _strip_comments(text)
    source = Source(text)
    declarations = re.sub(r'r(?P<h>\#*)"[\s\S]*?"(?P=h)|"(?:\\.|[^"\\])*"', lambda m: re.sub(r"[^\n]", " ", m.group()), text)
    for match in re.finditer(r"(?m)^\s*(?:pub(?:\([^\n]*?\))?\s+)?struct\s+(" + IDENT + r")\s*\{", declarations):
        end = _closing(text, match.end() - 1)
        if end is not None:
            source.structs[match[1]] = text[match.end():end]
    for match in re.finditer(r"(?m)^\s*(?:pub(?:\([^\n]*?\))?\s+)?(?:async\s+)?fn\s+(" + IDENT + r")\s*\(", declarations):
        end_args = _closing(text, match.end() - 1)
        if end_args is None:
            continue
        start_body = text.find("{", end_args)
        end = _closing(text, start_body) if start_body >= 0 else None
        if end is not None and ";" not in text[end_args:start_body]:
            source.functions[match[1]] = text[match.start():end + 1]
    for match in re.finditer(r"(?m)^\s*(" + IDENT + r")!\s*[({]", declarations):
        end = _closing(text, match.end() - 1)
        if end is not None:
            source.macros.append((match[1], text[match.end():end]))
    return source


def _source_index(root, errors):
    sources, bindings, groups = {}, {}, collections.defaultdict(set)
    for base in (*SOURCE_ROOTS, "crates/kiwoom-client/tests", "docs/contracts"):
        directory = root / base
        for path in sorted(directory.rglob("*.rs" if base.startswith("crates/") else "*.md")):
            relative = path.relative_to(root).as_posix()
            try:
                raw = _safe_file(root, relative).read_text(encoding="utf-8")
            except (OSError, ValueError, UnicodeError) as error:
                errors.append(f"source {relative}: {error}")
                continue
            for group, api in re.findall(r"https://openapi\.kiwoom\.com/(?:guide/apiGuideContents|m/guide/apiguide)/([a-z0-9]+)/([A-Za-z0-9]+)", raw):
                groups[api].add(group)
            if not relative.endswith(".rs"):
                continue
            source = _rust_source(raw)
            sources[relative] = source
            if "/tests/" in relative:
                continue
            for name, body in source.macros:
                if name == "endpoint":
                    match = re.match(r'\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*\w+\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*required\s*\{', body)
                    if match and match[1] == match[4]:
                        bindings[match[4]] = Binding("http", "read_only", match[2], match[3], match[1], match[5], not relative.startswith("crates/kiwoom-domain/"))
                elif name in {"mutation", "read"}:
                    match = re.fullmatch(r'\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)(?:\s*,\s*"([^"]+)")?\s*', body)
                    if match:
                        bindings[match[1]] = Binding("http", "mutation" if name == "mutation" else "read_only", match[2], match[3], match[1], match[4] or "/api/us/acnt", True)
                elif name == "endpoints":
                    for api, method, req, res, route in re.findall(r'\(\s*"([^"]+)"\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*"([^"]+)"\s*\)', body):
                        if api == method:
                            bindings[api] = Binding("http", "read_only", req, res, method, route, True)
                elif name == "session_contract":
                    match = re.match(r'\s*(\w+)\s*,\s*(\w+)\s*,\s*"([^"]+)"\s*,\s*&\[', body)
                    if match:
                        bindings[match[3]] = Binding("websocket", "read_only", match[1], match[2], match[3])
                elif name == "payloads":
                    for _, api, payload in re.findall(r'\(\s*(\w+)\s*,\s*"([^"]+)"\s*,\s*(\w+)\s*\)', body):
                        bindings[api] = Binding("websocket", "read_only", payload, payload, "")
    for path, source in sources.items():
        if "/tests/" in path:
            continue
        for name, body in source.functions.items():
            req = re.search(r"(?:request|r):\s*&(" + IDENT + r")", body)
            signature = body.split("{", 1)[0]
            res = re.search(r"Result<(?:Vec<Page<|Page<|WsReply<)?(" + IDENT + r")", signature)
            if req and res:
                if (name in bindings and bindings[name].request == req[1] and bindings[name].response == res[1]
                        and re.search(r"pub\s+async\s+fn", signature)
                        and re.search(r"self\s*\.\s*call\s*\(\s*(?:request|r)\s*\)", body)):
                    bindings[name].named = True
                api = {"token": "au10001", "revoke": "au10002"}.get(name, name)
                if _market(api) and re.search(r'"' + re.escape(api) + r'"', body) and api not in bindings:
                    route = re.search(r'"(/(?:api|oauth2)/[^"\n]+)"', body)
                    if route:
                        bindings[api] = Binding("http", "oauth" if api.startswith("au") else "read_only", req[1], res[1], name, route[1], True)
    return sources, bindings, groups


def _declaration(source, symbol):
    if symbol in source.structs:
        return source.structs[symbol]
    for name, body in source.macros:
        if name in {"request", "required_request", "optional_request", "response", "fields", "raw_row", "values", "order_book", "collection"}:
            match = re.match(r"\s*(" + IDENT + r")\s*(?:\{|,)", body)
            if match and match[1] == symbol:
                return ("price: String, quantity: String" if name == "order_book" else body[match.end():])
        elif name == "clear_response" and body.strip() == symbol:
            return "return_code: i64, trnm: String, seq: String"
        elif name in {"list_request", "clear_request"}:
            match = re.match(r"\s*(" + IDENT + r")\s*,\s*(?:ConditionService::)?" + IDENT, body)
            if match and match[1] == symbol:
                return "trnm: ConditionService, seq: String"
        elif name == "us_quote_request" and body.strip() == symbol:
            return "stex_tp: UsExchange, stk_cd: String"
        elif name == "stock_request" and re.fullmatch(r"\s*\w+(?:\s*,\s*\w+)*\s*,?\s*", body):
            if symbol in re.findall(IDENT, body):
                return "stk_cd: String"
        elif name == "endpoint":
            match = re.match(r'\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*"[^"]+"\s*,\s*"[^"]+"\s*,', body)
            if match and symbol in (match[2], match[3], match[4]):
                return body[match.end():]
    return None


def _concrete(sources, symbol, request):
    for path, source in sources.items():
        if "/tests/" in path:
            continue
        body = _declaration(source, symbol)
        if body is None:
            continue
        if symbol in source.structs:
            if request and not body.strip():
                return True
            fields = re.findall(r"\b(?:pub\s+)?(" + IDENT + r")\s*:\s*([^\n]+)", body)
            if any(name not in {"extra", "unknown_fids"} and not re.search(r"(?:serde_json::)?Value|BTreeMap|HashMap", value)
                   for name, value in fields):
                return True
            continue
        if request and not body.strip(" \n{}"):
            return True
        if re.search(r"\b(?:String|Secret|i64|BrokerDate|Decimal|Option|Vec)\b|\w+\s*(?:=>|[:,;}])|\b(?:scalars|required|optional)\s*\{", body):
            if not re.search(r"serde_json::Value|\bextra\b|\bunknown_fids\b", body):
                return True
    return False


def _test_macro_linked(name, args, symbol, row, binding):
    # 각 매크로의 실제 식별자 인자만 연결합니다. JSON·문자열의 이름은 근거가 아닙니다.
    ident = re.escape
    test = ident(symbol)
    request = ident(binding.request)
    response = ident(binding.response)
    method = ident(binding.method)
    route = ident(binding.path)
    if row["id"] in DOMESTIC_FEEDS | US_FEEDS:
        pattern = rf'\s*{test}\s*,\s*"{ident(row["id"])}"\s*,\s*{IDENT}\s*,\s*{request}\s*,'
    elif name == "contract_test":
        pattern = rf'\s*{test}\s*,\s*{method}\s*,\s*{request}\s*,\s*"{route}"\s*,'
    elif name == "write_case":
        if symbol != row["id"]:
            return False
        pattern = rf'\s*{test}\s*,\s*{request}\s*,'
    elif name == "contract" and binding.transport == "websocket":
        pattern = rf'\s*{test}\s*,\s*{request}\s*,\s*decode_{ident(row["id"])}\s*,'
    elif name == "contract":
        # 시장: API/path/요청 식. 분석: test/request/response/method/path.
        # 상품: test/request/method/path. 모든 형식은 앞부분의 고정 위치를 확인합니다.
        patterns = [
            rf'\s*{test}\s*,\s*{request}\s*,\s*{response}\s*,\s*{method}\s*,\s*"{route}"\s*,',
            rf'\s*{test}\s*,\s*{request}\s*,\s*{method}\s*,\s*"{route}"\s*,',
        ]
        if symbol == binding.method:
            patterns.append(rf'\s*{test}\s*,\s*"{route}"\s*,\s*{request}\s*\{{')
        return any(re.match(pattern, args) for pattern in patterns)
    else:
        return False
    return bool(re.match(pattern, args))


def _evidence(root, sources, row, binding, kind, evidence, errors, label):
    path, separator, symbol = evidence.partition("#")
    try:
        file = _safe_file(root, path)
        if "#" in symbol:
            raise ValueError("multiple anchors")
        if path.endswith(".md"):
            if kind == "test_evidence":
                raise ValueError("test evidence must reference Rust test declaration")
            text = file.read_text(encoding="utf-8")
            text = re.sub(r"<!--[\s\S]*?-->", "", text)
            text = re.sub(r"(?ms)^(`{3,}|~{3,})[^\n]*\n.*?^\1[^\n]*(?:\n|$)", "", text)
            if not separator and kind != "limitation_evidence":
                raise ValueError("documentation evidence requires API anchor")
            if separator:
                headings = set()
                for heading in re.findall(r"(?m)^#{1,6}\s+(.+?)\s*#*\s*$", text):
                    heading = re.sub(r"[^\w\s-]", "", heading.lower())
                    headings.add(re.sub(r"\s", "-", heading))
                api_row = any(re.search(r"(?<![A-Za-z0-9_])" + re.escape(row["id"]) + r"(?![A-Za-z0-9_])", line.split("|", 2)[1]) for line in text.splitlines() if line.startswith("|") and line.count("|") >= 2)
                api_link = bool(re.search(r"https://openapi\.kiwoom\.com/(?:guide/apiGuideContents|m/guide/apiguide)/" + re.escape(row["public_guide_group"]) + "/" + re.escape(row["id"]) + r"(?=[)\s])", text))
                if not symbol or (symbol not in headings and not (symbol == row["id"] and (api_row or api_link))):
                    raise ValueError("unresolved Markdown heading or exact API table reference")
                if kind == "contract_evidence" and symbol != row["id"]:
                    raise ValueError("contract documentation anchor must match API ID")
            return True, False
        if not separator or not re.fullmatch(IDENT, symbol):
            raise ValueError("code evidence requires exact #symbol")
        if binding is None:
            raise ValueError("no concrete API binding for code evidence")
        if kind == "test_evidence":
            if not path.startswith("crates/kiwoom-client/tests/"):
                raise ValueError("expected Rust test location")
            source = sources.get(path) or _rust_source(file.read_text(encoding="utf-8"))
            body = source.functions.get(symbol)
            linked = False
            if body:
                before = source.text[:source.text.find(body)].rstrip()
                is_test = bool(re.search(r"#\[(?:tokio::)?test\]\s*$", before))
                linked = is_test and bool(binding.method and re.search(r"\.\s*" + re.escape(binding.method) + r"\s*\(", re.sub(r'r(?P<h>\#*)"[\s\S]*?"(?P=h)|"(?:\\.|[^"\\])*"', " ", body)))
            for name, args in source.macros:
                if name not in {"contract", "contract_test", "write_case"}:
                    continue
                linked |= _test_macro_linked(name, args, symbol, row, binding)
            if not linked:
                raise ValueError("test symbol is not a supported test declaration/invocation linked to API")
        else:
            if path not in sources or not _production_source(path):
                raise ValueError("expected production Rust location")
            source = sources[path]
            if symbol in {binding.request, binding.response}:
                if _declaration(source, symbol) is None:
                    raise ValueError("missing concrete declaration or supported DTO macro input")
            elif symbol == binding.method and binding.named:
                if symbol not in source.functions and not any(re.match(r"\s*" + re.escape(symbol) + r"\s*,", args) for name, args in source.macros if name in {"endpoint", "mutation", "read"}):
                    raise ValueError("missing named API method declaration")
            else:
                raise ValueError("symbol does not name this API's concrete contract")
        return True, True
    except (OSError, ValueError, UnicodeError) as error:
        errors.append(f"{label}.{kind}: {evidence}: {error}")
        return False, False


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON object key: {key}")
        result[key] = value
    return result


def main(argv=None):
    """기존 CLI 옵션으로 보고서를 출력하고 검증 실패 시 1을 반환합니다."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--public-html", type=Path, help="Optional official public guide HTML outside project")
    parser.add_argument("--fail-on-unimplemented", action="store_true")
    parser.add_argument("--catalog", type=Path, default=ROOT / "docs/api-inventory.json")
    args = parser.parse_args(argv)
    try:
        catalog = json.loads(args.catalog.read_text(encoding="utf-8"), object_pairs_hook=_unique_object)
        report = validate_catalog(catalog)
        if args.public_html:
            observed = set(re.findall(r"goToGuide\('[^']+', '([^']+)'\)", args.public_html.read_text(encoding="utf-8")))
            ids = {row["id"] for row in catalog.get("entries", []) if isinstance(row, dict) and isinstance(row.get("id"), str)} if isinstance(catalog, dict) and isinstance(catalog.get("entries"), list) else set()
            if not observed:
                report["errors"].append("guide parser found no IDs; source layout must be reviewed")
            elif observed != ids:
                report["errors"].append(f"guide drift missing={sorted(observed - ids)} extra={sorted(ids - observed)}")
    except (OSError, ValueError, UnicodeError) as error:
        report = {"total": 0, "markets": {}, "status": {}, "websocket_catalog_entries": 0, "errors": [f"catalog: {error}"]}
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return int(bool(report["errors"]) or args.fail_on_unimplemented and
               any(status != "typed_mock_tested" for status in report["status"]))


if __name__ == "__main__":
    sys.exit(main())
