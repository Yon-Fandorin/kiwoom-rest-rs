---
name: kiwoom-api-maintainer
description: >-
  Track public Kiwoom REST and WebSocket guide changes, assess their impact on
  independent SDK contracts, and apply verified updates with mock regression
  evidence. Use for this SDK's Kiwoom API compatibility maintenance, change checks, and release notes for feature updates.
  Not for account operations or copying official implementations or schemas.
metadata:
  version: "1.0.0"
---

# Kiwoom API Maintainer

Keep an independent Kiwoom SDK aligned with the public guide without turning
source observations into claims of backend compatibility. Work in the target
repository's established boundaries; do not assume a repository name, crate
layout, API count, or that a previous review is still current.

## Establish scope

This is a project skill for the SDK that contains it. Read the repository's
instructions, API inventory and inventory method,
public entry points, and relevant contract notes. Identify the requested API or
market coverage and whether the user wants observation, an impact report, or
implementation. A change check permits reading public documentation and creating
private evidence; apply contract changes only within the user's implementation
request. Scheduling a recurring check, publishing, credential access, and broker
execution are separate actions, not consequences of detecting a change.

## Observe and compare

Read [source tracking](references/source-tracking.md) before acquisition or
comparison. Use `scripts/track_public_guide.py` for bounded public-guide table
snapshots and ID/routing differences. Store snapshots and review records in the
target repository's verified ignored `local/` directory; never commit them, official HTML, schemas, sample code, or downloaded SDK,
CLI, or MCP implementations. Author local contracts independently from public
primary-source documentation.

Capture the requested scope into a **new** snapshot directory. A first snapshot
is a baseline, not evidence of no changes. Compare it with the prior baseline;
failed pages, uncovered IDs, parser changes, and table changes are candidates for
review, not automatic contract patches. Read the live primary-source detail page
and applicable notice/error guidance for every material candidate. The helper
covers menus and contract tables only: prose outside tables, samples, notices,
and undocumented backend changes require separate observation. Report that
coverage explicitly, even when every captured table is unchanged.

## Assess and update

Read [contract maintenance](references/contract-maintenance.md) when judging
impact or changing implementation. Trace each candidate to its contract, call
entry point, execution policy, decoder, continuation rules, consumers, tests,
and inventory evidence. Classify it as presentation-only, compatible additive,
breaking, or unresolved; use evidence for that classification. Conflicting
published descriptions remain unresolved until primary-source evidence supports
a bounded choice. Preserve broker fields, raw strings, provenance, numeric
precision, and existing public compatibility paths.

For authorized changes, write independent request/response contracts and local
mock regressions for the changed behavior and affected safety policy. Retain
financial default denial, explicit opt-in before transport, and no automatic
financial retry. Do not read keys or call broker endpoints to validate a patch.
Run the repository's required checks and affected full tests. Update inventory
source observation separately from implementation promotion: `typed_mock_tested`
needs actual typed-contract/mock evidence; never set `live_verified` from docs,
synthetic tests, compilation, or source comparison. If existing evidence is stale,
mark the bounded support limitation using the repository's inventory convention.

## Keep release notes with implementation

Read [release notes](references/release-notes.md) for API contract changes and
ordinary feature updates. Reflect verified additions, modifications, removals,
compatibility implications and user-visible limits in the root Korean
`CHANGELOG.md` under `Unreleased`, using `scripts/update_release_notes.py` to
update stable entries without duplication. Inspect the final description against
the code. When a code commit is authorized, run the staged release-note check and
include that note with the scoped implementation, tests and inventory in the
same commit. A change check with no product change needs no public history entry.
This workflow does not create a release, tag or push authorization.

## Finish

Return a Korean report (or the user's requested language) with source URLs,
covered and uncovered scope, added/removed/rerouted IDs, material field/policy
changes, impact decisions, changed contract/test/inventory symbols, executed
checks, and unresolved questions. Keep dated tables and execution history private.
A complete change check can conclude **unknown** for unsupported source coverage;
do not say “no API changes” solely because a parser found no differences. A
contract update is complete only after the requested changes and required checks
pass, or with a precise remaining blocker. Commit, push, or create an automation
only when that action is authorized.
