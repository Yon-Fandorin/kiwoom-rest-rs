//! 공개 가이드를 바탕으로 독립 작성한 분석 조회 계약과 타입을 제공합니다.
//! 금액·수량의 원본 문자열을 보존하며 평가나 원장 값을 추론하지 않습니다.

use crate::ClientError;

// 호출 모듈이 검증 함수와 DTO 의존성을 가져와 private 어휘 범위에서 사용합니다.
macro_rules! endpoint {
    (
        $method:ident, $req:ident, $res:ident, $row:ident, $id:literal, $path:literal,
        required { $( $required:ident: ($rmax:expr, $rcodes:expr, $rdate:expr) ),* $(,)? },
        optional { $( $optional:ident: ($omax:expr, $ocodes:expr, $odate:expr) ),* $(,)? },
        $( arrays { $( $array:ident: $arraytype:ty ),* $(,)? } )?
        scalars { $( $scalar:ident $( => $scalar_wire:literal )? ),* $(,)? },
        $list:ident { $( $field:ident $( => $field_wire:literal )? ),* $(,)? }
    ) => {
        #[doc = concat!($id, " 분석 조회 요청. 원본 문자열을 보존합니다.")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $req {
            $(
                pub $required: String,
            )*
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $optional: Option<String>,
            )*
            $(
                $(
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub $array: Option<Vec<$arraytype>>,
                )*
            )?
        }

        impl $req {
            /// 필드 검증 뒤 날짜·순위의 관계를 확인하며 첫 오류를 반환합니다.
            pub fn validate(&self) -> Result<(), ClientError> {
                $(
                    field_guard(
                        $id,
                        stringify!($required),
                        &self.$required,
                        $rmax,
                        $rcodes,
                        $rdate,
                    )?;
                )*
                $(
                    if let Some(value) = &self.$optional {
                        field_guard(
                            $id,
                            stringify!($optional),
                            value,
                            $omax,
                            $ocodes,
                            $odate,
                        )?;
                    }
                )*
                $(
                    $(
                        if let Some(items) = &self.$array {
                            for item in items {
                                item.validate()?;
                            }
                        }
                    )*
                )?

                // Serde로 직렬화한 필드 이름을 기준으로 API별 관계 제약을 검사합니다.
                let value = serde_json::to_value(self)
                    .map_err(|_| ClientError::InvalidRequest("analytics serialization failed"))?;
                if let (Some(start), Some(end)) = (
                    value.get("strt_dt").and_then(serde_json::Value::as_str),
                    value.get("end_dt").and_then(serde_json::Value::as_str),
                ) {
                    if start > end {
                        return Err(ClientError::InvalidRequest("start date after end date"));
                    }
                }
                if let (Some(start), Some(end)) = (
                    value.get("st_dt").and_then(serde_json::Value::as_str),
                    value.get("ed_dt").and_then(serde_json::Value::as_str),
                ) {
                    if start > end {
                        return Err(ClientError::InvalidRequest("start date after end date"));
                    }
                }
                let explicit_dates = ($id == "ka10131"
                    && value.get("dt").and_then(serde_json::Value::as_str) == Some("0"))
                    || ($id == "ka10042"
                        && value.get("qry_dt_tp").and_then(serde_json::Value::as_str) == Some("1"));
                if explicit_dates
                    && (value.get("strt_dt").is_none() || value.get("end_dt").is_none())
                {
                    return Err(ClientError::InvalidRequest("date-based query needs both dates"));
                }
                for name in ["rank_strt", "rank_end"] {
                    if let Some(rank) = value.get(name).and_then(serde_json::Value::as_str) {
                        if rank.parse::<u16>().ok().filter(|rank| *rank <= 100).is_none() {
                            return Err(ClientError::InvalidRequest("rank must be between zero and 100"));
                        }
                    }
                }
                if let (Some(start), Some(end)) = (
                    value.get("rank_strt").and_then(serde_json::Value::as_str),
                    value.get("rank_end").and_then(serde_json::Value::as_str),
                ) {
                    if start.parse::<u16>().unwrap() > end.parse::<u16>().unwrap() {
                        return Err(ClientError::InvalidRequest("rank start after end"));
                    }
                }
                Ok(())
            }
        }

        #[doc = concat!($id, " 분석 응답 행. 알 수 없는 필드는 extra에 보존합니다.")]
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $row {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                $( #[serde(rename = $field_wire)] )?
                pub $field: Option<String>,
            )*
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>,
        }

        #[doc = concat!($id, " 분석 응답. 목록 생략과 빈 배열을 구분합니다.")]
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $res {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                $( #[serde(rename = $scalar_wire)] )?
                pub $scalar: Option<String>,
            )*
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub $list: Option<Vec<$row>>,
            #[serde(flatten)]
            pub extra: BTreeMap<String, serde_json::Value>,
        }

    }
}

