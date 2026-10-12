# Bounded WebSocket recovery

`ws_recovery::WsRecovery` supervises recovery when the caller invokes it, using
an injected fresh transport factory and token provider. Construction opens no
broker connection and loads no credentials. The concrete `TungsteniteFactory`
constructor validates limits without connecting; its async connection factory
requires explicit network opt-in and uses the existing fixed broker endpoints. The token-manager adapter supplies
a current local lease for each connection attempt. The supervisor has no financial
mutation request or financial retry path.

## Public contract and scope

The public [domestic 0B table](https://openapi.kiwoom.com/guide/apiGuideContents/14/0B)
and [US FE table](https://openapi.kiwoom.com/guide/apiGuideContents/40/FE), observed
2026-10-12, distinguish whole-group replacement (`refresh=0`) from adding/deleting
members (`refresh=1`). REG registers and REMOVE unregisters. The Rust implementation
uses a manual interpretation of those descriptions. No SDK, schema, official
sample, or source file was copied or transformed.

The SDK provides 23 typed `RealtimeSubscription` feeds and 8 typed condition
contracts. Recovery covers every feed and the two continuous condition searches:
domestic `Ka10173Request` and US `Usa20290Request`, paired with
exact typed clear requests `Ka10174Request` and `Usa20291Request`. The public
[ka10173 table](https://openapi.kiwoom.com/guide/apiGuideContents/15/ka10173)
and [usa20290 table](https://openapi.kiwoom.com/guide/apiGuideContents/39/usa20290)
label `search_type=1` as initial condition search plus realtime search.
List and one-shot condition requests are never replayed. Continuous conditions
are keyed by market and exact sequence, so domestic and US sequences stay distinct.
Domestic exchange is preserved; changing it requires a confirmed clear first,
consistent with the documented single-exchange selection. The published clear
requests target the exact sequence. Missing or error acknowledgements and canceled
add/clear operations leave the last confirmed condition configuration intact.

Each new session replays confirmed groups followed by confirmed conditions.
Condition replay returns a fresh initial snapshot. The supervisor checks it
against the response type and retains its raw JSON, API ID and connection generation.
It does not reconstruct missed incremental condition events. Snapshots are exposed
only while that generation is confirmed and connected; condition configuration
changes clear them. Failed or canceled replay never exposes partial snapshots.
REAL frames that arrive while awaiting the new snapshot keep their order in the
session's bounded pending buffer. The supervisor defines no condition-only REAL
feed identifier. The existing concrete condition DTOs validate all four methods.

Feed registration accepts only the existing validated, concrete
`RealtimeSubscription` and retains the last acknowledged configuration by group.
A REG/replace acknowledgement replaces the entire local group, while REG/keep
adds exact typed entries. Duplicate detection compares the entries' serialized,
validated wire fields. US item objects are split into atomic entries for exact
instrument removal; their object-array shape and exchange remain unchanged.
Domestic strings, omitted optional items and all-instrument arrays keep their
separate documented shapes and are never concatenated into a generic item value.
REMOVE/keep accepts only exact confirmed members. The recovery layer rejects
REMOVE/replace and unknown or wildcard reinterpretations before sending because
it has no verified operational meaning for them. Callers can explicitly use the
lower-level session contract separately.

`return_code=0` and successful typed decoding are required before a change or
replay is confirmed. Missing, noninteger or error codes never promote configuration.
If an acknowledgement includes `grp_no`, it must match the request. The published
acknowledgement can omit a request identifier/group. In that case, confirmation
establishes receipt of a matching service acknowledgement in the serialized,
single-owner session; it cannot establish independent broker-side equality of
every member. Condition acknowledgements require exact `seq` when present; an
omitted sequence likewise cannot positively correlate the request. Only one
request is outstanding, and pending REAL frames remain bounded. Absent server
fields provide no stronger correlation guarantee.

## Failure and cancellation rules

- Before its first awaited IO, an operation removes its live session from
  supervisor state. Dropping the subscription or event future drops that transport,
  leaves the supervisor disconnected, and retains an observable uncertainty marker.
  Cancellation and reconnect prevent stale transport buffers from being exposed.
- Unconfirmed REG/REMOVE keeps the last acknowledged configuration. Uncertain
  additions/removals are never promoted or automatically reissued. Recovery on a
  fresh session restores the last confirmed state. Callers decide whether to
  explicitly attempt their failed configuration change again.
- Each recovery attempt acquires a token, constructs a fresh transport, requires
  LOGIN acknowledgement, and sends one replacement REG per confirmed group.
  Every replay must be acknowledged before the session becomes available. Partial
  replay failures drop the new session. Only transport failures are retried; each
  retry starts with another fresh session and LOGIN, discarding the partially
  replayed state.
- Attempt count and one absolute recovery deadline bound token acquisition,
  factory connection, LOGIN, replay and backoff together. Protocol/API refusals
  stop immediately. Session deadlines also bound individual subscribe/event/close
  operations.
- Failed or canceled subscription/event IO and failed or canceled recovery
  attempts contribute gap markers. Successful reconnect/replay never erases
  existing gaps. When the bounded audit reaches capacity, the supervisor refuses
  further subscription/event/recovery IO instead of evicting unresolved gaps.
- Shutdown becomes terminal before awaiting close. A canceled or failed close
  cannot reconnect or expose the old session. Shutdown retains any pending gap
  but does not create a new gap marker for close failure. There is no background
  reconnect task, detached future, automatic stream consumption or implicit
  network opt-in.

`status()` reports the current connection, its confirmed generation, confirmed
group, entry and condition counts, unresolved gaps, and `history_verified_complete=false`.
The confirmed generation is cleared while disconnected. Re-established
subscriptions do not restore missing wire events or prove account history
completeness. Gap reconciliation requires authoritative historical evidence
outside this supervisor; no API to silently clear a gap is provided. Confirmed
configuration, snapshots and gap markers remain in memory. The SDK provides no
durable recovery checkpoint, so that state does not survive a process restart.

## Synthetic mock evidence

`crates/kiwoom-client/tests/ws_recovery.rs` exercises acknowledged replay, PING and
REAL interleaving, duplicate registration, replacement and removal, US atomic
member removal with mixed entry shapes, missing/failed/wrong-group feed acknowledgements,
partial replay failure, bounded retries, total token/factory/backoff deadline,
session timeout, future cancellation during subscription/receive/recovery/shutdown,
stale transport destruction, unknown removals, subscription limits and bounded
non-evicting gap audit.

Condition scenarios cover domestic/US continuous replay and exact clear, raw fresh
snapshot preservation, overlapping market sequence isolation, missing condition
acknowledgements, canceled add/clear, wrong sequence, refusal, one-shot selector
rejection and exchange-change guard. The tests also cover offline concrete factory
validation.

All tokens, tickers and frames are synthetic. No real broker socket or account was
accessed. These tests establish local recovery behavior; server acceptance and
live continuity remain unverified.
