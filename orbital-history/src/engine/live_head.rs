//! Merge host-supplied live entries above server-fetched pages.

use std::collections::HashSet;

use crate::types::HistoryEntry;

/// Prepend `live` entries above `fetched`, dropping duplicates by `id`.
///
/// Entries in `live` that already appear in `fetched` are skipped. Duplicate ids
/// within `live` keep the first occurrence only.
pub fn merge_live_head(fetched: &[HistoryEntry], live: &[HistoryEntry]) -> Vec<HistoryEntry> {
    if live.is_empty() {
        return fetched.to_vec();
    }

    let fetched_ids: HashSet<&str> = fetched.iter().map(|e| e.id.as_str()).collect();
    let mut seen = fetched_ids;
    let mut merged = Vec::with_capacity(live.len() + fetched.len());

    for entry in live {
        if seen.insert(entry.id.as_str()) {
            merged.push(entry.clone());
        }
    }
    merged.extend_from_slice(fetched);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    use crate::types::{HistoryActor, HistoryChange};

    fn entry(id: &str) -> HistoryEntry {
        HistoryEntry {
            id: id.into(),
            kind: "created".into(),
            changed_at: Utc::now(),
            actor: HistoryActor::System,
            change: HistoryChange::Created,
        }
    }

    #[test]
    fn prepends_live_and_dedupes_against_fetched() {
        let fetched = vec![entry("a"), entry("b")];
        let live = vec![entry("live"), entry("a"), entry("c")];
        let merged = merge_live_head(&fetched, &live);
        assert_eq!(merged.len(), 4);
        assert_eq!(merged[0].id, "live");
        assert_eq!(merged[1].id, "c");
        assert_eq!(merged[2].id, "a");
        assert_eq!(merged[3].id, "b");
    }

    #[test]
    fn empty_live_returns_fetched_clone() {
        let fetched = vec![entry("a")];
        let merged = merge_live_head(&fetched, &[]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].id, "a");
    }
}
