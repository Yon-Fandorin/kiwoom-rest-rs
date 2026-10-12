//! typed 호출, 페이지 수집, 요청 간격과 읽기 재시도를 함께 관리합니다.

use super::{ClientError, HttpRequest, HttpResponse, Page, Secret, Transport, WritePolicy};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// 페이지 상한, 읽기 재시도와 요청 간격을 제한하는 클라이언트 정책.
#[derive(Debug, Clone)]
pub struct ClientOptions {
    /// 전체 조회의 페이지 상한. 기본 100, 허용 범위 1..=10000.
    pub max_pages: usize,

    /// 최초 요청 이후 읽기 재시도 횟수. 기본 2, 최대 5.
    pub max_read_retries: usize,

    /// 읽기 재시도의 지수 대기 시작값. 기본 500 ms, 최대 30초.
    pub retry_base_delay: Duration,

    /// 동일 클라이언트의 요청 시작 간격. 기본 200 ms, 최대 60초.
    pub min_request_interval: Duration,

    /// 금융 변경 정책. 기본값은 [`WritePolicy::Deny`].
    pub writes: WritePolicy,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            max_pages: 100,
            max_read_retries: 2,
            retry_base_delay: Duration::from_millis(500),
            min_request_interval: Duration::from_millis(200),
            writes: WritePolicy::Deny,
        }
    }
}

/// 주입된 전송 포트와 요청 간격을 공유하며 typed 호출·페이지 수집을 관리하는 클라이언트.
///
/// OAuth와 금융 변경은 한 번만 시도합니다. 전송 후 오류나 취소가 발생하면
/// 서버 처리 여부가 불명확할 수 있으므로 호출자가 상태를 확인해야 합니다.
pub struct Client<T> {
    transport: Arc<T>,
    options: ClientOptions,
    last_request: Mutex<Option<tokio::time::Instant>>,
}

impl<T: Transport> Client<T> {
    /// 정책의 안전 상한을 검증하고 클라이언트를 구성합니다. 네트워크 연결은 열지 않습니다.
    pub fn new(transport: T, options: ClientOptions) -> Result<Self, ClientError> {
        if options.max_pages == 0
            || options.max_pages > 10000
            || options.max_read_retries > 5
            || options.retry_base_delay > Duration::from_secs(30)
            || options.min_request_interval > Duration::from_secs(60)
        {
            return Err(ClientError::InvalidRequest("invalid client limits"));
        }

        Ok(Self {
            transport: Arc::new(transport),
            options,
            last_request: Mutex::new(None),
        })
    }

    /// 구현된 금융 변경 메서드의 명시적 실행 허용을 확인합니다.
    pub fn require_write_opt_in(&self) -> Result<(), ClientError> {
        self.options.writes.require()
    }

    /// 실제 구현된 API 식별자인지 확인합니다. 인벤토리에만 있거나 알 수 없는 식별자는 차단합니다.
    pub fn require_implemented_api(&self, id: &str) -> Result<(), ClientError> {
        if crate::registry::implemented_id(id) {
            Ok(())
        } else {
            Err(ClientError::UnsupportedApi)
        }
    }

    /// 설정된 페이지 수 상한을 반환합니다.
    /// 단일 페이지 호출을 이어가는 경우 호출자가 전체 호출 수를 제한해야 합니다.
    pub fn max_pages(&self) -> usize {
        self.options.max_pages
    }