fn guard(value: &str, maximum: usize, codes: &[&str], date: bool) -> Result<(), ClientError> {
    if (value.trim().is_empty() && !codes.contains(&""))
        || (codes.is_empty() && value.chars().count() > maximum)
    {
        return Err(ClientError::InvalidRequest(
            "missing or overlong analytics field",
        ));
    }
    if !codes.is_empty() && !codes.contains(&value) {
        return Err(ClientError::InvalidRequest("undocumented analytics code"));
    }
    if date {
        crate::transactions::BrokerDate::new(value)?;
    }
    Ok(())
}

// 국내 순위 표는 길이 6과 거래소 접미사를 함께 명시합니다.
// 해당 API에 한해 6자리 숫자 뒤의 _NX·_AL 표기를 허용합니다.
fn field_guard(
    id: &str,
    field: &str,
    value: &str,
    maximum: usize,
    codes: &[&str],
    date: bool,
) -> Result<(), ClientError> {
    if matches!(id, "ka10038" | "ka10040" | "ka10042" | "ka10053")
        && field == "stk_cd"
        && let Some(base) = value
            .strip_suffix("_NX")
            .or_else(|| value.strip_suffix("_AL"))
        && base.len() == 6
        && base.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Ok(());
    }
    guard(value, maximum, codes, date)
}

mod domestic_rankings;
mod flows;
mod us_rankings;

pub use flows::{
    Ka01300Request, Ka01300Response, Ka01300Row, Ka01301Request, Ka01301Response, Ka01301Row,
    Ka10008Request, Ka10008Response, Ka10008Row, Ka10014Request, Ka10014Response, Ka10014Row,
    Ka10068Request, Ka10068Response, Ka10068Row, Ka10069Request, Ka10069Response, Ka10069Row,
    Ka10131Request, Ka10131Response, Ka10131Row, Ka20068Request, Ka20068Response, Ka20068Row,
    Ka52301Request, Ka52301Response, Ka52301Row, Ka90012Request, Ka90012Response, Ka90012Row,
    Usa20200Request, Usa20200Response, Usa20200Row, Usa20201Request, Usa20201Response, Usa20201Row,
};

pub use domestic_rankings::{
    Ka00190Request, Ka00190Response, Ka00190Row, Ka00196Request, Ka00196Response, Ka00196Row,
    Ka04196Request, Ka04196Response, Ka04196Row, Ka10020Request, Ka10020Response, Ka10020Row,
    Ka10021Request, Ka10021Response, Ka10021Row, Ka10022Request, Ka10022Response, Ka10022Row,
    Ka10023Request, Ka10023Response, Ka10023Row, Ka10027Request, Ka10027Response, Ka10027Row,
    Ka10029Request, Ka10029Response, Ka10029Row, Ka10030Request, Ka10030Response, Ka10030Row,
    Ka10031Request, Ka10031Response, Ka10031Row, Ka10032Request, Ka10032Response, Ka10032Row,
    Ka10033Request, Ka10033Response, Ka10033Row, Ka10034Request, Ka10034Response, Ka10034Row,
    Ka10035Request, Ka10035Response, Ka10035Row, Ka10036Request, Ka10036Response, Ka10036Row,
    Ka10037Request, Ka10037Response, Ka10037Row, Ka10038Request, Ka10038Response, Ka10038Row,
    Ka10039Request, Ka10039Response, Ka10039Row, Ka10040Request, Ka10040Response, Ka10040Row,
    Ka10042Request, Ka10042Response, Ka10042Row, Ka10053Request, Ka10053Response, Ka10053Row,
    Ka10062Request, Ka10062Response, Ka10062Row, Ka10065Request, Ka10065Response, Ka10065Row,
    Ka90009Request, Ka90009Response, Ka90009Row,
};

pub use us_rankings::{
    Usa01980Request, Usa01980Response, Usa01980Row, Usa01990Request, Usa01990Response, Usa01990Row,
    Usa20510Request, Usa20510Response, Usa20510Row, Usa20511Request, Usa20511Response, Usa20511Row,
    Usa20512Request, Usa20512Response, Usa20512Row, Usa20530Request, Usa20530Response, Usa20530Row,
    Usa20531Request, Usa20531Response, Usa20531Row, Usa20540Request, Usa20540Response, Usa20540Row,
    Usa20541Request, Usa20541Response, Usa20541Row, Usa20550Request, Usa20550Response, Usa20550Row,
    Usa20551Request, Usa20551Response, Usa20551Row, Usa20880Request, Usa20880Response, Usa20880Row,
    Usa20881Request, Usa20881Response, Usa20881Row, Usa20910Request, Usa20910Response, Usa20910Row,
    Usa20911Request, Usa20911Response, Usa20911Row, Usa20920Request, Usa20920Response, Usa20920Row,
    Usa20921Request, Usa20921Response, Usa20921Row, Usa20922Request, Usa20922Response, Usa20922Row,
    Usa20940Request, Usa20940Response, Usa20940Row, Usa20941Request, Usa20941Response, Usa20941Row,
    Usa20960Request, Usa20960Response, Usa20960Row, Usa20961Request, Usa20961Response, Usa20961Row,
    Usa24110Request, Usa24110Response, Usa24110Row, Usa24111Request, Usa24111Response, Usa24111Row,
    Usa24120Request, Usa24120Response, Usa24120Row, Usa24121Request, Usa24121Response, Usa24121Row,
    Usa24150Request, Usa24150Response, Usa24150Row, Usa24151Request, Usa24151Response, Usa24151Row,
    Usa24160Request, Usa24160Response, Usa24160Row, Usa24161Request, Usa24161Response, Usa24161Row,
    Usa24162Request, Usa24162Response, Usa24162Row, Usa24200Request, Usa24200Response, Usa24200Row,
    Usa24201Request, Usa24201Response, Usa24201Row, Usa24290Request, Usa24290Response, Usa24290Row,
    Usa24291Request, Usa24291Response, Usa24291Row, WatchlistSymbol,
};

