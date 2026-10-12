# kiwoom-client

키움 공개 REST·WebSocket API를 독립적으로 구현한 비공식 Rust SDK입니다. 국내·미국주식 API의 타입이 정의된 요청·응답과 클라이언트 메서드, HTTP 전송, 인증과 WebSocket 세션을 제공합니다.

## 구성과 사용

[저장소 README](../../README.md)의 path 의존성 설정으로 연결합니다. `rest::Client::new(transport, options)`에 `Transport` 구현과 `ClientOptions`를 전달해 HTTP 클라이언트를 구성합니다. 생성 시 네트워크 연결은 열지 않습니다.

| 구성 | 역할 |
| --- | --- |
| `rest::Client` / `rest::ClientOptions` | 타입별 호출, 요청 간격·페이지 수·읽기 재시도 제한 |
| `ReqwestTransport` | 등록된 API·경로를 확인하는 HTTP 전송 |
| `TokenManager` / `ManagedTransport` | 메모리 내 토큰 수명 관리와 인증된 HTTP 요청 |
| `ws::Client` (`WsSession`) | LOGIN/PING, 실시간 구독·해제와 조건검색 요청 |
| `ws::Recovery` (`WsRecovery`) | 호출자가 진행하는 제한된 재연결과 확인된 구독 재등록 |

기능별 요청·응답은 `domain::accounts`, `domain::market`, `domain::orders` 등의 모듈에 있습니다. 호출·인증·통신은 `kiwoom-client`, 순수 계약은 `kiwoom-domain`, 공통 값은 `kiwoom-core`가 소유합니다. [레이어 구조](../../docs/layering.md)와 [API 인벤토리](../../docs/api-inventory.json)를 참고하세요.

오프라인 호출 예제는 [거래 이력·페이지네이션](tests/contracts.rs), [관리형 인증](tests/managed_transport.rs), [WebSocket 세션](tests/ws_session.rs), [구독 복구](tests/ws_recovery.rs) 테스트에 있습니다.

```sh
cargo test --workspace --locked
```

## 전송과 인증 정책

HTTP·WebSocket 네트워크 생성 메서드는 명시적인 opt-in을 요구하며 고정된 증권사 도메인을 사용합니다. 자격 증명은 호출자가 전달하며 SDK는 비밀 파일·환경 변수·키체인을 읽지 않습니다.

HTTP 요청 간격, 읽기 재시도와 연속 조회에는 설정 가능한 제한이 있습니다. 이 제한만으로 증권사의 호출 할당량을 준수한다고 보장하지는 않습니다. OAuth와 금융 변경은 자동 재시도하지 않습니다.

주문·환전 등 금융 변경은 기본적으로 `WritePolicy::Deny`를 적용합니다. 호출자가 `WritePolicy::ExplicitOptIn`을 설정해야 전송할 수 있으며 정책은 전송 전에 검사합니다. 타임아웃이나 인증 실패로 실행 여부가 불확실해져도 자동으로 다시 제출하지 않습니다.

`TokenManager`는 호출자가 지정한 만료 시각 해석을 사용하며 동시 발급을 제어합니다. 공개 만료 표에 시간대와 고정 토큰 수명이 명시되어 있지 않으므로 호출자가 해석을 제공해야 합니다. `ManagedTransport`는 설정에 따라 HTTP 401 이후 토큰을 갱신하고 등록된 안전한 읽기 요청만 다시 실행합니다. [토큰 수명 주기](../../docs/contracts/token-lifecycle.md), [관리형 인증](../../docs/contracts/managed-auth.md).

## WebSocket 세션

WebSocket 기능은 `ws` 아래의 `session`, `transport`, `recovery`, `subscription`, `realtime`, `conditions`, `protocol`에서 찾을 수 있습니다. 기존 `ws_session`·`ws_transport`·`ws_recovery`·`realtime`·`conditions`·`websocket` 경로도 같은 모듈의 alias로 사용할 수 있습니다.

`WsSession`은 타입이 정의된 요청을 처리하고 작업 마감 시각과 프레임·버퍼 크기 제한을 적용합니다. 선택 필드인 `return_code`가 없으면 `acknowledged=false`이며 성공 승인으로 처리하지 않습니다. 요청과 응답 양쪽에 `seq`가 있을 때만 비교하며, 생략된 순번은 정확한 대응을 보장하지 않습니다. 프로토콜·전송 실패, 타임아웃이나 실행 중인 요청 future 취소 후에는 새 전송과 세션이 필요합니다.

`WsRecovery`는 ACK로 확인한 실시간 그룹과 국내·미국 조건검색 연속 구독을 새 세션에 재등록합니다. 누락된 이벤트 이력은 복구하지 않으며 해결되지 않은 데이터 공백은 유지합니다. [복구 계약](../../docs/contracts/ws-recovery.md).

## 원본 값과 지원 범위

페이지·WebSocket 응답의 원본 JSON과 문자열 형태의 금액·수량·FID를 보존합니다. 부호·패딩·빈 문자열을 경제적 값으로 정규화하지 않으며, 선택 컬렉션의 생략은 명시적인 빈 배열과 구분합니다. 응답 필드의 세금·결제 의미나 과거 환율 환산 값을 자동으로 추정하지 않습니다.

지원 범위는 공개 문서에 근거한 계약과 오프라인 mock 테스트 범위입니다. 실계좌 응답과 모든 입력 조합의 동작, 과거 이력의 완전성은 검증하지 않았습니다. API별 필드 제약과 공개 문서의 불일치는 [API 계약](../../docs/contracts), 검증 한계는 [검증 결과](../../docs/verification.md)를 참고하세요.

독립 작성한 코드와 문서는 [Apache-2.0](../../LICENSE)으로 제공합니다.
