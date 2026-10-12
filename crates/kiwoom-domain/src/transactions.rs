//! 여러 조회 계약에서 사용하는 날짜와 증권사 선택값을 정의합니다.
//! 거래 이력 DTO와 페이지 응답의 기존 경로는 같은 타입의 재수출로 유지합니다.

use crate::ClientError;
use serde::{Deserialize, Serialize};

pub use crate::accounts::{
    DomesticTransaction, DomesticTransactionType, Kt00015Request, Kt00015Response, UsTransaction,
    UsTransactionType, Ust21100Request, Ust21100Response,
};
pub use kiwoom_core::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BrokerDate(String);

impl BrokerDate {
    pub fn new(value: &str) -> Result<Self, ClientError> {
        Self::try_from(value.to_owned())
    }
}

impl TryFrom<String> for BrokerDate {
    type Error = ClientError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != 8
            || !s.bytes().all(|b| b.is_ascii_digit())
            || chrono::NaiveDate::parse_from_str(&s, "%Y%m%d").is_err()
        {
            return Err(ClientError::InvalidRequest("date must be YYYYMMDD"));
        }
        Ok(Self(s))
    }
}

impl From<BrokerDate> for String {
    fn from(d: BrokerDate) -> Self {
        d.0
    }
}

pub(crate) fn validate_dates(start: &BrokerDate, end: &BrokerDate) -> Result<(), ClientError> {
    if start.0 > end.0 {
        return Err(ClientError::InvalidRequest("start date after end date"));
    }
    Ok(())
}

macro_rules! wire_enum {
    ($name:ident {$($variant:ident => $value:literal),+ $(,)?}) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum $name {
            $(#[serde(rename = $value)] $variant),+
        }
    }
}

pub(crate) use wire_enum;

wire_enum!(ProductType {
    All => "0",
    DomesticStock => "1",
    Funds => "2",
    OverseasStock => "3",
    FinancialProducts => "4"
});

wire_enum!(DomesticExchange {
    All => "%",
    Krx => "KRX",
    Nxt => "NXT"
});

wire_enum!(UsExchange {
    Nasdaq => "ND",
    Nyse => "NY",
    Amex => "NA"
});

wire_enum!(SortOrder {
    Newest => "1",
    Oldest => "2"
});

wire_enum!(YesNo {
    Yes => "Y",
    No => "N"
});
