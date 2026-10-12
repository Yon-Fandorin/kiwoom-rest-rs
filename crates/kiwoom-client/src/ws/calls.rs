//! 조건검색의 이름 있는 호출은 WS 인터페이스가 소유합니다.
use super::{Client as WsSession, WsReply, WsTransport};
use kiwoom_core::ClientError;
use kiwoom_domain::ws::conditions::*;

// API별 이름을 사용하는 호출도 봉인된 세션의 동일한 검증·대기 흐름을 거칩니다.
impl<T: WsTransport> WsSession<T> {
    /// `ka10171` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn ka10171(
        &mut self,
        request: &Ka10171Request,
    ) -> Result<WsReply<Ka10171Response>, ClientError> {
        self.call(request).await
    }

    /// `ka10172` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn ka10172(
        &mut self,
        request: &Ka10172Request,
    ) -> Result<WsReply<Ka10172Response>, ClientError> {
        self.call(request).await
    }

    /// `ka10173` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn ka10173(
        &mut self,
        request: &Ka10173Request,
    ) -> Result<WsReply<Ka10173Response>, ClientError> {
        self.call(request).await
    }

    /// `ka10174` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn ka10174(
        &mut self,
        request: &Ka10174Request,
    ) -> Result<WsReply<Ka10174Response>, ClientError> {
        self.call(request).await
    }

    /// `usa20280` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn usa20280(
        &mut self,
        request: &Usa20280Request,
    ) -> Result<WsReply<Usa20280Response>, ClientError> {
        self.call(request).await
    }

    /// `usa20281` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn usa20281(
        &mut self,
        request: &Usa20281Request,
    ) -> Result<WsReply<Usa20281Response>, ClientError> {
        self.call(request).await
    }

    /// `usa20290` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn usa20290(
        &mut self,
        request: &Usa20290Request,
    ) -> Result<WsReply<Usa20290Response>, ClientError> {
        self.call(request).await
    }

    /// `usa20291` 조건검색 요청을 검증하고 typed 응답과 원본·성공 확인 여부를 반환합니다.
    pub async fn usa20291(
        &mut self,
        request: &Usa20291Request,
    ) -> Result<WsReply<Usa20291Response>, ClientError> {
        self.call(request).await
    }
}
