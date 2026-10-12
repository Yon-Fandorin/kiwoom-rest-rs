//! 키움 API의 REST·WS 호출과 도메인 계약을 분리한 독립 Rust SDK.
pub use kiwoom_core as core;
pub use kiwoom_domain as domain;
pub mod auth;
mod registry;
pub mod rest;
mod runtime;
pub mod ws;

// 기존 경로는 같은 타입과 모듈을 재수출합니다. 상태나 계약을 복제하지 않습니다.
pub use auth::{managed_transport, token_manager};
pub use kiwoom_core::{ClientError, Secret};
pub use kiwoom_domain::{accounts, analytics, charts, market, orders, products, transactions};
pub use rest::{Client, ClientOptions, ReqwestTransport, Transport};
pub use runtime::http as transport;
pub use ws::{
    conditions, protocol as websocket, realtime, recovery as ws_recovery, session as ws_session,
    transport as ws_transport,
};
