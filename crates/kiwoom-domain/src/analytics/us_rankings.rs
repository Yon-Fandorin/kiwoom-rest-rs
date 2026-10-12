//! 미국 순위 조회의 선택 코드·중첩 관심종목과 원본 응답을 보존합니다.
use super::{field_guard, guard};
use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 세 미국 관심종목 순위 요청 예제의 중첩 입력입니다.
/// 하위 필드의 길이·선택 코드는 표에 명시되지 않아 비어 있는 값만 거부합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistSymbol {
    pub stex_tp: String,
    pub stk_cd: String,
}
impl WatchlistSymbol {
    /// 거래소와 종목의 비어 있는 입력을 순서대로 거부합니다.
    pub fn validate(&self) -> Result<(), ClientError> {
        guard(&self.stex_tp, usize::MAX, &[], false)?;
        guard(&self.stk_cd, usize::MAX, &[], false)
    }
}
endpoint!(
    usa01980,
    Usa01980Request,
    Usa01980Response,
    Usa01980Row,
    "usa01980",
    "/api/us/rkinfo",
    required {},
    optional {
        svc_type: (4, &["B286", "B281", "B282", "B283", "B284"], false)
    },
    scalars {
        base_date,
        base_time
    },
    result_list {
        rank,
        stk_cd,
        stk_nm,
        sign,
        chg_val,
        curr_pric,
        sign_for_gjga,
        diff_rate_for_gjga,
        sign_for_prev,
        diff_rate_for_prev,
        stex_tp
    }
);
endpoint!(
    usa01990,
    Usa01990Request,
    Usa01990Response,
    Usa01990Row,
    "usa01990",
    "/api/us/rkinfo",
    required {},
    optional {
        dt_unit_tp: (usize::MAX, &["D", "W", "M"], false),
        stk_tp: (usize::MAX, &["A", "S", "E"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        rank_flu_sig,
        rank_flu,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt
    }
);
endpoint!(
    usa20510,
    Usa20510Request,
    Usa20510Response,
    Usa20510Row,
    "usa20510",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        tm: (usize::MAX, &["1", "5", "10", "30"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        stdt_base_pric,
        endt_base_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        stdt_trde_qty,
        endt_trde_qty
    }
);
endpoint!(
    usa20511,
    Usa20511Request,
    Usa20511Response,
    Usa20511Row,
    "usa20511",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        tm: (usize::MAX, &["1", "5", "10", "30"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        stdt_base_pric,
        endt_base_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        stdt_trde_qty,
        endt_trde_qty
    }
);
endpoint!(
    usa20512,
    Usa20512Request,
    Usa20512Response,
    Usa20512Row,
    "usa20512",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        tm: (usize::MAX, &["1", "5", "10", "30"], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
    },
    arrays {
        stk_cd: WatchlistSymbol,
    }
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        stdt_base_pric,
        endt_base_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        stdt_trde_qty,
        endt_trde_qty,
    }
);
endpoint!(
    usa20530,
    Usa20530Request,
    Usa20530Response,
    Usa20530Row,
    "usa20530",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        qry_tp: (usize::MAX, &["0", "1", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pred_rt,
        trde_prica
    }
);
endpoint!(
    usa20531,
    Usa20531Request,
    Usa20531Response,
    Usa20531Row,
    "usa20531",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        qry_tp: (usize::MAX, &["0", "1", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pred_rt,
        trde_prica
    }
);
endpoint!(
    usa20540,
    Usa20540Request,
    Usa20540Response,
    Usa20540Row,
    "usa20540",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        )
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pred_trde_qty,
        trde_prica
    }
);
endpoint!(
    usa20541,
    Usa20541Request,
    Usa20541Response,
    Usa20541Row,
    "usa20541",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        )
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        pred_trde_qty,
        trde_prica
    }
);
endpoint!(
    usa20550,
    Usa20550Request,
    Usa20550Response,
    Usa20550Row,
    "usa20550",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        )
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        mac,
        trde_pre_rt,
        mac_wght,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty
    }
);
endpoint!(
    usa20551,
    Usa20551Request,
    Usa20551Response,
    Usa20551Row,
    "usa20551",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        )
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        mac,
        trde_pre_rt,
        mac_wght,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty
    }
);
endpoint!(
    usa20880,
    Usa20880Request,
    Usa20880Response,
    Usa20880Row,
    "usa20880",
    "/api/us/rkinfo",
    required {},
    optional {
        qry_tp: (usize::MAX, &["1", "2", "3", "4", "5", "6", "7"], false),
        dt_unit_tp: (usize::MAX, &["1", "2", "3", "4", "5", "6", "7", "8"], false)
    },
    scalars {},
    result_list {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        kw_high_rank,
        kw_high_rank_sig,
        kw_high_rank_hl,
        cur_prc,
        flu_rt
    }
);
endpoint!(
    usa20881,
    Usa20881Request,
    Usa20881Response,
    Usa20881Row,
    "usa20881",
    "/api/us/rkinfo",
    required {},
    optional {
        qry_tp: (usize::MAX, &["1", "2", "3", "4", "5", "6", "7"], false),
        dt_unit_tp: (usize::MAX, &["1", "2", "3", "4", "5", "6", "7", "8"], false)
    },
    scalars {},
    result_list {
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        kw_high_rank,
        kw_high_rank_sig,
        kw_high_rank_hl,
        cur_prc,
        flu_rt
    }
);
endpoint!(
    usa20910,
    Usa20910Request,
    Usa20910Response,
    Usa20910Row,
    "usa20910",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        inds_cls_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        sort_tp: (usize::MAX, &["1", "2", "3", "4", "5"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_req,
        buy_req,
        trde_qty,
        cnt
    }
);
endpoint!(
    usa20911,
    Usa20911Request,
    Usa20911Response,
    Usa20911Row,
    "usa20911",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["1", "2", "3", "4", "5"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_req,
        buy_req,
        trde_qty,
        cnt
    }
);
endpoint!(
    usa20920,
    Usa20920Request,
    Usa20920Response,
    Usa20920Row,
    "usa20920",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["1", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        high_pric,
        low_pric,
        open_pric_pre,
        acc_trde_qty,
        cntr_tm,
        open_pric
    }
);
endpoint!(
    usa20921,
    Usa20921Request,
    Usa20921Response,
    Usa20921Row,
    "usa20921",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["1", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        high_pric,
        low_pric,
        open_pric_pre,
        acc_trde_qty,
        cntr_tm,
        open_pric
    }
);
endpoint!(
    usa20922,
    Usa20922Request,
    Usa20922Response,
    Usa20922Row,
    "usa20922",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        sort_tp: (usize::MAX, &["1", "2"], false),
        stk_tp: (usize::MAX, &["0", "1", "2"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
    },
    arrays {
        stk_cd: WatchlistSymbol,
    }
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        high_pric,
        low_pric,
        open_pric_pre,
        acc_trde_qty,
        cntr_tm,
        open_pric,
    }
);
endpoint!(
    usa20940,
    Usa20940Request,
    Usa20940Response,
    Usa20940Row,
    "usa20940",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "3", "4"], false),
        pric_cnd1: (usize::MAX, &[], false),
        pric_cnd2: (usize::MAX, &[], false),
        base_dt: (usize::MAX, &[], true)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        hgst_pric,
        stk_high_dt,
        lwst_pric,
        stk_low_dt,
        acc_flu_rt,
        acc_flu_amt
    }
);
endpoint!(
    usa20941,
    Usa20941Request,
    Usa20941Response,
    Usa20941Row,
    "usa20941",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "3", "4"], false),
        pric_cnd1: (usize::MAX, &[], false),
        pric_cnd2: (usize::MAX, &[], false),
        base_dt: (usize::MAX, &[], true)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        hgst_pric,
        stk_high_dt,
        lwst_pric,
        stk_low_dt,
        acc_flu_rt,
        acc_flu_amt
    }
);
endpoint!(
    usa20960,
    Usa20960Request,
    Usa20960Response,
    Usa20960Row,
    "usa20960",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        qry_tp: (usize::MAX, &["0", "1"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        pred_rt,
        acc_trde_qty
    }
);
endpoint!(
    usa20961,
    Usa20961Request,
    Usa20961Response,
    Usa20961Row,
    "usa20961",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        qry_tp: (usize::MAX, &["0", "1"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        pred_rt,
        acc_trde_qty
    }
);
endpoint!(
    usa24110,
    Usa24110Request,
    Usa24110Response,
    Usa24110Row,
    "usa24110",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1"], false),
        dt_tp: (usize::MAX, &["0", "1"], false),
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        oyr_hgst,
        oyr_lwst,
        week52_hgst_pric => "52wk_hgst_pric",
        week52_lwst_pric => "52wk_lwst_pric",
        hl_pre_sig,
        hl_pre,
        pre_flu_rt,
        base_dt,
        acc_trde_qty,
    }
);
endpoint!(
    usa24111,
    Usa24111Request,
    Usa24111Response,
    Usa24111Row,
    "usa24111",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1"], false),
        dt_tp: (usize::MAX, &["0", "1"], false),
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        oyr_hgst,
        oyr_lwst,
        week52_hgst_pric => "52wk_hgst_pric",
        week52_lwst_pric => "52wk_lwst_pric",
        hl_pre_sig,
        hl_pre,
        pre_flu_rt,
        base_dt,
        acc_trde_qty,
    }
);
endpoint!(
    usa24120,
    Usa24120Request,
    Usa24120Response,
    Usa24120Row,
    "usa24120",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        base_dt: (usize::MAX, &[], true),
        sort_tp: (usize::MAX, &["0", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        base_dt,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        acc_trde_qty,
        trde_prica,
        open_pric,
        high_pric,
        low_pric,
        flu_rt
    }
);
endpoint!(
    usa24121,
    Usa24121Request,
    Usa24121Response,
    Usa24121Row,
    "usa24121",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        base_dt: (usize::MAX, &[], true),
        sort_tp: (usize::MAX, &["0", "2"], false)
    },
    scalars {},
    result_list {
        rank,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        base_dt,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        acc_trde_qty,
        trde_prica,
        open_pric,
        high_pric,
        low_pric,
        flu_rt
    }
);
endpoint!(
    usa24150,
    Usa24150Request,
    Usa24150Response,
    Usa24150Row,
    "usa24150",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_tern_rt,
        day2_avg_tern_rt => "2d_avg_tern_rt",
        day10_avg_tern_rt => "10d_avg_tern_rt",
        day20_avg_tern_rt => "20d_avg_tern_rt",
    }
);
endpoint!(
    usa24151,
    Usa24151Request,
    Usa24151Response,
    Usa24151Row,
    "usa24151",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_tern_rt,
        day2_avg_tern_rt => "2d_avg_tern_rt",
        day10_avg_tern_rt => "10d_avg_tern_rt",
        day20_avg_tern_rt => "20d_avg_tern_rt",
    }
);
endpoint!(
    usa24160,
    Usa24160Request,
    Usa24160Response,
    Usa24160Row,
    "usa24160",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "2", "3", "4", "5"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        base_close_pric,
        base_pre_rt,
        conti_dt
    }
);
endpoint!(
    usa24161,
    Usa24161Request,
    Usa24161Response,
    Usa24161Row,
    "usa24161",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "2", "3", "4", "5"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        base_close_pric,
        base_pre_rt,
        conti_dt
    }
);
endpoint!(
    usa24162,
    Usa24162Request,
    Usa24162Response,
    Usa24162Row,
    "usa24162",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        trde_qty_tp: (usize::MAX, &["0", "10", "15", "20", "30", "50", "100", "300", "500"], false),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"], false),
        trde_prica_cnd: (
            usize::MAX,
            &["0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "2", "3", "4", "5"], false),
    },
    arrays {
        stk_cd: WatchlistSymbol,
    }
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        base_close_pric,
        base_pre_rt,
        conti_dt,
    }
);
endpoint!(
    usa24200,
    Usa24200Request,
    Usa24200Response,
    Usa24200Row,
    "usa24200",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["1", "2", "3", "4"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        acc_trde_qty,
        tot_sel_req,
        tot_buy_req,
        buy_req,
        buy_rt,
        sel_req,
        sel_rt,
        cntr_tm
    }
);
endpoint!(
    usa24201,
    Usa24201Request,
    Usa24201Response,
    Usa24201Row,
    "usa24201",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["1", "2", "3", "4"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        acc_trde_qty,
        tot_sel_req,
        tot_buy_req,
        buy_req,
        buy_rt,
        sel_req,
        sel_rt,
        cntr_tm
    }
);
endpoint!(
    usa24290,
    Usa24290Request,
    Usa24290Response,
    Usa24290Row,
    "usa24290",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        inds_cd: (usize::MAX, &[], false),
        stk_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        inds_cls_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        sort_tp: (usize::MAX, &["0", "1", "2", "3", "4"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        reg_close_pric,
        reg_pre_sig,
        reg_pre,
        dispty_rt,
        cntr_tm,
        cntr_str,
        cnt
    }
);
endpoint!(
    usa24291,
    Usa24291Request,
    Usa24291Response,
    Usa24291Row,
    "usa24291",
    "/api/us/rkinfo",
    required {},
    optional {
        stex_tp: (usize::MAX, &["0", "1", "2", "3"], false),
        etf_cat1: (usize::MAX, &[], false),
        etf_cat2: (usize::MAX, &[], false),
        trde_qty_tp: (
            usize::MAX,
            &["0", "10", "15", "20", "30", "50", "100", "300", "500"],
            false
        ),
        stk_cnd: (usize::MAX, &["0", "1", "2"], false),
        pric_cnd: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            false
        ),
        trde_prica_cnd: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        sort_tp: (usize::MAX, &["0", "1", "2", "3", "4"], false)
    },
    scalars {},
    result_list {
        rank,
        mgn_type,
        stex_tp,
        stk_cd,
        stk_nm,
        stk_enm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        reg_close_pric,
        reg_pre_sig,
        reg_pre,
        dispty_rt,
        cntr_tm,
        cntr_str,
        cnt,
        sel_bid,
        buy_bid,
        sel_req,
        buy_req
    }
);
