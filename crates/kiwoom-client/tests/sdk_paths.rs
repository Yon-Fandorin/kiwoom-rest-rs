use kiwoom_client::{accounts, auth, transactions, transport};
use std::any::TypeId;

#[test]
fn canonical_and_compatibility_paths_share_contracts_and_state_types() {
    fn same<A: 'static, B: 'static>() {
        assert_eq!(TypeId::of::<A>(), TypeId::of::<B>());
    }

    // 재수출 경로가 별도의 계약이나 토큰 상태를 만들지 않아야 합니다.
    same::<auth::token_manager::TokenManager, kiwoom_client::token_manager::TokenManager>();
    same::<
        auth::managed_transport::AuthenticationRetry,
        kiwoom_client::managed_transport::AuthenticationRetry,
    >();
    same::<
        auth::managed_transport::ManagedTransport<()>,
        kiwoom_client::managed_transport::ManagedTransport<()>,
    >();
    same::<
        transport::Page<accounts::Kt00015Response>,
        transactions::Page<transactions::Kt00015Response>,
    >();
    same::<accounts::Kt00015Request, transactions::Kt00015Request>();
    same::<accounts::Ust21100Request, transactions::Ust21100Request>();
    same::<accounts::DomesticTransactionType, transactions::DomesticTransactionType>();
    same::<accounts::UsTransactionType, transactions::UsTransactionType>();
}

#[test]
fn layered_entry_points_and_domain_types_preserve_existing_public_paths() {
    use kiwoom_client::{core, domain, rest, ws};
    fn same<A: 'static, B: 'static>() {
        assert_eq!(TypeId::of::<A>(), TypeId::of::<B>());
    }
    same::<rest::Client<()>, kiwoom_client::Client<()>>();
    same::<ws::Client<()>, kiwoom_client::ws_session::WsSession<()>>();
    same::<domain::accounts::Kt00015Request, accounts::Kt00015Request>();
    same::<domain::ws::conditions::Ka10171Request, kiwoom_client::conditions::Ka10171Request>();
    same::<core::ClientError, kiwoom_client::ClientError>();
    same::<core::Secret, kiwoom_client::Secret>();
    same::<core::Page<domain::accounts::Kt00015Response>, transport::Page<accounts::Kt00015Response>>(
    );
}
