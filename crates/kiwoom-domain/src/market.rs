//! 공개 시장 가이드를 바탕으로 독립 작성한 조회 계약.
//!
//! 가격·수량·날짜·환율·단위는 브로커 원본 문자열로 보존한다.
//! 내부 조회 그룹을 나누되 기존 `market` 타입 경로와 읽기 메서드는 유지한다.

use crate::ClientError;

// 응답 문자열과 알 수 없는 필드를 보존하며, 생략값을 빈 문자열로 바꾸지 않는다.
macro_rules! fields {
    ($name:ident {$($field:ident $(=> $wire:literal)?),* $(,)?}) => {
        /// 브로커 원본 문자열과 알 수 없는 필드를 보존하는 응답 또는 응답 행.
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(
                /// 원본 문자열. 생략값과 빈 문자열을 구분한다.
                #[serde(default, skip_serializing_if = "Option::is_none" $(, rename = $wire)?)]
                pub $field: Option<String>,
            )*
            /// 타입에 선언되지 않은 브로커 응답 필드.
            #[serde(flatten)]
            pub extra: std::collections::BTreeMap<String, serde_json::Value>,
        }
    };
}

macro_rules! collection {
    ($name:ident, $key:ident, $row:ty) => {
        /// 생략된 목록과 빈 목록을 구분하고 알 수 없는 필드를 보존하는 응답.
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            /// 원본 응답 목록. 생략값은 빈 목록으로 정규화하지 않는다.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub $key: Option<Vec<$row>>,
            /// 타입에 선언되지 않은 브로커 응답 필드.
            #[serde(flatten)]
            pub extra: std::collections::BTreeMap<String, serde_json::Value>,
        }
    };
}

macro_rules! wire_enum {
    ($name:ident {$($variant:ident => $value:literal),+ $(,)?}) => {
        /// 공개 가이드의 문자열 선택값을 그대로 직렬화하는 구분값.
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $(
                #[doc = concat!("브로커 문자열 `", $value, "`에 대응하는 선택값.")]
                #[serde(rename = $value)]
                $variant,
            )+
        }
    };
}

macro_rules! stock_request {
    ($name:ident, $max:literal) => {
        /// 종목코드의 공백 여부와 문서 길이 상한을 검사하는 요청.
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            /// 그대로 전송할 종목코드.
            pub stk_cd: String,
        }
        impl $name {
            /// 전송 전에 종목코드의 문서 제약을 검사한다.
            pub fn validate(&self) -> Result<(), crate::ClientError> {
                super::bounded(&self.stk_cd, $max)
            }
        }
    };
}

macro_rules! us_quote_request {
    ($name:ident) => {
        /// 거래소와 종목코드를 지정하는 미국주식 조회 요청.
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            /// 요청에 사용할 거래소 구분값.
            pub stex_tp: crate::transactions::UsExchange,
            /// 그대로 전송할 종목코드.
            pub stk_cd: String,
        }
        impl $name {
            /// 전송 전에 종목코드의 공백 여부를 검사한다.
            pub fn validate(&self) -> Result<(), crate::ClientError> {
                super::nonempty(&self.stk_cd)
            }
        }
    };
}

// 공백만 있는 값도 빈 종목코드로 거부한다.
fn nonempty(s: &str) -> Result<(), ClientError> {
    if s.trim().is_empty() {
        return Err(ClientError::InvalidRequest("empty stock code"));
    }
    Ok(())
}

// 기존 계약은 바이트 수가 아닌 문자 수로 길이 상한을 검사한다.
fn bounded(s: &str, max: usize) -> Result<(), ClientError> {
    nonempty(s)?;
    if s.chars().count() > max {
        return Err(ClientError::InvalidRequest(
            "stock code exceeds documented length",
        ));
    }
    Ok(())
}

// 선택 필드의 생략은 허용하되, 값이 있으면 열거된 문자열과 일치해야 한다.
fn choice(v: &Option<String>, allowed: &[&str]) -> Result<(), ClientError> {
    if v.as_ref().is_some_and(|s| !allowed.contains(&s.as_str())) {
        return Err(ClientError::InvalidRequest("undocumented filter value"));
    }
    Ok(())
}

