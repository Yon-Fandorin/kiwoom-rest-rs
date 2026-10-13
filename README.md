# kiwoom-rest-rs

키움증권 공개 REST·WebSocket API의 비공식 독립 Rust 클라이언트입니다. 국내·미국주식의 타입이 정의된 요청·응답과 클라이언트 메서드를 제공합니다. 키움증권의 공식 SDK나 제휴 프로젝트가 아닙니다.

- OAuth, 거래 이력, 계좌·시장·차트·분석·상품 조회와 주문·예약·환전 계약
- HTTP 요청 간격 제한, 페이지네이션, 오류 처리와 제한된 읽기 재시도
- 토큰 수명 관리, 관리형 인증과 `ws` 모듈의 실시간·조건검색 세션 및 제한된 재연결
- 조회·실시간 응답의 원본 값 보존
- `apps/kiwoom-mcp`의 조회용 MCP 서버, stdio·HTTP와 프로필별 도구 허용 목록

## 사용

패키지 발행은 비활성화되어 있습니다. 저장소를 복제한 뒤 로컬 경로를 path 의존성으로 연결하세요.

```toml
[dependencies]
kiwoom-client = { path = "../kiwoom-rest-rs/crates/kiwoom-client" }
```

호출은 `rest::Client`와 `ws::Client`, 요청·응답은 `domain`에서 시작합니다. 공통 값, 도메인 계약, 호출·실행은 세 SDK 크레이트로 분리했습니다. 이를 사용하는 실행 앱은 `apps/`에 둡니다. [레이어 구조](docs/layering.md)를 참고하세요.

구성과 호출 예제는 [클라이언트 문서](crates/kiwoom-client/README.md)를 참고하세요. Rust stable `1.93.1` 기준이며, 의존성 버전은 `Cargo.lock`에 고정했습니다.

저장소에서 크레이트 테스트를 실행합니다.

```sh
cargo test --workspace --locked
```

## MCP 앱과 변경 관리

[kiwoom-mcp](apps/kiwoom-mcp/README.md)는 조회 도구 5개를 제공하며 기본 실행은 합성 데모입니다. stdio와 Streamable HTTP, 사용자 설정 프로필을 지원합니다. HTTP stateful 설정은 이전 MCP 규격의 세션에 적용하며 최신 규격은 stateless입니다. 실시간 WS 구독 도구는 아직 제공하지 않습니다.

```sh
cargo run -p kiwoom-mcp --locked -- --transport stdio
```

키움 공개 가이드 변경은 프로젝트의 [API 유지보수 스킬](.agents/skills/kiwoom-api-maintainer/SKILL.md)로 확인합니다. 확인된 API 추가·수정·삭제와 일반 기능 변경은 [변경 이력](CHANGELOG.md)의 `Unreleased`에 반영하고 관련 코드와 같은 커밋에 포함합니다. 변경 감지 자료·검토 기록은 Git 제외 `local/`에 보관합니다. 이 흐름은 릴리즈 게시나 실계좌 호출을 수행하지 않습니다.

지속 점검에 사용하는 [모니터링 절차](.agents/skills/kiwoom-api-maintainer/references/monitoring.md)는 공식 GitHub의 업데이트 신호와 공개 가이드를 대조합니다. 문서 PR 작성이 승인된 경우 계약·지원 의미가 바뀌지 않는 문서 정정만 검사 후 PR로 제출하고, 코드·계약에 영향이 있거나 의미가 불명확한 변경은 판단을 요청합니다. Your dot 또는 로컬 에이전트의 실행 환경·주기 등록은 별도로 필요합니다.

## 실행 정책

네트워크 연결은 명시적으로 허용해야 하며 자격 증명은 호출자가 전달합니다. 주문·환전 등 금융 변경은 기본적으로 `WritePolicy::Deny`로 차단합니다. `WritePolicy::ExplicitOptIn`을 설정해도 자동 재시도하거나 인증 실패 후 다시 제출하지 않습니다.

MCP 앱은 별도의 명시적인 broker 모드를 선택할 때만 호출자가 제공한 토큰 환경 변수를 읽습니다. SDK 자체의 비밀 값 자동 탐색 정책은 바뀌지 않습니다.

실계좌 동작은 검증하지 않았습니다. 지원 범위는 공개 문서에 근거한 계약과 오프라인 mock 테스트 범위이며, 레거시 OCX는 포함하지 않습니다. WebSocket 재연결은 확인된 구독을 재등록하며 누락된 이벤트 이력을 복구하지 않습니다.

## 문서

- [API 클라이언트](crates/kiwoom-client/README.md): 구성, 호출 예제와 전송·인증 정책
- [API 계약](docs/contracts): API별 출처, 필드 제약과 문서 불일치
- [API 인벤토리](docs/api-inventory.json): 구현·mock 근거와 지원 상태
- [아키텍처](docs/architecture.md): 클라이언트와 전송·인증·WebSocket 경계
- [검증 결과](docs/verification.md): 검증 범위와 한계

## 라이선스

독립 작성한 코드와 문서는 [Apache License 2.0](LICENSE)으로 제공합니다. API 계약은 키움증권의 [공개 가이드](https://openapi.kiwoom.com/m/guide/apiguide?dummyVal=0)를 근거로 작성했습니다.
