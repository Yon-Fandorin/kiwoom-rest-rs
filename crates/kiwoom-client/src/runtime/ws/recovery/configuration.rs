//! 확인된 설정의 제안·한도·ACK 검증만 소유하며 연결·시계·공백을 다루지 않습니다.

use crate::{
    ClientError,
    domain::ws::conditions::{Ka10173Request, Usa20290Request},
    domain::ws::subscription::{
        RealtimeSubscription, RefreshMode, RegistrationAcknowledgement, SubscriptionAction,
        SubscriptionEntry,
    },
    runtime::ws::session::{WsReply, WsRequest},
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ConditionMarket {
    Domestic,
    Us,
}

#[derive(Clone)]
pub(super) enum ConditionSubscription {
    Domestic(Ka10173Request),
    Us(Usa20290Request),
}

#[derive(Default)]
pub(super) struct ConfirmedConfiguration {
    feeds: BTreeMap<String, Vec<SubscriptionEntry>>,
    conditions: BTreeMap<(ConditionMarket, String), ConditionSubscription>,
}

impl ConfirmedConfiguration {
    pub(super) fn group_count(&self) -> usize {
        self.feeds.len()
    }

    pub(super) fn entry_count(&self) -> usize {
        self.feeds.values().map(Vec::len).sum()
    }

    pub(super) fn condition_count(&self) -> usize {
        self.conditions.len()
    }

    pub(super) fn feed_groups(&self) -> impl Iterator<Item = (&String, &Vec<SubscriptionEntry>)> {
        self.feeds.iter()
    }

    pub(super) fn condition_subscriptions(
        &self,
    ) -> impl Iterator<Item = (&(ConditionMarket, String), &ConditionSubscription)> {
        self.conditions.iter()
    }

    pub(super) fn proposed_feed_change(
        &self,
        request: &RealtimeSubscription,
        max_groups: usize,
        max_entries: usize,
    ) -> Result<BTreeMap<String, Vec<SubscriptionEntry>>, ClientError> {
        request.validate()?;
        let mut proposed = self.feeds.clone();
        let group = request.group().to_owned();
        let entries = request.atomic_entries();
        match request.action() {
            SubscriptionAction::Register => {
                let existing = proposed.entry(group).or_default();
                if matches!(request.refresh_mode(), RefreshMode::ReplaceGroup) {
                    existing.clear();
                }
                for entry in entries {
                    let key = serde_json::to_value(&entry).map_err(|_| {
                        ClientError::InvalidRequest("subscription serialization failed")
                    })?;
                    if !existing
                        .iter()
                        .any(|e| serde_json::to_value(e).ok().as_ref() == Some(&key))
                    {
                        existing.push(entry);
                    }
                }
            }
            SubscriptionAction::Remove => {
                // refresh=0 + REMOVE나 와일드카드 제거의 의미를 임의로 만들지 않습니다.
                // 확인 응답을 받은 정확한 항목만 처리합니다.
                if matches!(request.refresh_mode(), RefreshMode::ReplaceGroup) {
                    return Err(ClientError::InvalidRequest(
                        "recovery REMOVE requires keep-group mode",
                    ));
                }
                let existing = proposed
                    .get_mut(&group)
                    .ok_or(ClientError::InvalidRequest("unknown subscription removal"))?;
                for entry in entries {
                    let key = serde_json::to_value(&entry).map_err(|_| {
                        ClientError::InvalidRequest("subscription serialization failed")
                    })?;
                    let index = existing
                        .iter()
                        .position(|e| serde_json::to_value(e).ok().as_ref() == Some(&key))
                        .ok_or(ClientError::InvalidRequest("unknown subscription removal"))?;
                    existing.remove(index);
                }
                if existing.is_empty() {
                    proposed.remove(&group);
                }
            }
        }
        if proposed.len() > max_groups
            || proposed.values().map(Vec::len).sum::<usize>() + self.conditions.len() > max_entries
        {
            return Err(ClientError::InvalidRequest(
                "subscription recovery capacity exceeded",
            ));
        }
        Ok(proposed)
    }

    pub(super) fn commit_feed_change(
        &mut self,
        proposed: BTreeMap<String, Vec<SubscriptionEntry>>,
    ) {
        self.feeds = proposed;
    }

    pub(super) fn check_domestic_exchange(
        &self,
        request: &Ka10173Request,
        key: &(ConditionMarket, String),
    ) -> Result<(), ClientError> {
        if let Some(ConditionSubscription::Domestic(existing)) = self.conditions.get(key)
            && existing.stex_tp != request.stex_tp
        {
            return Err(ClientError::InvalidRequest(
                "clear domestic condition before changing exchange",
            ));
        }
        Ok(())
    }

    pub(super) fn check_condition_change<Q: WsRequest>(
        &self,
        request: &Q,
        key: &(ConditionMarket, String),
        adding: bool,
        max_entries: usize,
    ) -> Result<(), ClientError> {
        request.validate()?;
        if !adding && !self.conditions.contains_key(key) {
            return Err(ClientError::InvalidRequest("unknown condition removal"));
        }
        if adding
            && !self.conditions.contains_key(key)
            && self.entry_count() + self.conditions.len() >= max_entries
        {
            return Err(ClientError::InvalidRequest(
                "subscription recovery capacity exceeded",
            ));
        }
        Ok(())
    }

    pub(super) fn commit_condition_change(
        &mut self,
        key: (ConditionMarket, String),
        change: Option<ConditionSubscription>,
    ) {
        if let Some(change) = change {
            self.conditions.insert(key, change);
        } else {
            self.conditions.remove(&key);
        }
    }

    pub(super) fn confirmed_ack(
        reply: &WsReply<RegistrationAcknowledgement>,
        group: &str,
    ) -> Result<(), ClientError> {
        if !reply.acknowledged {
            return Err(ClientError::Protocol(
                "unconfirmed subscription acknowledgement",
            ));
        }
        if reply
            .raw
            .get("grp_no")
            .is_some_and(|v| v.as_str() != Some(group))
        {
            return Err(ClientError::Protocol(
                "subscription acknowledgement group mismatch",
            ));
        }
        Ok(())
    }

    pub(super) fn condition_ack<R>(reply: &WsReply<R>, seq: &str) -> Result<(), ClientError> {
        if !reply.acknowledged {
            return Err(ClientError::Protocol(
                "unconfirmed condition acknowledgement",
            ));
        }
        if reply
            .raw
            .get("seq")
            .is_some_and(|v| v.as_str() != Some(seq))
        {
            return Err(ClientError::Protocol(
                "condition acknowledgement sequence mismatch",
            ));
        }
        Ok(())
    }
}
