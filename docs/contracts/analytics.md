# Analytics contracts

Observed 2026-10-12. These 72 independent Rust contracts were manually checked
against the public Kiwoom guide's Request/Response Body tables, URL and POST method.
Each endpoint has a current desktop detail URL below. Public HTML was read
temporarily outside the repository. Official SDK, CLI, MCP and schema files were
not used to implement these contracts and are not redistributed.

All endpoints here are HTTP reads through the shared authenticated, paginated
`Client::read_pages` implementation. Each has concrete request, response and
list-row types, a named client method, and an `(api-id, route)` registry entry.

Validation rejects missing required strings, empty or whitespace-only values,
and violations of documented maximum lengths. Enumerated wire codes reject
undocumented values. Validation checks calendar dates and their order. `ka10031` ranks must be
0 through 100, with the start no greater than the end. Date-based queries using
`ka10131 dt=0` and `ka10042 qry_dt_tp=1` require both dates. Omitted optional inputs
are absent from JSON; no defaults are guessed. Fields without a documented input
length have no added maximum. US nested watchlist symbol field names were verified
from the three request examples. No undocumented nested enum or size cap is added.

Optional response fields use `Option<String>` or an optional typed list. The typed
view preserves signs, padding, empty strings, source units and numeric formatting
without converting them to financial amounts. Known fields reject JSON numbers
instead of silently coercing them to text. Top-level and row extensions remain
in `extra`. Every page also retains the unmodified response JSON and documented
pagination headers. The raw JSON distinguishes missing values, explicit null and
empty strings, even where the optional typed view merges missing and null.
No DTO infers taxes, withholding, net/gross semantics, historical FX or dividends.

## Public documentation inconsistencies

- `ka10020 trde_qty_tp` lists Length 4 and the explicit value `00100` (five bytes).
  Exact enumerated wire values take precedence over the contradictory Length.
  The implementation does not add an `0100` alias.
- The same enumerated-length inconsistency occurs in `ka10022`, `ka10023`,
  `ka10030`, `ka10033`, `ka10038 dt` (`119` versus Length 2), and `ka10062 unit_tp`
  (`1000` versus Length 1). Explicit whitelists remain strict; free-form fields
  retain their documented length cap.
- `ka10038`, `ka10040`, `ka10042` and `ka10053` document Length 6 for `stk_cd`
  but show exchange-qualified `_NX` and `_AL` forms in the same row.
  Only six-digit codes with those two suffixes receive this exception. Other
  APIs such as `ka20068`, which specify only a six-character code, do not.
- `ka10038 dt` explicitly documents an empty string for date-range queries.
  The implementation accepts that empty code and keeps the field present.
- US rank request `stex_tp` numeric labels differ across endpoints:
  some guides say `2=NASDAQ, 3=AMEX`, others `2=AMEX, 3=NASDAQ`.
  The implementation preserves each endpoint's wire strings. It does not add a
  shared enum that assumes an exchange interpretation.
- The `usa24162` Body table says `result_list`, while its Response example says
  `result_lsit`. The typed view follows the table. The sample typo remains
  unknown raw data in `extra` and is never silently aliased. Whether a real
  server emits that spelling has not been tested.
- `usa24120/usa24121 trde_prica` is described as cumulative trading value, but its
  table specifies a share unit. Raw strings are preserved; the implementation
  does not repair or convert that unit by assumption.
- The guides mark provision of `ka52301 sell_uv/buy_uv` and
  `ka10040/ka10053 qry_dt/qry_tm` as under review. These fields remain optional;
  their availability on the server is unverified.

## Offline evidence

`cargo test -p kiwoom-client --test analytics --offline` runs one named mock
contract test per endpoint. Each case compares the request JSON, API ID and route,
decodes concrete row fields, and checks preservation of signed, zero-padded and
empty strings, unknown extensions and the raw page. It also distinguishes an
absent list from an empty list and rejects a numeric value in a string field.
Nonempty request types include an invalid date, code or length case that must fail
before transport. Three additional tests cover aggregate scalar fields, range
and nested-input guards, exchange-qualified codes, and the `result_lsit` conflict.

The fixtures are independently authored synthetic data. They are not broker
responses and do not establish server compatibility. No real broker endpoint or
secret was accessed.

## Per-endpoint sources and evidence

