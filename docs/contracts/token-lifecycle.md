# Token lifecycle: local policy and mock evidence

Primary source checked 2026-10-12: [au10001 public guide](https://openapi.kiwoom.com/guide/apiGuideContents/a1/au10001), also [mobile guide](https://openapi.kiwoom.com/m/guide/apiguide/a1/au10001).
The request is `POST /oauth2/token` with `grant_type=client_credentials`,
`appkey`, and `secretkey`. The response table lists `expires_dt`,
`token_type`, and `token` as required strings. Its example has a fourteen-digit
`expires_dt` such as `20241107083713`. The table does not specify a timezone,
fixed lifetime, clock-skew tolerance, or refresh-token grant. This project contains
no copied provider code, schema, or example implementation.

Callers must explicitly construct `auth::token_manager::TokenManager`, an in-memory
SDK policy object, and inject a `TokenIssuer`, `Clock`, and `ExpiryInterpreter`.
`ClientCredentialsIssuer` calls the existing, independently authored typed
`Client::token` operation with an injected `TokenRequest`. It does not read
environment variables, files, a keychain, or broker credentials. The test issuer
returns fake tokens. Issuance errors contain no provider error message or body.

`CompactExpiryWithOffset` accepts exactly fourteen ASCII digits, validates the
calendar and time, rejects leap seconds, and requires a caller-supplied
`chrono::FixedOffset`. The compact representation follows the public example.
The caller's offset interpretation needs external confirmation before real use.
There is no implicit Korean timezone, UTC assumption, or invented token lifetime.
The lease retains the raw expiry string. Callers can supply a different expiry
policy through the injected trait without changing the broker contract.

A lease is usable only when `now + refresh_skew < expires_at`. Equality triggers
issuance. The manager rejects new tokens whose remaining lifetime is at most the
configured skew and enters the failure cooldown; it never loops issuing them.
Date arithmetic uses checked addition. The injected clock is a trust boundary:
the manager cannot correct an inaccurate system clock or an incorrect timezone
interpretation. A lease may expire between selection and dispatch.

One asynchronous mutex serializes issuance for each manager. Concurrent callers
share the successful lease and its generation while the lease remains usable.
Issuance has a bounded timeout. Failures start a bounded cooldown measured by a
monotonic clock. During the cooldown, queued callers receive the same sanitized
failure without starting another issuance. The issuance timeout excludes time
queued for the mutex; callers needing a total operation deadline must wrap lease
acquisition in their own timeout. Queued calls remain cancellable.

Cancellation drops the issuer future and starts a full cooldown before releasing
the mutex, even if the original reservation expired while the leader was waiting.
After cooldown, the next lease request to acquire the mutex may try again. There
is no background refresh or unbounded issuance retry. An injected issuer must be
cancellation safe and must not start detached work; dropping its future cannot
roll back arbitrary external side effects. Separate managers do not share
singleflight state, so callers must reuse one manager per credential/provider
scope.

`invalidate_if_generation` clears only the matching local lease. A delayed
authentication failure from an older request cannot invalidate a newer token.
It does not call token revocation or retry or replay an economic operation.
Debug output masks credentials, tokens, and the raw expiry field. Callers already
holding a lease retain that memory value; the manager cannot recall it. The manager
persists no token.

The manager and `Arc<TokenManager>` implement `WsTokenProvider` to share the same
in-memory lease with WebSocket recovery. The provider returns only the secret
token and maps manager failures to a sanitized protocol error. A token that has
not expired but was revoked externally needs explicit local invalidation.
Arbitrary LOGIN failure codes are not interpreted as expiration or revocation.

The manager performs no resource-request replay. The separately owned managed
transport policy determines whether a safe query may be retried after a
documented HTTP authentication failure. Financial requests must remain single
attempt operations, including when authentication is renewed. Arbitrary business
errors are not interpreted as token expiration.

Mock evidence: `crates/kiwoom-client/tests/token_manager.rs` covers strict expiry
validation and explicit offsets, exact skew boundary, thirty concurrent callers
with one issuance, stale generations, shared failure and deliberate retries,
leader cancellation after the reservation deadline, timeout, invalid and short
lifetimes, incorrect bearer data, redacted Debug, arithmetic overflow, bounded
configuration, and injected client-credentials issuance. All tokens and keys in
the tests are explicitly synthetic. Actual server lifetime, timezone, revocation
behavior, and real-account authentication have not been verified.
