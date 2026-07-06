use chrono::Utc;
use leptos::prelude::*;

use crate::context::use_history_context;
use crate::engine::{
    apply_filter, apply_sort, compute_history_viewport, DEFAULT_HISTORY_ROW_HEIGHT_PX,
    DEFAULT_HISTORY_ROW_OVERSCAN, HISTORY_VIRTUALIZE_THRESHOLD,
};
use crate::format::with_date_dividers_in_tz;
use crate::types::{HistoryEntry, HistoryFeatures, HistoryListItem};

use super::{HistoryDateDivider, HistoryEntryRow};

/// Render a list of history entries with optional sort, filter, date dividers, and virtualization.
#[component]
pub fn HistoryEntryList(
    entries: Signal<Vec<HistoryEntry>>,
    /// When true, `entries` are already sort/filter projected (e.g. client paged window).
    #[prop(optional, default = false)]
    pre_projected: bool,
) -> impl IntoView {
    let ctx = use_history_context();

    let entry_items = Memo::new(move |_| {
        let mut list = entries.get();
        let locale = ctx.locale.get();
        if !pre_projected {
            let server_filter =
                !ctx.is_client && ctx.features.contains(HistoryFeatures::SERVER_FILTER);
            if ctx.is_client && ctx.features.contains(HistoryFeatures::CLIENT_SORT) {
                list = apply_sort(&list, ctx.sort.get());
            }
            if !server_filter {
                list = apply_filter(&list, &ctx.filter.get(), &locale);
            }
        }
        if ctx.features.contains(HistoryFeatures::DATE_DIVIDERS) {
            let tz = ctx.display_timezone.get();
            with_date_dividers_in_tz(&list, Utc::now(), tz)
        } else {
            list.into_iter().map(HistoryListItem::Entry).collect()
        }
    });

    let virtualize = Memo::new(move |_| {
        ctx.features.contains(HistoryFeatures::VIRTUALIZE)
            && entry_items.get().len() >= HISTORY_VIRTUALIZE_THRESHOLD
    });

    let viewport = Memo::new(move |_| {
        let items = entry_items.get();
        if !virtualize.get() {
            return (items, 0.0, 0.0);
        }
        let vp = compute_history_viewport(
            ctx.scroll_top.get(),
            400.0,
            items.len(),
            DEFAULT_HISTORY_ROW_HEIGHT_PX,
            DEFAULT_HISTORY_ROW_OVERSCAN,
        );
        let slice = items[vp.start..vp.end].to_vec();
        (slice, vp.padding_top_px, vp.padding_bottom_px)
    });

    view! {
        <ul class="orbital-history__list" role="list" data-testid="history-entry-list">
            <Show when=move || virtualize.get() && viewport.with(|(_, top, _)| *top > 0.0) fallback=|| ()>
                <li
                    class="orbital-history__virtual-spacer"
                    aria-hidden="true"
                    style=move || format!("height: {}px; list-style: none;", viewport.with(|(_, top, _)| *top))
                />
            </Show>
            <For
                each=move || viewport.with(|(items, _, _)| items.clone())
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
            <Show when=move || virtualize.get() && viewport.with(|(_, _, bottom)| *bottom > 0.0) fallback=|| ()>
                <li
                    class="orbital-history__virtual-spacer"
                    aria-hidden="true"
                    style=move || format!("height: {}px; list-style: none;", viewport.with(|(_, _, bottom)| *bottom))
                />
            </Show>
        </ul>
    }
}

/// Project source entries through sort + filter (no dividers). Used for empty-state selection and paging.
pub fn project_entries(
    entries: &[HistoryEntry],
    is_client: bool,
    features: HistoryFeatures,
    sort: crate::types::HistorySort,
    filter: &crate::types::HistoryFilter,
    locale: &crate::types::HistoryLocale,
) -> Vec<HistoryEntry> {
    let mut list = entries.to_vec();
    let server_filter = !is_client && features.contains(HistoryFeatures::SERVER_FILTER);
    if is_client && features.contains(HistoryFeatures::CLIENT_SORT) {
        list = apply_sort(&list, sort);
    }
    if !server_filter {
        list = apply_filter(&list, filter, locale);
    }
    list
}
