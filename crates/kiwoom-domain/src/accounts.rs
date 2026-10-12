//! 공개 웹 문서를 근거로 독립 작성한 계좌 조회 계약(2026-10-12).
//! 문자열의 부호·패딩·빈 값을 보존하며 세금, 배당 이력의 완전성이나 과거 환율의
//! 의미를 확정하지 않습니다. 계약 근거는 `docs/contracts/accounts.md`를 참고하세요.
//!
//! 국내·미국 계좌 구현은 비공개 하위 모듈이 맡고, 공개 DTO는 기존 `accounts` 경로로
//! 제공합니다. 거래 이력 `kt00015`·`ust21100`의 요청·응답도 계좌 계약에 둡니다.
//! 요청 검증은 각 API 계약에, 공통 필드·기간 검사는 내부 검증 모듈에 둡니다.

mod domestic;
mod transactions;
mod us;

pub use domestic::{
    Ka00001Request, Ka00001Response, Ka01690Request, Ka01690Response, Ka01690Row, Ka10072Request,
    Ka10072Response, Ka10072Row, Ka10073Request, Ka10073Response, Ka10073Row, Ka10074Request,
    Ka10074Response, Ka10074Row, Ka10075Request, Ka10075Response, Ka10075Row, Ka10076Request,
    Ka10076Response, Ka10076Row, Ka10077Request, Ka10077Response, Ka10077Row, Ka10085Request,
    Ka10085Response, Ka10085Row, Ka10088Request, Ka10088Response, Ka10088Row, Ka10170Request,
    Ka10170Response, Ka10170Row, Kt00001Request, Kt00001Response, Kt00001Row, Kt00002Request,
    Kt00002Response, Kt00002Row, Kt00003Request, Kt00003Response, Kt00004Request, Kt00004Response,
    Kt00004Row, Kt00005Request, Kt00005Response, Kt00005Row, Kt00007Request, Kt00007Response,
    Kt00007Row, Kt00008Request, Kt00008Response, Kt00008Row, Kt00009Request, Kt00009Response,
    Kt00009Row, Kt00010Request, Kt00010Response, Kt00011Request, Kt00011Response, Kt00012Request,
    Kt00012Response, Kt00013Request, Kt00013Response, Kt00016Request, Kt00016Response,
    Kt00017Request, Kt00017Response, Kt00018Request, Kt00018Response, Kt00018Row, Kt50020Request,
    Kt50020Response, Kt50020Row, Kt50021Request, Kt50021Response, Kt50030Request, Kt50030Response,
    Kt50030Row, Kt50031Request, Kt50031Response, Kt50031Row, Kt50032Request, Kt50032Response,
    Kt50032Row, Kt50075Request, Kt50075Response, Kt50075Row,
};

pub use us::{
    Usa21670Request, Usa21670Response, Usa21670Row, Usa21680Request, Usa21680Response, Usa21680Row,
    Usa21690Request, Usa21690Response, Usa21690Row, Usa21730Request, Usa21730Response, Usa21730Row,
    Usa21731Request, Usa21731Response, Usa21731Row, Usa21732Request, Usa21732Response, Usa21732Row,
    Ust21050Request, Ust21050Response, Ust21050Row, Ust21070Request, Ust21070Response, Ust21070Row,
    Ust21110Request, Ust21110Response, Ust21110Row, Ust21111Request, Ust21111Response,
    Ust21120Request, Ust21120Response, Ust21120Row, Ust21121Request, Ust21121Response, Ust21121Row,
    Ust21131Request, Ust21131Response, Ust21131Row, Ust21132Request, Ust21132Response, Ust21132Row,
    Ust21150Request, Ust21150Response, Ust21150Row, Ust21160Request, Ust21160Response,
    Ust21170Request, Ust21170Response, Ust21170Row, Ust21180Request, Ust21180Response, Ust21180Row,
    Ust21510Request, Ust21510Response, Ust21510Row, Ust21530Request, Ust21530Response, Ust21530Row,
    Ust21610Request, Ust21610Response, Ust21610Row, Ust21620Request, Ust21620Response, Ust21620Row,
    Ust21630Request, Ust21630Response, Ust21630Row, Ust21640Request, Ust21640Response, Ust21640Row,
    Ust21650Request, Ust21650Response, Ust21660Request, Ust21660Response, Ust21660Row,
    Ust21661Request, Ust21661Response, Ust21661Row,
};

pub use transactions::{
    DomesticTransaction, DomesticTransactionType, Kt00015Request, Kt00015Response, UsTransaction,
    UsTransactionType, Ust21100Request, Ust21100Response,
};

