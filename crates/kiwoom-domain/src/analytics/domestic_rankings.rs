//! 국내 순위 조회는 API별 선택 코드와 거래소 접미사 제약을 유지합니다.
use super::field_guard;
use crate::ClientError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// 순위 요청의 선택 코드는 각 API가 명시한 목록으로 검증합니다.
endpoint!(
    ka00190,
    Ka00190Request,
    Ka00190Response,
    Ka00190Row,
    "ka00190",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (usize::MAX, &["000", "001", "101"], false),
        sort_tp: (usize::MAX, &["1", "2"], false),
        case_pric_tp: (
            usize::MAX,
            &["1", "2", "3", "4", "5", "10", "30", "50"],
            false
        ),
        trde_prica_tp: (
            usize::MAX,
            &["0", "3", "5", "10", "30", "50", "100", "-1"],
            false
        ),
        stk_tp: (
            usize::MAX,
            &[
                "0", "1", "3", "5", "6", "13", "12", "7", "8", "9", "11", "4", "14", "15", "20"
            ],
            false
        ),
        trde_qty_tp: (
            usize::MAX,
            &[
                "0", "10", "50", "100", "150", "200", "300", "500", "1000", "-1"
            ],
            false
        ),
        crd_tp: (
            usize::MAX,
            &["0", "9", "1", "2", "3", "4", "7", "5", "8"],
            false
        ),
        pric_tp: (
            usize::MAX,
            &["0", "1", "8", "2", "3", "4", "10", "5", "-1"],
            false
        ),
        mac_tp: (
            usize::MAX,
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "-1"],
            false
        ),
        stex_tp: (usize::MAX, &["1", "2", "3"], false)
    },
    optional {
        price_cnd_tp: (usize::MAX, &["0", "1"], false),
        price_cnd_min: (usize::MAX, &[], false),
        price_cnd_max: (usize::MAX, &[], false),
        price_cnd_min_sign: (usize::MAX, &["0", "1"], false),
        price_cnd_max_sign: (usize::MAX, &["0", "1"], false),
        mac_cnd_tp: (usize::MAX, &["0", "1"], false),
        mac_cnd_min: (usize::MAX, &[], false),
        mac_cnd_max: (usize::MAX, &[], false),
        mac_cnd_min_sign: (usize::MAX, &["0", "1"], false),
        mac_cnd_max_sign: (usize::MAX, &["0", "1"], false),
        trde_prica_cnd_tp: (usize::MAX, &["0", "1"], false),
        trde_prica_cnd_min: (usize::MAX, &[], false),
        trde_prica_cnd_max: (usize::MAX, &[], false),
        trde_prica_cnd_min_sgin: (usize::MAX, &["0", "1"], false),
        trde_prica_cnd_max_sign: (usize::MAX, &["0", "1"], false),
        trde_qty_cnd_tp: (usize::MAX, &["0", "1"], false),
        trde_qty_cnd_min: (usize::MAX, &[], false),
        trde_qty_cnd_max: (usize::MAX, &[], false),
        trde_qty_cnd_min_sign: (usize::MAX, &["0", "1"], false),
        trde_qty_cnd_max_sign: (usize::MAX, &["0", "1"], false)
    },
    scalars {},
    result_list {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        buy_cnt,
        sel_cnt,
        pre,
        trde_prica
    }
);
endpoint!(
    ka00196,
    Ka00196Request,
    Ka00196Response,
    Ka00196Row,
    "ka00196",
    "/api/dostk/rkinfo",
    required {},
    optional {
        stk_cd: (usize::MAX, &[], false)
    },
    scalars {},
    result_list {
        sel_trde_qty,
        buy_trde_qty,
        tot_trde_qty,
        sel_cnt,
        buy_cnt,
        tot_net_cnt,
        sel_rt,
        buy_rt,
        tot_net_rt,
        tot_avg_pric,
        mont_cntr_amt,
        sel_avg_pric,
        buy_avg_pric
    }
);
endpoint!(
    ka04196,
    Ka04196Request,
    Ka04196Response,
    Ka04196Row,
    "ka04196",
    "/api/dostk/rkinfo",
    required {},
    optional {
        stk_cd: (usize::MAX, &[], false),
        st_dt: (usize::MAX, &[], true),
        ed_dt: (usize::MAX, &[], true),
        qry_tp: (usize::MAX, &["0", "1", "2", "3", "4", "5", "6"], false),
        stex_tp: (usize::MAX, &["1", "2", "3"], false)
    },
    scalars {},
    result_list {
        rank,
        stk_cd,
        stk_nm,
        cur_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        buy_qty,
        sel_qty,
        netprps_qty,
        buy_amt,
        sell_amt,
        netprps_amt,
        acct_qty,
        qty,
        avg_pur_pric,
        prft_rt
    }
);
endpoint!(
    ka10020,
    Ka10020Request,
    Ka10020Response,
    Ka10020Row,
    "ka10020",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["001", "101"], false),
        sort_tp: (1, &["1", "2", "3", "4"], false),
        trde_qty_tp: (4, &["0000", "0010", "0050", "00100"], false),
        stk_cnd: (1, &["0", "1", "5", "6", "7", "8", "9"], false),
        crd_cnd: (1, &["0", "1", "2", "3", "4", "7", "9"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    bid_req_upper {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        trde_qty,
        tot_sel_req,
        tot_buy_req,
        netprps_req,
        buy_rt
    }
);
endpoint!(
    ka10021,
    Ka10021Request,
    Ka10021Response,
    Ka10021Row,
    "ka10021",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["001", "101"], false),
        trde_tp: (1, &["1", "2"], false),
        sort_tp: (1, &["1", "2"], false),
        tm_tp: (2, &[], false),
        trde_qty_tp: (4, &["1", "5", "10", "50", "100"], false),
        stk_cnd: (1, &["0", "1", "5", "6", "7", "8", "9"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    bid_req_sdnin {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        int,
        now,
        sdnin_qty,
        sdnin_rt,
        tot_buy_qty
    }
);
endpoint!(
    ka10022,
    Ka10022Request,
    Ka10022Response,
    Ka10022Row,
    "ka10022",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["001", "101"], false),
        rt_tp: (1, &["1", "2"], false),
        tm_tp: (2, &[], false),
        trde_qty_tp: (1, &["5", "10", "50", "100"], false),
        stk_cnd: (1, &["0", "1", "5", "6", "7", "8", "9"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    req_rt_sdnin {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        int,
        now_rt,
        sdnin_rt,
        tot_sel_req,
        tot_buy_req
    }
);
endpoint!(
    ka10023,
    Ka10023Request,
    Ka10023Response,
    Ka10023Row,
    "ka10023",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        sort_tp: (1, &["1", "2", "3", "4"], false),
        tm_tp: (1, &["1", "2"], false),
        trde_qty_tp: (
            1,
            &["5", "10", "50", "100", "200", "300", "500", "1000"],
            false
        ),
        stk_cnd: (
            1,
            &[
                "0", "1", "3", "11", "4", "5", "6", "13", "12", "7", "8", "9", "17", "14", "18",
                "15", "20"
            ],
            false
        ),
        pric_tp: (1, &["0", "2", "5", "6", "8", "9"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {
        tm: (2, &[], false)
    },
    scalars {},
    trde_qty_sdnin {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        prev_trde_qty,
        now_trde_qty,
        sdnin_qty,
        sdnin_rt
    }
);
endpoint!(
    ka10027,
    Ka10027Request,
    Ka10027Response,
    Ka10027Row,
    "ka10027",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        sort_tp: (1, &["1", "2", "3", "4", "5"], false),
        trde_qty_cnd: (
            5,
            &[
                "0000", "0010", "0050", "0100", "0150", "0200", "0300", "0500", "1000"
            ],
            false
        ),
        stk_cnd: (
            2,
            &[
                "0", "1", "4", "3", "5", "6", "7", "8", "9", "11", "12", "13", "14", "15", "16"
            ],
            false
        ),
        crd_cnd: (1, &["0", "1", "2", "3", "4", "7", "9"], false),
        updown_incls: (2, &["0", "1"], false),
        pric_cnd: (2, &["0", "1", "2", "3", "4", "5", "8", "10"], false),
        trde_prica_cnd: (
            4,
            &[
                "0", "3", "5", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    pred_pre_flu_rt_upper {
        stk_cls,
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_req,
        buy_req,
        now_trde_qty,
        cntr_str,
        cnt
    }
);
endpoint!(
    ka10029,
    Ka10029Request,
    Ka10029Response,
    Ka10029Row,
    "ka10029",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        sort_tp: (1, &["1", "2", "3", "4", "5", "6", "7", "8"], false),
        trde_qty_cnd: (5, &["0", "1", "3", "5", "10", "50", "100"], false),
        stk_cnd: (
            2,
            &[
                "0", "1", "3", "4", "5", "6", "7", "8", "9", "11", "12", "13", "14", "15", "16"
            ],
            false
        ),
        crd_cnd: (1, &["0", "1", "2", "3", "4", "5", "7", "8", "9"], false),
        pric_cnd: (2, &["0", "1", "2", "3", "4", "5", "8", "10"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    exp_cntr_flu_rt_upper {
        stk_cd,
        stk_nm,
        exp_cntr_pric,
        base_pric,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        exp_cntr_qty,
        sel_req,
        sel_bid,
        buy_bid,
        buy_req
    }
);
endpoint!(
    ka10030,
    Ka10030Request,
    Ka10030Response,
    Ka10030Row,
    "ka10030",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        sort_tp: (1, &["1", "2", "3"], false),
        mang_stk_incls: (
            1,
            &[
                "0", "1", "3", "11", "4", "5", "6", "13", "12", "7", "8", "9", "14", "15", "16"
            ],
            false
        ),
        crd_tp: (1, &["0", "9", "1", "2", "3", "4", "8"], false),
        trde_qty_tp: (
            1,
            &["0", "5", "10", "50", "100", "200", "300", "500", "1000"],
            false
        ),
        pric_tp: (
            1,
            &["0", "1", "2", "3", "4", "5", "6", "10", "7", "8", "9"],
            false
        ),
        trde_prica_tp: (
            1,
            &[
                "0", "1", "3", "4", "10", "30", "50", "100", "300", "500", "1000", "3000", "5000"
            ],
            false
        ),
        mrkt_open_tp: (1, &["0", "1", "2", "3"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    tdy_trde_qty_upper {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        trde_qty,
        pred_rt,
        trde_tern_rt,
        trde_amt,
        opmr_trde_qty,
        opmr_pred_rt,
        opmr_trde_rt,
        opmr_trde_amt,
        af_mkrt_trde_qty,
        af_mkrt_pred_rt,
        af_mkrt_trde_rt,
        af_mkrt_trde_amt,
        bf_mkrt_trde_qty,
        bf_mkrt_pred_rt,
        bf_mkrt_trde_rt,
        bf_mkrt_trde_amt
    }
);
endpoint!(
    ka10031,
    Ka10031Request,
    Ka10031Response,
    Ka10031Row,
    "ka10031",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        qry_tp: (1, &["1", "2"], false),
        rank_strt: (3, &[], false),
        rank_end: (3, &[], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    pred_trde_qty_upper {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        trde_qty
    }
);
endpoint!(
    ka10032,
    Ka10032Request,
    Ka10032Response,
    Ka10032Row,
    "ka10032",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        mang_stk_incls: (1, &["0", "1"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    trde_prica_upper {
        stk_cd,
        now_rank,
        pred_rank,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_bid,
        buy_bid,
        now_trde_qty,
        pred_trde_qty,
        trde_prica
    }
);
endpoint!(
    ka10033,
    Ka10033Request,
    Ka10033Response,
    Ka10033Row,
    "ka10033",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        trde_qty_tp: (
            3,
            &["0", "10", "50", "100", "200", "300", "500", "1000"],
            false
        ),
        stk_cnd: (1, &["0", "1", "5", "6", "7", "8", "9"], false),
        updown_incls: (1, &["0", "1"], false),
        crd_cnd: (1, &["0", "1", "2", "3", "4", "7", "9"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    crd_rt_upper {
        stk_infr,
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        crd_rt,
        sel_req,
        buy_req,
        now_trde_qty
    }
);
endpoint!(
    ka10034,
    Ka10034Request,
    Ka10034Response,
    Ka10034Row,
    "ka10034",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        trde_tp: (1, &["1", "2", "3"], false),
        dt: (2, &["0", "1", "5", "10", "20", "60"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    for_dt_trde_upper {
        rank,
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        sel_bid,
        buy_bid,
        trde_qty,
        netprps_qty,
        gain_pos_stkcnt
    }
);
endpoint!(
    ka10035,
    Ka10035Request,
    Ka10035Response,
    Ka10035Row,
    "ka10035",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        trde_tp: (1, &["1", "2"], false),
        base_dt_tp: (1, &["0", "1"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    for_cont_nettrde_upper {
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        dm1,
        dm2,
        dm3,
        tot,
        limit_exh_rt,
        pred_pre_1,
        pred_pre_2,
        pred_pre_3
    }
);
endpoint!(
    ka10036,
    Ka10036Request,
    Ka10036Response,
    Ka10036Row,
    "ka10036",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        dt: (2, &["0", "1", "5", "10", "20", "60"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    for_limit_exh_rt_incrs_upper {
        rank,
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        trde_qty,
        poss_stkcnt,
        gain_pos_stkcnt,
        base_limit_exh_rt,
        limit_exh_rt,
        exh_rt_incrs
    }
);
endpoint!(
    ka10037,
    Ka10037Request,
    Ka10037Response,
    Ka10037Row,
    "ka10037",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        dt: (2, &["0", "1", "5", "10", "20", "60"], false),
        trde_tp: (1, &["1", "2", "3", "4"], false),
        sort_tp: (1, &["1", "2"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    frgn_wicket_trde_upper {
        rank,
        stk_cd,
        stk_nm,
        cur_prc,
        pred_pre_sig,
        pred_pre,
        flu_rt,
        sel_trde_qty,
        buy_trde_qty,
        netprps_trde_qty,
        netprps_prica,
        trde_qty,
        trde_prica
    }
);
endpoint!(
    ka10038,
    Ka10038Request,
    Ka10038Response,
    Ka10038Row,
    "ka10038",
    "/api/dostk/rkinfo",
    required {
        stk_cd: (6, &[], false),
        qry_tp: (1, &["1", "2"], false)
    },
    optional {
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true),
        dt: (2, &["", "1", "4", "9", "19", "39", "59", "119"], false)
    },
    scalars {
        rank_1,
        rank_2,
        rank_3,
        prid_trde_qty
    },
    stk_sec_rank {
        rank,
        mmcm_nm,
        buy_qty,
        sell_qty,
        acc_netprps_qty
    }
);
endpoint!(
    ka10039,
    Ka10039Request,
    Ka10039Response,
    Ka10039Row,
    "ka10039",
    "/api/dostk/rkinfo",
    required {
        mmcm_cd: (3, &[], false),
        trde_qty_tp: (4, &["0", "5", "10", "50", "100", "500", "1000"], false),
        trde_tp: (2, &["1", "2"], false),
        dt: (2, &["0", "1", "5", "10", "60"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {},
    scalars {},
    sec_trde_upper {
        rank,
        stk_cd,
        stk_nm,
        prid_stkpc_flu,
        flu_rt,
        prid_trde_qty,
        netprps,
        buy_trde_qty,
        sel_trde_qty,
        netprps_amt,
        buy_amt,
        sell_amt
    }
);
endpoint!(
    ka10040,
    Ka10040Request,
    Ka10040Response,
    Ka10040Row,
    "ka10040",
    "/api/dostk/rkinfo",
    required {
        stk_cd: (6, &[], false)
    },
    optional {},
    scalars {
        sel_trde_ori_irds_1,
        sel_trde_ori_qty_1,
        sel_trde_ori_1,
        sel_trde_ori_cd_1,
        buy_trde_ori_1,
        buy_trde_ori_cd_1,
        buy_trde_ori_qty_1,
        buy_trde_ori_irds_1,
        sel_trde_ori_irds_2,
        sel_trde_ori_qty_2,
        sel_trde_ori_2,
        sel_trde_ori_cd_2,
        buy_trde_ori_2,
        buy_trde_ori_cd_2,
        buy_trde_ori_qty_2,
        buy_trde_ori_irds_2,
        sel_trde_ori_irds_3,
        sel_trde_ori_qty_3,
        sel_trde_ori_3,
        sel_trde_ori_cd_3,
        buy_trde_ori_3,
        buy_trde_ori_cd_3,
        buy_trde_ori_qty_3,
        buy_trde_ori_irds_3,
        sel_trde_ori_irds_4,
        sel_trde_ori_qty_4,
        sel_trde_ori_4,
        sel_trde_ori_cd_4,
        buy_trde_ori_4,
        buy_trde_ori_cd_4,
        buy_trde_ori_qty_4,
        buy_trde_ori_irds_4,
        sel_trde_ori_irds_5,
        sel_trde_ori_qty_5,
        sel_trde_ori_5,
        sel_trde_ori_cd_5,
        buy_trde_ori_5,
        buy_trde_ori_cd_5,
        buy_trde_ori_qty_5,
        buy_trde_ori_irds_5,
        frgn_sel_prsm_sum_chang,
        frgn_sel_prsm_sum,
        frgn_buy_prsm_sum,
        frgn_buy_prsm_sum_chang
    },
    tdy_main_trde_ori {
        sel_scesn_tm,
        sell_qty,
        sel_upper_scesn_ori,
        buy_scesn_tm,
        buy_qty,
        buy_upper_scesn_ori,
        qry_dt,
        qry_tm
    }
);
endpoint!(
    ka10042,
    Ka10042Request,
    Ka10042Response,
    Ka10042Row,
    "ka10042",
    "/api/dostk/rkinfo",
    required {
        stk_cd: (6, &[], false),
        qry_dt_tp: (1, &["0", "1"], false),
        pot_tp: (1, &["0", "1"], false),
        sort_base: (1, &["1", "2"], false)
    },
    optional {
        strt_dt: (8, &[], true),
        end_dt: (8, &[], true),
        dt: (4, &["5", "10", "20", "40", "60", "120"], false)
    },
    scalars {},
    netprps_trde_ori_rank {
        rank,
        mmcm_cd,
        mmcm_nm
    }
);
endpoint!(
    ka10053,
    Ka10053Request,
    Ka10053Response,
    Ka10053Row,
    "ka10053",
    "/api/dostk/rkinfo",
    required {
        stk_cd: (6, &[], false)
    },
    optional {},
    scalars {},
    tdy_upper_scesn_ori {
        sel_scesn_tm,
        sell_qty,
        sel_upper_scesn_ori,
        buy_scesn_tm,
        buy_qty,
        buy_upper_scesn_ori,
        qry_dt,
        qry_tm
    }
);
endpoint!(
    ka10062,
    Ka10062Request,
    Ka10062Response,
    Ka10062Row,
    "ka10062",
    "/api/dostk/rkinfo",
    required {
        strt_dt: (8, &[], true),
        mrkt_tp: (3, &["000", "001", "101"], false),
        trde_tp: (1, &["1", "2"], false),
        sort_cnd: (1, &["1", "2"], false),
        unit_tp: (1, &["1", "1000"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {
        end_dt: (8, &[], true)
    },
    scalars {},
    eql_nettrde_rank {
        stk_cd,
        rank,
        stk_nm,
        cur_prc,
        pre_sig,
        pred_pre,
        flu_rt,
        acc_trde_qty,
        orgn_nettrde_qty,
        orgn_nettrde_amt,
        orgn_nettrde_avg_pric,
        for_nettrde_qty,
        for_nettrde_amt,
        for_nettrde_avg_pric,
        nettrde_qty,
        nettrde_amt
    }
);
endpoint!(
    ka10065,
    Ka10065Request,
    Ka10065Response,
    Ka10065Row,
    "ka10065",
    "/api/dostk/rkinfo",
    required {
        trde_tp: (1, &["1", "2"], false),
        mrkt_tp: (3, &["000", "001", "101"], false),
        orgn_tp: (
            4,
            &[
                "9000", "9100", "1000", "3000", "5000", "4000", "2000", "6000", "7000", "7100",
                "9999"
            ],
            false
        )
    },
    optional {
        amt_qty_tp: (1, &["1", "2"], false)
    },
    scalars {},
    opmr_invsr_trde_upper {
        stk_cd,
        stk_nm,
        sel_qty,
        buy_qty,
        netslmt
    }
);
endpoint!(
    ka90009,
    Ka90009Request,
    Ka90009Response,
    Ka90009Row,
    "ka90009",
    "/api/dostk/rkinfo",
    required {
        mrkt_tp: (3, &["000", "001", "101"], false),
        amt_qty_tp: (1, &["1", "2"], false),
        qry_dt_tp: (1, &["0", "1"], false),
        stex_tp: (1, &["1", "2", "3"], false)
    },
    optional {
        date: (8, &[], true)
    },
    scalars {},
    frgnr_orgn_trde_upper {
        for_netslmt_stk_cd,
        for_netslmt_stk_nm,
        for_netslmt_amt,
        for_netslmt_qty,
        for_netprps_stk_cd,
        for_netprps_stk_nm,
        for_netprps_amt,
        for_netprps_qty,
        orgn_netslmt_stk_cd,
        orgn_netslmt_stk_nm,
        orgn_netslmt_amt,
        orgn_netslmt_qty,
        orgn_netprps_stk_cd,
        orgn_netprps_stk_nm,
        orgn_netprps_amt,
        orgn_netprps_qty
    }
);
