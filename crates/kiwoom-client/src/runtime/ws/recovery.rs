//! 확인 응답을 받은 실시간 구독을 호출자가 정해진 한도 안에서 복구합니다.
//! 새 연결은 구독 설정을 복원하며 유실된 이벤트 스트림·이력을 복원하지 않습니다.

mod configuration;

use super::{
    conditions::{
        Ka10173Request, Ka10173Response, Ka10174Request, Ka10174Response, Usa20290Request,
        Usa20290Response, Usa20291Request, Usa20291Response,
    },
    session::{SessionOptions, WsReply, WsRequest, WsSession, WsTransport},
    subscription::{
        RealtimeSubscription, RefreshMode, RegistrationAcknowledgement, SubscriptionAction,
    },
};
use crate::{ClientError, Secret, runtime::auth::token_manager::TokenManager};
use async_trait::async_trait;
use configuration::{ConditionMarket, ConditionSubscription, ConfirmedConfiguration};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

/// 복구 시도마다 새 연결을 제공하는 주입 포트.
#[async_trait]
pub trait WsTransportFactory: Send {
    /// 팩토리가 생성하는 세션 전송 타입.
    type Transport: WsTransport;

    /// 새 전송 객체를 생성해야 합니다.
    /// 여기서는 증권사·네트워크 연결을 암묵적으로 설정하지 않습니다.
    async fn connect(&mut self) -> Result<Self::Transport, ClientError>;
}

/// 호출자가 제공하는 토큰 획득 포트.
#[async_trait]
pub trait WsTokenProvider: Send + Sync {
    /// LOGIN에 사용할 토큰을 반환하며 발급·재사용 정책은 구현이 소유합니다.
    async fn token(&self) -> Result<Secret, ClientError>;
}

/// 시도·전체 기한·확인된 구독·공백 기록의 복구 한도.
#[derive(Debug, Clone)]
pub struct RecoveryOptions {
    /// 복구 시도의 최대 횟수.
    pub max_attempts: usize,

    /// 토큰 획득·팩토리 연결·LOGIN·구독 재등록·재시도 대기까지 포함하는 전체 기한입니다.
    pub deadline: Duration,

    /// 두 번째 시도부터 적용하며 전체 기한에 포함되는 대기 시간.
    pub backoff: Duration,

    /// 확인된 실시간 그룹의 최대 개수.
    pub max_groups: usize,

    /// 실시간 원자 항목과 연속 조건검색을 합친 최대 개수.
    pub max_entries: usize,

    /// 감사 기록이 한도에 도달하면 작업을 차단하며, 미해결 공백을 삭제하지 않습니다.
    pub max_gap_markers: usize,
}

impl Default for RecoveryOptions {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            deadline: Duration::from_secs(60),
            backoff: Duration::from_secs(1),
            max_groups: 100,
            max_entries: 1000,
            max_gap_markers: 1000,
        }
    }
}

/// 데이터 연속성을 확인하지 못한 작업의 구분.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapReason {
    /// 새 연결·구독 재등록 작업.
    Recovery,

    /// 구독 변경 결과가 미확정인 작업.
    SubscriptionUncertain,

    /// 수신 결과가 미확정인 작업.
    ReceiveUncertain,

    /// 종료 상황을 표현할 구분값.
    Shutdown,
}

/// 해당 연결 세대에 남은 미해결 공백.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryGap {
    /// 공백이 기록된 연결 세대.
    pub generation: u64,

    /// 미확정 작업의 사유.
    pub reason: GapReason,
}

/// 현재 로컬 연결·확인된 설정·공백 상태의 조회 결과.
#[derive(Debug, Clone)]
pub struct RecoveryStatus {
    /// 인증된 세션을 보유하는지 여부. 서버 생존 확인을 뜻하지 않습니다.
    pub connected: bool,

    /// 종료 후 새 작업을 허용하지 않는 상태.
    pub shutdown: bool,

    /// 마지막 ACK로 확인한 실시간 그룹 개수.
    pub confirmed_groups: usize,

    /// 마지막 ACK로 확인한 실시간 원자 항목 개수.
    pub confirmed_entries: usize,

    /// 시장·순번으로 구분한 연속 조건검색 개수.
    pub confirmed_conditions: usize,

    /// 현재 연결과 확인된 설정이 일치하는 세대.
    pub confirmed_generation: Option<u64>,

    /// 완료된 실패와 진행 중 미확정 작업의 공백 목록.
    pub unresolved_gaps: Vec<RecoveryGap>,

    /// 항상 false입니다.
    /// 이 API는 기준 이력과의 대사를 제공하지 않습니다.
    pub history_verified_complete: bool,
}