// 등록된 계좌 API의 ID/path 쌍은 공통 라우트 검증에서 사용한다.
pub(crate) const CONTRACTS: &[(&str, &str)] = &[
    ("ka00001", "/api/dostk/acnt"),
    ("ka01690", "/api/dostk/acnt"),
    ("ka10072", "/api/dostk/acnt"),
    ("ka10073", "/api/dostk/acnt"),
    ("ka10074", "/api/dostk/acnt"),
    ("ka10075", "/api/dostk/acnt"),
    ("ka10076", "/api/dostk/acnt"),
    ("ka10077", "/api/dostk/acnt"),
    ("ka10085", "/api/dostk/acnt"),
    ("ka10088", "/api/dostk/acnt"),
    ("ka10170", "/api/dostk/acnt"),
    ("kt00001", "/api/dostk/acnt"),
    ("kt00002", "/api/dostk/acnt"),
    ("kt00003", "/api/dostk/acnt"),
    ("kt00004", "/api/dostk/acnt"),
    ("kt00005", "/api/dostk/acnt"),
    ("kt00007", "/api/dostk/acnt"),
    ("kt00008", "/api/dostk/acnt"),
    ("kt00009", "/api/dostk/acnt"),
    ("kt00010", "/api/dostk/acnt"),
    ("kt00011", "/api/dostk/acnt"),
    ("kt00012", "/api/dostk/acnt"),
    ("kt00013", "/api/dostk/acnt"),
    ("kt00015", "/api/dostk/acnt"),
    ("kt00016", "/api/dostk/acnt"),
    ("kt00017", "/api/dostk/acnt"),
    ("kt00018", "/api/dostk/acnt"),
    ("kt50020", "/api/dostk/acnt"),
    ("kt50021", "/api/dostk/acnt"),
    ("kt50030", "/api/dostk/acnt"),
    ("kt50031", "/api/dostk/acnt"),
    ("kt50032", "/api/dostk/acnt"),
    ("kt50075", "/api/dostk/acnt"),
    ("usa21670", "/api/us/acnt"),
    ("usa21680", "/api/us/acnt"),
    ("usa21690", "/api/us/acnt"),
    ("usa21730", "/api/us/acnt"),
    ("usa21731", "/api/us/acnt"),
    ("usa21732", "/api/us/acnt"),
    ("ust21050", "/api/us/acnt"),
    ("ust21070", "/api/us/acnt"),
    ("ust21100", "/api/us/acnt"),
    ("ust21110", "/api/us/acnt"),
    ("ust21111", "/api/us/acnt"),
    ("ust21120", "/api/us/acnt"),
    ("ust21121", "/api/us/acnt"),
    ("ust21131", "/api/us/acnt"),
    ("ust21132", "/api/us/acnt"),
    ("ust21150", "/api/us/acnt"),
    ("ust21160", "/api/us/acnt"),
    ("ust21170", "/api/us/acnt"),
    ("ust21180", "/api/us/acnt"),
    ("ust21510", "/api/us/acnt"),
    ("ust21530", "/api/us/acnt"),
    ("ust21610", "/api/us/acnt"),
    ("ust21620", "/api/us/acnt"),
    ("ust21630", "/api/us/acnt"),
    ("ust21640", "/api/us/acnt"),
    ("ust21650", "/api/us/acnt"),
    ("ust21660", "/api/us/acnt"),
    ("ust21661", "/api/us/acnt"),
];

mod validation {
    //! 국내·미국 계좌 요청이 공유하는 필드·조회 기간 검증.

    use crate::ClientError;

    // 검사 순서를 유지해 필수 값, 길이, 선택값, 날짜 오류의 우선순위를 보존한다.
    pub(super) fn validate_field(
        value: &str,
        required: bool,
        max: usize,
        format: &str,
        choices: &[&str],
    ) -> Result<(), ClientError> {
        if required && value.trim().is_empty() {
            return Err(ClientError::InvalidRequest(
                "required account field is empty",
            ));
        }
        if max != 0 && value.len() > max {
            return Err(ClientError::InvalidRequest(
                "account field exceeds documented length",
            ));
        }
        if !choices.is_empty() && !choices.contains(&value) {
            return Err(ClientError::InvalidRequest(
                "unsupported documented account selector",
            ));
        }
        if !format.is_empty() && !value.is_empty() {
            let valid = match format {
                "day" => {
                    value.len() == 8
                        && value.bytes().all(|b| b.is_ascii_digit())
                        && chrono::NaiveDate::parse_from_str(value, "%Y%m%d").is_ok()
                }
                "month" => {
                    value.len() == 6
                        && value.bytes().all(|b| b.is_ascii_digit())
                        && chrono::NaiveDate::parse_from_str(&format!("{value}01"), "%Y%m%d")
                            .is_ok()
                }
                "year" => {
                    value.len() == 4 && value.bytes().all(|b| b.is_ascii_digit()) && value != "0000"
                }
                _ => false,
            };
            if !valid {
                return Err(ClientError::InvalidRequest("invalid account date period"));
            }
        }
        Ok(())
    }

    // 두 값이 모두 있고 비어 있지 않을 때만 시작·종료 순서를 비교한다.
    pub(super) fn validate_range(
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<(), ClientError> {
        if let (Some(a), Some(b)) = (start, end)
            && !a.is_empty()
            && !b.is_empty()
            && a > b
        {
            return Err(ClientError::InvalidRequest("account start after end"));
        }
        Ok(())
    }
}