    /// 크레이트의 typed 요청이 공유하는 단일 페이지 조회·응답 검증 흐름.
    pub(crate) async fn read_page<Q: Serialize, R: DeserializeOwned>(
        &self,
        token: &Secret,
        body: &Q,
        api_id: &'static str,
        path: &'static str,
        continuation: Option<&str>,
    ) -> Result<Page<R>, ClientError> {
        if !crate::registry::read_only_pair(api_id, path) {
            return Err(ClientError::UnsupportedApi);
        }

        if continuation.is_some_and(|key| key.trim().is_empty() || key.len() > 50) {
            return Err(ClientError::InvalidRequest("invalid continuation cursor"));
        }

        let mut request = Self::request(path, api_id, Some(token.clone()), body)?;
        request.continuation = continuation.map(str::to_owned);
        let response = self.send(request, true).await?;
        if response
            .headers
            .get("api-id")
            .is_some_and(|id| id != api_id)
        {
            return Err(ClientError::Protocol("response API identifier mismatch"));
        }

        decode_page(response)
    }

    /// 크레이트의 구체적인 DTO 메서드가 공유하는 읽기 페이지 수집 흐름.
    pub(crate) async fn read_pages<Q: Serialize, R: DeserializeOwned>(
        &self,
        token: &Secret,
        request: &Q,
        api_id: &'static str,
        path: &'static str,
    ) -> Result<Vec<Page<R>>, ClientError> {
        if !crate::registry::read_only_pair(api_id, path) {
            return Err(ClientError::UnsupportedApi);
        }

        self.pages(Self::request(path, api_id, Some(token.clone()), request)?)
            .await
    }

    /// 명시적 실행 허용을 확인한 금융 변경을 한 번만 전송합니다.
    /// 실패나 취소 후 서버 처리 여부는 별도로 확인해야 하며 자동 재제출하지 않습니다.
    pub(crate) async fn write_once<Q: Serialize, R: DeserializeOwned>(
        &self,
        token: &Secret,
        request: &Q,
        api_id: &'static str,
        path: &'static str,
    ) -> Result<Page<R>, ClientError> {
        self.require_write_opt_in()?;

        if !crate::registry::financial_write_pair(api_id, path) {
            return Err(ClientError::UnsupportedApi);
        }

        let mut request = Self::request(path, api_id, Some(token.clone()), request)?;
        request.financial_write_authorized = true;
        let response = self.send(request, false).await?;
        if response
            .headers
            .get("api-id")
            .is_some_and(|id| id != api_id)
        {
            return Err(ClientError::Protocol("response API identifier mismatch"));
        }

        decode_page(response)
    }

    pub(crate) fn request<R: Serialize>(
        path: &'static str,
        api_id: &'static str,
        authorization: Option<Secret>,
        r: &R,
    ) -> Result<HttpRequest, ClientError> {
        let body = serde_json::to_value(r)
            .map_err(|_| ClientError::InvalidRequest("serialization failed"))?;
        Ok(HttpRequest {
            path,
            api_id,
            authorization,
            continuation: None,
            body,
            financial_write_authorized: false,
        })
    }

    async fn pages<R: DeserializeOwned>(
        &self,
        mut request: HttpRequest,
    ) -> Result<Vec<Page<R>>, ClientError> {
        let mut pages = Vec::new();
        let mut seen = BTreeSet::new();

        for _ in 0..self.options.max_pages {
            let response = self.send(request.clone(), true).await?;
            if response
                .headers
                .get("api-id")
                .is_some_and(|id| id != request.api_id)
            {
                return Err(ClientError::Protocol("response API identifier mismatch"));
            }

            let continuation = match response.headers.get("cont-yn").map(String::as_str) {
                None | Some("N") | Some("") => None,
                Some("Y") => {
                    let key = response
                        .headers
                        .get("next-key")
                        .filter(|s| !s.trim().is_empty() && s.len() <= 50)
                        .ok_or(ClientError::Protocol("continuation requires next-key"))?;
                    if !seen.insert(key.clone()) {
                        return Err(ClientError::Protocol("repeated pagination cursor"));
                    }

                    Some(key.clone())
                }
                _ => return Err(ClientError::Protocol("invalid cont-yn header")),
            };

            pages.push(decode_page(response)?);

            match continuation {
                None => return Ok(pages),
                Some(key) => request.continuation = Some(key),
            }
        }

        // 페이지 상한에 도달한 부분 이력을 완료된 조회로 반환하지 않습니다.
        Err(ClientError::PageLimit)
    }

