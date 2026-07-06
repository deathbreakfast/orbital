use serde::{Deserialize, Serialize};

use super::{HistoryFilter, HistorySort};

/// Serializable snapshot of timeline UI state for deep-linking and host persistence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistorySerializedState {
    pub filter: HistoryFilter,
    pub sort: HistorySort,
    /// 1-based page index when [`HistoryPagingMode::Paged`](crate::HistoryPagingMode::Paged) is active.
    pub page: Option<usize>,
    /// Scroll offset of the list container (px).
    pub scroll_top: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialized_state_round_trip() {
        use leptos::serde_json;
        let state = HistorySerializedState {
            filter: HistoryFilter {
                query: "jordan".into(),
                kinds: Some(vec!["comment".into()]),
                actor_ids: None,
            },
            sort: HistorySort::OldestFirst,
            page: Some(2),
            scroll_top: Some(120.0),
        };
        let json = serde_json::to_string(&state).expect("serialize");
        let restored: HistorySerializedState = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(state, restored);
    }
}
