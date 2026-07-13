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

/// Projected list item: section divider, unread divider, group header, or entry.
#[derive(Clone, Debug, PartialEq)]
pub enum HistoryListItem {
    Divider(HistoryDateBucket),
    UnreadDivider,
    GroupHeader {
        key: String,
        label: String,
        child_count: usize,
        group_by: super::HistoryGroupBy,
        changed_at: chrono::DateTime<chrono::Utc>,
    },
    Entry(HistoryEntry),
}
