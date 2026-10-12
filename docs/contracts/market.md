# Independently authored market contracts

Source observation date: 2026-10-12. Public HTML documentation was read without
copying or converting official SDK, CLI, MCP source or schemas. All tests use
synthetic fixtures with injected transports. They do not establish live-account or
server support. Request and response DTOs are concrete and independently written.
`Client::read_pages` preserves raw JSON and documented headers alongside typed views.
Every documented optional scalar uses `Option<String>`, preserving signs and padding
in prices, quantities, times, flags and FX. Known arrays have concrete row types.
Unknown fields are flattened at both the top level and row level. Missing arrays
remain distinct from empty arrays.

## Domestic and US instruments and quotes

The table lists the exact source pages read. Each ID has a mock test with the same
name in `crates/kiwoom-client/tests/market.rs`; each test checks exact request JSON,
route, API identifier, typed response values, original raw response and
business-error handling. Request and response types are defined in
`crates/kiwoom-domain/src/market/instruments.rs` and re-exported through `market`.

| ID | Current public source | Request contract |
|---|---|---|
| ka10001 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10001 | required `stk_cd`, length 20, KRX/NXT/SOR examples |
| ka10002 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10002 | required `stk_cd`, length 20 |
| ka10003 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10003 | required `stk_cd`, length 20 |
| ka10004 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10004 | required `stk_cd`, length 20 |
| ka10005 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10005 | required `stk_cd`, length 20 |
| ka10006 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10006 | required `stk_cd`, length 20 |
| ka10099 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10099 | required `mrkt_tp`, length 2; 0/10/30/50/60/70/80/90/2/3/4/5/6/7/8/9 |
| ka10100 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10100 | required `stk_cd`, length 6; six-digit code |
| ka10102 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10102 | empty body |
| ka10103 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10103 | optional `input_list`; optional nested `stk_cd`, length 12 |
| usa10098 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10098 | required `stk_cd`; no maximum length given |
| usa10099 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10099 | required `stex_tp`: %/NA/ND/NY |
| usa10100 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10100 | required `stk_cd`; optional `stex_tp`: NA/ND/NY |
| usa10101 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10101 | optional `gubun`: %/1 |
| usa10102 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10102 | optional `index_qry_tp`: %/NQ/NS/NW |
| usa10103 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10103 | optional `input_list`; nested optional `stex_tp` NA/ND/NY, length 2; optional `stk_cd`, length 18 |
| usa10104 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10104 | optional `stex_tp`: %/NA/ND/NY |
| usa10105 | https://openapi.kiwoom.com/guide/apiGuideContents/31/usa10105 | optional `gubun`: %/1/2 |
| usa20100 | https://openapi.kiwoom.com/guide/apiGuideContents/33/usa20100 | required `stex_tp` NA/ND/NY and `stk_cd` |
| usa20101 | https://openapi.kiwoom.com/guide/apiGuideContents/33/usa20101 | required `stex_tp` NA/ND/NY and `stk_cd` |
| usa20150 | https://openapi.kiwoom.com/guide/apiGuideContents/33/usa20150 | required `stex_tp` NA/ND/NY and `stk_cd` |
| usa20151 | https://openapi.kiwoom.com/guide/apiGuideContents/33/usa20151 | required `stex_tp` NA/ND/NY and `stk_cd`; optional `base_dt` |
| usa20590 | https://openapi.kiwoom.com/guide/apiGuideContents/33/usa20590 | optional `stex_tp` NA/ND/NY, `stk_cd`, `base_dt` |

All endpoints above use POST. Domestic instrument requests use `/api/dostk/stkinfo`;
ka10004/ka10005/ka10006 use `/api/dostk/mrkcond`. US instruments use
`/api/us/stkinfo`; all five US quote endpoints use `/api/us/mrkcond`. Dates use the
independently implemented `BrokerDate` type, which validates calendar dates in
YYYYMMDD format. Required strings reject blank values before transport. No maximum
ticker length is enforced when the source leaves it unspecified. The StockListMarket
enum preserves all listed wire values. Omitted optional filters are absent from
serialized bodies.

