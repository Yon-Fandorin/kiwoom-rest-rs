# Contract impact and validation

For each candidate record a compact private row: API ID and source URL, observed
change, current contract/call/test symbols, transport and safety implications,
classification with evidence, action, checks, and remaining unknowns. Read the
source details and code; a menu ID or hash cannot establish contract correctness.

- **Presentation-only:** formatting/labels changed with the documented contract
  unchanged. Explain what was read; avoid unsupported backend conclusions.
- **Compatible additive:** a documented optional response field or bounded new
  capability can be independently added without changing old callers. Review raw
  unknown-field retention and request validation; additive requests are not
  automatically compatible with every old backend.
- **Breaking:** method/route, required fields, type/length/value meaning,
  continuation headers, WS payload/ACK behavior, or eligibility changed. Determine
  the bounded migration and preserve public paths where feasible; do not silently
  broaden enums or replace older contracts without evidence.
- **Unresolved:** tables/prose/samples conflict, a source is unavailable, or scope
  was not observed. Preserve the current bounded contract with a limitation or
  remove the unsupported claim using the repository's convention. Do not invent
  defaults, taxes, historical FX, aliases, or server acceptance.

Keep REST/WS public calls in the established SDK entry points; contract validation
must remain independent of network and mutable execution state. Execution policy,
authentication, deadlines, retries and financial opt-in belong to their actual
runtime owner. For WS, assess subscription/ACK/cancellation/reconnect generation
and unrecovered gaps; a field decoder test alone cannot establish lifecycle
compatibility. For MCP or another caller, assess its DTO/output bounds and support
claims only if those consumers exist and are in scope.

Author fixtures from the selected bounded contract, using synthetic identifiers,
account values and timestamps rather than real accounts or copied official
samples. Test changed fields, preservation of original strings/unknown data,
validation boundaries, continuation or decoding where affected. Changes touching
financial classifications must prove denial before token acquisition/transport and
absence of automatic retries using mocks. Run target-repository checks, including
its inventory checker; record which full suites actually ran. A check that verifies
symbols exist is separate from compilation, passing tests, source review, and live
verification.

Update primary-source observations and inventory state independently. A newly
listed ID starts at the repository's unsupported/planned state. Promote only the
concrete implemented/tested scope with actual contract and test symbols. Existing
`typed_mock_tested` claims need re-evaluation if the documented contract changes;
if the catalog's schema has no stale state, use its planned/partial state or
explicit limitation convention instead of inventing a new enum value. Do not
modify `live_verified` without separately authorized and actually completed live
evidence; this workflow does not obtain that authorization.
