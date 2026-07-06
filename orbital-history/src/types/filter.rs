use serde::{Deserialize, Serialize};

/// One selectable actor for filter chrome (`id` + display label).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryFilterActorOption {
    pub id: String,
    pub label: String,
}

/// Client-side filter over currently loaded history entries.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryFilter {
    /// Case-insensitive substring match against actor display name, kind, and change summary.
    pub query: String,
    /// When `Some` and non-empty, entry `kind` must be in the set.
    pub kinds: Option<Vec<String>>,
    /// When `Some` and non-empty, `HistoryActor::User { id }` must be in the set.
    /// `System` actors never match a non-empty `actor_ids` filter.
    pub actor_ids: Option<Vec<String>>,
}

impl HistoryFilter {
    pub fn is_active(&self) -> bool {
        !self.query.trim().is_empty()
            || self.kinds.as_ref().is_some_and(|k| !k.is_empty())
            || self.actor_ids.as_ref().is_some_and(|a| !a.is_empty())
    }
}
