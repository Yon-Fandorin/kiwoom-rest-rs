# Read-only MCP application contract

`apps/kiwoom-mcp` is an application consumer of the SDK, not a broker-contract
owner. It imports `kiwoom_client::rest` and `::domain`; no SDK crate depends on the
application or MCP framework. Source dependencies flow from MCP to client and
from client to domain/core. The application owns tool representation, selected
profile policy, process configuration, HTTP access and legacy MCP session limits.
Broker validation, request classification and communication remain in the SDK.

## Protocol and scope

Primary sources: [MCP transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports),
[Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http),
[legacy Streamable HTTP](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports)
and [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk).
The application uses the published `rmcp` dependency; it does not copy official
Kiwoom SDK/CLI/MCP implementations or schemas. Five independently authored tools
adapt already verified named SDK calls: `ka10001`, `ka10004`, `ka10099`, `ka10081`
and `kt00018`. This adds no broker API ID or live-verification claim to the inventory.

The same handler serves stdio and Streamable HTTP. `2026-07-28` is stateless at
protocol level in either configured mode. `--session-mode` controls legacy HTTP
sessions only. Legacy stateful responses may use SSE; preferring JSON does not
turn a stateful response into a stateless exchange. No MCP WS subscription tool,
financial mutation, token issuance or refresh operation is exposed.

## Policy and lifecycle

The default backend returns explicitly labelled synthetic fixtures. Broker mode
requires both `--backend broker` and `--allow-broker-network`, then reads only the
explicit runtime `KIWOOM_ACCESS_TOKEN`. The SDK chooses fixed broker URLs; the
application has no arbitrary URL proxy. Network construction is not broker
verification. Access token expiry remains caller-owned. Every client is created
with `WritePolicy::Deny` and zero automatic read retries.

Each process has one broker-token scope and one immutable tool profile. Explicit
JSON config is bounded to 64 KiB and contains tool names only. Invalid/missing
profiles fail closed; empty tools disable every call. `ToolPolicy` removes routes
from the actual per-instance router used for listing, tool lookup and dispatch.
Changing a profile requires restart. The profile is not a principal identity and
does not provide multi-user account isolation inside a shared HTTP process.

HTTP access uses an independent Bearer token, only loopback bindings, exact
Host/Origin allowlists, and rejection of ambiguous duplicate control headers.
Every HTTP request, including DELETE and session-bearing traffic, passes access
checks. Session IDs never bypass authorization or profile filtering. Requests
are bounded to 16 KiB including chunked bodies and a 30-second handler deadline.
Per-request MCP cancellation drops the SDK operation. Legacy session deletion or
expiry uses an application-owned cancellation token carried through HTTP request
extensions; absent legacy version headers receive the same policy. HTTP shutdown
signals active operations directly, independent of transport-drain timing.
The application wraps the existing session-manager API to cap legacy sessions
at 32 and expire them after a fixed 10-minute lifetime. A periodic reaper removes
abandoned map entries; shutdown cancels traffic and closes sessions. Session state
is not durable and does not establish continuity across expiry or restart.

All transports share one SDK client per process, preserving its request interval.
Tool calls are capped at eight simultaneous operations and a 30-second overall
call deadline, including SDK interval/pagination waits. At most four pages are
collected. The serialized `CallToolResult` is limited to 128 KiB; oversize or
incomplete pagination yields a tool error without presenting partial success.
Successful output preserves typed values, raw JSON, continuation headers and
primary-source URL. `complete=true` means the SDK observed pagination termination,
not complete historical broker coverage. `live_verified` remains false.

## Verification evidence

`tests/tools.rs` checks exact five-tool discovery, input rejection before I/O,
named typed route/body mapping, source links, raw/unknown-field preservation,
continuation, page/result/concurrency/deadline limits, no error retry and profile
isolation/dispatch rejection. `tests/cli.rs` executes the binary with a cleared
environment and synthetic config, checking stdio and config/network refusals.
`tests/config.rs` checks strict profile configuration. `tests/http.rs` checks
modern stateless traffic in both configured modes, legacy initialization/session
reuse/deletion/expiry, auth/Host/Origin/control-header validation, streaming body
limits and profile enforcement on session-bearing calls. `tests/cancellation.rs` exercises notification cancellation, modern connection
close and shutdown, and legacy DELETE/shutdown including absent-version headers;
pending SDK I/O is dropped and no continuation is sent. Private session tests in `src/http/session/tests/mod.rs`
use controlled Tokio time for capacity, expiry, reaping and shutdown.

These tests use synthetic SDK transports and local HTTP only. They do not verify
broker behavior, real credentials, external MCP product compatibility, distributed
sessions or realtime market streaming through MCP.