/// 완전히 성공한 구독 복구에서 받은 조건검색 초기 결과.
#[derive(Debug, Clone)]
pub struct ConditionReplaySnapshot {
    /// 결과를 얻은 연결 세대.
    pub generation: u64,

    /// 재등록한 조건검색 계약 식별자.
    pub api_id: &'static str,

    /// 알 수 없는 필드까지 보존한 초기 응답 JSON.
    pub raw: Value,
}

/// 확인된 구독만 새 세션에 재등록하며 미해결 공백을 보존하는 복구 소유자.
pub struct WsRecovery<F: WsTransportFactory, P: WsTokenProvider> {
    factory: F,
    tokens: P,
    session_options: SessionOptions,
    options: RecoveryOptions,
    session: Option<WsSession<F::Transport>>,
    configuration: ConfirmedConfiguration,
    condition_snapshots: Vec<ConditionReplaySnapshot>,
    generation: u64,
    confirmed_generation: Option<u64>,
    gaps: Vec<RecoveryGap>,
    pending: Option<RecoveryGap>,
    shutdown: bool,
}

impl<F: WsTransportFactory, P: WsTokenProvider> WsRecovery<F, P> {
    /// 연결 없이 의존성과 복구 한도를 검증합니다.
    pub fn new(
        factory: F,
        tokens: P,
        session_options: SessionOptions,
        options: RecoveryOptions,
    ) -> Result<Self, ClientError> {
        if options.max_attempts == 0
            || options.deadline.is_zero()
            || options.max_groups == 0
            || options.max_entries == 0
            || options.max_gap_markers == 0
            || session_options.timeout.is_zero()
            || session_options.max_frame_bytes == 0
            || session_options.max_pending_events == 0
            || session_options.max_interleaved_frames == 0
            || tokio::time::Instant::now()
                .checked_add(options.deadline)
                .is_none()
            || tokio::time::Instant::now()
                .checked_add(options.backoff)
                .is_none()
            || tokio::time::Instant::now()
                .checked_add(session_options.timeout)
                .is_none()
        {
            return Err(ClientError::InvalidRequest(
                "invalid WebSocket recovery limits",
            ));
        }
        Ok(Self {
            factory,
            tokens,
            session_options,
            options,
            session: None,
            configuration: ConfirmedConfiguration::default(),
            condition_snapshots: Vec::new(),
            generation: 0,
            confirmed_generation: None,
            gaps: Vec::new(),
            pending: None,
            shutdown: false,
        })
    }

    /// 현재 로컬 연결·확인된 설정·미해결 공백을 반환합니다.
    pub fn status(&self) -> RecoveryStatus {
        RecoveryStatus {
            connected: self
                .session
                .as_ref()
                .is_some_and(WsSession::is_authenticated),
            shutdown: self.shutdown,
            confirmed_groups: self.configuration.group_count(),
            confirmed_entries: self.configuration.entry_count(),
            confirmed_conditions: self.configuration.condition_count(),
            confirmed_generation: self.confirmed_generation,
            unresolved_gaps: self
                .gaps
                .iter()
                .cloned()
                .chain(self.pending.iter().cloned())
                .collect(),
            history_verified_complete: false,
        }
    }

    /// 마지막으로 성공한 복구에서 새로 얻은 초기 검색 결과입니다.
    /// 스냅샷이며 누락된 증분 이벤트의 재실행 결과가 아닙니다.
    pub fn condition_replay_snapshots(&self) -> &[ConditionReplaySnapshot] {
        if self.session.is_some() && self.confirmed_generation == Some(self.generation) {
            &self.condition_snapshots
        } else {
            &[]
        }
    }

    fn begin(&mut self, reason: GapReason) -> Result<(), ClientError> {
        if self.shutdown {
            return Err(ClientError::Protocol("WebSocket recovery is shut down"));
        }
        if let Some(pending) = self.pending.take() {
            self.gaps.push(pending);
        }
        if self.gaps.len() >= self.options.max_gap_markers {
            return Err(ClientError::Protocol("WebSocket gap audit full"));
        }
        self.pending = Some(RecoveryGap {
            generation: self.generation,
            reason,
        });
        self.confirmed_generation = None;
        Ok(())
    }

    fn finish_failure(&mut self) {
        if let Some(pending) = self.pending.take() {
            self.gaps.push(pending);
        }
    }