| API | Primary guide | Route | Concrete types | Mock test |
| --- | --- | --- | --- | --- |
| `ka10014` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/17/ka10014) | `/api/dostk/shsa` | `Ka10014Request`, `Ka10014Response`, `Ka10014Row` | `tests/analytics.rs::ka10014_contract` |
| `ka01300` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/18/ka01300) | `/api/dostk/watchlist` | `Ka01300Request`, `Ka01300Response`, `Ka01300Row` | `tests/analytics.rs::ka01300_contract` |
| `ka01301` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/18/ka01301) | `/api/dostk/watchlist` | `Ka01301Request`, `Ka01301Response`, `Ka01301Row` | `tests/analytics.rs::ka01301_contract` |
| `ka10008` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/03/ka10008) | `/api/dostk/frgnistt` | `Ka10008Request`, `Ka10008Response`, `Ka10008Row` | `tests/analytics.rs::ka10008_contract` |
| `ka10131` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/03/ka10131) | `/api/dostk/frgnistt` | `Ka10131Request`, `Ka10131Response`, `Ka10131Row` | `tests/analytics.rs::ka10131_contract` |
| `ka52301` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/03/ka52301) | `/api/dostk/frgnistt` | `Ka52301Request`, `Ka52301Response`, `Ka52301Row` | `tests/analytics.rs::ka52301_contract` |
| `ka10068` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/12/ka10068) | `/api/dostk/slb` | `Ka10068Request`, `Ka10068Response`, `Ka10068Row` | `tests/analytics.rs::ka10068_contract` |
| `ka10069` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/12/ka10069) | `/api/dostk/slb` | `Ka10069Request`, `Ka10069Response`, `Ka10069Row` | `tests/analytics.rs::ka10069_contract` |
| `ka20068` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/12/ka20068) | `/api/dostk/slb` | `Ka20068Request`, `Ka20068Response`, `Ka20068Row` | `tests/analytics.rs::ka20068_contract` |
| `ka90012` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/12/ka90012) | `/api/dostk/slb` | `Ka90012Request`, `Ka90012Response`, `Ka90012Row` | `tests/analytics.rs::ka90012_contract` |
| `usa20200` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/42/usa20200) | `/api/us/watchlist` | `Usa20200Request`, `Usa20200Response`, `Usa20200Row` | `tests/analytics.rs::usa20200_contract` |
| `usa20201` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/42/usa20201) | `/api/us/watchlist` | `Usa20201Request`, `Usa20201Response`, `Usa20201Row` | `tests/analytics.rs::usa20201_contract` |
| `ka00190` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka00190) | `/api/dostk/rkinfo` | `Ka00190Request`, `Ka00190Response`, `Ka00190Row` | `tests/analytics.rs::ka00190_contract` |
| `ka00196` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka00196) | `/api/dostk/rkinfo` | `Ka00196Request`, `Ka00196Response`, `Ka00196Row` | `tests/analytics.rs::ka00196_contract` |
| `ka04196` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka04196) | `/api/dostk/rkinfo` | `Ka04196Request`, `Ka04196Response`, `Ka04196Row` | `tests/analytics.rs::ka04196_contract` |
| `ka10020` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10020) | `/api/dostk/rkinfo` | `Ka10020Request`, `Ka10020Response`, `Ka10020Row` | `tests/analytics.rs::ka10020_contract` |
| `ka10021` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10021) | `/api/dostk/rkinfo` | `Ka10021Request`, `Ka10021Response`, `Ka10021Row` | `tests/analytics.rs::ka10021_contract` |
| `ka10022` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10022) | `/api/dostk/rkinfo` | `Ka10022Request`, `Ka10022Response`, `Ka10022Row` | `tests/analytics.rs::ka10022_contract` |
| `ka10023` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10023) | `/api/dostk/rkinfo` | `Ka10023Request`, `Ka10023Response`, `Ka10023Row` | `tests/analytics.rs::ka10023_contract` |
| `ka10027` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10027) | `/api/dostk/rkinfo` | `Ka10027Request`, `Ka10027Response`, `Ka10027Row` | `tests/analytics.rs::ka10027_contract` |
| `ka10029` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10029) | `/api/dostk/rkinfo` | `Ka10029Request`, `Ka10029Response`, `Ka10029Row` | `tests/analytics.rs::ka10029_contract` |
| `ka10030` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10030) | `/api/dostk/rkinfo` | `Ka10030Request`, `Ka10030Response`, `Ka10030Row` | `tests/analytics.rs::ka10030_contract` |
| `ka10031` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10031) | `/api/dostk/rkinfo` | `Ka10031Request`, `Ka10031Response`, `Ka10031Row` | `tests/analytics.rs::ka10031_contract` |
| `ka10032` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10032) | `/api/dostk/rkinfo` | `Ka10032Request`, `Ka10032Response`, `Ka10032Row` | `tests/analytics.rs::ka10032_contract` |
| `ka10033` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10033) | `/api/dostk/rkinfo` | `Ka10033Request`, `Ka10033Response`, `Ka10033Row` | `tests/analytics.rs::ka10033_contract` |
| `ka10034` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10034) | `/api/dostk/rkinfo` | `Ka10034Request`, `Ka10034Response`, `Ka10034Row` | `tests/analytics.rs::ka10034_contract` |
| `ka10035` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10035) | `/api/dostk/rkinfo` | `Ka10035Request`, `Ka10035Response`, `Ka10035Row` | `tests/analytics.rs::ka10035_contract` |
| `ka10036` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10036) | `/api/dostk/rkinfo` | `Ka10036Request`, `Ka10036Response`, `Ka10036Row` | `tests/analytics.rs::ka10036_contract` |
| `ka10037` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10037) | `/api/dostk/rkinfo` | `Ka10037Request`, `Ka10037Response`, `Ka10037Row` | `tests/analytics.rs::ka10037_contract` |
| `ka10038` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10038) | `/api/dostk/rkinfo` | `Ka10038Request`, `Ka10038Response`, `Ka10038Row` | `tests/analytics.rs::ka10038_contract` |
| `ka10039` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10039) | `/api/dostk/rkinfo` | `Ka10039Request`, `Ka10039Response`, `Ka10039Row` | `tests/analytics.rs::ka10039_contract` |
| `ka10040` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10040) | `/api/dostk/rkinfo` | `Ka10040Request`, `Ka10040Response`, `Ka10040Row` | `tests/analytics.rs::ka10040_contract` |
| `ka10042` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10042) | `/api/dostk/rkinfo` | `Ka10042Request`, `Ka10042Response`, `Ka10042Row` | `tests/analytics.rs::ka10042_contract` |
| `ka10053` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10053) | `/api/dostk/rkinfo` | `Ka10053Request`, `Ka10053Response`, `Ka10053Row` | `tests/analytics.rs::ka10053_contract` |
| `ka10062` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10062) | `/api/dostk/rkinfo` | `Ka10062Request`, `Ka10062Response`, `Ka10062Row` | `tests/analytics.rs::ka10062_contract` |
| `ka10065` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka10065) | `/api/dostk/rkinfo` | `Ka10065Request`, `Ka10065Response`, `Ka10065Row` | `tests/analytics.rs::ka10065_contract` |
| `ka90009` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/05/ka90009) | `/api/dostk/rkinfo` | `Ka90009Request`, `Ka90009Response`, `Ka90009Row` | `tests/analytics.rs::ka90009_contract` |
| `usa01980` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa01980) | `/api/us/rkinfo` | `Usa01980Request`, `Usa01980Response`, `Usa01980Row` | `tests/analytics.rs::usa01980_contract` |
| `usa01990` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa01990) | `/api/us/rkinfo` | `Usa01990Request`, `Usa01990Response`, `Usa01990Row` | `tests/analytics.rs::usa01990_contract` |
| `usa20510` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20510) | `/api/us/rkinfo` | `Usa20510Request`, `Usa20510Response`, `Usa20510Row` | `tests/analytics.rs::usa20510_contract` |
| `usa20511` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20511) | `/api/us/rkinfo` | `Usa20511Request`, `Usa20511Response`, `Usa20511Row` | `tests/analytics.rs::usa20511_contract` |
| `usa20512` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20512) | `/api/us/rkinfo` | `Usa20512Request`, `Usa20512Response`, `Usa20512Row` | `tests/analytics.rs::usa20512_contract` |
| `usa20530` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20530) | `/api/us/rkinfo` | `Usa20530Request`, `Usa20530Response`, `Usa20530Row` | `tests/analytics.rs::usa20530_contract` |
| `usa20531` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20531) | `/api/us/rkinfo` | `Usa20531Request`, `Usa20531Response`, `Usa20531Row` | `tests/analytics.rs::usa20531_contract` |
| `usa20540` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20540) | `/api/us/rkinfo` | `Usa20540Request`, `Usa20540Response`, `Usa20540Row` | `tests/analytics.rs::usa20540_contract` |
| `usa20541` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20541) | `/api/us/rkinfo` | `Usa20541Request`, `Usa20541Response`, `Usa20541Row` | `tests/analytics.rs::usa20541_contract` |
| `usa20550` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20550) | `/api/us/rkinfo` | `Usa20550Request`, `Usa20550Response`, `Usa20550Row` | `tests/analytics.rs::usa20550_contract` |
| `usa20551` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20551) | `/api/us/rkinfo` | `Usa20551Request`, `Usa20551Response`, `Usa20551Row` | `tests/analytics.rs::usa20551_contract` |
| `usa20880` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20880) | `/api/us/rkinfo` | `Usa20880Request`, `Usa20880Response`, `Usa20880Row` | `tests/analytics.rs::usa20880_contract` |
| `usa20881` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20881) | `/api/us/rkinfo` | `Usa20881Request`, `Usa20881Response`, `Usa20881Row` | `tests/analytics.rs::usa20881_contract` |
| `usa20910` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20910) | `/api/us/rkinfo` | `Usa20910Request`, `Usa20910Response`, `Usa20910Row` | `tests/analytics.rs::usa20910_contract` |
| `usa20911` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20911) | `/api/us/rkinfo` | `Usa20911Request`, `Usa20911Response`, `Usa20911Row` | `tests/analytics.rs::usa20911_contract` |
| `usa20920` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20920) | `/api/us/rkinfo` | `Usa20920Request`, `Usa20920Response`, `Usa20920Row` | `tests/analytics.rs::usa20920_contract` |
| `usa20921` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20921) | `/api/us/rkinfo` | `Usa20921Request`, `Usa20921Response`, `Usa20921Row` | `tests/analytics.rs::usa20921_contract` |
| `usa20922` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20922) | `/api/us/rkinfo` | `Usa20922Request`, `Usa20922Response`, `Usa20922Row` | `tests/analytics.rs::usa20922_contract` |
| `usa20940` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20940) | `/api/us/rkinfo` | `Usa20940Request`, `Usa20940Response`, `Usa20940Row` | `tests/analytics.rs::usa20940_contract` |
| `usa20941` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20941) | `/api/us/rkinfo` | `Usa20941Request`, `Usa20941Response`, `Usa20941Row` | `tests/analytics.rs::usa20941_contract` |
| `usa20960` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20960) | `/api/us/rkinfo` | `Usa20960Request`, `Usa20960Response`, `Usa20960Row` | `tests/analytics.rs::usa20960_contract` |
| `usa20961` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa20961) | `/api/us/rkinfo` | `Usa20961Request`, `Usa20961Response`, `Usa20961Row` | `tests/analytics.rs::usa20961_contract` |
| `usa24110` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24110) | `/api/us/rkinfo` | `Usa24110Request`, `Usa24110Response`, `Usa24110Row` | `tests/analytics.rs::usa24110_contract` |
| `usa24111` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24111) | `/api/us/rkinfo` | `Usa24111Request`, `Usa24111Response`, `Usa24111Row` | `tests/analytics.rs::usa24111_contract` |
| `usa24120` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24120) | `/api/us/rkinfo` | `Usa24120Request`, `Usa24120Response`, `Usa24120Row` | `tests/analytics.rs::usa24120_contract` |
| `usa24121` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24121) | `/api/us/rkinfo` | `Usa24121Request`, `Usa24121Response`, `Usa24121Row` | `tests/analytics.rs::usa24121_contract` |
| `usa24150` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24150) | `/api/us/rkinfo` | `Usa24150Request`, `Usa24150Response`, `Usa24150Row` | `tests/analytics.rs::usa24150_contract` |
| `usa24151` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24151) | `/api/us/rkinfo` | `Usa24151Request`, `Usa24151Response`, `Usa24151Row` | `tests/analytics.rs::usa24151_contract` |
| `usa24160` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24160) | `/api/us/rkinfo` | `Usa24160Request`, `Usa24160Response`, `Usa24160Row` | `tests/analytics.rs::usa24160_contract` |
| `usa24161` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24161) | `/api/us/rkinfo` | `Usa24161Request`, `Usa24161Response`, `Usa24161Row` | `tests/analytics.rs::usa24161_contract` |
| `usa24162` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24162) | `/api/us/rkinfo` | `Usa24162Request`, `Usa24162Response`, `Usa24162Row` | `tests/analytics.rs::usa24162_contract` |
| `usa24200` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24200) | `/api/us/rkinfo` | `Usa24200Request`, `Usa24200Response`, `Usa24200Row` | `tests/analytics.rs::usa24200_contract` |
| `usa24201` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24201) | `/api/us/rkinfo` | `Usa24201Request`, `Usa24201Response`, `Usa24201Row` | `tests/analytics.rs::usa24201_contract` |
| `usa24290` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24290) | `/api/us/rkinfo` | `Usa24290Request`, `Usa24290Response`, `Usa24290Row` | `tests/analytics.rs::usa24290_contract` |
| `usa24291` | [Body table](https://openapi.kiwoom.com/guide/apiGuideContents/35/usa24291) | `/api/us/rkinfo` | `Usa24291Request`, `Usa24291Response`, `Usa24291Row` | `tests/analytics.rs::usa24291_contract` |