The independently checked response shapes are: ka10003 `cntr_infr`; ka10005
`stk_ddwkmm`; ka10099/ka10102/usa10098/usa10099/usa10101/usa10102/usa10104/usa10105
`list`; ka10103/usa10103 `rtcd` plus `result_list`; usa20150/usa20151/usa20590
`result_list`. Other responses in this section contain top-level scalar fields.
Camel-case `listCount`, `isEtf`, `nxtEnable`, etc., and numeric-leading `250hgst`,
`52wk_hgst_pric`, etc. have explicit serde renames. All listed fields are retained,
including ka10002's five broker levels, ka10004's ten book levels and usa20101's ten
book levels plus the unusual documented `buy_1th_pre_req_pre` field.

Units remain as documented: ka10005/ka10006 `trde_prica` is described as million KRW;
usa20101 `trde_prica` as 1000 USD; usa20100 `mac` as thousand USD.
`usa20100.base_exrt` is a quote-page FX field. It does not establish a historical
execution FX rate or a ledger valuation. US quote time descriptions use ET.

## US instrument screening and seasonal returns

All 26 contracts below were independently checked on 2026-10-12 at
`https://openapi.kiwoom.com/guide/apiGuideContents/31/<ID>`. They use POST
`/api/us/stkinfo` and each has an ID-specific mock test with the same name. Concrete
request and response types are defined in
`crates/kiwoom-domain/src/market/us_screening.rs`. Most inputs are optional; the DTOs
supply no undocumented defaults. Prices use exact Decimal parsing to validate constraints without rounding across a bound
or assigning a valuation. Inputs that exceed representable precision are rejected.

| IDs | Independently checked inputs/constraints |
|---|---|
| usa20520 / usa20521 | optional numeric exchange, industry+stock / ETF categories; tm 5/10/20/30; margin, price, amount, volume filters |
| usa20570 / usa20571 | optional numeric exchange, industry+stock / ETF categories, margin, volume/amount; price bounds at most 999999.99 |
| usa20930 / usa20931 / usa20932 | surge type 1/2; time type 1/2/3; tm at most 30 (period code at most 9 for type 3); stock+industry / ETF / typed watch list; margin/price/volume/amount |
| usa20970 / usa20971 / usa20972 | high-low type 1/2; proximity 0.5/1.0/1.5/2.0/2.5/3.5; stock+industry / ETF / watch list; price endpoints, margin/volume/amount |
| usa23400 / usa23401 / usa23402 | 5/10/20/60/250 day filter; industry field **stk_cd** on usa23400 / ETF / typed **stk_cd array** on usa23402; margin/price/volume/amount |
| usa24100 / usa24101 | new high-low type 1/2, high-low basis 1/2, period at most 250; industry+stock / ETF; margin/price/volume/amount |
| usa24140 / usa24141 | sort 1/2; updown 1/2; gap filter 3/5/10/50/100/150/200 (duplicate 10 in documentation retained as one value); industry+stock / ETF; margin/price/volume/amount |
| usa24210 / usa24211 | ratio 0/1; tm 0..30; industry+stock / ETF; margin/price/volume/amount |
| usa24220 / usa24221 | period at most 300; concentration at most 100; condition 0/1/2/3; band count at most 99; industry+stock / ETF; price/volume/amount |
| usa26410 | required NA/ND/NY exchange and stock ticker |
| usa26411 / usa26412 | optional industry / ETF categories and four-digit YYYY year |
| usa26413 / usa26414 | optional industry / ETF categories |

The verified common optional screening values are: numeric exchange 0/1/2/3; stk_tp
0/1; stk_cnd 0/1/2; pric_cnd 0..10; volume 0/10/15/20/30/50/100/300/500; amount
0/1/3/5/10/30/50/100/300/500/1000/3000/5000. Amount and volume values are threshold
codes, not transaction amounts or quantities. Source enum descriptions for numeric
exchange conflict: usa20520, usa20570, usa20930, usa24100, usa24140, usa24220 say 2
NASDAQ / 3 AMEX; usa20932, usa20970, usa23400, usa24210 say 2 AMEX / 3 NASDAQ. The
DTOs validate the raw numeric values without assigning a universal exchange meaning.
No live-server interpretation is asserted.

