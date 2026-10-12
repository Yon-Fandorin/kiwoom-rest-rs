# Workspace guidance

This is an independent Rust SDK pilot for the public Kiwoom REST and WebSocket
APIs. Read `README.md` for scope and `docs/architecture.md` for SDK boundaries.
Expose REST and WebSocket calls through `kiwoom-client::rest` and `::ws`.
Keep API contracts and pure validation in `kiwoom-domain`, shared values in
`kiwoom-core`, and execution policies and concrete communication in the client's
private runtime. Dependencies flow from client to domain/core and domain to core;
contracts must not depend on clients, network I/O, or mutable execution state.
The MCP application belongs in `apps/kiwoom-mcp` and consumes the public SDK.
Keep MCP tools, profile policy and server lifecycle in the app; SDK crates must
not depend on the application or MCP framework.

Preserve original broker fields and provenance. Never invent transaction amounts,
withholding semantics, historical FX, or API support. Use Decimal for money and
quantity values that the contract interprets numerically, and retain raw broker
strings as evidence.

Official SDK, CLI, or MCP source and official schema files must not be copied
or transformed here. Use public primary-source documentation to write independent
contracts. Do not read credentials or call broker endpoints during implementation
or testing. Use synthetic fixtures and local mock servers.

Every promoted API needs a concrete typed contract appropriate to its transport,
with primary-source and mock-test symbol evidence recorded in
`docs/api-inventory.json`. HTTP and RPC APIs need named client methods with typed
requests and responses. WebSocket feeds need typed payload decoding contracts;
feeds may share a subscription request shape. Generic JSON transport does not
qualify as typed API support. Keep support claims within the documented and
tested contract scope; mock verification does not establish live broker behavior.

Financial changes default to `WritePolicy::Deny`. Preserve the explicit opt-in
check before transport and the prohibition on automatic financial retries.
Implementing or testing a financial method does not authorize its execution.
This guidance grants no broker execution authorization.

## Required checks

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
python3 scripts/check_api_inventory.py
```

## Documentation language and local records

Write human-facing prose in Korean: the root README, module guides, and
`docs/verification.md`. Keep `AGENTS.md` and agent-facing architecture,
inventory-method, and contract documents in English.

The root `.codewright.toml` uses version 1 with `language.comments`,
`language.api_documentation`, and `language.test_descriptions` set to `ko`.
These settings govern production/test code comments, API documentation attached
to code (including rustdoc), and test scenario/expected-result descriptions.
The documentation policy above governs prose outside code. This policy does not
request a bulk conversion of existing source comments.

Preserve exact API literals, identifiers, serialized fields, commands, paths,
code examples, and data. Translate explanatory prose without changing contracts
or evidence.

Keep task review tables and execution/review history only under the ignored
`/local/` directory. Never publish those local records or include local branch
names or work history in public documentation.

## API maintenance and release notes

For Kiwoom public-guide change checks and contract maintenance, use the project
skill at `.agents/skills/kiwoom-api-maintainer/SKILL.md`. Confirm additions,
modifications, and removal candidates against public primary-source detail pages;
failed or partial collection does not establish removal. Keep source snapshots
and task evidence ignored under `local/`, independently author contracts, and
update typed/mock evidence without claiming live verification.

Every API contract or user-visible feature update must update the Korean root
`CHANGELOG.md` under `[Unreleased]` automatically as part of that implementation
workflow. Read the final code and describe actual behavior and support limits.
Use the project skill's idempotent release-note helper; when a code commit is
authorized, include the relevant note with the scoped code/tests/inventory in
that same commit and run:

```sh
python3 .agents/skills/kiwoom-api-maintainer/scripts/update_release_notes.py check --repo . --staged
```

Observation-only checks and unchanged product behavior need no new public history
entry. Keep task reviews and execution chronology private. Do not claim a version
was released without actual release evidence. This workflow authorizes no commit
beyond the user's scope, and no push, version tag, release publication, broker
execution, credential access, or recurring automation creation by itself.
