//! Pure list projection (sort / filter / live merge / unread).

mod grouping;
mod live_head;
mod row_heights;
mod unread;
mod viewport;

pub use grouping::*;
pub use live_head::*;
pub use row_heights::*;
pub use unread::*;
pub use viewport::*;

use crate::format::format_change;
use crate::types::{HistoryActor, HistoryEntry, HistoryFilter, HistoryLocale, HistorySort};

/// Sort entries by `changed_at` (stable by `id` on ties).
pub fn apply_sort(entries: &[HistoryEntry], sort: HistorySort) -> Vec<HistoryEntry> {
    let mut out = entries.to_vec();
    out.sort_by(|a, b| {
        let ord = match sort {
            HistorySort::NewestFirst => b.changed_at.cmp(&a.changed_at),
            HistorySort::OldestFirst => a.changed_at.cmp(&b.changed_at),
        };
        ord.then_with(|| a.id.cmp(&b.id))
    });
    out
}

/// Filter loaded entries by query / kind / actor id.
pub fn apply_filter(
    entries: &[HistoryEntry],
    filter: &HistoryFilter,
    locale: &HistoryLocale,
) -> Vec<HistoryEntry> {
    if !filter.is_active() {
        return entries.to_vec();
    }

    let query = filter.query.trim().to_lowercase();
    let kinds = filter.kinds.as_ref().filter(|k| !k.is_empty());
    let actor_ids = filter.actor_ids.as_ref().filter(|a| !a.is_empty());

    entries
        .iter()
        .filter(|entry| {
            if let Some(kinds) = kinds {
                if !kinds.iter().any(|k| k == &entry.kind) {
                    return false;
                }
            }
            if let Some(actor_ids) = actor_ids {
                match &entry.actor {
                    HistoryActor::System => return false,
                    HistoryActor::User { id, .. } => {
                        if !actor_ids.iter().any(|a| a == id) {
                            return false;
                        }
                    }
                }
            }
            if query.is_empty() {
                return true;
            }
            let actor_name = match &entry.actor {
                HistoryActor::System => locale.system_actor.as_str(),
                HistoryActor::User { display_name, .. } => display_name.as_str(),
            };
            let summary = format_change(&entry.change, locale);
            actor_name.to_lowercase().contains(&query)
                || entry.kind.to_lowercase().contains(&query)
                || summary.to_lowercase().contains(&query)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    use crate::types::{HistoryActor, HistoryChange};

    fn entry(id: &str, kind: &str, actor: HistoryActor, change: HistoryChange) -> HistoryEntry {
        HistoryEntry {
            id: id.into(),
            kind: kind.into(),
            changed_at: Utc.with_ymd_and_hms(2026, 7, 3, 12, 0, 0).unwrap(),
            actor,
            change,
        }
    }

    #[test]
    fn filter_by_query_matches_actor_and_kind() {
        let locale = HistoryLocale::english();
        let entries = vec![
            entry(
                "1",
                "field_diff",
                HistoryActor::User {
                    id: "u1".into(),
                    display_name: "Jordan Lee".into(),
                    href: None,
                },
                HistoryChange::Created,
            ),
            entry(
                "2",
                "comment",
                HistoryActor::System,
                HistoryChange::Custom {
                    summary: "hello".into(),
                },
            ),
        ];
        let filtered = apply_filter(
            &entries,
            &HistoryFilter {
                query: "jordan".into(),
                ..Default::default()
            },
            &locale,
        );
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "1");
    }

    #[test]
    fn filter_by_kinds_and_actor_ids() {
        let locale = HistoryLocale::english();
        let entries = vec![
            entry(
                "1",
                "field_diff",
                HistoryActor::User {
                    id: "u1".into(),
                    display_name: "Jordan Lee".into(),
                    href: None,
                },
                HistoryChange::Created,
            ),
            entry(
                "2",
                "comment",
                HistoryActor::User {
                    id: "u2".into(),
                    display_name: "Sam Rivera".into(),
                    href: None,
                },
                HistoryChange::Created,
            ),
        ];
        let filtered = apply_filter(
            &entries,
            &HistoryFilter {
                query: String::new(),
                kinds: Some(vec!["field_diff".into()]),
                actor_ids: Some(vec!["u1".into()]),
            },
            &locale,
        );
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "1");
    }

    #[test]
    fn sort_oldest_first() {
        let mut a = entry("a", "x", HistoryActor::System, HistoryChange::Created);
        let mut b = entry("b", "x", HistoryActor::System, HistoryChange::Created);
        a.changed_at = Utc.with_ymd_and_hms(2026, 7, 1, 0, 0, 0).unwrap();
        b.changed_at = Utc.with_ymd_and_hms(2026, 7, 3, 0, 0, 0).unwrap();
        let sorted = apply_sort(&[b.clone(), a.clone()], HistorySort::OldestFirst);
        assert_eq!(sorted[0].id, "a");
        assert_eq!(sorted[1].id, "b");
    }
}
