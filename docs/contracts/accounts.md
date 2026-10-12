# Account contracts

The 59 independently authored account contracts use public guide tables and
synthetic mock inputs. They include no official SDK, CLI or MCP source, downloaded
specification, or example response. All documented response fields use `Option` to preserve absence. Textual
amounts remain verbatim, and `Page.raw` retains the entire response. These types
describe the broker response structure; financial normalization and live-account
validation remain unverified.

Required request fields reject missing or empty values. Validation runs before
transport and enforces documented maximum lengths, listed selector values, valid
calendar periods, and ordered date ranges. Optional calendar fields may be empty for
documented current/default-date queries; omitted optional fields are not serialized.
Fields without a documented length have no imposed maximum. The `ka10088` guide lists
length 20 and describes 7 digits. Validation uses the length column as the maximum
without imposing an undocumented fixed length. The `ust21510.stex_tp` guide lists
length 2 without enumerating selector values, so validation preserves its values
without applying another endpoint's exchange enumeration.

Fields such as `dvid_amt`, `exrt`, `exch_rate`, `crnc_rt`, and tax fields leave lifetime
dividend completeness, after-tax semantics, and the historical conversion basis
unresolved. No DTO automatically imports a ledger event. The `ust21100` contract
uses the web guide's `tp=0/3/4/5` values; `tp=8` support remains unconfirmed. KRX gold
queries belong to the guide's group 08 and are included in that public REST inventory.
Support for a separate undocumented product API remains outside this contract.

