//! API·경로 허용 목록과 응답 크기 제한을 적용하는 실제 HTTP 어댑터.

use super::{
    CONTROL_HEADERS, ClientError, Environment, HttpRequest, HttpResponse, Transport, TransportError,
};
use async_trait::async_trait;
use serde_json::Value;
use std::time::Duration;

/// 호출자의 명시적인 네트워크 허용으로 생성하는 HTTP 어댑터.
///
/// 환경에서 자격 증명을 읽지 않으며 자동 재시도하지 않습니다.
pub struct ReqwestTransport {
    client: reqwest::Client,
    base: String,
}

impl ReqwestTransport {
    /// 대상 서버를 선택하고 30초 제한·리다이렉트 차단을 적용한 전송기를 생성합니다.
    pub fn with_explicit_network_opt_in(environment: Environment) -> Result<Self, ClientError> {
        let client = Self::client_builder()
            .build()
            .map_err(|_| ClientError::Transport)?;
        Ok(Self {
            client,
            base: match environment {
                Environment::Production => "https://api.kiwoom.com".into(),
                Environment::Sandbox => "https://mockapi.kiwoom.com".into(),
            },
        })
    }

    fn client_builder() -> reqwest::ClientBuilder {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
    }
}

#[async_trait]
impl Transport for ReqwestTransport {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.execute_checked(request)
            .await
            .map_err(|_| TransportError)
    }

    async fn execute_checked(&self, r: HttpRequest) -> Result<HttpResponse, ClientError> {
        // 직접 전송 포트를 호출해도 정확한 API·경로 계약과 금융 실행 허용을 확인합니다.
        r.validate_dispatch()?;

        let mut request = self
            .client
            .post(format!("{}{}", self.base, r.path))
            .header("content-type", "application/json;charset=UTF-8")
            .header("api-id", r.api_id)
            .json(&r.body);

        if let Some(token) = r.authorization {
            request = request.bearer_auth(token.expose());
        }

        if let Some(key) = r.continuation {
            request = request.header("cont-yn", "Y").header("next-key", key);
        }

        let mut response = request.send().await.map_err(|_| ClientError::Transport)?;
        let status = response.status().as_u16();
        // 단일 제어 헤더의 중복·인코딩 오류는 값 수집 전에 거부합니다.
        for name in CONTROL_HEADERS {
            let values: Vec<_> = response.headers().get_all(*name).iter().collect();
            if values.len() > 1 {
                return Err(ClientError::Protocol("duplicate HTTP control header"));
            }
            if values.first().is_some_and(|v| v.to_str().is_err()) {
                return Err(ClientError::Protocol("invalid HTTP control header"));
            }
        }
        let headers = response
            .headers()
            .iter()
            .filter_map(|(k, v)| {
                v.to_str()
                    .ok()
                    .map(|s| (k.as_str().to_owned(), s.to_owned()))
            })
            .collect();

        // 실패 상태·재시도 지시를 본문 수신 실패로 덮지 않으며 오류 본문은 읽지 않습니다.
        if !(200..300).contains(&status) {
            return HttpResponse {
                status,
                headers,
                body: Value::Null,
            }
            .normalized();
        }

        // 수신 중 본문을 8 MiB로 제한하여 전부 받은 뒤에만 JSON을 해석합니다.
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| ClientError::Transport)? {
            if bytes.len() + chunk.len() > 8 * 1024 * 1024 {
                return Err(ClientError::Transport);
            }
            bytes.extend_from_slice(&chunk);
        }

        let body = serde_json::from_slice(&bytes).map_err(|_| ClientError::Transport)?;
        HttpResponse {
            status,
            headers,
            body,
        }
        .normalized()
    }
}

#[cfg(test)]
mod tests;
