//! 공개 실시간 FID 매핑과 피드별 값 디코딩을 함께 소유합니다.
//! 알려진 FID의 원문 문자열과 아직 계약에 없는 FID를 보존합니다.

use crate::ClientError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

macro_rules! values {
    ($name:ident {$($field:ident => $fid:literal),+ $(,)?}) => {
        /// 알려진 FID는 원문 문자열로, 나머지 FID는 JSON 값으로 보존하는 피드 계약입니다.
        /// 생략은 `None`이며 빈 문자열·부호·패딩을 보존합니다.
        /// 숫자·가격·수량의 단위나 의미를 변환하지 않습니다.
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[doc = concat!("FID `", $fid, "`의 원문 문자열.")]
                #[serde(rename = $fid, default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,

            )+

            /// 이 계약에 없는 FID와 해당 JSON 값입니다.
            #[serde(flatten)]
            pub unknown_fids: BTreeMap<String, Value>,
        }

        impl $name {
            /// 이 피드에서 타입이 정의된 FID 목록입니다. 선언 순서를 유지합니다.
            pub const FIDS: &[&str] = &[$($fid),+];
        }
    };
}

values!(DomesticOrderValues {
    account_number => "9201",
    order_number => "9203",
    manager_id => "9205",
    instrument_code => "9001",
    order_business_class => "912",
    order_status => "913",
    instrument_name => "302",
    order_quantity => "900",
    order_price => "901",
    unfilled_quantity => "902",
    cumulative_execution_amount => "903",
    original_order_number => "904",
    order_class => "905",
    trade_class => "906",
    sell_buy_class => "907",
    order_execution_time => "908",
    execution_number => "909",
    execution_price => "910",
    execution_quantity => "911",
    current_price => "10",
    best_ask => "27",
    best_bid => "28",
    unit_execution_price => "914",
    unit_execution_quantity => "915",
    commission => "938",
    trade_tax => "939",
    rejection_reason => "919",
    screen_number => "920",
    terminal_number => "921",
    credit_class => "922",
    loan_date => "923",
    exchange_class => "2134",
    exchange_name => "2135",
    sor_flag => "2136"
});

values!(DomesticBalanceValues {
    account_number => "9201",
    instrument_code => "9001",
    credit_class => "917",
    loan_date => "916",
    instrument_name => "302",
    current_price => "10",
    holding_quantity => "930",
    purchase_unit_price => "931",
    daily_cumulative_purchase_amount => "932",
    order_available_quantity => "933",
    daily_net_purchase_quantity => "945",
    sell_buy_class => "946",
    daily_total_sale_profit_loss => "950",
    extra_item_951 => "951",
    best_ask => "27",
    best_bid => "28",
    base_price => "307",
    realized_profit_loss_rate => "8019",
    credit_amount => "957",
    credit_interest => "958",
    maturity_date => "918",
    daily_cash_realized_profit_loss => "990",
    daily_cash_realized_profit_loss_rate => "991",
    daily_credit_realized_profit_loss => "992",
    daily_credit_realized_profit_loss_rate => "993",
    collateral_loan_quantity => "959",
    extra_item_924 => "924"
});

values!(StockQuoteValues {
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    best_ask => "27",
    best_bid => "28",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    open_price => "16",
    high_price => "17",
    low_price => "18",
    previous_change_sign => "25",
    previous_volume_change => "26",
    trade_amount_change => "29",
    previous_volume_ratio => "30",
    turnover_rate => "31",
    trade_cost => "32",
    market_cap_hundred_million => "311",
    upper_limit_time => "567",
    lower_limit_time => "568"
});

