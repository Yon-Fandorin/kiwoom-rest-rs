# Managed authenticated transport

The independently authored `ManagedTransport<T>` combines an injected in-memory
`TokenManager` with the existing allowlisted transport. It does not enable network
access, read credentials from files or the environment, or persist tokens. See
[token lifecycle](token-lifecycle.md) for the current au10001 contract and the
requirement for an explicit expiry-timezone interpretation. The wrapper masks
its inner transport and token manager in Debug output.

Before acquiring a lease, the wrapper verifies the exact registered API/path pair
and checks the private authorization marker for financial routes. Authenticated
queries and explicitly enabled financial operations acquire a lease and replace
the request's token with the lease's token. OAuth issuance and revocation bypass
lease acquisition to prevent recursion. Both remain single-attempt operations
with explicit request bodies.

`AuthenticationRetry::Never` is the default. Callers can explicitly select
`SafeReadHttp401Once` to invalidate the matching token generation and replay a
registered read query once with a renewed token per transport attempt.
[HTTP 401 semantics](https://www.rfc-editor.org/rfc/rfc9110.html#name-401-unauthorized)
indicate missing or invalid authentication credentials. They do not establish that
a Kiwoom token expired or identify a Kiwoom business error. The wrapper does not
reclassify broker business codes as authentication errors. It returns a second 401
and invalidates that lease without starting a refresh loop. A delayed failure
cannot invalidate a newer generation.

The outer client retries safe reads on 429, 502-504, and transport failures.
If the authentication replay encounters a separately retryable read failure, the
page can require at most `2 * (max_read_retries + 1)` resource sends. This bound is
local configuration, not a broker quota. Repeated 401 responses stop after two
sends because the outer client does not retry that status. Authentication replay
preserves page and continuation state. Collecting pages never performs a financial
write.

Every financial request executes at most once, including after a 401, 503,
transport failure, or broker error. The wrapper may acquire a usable token before
the initial send and invalidate a matching local token after a 401. It never
replays the financial operation automatically. An uncertain financial outcome
requires external reconciliation. `WritePolicy::Deny` is the default. The client
rejects denied operations, and the wrapper rejects forged raw financial requests,
before any OAuth request.

Mock tests in `tests/managed_transport.rs` cover replacement of the caller's
token with the manager's token, 401 responses with authentication retry disabled,
bounded repeated 401 responses, default write denial, raw route forgery, OAuth
without recursion, preservation of arbitrary business errors, and all four
financial failure classes without replay. Invalid opted-in input is rejected before token issuance; cancellation during issuance, request spacing, or a resource send has no background replay. A protocol-control header error remains terminal through both authentication and outer read retry layers. The single-page typed test also checks
that invalid cursors are rejected before I/O and malformed continuation headers
remain in the returned page. All keys, tokens, and responses are synthetic.

The per-page client returns only successfully decoded typed pages. That API does
not expose the body of a malformed or failed response.
