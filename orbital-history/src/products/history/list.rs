use chrono::Utc;
use leptos::html::Div;
use leptos::prelude::*;

use crate::context::use_history_context;
use crate::engine::{
    apply_filter, apply_sort, compute_history_viewport, compute_variable_viewport,
    insert_unread_divider, list_item_cache_key, list_item_heights, project_entry_groups,
    DEFAULT_HISTORY_ROW_OVERSCAN, HISTORY_VIRTUALIZE_THRESHOLD,
};
use crate::format::{with_date_dividers_in_tz, with_date_dividers_on_list_items};
use crate::types::{HistoryEntry, HistoryFeatures, HistoryGroupBy, HistoryListItem};

use super::resize::use_scrollport_height;
use super::{HistoryDateDivider, HistoryEntryRow, HistoryGroupHeader, HistoryUnreadDivider};

/// Render a list of history entries with optional sort, filter, date dividers, and virtualization.
#[component]
pub fn HistoryEntryList(
    entries: Signal<Vec<HistoryEntry>>,
    /// When true, `entries` are already sort/filter projected (e.g. client paged window).
    #[prop(optional, default = false)]
    pre_projected: bool,
    /// Scrollport element for measured viewport height when virtualized.
    #[prop(optional)]
    scrollport: Option<NodeRef<Div>>,
) -> impl IntoView {
    let ctx = use_history_context();
    let has_scrollport = scrollport.is_some();
    let measure_ref = scrollport.unwrap_or_default();
    let measured_height = use_scrollport_height(measure_ref, 400.0);
    let viewport_height = Memo::new(move |_| {
        if has_scrollport {
            measured_height.get()
        } else {
            400.0
        }
    });

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

        let group_by = if ctx.features.contains(HistoryFeatures::GROUP_COLLAPSE) {
            ctx.group_by.get()
        } else {
            HistoryGroupBy::None
        };

        let mut items = if group_by != HistoryGroupBy::None {
            project_entry_groups(&list, group_by, &ctx.expanded_groups.get())
        } else if ctx.features.contains(HistoryFeatures::DATE_DIVIDERS) {
            let tz = ctx.display_timezone.get();
            with_date_dividers_in_tz(&list, Utc::now(), tz)
        } else {
            list.into_iter().map(HistoryListItem::Entry).collect()
        };

        if group_by != HistoryGroupBy::None && ctx.features.contains(HistoryFeatures::DATE_DIVIDERS)
        {
            let tz = ctx.display_timezone.get();
            items = with_date_dividers_on_list_items(items, Utc::now(), tz);
        }

        if let Some(wm) = ctx.read_watermark.get() {
            insert_unread_divider(
                items,
                wm,
                ctx.features.contains(HistoryFeatures::UNREAD_HIGHLIGHT),
            )
        } else {
            items
        }
    });

    Effect::new({
        let list_layout_keys = ctx.list_layout_keys;
        move |_| {
            let keys: Vec<_> = entry_items.get().iter().map(list_item_cache_key).collect();
            list_layout_keys.set(keys);
        }
    });

    let virtualize = Memo::new(move |_| {
        ctx.features.contains(HistoryFeatures::VIRTUALIZE)
            && entry_items.get().len() >= HISTORY_VIRTUALIZE_THRESHOLD
    });

    let row_height = ctx.virtual_row_height;
    let variable_height = Memo::new(move |_| {
        ctx.features.contains(HistoryFeatures::VARIABLE_ROW_HEIGHT) && virtualize.get()
    });

    let viewport = Memo::new(move |_| {
        let items = entry_items.get();
        if !virtualize.get() {
            return (items, 0.0, 0.0);
        }
        if variable_height.get() {
            let cache = ctx.row_height_cache.get();
            let heights = list_item_heights(&items, &cache, row_height);
            let vp = compute_variable_viewport(
                ctx.scroll_top.get(),
                viewport_height.get(),
                &heights,
                DEFAULT_HISTORY_ROW_OVERSCAN,
            );
            let slice = items[vp.start..vp.end].to_vec();
            return (slice, vp.padding_top_px, vp.padding_bottom_px);
        }
        let vp = compute_history_viewport(
            ctx.scroll_top.get(),
            viewport_height.get(),
            items.len(),
            row_height,
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
                key=|item| list_item_cache_key(item)
                children=move |item| match item {
                    HistoryListItem::Divider(bucket) => view! {
                        <HistoryDateDivider bucket=bucket />
                    }
                    .into_any(),
                    HistoryListItem::UnreadDivider => view! {
                        <HistoryUnreadDivider />
                    }
                    .into_any(),
                    HistoryListItem::GroupHeader {
                        key,
                        label,
                        child_count,
                        group_by,
                        ..
                    } => view! {
                        <HistoryGroupHeader
                            key=key
                            label=label
                            child_count=child_count
                            group_by=group_by
                        />
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