    /// 전송 실패에 한해서 새 세션으로 안전한 읽기 전용 작업을 재시도합니다.
    /// 프로토콜·API 거부나 전체 기한 도달 시 복구를 중단합니다.
    pub async fn recover(&mut self) -> Result<(), ClientError> {
        if self.shutdown {
            return Err(ClientError::Protocol("WebSocket recovery is shut down"));
        }
        if self.session.is_some() {
            return Ok(());
        }
        self.begin(GapReason::Recovery)?;
        self.condition_snapshots.clear();
        let deadline = tokio::time::Instant::now()
            .checked_add(self.options.deadline)
            .ok_or(ClientError::InvalidRequest(
                "WebSocket recovery deadline overflow",
            ))?;
        let mut result = Err(ClientError::Transport);
        for attempt in 0..self.options.max_attempts {
            if attempt > 0 {
                self.begin(GapReason::Recovery)?;
            }
            self.generation = self
                .generation
                .checked_add(1)
                .ok_or(ClientError::Protocol("WebSocket generation exhausted"))?;
            if let Some(pending) = &mut self.pending {
                pending.generation = self.generation;
            }
            let operation = async {
                if attempt > 0 {
                    tokio::time::sleep(self.options.backoff).await;
                }
                let token = self.tokens.token().await?;
                let transport = self.factory.connect().await?;
                let mut session = WsSession::new(transport, self.session_options.clone())?;
                session.login(&token).await?;
                for (group, entries) in self.configuration.feed_groups() {
                    let request = RealtimeSubscription::new(
                        SubscriptionAction::Register,
                        group,
                        RefreshMode::ReplaceGroup,
                        entries.clone(),
                    )?;
                    let reply = session.call(&request).await?;
                    ConfirmedConfiguration::confirmed_ack(&reply, group)?;
                }
                let mut snapshots = Vec::new();
                for ((_, seq), subscription) in self.configuration.condition_subscriptions() {
                    let (api_id, raw) = match subscription {
                        ConditionSubscription::Domestic(request) => {
                            let reply = session.call(request).await?;
                            ConfirmedConfiguration::condition_ack(&reply, seq)?;
                            (request.api_id(), reply.raw)
                        }
                        ConditionSubscription::Us(request) => {
                            let reply = session.call(request).await?;
                            ConfirmedConfiguration::condition_ack(&reply, seq)?;
                            (request.api_id(), reply.raw)
                        }
                    };
                    snapshots.push(ConditionReplaySnapshot {
                        generation: self.generation,
                        api_id,
                        raw,
                    });
                }
                Ok((session, snapshots))
            };
            match tokio::time::timeout_at(deadline, operation).await {
                Ok(Ok((session, snapshots))) => {
                    self.session = Some(session);
                    self.condition_snapshots = snapshots;
                    self.confirmed_generation = Some(self.generation);
                    // 완료된 복구의 진행 중 표시만 해제합니다.
                    // 기존 실패·취소 표시는 계속 보존합니다.
                    self.pending = None;
                    return Ok(());
                }
                Ok(Err(error)) => {
                    let retry = matches!(error, ClientError::Transport);
                    result = Err(error);
                    self.finish_failure();
                    if !retry {
                        break;
                    }
                }
                Err(_) => {
                    result = Err(ClientError::Transport);
                    self.finish_failure();
                    break;
                }
            }
        }
        self.finish_failure();
        result
    }

    async fn condition_change<Q: WsRequest>(
        &mut self,
        request: &Q,
        key: (ConditionMarket, String),
        change: Option<ConditionSubscription>,
    ) -> Result<WsReply<Q::Response>, ClientError> {
        self.configuration.check_condition_change(
            request,
            &key,
            change.is_some(),
            self.options.max_entries,
        )?;
        if self.session.is_none() {
            return Err(ClientError::Protocol("WebSocket recovery required"));
        }
        self.begin(GapReason::SubscriptionUncertain)?;
        self.condition_snapshots.clear();
        let mut session = self.session.take().ok_or(ClientError::Transport)?;
        let result = session.call(request).await.and_then(|reply| {
            ConfirmedConfiguration::condition_ack(&reply, &key.1)?;
            Ok(reply)
        });
        match result {
            Ok(reply) => {
                self.configuration.commit_condition_change(key, change);
                self.session = Some(session);
                self.pending = None;
                self.confirmed_generation = Some(self.generation);
                Ok(reply)
            }
            Err(error) => {
                self.finish_failure();
                Err(error)
            }
        }
    }

    /// 국내 연속 조건검색을 등록하며 ACK를 확인한 뒤 설정에 반영합니다.
    pub async fn subscribe_domestic_condition(
        &mut self,
        request: &Ka10173Request,
    ) -> Result<WsReply<Ka10173Response>, ClientError> {
        let key = (ConditionMarket::Domestic, request.seq.clone());
        self.configuration.check_domestic_exchange(request, &key)?;
        self.condition_change(
            request,
            key,
            Some(ConditionSubscription::Domestic(request.clone())),
        )
        .await
    }