All response collections use `result_list` with independently declared concrete row
types. usa20570/usa20571 also expose optional stock/rising/flat/falling totals.
Seasonal responses have all twelve `mNN_prft_rt` fields; `dt` may be `0` for average
or YYYY for a year and is preserved as text. usa20972 lists percentage units for its
bid/high/low fields, unlike neighboring USD quote contracts. Those fields remain
strings, with the raw numbers and documented units preserved despite the discrepancy.

## Domestic rights, credit, institutional and execution data

These domestic contracts and the remaining domestic sections are defined in
`crates/kiwoom-domain/src/market/domestic.rs` and re-exported through `market`.

Primary-source pages checked 2026-10-12:

| ID | Current source | Required and optional constraints |
|---|---|---|
| ka00198 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka00198 | required qry_tp 1/2/3/4/5 |
| ka10013 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10013 | required stk_cd length 20, dt YYYYMMDD, qry_tp 1/2 |
| ka10015 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10015 | required stk_cd length 20, strt_dt length 8; description typo YYYMMDD, date interpreted as valid eight-digit YYYYMMDD consistent with length and response |
| ka10055 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10055 | required stk_cd length 20, tdy_pred 1/2 |
| ka10084 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10084 | required stk_cd length 20, tdy_pred 1/2, tic_min 0/1; optional tm length 4 HHMM or documented empty value |
| ka90003 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka90003 | required trde_upper_tp 1/2, amt_qty_tp 1/2, mrkt_tp P00101/P10102, stex_tp 1/2/3 |
| ka90004 | https://openapi.kiwoom.com/guide/apiGuideContents/01/ka90004 | required dt YYYYMMDD, mrkt_tp P00101/P10102, stex_tp 1/2/3 |
| kt20016 | https://openapi.kiwoom.com/guide/apiGuideContents/01/kt20016 | required mrkt_deal_tp %/1/0; optional crd_stk_grde_tp %/A/B/C/D/E, stk_cd length 12 |
| kt20017 | https://openapi.kiwoom.com/guide/apiGuideContents/01/kt20017 | required stk_cd length 12 |
| ka10011 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10011 | required newstk_recvrht_tp 00/05/07 |
| ka10044 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10044 | required strt_dt/end_dt YYYYMMDD, trde_tp 1/2, mrkt_tp 001/101, stex_tp 1/2/3 |
| ka10045 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10045 | required stk_cd length 20, strt_dt/end_dt YYYYMMDD, orgn_prsm_unp_tp 1/2, for_prsm_unp_tp 1/2 |
| ka10078 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10078 | required mmcm_cd length 3, stk_cd length 20, strt_dt/end_dt YYYYMMDD |
| ka10086 | https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10086 | required stk_cd length 20, qry_dt YYYYMMDD, indc_tp 0/1 |

POST routes: group01 `/api/dostk/stkinfo`, group02 `/api/dostk/mrkcond`. Dates must
be valid calendar dates; a start date after the end date is rejected before
transport. An optional empty ka10084 tm value is serialized exactly, preserving the
documented current-time behavior. It remains distinct from an omitted tm. No broker
execution or valuation is inferred. Request validation, exact wire JSON, typed
arrays, optional raw totals and API errors have ID-specific mock evidence.

## Domestic intraday, gold and program quotes

Every ID below was checked on 2026-10-12 at
`https://openapi.kiwoom.com/guide/apiGuideContents/02/<ID>`. Each has a separate
named client method, request and response DTOs, row types and a mock test with the
same name. All use POST `/api/dostk/mrkcond`.

