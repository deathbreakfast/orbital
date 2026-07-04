use chrono::Utc;
use leptos::prelude::*;

use crate::context::use_history_context;
use crate::engine::{apply_filter, apply_sort};
use crate::format::with_date_dividers_in_tz;
use crate::types::{HistoryEntry, HistoryFeatures, HistoryListItem};

use super::{HistoryDateDivider, HistoryEntryRow};

/// Render a list of history entries with optional sort, filter, and date dividers.
#[component]
pub fn HistoryEntryList(entries: Signal<Vec<HistoryEntry>>) -> impl IntoView {
    let ctx = use_history_context();

    let items = Memo::new(move |_| {
        let mut list = entries.get();
        let locale = ctx.locale.get();
        if ctx.is_client && ctx.features.contains(HistoryFeatures::CLIENT_SORT) {
            list = apply_sort(&list, ctx.sort.get());
        }
        list = apply_filter(&list, &ctx.filter.get(), &locale);
        if ctx.features.contains(HistoryFeatures::DATE_DIVIDERS) {
            let tz = ctx.display_timezone.get();
            with_date_dividers_in_tz(&list, Utc::now(), tz)
        } else {
            list.into_iter().map(HistoryListItem::Entry).collect()
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

/// Project source entries through sort + filter (no dividers). Used for empty-state selection.
pub fn project_entries(
    entries: &[HistoryEntry],
    is_client: bool,
    features: HistoryFeatures,
    sort: crate::types::HistorySort,
    filter: &crate::types::HistoryFilter,
    locale: &crate::types::HistoryLocale,
) -> Vec<HistoryEntry> {
    let mut list = entries.to_vec();
    if is_client && features.contains(HistoryFeatures::CLIENT_SORT) {
        list = apply_sort(&list, sort);
    }
    apply_filter(&list, filter, locale)
}