    pub(crate) async fn send(
        &self,
        request: HttpRequest,
        read_only: bool,
    ) -> Result<HttpResponse, ClientError> {
        // OAuth와 금융 변경은 읽기 재시도 정책과 관계없이 한 번만 시도합니다.
        let retry_limit = if read_only {
            self.options.max_read_retries
        } else {
            0
        };

        for attempt in 0..=retry_limit {
            {
                // 간격 대기와 시작 시각 갱신을 한 잠금으로 직렬화하고 전송 전에 잠금을 풉니다.
                let mut last = self.last_request.lock().await;
                if let Some(t) = *last {
                    tokio::time::sleep_until(t + self.options.min_request_interval).await;
                }
                *last = Some(tokio::time::Instant::now());
            }

            let result = self.transport.execute_checked(request.clone()).await;
            match result {
                Err(ClientError::Transport) if attempt < retry_limit => {}
                Err(error) => return Err(error),
                Ok(response) => {
                    let response = response.normalized()?;
                    if (200..300).contains(&response.status) {
                        let code = response
                            .body
                            .get("return_code")
                            .and_then(Value::as_i64)
                            .ok_or(ClientError::Protocol("missing integer return_code"))?;
                        if code != 0 {
                            return Err(ClientError::Api(code));
                        }

                        return Ok(response);
                    }

                    let retry_after = response
                        .headers
                        .get("retry-after")
                        .and_then(|s| retry_after_seconds(s, chrono::Utc::now()));
                    let retryable = response.status == 429 || matches!(response.status, 502..=504);
                    if !retryable
                        || attempt == retry_limit
                        || (response.headers.contains_key("retry-after") && retry_after.is_none())
                    {
                        return Err(ClientError::Http {
                            status: response.status,
                            retry_after,
                        });
                    }

                    if let Some(seconds) = retry_after {
                        // 서버 대기가 상한보다 길면 조기 재시도하지 않고 호출자에게 반환합니다.
                        if seconds > 30 {
                            return Err(ClientError::Http {
                                status: response.status,
                                retry_after,
                            });
                        }

                        tokio::time::sleep(Duration::from_secs(seconds)).await;
                        continue;
                    }
                }
            }

            let delay = self
                .options
                .retry_base_delay
                .saturating_mul(1u32 << attempt)
                .min(Duration::from_secs(30));
            tokio::time::sleep(delay).await;
        }

        unreachable!()
    }
}

fn decode_page<R: DeserializeOwned>(response: HttpResponse) -> Result<Page<R>, ClientError> {
    Ok(Page {
        typed: decode(&response.body)?,
        raw: response.body,
        response_headers: response
            .headers
            .into_iter()
            .filter(|(name, _)| matches!(name.as_str(), "cont-yn" | "next-key" | "api-id"))
            .collect(),
    })
}

pub(crate) fn decode<R: DeserializeOwned>(body: &Value) -> Result<R, ClientError> {
    serde_json::from_value(body.clone())
        .map_err(|_| ClientError::Protocol("typed response decode failed"))
}

fn retry_after_seconds(value: &str, now: chrono::DateTime<chrono::Utc>) -> Option<u64> {
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(seconds);
    }

    let date: chrono::DateTime<chrono::Utc> = httpdate::parse_http_date(value).ok()?.into();
    let remaining = date - now;
    if remaining <= chrono::Duration::zero() {
        return Some(0);
    }
    // 마감 직전의 양수 소수초도 반올림해 서버가 요구한 대기보다 일찍 보내지 않습니다.
    let seconds = remaining.num_seconds();
    Some(seconds as u64 + u64::from(remaining > chrono::Duration::seconds(seconds)))
}

#[cfg(test)]
mod tests;