values!(StockExecutionValues {
    execution_time => "20",
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    best_ask => "27",
    best_bid => "28",
    signed_execution_quantity => "15",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    open_price => "16",
    high_price => "17",
    low_price => "18",
    previous_change_sign => "25",
    previous_volume_change => "26",
    trade_amount_change => "29",
    previous_volume_ratio => "30",
    turnover_rate => "31",
    trade_cost => "32",
    execution_strength => "228",
    market_cap_hundred_million => "311",
    session_class => "290",
    ko_proximity => "691",
    upper_limit_time => "567",
    lower_limit_time => "568",
    previous_same_time_volume_ratio => "851",
    open_time => "1890",
    high_time => "1891",
    low_time => "1892",
    sell_execution_quantity => "1030",
    buy_execution_quantity => "1031",
    buy_ratio => "1032",
    sell_execution_count => "1071",
    buy_execution_count => "1072",
    instant_trade_amount => "1313",
    single_sell_execution_quantity => "1315",
    single_buy_execution_quantity => "1316",
    net_buy_execution_quantity => "1314",
    cfd_margin => "1497",
    maintenance_margin => "1498",
    daily_average_trade_price => "620",
    cfd_trade_cost => "732",
    stock_loan_trade_cost => "852",
    exchange_class => "9081"
});

values!(BestQuoteValues {
    best_ask => "27",
    best_bid => "28"
});

// 호가 단계의 60개 필드를 명시적으로 정의합니다.
// 국내 0D와 미국 FT 공개 표의 숫자 FID가 같으므로 이 필드만 공유합니다.
macro_rules! order_book {
    ($name:ident {$($field:ident=>$fid:literal),* $(,)?}) => {
        values!($name {
            quote_time => "21",
            ask_price_1 => "41",
            ask_quantity_1 => "61",
            ask_previous_change_1 => "81",
            bid_price_1 => "51",
            bid_quantity_1 => "71",
            bid_previous_change_1 => "91",
            ask_price_2 => "42",
            ask_quantity_2 => "62",
            ask_previous_change_2 => "82",
            bid_price_2 => "52",
            bid_quantity_2 => "72",
            bid_previous_change_2 => "92",
            ask_price_3 => "43",
            ask_quantity_3 => "63",
            ask_previous_change_3 => "83",
            bid_price_3 => "53",
            bid_quantity_3 => "73",
            bid_previous_change_3 => "93",
            ask_price_4 => "44",
            ask_quantity_4 => "64",
            ask_previous_change_4 => "84",
            bid_price_4 => "54",
            bid_quantity_4 => "74",
            bid_previous_change_4 => "94",
            ask_price_5 => "45",
            ask_quantity_5 => "65",
            ask_previous_change_5 => "85",
            bid_price_5 => "55",
            bid_quantity_5 => "75",
            bid_previous_change_5 => "95",
            ask_price_6 => "46",
            ask_quantity_6 => "66",
            ask_previous_change_6 => "86",
            bid_price_6 => "56",
            bid_quantity_6 => "76",
            bid_previous_change_6 => "96",
            ask_price_7 => "47",
            ask_quantity_7 => "67",
            ask_previous_change_7 => "87",
            bid_price_7 => "57",
            bid_quantity_7 => "77",
            bid_previous_change_7 => "97",
            ask_price_8 => "48",
            ask_quantity_8 => "68",
            ask_previous_change_8 => "88",
            bid_price_8 => "58",
            bid_quantity_8 => "78",
            bid_previous_change_8 => "98",
            ask_price_9 => "49",
            ask_quantity_9 => "69",
            ask_previous_change_9 => "89",
            bid_price_9 => "59",
            bid_quantity_9 => "79",
            bid_previous_change_9 => "99",
            ask_price_10 => "50",
            ask_quantity_10 => "70",
            ask_previous_change_10 => "90",
            bid_price_10 => "60",
            bid_quantity_10 => "80",
            bid_previous_change_10 => "100",
            total_ask_quantity => "121",
            total_ask_quantity_change => "122",
            total_bid_quantity => "125",
            total_bid_quantity_change => "126",
            $($field=>$fid,)*
        });
    };
}

