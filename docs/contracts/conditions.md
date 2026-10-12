# Condition-search WebSocket RPC boundaries

The eight condition-search contracts use `/api/websocket`. Their primary sources
are the public guide's request/response tables. Although the guide's basic
information table lists Method POST, these requests are WebSocket JSON messages.
The HTTP client does not expose them.

`Request.encode` validates required fields, fixed service/search selectors,
documented sequence lengths, domestic exchange selectors K/A, and continuation
selectors Y/N. A Y continuation requires a nonempty key with length at most 20.
Clear requests have no documented seq maximum, so the encoders impose none.
Encoders preserve the exact field names: request `cont_yn`/`next_key` and US realtime
snapshot response `stexTp`.

Concrete response DTOs preserve optional field presence and unknown frame/row
fields. `ConditionFrame.raw` retains the full input. Decoding a response with absent
optional `return_code` or `trnm` does not establish a successful acknowledgement.
A supplied nonzero result code or mismatched service name causes decoding to fail.
Clear responses require the four fields listed in their guides. These contracts
do not verify live data shapes, the complete subscription lifecycle, replay, or
delivery.

| API | Current primary source | Independently verified boundary | Mock evidence |
| --- | --- | --- | --- |
| `ka10171` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/15/ka10171) | CNSRLST; list rows with seq/name | `tests/conditions.rs#mock_ka10171_contract` |
| `ka10172` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/15/ka10172) | CNSRREQ; maximum seq length of 3; search_type 0; stex_tp K/A; optional pagination; 10 row fields with numeric FID names and string values | `tests/conditions.rs#mock_ka10172_contract` |
| `ka10173` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/15/ka10173) | CNSRREQ; maximum seq length of 3; search_type 1; one stex_tp K/A; snapshot jmcode; six REAL values | `tests/conditions.rs#mock_ka10173_contract`, `#condition_realtime_values_and_unknown_fields_are_preserved` |
| `ka10174` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/15/ka10174) | CNSRCLR; required seq; required clear acknowledgement fields | `tests/conditions.rs#mock_ka10174_contract` |
| `usa20280` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/39/usa20280) | Request GCNSRLST; list rows with seq/name; response service discrepancy described below | `tests/conditions.rs#mock_usa20280_contract`, `#us_list_service_document_disagreement_is_retained` |
| `usa20281` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/39/usa20281) | GCNSRREQ; maximum seq length of 3; search_type 0; pagination; 12 row fields including 318/stex_tp | `tests/conditions.rs#mock_usa20281_contract` |
| `usa20290` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/39/usa20290) | GCNSRREQ; maximum seq length of 3; search_type 1; snapshot jmcode/stexTp; six REAL values | `tests/conditions.rs#mock_usa20290_contract`, `#condition_realtime_values_and_unknown_fields_are_preserved` |
| `usa20291` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/39/usa20291) | GCNSRCLR; required seq; required clear acknowledgement fields | `tests/conditions.rs#mock_usa20291_contract` |

The usa20280 request trnm description says GCNSRLST, while its response trnm
description says GCNSRLT (missing S). Response decoding accepts both strings;
the request sends only GCNSRLST. This inconsistency appears on the same guide page.
The contracts make no claim about the actual server's spelling or version behavior.

The REAL appendices for ka10173 and usa20290 require data/trnm, each item's
type/name/values, and values 841/9001/843/20/907/9081. They list I/D as the values
for 843. They do not specify a single realtime type identifier for condition search;
the text gives only general TR examples. `decode_condition_realtime` models these
fields and validates I/D and a type length of 2. It does not infer a particular type
or implicitly register a subscription.

All FIDs that appear to contain monetary values remain textual source fields. The
conditions module provides request/response DTOs and realtime notification decoding.
Socket connection, authentication, automatic continuation, reconnect, and
subscription replay are outside this module.

The eight request types implement the shared sealed `WsRequest` contract and can
be used with `WsSession::call` after LOGIN. Each per-API mock checks both direct
encoding/decoding and a synthetic LOGIN followed by a session call. The shared
session owns transport/control frames. This module adds no socket or credential
loader.

The conditions test suite (`cargo test -p kiwoom-client --test conditions`) covers
per-API fixtures with every field through a mock authenticated session, field
absence and unknown-field preservation, selector/cursor failures, realtime FID
validation, the documented service spelling disagreement, and acknowledgement
behavior when the result code is absent.

`WsReply.raw` preserves explicit null values separately from typed optional absence,
and `acknowledged=false` records uncertainty when the code is absent.