| IDs | Verified request constraints |
|---|---|
| ka10063 | required mrkt_tp 000/001/101, amt_qty_tp **only 1**, invsr 0..7, frgn_all 0/1, smtm_netprps_tp 0/1, stex_tp 1/2/3 |
| ka10066 | required mrkt_tp 000/001/101, amt_qty_tp 1/2, trde_tp 0/1/2, stex_tp 1/2/3 |
| ka50010 / ka50087 / ka50100 | required stock code length 20, gold symbol examples M04020000/M04020100 |
| ka50012 | required stock code length 20 and base_dt YYYYMMDD |
| ka50101 | required stock code length 20 and tic_scope 1/3/5/10/30 |
| ka90005 | required date length 8, amt_qty_tp 1/2, min_tic_tp 0/1, stex_tp 1/2/3 and matching market code |
| ka90006 | required date YYYYMMDD and stex_tp 1/2/3 |
| ka90007 | required date YYYYMMDD, amt_qty_tp 1/2, mrkt_tp 0/1, stex_tp 1/2/3 |
| ka90010 | required date YYYYMMDD, amt_qty_tp 1/2, min_tic_tp 0/1, stex_tp 1/2/3 and matching literal documented market code |
| ka90013 | required stock code length 20; optional amt_qty_tp 1/2 and date YYYYMMDD |

The ka90005 documentation says changing the date still returns the current day's
information, and the date parameter is scheduled for removal. No historical retrieval
claim is made. ka90007 describes one year's data relative to the query end date; no
live coverage validation is claimed. ka90005 matching markets: exchange 1
P00101/P10102, exchange 2 P001_NX01/P101_NX02, exchange 3 P001_AL01/P101_AL02.
ka90010 differs by literally listing **P001_AL02** for KOSDAQ exchange 3. Request
validation preserves this source discrepancy; server acceptance is unverified.
ka90010 `cntr_tm` includes the date, unlike ka90005's HHmmss. Both strings remain
raw. Gold endpoints are part of the public REST inventory, with no extra product API
assumed. The gold output fields' documented units vary among raw KRW, million KRW,
shares and thousand shares; no universal conversion is applied.

## Domestic screening and investor data

Every ID below was checked on 2026-10-12 at
`https://openapi.kiwoom.com/guide/apiGuideContents/01/<ID>`. All use POST
`/api/dostk/stkinfo` and have concrete request and response types plus a mock
contract test with the same name. Required input fields are modeled as required Rust
members.

| ID | Main independently verified constraints |
|---|---|
| ka10016 | mrkt_tp 000/001/101, ntl_tp 1/2, high_low_close_tp 1/2, stock/credit filters, padded volume enum, updown flag 0/1, period up to 250 with at most three digits, stex_tp 1/2/3 |
| ka10017 | updown_tp 1..7, sort_tp 1/2/3, stock condition including literal 10, padded volume enum, credit filter, price codes 0/1/2/3/4/5/8 |
| ka10018 | high_low_tp 1/2, alacc_rt 05/10/15/20/25/30, market/stock/credit/volume/exchange filters |
| ka10019 | market including 201, flu_tp 1/2, tm_tp 1/2, tm length 2 numeric, padded volume enum, stock/credit/price filters, updown flag 0/1 |
| ka10024 | cycle_tp 5/10/20/60/250, unpadded volume 5/10/50/100/200/300/500/1000, market/exchange |
| ka10025 | concentration integer 0..100 length 3, cur_prc_entry 0/1, prpscnt length 2 numeric, cycle 50/100/150/200/250/300, market/exchange |
| ka10026 | pertp 1..6, exchange 1/2/3 |
| ka10028 | sort 1..4, volume 0000/0010/0050/0100/0500/1000, stock/credit/market, updown 0/1, amount 0/3/5/10/30/50/100/300/500/1000/3000/5000, flu_cnd 1/2 |
| ka10043 | stock length 20, required start/end dates, qry_dt_tp 0/1, pot_tp 0/1, period 5/10/20/40/60/120, sort_base 1/2, member length 3, exchange |
| ka10052 | member length 3, market 0/1/2/3, qty 0/1/2/3/5/10/30/50/100, price 0/1/2/3/4/5/8, exchange; optional stock length 20 |
| ka10054 | market, bf_mkrt_tp only 0, motn_tp 0..3, skip_stk nine 0/1 flags, volume/amount enable 0/1, required min/max strings (blank allowed), quantity lengths 12 / amount 10, maximum values 100000000, motn_drc 0/1/2, exchange; optional stock length 20 with documented blank allowed |
| ka10058 | start/end dates, trde_tp 1/2, market 001/101, invsr_tp 8000/9000/1000/3000/3100/5000/4000/2000/6000/7000/7100/9999, exchange |
| ka10059 | required date and stock, amount/quantity 1/2, trade 0/1/2, unit 1000/1 |
| ka10061 | required stock and start/end dates, amount/quantity 1/2, **trade only 0**, unit 1000/1 |
| ka10095 | stock code scalar as current request example; multiple codes use documented pipe separator; length 20, empty components rejected |