order_book!(DomesticOrderBookValues {
    expected_execution_price_23 => "23",
    expected_execution_quantity_24 => "24",
    net_buy_quantity => "128",
    buy_ratio => "129",
    net_sell_quantity => "138",
    sell_ratio => "139",
    expected_previous_close_change => "200",
    expected_previous_close_change_rate => "201",
    expected_previous_close_change_sign => "238",
    expected_execution_price_291 => "291",
    expected_execution_quantity_292 => "292",
    expected_previous_change_sign => "293",
    expected_previous_change => "294",
    expected_previous_change_rate => "295",
    krx_ask_quantity_1 => "6044",
    krx_ask_quantity_2 => "6045",
    krx_ask_quantity_3 => "6046",
    krx_ask_quantity_4 => "6047",
    krx_ask_quantity_5 => "6048",
    krx_ask_quantity_6 => "6049",
    krx_ask_quantity_7 => "6050",
    krx_ask_quantity_8 => "6051",
    krx_ask_quantity_9 => "6052",
    krx_ask_quantity_10 => "6053",
    krx_bid_quantity_1 => "6054",
    krx_bid_quantity_2 => "6055",
    krx_bid_quantity_3 => "6056",
    krx_bid_quantity_4 => "6057",
    krx_bid_quantity_5 => "6058",
    krx_bid_quantity_6 => "6059",
    krx_bid_quantity_7 => "6060",
    krx_bid_quantity_8 => "6061",
    krx_bid_quantity_9 => "6062",
    krx_bid_quantity_10 => "6063",
    krx_total_ask_quantity => "6064",
    krx_total_bid_quantity => "6065",
    nxt_ask_quantity_1 => "6066",
    nxt_ask_quantity_2 => "6067",
    nxt_ask_quantity_3 => "6068",
    nxt_ask_quantity_4 => "6069",
    nxt_ask_quantity_5 => "6070",
    nxt_ask_quantity_6 => "6071",
    nxt_ask_quantity_7 => "6072",
    nxt_ask_quantity_8 => "6073",
    nxt_ask_quantity_9 => "6074",
    nxt_ask_quantity_10 => "6075",
    nxt_bid_quantity_1 => "6076",
    nxt_bid_quantity_2 => "6077",
    nxt_bid_quantity_3 => "6078",
    nxt_bid_quantity_4 => "6079",
    nxt_bid_quantity_5 => "6080",
    nxt_bid_quantity_6 => "6081",
    nxt_bid_quantity_7 => "6082",
    nxt_bid_quantity_8 => "6083",
    nxt_bid_quantity_9 => "6084",
    nxt_bid_quantity_10 => "6085",
    nxt_total_ask_quantity => "6086",
    nxt_total_bid_quantity => "6087",
    krx_midpoint_ask_quantity_change => "6100",
    krx_midpoint_ask_quantity => "6101",
    krx_midpoint_price => "6102",
    krx_midpoint_bid_quantity => "6103",
    krx_midpoint_bid_quantity_change => "6104",
    nxt_midpoint_ask_quantity_change => "6105",
    nxt_midpoint_ask_quantity => "6106",
    nxt_midpoint_price => "6107",
    nxt_midpoint_bid_quantity => "6108",
    nxt_midpoint_bid_quantity_change => "6109",
    krx_midpoint_change => "6110",
    krx_midpoint_change_sign => "6111",
    krx_midpoint_change_rate => "6112",
    nxt_midpoint_change => "6113",
    nxt_midpoint_change_sign => "6114",
    nxt_midpoint_change_rate => "6115"
});

order_book!(UsOrderBookValues {});

values!(AfterHoursQuoteValues {
    quote_time => "21",
    total_ask_quantity => "131",
    total_ask_quantity_change => "132",
    total_bid_quantity => "135",
    total_bid_quantity_change => "136"
});

