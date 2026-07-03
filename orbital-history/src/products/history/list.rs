use chrono::Utc;
use leptos::prelude::*;

use crate::context::use_history_context;
use crate::format::with_date_dividers;
use crate::types::{HistoryEntry, HistoryFeatures, HistoryListItem};

use super::{HistoryDateDivider, HistoryEntryRow};

/// Render a list of history entries with optional date dividers.
#[component]
pub fn HistoryEntryList(entries: Signal<Vec<HistoryEntry>>) -> impl IntoView {
    let ctx = use_history_context();

    let items = Memo::new(move |_| {
        let entries = entries.get();
        if ctx.features.contains(HistoryFeatures::DATE_DIVIDERS) {
            with_date_dividers(&entries, Utc::now())
        } else {
            entries
                .into_iter()
                .map(HistoryListItem::Entry)
                .collect::<Vec<_>>()
        }
    });

    view! {
        <ul class="orbital-history__list" role="list" data-testid="history-entry-list">
            <For
                each=move || items.get()
                key=|item| match item {
                    HistoryListItem::Divider(b) => format!("divider-{b:?}"),
                    HistoryListItem::Entry(e) => e.id.clone(),
                }
                children=move |item| match item {
                    HistoryListItem::Divider(bucket) => view! {
                        <HistoryDateDivider bucket=bucket />
                    }
                    .into_any(),
                    HistoryListItem::Entry(entry) => view! {
                        <HistoryEntryRow entry=entry />
                    }
                    .into_any(),
                }
            />
        </ul>
    }
}
