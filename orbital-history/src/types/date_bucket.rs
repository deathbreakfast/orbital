use super::HistoryEntry;

/// Relative date section for dividers (UTC calendar days).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HistoryDateBucket {
    Today,
    Yesterday,
    Last7Days,
    Last30Days,
    Older,
}

/// Projected list item: section divider or entry.
#[derive(Clone, Debug, PartialEq)]
pub enum HistoryListItem {
    Divider(HistoryDateBucket),
    Entry(HistoryEntry),
}
