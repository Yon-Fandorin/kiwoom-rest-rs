# API inventory method

The inventory tracks the independent Rust SDK in `crates/kiwoom-client`, the public call crate, backed by `kiwoom-domain` contracts and `kiwoom-core` common values. It records API identifiers, current public-guide groups and source URLs, HTTP or WebSocket transport, safety classification, implementation state, and code and test references. It contains compatibility facts and references without upstream SDK/CLI/MCP implementations, copied schemas, generated schema conversions, full documentation, or official samples.

## What is counted

The baseline is the identifiers exposed by the [official public mobile guide](https://openapi.kiwoom.com/m/guide/apiguide?dummyVal=0), observed on 2026-10-12. The current checklist has 351 unique identifiers: OAuth 2, domestic 214, US 135. The market totals include WebSocket identifiers, so the combined count remains 351. Realtime identifiers are case sensitive. An ID that appears more than once in the source counts once.

Coverage is limited to the public guide. A listed ID alone establishes neither account eligibility nor a working typed method, and the checklist does not certify the running broker backend. Legacy OCX and independently documented product interfaces are outside the inventory scope.

## Collect, reconcile, verify

1. Read the identifiers and group routing in the public menu. Maintain factual IDs and source URLs in `api-inventory.json`; retain no full guide HTML in the project.
2. Compare the local ID set with a freshly acquired public-guide HTML file held outside the project. The current checker recognizes the guide's `goToGuide(group, id)` menu wiring. If the guide layout changes or no IDs are found, stop and review the parser. An empty parsed set must fail the check.
3. Read each promoted API's current primary-source detail page. Independently implement named request/response contracts, endpoint routing, validation, broker error handling, and continuation behavior. Preserve original optional fields; missing values do not imply zero, an empty collection, or an unknown tax/FX convention.
4. Write mock contract tests with independently authored inputs. Verify expected messages and failure handling without production network access.
5. Link local contract and test symbols into the inventory. Promote only the scope actually implemented and checked. A generic JSON request facility never establishes typed API coverage.
6. Run the offline checker. When observing the public guide again, update the observation date and record source differences separately from implementation promotion. Resolve source ambiguities before adding values to the supported contract.

```sh
python3 scripts/check_api_inventory.py
python3 scripts/check_api_inventory.py --public-html /tmp/kiwoom-public-guide.html
python3 scripts/check_api_inventory.py --fail-on-unimplemented
```

The last command deliberately fails while any identifier has incomplete typed coverage. The external HTML comparison checks ID sets; it does not compare complete request/response semantics, page required flags, or broker behavior.

## States and evidence

| State | Meaning |
| --- | --- |
| `planned` | Publicly listed; no supported typed contract claimed |
| `contract_verified` | A bounded contract was independently checked; contract evidence required |
| `partial_mock_tested` | Named partial behavior has mock evidence; full API coverage remains incomplete |
| `typed_mock_tested` | Concrete typed contract and mock tests exist for the documented scope |
| `live_verified` field | Separate real-account verification flag; false for every current entry |

The inventory observed on 2026-10-12 marks all 351 identifiers as `typed_mock_tested`: HTTP 320 and WS 31, with zero live verification. The checker passes `--fail-on-unimplemented`; a compiled registry test also checks exact ID-set equality and route/mutation classifications. These counts record acceptance of the documented contract/mock scope. Backend compatibility remains unverified.

The checker validates the known catalog schema, exact boolean flags, market/group/transport/safety classifications, unique IDs, and exact primary-guide detail URLs. Promoted entries require repository-contained evidence paths and concrete declarations linked to their API ID and request/response types. Recognized category macros and named WebSocket forwarding methods are checked structurally; comments, unrelated symbols and generic calls alone do not qualify. Test evidence must name an actual test or a recognized test macro.

This is a bounded source-structure check, not a Rust compiler or proof of field semantics. It does not establish that a test ran. Documentation references such as `#ka10001` identify an exact API mention or table entry; they need not be Markdown heading anchors. Actual limitation headings are checked separately. The reviewer must inspect the contracts, run the tests, and check the current source. The dated [verification report](verification.md) records executed checks and their results. Offline contract tests and local mock network checks provide no live broker verification.

Each API category uses concrete named DTOs and methods over one shared transport. Code-generation macros expand independently authored field declarations into concrete types; a method accepting only arbitrary JSON is not a promoted typed contract. Response DTOs preserve broker field names, optional strings, and unknown fields where declared. HTTP `Page<T>` retains the parsed JSON body alongside its typed view and documented continuation/API headers. Broker strings retain signs, padding, and empty values. The SDK does not assign gross/net, withholding, or historical FX semantics to those values.

Record documentation ambiguities per endpoint. Known ambiguities include disagreements between `ka20007`'s date description and length column, product-query length/range contradictions, exchange-selector descriptions with different code meanings across endpoints, and different collection spellings in `usa24162`'s body table and sample. Preserve the chosen bounded contract and unknown raw fields, add focused tests, and link the limitation note. Do not silently create aliases, expand unsupported values, reuse another endpoint's enum meaning, or infer which published version the backend follows. `typed_mock_tested` does not resolve such ambiguities or establish support for every possible documented input combination.

## US transaction contract limits

The current public detail page for [`ust21100`](https://openapi.kiwoom.com/m/guide/apiguide/32/ust21100) requires request `tp` with values `0/3/4/5`. The typed request supports those values; `tp=8` is outside the verified contract. Dividend rows in an all-transactions response, backend acceptance of `8`, maximum lookback, tax composition, and historical KRW valuation availability remain untested.

## Redistribution and operational boundaries

Independently authored project material uses Apache-2.0. See the workspace [LICENSE](../LICENSE) and [NOTICE](../NOTICE). The license does not cover official material; redistribution rights for official documentation and schemas require separate confirmation.

Contract verification uses public documentation and offline mocks. Inventory verification involves no credential reads, broker endpoint calls, orders, conversions, transfers, or account changes. Listing or implementing a typed API does not authorize its execution. Category modules share the SDK transport boundary and typed registry. `ClientOptions` defaults to `WritePolicy::Deny`; financial methods require `WritePolicy::ExplicitOptIn` before transport and execute at most once per call. `ReqwestTransport` and `ManagedTransport` check the registered API/route pair and private financial authorization. Financial operations have no automatic transport retry or authentication replay. Application policy and accounting interpretation belong to the SDK caller.
