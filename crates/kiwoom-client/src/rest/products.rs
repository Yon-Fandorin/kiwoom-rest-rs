//! products 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::http::Transport;
use kiwoom_core::{ClientError, Page, Secret};
use kiwoom_domain::products::*;

read!(ka10048, Ka10048Request, Ka10048Response, "/api/dostk/elw");
read!(ka10050, Ka10050Request, Ka10050Response, "/api/dostk/elw");
read!(ka30011, Ka30011Request, Ka30011Response, "/api/dostk/elw");
read!(ka30012, Ka30012Request, Ka30012Response, "/api/dostk/elw");
read!(ka40002, Ka40002Request, Ka40002Response, "/api/dostk/etf");
read!(ka40003, Ka40003Request, Ka40003Response, "/api/dostk/etf");
read!(ka40006, Ka40006Request, Ka40006Response, "/api/dostk/etf");
read!(ka40007, Ka40007Request, Ka40007Response, "/api/dostk/etf");
read!(ka40008, Ka40008Request, Ka40008Response, "/api/dostk/etf");
read!(ka40009, Ka40009Request, Ka40009Response, "/api/dostk/etf");
read!(ka40010, Ka40010Request, Ka40010Response, "/api/dostk/etf");
read!(ka30001, Ka30001Request, Ka30001Response, "/api/dostk/elw");
read!(ka30002, Ka30002Request, Ka30002Response, "/api/dostk/elw");
read!(ka30003, Ka30003Request, Ka30003Response, "/api/dostk/elw");
read!(ka30004, Ka30004Request, Ka30004Response, "/api/dostk/elw");
read!(ka30005, Ka30005Request, Ka30005Response, "/api/dostk/elw");
read!(ka30009, Ka30009Request, Ka30009Response, "/api/dostk/elw");
read!(ka30010, Ka30010Request, Ka30010Response, "/api/dostk/elw");
read!(ka40001, Ka40001Request, Ka40001Response, "/api/dostk/etf");
read!(ka40004, Ka40004Request, Ka40004Response, "/api/dostk/etf");
read!(ka90001, Ka90001Request, Ka90001Response, "/api/dostk/thme");
read!(ka90002, Ka90002Request, Ka90002Response, "/api/dostk/thme");
read!(
    usa24300,
    Usa24300Request,
    Usa24300Response,
    "/api/us/invtinfo"
);
read!(
    ust31300,
    Ust31300Request,
    Ust31300Response,
    "/api/us/exchange"
);
read!(
    ust31301,
    Ust31301Request,
    Ust31301Response,
    "/api/us/exchange"
);
