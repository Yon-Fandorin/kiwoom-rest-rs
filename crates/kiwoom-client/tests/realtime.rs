use async_trait::async_trait;
use kiwoom_client::{ClientError, Secret, ws::realtime::*, ws::session::*};
use serde_json::{Value, json};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::{Arc, Mutex},
};

struct MockSocket {
    sent: Arc<Mutex<Vec<Value>>>,
    incoming: VecDeque<String>,
}

#[async_trait]
impl WsTransport for MockSocket {
    async fn send(&mut self, text: String) -> Result<(), ClientError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }
    async fn receive(&mut self) -> Result<String, ClientError> {
        self.incoming.pop_front().ok_or(ClientError::Transport)
    }
    async fn close(&mut self) -> Result<(), ClientError> {
        Ok(())
    }
}

fn fixture(fids: &str) -> Value {
    let mut object = serde_json::Map::new();
    for (n, fid) in fids.split_whitespace().enumerate() {
        object.insert(fid.into(), json!(format!("+{n:06}.2500")));
    }
    object.insert("futureFID".into(), json!({"original":"preserved"}));
    Value::Object(object)
}

macro_rules! contract {
    ($test:ident, $api:literal, $variant:ident, $values:ident, $fids:literal, $probe:ident, $probe_fid:literal, $entry:expr, $item:expr) => {
        // 문서의 FID와 원본 실시간 값을 보존하고 등록·해제 요청 형태를 검증한다.
        #[tokio::test]
        async fn $test() {
            let typed_fids: BTreeSet<_> = $values::FIDS.iter().copied().collect();
            // 공개 FIDS의 선언 순서를 고정한다. JSON 객체의 키 순서는 검사하지 않는다.
            let expected_order: Vec<_> = $fids.split_whitespace().collect();
            assert_eq!($values::FIDS, expected_order.as_slice());
            let expected_fids: BTreeSet<_> = expected_order.into_iter().collect();
            assert_eq!(typed_fids, expected_fids);
            assert_eq!(typed_fids.len(), $values::FIDS.len());

            let values = fixture($fids);
            let raw = json!({"trnm":"REAL","data":[{"type":$api,"name":"synthetic feed","item":"synthetic instrument","stexTp":"ND","values":values}],"futureEnvelope":"kept"});
            let raw_text = format!("  {raw}\n");
            let decoded = decode_realtime(&raw_text).unwrap();
            assert_eq!(decoded.raw, raw);
            assert_eq!(decoded.raw_text, raw_text);
            let item = &decoded.data.as_ref().unwrap()[0];
            assert_eq!(item.api_id.as_deref(), Some($api));
            assert_eq!(item.stex_tp.as_deref(), Some("ND"));
            assert_eq!(item.raw, raw["data"][0]);
            match item.values.as_ref().unwrap() {
                RealtimePayload::$variant(v) => {
                    assert_eq!(v.$probe.as_deref(), values[$probe_fid].as_str());
                    assert_eq!(v.unknown_fids.len(), 1);
                    assert_eq!(v.unknown_fids["futureFID"], json!({"original":"preserved"}));
                    assert_eq!(serde_json::to_value(v).unwrap(), values);
                },
                other => panic!("wrong typed dispatch: {other:?}"),
            }

            let sent = Arc::new(Mutex::new(Vec::new()));
            let mock = MockSocket {
                sent: sent.clone(),
                incoming: VecDeque::from([
                    json!({"trnm":"LOGIN","return_code":0}).to_string(),
                    json!({"trnm":"REG","return_code":0,"future_ack":"retained"}).to_string(),
                    json!({"trnm":"REMOVE","return_code":0}).to_string(),
                ])
            };
            let mut session = WsSession::new(mock, SessionOptions::default()).unwrap();
            session
                .login(&Secret::new("offline-realtime-token".into()))
                .await
                .unwrap();
            let register = RealtimeSubscription::new(
                SubscriptionAction::Register,
                "0001",
                RefreshMode::KeepGroup,
                vec![$entry]
            )
            .unwrap();
            assert_eq!(WsRequest::api_id(&register), $api);
            let ack = session.call(&register).await.unwrap();
            assert_eq!(ack.typed.return_code, 0);
            assert_eq!(ack.typed.extra["future_ack"], "retained");
            assert_eq!(ack.raw["future_ack"], "retained");
            let remove = RealtimeSubscription::new(
                SubscriptionAction::Remove,
                "0001",
                RefreshMode::ReplaceGroup,
                vec![$entry]
            )
            .unwrap();
            let ack = session.call(&remove).await.unwrap();
            assert_eq!(ack.typed.trnm, "REMOVE");

            let sent = sent.lock().unwrap();
            assert_eq!(
                sent[1],
                json!({"trnm":"REG","grp_no":"0001","refresh":"1","data":[{"item":$item,"type":[$api]}]})
            );
            assert_eq!(
                sent[2],
                json!({"trnm":"REMOVE","grp_no":"0001","refresh":"0","data":[{"item":$item,"type":[$api]}]})
            );
        }
          };
}