// 생략과 달리 빈 문자열은 정수 조건을 만족하지 않는다.
fn integer_limit(v: &Option<String>, max: u32) -> Result<(), ClientError> {
    if let Some(s) = v
        && (s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || s.parse::<u32>().map_or(true, |n| n > max))
    {
        return Err(ClientError::InvalidRequest(
            "filter integer outside documented range",
        ));
    }
    Ok(())
}

fn required_choice(v: &str, allowed: &[&str]) -> Result<(), ClientError> {
    if !allowed.contains(&v) {
        return Err(ClientError::InvalidRequest("undocumented required filter"));
    }
    Ok(())
}

mod domestic;
mod instruments;
mod us_screening;

pub use instruments::{
    DomesticExecution, DomesticIndustry, DomesticIndustryMarket, DomesticMember, DomesticMemo,
    DomesticMemoInput, DomesticPeriod, DomesticStock, Ka10001Request, Ka10001Response,
    Ka10002Request, Ka10002Response, Ka10003Request, Ka10003Response, Ka10004Request,
    Ka10004Response, Ka10005Request, Ka10005Response, Ka10006Request, Ka10006Response,
    Ka10084Request, Ka10084Response, Ka10099Request, Ka10099Response, Ka10100Request,
    Ka10100Response, Ka10101Request, Ka10101Response, Ka10102Request, Ka10102Response,
    Ka10103Request, Ka10103Response, StockListMarket, UsCategoryFilter, UsDailyPrice, UsDailyQuote,
    UsEtf, UsEtfCategory, UsExchangeFilter, UsExecution, UsIndex, UsIndexFilter, UsIndustry,
    UsMemo, UsMemoInput, UsRegionFilter, UsStock, Usa10098Request, Usa10098Response,
    Usa10099Request, Usa10099Response, Usa10100Request, Usa10100Response, Usa10101Request,
    Usa10101Response, Usa10102Request, Usa10102Response, Usa10103Request, Usa10103Response,
    Usa10104Request, Usa10104Response, Usa10105Request, Usa10105Response, Usa20100Request,
    Usa20100Response, Usa20101Request, Usa20101Response, Usa20150Request, Usa20150Response,
    Usa20151Request, Usa20151Response, Usa20590Request, Usa20590Response,
};

pub use us_screening::{
    UsBookRatioSurge, UsConcentratedSupply, UsGap, UsLowVolume, UsNearHighLow, UsNewHighLow,
    UsPriceBand, UsPriceSurge, UsSeasonalStock, UsSeasonalYear, UsVolumeSurge, UsWatchItem,
    UsWatchNearHighLow, UsWatchPriceSurge, Usa20520Request, Usa20520Response, Usa20521Request,
    Usa20521Response, Usa20570Request, Usa20570Response, Usa20571Request, Usa20571Response,
    Usa20930Request, Usa20930Response, Usa20931Request, Usa20931Response, Usa20932Request,
    Usa20932Response, Usa20970Request, Usa20970Response, Usa20971Request, Usa20971Response,
    Usa20972Request, Usa20972Response, Usa23400Request, Usa23400Response, Usa23401Request,
    Usa23401Response, Usa23402Request, Usa23402Response, Usa24100Request, Usa24100Response,
    Usa24101Request, Usa24101Response, Usa24140Request, Usa24140Response, Usa24141Request,
    Usa24141Response, Usa24210Request, Usa24210Response, Usa24211Request, Usa24211Response,
    Usa24220Request, Usa24220Response, Usa24221Request, Usa24221Response, Usa26410Request,
    Usa26410Response, Usa26411Request, Usa26411Response, Usa26412Request, Usa26412Response,
    Usa26413Request, Usa26413Response, Usa26414Request, Usa26414Response,
};

