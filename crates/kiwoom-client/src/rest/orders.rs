//! orders 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::http::Transport;
use kiwoom_core::{ClientError, Page, Secret};
use kiwoom_domain::orders::*;

mutation!(
    kt10000,
    Kt10000Request,
    DomesticOrderResponse,
    "/api/dostk/ordr"
);
mutation!(
    kt10001,
    Kt10001Request,
    DomesticOrderResponse,
    "/api/dostk/ordr"
);
mutation!(
    kt10002,
    Kt10002Request,
    DomesticModifyResponse,
    "/api/dostk/ordr"
);
mutation!(kt10003, Kt10003Request, CancelResponse, "/api/dostk/ordr");
mutation!(
    kt10006,
    Kt10006Request,
    DomesticOrderResponse,
    "/api/dostk/crdordr"
);
mutation!(
    kt10007,
    Kt10007Request,
    DomesticOrderResponse,
    "/api/dostk/crdordr"
);
mutation!(
    kt10008,
    Kt10008Request,
    DomesticModifyResponse,
    "/api/dostk/crdordr"
);
mutation!(
    kt10009,
    Kt10009Request,
    CreditCancelResponse,
    "/api/dostk/crdordr"
);
mutation!(
    kt50000,
    Kt50000Request,
    GoldOrderResponse,
    "/api/dostk/ordr"
);
mutation!(
    kt50001,
    Kt50001Request,
    GoldOrderResponse,
    "/api/dostk/ordr"
);
mutation!(
    kt50002,
    Kt50002Request,
    GoldModifyResponse,
    "/api/dostk/ordr"
);
mutation!(kt50003, Kt50003Request, CancelResponse, "/api/dostk/ordr");
mutation!(ust20000, Ust20000Request, UsBuyResponse, "/api/us/ordr");
mutation!(ust20001, Ust20001Request, UsSellResponse, "/api/us/ordr");
mutation!(ust20002, Ust20002Request, UsModifyResponse, "/api/us/ordr");
mutation!(ust20003, Ust20003Request, UsCancelResponse, "/api/us/ordr");
mutation!(
    ust21200,
    Ust21200Request,
    ReservationResponse,
    "/api/us/ordr"
);
mutation!(
    ust21201,
    Ust21201Request,
    ReservationResponse,
    "/api/us/ordr"
);
mutation!(
    ust21203,
    Ust21203Request,
    ReservationCancelResponse,
    "/api/us/ordr"
);
read!(ust21204, Ust21204Request, Ust21204Response, "/api/us/ordr");
read!(ust21205, Ust21205Request, Ust21205Response, "/api/us/ordr");
read!(ust31490, Ust31490Request, Ust31490Response, "/api/us/ordr");
mutation!(
    ust31302,
    Ust31302Request,
    Ust31302Response,
    "/api/us/exchange"
);