    /// 미국 연속 조건검색을 등록하며 ACK를 확인한 뒤 설정에 반영합니다.
    pub async fn subscribe_us_condition(
        &mut self,
        request: &Usa20290Request,
    ) -> Result<WsReply<Usa20290Response>, ClientError> {
        self.condition_change(
            request,
            (ConditionMarket::Us, request.seq.clone()),
            Some(ConditionSubscription::Us(request.clone())),
        )
        .await
    }

    /// 확인된 국내 연속 조건검색을 해제하고 성공 ACK 뒤 설정에서 제거합니다.
    pub async fn clear_domestic_condition(
        &mut self,
        request: &Ka10174Request,
    ) -> Result<WsReply<Ka10174Response>, ClientError> {
        self.condition_change(
            request,
            (ConditionMarket::Domestic, request.seq.clone()),
            None,
        )
        .await
    }

    /// 확인된 미국 연속 조건검색을 해제하고 성공 ACK 뒤 설정에서 제거합니다.
    pub async fn clear_us_condition(
        &mut self,
        request: &Usa20291Request,
    ) -> Result<WsReply<Usa20291Response>, ClientError> {
        self.condition_change(request, (ConditionMarket::Us, request.seq.clone()), None)
            .await
    }

    /// ACK로 확인된 변경만 복구 설정에 반영합니다.
    /// 결과가 불확실한 변경은 재실행하지 않습니다.
    /// 새 입출력 전에 호출자가 명시적으로 복구해야 합니다.
    pub async fn subscribe(
        &mut self,
        request: &RealtimeSubscription,
    ) -> Result<WsReply<RegistrationAcknowledgement>, ClientError> {
        let proposed = self.configuration.proposed_feed_change(
            request,
            self.options.max_groups,
            self.options.max_entries,
        )?;
        if self.session.is_none() {
            return Err(ClientError::Protocol("WebSocket recovery required"));
        }
        self.begin(GapReason::SubscriptionUncertain)?;
        let mut session = self.session.take().ok_or(ClientError::Transport)?;
        let result = session.call(request).await.and_then(|reply| {
            ConfirmedConfiguration::confirmed_ack(&reply, request.group())?;
            Ok(reply)
        });
        match result {
            Ok(reply) if reply.acknowledged => {
                self.configuration.commit_feed_change(proposed);
                self.session = Some(session);
                self.pending = None;
                self.confirmed_generation = Some(self.generation);
                Ok(reply)
            }
            Ok(_) => {
                self.finish_failure();
                Err(ClientError::Protocol(
                    "unconfirmed subscription acknowledgement",
                ))
            }
            Err(error) => {
                self.finish_failure();
                Err(error)
            }
        }
    }

    /// 현재 세션의 대기 이벤트를 수신 순서로 꺼냅니다.
    pub fn pop_pending(&mut self) -> Option<Value> {
        self.session.as_mut().and_then(WsSession::pop_pending)
    }

    /// 수신 전에 세션을 분리해 실패·취소 이후의 불확실한 연결 재사용을 막습니다.
    pub async fn next_event(&mut self) -> Result<Value, ClientError> {
        if self.session.is_none() {
            return Err(ClientError::Protocol("WebSocket recovery required"));
        }
        self.begin(GapReason::ReceiveUncertain)?;
        let mut session = self.session.take().ok_or(ClientError::Transport)?;
        match session.next_event().await {
            Ok(event) => {
                self.session = Some(session);
                self.pending = None;
                self.confirmed_generation = Some(self.generation);
                Ok(event)
            }
            Err(error) => {
                self.finish_failure();
                Err(error)
            }
        }
    }

    /// 종료 작업이 실패·시간 초과·취소되더라도 로컬에서는 종료 상태를 유지합니다.
    pub async fn shutdown(&mut self) -> Result<(), ClientError> {
        if self.shutdown {
            return Ok(());
        }
        if let Some(pending) = self.pending.take() {
            self.gaps.push(pending);
        }
        self.shutdown = true;
        self.confirmed_generation = None;
        let session = self.session.take();
        if let Some(mut session) = session {
            session.close().await
        } else {
            Ok(())
        }
    }
}

// WS 복구가 사용하는 인증 어댑터는 소비자 쪽에 두어 인증 모듈의 WS 의존을 피합니다.
#[async_trait]
impl WsTokenProvider for TokenManager {
    async fn token(&self) -> Result<Secret, crate::ClientError> {
        self.lease()
            .await
            .map(|lease| lease.token().clone())
            .map_err(|_| crate::ClientError::Protocol("token acquisition failed"))
    }
}

#[async_trait]
impl WsTokenProvider for Arc<TokenManager> {
    async fn token(&self) -> Result<Secret, crate::ClientError> {
        WsTokenProvider::token(self.as_ref()).await
    }
}