pub use domestic::{
    DomesticConcentratedSupply, DomesticCreditAvailable, DomesticCreditTrend,
    DomesticDailyStockPrice, DomesticDailyTrade, DomesticExecutionStrengthDate,
    DomesticExecutionStrengthTime, DomesticGoldBook, DomesticGoldDaily, DomesticGoldExecution,
    DomesticGoldExpected, DomesticInquiryRank, DomesticInstitutionalStock,
    DomesticInstitutionalTrend, DomesticIntradayInvestor, DomesticInvestorAfterClose,
    DomesticInvestorDailyStock, DomesticInvestorStock, DomesticInvestorTotals, DomesticLimitPrice,
    DomesticMemberMoment, DomesticMemberStockTrend, DomesticMemberSupply, DomesticNearHighLow,
    DomesticNewHighLow, DomesticPriceSurge, DomesticProgramArbitrageBalance,
    DomesticProgramCumulative, DomesticProgramDailyStock, DomesticProgramRank,
    DomesticProgramStock, DomesticProgramTimeStock, DomesticProgramTrend, DomesticRelativeOpen,
    DomesticRightsQuote, DomesticSpecifiedStock, DomesticTodayPreviousExecution,
    DomesticValuationRank, DomesticViActivation, DomesticVolumeUpdate, Ka00198Request,
    Ka00198Response, Ka10007Request, Ka10007Response, Ka10011Request, Ka10011Response,
    Ka10013Request, Ka10013Response, Ka10015Request, Ka10015Response, Ka10016Request,
    Ka10016Response, Ka10017Request, Ka10017Response, Ka10018Request, Ka10018Response,
    Ka10019Request, Ka10019Response, Ka10024Request, Ka10024Response, Ka10025Request,
    Ka10025Response, Ka10026Request, Ka10026Response, Ka10028Request, Ka10028Response,
    Ka10043Request, Ka10043Response, Ka10044Request, Ka10044Response, Ka10045Request,
    Ka10045Response, Ka10046Request, Ka10046Response, Ka10047Request, Ka10047Response,
    Ka10052Request, Ka10052Response, Ka10054Request, Ka10054Response, Ka10055Request,
    Ka10055Response, Ka10058Request, Ka10058Response, Ka10059Request, Ka10059Response,
    Ka10061Request, Ka10061Response, Ka10063Request, Ka10063Response, Ka10066Request,
    Ka10066Response, Ka10078Request, Ka10078Response, Ka10086Request, Ka10086Response,
    Ka10095Request, Ka10095Response, Ka50010Request, Ka50010Response, Ka50012Request,
    Ka50012Response, Ka50087Request, Ka50087Response, Ka50100Request, Ka50100Response,
    Ka50101Request, Ka50101Response, Ka90003Request, Ka90003Response, Ka90004Request,
    Ka90004Response, Ka90005Request, Ka90005Response, Ka90006Request, Ka90006Response,
    Ka90007Request, Ka90007Response, Ka90008Request, Ka90008Response, Ka90010Request,
    Ka90010Response, Ka90013Request, Ka90013Response, Kt20016Request, Kt20016Response,
    Kt20017Request, Kt20017Response,
};

