//! 계좌·시장·주문 등의 API 계약과 검증입니다.
//! 호출·인증 갱신·통신·세션 상태는 상위 클라이언트가 소유합니다.
pub use kiwoom_core::{ClientError, Secret};
pub mod accounts;
pub mod analytics;
pub mod auth;
pub mod charts;
pub mod market;
pub mod orders;
pub mod products;
pub mod registry;
pub mod transactions;
pub mod ws;
