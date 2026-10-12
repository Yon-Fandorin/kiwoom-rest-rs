# Order, reservation and execution contracts

These 23 contracts were independently reviewed against public documentation on 2026-10-12. The review made no broker calls and copied no SDK or schema files.

The 20 financial methods require ClientOptions.writes=ExplicitOptIn. The default WritePolicy is Deny, which rejects calls before request validation or any transport operation. A private request authorization marker prevents callers using the raw read port from bypassing this check. After opt-in and successful validation, each mutation makes exactly one attempt, with no continuation or automatic retry after a transport failure or HTTP 429/503. Authentication renewal does not replay mutations. Callers must reconcile an unknown execution outcome before deciding what to do next.

The three read methods ust21204/ust21205/ust31490 use the normal bounded read transport. DTOs preserve documented string fields, omit absent optional request fields, and retain unknown raw response fields. Request validation checks documented codes and field lengths, decimal signs and whole-share quantities, dates, and explicit conditional STOP and credit-date rules.

The public documentation marks domestic order and reservation price fields as optional and does not give them the conditional price requirement stated for live US orders. These methods therefore do not impose that requirement. Ust20002 STOP requirements depend on the original order kind, which its request does not carry, so the client cannot validate that context automatically. The reservation tables also mark start and end dates as optional; the client validates supplied dates. These documented rules do not establish every trading combination that a real server permits.

## API source and mock evidence

| ID | Current public source | Request type | Mock evidence |
| --- | --- | --- | --- |
| ust21204 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust21204) | `Ust21204Request` | `tests/orders.rs#read_only_order_queries` |
| ust21205 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust21205) | `Ust21205Request` | `tests/orders.rs#read_only_order_queries` |
| ust31490 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust31490) | `Ust31490Request` | `tests/orders.rs#read_only_order_queries` |
| kt10000 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt10000) | `Kt10000Request` | `tests/orders.rs#kt10000` |
| kt10001 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt10001) | `Kt10001Request` | `tests/orders.rs#kt10001` |
| kt10002 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt10002) | `Kt10002Request` | `tests/orders.rs#kt10002` |
| kt10003 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt10003) | `Kt10003Request` | `tests/orders.rs#kt10003` |
| kt50000 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt50000) | `Kt50000Request` | `tests/orders.rs#kt50000` |
| kt50001 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt50001) | `Kt50001Request` | `tests/orders.rs#kt50001` |
| kt50002 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt50002) | `Kt50002Request` | `tests/orders.rs#kt50002` |
| kt50003 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/13/kt50003) | `Kt50003Request` | `tests/orders.rs#kt50003` |
| kt10006 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/16/kt10006) | `Kt10006Request` | `tests/orders.rs#kt10006` |
| kt10007 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/16/kt10007) | `Kt10007Request` | `tests/orders.rs#kt10007` |
| kt10008 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/16/kt10008) | `Kt10008Request` | `tests/orders.rs#kt10008` |
| kt10009 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/16/kt10009) | `Kt10009Request` | `tests/orders.rs#kt10009` |
| ust20000 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust20000) | `Ust20000Request` | `tests/orders.rs#ust20000` |
| ust20001 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust20001) | `Ust20001Request` | `tests/orders.rs#ust20001` |
| ust20002 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust20002) | `Ust20002Request` | `tests/orders.rs#ust20002` |
| ust20003 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust20003) | `Ust20003Request` | `tests/orders.rs#ust20003` |
| ust21200 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust21200) | `Ust21200Request` | `tests/orders.rs#ust21200` |
| ust21201 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust21201) | `Ust21201Request` | `tests/orders.rs#ust21201` |
| ust21203 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/38/ust21203) | `Ust21203Request` | `tests/orders.rs#ust21203` |
| ust31302 | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/41/ust31302) | `Ust31302Request` | `tests/orders.rs#ust31302` |