values!(StockBrokerValues {
    sell_broker_name_1 => "141",
    sell_broker_quantity_1 => "161",
    sell_broker_quantity_change_1 => "166",
    sell_broker_code_1 => "146",
    sell_broker_color_1 => "271",
    buy_broker_name_1 => "151",
    buy_broker_quantity_1 => "171",
    buy_broker_quantity_change_1 => "176",
    buy_broker_code_1 => "156",
    buy_broker_color_1 => "281",
    sell_broker_name_2 => "142",
    sell_broker_quantity_2 => "162",
    sell_broker_quantity_change_2 => "167",
    sell_broker_code_2 => "147",
    sell_broker_color_2 => "272",
    buy_broker_name_2 => "152",
    buy_broker_quantity_2 => "172",
    buy_broker_quantity_change_2 => "177",
    buy_broker_code_2 => "157",
    buy_broker_color_2 => "282",
    sell_broker_name_3 => "143",
    sell_broker_quantity_3 => "163",
    sell_broker_quantity_change_3 => "168",
    sell_broker_code_3 => "148",
    sell_broker_color_3 => "273",
    buy_broker_name_3 => "153",
    buy_broker_quantity_3 => "173",
    buy_broker_quantity_change_3 => "178",
    buy_broker_code_3 => "158",
    buy_broker_color_3 => "283",
    sell_broker_name_4 => "144",
    sell_broker_quantity_4 => "164",
    sell_broker_quantity_change_4 => "169",
    sell_broker_code_4 => "149",
    sell_broker_color_4 => "274",
    buy_broker_name_4 => "154",
    buy_broker_quantity_4 => "174",
    buy_broker_quantity_change_4 => "179",
    buy_broker_code_4 => "159",
    buy_broker_color_4 => "284",
    sell_broker_name_5 => "145",
    sell_broker_quantity_5 => "165",
    sell_broker_quantity_change_5 => "170",
    sell_broker_code_5 => "150",
    sell_broker_color_5 => "275",
    buy_broker_name_5 => "155",
    buy_broker_quantity_5 => "175",
    buy_broker_quantity_change_5 => "180",
    buy_broker_code_5 => "160",
    buy_broker_color_5 => "285",
    foreign_estimated_sell_quantity => "261",
    foreign_estimated_sell_quantity_change => "262",
    foreign_estimated_buy_quantity => "263",
    foreign_estimated_buy_quantity_change => "264",
    foreign_estimated_net_buy_quantity => "267",
    foreign_estimated_net_buy_quantity_change => "268",
    exchange_class => "337"
});

values!(EtfNavValues {
    nav => "36",
    nav_previous_change => "37",
    nav_change_rate => "38",
    tracking_error_rate => "39",
    execution_time => "20",
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    cumulative_volume => "13",
    previous_change_sign => "25",
    nav_index_disparity_rate => "265",
    nav_etf_disparity_rate => "266"
});

values!(ExpectedExecutionValues {
    execution_time => "20",
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    signed_execution_quantity => "15",
    cumulative_volume => "13",
    previous_change_sign => "25"
});

values!(InternationalGoldValues {
    current_price => "10",
    previous_change_sign => "25",
    previous_change => "11",
    change_rate => "12"
});

values!(IndustryIndexValues {
    execution_time => "20",
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    signed_execution_quantity => "15",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    open_price => "16",
    high_price => "17",
    low_price => "18",
    previous_change_sign => "25",
    previous_volume_change => "26"
});

values!(IndustryChangeValues {
    execution_time => "20",
    rising_instrument_count => "252",
    upper_limit_instrument_count => "251",
    unchanged_instrument_count => "253",
    falling_instrument_count => "255",
    lower_limit_instrument_count => "254",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    traded_instrument_count => "256",
    traded_instrument_ratio => "257",
    previous_change_sign => "25"
});

values!(StockInformationValues {
    arbitrary_extension => "297",
    pre_open_arbitrary_extension => "592",
    post_close_arbitrary_extension => "593",
    upper_limit_price => "305",
    lower_limit_price => "306",
    base_price => "307",
    early_termination_elw => "689",
    currency_unit => "594",
    margin_rate_display => "382",
    instrument_information => "370"
});

