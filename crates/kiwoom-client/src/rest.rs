//! REST 최상위 호출 인터페이스. 계약은 domain, 호출 정책과 통신은 내부 실행 계층에 둡니다.
pub use crate::runtime::http::{
    Client, ClientOptions, Environment, HttpRequest, HttpResponse, Page, ReqwestTransport,
    Transport, TransportError, WritePolicy,
};
macro_rules! read {
    ($method:ident, $req:ident, $res:ident, $path:literal) => {
        impl<T: Transport> Client<T> {
            /// 계약을 검증하고 조회의 연속 페이지를 원본 응답과 함께 반환합니다.
            pub async fn $method(
                &self,
                token: &Secret,
                request: &$req,
            ) -> Result<Vec<Page<$res>>, ClientError> {
                request.validate()?;
                self.read_pages(token, request, stringify!($method), $path)
                    .await
            }
        }
    };
}
macro_rules! mutation {
    ($method:ident, $req:ident, $res:ident, $path:literal) => {
        impl<T: Transport> Client<T> {
            /// 명시적 실행 허용을 먼저 확인하고 금융 요청을 한 번만 전송합니다.
            pub async fn $method(
                &self,
                token: &Secret,
                request: &$req,
            ) -> Result<Page<$res>, ClientError> {
                self.require_write_opt_in()?;
                request.validate()?;
                self.write_once(token, request, stringify!($method), $path)
                    .await
            }
        }
    };
}

mod accounts;
mod analytics;
mod charts;
mod market;
pub mod oauth;
mod orders;
mod products;
