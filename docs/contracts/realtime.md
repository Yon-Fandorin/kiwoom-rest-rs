# Independently authored realtime contracts

These independently authored contracts use the public HTML pages linked below.
No SDK, CLI, MCP source or machine-readable specification is copied, transformed
or embedded. All fixtures are synthetic; none copies an official example payload.
The source pages show no modification date.

## Scope and evidence

The 23 feeds contain 575 documented FID slots, counted per feed. A numeric ID shared
by several feeds counts once in each contract. Each documented FID has an explicit
serde rename and a named public optional string field. `unknown_fids` preserves
additional values. `RealtimeFrame` retains the exact raw text, parsed frame and
each parsed item. Session limits bound received frames and pending events; direct
calls to the decoder apply no size limit. This layer performs no financial
arithmetic and preserves broker signs, leading zeros, empty strings and scale as
wire strings.

`RealtimePayload` dispatches by exact case-sensitive ID. Malformed values for a
known feed fail decoding. An unknown feed is retained as `Unknown` without a
support claim. Missing or null data/values remain absent; explicit empty
collections remain empty. The US response field `stexTp` is separate from
registration `stex_tp`.

Each listed test compares the typed FIDs with a separately listed complete set,
fills every known FID with synthetic strings and checks typed dispatch and a
named field. It also checks that only the synthetic future FID remains unknown
and that all values round-trip without loss. Using `WsSession` with a mock socket,
each test verifies LOGIN followed by exact REG and REMOVE serialization and a
typed acknowledgement through the sealed `WsRequest` boundary. These tests cover
offline contracts. They do not verify a real server or account.