values!(ElwTheoreticalValues {
    execution_time => "20",
    current_price => "10",
    theoretical_price => "670",
    implied_volatility => "671",
    delta => "672",
    gamma => "673",
    theta => "674",
    vega => "675",
    rho => "676",
    lp_quote_implied_volatility => "706"
});

values!(MarketTimeValues {
    market_operation_class => "215",
    execution_time => "20",
    expected_open_remaining_time => "214"
});

values!(ElwIndicatorValues {
    execution_time => "20",
    parity => "666",
    premium => "1211",
    gearing_ratio => "667",
    break_even_rate => "668",
    capital_support_point => "669"
});

values!(ProgramTradingValues {
    execution_time => "20",
    current_price => "10",
    previous_change_sign => "25",
    previous_change => "11",
    change_rate => "12",
    cumulative_volume => "13",
    sell_quantity => "202",
    sell_amount => "204",
    buy_quantity => "206",
    buy_amount => "208",
    net_buy_quantity => "210",
    net_buy_quantity_change => "211",
    net_buy_amount => "212",
    net_buy_amount_change => "213"
});

values!(VolatilityInterruptionValues {
    instrument_code => "9001",
    instrument_name => "302",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    activation_class => "9068",
    market_class => "9008",
    pre_open_class => "9075",
    activation_price => "1221",
    activation_time => "1223",
    release_time => "1224",
    application_class => "1225",
    static_base_price => "1236",
    dynamic_base_price => "1237",
    static_disparity_rate => "1238",
    dynamic_disparity_rate => "1239",
    activation_price_change_rate => "1489",
    activation_count => "1490",
    activation_direction => "9069",
    extra_item_1279 => "1279"
});

values!(UsOrderConfirmationValues {
    account_number => "9201",
    order_number => "9203",
    instrument_code => "9001",
    order_class => "905",
    sell_buy_class => "907",
    original_order_number => "904",
    order_quantity => "900",
    order_price => "901",
    trade_class => "906",
    order_status => "913",
    order_execution_time => "908",
    order_stop_price => "50810",
    currency_code => "8043",
    reservation_class => "50841",
    financial_country_code => "55190",
    country_name => "1091",
    sell_buy_name => "50072",
    instrument_name => "302",
    trade_class_name => "50073"
});

values!(UsExecutionValues {
    country_name => "1091",
    exchange_code => "8046",
    instrument_code => "9001",
    instrument_name => "302",
    original_order_number => "904",
    order_number => "9203",
    order_class => "905",
    sell_buy_class => "907",
    order_execution_time => "908",
    order_status => "913",
    order_quantity => "900",
    order_price => "901",
    unfilled_quantity => "902",
    execution_number => "909",
    execution_price => "910",
    execution_quantity => "911",
    holding_quantity => "930",
    purchase_unit_price => "931",
    daily_sell_quantity => "934",
    daily_buy_quantity => "936",
    previous_sell_quantity => "8004",
    previous_buy_quantity => "8005",
    profit_loss_amount => "8018",
    profit_loss_rate => "8019",
    currency_code => "8043",
    tax => "8075",
    account_number => "9201",
    commission => "13006",
    sell_buy_name => "50072",
    trade_class_name => "50073",
    realized_profit_loss_purchase_amount => "50724",
    fx_realized_profit_loss_purchase_amount => "50725",
    order_stop_price => "50810",
    reservation_class => "50841",
    fx_realized_profit_loss_amount => "50844",
    financial_country_code => "55190"
});

values!(UsTradePriceValues {
    current_price => "10",
    previous_change => "11",
    change_rate => "12",
    cumulative_volume => "13",
    cumulative_trade_amount => "14",
    execution_quantity => "15",
    open_price => "16",
    high_price => "17",
    low_price => "18",
    time => "20",
    execution_date => "22",
    previous_change_sign => "25",
    best_ask => "27",
    best_bid => "28",
    previous_volume_ratio => "30",
    execution_strength => "228",
    session_class => "290",
    execution_time => "51020"
});

