//! Unread entry projection from a read watermark.

use chrono::{DateTime, Utc};

use crate::types::{HistoryEntry, HistoryListItem};

/// Returns true when the entry is unread relative to `watermark`.
pub fn is_entry_unread(entry: &HistoryEntry, watermark: DateTime<Utc>) -> bool {
    entry.changed_at > watermark
}

/// Insert an unread divider before the first unread entry in a projected list.
pub fn insert_unread_divider(
    items: Vec<HistoryListItem>,
    watermark: DateTime<Utc>,
    show_divider: bool,
) -> Vec<HistoryListItem> {
    if !show_divider {
        return items;
    }
    let mut out = Vec::with_capacity(items.len() + 1);
    let mut divider_inserted = false;
    for item in items {
        if !divider_inserted {
            if let HistoryListItem::Entry(entry) = &item {
                if is_entry_unread(entry, watermark) {
                    out.push(HistoryListItem::UnreadDivider);
                    divider_inserted = true;
                }
            }
        }
        out.push(item);
    }
    out
}

/// Insert an unread divider before the first unread entry when enabled.
pub fn with_unread_divider(
    entries: &[HistoryEntry],
    watermark: DateTime<Utc>,
    show_divider: bool,
) -> Vec<HistoryListItem> {
    if !show_divider {
        return entries.iter().cloned().map(HistoryListItem::Entry).collect();
    }

    let mut out = Vec::with_capacity(entries.len() + 1);
    let mut divider_inserted = false;

    for entry in entries {
        if !divider_inserted && is_entry_unread(entry, watermark) {
            out.push(HistoryListItem::UnreadDivider);
            divider_inserted = true;
        }
        out.push(HistoryListItem::Entry(entry.clone()));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    use crate::types::{HistoryActor, HistoryChange};

    fn entry(id: &str, at: DateTime<Utc>) -> HistoryEntry {
        HistoryEntry {
            id: id.into(),
            kind: "created".into(),
            changed_at: at,
            actor: HistoryActor::System,
            change: HistoryChange::Created,
        }
    }

    #[test]
    fn unread_divider_before_first_unread() {
        let wm = Utc.with_ymd_and_hms(2026, 7, 1, 12, 0, 0).unwrap();
        let old = entry("a", wm - chrono::Duration::hours(1));
        let new = entry("b", wm + chrono::Duration::hours(1));
        let items = with_unread_divider(&[old, new], wm, true);
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], HistoryListItem::Entry(_)));
        assert!(matches!(items[1], HistoryListItem::UnreadDivider));
        assert!(matches!(items[2], HistoryListItem::Entry(_)));
    }
}
