//! 공매도·외국인·기관·대차 흐름과 관심종목 목록의 조회 계약입니다.
//! 금액·수량은 원본 문자열로 보존하며 평가나 원장 값을 추론하지 않습니다.
use super::field_guard;
use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

endpoint!(
    ka10014,
    Ka10014Request,
    Ka10014Response,
    Ka10014Row,
    "ka10014",
    "/api/dostk/shsa",
    required {
        stk_cd: (20, &[], false),
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true)
    },
    optional {
        tm_tp: (1, &["0", "1"], false)
    },
    scalars {},
    shrts_trnsn {
        dt,
        close_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        shrts_qty,
        ovr_shrts_qty,
        trde_wght,
        shrts_trde_prica,
        shrts_avg_pric
    }
);
endpoint!(
    ka01300,
    Ka01300Request,
    Ka01300Response,
    Ka01300Row,
    "ka01300",
    "/api/dostk/watchlist",
    required {},
    optional {},
    scalars { rtcd },
    nofi { gcod, name }
);
endpoint!(
    ka01301,
    Ka01301Request,
    Ka01301Response,
    Ka01301Row,
    "ka01301",
    "/api/dostk/watchlist",
    required {
        arn_grp_id: (3, &[], false)
    },
    optional {},
    scalars { rtcd },
    nofj { cod2, bgb, bgb_clr }
);
endpoint!(
    ka10008,
    Ka10008Request,
    Ka10008Response,
    Ka10008Row,
    "ka10008",
    "/api/dostk/frgnistt",
    required {
        stk_cd: (20, &[], false)
    },
    optional {},
    scalars {},
    stk_frgnr {
        dt,
        close_pric,
        pred_pre,
        trde_qty,
        chg_qty,
        poss_stkcnt,
        wght,
        gain_pos_stkcnt,
        frgnr_limit,
        frgnr_limit_irds,
        limit_exh_rt
    }
);
endpoint!(
    ka10131,
    Ka10131Request,
    Ka10131Response,
    Ka10131Row,
    "ka10131",
    "/api/dostk/frgnistt",
    required {
        dt: (3, &["1", "3", "5", "10", "20", "120", "0"], false),
        mrkt_tp: (3, &["001", "101"], false),
        netslmt_tp: (1, &["2"], false),
        stk_inds_tp: (1, &["0", "1"], false),
        amt_qty_tp: (1, &["0", "1"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true)
    },
    scalars {},
    orgn_frgnr_cont_trde_prst {
        rank,
        stk_cd,
        stk_nm,
        prid_stkpc_flu_rt,
        orgn_nettrde_amt,
        orgn_nettrde_qty,
        orgn_cont_netprps_dys,
        orgn_cont_netprps_qty,
        orgn_cont_netprps_amt,
        frgnr_nettrde_qty,
        frgnr_nettrde_amt,
        frgnr_cont_netprps_dys,
        frgnr_cont_netprps_qty,
        frgnr_cont_netprps_amt,
        nettrde_qty,
        nettrde_amt,
        tot_cont_netprps_dys,
        tot_cont_nettrde_qty,
        tot_cont_netprps_amt
    }
);
endpoint!(
    ka52301,
    Ka52301Request,
    Ka52301Response,
    Ka52301Row,
    "ka52301",
    "/api/dostk/frgnistt",
    required {},
    optional {},
    scalars {},
    inve_trad_stat {
        all_dfrt_trst_sell_qty,
        sell_qty_irds,
        all_dfrt_trst_sell_amt,
        sell_amt_irds,
        all_dfrt_trst_buy_qty,
        buy_qty_irds,
        all_dfrt_trst_buy_amt,
        buy_amt_irds,
        all_dfrt_trst_netprps_qty,
        netprps_qty_irds,
        all_dfrt_trst_netprps_amt,
        netprps_amt_irds,
        sell_uv,
        buy_uv,
        stk_nm,
        acc_netprps_amt,
        acc_netprps_qty,
        stk_cd
    }
);
endpoint!(
    ka10068,
    Ka10068Request,
    Ka10068Response,
    Ka10068Row,
    "ka10068",
    "/api/dostk/slb",
    required {
        all_tp: (6, &["1"], false)
    },
    optional {
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true)
    },
    scalars {},
    dbrt_trde_trnsn {
        dt,
        dbrt_trde_cntrcnt,
        dbrt_trde_rpy,
        dbrt_trde_irds,
        rmnd,
        remn_amt
    }
);
endpoint!(
    ka10069,
    Ka10069Request,
    Ka10069Response,
    Ka10069Row,
    "ka10069",
    "/api/dostk/slb",
    required {
        strt_dt: (8, &[], true),
        mrkt_tp: (3, &["001", "101"], false)
    },
    optional {
        end_dt: (8, &[], true)
    },
    scalars {
        dbrt_trde_cntrcnt_sum,
        dbrt_trde_rpy_sum,
        rmnd_sum,
        remn_amt_sum,
        dbrt_trde_cntrcnt_rt,
        dbrt_trde_rpy_rt,
        rmnd_rt,
        remn_amt_rt
    },
    dbrt_trde_upper_10stk {
        stk_nm,
        stk_cd,
        dbrt_trde_cntrcnt,
        dbrt_trde_rpy,
        rmnd,
        remn_amt
    }
);
endpoint!(
    ka20068,
    Ka20068Request,
    Ka20068Response,
    Ka20068Row,
    "ka20068",
    "/api/dostk/slb",
    required {
        stk_cd: (6, &[], false)
    },
    optional {
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true),
        all_tp: (1, &["0"], false)
    },
    scalars {},
    dbrt_trde_trnsn {
        dt,
        dbrt_trde_cntrcnt,
        dbrt_trde_rpy,
        dbrt_trde_irds,
        rmnd,
        remn_amt
    }
);
endpoint!(
    ka90012,
    Ka90012Request,
    Ka90012Response,
    Ka90012Row,
    "ka90012",
    "/api/dostk/slb",
    required {
        dt: (8, &[], true),
        mrkt_tp: (3, &["001", "101"], false)
    },
    optional {},
    scalars {},
    dbrt_trde_prps {
        stk_nm,
        stk_cd,
        dbrt_trde_cntrcnt,
        dbrt_trde_rpy,
        rmnd,
        remn_amt
    }
);
endpoint!(
    usa20200,
    Usa20200Request,
    Usa20200Response,
    Usa20200Row,
    "usa20200",
    "/api/us/watchlist",
    required {},
    optional {},
    scalars { rtcd },
    nofi { gcod, name }
);
endpoint!(
    usa20201,
    Usa20201Request,
    Usa20201Response,
    Usa20201Row,
    "usa20201",
    "/api/us/watchlist",
    required {},
    optional {
        arn_grp_id: (3, &[], false)
    },
    scalars { rtcd },
    nofj {
        cod2,
        bgb,
        bgb_clr,
        stex_tp
    }
);