| API / primary source | Typed values | FID count | Mock test |
| --- | --- | --- | --- |
| [00](https://openapi.kiwoom.com/guide/apiGuideContents/14/00) | `DomesticOrderValues` | 34 | `domestic_order_00_contract` |
| [04](https://openapi.kiwoom.com/guide/apiGuideContents/14/04) | `DomesticBalanceValues` | 27 | `domestic_balance_04_contract` |
| [0A](https://openapi.kiwoom.com/guide/apiGuideContents/14/0A) | `StockQuoteValues` | 19 | `stock_quote_0a_contract` |
| [0B](https://openapi.kiwoom.com/guide/apiGuideContents/14/0B) | `StockExecutionValues` | 43 | `stock_execution_0b_contract` |
| [0C](https://openapi.kiwoom.com/guide/apiGuideContents/14/0C) | `BestQuoteValues` | 2 | `best_quote_0c_contract` |
| [0D](https://openapi.kiwoom.com/guide/apiGuideContents/14/0D) | `DomesticOrderBookValues` | 139 | `domestic_order_book_0d_contract` |
| [0E](https://openapi.kiwoom.com/guide/apiGuideContents/14/0E) | `AfterHoursQuoteValues` | 5 | `after_hours_quote_0e_contract` |
| [0F](https://openapi.kiwoom.com/guide/apiGuideContents/14/0F) | `StockBrokerValues` | 57 | `stock_broker_0f_contract` |
| [0G](https://openapi.kiwoom.com/guide/apiGuideContents/14/0G) | `EtfNavValues` | 12 | `etf_nav_upper_0g_contract` |
| [0H](https://openapi.kiwoom.com/guide/apiGuideContents/14/0H) | `ExpectedExecutionValues` | 7 | `expected_execution_0h_contract` |
| [0I](https://openapi.kiwoom.com/guide/apiGuideContents/14/0I) | `InternationalGoldValues` | 4 | `international_gold_0i_contract` |
| [0J](https://openapi.kiwoom.com/guide/apiGuideContents/14/0J) | `IndustryIndexValues` | 12 | `industry_index_0j_contract` |
| [0U](https://openapi.kiwoom.com/guide/apiGuideContents/14/0U) | `IndustryChangeValues` | 14 | `industry_change_upper_0u_contract` |
| [0g](https://openapi.kiwoom.com/guide/apiGuideContents/14/0g) | `StockInformationValues` | 10 | `stock_information_lower_0g_contract` |
| [0m](https://openapi.kiwoom.com/guide/apiGuideContents/14/0m) | `ElwTheoreticalValues` | 10 | `elw_theoretical_0m_contract` |
| [0s](https://openapi.kiwoom.com/guide/apiGuideContents/14/0s) | `MarketTimeValues` | 3 | `market_time_0s_contract` |
| [0u](https://openapi.kiwoom.com/guide/apiGuideContents/14/0u) | `ElwIndicatorValues` | 6 | `elw_indicator_lower_0u_contract` |
| [0w](https://openapi.kiwoom.com/guide/apiGuideContents/14/0w) | `ProgramTradingValues` | 14 | `program_trading_0w_contract` |
| [1h](https://openapi.kiwoom.com/guide/apiGuideContents/14/1h) | `VolatilityInterruptionValues` | 19 | `volatility_interruption_1h_contract` |
| [F4](https://openapi.kiwoom.com/guide/apiGuideContents/40/F4) | `UsOrderConfirmationValues` | 19 | `us_order_confirmation_f4_contract` |
| [F5](https://openapi.kiwoom.com/guide/apiGuideContents/40/F5) | `UsExecutionValues` | 36 | `us_execution_f5_contract` |
| [FE](https://openapi.kiwoom.com/guide/apiGuideContents/40/FE) | `UsTradePriceValues` | 18 | `us_trade_price_fe_contract` |
| [FT](https://openapi.kiwoom.com/guide/apiGuideContents/40/FT) | `UsOrderBookValues` | 65 | `us_order_book_ft_contract` |

## Registration shapes and source ambiguities

- `RealtimeSubscription` requires `trnm` REG/REMOVE, `grp_no`, `refresh` 0/1 and a
  nonempty `data` array. Each entry has the exact feed ID in a one-element `type`
  array. Private fields and validating constructors prevent unchecked arbitrary
  messages or order requests.
- Most domestic tables specify optional String `item`, length 200. The 0g and 0u
  tables specify length 100. `SubscriptionEntry::domestic` emits a single String
  or omits the optional item. Whitespace-only domestic items are rejected before
  socket send. It validates ELW 0m/0u six-character codes, industry 0J three-digit
  codes and gold 0I MGD/MGU against the displayed descriptions.
- Domestic 00, 0s and 1h tables specify String, but their descriptions prescribe
  `[""]`. `all_instruments` follows the descriptions. F4/F5 also prescribe `[""]`,
  although their table type is `List<Map>`. These discrepancies leave other
  server-accepted shapes unverified.
- FE/FT descriptions specify an array of objects with `jmcode` and `stex_tp`.
  `us_market` emits that shape with typed ND/NY/NA exchange codes. These realtime
  tables specify no ticker maximum length, so the constructor imposes none.
- Domestic tables specify a group length of four; US tables leave it unspecified.
  The constructor permits longer groups only when all entries are US feeds. It
  restricts group numbers to ASCII digits as a conservative local guard.
- 0B retains the single-item String shape documented in its table. Multi-symbol
  arrays for String-documented feeds remain unverified and are not exposed by
  the constructors. Entries can be batched in `data` while keeping each item's contract.
- The lowercase 0u detail page has request/response tables but no descriptive
  blockquote title. Its case, displayed ELW field identifiers and menu selection
  are preserved. It is distinct from uppercase 0U (industry change).
- F5 description labels for envelope `type`/`name` appear reversed. The shared
  envelope uses the field identifiers `type` for the exact API ID and optional
  `name`. The wording inconsistency remains unresolved.
- Domestic `values` is labeled `List<Map>` and described as Data Object. The decoder
  follows the object description and published FID key layout, rejecting arrays
  for known values. US tables label values as Map.

## Operational and financial limits

This module performs no network access, authentication, reconnect, replay,
subscription recovery or account mutation. The shared `ws::session` boundary owns
LOGIN/PING, acknowledgement waiting and socket transport. Its injectable mock
verifies protocol serialization; no feed was called on a broker service. The
partial `websocket::StockExecutionValues` remains for compatibility. The complete
0B contract is in `realtime::StockExecutionValues`.

Order and holding notifications can include confidential account fields. They
remain source DTOs. This layer assigns no accounting meaning to price, tax,
commission, balance, order status or currency fields. F5 financial/FX
realized-profit fields cannot establish a dividend receipt, its after-tax
settlement or a historical conversion rate. Missing information is never filled
with zero or a current rate.

Additional tests cover case distinctions, unknown feed preservation,
missing/empty/malformed data, registration shape and length guards, and original
signed trade/tax/currency strings.