Documented short lengths conflict with explicit allowed enums on several pages:
ka10017 stk_cnd length 1 lists 10; ka10019 trde_qty_tp length 4 lists five-digit
codes; ka10024 cycle/volume length 1 lists up to 250/1000; ka10025 cycle length 2
lists up to 300. The explicitly listed wire values take precedence over the
contradictory short lengths; no padding is removed. ka10024 accepts 1000 while
ka10016 accepts 01000, tested separately. ka10095 type column says `String []`, but
request example uses a scalar and description says pipe separators. The scalar
request body follows the public example and separator instruction. Array requests
remain unverified. No maximum number of codes beyond the listed scalar length is
inferred.

ka10054 preserves blank required input fields and optional blank stock input. Stock
skip flags are exactly nine binary values in the documented order. Field presence,
original signs and padding, conditional units and fields under data-provision review
remain intact. No DTO estimates missing market figures or resolves institution
average prices into actual execution costs.

## Domestic complete quote snapshot

https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10007 read 2026-10-12. POST
`/api/dostk/mrkcond`; required stk_cd length 20. Its independently authored
Ka10007Response contains all current optional snapshot fields: price/time/symbol
data, ten order-book levels on each side, quantities and previous changes for all ten
levels, five bid-count levels, ten liquidity-provider quantities on each side, totals
and expected-execution fields. The mock test checks tenth-level LP quantity and
signed previous-level changes, preserving raw and unknown fields. No book
aggregation, execution assumption or price-based ledger valuation is applied.

## Public-document ambiguities

Current primary-source pages read on 2026-10-12:

- https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10101
- https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10046
- https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10047
- https://openapi.kiwoom.com/guide/apiGuideContents/02/ka90008

ka10101 POST `/api/dostk/stkinfo`, required mrkt_tp 0/1/2/4/7. The public response
example uses `list` rows with scalar string `marketCode`, `code`, `name`, `group`,
even though its table labels marketCode as List<Map>. The DTO follows the explicit
scalar example and description. Its synthetic fixture uses independently chosen
values; the official example body is not copied into the project. Typed
deserialization rejects an unexpected map-array shape instead of interpreting it as a
guessed code.

The other three endpoints use POST `/api/dostk/mrkcond`. Their tables list required
stk_cd with length 6, while the explicit description includes the longer NXT/SOR
forms. Validation accepts exactly six ASCII digits, optionally followed by `_NX` or
`_AL`. This documented exception has mock tests; arbitrary suffixes, symbols, lengths
or prefixes are rejected. Server acceptance of these forms remains unverified.
ka90008 also requires amt_qty_tp 1/2 and date length 8. Its documentation says
changing date still returns only current-day data, with date planned for removal; no
historical program-trade coverage is claimed. ka10047's 5/20/60 fields retain their
documented `min` suffixes despite descriptions indicating daily intervals.

## Support and evidence

All 95 market inventory IDs have concrete independently written DTOs, named methods,
source contracts and ID-specific mock evidence; all remain `live_verified=false`.
Mock evidence and agreement with documented field names do not establish production
broker availability, units, complete historical coverage, or account semantics.
API-specific source and test references are recorded in `docs/api-inventory.json`.
Executed check results are recorded in [verification](../verification.md).