| API | Public source | Reviewed contract boundary | Mock evidence |
| --- | --- | --- | --- |
| `ka00001` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka00001) | empty request body; response 1 scalar field | `tests/accounts.rs#mock_ka00001_contract` |
| `ka01690` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka01690) | qry_dt required; response 8 scalar fields + `day_bal_rt` (10 row fields) | `tests/accounts.rs#mock_ka01690_contract` |
| `ka10072` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10072) | stk_cd optional, strt_dt required; response 0 scalar fields + `dt_stk_div_rlzt_pl` (14 row fields) | `tests/accounts.rs#mock_ka10072_contract` |
| `ka10073` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10073) | stk_cd optional, strt_dt required, end_dt required; response 0 scalar fields + `dt_stk_rlzt_pl` (14 row fields) | `tests/accounts.rs#mock_ka10073_contract` |
| `ka10074` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10074) | strt_dt required, end_dt required; response 5 scalar fields + `dt_rlzt_pl` (6 row fields) | `tests/accounts.rs#mock_ka10074_contract` |
| `ka10075` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10075) | all_stk_tp required, trde_tp required, stk_cd optional, stex_tp required; response 0 scalar fields + `oso` (30 row fields) | `tests/accounts.rs#mock_ka10075_contract` |
| `ka10076` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10076) | stk_cd optional, qry_tp required, sell_tp required, ord_no optional, stex_tp required; response 0 scalar fields + `cntr` (19 row fields) | `tests/accounts.rs#mock_ka10076_contract` |
| `ka10077` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10077) | stk_cd required; response 1 scalar field + `tdy_rlzt_pl_dtl` (9 row fields) | `tests/accounts.rs#mock_ka10077_contract` |
| `ka10085` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10085) | stex_tp required; response 0 scalar fields + `acnt_prft_rt` (17 row fields) | `tests/accounts.rs#mock_ka10085_contract` |
| `ka10088` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10088) | ord_no required; response 0 scalar fields + `osop` (14 row fields) | `tests/accounts.rs#mock_ka10088_contract` |
| `ka10170` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/ka10170) | base_dt optional, ottks_tp required, ch_crd_tp required; response 6 scalar fields + `tdy_trde_diary` (11 row fields) | `tests/accounts.rs#mock_ka10170_contract` |
| `kt00001` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00001) | qry_tp required; response 69 scalar fields + `stk_entr_prst` (14 row fields) | `tests/accounts.rs#mock_kt00001_contract` |
| `kt00002` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00002) | start_dt required, end_dt required; response 0 scalar fields + `daly_prsm_dpst_aset_amt_prst` (8 row fields) | `tests/accounts.rs#mock_kt00002_contract` |
| `kt00003` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00003) | qry_tp required; response 1 scalar field | `tests/accounts.rs#mock_kt00003_contract` |
| `kt00004` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00004) | qry_tp required, dmst_stex_tp required; response 18 scalar fields + `stk_acnt_evlt_prst` (15 row fields) | `tests/accounts.rs#mock_kt00004_contract` |
| `kt00005` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00005) | dmst_stex_tp required; response 30 scalar fields + `stk_cntr_remn` (13 row fields) | `tests/accounts.rs#mock_kt00005_contract` |
| `kt00007` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00007) | ord_dt optional, qry_tp required, stk_bond_tp required, sell_tp required, stk_cd optional, fr_ord_no optional, dmst_stex_tp required; response 0 scalar fields + `acnt_ord_cntr_prps_dtl` (22 row fields) | `tests/accounts.rs#mock_kt00007_contract` |
| `kt00008` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00008) | strt_dcd_seq optional; response 4 scalar fields + `acnt_nxdy_setl_frcs_prps_array` (15 row fields) | `tests/accounts.rs#mock_kt00008_contract` |
| `kt00009` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00009) | ord_dt optional, stk_bond_tp required, mrkt_tp required, sell_tp required, qry_tp required, stk_cd optional, fr_ord_no optional, dmst_stex_tp required; response 3 scalar fields + `acnt_ord_cntr_prst_array` (22 row fields) | `tests/accounts.rs#mock_kt00009_contract` |
| `kt00010` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00010) | io_amt optional, stk_cd required, trde_tp required, trde_qty optional, uv required, exp_buy_unp optional; response 28 scalar fields | `tests/accounts.rs#mock_kt00010_contract` |
| `kt00011` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00011) | stk_cd required, uv optional; response 36 scalar fields | `tests/accounts.rs#mock_kt00011_contract` |
| `kt00012` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00012) | stk_cd required, uv optional; response 27 scalar fields | `tests/accounts.rs#mock_kt00012_contract` |
| `kt00013` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00013) | empty request body; response 50 scalar fields | `tests/accounts.rs#mock_kt00013_contract` |
| `kt00016` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00016) | fr_dt required, to_dt required; response 39 scalar fields | `tests/accounts.rs#mock_kt00016_contract` |
| `kt00017` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00017) | empty request body; response 25 scalar fields | `tests/accounts.rs#mock_kt00017_contract` |
| `kt00018` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00018) | qry_tp required, dmst_stex_tp required; response 8 scalar fields + `acnt_evlt_remn_indv_tot` (23 row fields) | `tests/accounts.rs#mock_kt00018_contract` |
| `kt50020` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50020) | empty request body; response 8 scalar fields + `gold_acnt_evlt_prst` (16 row fields) | `tests/accounts.rs#mock_kt50020_contract` |
| `kt50021` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50021) | empty request body; response 14 scalar fields | `tests/accounts.rs#mock_kt50021_contract` |
| `kt50030` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50030) | ord_dt required, qry_tp optional, mrkt_deal_tp required, stk_bond_tp required, slby_tp required, stk_cd optional, fr_ord_no optional, dmst_stex_tp optional; response 0 scalar fields + `acnt_ord_cntr_prst` (22 row fields) | `tests/accounts.rs#mock_kt50030_contract` |
| `kt50031` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50031) | ord_dt optional, qry_tp required, stk_bond_tp required, sell_tp required, stk_cd optional, fr_ord_no optional, dmst_stex_tp required; response 0 scalar fields + `acnt_ord_cntr_prps_dtl` (22 row fields) | `tests/accounts.rs#mock_kt50031_contract` |
| `kt50032` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50032) | strt_dt optional, end_dt optional, tp optional, stk_cd optional; response 1 scalar field + `gold_trde_hist` (24 row fields) | `tests/accounts.rs#mock_kt50032_contract` |
| `kt50075` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/08/kt50075) | ord_dt required, qry_tp optional, mrkt_deal_tp required, stk_bond_tp required, sell_tp required, stk_cd optional, fr_ord_no optional, dmst_stex_tp optional; response 0 scalar fields + `acnt_ord_oso_prst` (22 row fields) | `tests/accounts.rs#mock_kt50075_contract` |
| `usa21670` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21670) | from required, to required; response 0 scalar fields + `result_list` (16 row fields) | `tests/accounts.rs#mock_usa21670_contract` |
| `usa21680` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21680) | from required, to required; response 0 scalar fields + `result_list` (16 row fields) | `tests/accounts.rs#mock_usa21680_contract` |
| `usa21690` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21690) | from required, to required; response 0 scalar fields + `result_list` (16 row fields) | `tests/accounts.rs#mock_usa21690_contract` |
| `usa21730` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21730) | from required, to required, stex_tp optional, stk_cd required; response 0 scalar fields + `result_list` (19 row fields) | `tests/accounts.rs#mock_usa21730_contract` |
| `usa21731` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21731) | from required, to required, stex_tp optional, stk_cd required; response 0 scalar fields + `result_list` (19 row fields) | `tests/accounts.rs#mock_usa21731_contract` |
| `usa21732` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/usa21732) | from required, to required, stex_tp optional, stk_cd required; response 0 scalar fields + `result_list` (19 row fields) | `tests/accounts.rs#mock_usa21732_contract` |
| `ust21050` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21050) | ord_dt optional, slby_tp optional, stex_tp optional, stk_cd optional; response 0 scalar fields + `result_list` (25 row fields) | `tests/accounts.rs#mock_ust21050_contract` |
| `ust21070` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21070) | stex_tp optional, stk_cd optional; response 13 scalar fields + `result_list` (24 row fields) | `tests/accounts.rs#mock_ust21070_contract` |
| `ust21110` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21110) | empty request body; response 3 scalar fields + `result_list` (10 row fields) | `tests/accounts.rs#mock_ust21110_contract` |
| `ust21111` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21111) | empty request body; response 3 scalar fields | `tests/accounts.rs#mock_ust21111_contract` |
| `ust21120` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21120) | cmsn_incl_tp optional, exrt_tp optional; response 2 scalar fields + `result_list` (7 row fields) | `tests/accounts.rs#mock_ust21120_contract` |
| `ust21121` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21121) | cmsn_incl_tp optional, exrt_tp optional; response 0 scalar fields + `result_list` (10 row fields) | `tests/accounts.rs#mock_ust21121_contract` |
| `ust21131` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21131) | base_dt required; response 0 scalar fields + `result_list` (10 row fields) | `tests/accounts.rs#mock_ust21131_contract` |
| `ust21132` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21132) | base_dt required; response 6 scalar fields + `result_list` (7 row fields) | `tests/accounts.rs#mock_ust21132_contract` |
| `ust21150` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21150) | ord_dt optional, query_tp required, slby_tp required, stex_tp optional, stk_cd optional, oppo_trde_tp optional, fr_ord_no optional; response 0 scalar fields + `result_list` (28 row fields) | `tests/accounts.rs#mock_ust21150_contract` |
| `ust21160` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21160) | empty request body; response 32 scalar fields | `tests/accounts.rs#mock_ust21160_contract` |
| `ust21170` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21170) | fc_krw_tp required; response 7 scalar fields + `result_list` (20 row fields) | `tests/accounts.rs#mock_ust21170_contract` |
| `ust21180` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21180) | strt_dt required, end_dt required, slby_tp optional, stex_tp optional, stk_cd optional, oppo_trde_tp optional; response 0 scalar fields + `result_list` (24 row fields) | `tests/accounts.rs#mock_ust21180_contract` |
| `ust21510` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21510) | slby_tp optional, stex_tp optional, stk_cd optional; response 0 scalar fields + `result_list` (23 row fields) | `tests/accounts.rs#mock_ust21510_contract` |
| `ust21530` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21530) | strt_dt optional, end_dt optional, fc_krw_tp optional; response 6 scalar fields + `result_list` (18 row fields) | `tests/accounts.rs#mock_ust21530_contract` |
| `ust21610` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21610) | base_dt optional, qry_tp required, fc_krw_tp required; response 7 scalar fields + `result_list` (14 row fields) | `tests/accounts.rs#mock_ust21610_contract` |
| `ust21620` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21620) | stex_tp optional, stk_cd optional, fc_krw_tp required; response 0 scalar fields + `result_list` (13 row fields) | `tests/accounts.rs#mock_ust21620_contract` |
| `ust21630` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21630) | stex_tp optional, stk_cd optional, fc_krw_tp required; response 2 scalar fields + `result_list` (13 row fields) | `tests/accounts.rs#mock_ust21630_contract` |
| `ust21640` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21640) | stex_tp optional, stk_cd optional, cntr_dt required, fc_krw_tp required; response 2 scalar fields + `result_list` (13 row fields) | `tests/accounts.rs#mock_ust21640_contract` |
| `ust21650` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21650) | fr_dt required, to_dt required; response 27 scalar fields | `tests/accounts.rs#mock_ust21650_contract` |
| `ust21660` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21660) | strt_dt optional, end_dt optional; response 5 scalar fields + `result_list` (6 row fields) | `tests/accounts.rs#mock_ust21660_contract` |
| `ust21661` | [guide](https://openapi.kiwoom.com/guide/apiGuideContents/32/ust21661) | strt_dt optional, end_dt optional; response 5 scalar fields + `result_list` (6 row fields) | `tests/accounts.rs#mock_ust21661_contract` |

`tests/accounts.rs` includes one full-field mock per API. The mocks deserialize
every documented response field into its concrete DTO and check that only response
metadata and intentional unknown markers remain in `extra`. Additional tests cover
invalid requests before transport, preservation of omitted response/request fields,
and calendar/range boundaries.