contract!(
    domestic_order_00_contract,
    "00",
    DomesticOrder,
    DomesticOrderValues,
    "9201 9203 9205 9001 912 913 302 900 901 902 903 904 905 906 907 908 909 910 911 10 27 28 914 915 938 939 919 920 921 922 923 2134 2135 2136",
    commission,
    "938",
    SubscriptionEntry::all_instruments(RealtimeType::DomesticOrder).unwrap(),
    json!([""])
);

contract!(
    domestic_balance_04_contract,
    "04",
    DomesticBalance,
    DomesticBalanceValues,
    "9201 9001 917 916 302 10 930 931 932 933 945 946 950 951 27 28 307 8019 957 958 918 990 991 992 993 959 924",
    daily_cumulative_purchase_amount,
    "932",
    SubscriptionEntry::domestic(RealtimeType::DomesticBalance, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    stock_quote_0a_contract,
    "0A",
    StockQuote,
    StockQuoteValues,
    "10 11 12 27 28 13 14 16 17 18 25 26 29 30 31 32 311 567 568",
    market_cap_hundred_million,
    "311",
    SubscriptionEntry::domestic(RealtimeType::StockQuote, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    stock_execution_0b_contract,
    "0B",
    StockExecution,
    StockExecutionValues,
    "20 10 11 12 27 28 15 13 14 16 17 18 25 26 29 30 31 32 228 311 290 691 567 568 851 1890 1891 1892 1030 1031 1032 1071 1072 1313 1315 1316 1314 1497 1498 620 732 852 9081",
    signed_execution_quantity,
    "15",
    SubscriptionEntry::domestic(RealtimeType::StockExecution, Some("500001_NX")).unwrap(),
    json!("500001_NX")
);

contract!(
    best_quote_0c_contract,
    "0C",
    BestQuote,
    BestQuoteValues,
    "27 28",
    best_bid,
    "28",
    SubscriptionEntry::domestic(RealtimeType::BestQuote, Some("500001_AL")).unwrap(),
    json!("500001_AL")
);

// 현재 0D 공개 표의 10단계 순서는 50 70 60 90 80 100이다.
// 아래 기대값은 공개 FIDS의 선언 순서(매도 가격·수량·증감, 매수 가격·수량·증감)를 고정한다.
// 문서 순서나 JSON 객체의 키 순서가 같다는 주장은 하지 않는다.
contract!(
    domestic_order_book_0d_contract,
    "0D",
    DomesticOrderBook,
    DomesticOrderBookValues,
    "21 41 61 81 51 71 91 42 62 82 52 72 92 43 63 83 53 73 93 44 64 84 54 74 94 45 65 85 55 75 95 46 66 86 56 76 96 47 67 87 57 77 97 48 68 88 58 78 98 49 69 89 59 79 99 50 70 90 60 80 100 121 122 125 126 23 24 128 129 138 139 200 201 238 291 292 293 294 295 6044 6045 6046 6047 6048 6049 6050 6051 6052 6053 6054 6055 6056 6057 6058 6059 6060 6061 6062 6063 6064 6065 6066 6067 6068 6069 6070 6071 6072 6073 6074 6075 6076 6077 6078 6079 6080 6081 6082 6083 6084 6085 6086 6087 6100 6101 6102 6103 6104 6105 6106 6107 6108 6109 6110 6111 6112 6113 6114 6115",
    nxt_midpoint_change_rate,
    "6115",
    SubscriptionEntry::domestic(RealtimeType::DomesticOrderBook, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    after_hours_quote_0e_contract,
    "0E",
    AfterHoursQuote,
    AfterHoursQuoteValues,
    "21 131 132 135 136",
    total_bid_quantity_change,
    "136",
    SubscriptionEntry::domestic(RealtimeType::AfterHoursQuote, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    stock_broker_0f_contract,
    "0F",
    StockBroker,
    StockBrokerValues,
    "141 161 166 146 271 151 171 176 156 281 142 162 167 147 272 152 172 177 157 282 143 163 168 148 273 153 173 178 158 283 144 164 169 149 274 154 174 179 159 284 145 165 170 150 275 155 175 180 160 285 261 262 263 264 267 268 337",
    buy_broker_quantity_change_5,
    "180",
    SubscriptionEntry::domestic(RealtimeType::StockBroker, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    etf_nav_upper_0g_contract,
    "0G",
    EtfNav,
    EtfNavValues,
    "36 37 38 39 20 10 11 12 13 25 265 266",
    nav_etf_disparity_rate,
    "266",
    SubscriptionEntry::domestic(RealtimeType::EtfNav, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    expected_execution_0h_contract,
    "0H",
    ExpectedExecution,
    ExpectedExecutionValues,
    "20 10 11 12 15 13 25",
    signed_execution_quantity,
    "15",
    SubscriptionEntry::domestic(RealtimeType::ExpectedExecution, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    international_gold_0i_contract,
    "0I",
    InternationalGold,
    InternationalGoldValues,
    "10 25 11 12",
    change_rate,
    "12",
    SubscriptionEntry::domestic(RealtimeType::InternationalGold, Some("MGD")).unwrap(),
    json!("MGD")
);

contract!(
    industry_index_0j_contract,
    "0J",
    IndustryIndex,
    IndustryIndexValues,
    "20 10 11 12 15 13 14 16 17 18 25 26",
    previous_volume_change,
    "26",
    SubscriptionEntry::domestic(RealtimeType::IndustryIndex, Some("001")).unwrap(),
    json!("001")
);

contract!(
    industry_change_upper_0u_contract,
    "0U",
    IndustryChange,
    IndustryChangeValues,
    "20 252 251 253 255 254 13 14 10 11 12 256 257 25",
    traded_instrument_ratio,
    "257",
    SubscriptionEntry::domestic(RealtimeType::IndustryChange, Some("001")).unwrap(),
    json!("001")
);

contract!(
    stock_information_lower_0g_contract,
    "0g",
    StockInformation,
    StockInformationValues,
    "297 592 593 305 306 307 689 594 382 370",
    margin_rate_display,
    "382",
    SubscriptionEntry::domestic(RealtimeType::StockInformation, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    elw_theoretical_0m_contract,
    "0m",
    ElwTheoretical,
    ElwTheoreticalValues,
    "20 10 670 671 672 673 674 675 676 706",
    rho,
    "676",
    SubscriptionEntry::domestic(RealtimeType::ElwTheoretical, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    market_time_0s_contract,
    "0s",
    MarketTime,
    MarketTimeValues,
    "215 20 214",
    expected_open_remaining_time,
    "214",
    SubscriptionEntry::all_instruments(RealtimeType::MarketTime).unwrap(),
    json!([""])
);

contract!(
    elw_indicator_lower_0u_contract,
    "0u",
    ElwIndicator,
    ElwIndicatorValues,
    "20 666 1211 667 668 669",
    capital_support_point,
    "669",
    SubscriptionEntry::domestic(RealtimeType::ElwIndicator, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    program_trading_0w_contract,
    "0w",
    ProgramTrading,
    ProgramTradingValues,
    "20 10 25 11 12 13 202 204 206 208 210 211 212 213",
    net_buy_amount_change,
    "213",
    SubscriptionEntry::domestic(RealtimeType::ProgramTrading, Some("500001")).unwrap(),
    json!("500001")
);

contract!(
    volatility_interruption_1h_contract,
    "1h",
    VolatilityInterruption,
    VolatilityInterruptionValues,
    "9001 302 13 14 9068 9008 9075 1221 1223 1224 1225 1236 1237 1238 1239 1489 1490 9069 1279",
    activation_direction,
    "9069",
    SubscriptionEntry::all_instruments(RealtimeType::VolatilityInterruption).unwrap(),
    json!([""])
);

contract!(
    us_order_confirmation_f4_contract,
    "F4",
    UsOrderConfirmation,
    UsOrderConfirmationValues,
    "9201 9203 9001 905 907 904 900 901 906 913 908 50810 8043 50841 55190 1091 50072 302 50073",
    order_stop_price,
    "50810",
    SubscriptionEntry::all_instruments(RealtimeType::UsOrderConfirmation).unwrap(),
    json!([""])
);

contract!(
    us_execution_f5_contract,
    "F5",
    UsExecution,
    UsExecutionValues,
    "1091 8046 9001 302 904 9203 905 907 908 913 900 901 902 909 910 911 930 931 934 936 8004 8005 8018 8019 8043 8075 9201 13006 50072 50073 50724 50725 50810 50841 50844 55190",
    fx_realized_profit_loss_amount,
    "50844",
    SubscriptionEntry::all_instruments(RealtimeType::UsExecution).unwrap(),
    json!([""])
);

contract!(
    us_trade_price_fe_contract,
    "FE",
    UsTradePrice,
    UsTradePriceValues,
    "10 11 12 13 14 15 16 17 18 20 22 25 27 28 30 228 290 51020",
    execution_time,
    "51020",
    SubscriptionEntry::us_market(
        RealtimeType::UsTradePrice,
        vec![UsInstrument::new("SYNTH", UsExchange::Nasdaq).unwrap()]
    )
    .unwrap(),
    json!([{"jmcode":"SYNTH","stex_tp":"ND"}])
);

contract!(
    us_order_book_ft_contract,
    "FT",
    UsOrderBook,
    UsOrderBookValues,
    "21 41 61 81 51 71 91 42 62 82 52 72 92 43 63 83 53 73 93 44 64 84 54 74 94 45 65 85 55 75 95 46 66 86 56 76 96 47 67 87 57 77 97 48 68 88 58 78 98 49 69 89 59 79 99 50 70 90 60 80 100 121 122 125 126",
    bid_previous_change_10,
    "100",
    SubscriptionEntry::us_market(
        RealtimeType::UsOrderBook,
        vec![UsInstrument::new("SYNTH", UsExchange::Nyse).unwrap()]
    )
    .unwrap(),
    json!([{"jmcode":"SYNTH","stex_tp":"NY"}])
);

// 대소문자를 구분하는 API ID와 알 수 없는 피드를 보존한다.
#[test]
fn case_sensitive_ids_and_unknown_feed_are_preserved() {
    for (upper, lower) in [("0G", "0g"), ("0U", "0u")] {
        let a: RealtimeType = serde_json::from_value(json!(upper)).unwrap();
        let b: RealtimeType = serde_json::from_value(json!(lower)).unwrap();
        assert_ne!(a, b);
    }
    assert!(serde_json::from_value::<RealtimeType>(json!("0M")).is_err());
    let raw = json!({"trnm":"REAL","data":[{"type":"Future","values":{"odd":[1,2]}}]});
    let d = decode_realtime(&raw.to_string()).unwrap();
    match &d.data.as_ref().unwrap()[0].values {
        Some(RealtimePayload::Unknown { api_id, values }) => {
            assert_eq!(api_id.as_deref(), Some("Future"));
            assert_eq!(*values, json!({"odd":[1,2]}));
        }
        _ => panic!("unknown feed lost"),
    }
}

// 실시간 응답의 누락 목록과 빈 목록을 구분하고 잘못된 타입은 거부한다.
#[test]
fn realtime_missing_empty_and_wrong_type_fail_without_inference() {
    let missing = decode_realtime(r#"{"trnm":"REAL"}"#).unwrap();
    assert!(missing.data.is_none());
    let empty = decode_realtime(r#"{"trnm":"REAL","data":[]}"#).unwrap();
    assert!(empty.data.unwrap().is_empty());
    let fields_missing = decode_realtime(r#"{"trnm":"REAL","data":[{"type":"0B"}]}"#).unwrap();
    assert!(fields_missing.data.as_ref().unwrap()[0].values.is_none());
    for raw in [
        r#"{"trnm":"REAL","data":{}}"#,
        r#"{"trnm":"REAL","data":[{"type":"0B","values":[] }]}"#,
        r#"{"trnm":"REAL","data":[{"type":"0B","values":{"10":12.3}}]}"#,
        r#"{"trnm":"REG","return_code":0}"#,
    ] {
        assert!(decode_realtime(raw).is_err());
    }
    assert!(matches!(
        decode_realtime(r#"{"trnm":"REAL","return_code":42}"#),
        Err(ClientError::Api(42))
    ));
    assert!(decode_realtime(r#"{"trnm":"REAL","return_code":"0"}"#).is_err());
}

// 구독 요청의 항목 형태, 길이, 필수값을 검증한다.
#[test]
fn subscription_shapes_lengths_and_required_values_are_guarded() {
    assert!(SubscriptionEntry::domestic(RealtimeType::UsTradePrice, Some("SYNTH")).is_err());
    assert!(SubscriptionEntry::all_instruments(RealtimeType::StockExecution).is_err());
    assert!(SubscriptionEntry::us_market(RealtimeType::UsTradePrice, vec![]).is_err());
    assert!(SubscriptionEntry::domestic(RealtimeType::ElwIndicator, Some("longer-code")).is_err());
    assert!(SubscriptionEntry::domestic(RealtimeType::IndustryIndex, Some("01")).is_err());
    assert!(SubscriptionEntry::domestic(RealtimeType::InternationalGold, Some("UNKNOWN")).is_err());
    assert!(
        SubscriptionEntry::domestic(RealtimeType::StockInformation, Some(&"x".repeat(101)))
            .is_err()
    );
    assert!(
        RealtimeSubscription::new(
            SubscriptionAction::Register,
            "12345",
            RefreshMode::KeepGroup,
            vec![SubscriptionEntry::all_instruments(RealtimeType::DomesticOrder).unwrap()]
        )
        .is_err()
    );
    assert!(
        RealtimeSubscription::new(
            SubscriptionAction::Register,
            "",
            RefreshMode::KeepGroup,
            vec![]
        )
        .is_err()
    );
    let optional = SubscriptionEntry::domestic(RealtimeType::StockExecution, None).unwrap();
    assert_eq!(
        serde_json::to_value(optional).unwrap(),
        json!({"type":["0B"]})
    );
    assert!(serde_json::from_value::<RegistrationAcknowledgement>(json!({"trnm":"REG"})).is_err());
}

// 부호가 있는 거래 문자열과 원본 세금·통화를 추가 계산 없이 보존한다.
#[test]
fn signed_trade_strings_preserve_original_tax_and_currency_without_derivation() {
    let raw = r#"{"trnm":"REAL","data":[{"type":"0B","values":{"15":"-0005","1313":"+000010.00"}},{"type":"F5","values":{"8075":"000002.00","13006":"000001.00","8043":"USD","50725":""}}]}"#;
    let d = decode_realtime(raw).unwrap();
    let items = d.data.unwrap();
    match items[0].values.as_ref().unwrap() {
        RealtimePayload::StockExecution(v) => {
            assert_eq!(v.signed_execution_quantity.as_deref(), Some("-0005"))
        }
        _ => panic!("wrong feed"),
    }
    match items[1].values.as_ref().unwrap() {
        RealtimePayload::UsExecution(v) => {
            assert_eq!(v.tax.as_deref(), Some("000002.00"));
            assert_eq!(v.currency_code.as_deref(), Some("USD"));
            assert_eq!(
                v.fx_realized_profit_loss_purchase_amount.as_deref(),
                Some("")
            );
        }
        _ => panic!("wrong feed"),
    }
}

// 공백뿐인 구독 항목은 소켓 전송 전에 거부한다.
#[test]
fn whitespace_item_is_rejected_before_socket_send() {
    assert!(SubscriptionEntry::domestic(RealtimeType::StockExecution, Some("   ")).is_err());
}

// 기대 매핑은 공개 HTML 필드 표를 확인하여 별도로 작성한 사실이다.
// 각 FID마다 다른 합성 문자열을 넣고 이름으로 읽어, 잘못된 serde 매핑의 왕복 성공을 차단한다.
// 공식 예제 값과 생산 코드의 FIDS/필드 목록을 테스트 입력 생성에 사용하지 않는다.
macro_rules! named_bindings {
    ($test:ident, $api:literal, $variant:ident, {$($field:ident => $fid:literal),+ $(,)?}) => {
        #[test]
        fn $test() {
            let mut values = serde_json::Map::new();
            $(values.insert($fid.into(), json!(concat!("synthetic-fid-", $fid, ": +000.2500 ")));)+
            values.insert("futureFID".into(), json!({"preserved": true}));
            let values = Value::Object(values);
            let raw = json!({"trnm":"REAL","data":[{"type":$api,"values":values}]});
            let frame = decode_realtime(&raw.to_string()).unwrap();
            let RealtimePayload::$variant(typed) = frame.data.as_ref().unwrap()[0].values.as_ref().unwrap() else {
                panic!("피드의 구체 타입이 일치해야 한다");
            };
            $(assert_eq!(
                typed.$field.as_deref(),
                Some(concat!("synthetic-fid-", $fid, ": +000.2500 ")),
                concat!(stringify!($field), " <- FID ", $fid)
            );)+
            assert_eq!(typed.unknown_fids.len(), 1);
            assert_eq!(typed.unknown_fids["futureFID"], json!({"preserved":true}));
            assert_eq!(frame.raw, raw);
        }
    };
}

// 2026-10-12 공개 표: https://openapi.kiwoom.com/guide/apiGuideContents/14/0B
// HTML SHA256: 8f94a1a6cb9a3fa289e32c4dff6e7c56a607495b00343a06363e9a9cc7467b04
named_bindings!(stock_execution_0b_named_bindings, "0B", StockExecution, {
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

// 2026-10-12 공개 표: https://openapi.kiwoom.com/guide/apiGuideContents/14/0D
// HTML SHA256: 24311f5488c4bbeedab54ddb8cbc3b1f2b053d38f9ab49d8efd980f9b4ce41a8
// 60은 매수호가10, 90은 매도호가직전대비10이다. 표의 행 순서는 의미 매핑과 별개다.
named_bindings!(domestic_order_book_0d_named_bindings, "0D", DomesticOrderBook, {
    quote_time => "21",
    ask_price_1 => "41", ask_quantity_1 => "61", ask_previous_change_1 => "81",
    bid_price_1 => "51", bid_quantity_1 => "71", bid_previous_change_1 => "91",
    ask_price_2 => "42", ask_quantity_2 => "62", ask_previous_change_2 => "82",
    bid_price_2 => "52", bid_quantity_2 => "72", bid_previous_change_2 => "92",
    ask_price_3 => "43", ask_quantity_3 => "63", ask_previous_change_3 => "83",
    bid_price_3 => "53", bid_quantity_3 => "73", bid_previous_change_3 => "93",
    ask_price_4 => "44", ask_quantity_4 => "64", ask_previous_change_4 => "84",
    bid_price_4 => "54", bid_quantity_4 => "74", bid_previous_change_4 => "94",
    ask_price_5 => "45", ask_quantity_5 => "65", ask_previous_change_5 => "85",
    bid_price_5 => "55", bid_quantity_5 => "75", bid_previous_change_5 => "95",
    ask_price_6 => "46", ask_quantity_6 => "66", ask_previous_change_6 => "86",
    bid_price_6 => "56", bid_quantity_6 => "76", bid_previous_change_6 => "96",
    ask_price_7 => "47", ask_quantity_7 => "67", ask_previous_change_7 => "87",
    bid_price_7 => "57", bid_quantity_7 => "77", bid_previous_change_7 => "97",
    ask_price_8 => "48", ask_quantity_8 => "68", ask_previous_change_8 => "88",
    bid_price_8 => "58", bid_quantity_8 => "78", bid_previous_change_8 => "98",
    ask_price_9 => "49", ask_quantity_9 => "69", ask_previous_change_9 => "89",
    bid_price_9 => "59", bid_quantity_9 => "79", bid_previous_change_9 => "99",
    ask_price_10 => "50", ask_quantity_10 => "70", ask_previous_change_10 => "90",
    bid_price_10 => "60", bid_quantity_10 => "80", bid_previous_change_10 => "100",
    total_ask_quantity => "121", total_ask_quantity_change => "122",
    total_bid_quantity => "125", total_bid_quantity_change => "126",
    expected_execution_price_23 => "23", expected_execution_quantity_24 => "24",
    net_buy_quantity => "128", buy_ratio => "129", net_sell_quantity => "138", sell_ratio => "139",
    expected_previous_close_change => "200", expected_previous_close_change_rate => "201",
    expected_previous_close_change_sign => "238", expected_execution_price_291 => "291",
    expected_execution_quantity_292 => "292", expected_previous_change_sign => "293",
    expected_previous_change => "294", expected_previous_change_rate => "295",
    krx_ask_quantity_1 => "6044", krx_ask_quantity_2 => "6045", krx_ask_quantity_3 => "6046",
    krx_ask_quantity_4 => "6047", krx_ask_quantity_5 => "6048", krx_ask_quantity_6 => "6049",
    krx_ask_quantity_7 => "6050", krx_ask_quantity_8 => "6051", krx_ask_quantity_9 => "6052",
    krx_ask_quantity_10 => "6053",
    krx_bid_quantity_1 => "6054", krx_bid_quantity_2 => "6055", krx_bid_quantity_3 => "6056",
    krx_bid_quantity_4 => "6057", krx_bid_quantity_5 => "6058", krx_bid_quantity_6 => "6059",
    krx_bid_quantity_7 => "6060", krx_bid_quantity_8 => "6061", krx_bid_quantity_9 => "6062",
    krx_bid_quantity_10 => "6063", krx_total_ask_quantity => "6064", krx_total_bid_quantity => "6065",
    nxt_ask_quantity_1 => "6066", nxt_ask_quantity_2 => "6067", nxt_ask_quantity_3 => "6068",
    nxt_ask_quantity_4 => "6069", nxt_ask_quantity_5 => "6070", nxt_ask_quantity_6 => "6071",
    nxt_ask_quantity_7 => "6072", nxt_ask_quantity_8 => "6073", nxt_ask_quantity_9 => "6074",
    nxt_ask_quantity_10 => "6075",
    nxt_bid_quantity_1 => "6076", nxt_bid_quantity_2 => "6077", nxt_bid_quantity_3 => "6078",
    nxt_bid_quantity_4 => "6079", nxt_bid_quantity_5 => "6080", nxt_bid_quantity_6 => "6081",
    nxt_bid_quantity_7 => "6082", nxt_bid_quantity_8 => "6083", nxt_bid_quantity_9 => "6084",
    nxt_bid_quantity_10 => "6085", nxt_total_ask_quantity => "6086", nxt_total_bid_quantity => "6087",
    krx_midpoint_ask_quantity_change => "6100", krx_midpoint_ask_quantity => "6101",
    krx_midpoint_price => "6102", krx_midpoint_bid_quantity => "6103",
    krx_midpoint_bid_quantity_change => "6104", nxt_midpoint_ask_quantity_change => "6105",
    nxt_midpoint_ask_quantity => "6106", nxt_midpoint_price => "6107",
    nxt_midpoint_bid_quantity => "6108", nxt_midpoint_bid_quantity_change => "6109",
    krx_midpoint_change => "6110", krx_midpoint_change_sign => "6111", krx_midpoint_change_rate => "6112",
    nxt_midpoint_change => "6113", nxt_midpoint_change_sign => "6114", nxt_midpoint_change_rate => "6115"
});

// 2026-10-12 공개 표: https://openapi.kiwoom.com/guide/apiGuideContents/40/FE
// HTML SHA256: ee1a00bac034da7b5bbe008ecf9d45b3fafaff6d142fb19282e579fae7b3e308
// 20(시간)과 51020(체결시간)을 다른 문자열로 검증하고 시간대 변환을 추론하지 않는다.
named_bindings!(us_trade_price_fe_named_bindings, "FE", UsTradePrice, {
    current_price => "10", previous_change => "11", change_rate => "12",
    cumulative_volume => "13", cumulative_trade_amount => "14", execution_quantity => "15",
    open_price => "16", high_price => "17", low_price => "18", time => "20",
    execution_date => "22", previous_change_sign => "25", best_ask => "27", best_bid => "28",
    previous_volume_ratio => "30", execution_strength => "228", session_class => "290",
    execution_time => "51020"
});
