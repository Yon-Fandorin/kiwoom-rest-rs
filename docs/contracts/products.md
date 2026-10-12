# Products public-document contracts

The primary sources are the linked public HTML pages, observed on 2026-10-12 without
published modification dates. The Rust DTOs are handwritten independently and the
offline fixtures use synthetic responses. No official SDK, CLI, MCP, or machine-readable
specification was used to generate them. The project includes neither full HTML nor
official examples. The fixtures contain no copied published response examples.

All entries use POST with authorization, API-id, and continuation headers through the
shared client. Each named test checks the exact API-id, path, serialized request,
typed response fields, and preserved raw response. It also verifies that an invalid
request is rejected before the mock transport is called. Dates use validated
`BrokerDate`. Deserialization rejects missing required request fields. Absent optional
fields are omitted from serialization.

Every documented response field is an optional string or typed row collection.
Unknown fields remain in `extra`, and `Page.raw` retains the complete response. Signs,
leading zeros, and broker scale are preserved. Missing arrays remain distinct from
confirmed empty arrays. Malformed arrays and numeric JSON in documented string fields
fail decoding.

ELW `/api/dostk/elw`:

| API / primary source | Handwritten request contract | Typed response | Test |
| --- | --- | --- | --- |
| [ka10048](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka10048) | `Ka10048Request`: required six-character `stk_cd` | `Ka10048Response.elwdaly_snst_ix`, `Ka10048Row` | `ka10048_contract` |
| [ka10050](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka10050) | `Ka10050Request`: required six-character `stk_cd` | `Ka10050Response.elwsnst_ix_array`, `Ka10050Row` | `ka10050_contract` |
| [ka30001](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30001) | `Ka30001Request`: required flu/time codes, time, volume threshold, issuer/asset/LP codes, right type, and expired flag | `Ka30001Response.base_pric_tm`, `elwpric_jmpflu`, `Ka30001Row` | `ka30001_contract` |
| [ka30002](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30002) | `Ka30002Request`: required issuer code (3 digits), volume, trade side, period, and expired flag | `Ka30002Response.trde_ori_elwnettrde_upper`, `Ka30002Row` | `ka30002_contract` |
| [ka30003](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30003) | `Ka30003Request`: required `bsis_aset_cd`, `base_dt` | `Ka30003Response.elwlpposs_daly_trnsn`, `Ka30003Row` | `ka30003_contract` |
| [ka30004](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30004) | `Ka30004Request`: required issuer/asset/LP codes, right type, and expired flag | `Ka30004Response.elwdispty_rt`, `Ka30004Row` | `ka30004_contract` |
| [ka30005](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30005) | `Ka30005Request`: required issuer code (12 digits), asset/LP codes, one-digit right type, and sort | `Ka30005Response.elwcnd_qry`, `Ka30005Row` | `ka30005_contract` |
| [ka30009](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30009) | `Ka30009Request`: required sort, three-digit right type, and expired flag | `Ka30009Response.elwflu_rt_rank`, `Ka30009Row` | `ka30009_contract` |
| [ka30010](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30010) | `Ka30010Request`: required sort, three-digit right type, and expired flag | `Ka30010Response.elwreq_rank`, `Ka30010Row` | `ka30010_contract` |
| [ka30011](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30011) | `Ka30011Request`: required six-character `stk_cd` | `Ka30011Response.elwalacc_rt`, `Ka30011Row` | `ka30011_contract` |
| [ka30012](https://openapi.kiwoom.com/guide/apiGuideContents/06/ka30012) | `Ka30012Request`: required six-character `stk_cd` | `Ka30012Response`: all named scalar fields, including five asset/weight pairs | `ka30012_contract` |

ETF `/api/dostk/etf`:

| API / primary source | Handwritten request contract | Typed response | Test |
| --- | --- | --- | --- |
| [ka40001](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40001) | `Ka40001Request`: required stock code (6 characters), ETF index code (3 digits), and period 0/1/2/3 | `Ka40001Response.etfprft_rt_lst`, `Ka40001Row` | `ka40001_contract` |
| [ka40002](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40002) | `Ka40002Request`: required stock code (6 characters) | `Ka40002Response`: five named scalars | `ka40002_contract` |
| [ka40003](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40003) | `Ka40003Request`: required stock code (6 characters) | `Ka40003Response.etfdaly_trnsn`, `Ka40003Row` | `ka40003_contract` |
| [ka40004](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40004) | `Ka40004Request`: required tax type, NAV comparison, manager code (4 digits), tax flag, trace index, and exchange | `Ka40004Response.etfall_mrpr`, `Ka40004Row` | `ka40004_contract` |
| [ka40006](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40006) | `Ka40006Request`: required stock code (6 characters) | `Ka40006Response`: five scalars, `etftisl_trnsn`, `Ka40006Row` | `ka40006_contract` |
| [ka40007](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40007) | `Ka40007Request`: required stock code (6 characters) | `Ka40007Response`: six scalars, `etftisl_cntr_array`, `Ka40007Row` | `ka40007_contract` |
| [ka40008](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40008) | `Ka40008Request`: required stock code (6 characters) | `Ka40008Response`: five scalars, `etfnetprps_qty_array`, `Ka40008Row` | `ka40008_contract` |
| [ka40009](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40009) | `Ka40009Request`: required stock code (6 characters) | `Ka40009Response.etfnavarray`, `Ka40009Row` | `ka40009_contract` |
| [ka40010](https://openapi.kiwoom.com/guide/apiGuideContents/10/ka40010) | `Ka40010Request`: required stock code (6 characters) | `Ka40010Response.etftisl_trnsn`, `Ka40010Row` | `ka40010_contract` |

Theme `/api/dostk/thme`, US research `/api/us/invtinfo`, FX quote
`/api/us/exchange`:

| API / primary source | Handwritten request contract | Typed response | Test |
| --- | --- | --- | --- |
| [ka90001](https://openapi.kiwoom.com/guide/apiGuideContents/11/ka90001) | `Ka90001Request`: required search, days (1..99), ranking, and exchange; optional stock/theme | `Ka90001Response.thema_grp`, `Ka90001Row` | `ka90001_contract` |
| [ka90002](https://openapi.kiwoom.com/guide/apiGuideContents/11/ka90002) | `Ka90002Request`: required group code (max 6 digits) and exchange; optional days | `Ka90002Response`: two scalars, `thema_comp_stk`, `Ka90002Row` | `ka90002_contract` |
| [usa24300](https://openapi.kiwoom.com/guide/apiGuideContents/37/usa24300) | `Usa24300Request`: optional `qry_tp` 0/1 | `Usa24300Response.result_list`, `Usa24300Row` | `usa24300_contract` |
| [ust31300](https://openapi.kiwoom.com/guide/apiGuideContents/41/ust31300) | `Ust31300Request`: required direction 1/2; optional decimal amount (max 12 characters) | `Ust31300Response`: all fourteen named quote/availability scalars | `ust31300_contract` |
| [ust31301](https://openapi.kiwoom.com/guide/apiGuideContents/41/ust31301) | `Ust31301Request`: required direction 1/2 | `Ust31301Response`: all six named quote scalars | `ust31301_contract` |

## Documentation limits and implementation boundaries

- `ka30003.bsis_aset_cd` has a table length of 12, but the description identifies a
  six-character ELW stock code. Validation uses the six-character semantic description.
- `ka30009`/`ka30010.rght_tp` omit `005`, unlike `ka30004`. Both reject `005`;
  enum sets remain API-specific. `ka30005` uses one-digit right types.
- `ka90002.date_tp` is optional with a table length of 1 and a description of 1..99.
  Validation follows the explicit 1..99 description, allowing up to two digits; the
  length contradiction remains recorded and server behavior is unverified.
- `ka90001.thema_nm` is marked for future deletion; it is optional and never auto-filled.
- `ka40009` marks many NAV-related fields as under consideration. The typed shape
  does not guarantee that the server supplies those fields; absent values stay unknown.
- `ust31300` queries exchange availability/quotes; `ust31301` queries exchange rates.
  Neither executes currency exchange. `ust31302` is intentionally absent.
- FX quote responses have no historical valuation date contract. Current displayed
  rates can change before exchange execution and cannot establish a ledger's historical
  conversion rate. No amount/fee/tax conversion is performed here.
- `ka40004.pred_dvida` is a market information field, not evidence of the account's
  after-tax dividend receipt. ETF/ELW indicators retain their published scale;
  examples with apparent decimal-scale inconsistencies are not interpreted.
- These 25 contracts have synthetic mock evidence only. No broker call, credential,
  actual account response, server behavior, or completeness of history was verified.