macro_rules! payloads {
    ($(($variant:ident,$id:literal,$values:ident)),+ $(,)?) => {
        /// 타입이 정의된 실시간 피드 ID 목록입니다. 등록 검증에서도 이 목록을 사용합니다.
        pub const CONTRACT_IDS: &[&str] = &[$($id),+];

        /// 등록 요청과 페이로드 디코딩에 사용하는 공개 피드 식별자입니다.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum RealtimeType {
            $(
                #[doc = concat!("공개 실시간 피드 `", $id, "`입니다.")]
                #[serde(rename = $id)]
                $variant

            ),+
        }

        impl RealtimeType {
            /// 증권사 요청·응답에서 사용하는 대소문자 그대로의 피드 ID입니다.
            pub const fn api_id(self) -> &'static str {
                match self {
                    $(Self::$variant => $id),+
                }
            }
        }

        /// 피드 ID로 선택한 FID 계약과 알 수 없는 피드의 원문 값입니다.
        #[derive(Debug, Clone)]
        pub enum RealtimePayload {
            $(
                #[doc = concat!("피드 `", $id, "`의 값입니다. 각 FID의 원문과 알 수 없는 FID를 보존합니다.")]
                $variant(Box<$values>),

            )+
            /// 타입이 정의되지 않았거나 ID가 없는 피드입니다. 값의 JSON 형식을 제한하지 않습니다.
            Unknown {
                /// 전달된 피드 ID입니다. 누락된 경우 `None`입니다.
                api_id: Option<String>,

                /// 타입 변환 없이 보존한 페이로드 JSON 값입니다.
                values: Value,
            },
        }

        // 프레임 소유자는 개별 매핑을 알 필요 없이 이 연산으로 FID 계약을 선택합니다.
        pub(super) fn decode_payload(id: Option<&str>, value: Value) -> Result<RealtimePayload, ClientError> {
            match id {
                $(Some($id) => {
                    if !value.is_object() {
                        return Err(ClientError::Protocol("invalid realtime FID object"));
                    }
                    serde_json::from_value::<$values>(value)
                        .map(|v| RealtimePayload::$variant(Box::new(v)))
                        .map_err(|_| ClientError::Protocol("invalid realtime FID object"))
                },)+
                _ => Ok(RealtimePayload::Unknown { api_id: id.map(str::to_owned), values: value }),
            }
        }
    };
}

payloads!(
    (DomesticOrder, "00", DomesticOrderValues),
    (DomesticBalance, "04", DomesticBalanceValues),
    (StockQuote, "0A", StockQuoteValues),
    (StockExecution, "0B", StockExecutionValues),
    (BestQuote, "0C", BestQuoteValues),
    (DomesticOrderBook, "0D", DomesticOrderBookValues),
    (AfterHoursQuote, "0E", AfterHoursQuoteValues),
    (StockBroker, "0F", StockBrokerValues),
    (EtfNav, "0G", EtfNavValues),
    (ExpectedExecution, "0H", ExpectedExecutionValues),
    (InternationalGold, "0I", InternationalGoldValues),
    (IndustryIndex, "0J", IndustryIndexValues),
    (IndustryChange, "0U", IndustryChangeValues),
    (StockInformation, "0g", StockInformationValues),
    (ElwTheoretical, "0m", ElwTheoreticalValues),
    (MarketTime, "0s", MarketTimeValues),
    (ElwIndicator, "0u", ElwIndicatorValues),
    (ProgramTrading, "0w", ProgramTradingValues),
    (VolatilityInterruption, "1h", VolatilityInterruptionValues),
    (UsOrderConfirmation, "F4", UsOrderConfirmationValues),
    (UsExecution, "F5", UsExecutionValues),
    (UsTradePrice, "FE", UsTradePriceValues),
    (UsOrderBook, "FT", UsOrderBookValues),
);
