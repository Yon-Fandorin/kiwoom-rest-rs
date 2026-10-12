//! 2026-10-12의 공개 가이드 표를 바탕으로 독립적으로 작성한 조건검색 계약입니다.
//! 소켓·인증·구독 재실행은 제공하지 않습니다.

mod notifications;
mod rpc;

pub use notifications::{
    ConditionRealtimeItem, ConditionRealtimeResponse, ConditionRealtimeValues,
    decode_condition_realtime,
};

pub use rpc::{
    ConditionFrame, ConditionService, Ka10171Request, Ka10171Response, Ka10171Row, Ka10172Request,
    Ka10172Response, Ka10172Row, Ka10173Request, Ka10173Response, Ka10173Row, Ka10174Request,
    Ka10174Response, Usa20280Request, Usa20280Response, Usa20280Row, Usa20281Request,
    Usa20281Response, Usa20281Row, Usa20290Request, Usa20290Response, Usa20290Row, Usa20291Request,
    Usa20291Response, decode_ka10171, decode_ka10172, decode_ka10173, decode_ka10174,
    decode_usa20280, decode_usa20281, decode_usa20290, decode_usa20291,
};

/// 타입이 정의된 여덟 조건검색 WebSocket 계약 ID입니다.
pub const CONTRACT_IDS: &[&str] = &[
    "ka10171", "ka10172", "ka10173", "ka10174", "usa20280", "usa20281", "usa20290", "usa20291",
];
