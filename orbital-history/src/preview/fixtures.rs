use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::{Duration, Utc};
use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

use crate::types::{
    HistoryActor, HistoryChange, HistoryEntry, HistoryFieldDiff, HistoryPageFetcher,
};

/// Small newest-first field-diff list for client previews.
pub fn sample_entries() -> Vec<HistoryEntry> {
    let now = Utc::now();
    vec![
        HistoryEntry {
            id: "1".into(),
            kind: "field_diff".into(),
            changed_at: now - Duration::minutes(15),
            actor: HistoryActor::User {
                id: "u1".into(),
                display_name: "Jordan Lee".into(),
                href: Some("/users/u1".into()),
            },
            change: HistoryChange::FieldDiff {
                field: "name".into(),
                old_value: "Acme".into(),
                new_value: "Acme Corp".into(),
            },
        },
        HistoryEntry {
            id: "2".into(),
            kind: "created".into(),
            changed_at: now - Duration::hours(3),
            actor: HistoryActor::System,
            change: HistoryChange::Created,
        },
        HistoryEntry {
            id: "3".into(),
            kind: "deleted".into(),
            changed_at: now - Duration::days(1),
            actor: HistoryActor::User {
                id: "u2".into(),
                display_name: "Sam Rivera".into(),
                href: None,
            },
            change: HistoryChange::Deleted {
                label: "Draft note".into(),
            },
        },
    ]
}

pub fn empty_entries() -> Vec<HistoryEntry> {
    vec![]
}

pub fn long_value_entries() -> Vec<HistoryEntry> {
    let now = Utc::now();
    let long = "x".repeat(120);
    vec![HistoryEntry {
        id: "long-1".into(),
        kind: "field_diff".into(),
        changed_at: now,
        actor: HistoryActor::System,
        change: HistoryChange::FieldDiff {
            field: "description".into(),
            old_value: long.clone(),
            new_value: format!("{long}-new"),
        },
    }]
}

pub fn multi_kind_entries() -> Vec<HistoryEntry> {
    let mut entries = sample_entries();
    entries.insert(
        0,
        HistoryEntry {
            id: "comment-1".into(),
            kind: "comment".into(),
            changed_at: Utc::now(),
            actor: HistoryActor::User {
                id: "u1".into(),
                display_name: "Jordan Lee".into(),
                href: None,
            },
            change: HistoryChange::Custom {
                summary: "Left a comment".into(),
            },
        },
    );
    entries
}

pub fn multi_diff_entries() -> Vec<HistoryEntry> {
    vec![HistoryEntry {
        id: "md-1".into(),
        kind: "field_diffs".into(),
        changed_at: Utc::now(),
        actor: HistoryActor::User {
            id: "u1".into(),
            display_name: "Jordan Lee".into(),
            href: None,
        },
        change: HistoryChange::FieldDiffs {
            fields: vec![
                HistoryFieldDiff {
                    field: "status".into(),
                    old_value: "open".into(),
                    new_value: "closed".into(),
                },
                HistoryFieldDiff {
                    field: "owner".into(),
                    old_value: "A".into(),
                    new_value: "B".into(),
                },
            ],
        },
    }]
}

/// Entries spanning Today, Yesterday, Last 7 days, Last 30 days, Older.
pub fn bucket_span_entries() -> Vec<HistoryEntry> {
    let now = Utc::now();
    vec![
        entry("b1", now, HistoryChange::Created),
        entry("b2", now - Duration::days(1), HistoryChange::Created),
        entry("b3", now - Duration::days(4), HistoryChange::Created),
        entry("b4", now - Duration::days(12), HistoryChange::Created),
        entry("b5", now - Duration::days(40), HistoryChange::Created),
    ]
}

fn entry(id: &str, at: chrono::DateTime<Utc>, change: HistoryChange) -> HistoryEntry {
    HistoryEntry {
        id: id.into(),
        kind: "created".into(),
        changed_at: at,
        actor: HistoryActor::System,
        change,
    }
}

fn all_fixture_pages() -> Vec<HistoryEntry> {
    let mut entries = sample_entries();
    entries.extend(bucket_span_entries());
    for i in 0..30 {
        entries.push(HistoryEntry {
            id: format!("page-{i}"),
            kind: "field_diff".into(),
            changed_at: Utc::now() - Duration::minutes(i as i64 * 10),
            actor: HistoryActor::System,
            change: HistoryChange::FieldDiff {
                field: "counter".into(),
                old_value: format!("{i}"),
                new_value: format!("{}", i + 1),
            },
        });
    }
    entries
}

async fn mock_paged_history(request: PageRequest) -> Result<Page<HistoryEntry>, ServerFnError> {
    let all = all_fixture_pages();
    let offset = request.offset as usize;
    let limit = request.limit as usize;
    let slice: Vec<_> = all.iter().skip(offset).take(limit).cloned().collect();
    let next = offset + slice.len();
    let has_more = next < all.len();
    Ok(Page {
        items: slice,
        has_more,
        total_count: Some(all.len() as u64),
        next_request_offset: has_more.then_some(next as u32),
    })
}

/// Mock page fetcher that pages fixture data (newest-first).
pub fn mock_page_fetcher() -> HistoryPageFetcher {
    Arc::new(|request: PageRequest| {
        Box::pin(mock_paged_history(request))
            as Pin<Box<dyn Future<Output = Result<Page<HistoryEntry>, ServerFnError>> + Send>>
    })
}
