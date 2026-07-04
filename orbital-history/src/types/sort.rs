/// Client-side sort order (only when [`HistoryFeatures::CLIENT_SORT`](crate::HistoryFeatures::CLIENT_SORT) is enabled).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistorySort {
    #[default]
    NewestFirst,
    OldestFirst,
}
