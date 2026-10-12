# kiwoom-mcp

기존 `kiwoom-client`의 조회 계약을 AI 클라이언트에 제공하는 비공식 MCP 앱입니다. 기본 실행은 합성 데모이며 증권사에 연결하거나 자격 증명을 읽지 않습니다. 주문·환전·OAuth 발급·실시간 WS 구독 도구는 제공하지 않습니다.

## 실행

저장소 루트에서 실행합니다. 로컬 MCP 클라이언트에는 빌드한 `target/debug/kiwoom-mcp`를 stdio 명령으로 등록할 수 있습니다.

```sh
cargo build -p kiwoom-mcp --locked
cargo run -p kiwoom-mcp --locked -- --transport stdio
```

HTTP는 loopback 주소만 허용하며 브로커 토큰과 독립적인 접근 토큰이 필요합니다. 아래 값은 합성 데모 실행 예제입니다. 실제 사용에는 별도의 비밀 값을 지정하세요.

```sh
KIWOOM_MCP_HTTP_TOKEN=synthetic-local-access-0123456789 \
  cargo run -p kiwoom-mcp --locked -- --transport http --session-mode stateless
```

접속 주소는 `http://127.0.0.1:8080/mcp`이며 모든 요청에 `Authorization: Bearer <접근 토큰>`을 전달합니다. `--bind`로 포트가 지정된 loopback 주소를 선택할 수 있습니다.

| 전송·설정 | 동작 |
| --- | --- |
| `--transport stdio` | 실행 프로세스의 표준 입출력으로 MCP 통신. stdout에는 MCP 메시지만 출력 |
| HTTP `--session-mode stateless` | 최신·이전 규격 모두 요청 사이의 MCP 세션을 보관하지 않음 |
| HTTP `--session-mode stateful` | 이전 규격의 MCP 세션을 메모리에 보관. 최신 규격은 계속 stateless |

[MCP 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)은 프로토콜 세션을 제거했습니다. 이전 규격 `2025-11-25`의 stateful 모드는 `Mcp-Session-Id`와 SSE 응답을 사용합니다. 이 설정은 브로커 WS 연결·구독을 유지하는 기능과 다릅니다. 최신 규격과 이전 규격의 로컬 mock 왕복을 검사했으며 개별 외부 MCP 제품과의 접속은 검증하지 않았습니다.

## 사용자별 도구 설정

서버 프로세스마다 하나의 계정·설정 프로필을 사용합니다. HTTP 접근 토큰은 해당 프로세스에 대한 권한이며 여러 사용자의 계좌를 분리하는 서비스는 아닙니다. 각 사용자·권한 범위에는 별도 프로세스, 접근 토큰과 필요하면 별도 포트를 구성하세요.

명시적으로 선택한 JSON 설정에서 도구 허용 목록을 읽습니다. 자격 증명은 이 파일에 넣지 않습니다. 예를 들어 Git 제외 `local/mcp-profiles.json`에 다음 설정을 작성합니다.

```json
{
  "version": 1,
  "profiles": {
    "research": {
      "tools": ["kiwoom_stock_info", "kiwoom_order_book", "kiwoom_daily_chart"]
    },
    "portfolio": {
      "tools": ["kiwoom_account_evaluation"]
    },
    "disabled": { "tools": [] }
  }
}
```

```sh
cargo run -p kiwoom-mcp --locked -- \
  --config local/mcp-profiles.json --profile research
```

프로필은 `tools/list`와 `tools/call`에 동일하게 적용합니다. 제외한 도구는 이름을 직접 호출해도 실행하지 않습니다. 설정을 생략하면 조회 도구 5개를 모두 제공합니다. 설정 파일만 지정하면 `default` 프로필을 선택하며, 존재하지 않거나 잘못된 프로필을 전체 허용으로 대체하지 않습니다. 빈 목록은 모든 도구를 숨기고 차단합니다. 알 수 없는 도구·중복 이름·설정 버전·추가 필드·64 KiB를 넘는 설정은 거부합니다. 프로필 변경은 서버를 재시작해야 반영됩니다.

