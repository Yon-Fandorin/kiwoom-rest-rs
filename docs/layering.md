# REST·WS와 도메인 계약의 레이어

호출은 `rest`와 `ws`에서 시작하고, 계좌·시장·주문 등의 요청·응답은 `domain`에서 가져옵니다. 도메인 계약은 통신과 세션 상태를 참조하지 않습니다.

```text
사용자 코드 / apps/kiwoom-mcp
  ├─ rest::Client ── REST 실행 정책 ── HTTP 전송
  └─ ws::Client / ws::Recovery ── WS 세션·복구 ── 소켓 전송
          │
          └─ domain: 계좌·시장·주문·구독의 계약과 검증
                    │
                    └─ core: 공통 값·오류·페이지
```

| 크레이트 | 책임 |
| --- | --- |
| `kiwoom-mcp` (`apps/`) | SDK를 조합하는 MCP 도구·설정·접근 정책과 MCP 세션 수명 |
| `kiwoom-core` | 비밀 문자열, 오류, 서버 환경과 원본 응답 페이지 |
| `kiwoom-domain` | 요청·응답, 입력 검증, 구독 계약과 오프라인 디코딩 |
| `kiwoom-client` | REST·WS 호출, 인증 구성, 호출 제한·취소·재시도와 실제 통신 |

앱의 Cargo 의존 방향은 `mcp → client`이며 SDK는 `client → domain → core`와 `client → core`입니다. 앱은 SDK 공개 인터페이스만 사용하고 SDK는 앱이나 MCP 프레임워크를 참조하지 않습니다. 도메인 크레이트는 `Client`·Reqwest·Tokio를 의존성으로 갖지 않습니다. 클라이언트로 향하는 역방향 의존은 Cargo의 순환 의존 검사로 차단합니다. [레이어 검사](../scripts/check_architecture.py)는 의존성 변경으로 이 경계를 우회하거나 도메인 안에 비동기 실행을 넣는 경우도 검사합니다.

## 호출과 계약 사용

HTTP 사용자는 `rest::Client`, WS 사용자는 `ws::Client`를 구성합니다. 요청 타입은 `domain::accounts::Ka00001Request`처럼 가져옵니다. `rest::Transport`와 `ws::WsTransport`에 mock 또는 실제 어댑터를 주입할 수 있습니다. 객체 생성과 계약 검증만으로 네트워크 연결이 열리지는 않습니다.

- [REST mock 호출](../crates/kiwoom-client/tests/layered_calls.rs)
- [WS mock 호출과 이전 경로 호환성](../crates/kiwoom-client/tests/ws_paths.rs)
- [레이어별 타입 경로 호환성](../crates/kiwoom-client/tests/sdk_paths.rs)

토큰 관리자는 공유 인증 상태를 관리하고 발급자를 주입받습니다. REST의 `ClientCredentialsIssuer`가 OAuth 발급을 담당하며, WS 복구는 토큰 공급 포트를 사용합니다. 도메인 요청 타입은 토큰 발급이나 연결 복구를 수행하지 않습니다.

SDK의 도메인은 키움의 API 계약 영역입니다. 투자 분석이나 원장 계산을 추가할 때는 이 SDK 위의 애플리케이션 계층에서 계약 데이터를 사용합니다.

## 검증과 호환성

기존 `Client`, `accounts`, `transport`, `ws_session` 등의 경로는 같은 타입의 재수출로 유지합니다. 레이어를 통과해도 상태 객체를 한 번 더 감싸거나 API 요청을 재실행하지 않습니다. 금융 변경의 기본 차단, 원본 문자열 보존과 취소 이후 세션 무효화 정책도 유지합니다.

```sh
python3 -B scripts/check_architecture.py
python3 -B scripts/check_api_inventory.py --fail-on-unimplemented
cargo test --workspace --locked --offline
```

구현자를 위한 상세 경계는 [아키텍처 참조](architecture.md), 실제 검사 범위는 [검증 결과](verification.md)에 있습니다.
