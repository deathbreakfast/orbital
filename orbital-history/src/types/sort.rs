use serde::{Deserialize, Serialize};

/// Client-side sort order (only when [`HistoryFeatures::CLIENT_SORT`](crate::HistoryFeatures::CLIENT_SORT) is enabled).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistorySort {
    #[default]
    NewestFirst,
    OldestFirst,
}