## 조회 도구

| MCP 도구 | SDK 메서드 | 입력 |
| --- | --- | --- |
| `kiwoom_stock_info` | `rest::Client::ka10001` | `stk_cd` |
| `kiwoom_order_book` | `rest::Client::ka10004` | `stk_cd` |
| `kiwoom_stock_list` | `rest::Client::ka10099` | `mrkt_tp` |
| `kiwoom_daily_chart` | `rest::Client::ka10081` | `stk_cd`, `base_dt`, `upd_stkpc_tp` |
| `kiwoom_account_evaluation` | `rest::Client::kt00018` | `qry_tp`, `dmst_stex_tp` |

입력 스키마와 SDK 검증을 모두 적용합니다. 날짜나 거래소·시장 선택값을 추정하지 않습니다. 결과는 `api_id`, `source_url`, `backend`, `live_verified=false`, `complete`, `pages`를 포함합니다. 각 페이지의 `typed`, `raw`, `response_headers`를 보존하며 금액·수량 문자열의 부호·패딩·정밀도를 바꾸지 않습니다. `synthetic_demo`는 합성 데이터입니다. `broker`도 실제 동작을 검증했다는 뜻이 아닙니다.

조회는 최대 4페이지·전체 30초·동시 8개로 제한합니다. SDK의 기본 200ms 요청 간격을 공유하며 자동 읽기 재시도는 껐습니다. 직렬화한 MCP 도구 결과는 128 KiB 이하만 반환합니다. 상한을 넘으면 일부 결과를 자르거나 성공으로 반환하지 않습니다. 연속 조회의 완전성은 브로커 페이지 종료 표시까지 처리했다는 의미이며 과거 데이터의 완전성을 증명하지 않습니다. MCP 호출자가 후속 페이지부터 이어 읽는 커서는 아직 제공하지 않습니다.

## 명시적인 브로커 구성

앱은 `--backend broker --allow-broker-network`를 함께 지정할 때만 `KIWOOM_ACCESS_TOKEN`에서 호출자가 제공한 Bearer 토큰을 읽고 SDK 전송을 구성합니다. `--environment sandbox`가 기본이며 `production`도 명시적으로 선택해야 합니다. 키 파일·키체인·앱 키는 읽지 않으며 토큰 발급·갱신을 수행하지 않습니다. 만료된 토큰은 호출자가 교체하고 서버를 재시작해야 합니다. HTTP 접근 토큰과 브로커 토큰은 같은 값을 사용할 수 없습니다.

실계좌 호출은 이 저장소의 구현·검증 범위에 포함하지 않습니다. 금융 변경은 앱에서 노출하지 않고 SDK의 `WritePolicy::Deny`를 고정합니다.

HTTP는 정확한 loopback Host·Origin과 Bearer 인증을 검증합니다. 본문은 chunked 요청도 16 KiB로 제한합니다. 이전 규격 세션은 최대 32개, 생성 후 10분이며 만료·DELETE·서버 종료 시 정리합니다. 세션 ID는 접근 권한이나 브로커 토큰이 아닙니다. 세션은 영속화하지 않으며 서버 재시작·만료 후 다시 초기화해야 합니다.

## 검증

```sh
cargo test -p kiwoom-mcp --locked
```

실제 실행 파일의 stdio 왕복, 두 규격의 HTTP 동작, 프로필 필터 우회 거부, 원문 보존·검증 오류·조회 제한·세션 종료를 합성 전송과 임시 loopback 서버로 검사합니다. 실제 브로커 응답, 외부 MCP 제품과의 상호운용, 여러 인스턴스 사이의 세션 공유, WS 실시간 MCP는 검증·지원 범위에 포함하지 않습니다.

앱 구조는 [아키텍처](../../docs/architecture.md), 계약과 테스트 근거는 [MCP 계약](../../docs/contracts/mcp.md), 프로젝트 전체 한계는 [검증 결과](../../docs/verification.md)를 참고하세요.
