//! 이전 규격의 세션 개수와 최대 수명을 제한합니다.
use futures::{Stream, future::join_all};
use rmcp::{
    model::{ClientJsonRpcMessage, ServerJsonRpcMessage},
    transport::streamable_http_server::session::{
        ServerSseMessage, SessionId, SessionManager,
        local::{LocalSessionManager, LocalSessionManagerError},
    },
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};
use tokio_util::sync::CancellationToken;

const MAX_SESSIONS: usize = 32;
const SESSION_LIFETIME: Duration = Duration::from_secs(600);

#[derive(Debug, thiserror::Error)]
pub(super) enum SessionError {
    #[error("MCP 세션 한도에 도달했거나 서버가 종료되었습니다.")]
    Unavailable,

    #[error(transparent)]
    Inner(#[from] LocalSessionManagerError),
}

struct SessionLife {
    created: Instant,
    cancellation: CancellationToken,
}

#[derive(Default)]
struct State {
    created: HashMap<SessionId, SessionLife>,
    stopped: bool,
}

pub(super) struct BoundedSessions {
    inner: LocalSessionManager,
    state: Mutex<State>,

    capacity: usize,
    lifetime: Duration,
}

impl Default for BoundedSessions {
    fn default() -> Self {
        Self::new(MAX_SESSIONS, SESSION_LIFETIME)
    }
}

impl BoundedSessions {
    fn new(capacity: usize, lifetime: Duration) -> Self {
        Self {
            inner: LocalSessionManager::default(),
            state: Mutex::new(State::default()),

            capacity,
            lifetime,
        }
    }

    pub(super) async fn shutdown(&self) {
        let mut state = self.state.lock().await;
        state.stopped = true;
        let ids = state
            .created
            .drain()
            .map(|(id, life)| {
                life.cancellation.cancel();
                id
            })
            .collect();
        self.close_many(ids).await;
    }

    pub(super) async fn cancellation_for(&self, id: &str) -> Option<CancellationToken> {
        self.reap().await;
        self.state
            .lock()
            .await
            .created
            .get(id)
            .map(|life| life.cancellation.clone())
    }

    pub(super) fn spawn_reaper(self: &Arc<Self>, cancellation: CancellationToken) {
        let sessions = Arc::downgrade(self);
        let period = self.lifetime.min(Duration::from_secs(30));
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(period);
            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => {
                        if let Some(sessions) = sessions.upgrade() {
                            sessions.shutdown().await;
                        }
                        break;
                    }
                    _ = interval.tick() => {
                        let Some(sessions) = sessions.upgrade() else { break; };
                        sessions.reap().await;
                    }
                }
            }
        });
    }

    async fn close_many(&self, ids: Vec<SessionId>) {
        // 종료 응답을 기다리는 동안에도 모든 세션의 제거를 함께 시작합니다.
        let closes = ids.iter().map(|id| self.inner.close_session(id));
        let _ = tokio::time::timeout(Duration::from_secs(2), join_all(closes)).await;
    }

    async fn reap(&self) {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        let expired = state
            .created
            .iter()
            .filter(|(_, time)| now.duration_since(time.created) >= self.lifetime)
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        for id in &expired {
            if let Some(life) = state.created.remove(id) {
                life.cancellation.cancel();
            }
        }
        self.close_many(expired).await;
    }
}

impl SessionManager for BoundedSessions {
    type Error = SessionError;
    type Transport = <LocalSessionManager as SessionManager>::Transport;

    async fn create_session(&self) -> Result<(SessionId, Self::Transport), Self::Error> {
        self.reap().await;
        let mut state = self.state.lock().await;
        if state.stopped || state.created.len() >= self.capacity {
            return Err(SessionError::Unavailable);
        }
        let (id, transport) = self.inner.create_session().await?;
        state.created.insert(
            id.clone(),
            SessionLife {
                created: Instant::now(),
                cancellation: CancellationToken::new(),
            },
        );
        Ok((id, transport))
    }

    async fn has_session(&self, id: &SessionId) -> Result<bool, Self::Error> {
        self.reap().await;
        if !self.state.lock().await.created.contains_key(id) {
            return Ok(false);
        }
        Ok(self.inner.has_session(id).await?)
    }

    async fn close_session(&self, id: &SessionId) -> Result<(), Self::Error> {
        if let Some(life) = self.state.lock().await.created.remove(id) {
            life.cancellation.cancel();
        }
        Ok(self.inner.close_session(id).await?)
    }

    async fn initialize_session(
        &self,
        id: &SessionId,
        message: ClientJsonRpcMessage,
    ) -> Result<ServerJsonRpcMessage, Self::Error> {
        Ok(self.inner.initialize_session(id, message).await?)
    }

    async fn create_stream(
        &self,
        id: &SessionId,
        message: ClientJsonRpcMessage,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        Ok(self.inner.create_stream(id, message).await?)
    }

    async fn accept_message(
        &self,
        id: &SessionId,
        message: ClientJsonRpcMessage,
    ) -> Result<(), Self::Error> {
        Ok(self.inner.accept_message(id, message).await?)
    }

    async fn create_standalone_stream(
        &self,
        id: &SessionId,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        Ok(self.inner.create_standalone_stream(id).await?)
    }

    async fn resume(
        &self,
        id: &SessionId,
        last_event_id: String,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        Ok(self.inner.resume(id, last_event_id).await?)
    }
}

#[cfg(test)]
mod tests;