/// 분석 조회가 허용하는 API 식별자와 경로입니다.
pub const CONTRACTS: &[(&str, &str)] = &[
    ("ka10014", "/api/dostk/shsa"),
    ("ka01300", "/api/dostk/watchlist"),
    ("ka01301", "/api/dostk/watchlist"),
    ("ka10008", "/api/dostk/frgnistt"),
    ("ka10131", "/api/dostk/frgnistt"),
    ("ka52301", "/api/dostk/frgnistt"),
    ("ka10068", "/api/dostk/slb"),
    ("ka10069", "/api/dostk/slb"),
    ("ka20068", "/api/dostk/slb"),
    ("ka90012", "/api/dostk/slb"),
    ("usa20200", "/api/us/watchlist"),
    ("usa20201", "/api/us/watchlist"),
    ("ka00190", "/api/dostk/rkinfo"),
    ("ka00196", "/api/dostk/rkinfo"),
    ("ka04196", "/api/dostk/rkinfo"),
    ("ka10020", "/api/dostk/rkinfo"),
    ("ka10021", "/api/dostk/rkinfo"),
    ("ka10022", "/api/dostk/rkinfo"),
    ("ka10023", "/api/dostk/rkinfo"),
    ("ka10027", "/api/dostk/rkinfo"),
    ("ka10029", "/api/dostk/rkinfo"),
    ("ka10030", "/api/dostk/rkinfo"),
    ("ka10031", "/api/dostk/rkinfo"),
    ("ka10032", "/api/dostk/rkinfo"),
    ("ka10033", "/api/dostk/rkinfo"),
    ("ka10034", "/api/dostk/rkinfo"),
    ("ka10035", "/api/dostk/rkinfo"),
    ("ka10036", "/api/dostk/rkinfo"),
    ("ka10037", "/api/dostk/rkinfo"),
    ("ka10038", "/api/dostk/rkinfo"),
    ("ka10039", "/api/dostk/rkinfo"),
    ("ka10040", "/api/dostk/rkinfo"),
    ("ka10042", "/api/dostk/rkinfo"),
    ("ka10053", "/api/dostk/rkinfo"),
    ("ka10062", "/api/dostk/rkinfo"),
    ("ka10065", "/api/dostk/rkinfo"),
    ("ka90009", "/api/dostk/rkinfo"),
    ("usa01980", "/api/us/rkinfo"),
    ("usa01990", "/api/us/rkinfo"),
    ("usa20510", "/api/us/rkinfo"),
    ("usa20511", "/api/us/rkinfo"),
    ("usa20512", "/api/us/rkinfo"),
    ("usa20530", "/api/us/rkinfo"),
    ("usa20531", "/api/us/rkinfo"),
    ("usa20540", "/api/us/rkinfo"),
    ("usa20541", "/api/us/rkinfo"),
    ("usa20550", "/api/us/rkinfo"),
    ("usa20551", "/api/us/rkinfo"),
    ("usa20880", "/api/us/rkinfo"),
    ("usa20881", "/api/us/rkinfo"),
    ("usa20910", "/api/us/rkinfo"),
    ("usa20911", "/api/us/rkinfo"),
    ("usa20920", "/api/us/rkinfo"),
    ("usa20921", "/api/us/rkinfo"),
    ("usa20922", "/api/us/rkinfo"),
    ("usa20940", "/api/us/rkinfo"),
    ("usa20941", "/api/us/rkinfo"),
    ("usa20960", "/api/us/rkinfo"),
    ("usa20961", "/api/us/rkinfo"),
    ("usa24110", "/api/us/rkinfo"),
    ("usa24111", "/api/us/rkinfo"),
    ("usa24120", "/api/us/rkinfo"),
    ("usa24121", "/api/us/rkinfo"),
    ("usa24150", "/api/us/rkinfo"),
    ("usa24151", "/api/us/rkinfo"),
    ("usa24160", "/api/us/rkinfo"),
    ("usa24161", "/api/us/rkinfo"),
    ("usa24162", "/api/us/rkinfo"),
    ("usa24200", "/api/us/rkinfo"),
    ("usa24201", "/api/us/rkinfo"),
    ("usa24290", "/api/us/rkinfo"),
    ("usa24291", "/api/us/rkinfo"),
];