/// 시장 조회 API의 식별자와 정확한 HTTP 경로.
pub const CONTRACTS: &[(&str, &str)] = &[
    ("ka10001", "/api/dostk/stkinfo"),
    ("ka10002", "/api/dostk/stkinfo"),
    ("ka10003", "/api/dostk/stkinfo"),
    ("ka10004", "/api/dostk/mrkcond"),
    ("ka10005", "/api/dostk/mrkcond"),
    ("ka10006", "/api/dostk/mrkcond"),
    ("ka10099", "/api/dostk/stkinfo"),
    ("ka10100", "/api/dostk/stkinfo"),
    ("ka10102", "/api/dostk/stkinfo"),
    ("ka10103", "/api/dostk/stkinfo"),
    ("usa10098", "/api/us/stkinfo"),
    ("usa10099", "/api/us/stkinfo"),
    ("usa10100", "/api/us/stkinfo"),
    ("usa10101", "/api/us/stkinfo"),
    ("usa10102", "/api/us/stkinfo"),
    ("usa10103", "/api/us/stkinfo"),
    ("usa10104", "/api/us/stkinfo"),
    ("usa10105", "/api/us/stkinfo"),
    ("usa20100", "/api/us/mrkcond"),
    ("usa20101", "/api/us/mrkcond"),
    ("usa20150", "/api/us/mrkcond"),
    ("usa20151", "/api/us/mrkcond"),
    ("usa20590", "/api/us/mrkcond"),
    ("usa20520", "/api/us/stkinfo"),
    ("usa20521", "/api/us/stkinfo"),
    ("usa20570", "/api/us/stkinfo"),
    ("usa20571", "/api/us/stkinfo"),
    ("usa20930", "/api/us/stkinfo"),
    ("usa20931", "/api/us/stkinfo"),
    ("usa20932", "/api/us/stkinfo"),
    ("usa20970", "/api/us/stkinfo"),
    ("usa20971", "/api/us/stkinfo"),
    ("usa20972", "/api/us/stkinfo"),
    ("usa23400", "/api/us/stkinfo"),
    ("usa23401", "/api/us/stkinfo"),
    ("usa23402", "/api/us/stkinfo"),
    ("usa24100", "/api/us/stkinfo"),
    ("usa24101", "/api/us/stkinfo"),
    ("usa24140", "/api/us/stkinfo"),
    ("usa24141", "/api/us/stkinfo"),
    ("usa24210", "/api/us/stkinfo"),
    ("usa24211", "/api/us/stkinfo"),
    ("usa24220", "/api/us/stkinfo"),
    ("usa24221", "/api/us/stkinfo"),
    ("usa26410", "/api/us/stkinfo"),
    ("usa26411", "/api/us/stkinfo"),
    ("usa26412", "/api/us/stkinfo"),
    ("usa26413", "/api/us/stkinfo"),
    ("usa26414", "/api/us/stkinfo"),
    ("ka00198", "/api/dostk/stkinfo"),
    ("ka10013", "/api/dostk/stkinfo"),
    ("ka10015", "/api/dostk/stkinfo"),
    ("ka10055", "/api/dostk/stkinfo"),
    ("ka10084", "/api/dostk/stkinfo"),
    ("ka90003", "/api/dostk/stkinfo"),
    ("ka90004", "/api/dostk/stkinfo"),
    ("kt20016", "/api/dostk/stkinfo"),
    ("kt20017", "/api/dostk/stkinfo"),
    ("ka10011", "/api/dostk/mrkcond"),
    ("ka10044", "/api/dostk/mrkcond"),
    ("ka10045", "/api/dostk/mrkcond"),
    ("ka10078", "/api/dostk/mrkcond"),
    ("ka10086", "/api/dostk/mrkcond"),
    ("ka10063", "/api/dostk/mrkcond"),
    ("ka10066", "/api/dostk/mrkcond"),
    ("ka50010", "/api/dostk/mrkcond"),
    ("ka50012", "/api/dostk/mrkcond"),
    ("ka50087", "/api/dostk/mrkcond"),
    ("ka50100", "/api/dostk/mrkcond"),
    ("ka50101", "/api/dostk/mrkcond"),
    ("ka90005", "/api/dostk/mrkcond"),
    ("ka90006", "/api/dostk/mrkcond"),
    ("ka90007", "/api/dostk/mrkcond"),
    ("ka90010", "/api/dostk/mrkcond"),
    ("ka90013", "/api/dostk/mrkcond"),
    ("ka10016", "/api/dostk/stkinfo"),
    ("ka10017", "/api/dostk/stkinfo"),
    ("ka10018", "/api/dostk/stkinfo"),
    ("ka10019", "/api/dostk/stkinfo"),
    ("ka10024", "/api/dostk/stkinfo"),
    ("ka10025", "/api/dostk/stkinfo"),
    ("ka10026", "/api/dostk/stkinfo"),
    ("ka10028", "/api/dostk/stkinfo"),
    ("ka10043", "/api/dostk/stkinfo"),
    ("ka10052", "/api/dostk/stkinfo"),
    ("ka10054", "/api/dostk/stkinfo"),
    ("ka10058", "/api/dostk/stkinfo"),
    ("ka10059", "/api/dostk/stkinfo"),
    ("ka10061", "/api/dostk/stkinfo"),
    ("ka10095", "/api/dostk/stkinfo"),
    ("ka10007", "/api/dostk/mrkcond"),
    ("ka10101", "/api/dostk/stkinfo"),
    ("ka10046", "/api/dostk/mrkcond"),
    ("ka10047", "/api/dostk/mrkcond"),
    ("ka90008", "/api/dostk/mrkcond"),
];
