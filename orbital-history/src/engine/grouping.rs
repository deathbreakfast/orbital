//! Consecutive entry grouping for collapse UI.

use std::collections::HashSet;

use crate::types::{HistoryEntry, HistoryGroupBy, HistoryListItem};

fn entry_group_key(entry: &HistoryEntry, group_by: HistoryGroupBy) -> Option<String> {
    match group_by {
        HistoryGroupBy::None => None,
        HistoryGroupBy::Actor => match &entry.actor {
            crate::types::HistoryActor::System => Some("system".into()),
            crate::types::HistoryActor::User { id, .. } => Some(format!("actor:{id}")),
        },
        HistoryGroupBy::Kind => Some(format!("kind:{}", entry.kind)),
    }
}

fn group_label(entry: &HistoryEntry, group_by: HistoryGroupBy) -> String {
    match group_by {
        HistoryGroupBy::Actor => match &entry.actor {
            crate::types::HistoryActor::System => "System".into(),
            crate::types::HistoryActor::User { display_name, .. } => display_name.clone(),
        },
        HistoryGroupBy::Kind => entry.kind.clone(),
        HistoryGroupBy::None => String::new(),
    }
}

/// Collapse consecutive entries sharing the same actor or kind.
pub fn project_entry_groups(
    entries: &[HistoryEntry],
    group_by: HistoryGroupBy,
    expanded: &HashSet<String>,
) -> Vec<HistoryListItem> {
    if group_by == HistoryGroupBy::None || entries.is_empty() {
        return entries.iter().cloned().map(HistoryListItem::Entry).collect();
    }

    let mut out = Vec::new();
    let mut index = 0usize;
    while index < entries.len() {
        let Some(key) = entry_group_key(&entries[index], group_by) else {
            out.push(HistoryListItem::Entry(entries[index].clone()));
            index += 1;
            continue;
        };
        let label = group_label(&entries[index], group_by);
        let mut end = index + 1;
        while end < entries.len() {
            if entry_group_key(&entries[end], group_by) != Some(key.clone()) {
                break;
            }
            end += 1;
        }
        let child_count = end - index;
        if child_count > 1 {
            out.push(HistoryListItem::GroupHeader {
                key: key.clone(),
                label,
                child_count,
                group_by,
                changed_at: entries[index].changed_at,
            });
            if expanded.contains(&key) {
                out.extend(
                    entries[index..end]
                        .iter()
                        .cloned()
                        .map(HistoryListItem::Entry),
                );
            }
        } else {
            out.push(HistoryListItem::Entry(entries[index].clone()));
        }
        index = end;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    use crate::types::{HistoryActor, HistoryChange, HistoryEntry};

    fn entry(id: &str, kind: &str, actor_id: &str) -> HistoryEntry {
        HistoryEntry {
            id: id.into(),
            kind: kind.into(),
            changed_at: Utc::now(),
            actor: HistoryActor::User {
                id: actor_id.into(),
                display_name: format!("User {actor_id}"),
                href: None,
            },
            change: HistoryChange::Created,
        }
    }

    #[test]
    fn collapses_consecutive_same_actor() {
        let entries = vec![
            entry("1", "created", "a"),
            entry("2", "updated", "a"),
            entry("3", "created", "b"),
        ];
        let items = project_entry_groups(&entries, HistoryGroupBy::Actor, &HashSet::new());
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0], HistoryListItem::GroupHeader { child_count: 2, .. }));
        assert!(matches!(items[1], HistoryListItem::Entry(_)));
    }

    #[test]
    fn expanded_group_shows_children() {
        let entries = vec![entry("1", "created", "a"), entry("2", "updated", "a")];
        let mut expanded = HashSet::new();
        expanded.insert("actor:a".into());
        let items = project_entry_groups(&entries, HistoryGroupBy::Actor, &expanded);
        assert_eq!(items.len(), 3);
    }
}
