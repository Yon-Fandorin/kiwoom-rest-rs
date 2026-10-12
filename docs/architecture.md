# SDK architecture reference

The workspace separates protocol calls, domain contracts and common values into three SDK crates. `apps/kiwoom-mcp` is a separate application consuming their public SDK interfaces. The public SDK entry points are `kiwoom_client::rest` and `kiwoom_client::ws`; domain contracts are available through `kiwoom_client::domain` or a direct `kiwoom-domain` dependency.

## Dependency direction

```text
kiwoom-mcp ──→ kiwoom-client
kiwoom-client ──→ kiwoom-domain ──→ kiwoom-core
kiwoom-client ──→ kiwoom-core
```

| Crate | Ownership | Permitted dependencies |
| --- | --- | --- |
| `kiwoom-mcp` (`apps/`) | MCP tools, selected profile, process configuration and MCP HTTP/session lifecycle | client and MCP/application libraries |
| `kiwoom-core` | `Secret`, `ClientError`, `Environment`, `Page<T>` | Serde, serde_json, thiserror |
| `kiwoom-domain` | Account/market/order and other API wire contracts, validation, sealed WS request metadata, offline decoding and route registration | core, Serde, serde_json, Chrono, Decimal |
| `kiwoom-client` | REST/WS call interfaces, authentication composition, execution policies and concrete transports | core, domain and the existing async/network libraries |

Here, domain means the SDK's broker-facing account, market, order and subscription contracts. Monetary strings, optional fields and unknown values retain their documented wire form. Business applications can interpret confirmed semantics above the SDK. Constructors and validation in the domain crate do not issue requests, acquire tokens or maintain connection state.

Cargo prevents domain and core from importing the client crate without changing the dependency graph. `scripts/check_architecture.py` verifies the workspace’s direct dependency graph, lower-layer dependency allowlists, absence of async/I/O execution in contract owners, and absence of outer-interface imports in runtime owners. These checks complement compilation and runtime tests; they are not a general proof of arbitrary Rust program purity.

## MCP application consumer

The reusable SDK remains under `crates/`; the executable consumer is under
`apps/kiwoom-mcp`. Its `tools` owner translates MCP inputs into existing typed SDK
requests; `config` owns immutable tool allowlists; `backend` composes injected SDK
transports; `http` owns MCP access and bounded legacy-session lifecycle. The
binary selects the backend and profile before serving. SDK crates cannot import
MCP, the application, or its framework. Tool filtering changes both discovery and
dispatch without moving broker contracts or financial policy into MCP metadata.
One process owns one account-token scope and selected profile; it is not a shared
multi-account identity service. See [MCP contract](contracts/mcp.md).

## Client interfaces and execution owners

```text
kiwoom-client/src/
  rest.rs                 # REST entry point and local call macros
  rest/                   # named calls consuming domain contracts
  ws.rs                   # WS entry point
  ws/calls.rs             # named condition calls
  auth.rs                 # public authentication composition
  runtime/http.rs         # HTTP port/envelopes and financial dispatch guard
  runtime/http/client.rs  # shared pagination, timing, retry and single-send state
  runtime/http/reqwest.rs # concrete bounded HTTP I/O
  runtime/auth/           # token leases and managed authentication policy
  runtime/ws/             # session, recovery, confirmation state and codec
```

The `runtime` module is private. Public interfaces reexport its same client, session and port types; they do not add wrapping state or extra async calls. Named REST methods belong to `rest/{accounts,market,analytics,charts,products,orders,oauth}.rs`, where they validate imported contracts and delegate to crate-private execution helpers. OAuth and financial calls retain their single-attempt paths; the mutation macro still denies writes before validating inputs.

`rest::Client<T>` receives a `Transport` port. Public `rest` exports provide the request/response envelopes, transport error, options and concrete adapter needed to compose or mock it. `ws::Client<T>` is the same session type as `WsSession<T>` and receives `WsTransport`; `ws::Recovery<F,P>` is the same supervisor as `WsRecovery<F,P>`. Factories and token providers remain injected.

The HTTP execution owner contains no endpoint DTO imports. It depends on common values and the domain registry's immutable exact API/path classification. Concrete HTTP and managed authentication independently call the same request-owned dispatch guard. A caller cannot set the private financial authorization marker through public request constructors.

## Domain owners

The domain crate contains `auth`, `accounts`, `market`, `analytics`, `charts`, `products`, `orders`, `transactions` and `ws`. Domestic/US account owners and shared category validation remain private children with explicit public reexports. Contract macros create DTOs and validators only; they do not create `Client` methods. The public call macros consume those concrete types in the client crate.

`accounts/transactions.rs` owns the domestic and US transaction requests, selectors and response rows. `transactions.rs` owns shared dates and wire vocabulary, plus same-type compatibility exports. `orders.rs` contains request validation and response models grouped by domestic cash/credit, gold, US live orders, reservations, purchasing-power queries and exchange. Execution permission belongs to the client.

Domain `ws/request.rs` owns the sealed `WsRequest` metadata trait. Only independently implemented domain requests can satisfy its private seal. Its implementations provide API IDs, response types, service correlation metadata and validation, with no session or transport reference. Conditions, subscription entries, realtime payloads, FID dispatch and offline frame decoding retain their separate owners.

The domain registry combines OAuth and category route tables and exposes read-only classification functions. The client consumes that registration; the domain has no reverse dependency on the client. The inventory checker scans the three explicit source roots and the existing client integration tests. A domain DTO or metadata declaration alone cannot establish a named executable API contract.

## Authentication and WebSocket lifecycle

`TokenManager` belongs to internal authentication runtime and receives a `TokenIssuer`, `Clock` and `ExpiryInterpreter`. It has no HTTP client field. The REST `ClientCredentialsIssuer` implements that port through explicit OAuth issuance and belongs to `rest/oauth.rs`. `ManagedTransport` uses leases to apply tokens and the caller's selected authentication replay policy. OAuth bypasses managed acquisition to avoid recursion.

The WS recovery owner adapts `TokenManager` to `WsTokenProvider`; authentication does not import WS. Recovery, session and concrete codec own their mutable lifecycle state. The domain request trait contains none of that state. ACK promotion, absolute deadlines, capacity limits, raw responses, generation-aware lease invalidation and cancellation invalidation remain in their existing runtime owners. See [token lifecycle](contracts/token-lifecycle.md), [managed authentication](contracts/managed-auth.md), [WS recovery](contracts/ws-recovery.md) and [verification](verification.md).

## Public compatibility and safety

Existing root `Client`, `transport`, category DTO modules and WS module aliases reexport the same definitions. Canonical construction uses `rest::Client` and `ws::Client`; new code imports request/response types through `domain`. Existing public integration test targets remain, and layered-path tests check type identity and mock calls. Moving a type to another crate does not promise compatibility with already compiled Rust artifacts; source-level paths and public behavior are tested.

Default financial execution remains denied. Explicit opt-in never permits automatic resubmission after transport, broker or authentication errors. Response strings, parsed raw JSON and unknown fields remain preserved. Actual account behavior, broker quotas, expiry timezone, historical coverage and economic field meanings remain unverified.

The inventory has 351 public-guide IDs (HTTP 320, WS 31), all `typed_mock_tested` and `live_verified=false`. Its source/evidence references are independently authored compatibility facts, not copied official SDK code, schemas or full documentation. Apache-2.0 applies to project material for which its authors hold rights; it does not grant official-material or broker access rights.

Required checks are workspace tests, fmt, Clippy with warnings denied, rustdoc with warnings denied, Python regression tests, API inventory and architecture checks. Keep task work tables and execution history under ignored `/local/`. Package publication, real broker calls and deployment require separate authorization.
