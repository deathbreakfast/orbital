use leptos::html::Div;
use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::ScrollArea;
use orbital_macros::component_doc;
use orbital_paging::{use_paged_infinite_scroll, Page, PageRequest};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;

use crate::context::{use_history_context, HistoryContext};
use crate::engine::{
    merge_live_head, scroll_offset_for_index, HistoryRowHeightCache, DEFAULT_HISTORY_ROW_HEIGHT_PX,
};
use crate::products::history::list::project_entries;
use crate::types::{
    resolve_history_locale, HistoryChangeSlot, HistoryEmptyView, HistoryEndView, HistoryEntry,
    HistoryEntrySlot, HistoryErrorView, HistoryEvents, HistoryFeatures, HistoryFetchParams,
    HistoryFilter, HistoryFilterActorOption, HistoryGroupBy, HistoryHandle, HistoryHeader,
    HistoryLayout, HistoryLiveScrollPolicy, HistoryLoadingMoreView, HistoryLoadingView,
    HistoryLocale, HistoryPageFetcher, HistoryPaginationView, HistoryPagingMode, HistoryRenderers,
    HistorySerializedState, HistorySlots, HistorySort, HistorySource,
};
use leptos::context::Provider;

use super::scroll::{
    attach_scroll_top_listener, entry_in_dom, schedule_scroll_entry_into_view,
    scroll_container_to_offset, scroll_container_to_top,
};
use super::styles::{density_modifier_class, history_styles};
use super::{
    HistoryDefaultEmptyView, HistoryDefaultEndView, HistoryDefaultErrorView, HistoryDefaultHeader,
    HistoryDefaultLoadingMoreView, HistoryDefaultNoMatchesView, HistoryDefaultPagination,
    HistoryEntryList, HistoryTimelineSkeleton,
};

/// Shared InfiniteScroll state for handle hunt / refresh coordination.
#[derive(Clone)]
struct InfiniteScrollState {
    items: RwSignal<Vec<HistoryEntry>>,
    has_more: RwSignal<bool>,
    next_offset: RwSignal<u32>,
    loading: RwSignal<bool>,
    fetcher: HistoryPageFetcher,
    page_size: u32,
    filter: Signal<HistoryFilter>,
    sort: Signal<HistorySort>,
}

/// Scrollable audit timeline from a client signal or server page fetcher.
///
/// # Live updates
///
/// - **Client:** prepend or replace entries on the host `RwSignal`; the timeline reacts.
/// - **Server:** call [`HistoryHandle::refresh`] after the host's own subscription/poll, or pass
///   `live_head` / call [`HistoryHandle::prepend_live`] to merge newest rows without a full refetch.
///
/// Capture the handle via [`HistoryEvents::on_handle`].
///
/// # Examples
///
/// ## Client signal list
/// Newest-first field-diff entries in the default natural layout.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: Some("/users/u1".into()),
///         },
///         change: HistoryChange::FieldDiff {
///             field: "name".into(),
///             old_value: "Acme".into(),
///             new_value: "Acme Corp".into(),
///         },
///     },
///     HistoryEntry {
///         id: "2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::hours(3),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Draft note".into(),
///         },
///     },
/// ]);
/// view! {
///     <div data-testid="history-timeline-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-timeline",
    preview_label = "History timeline",
    preview_icon = icondata::LuHistory,
)]
#[component]
pub fn HistoryTimeline(
    data_source: HistorySource,
    #[prop(optional, default = HistoryLayout::Natural)] layout: HistoryLayout,
    #[prop(optional, default = HistoryFeatures::default_enabled())] features: HistoryFeatures,
    #[prop(optional)] locale: Option<HistoryLocale>,
    /// e.g. Some("320px"). None = flex-fill (`min-height: 0`) in parent.
    #[prop(optional)]
    max_height: Option<String>,
    #[prop(optional, default = HistoryPagingMode::InfiniteScroll)] paging: HistoryPagingMode,
    /// Host override for loading. When `None`, Server derives from the paging hook.
    #[prop(optional)]
    loading: Option<Signal<bool>>,
    /// Placeholder rows in the initial skeleton (default 5).
    #[prop(optional, default = 5)]
    skeleton_row_count: u32,
    /// Wall-clock timezone for date-bucket boundaries and compact timestamps. `None` uses UTC.
    #[prop(optional)]
    display_timezone: Option<Signal<DatetimeTimezone>>,
    /// Controlled filter. When omitted, use [`HistoryHandle::set_filter`].
    #[prop(optional)]
    filter: Option<Signal<HistoryFilter>>,
    /// Controlled sort (Client + `CLIENT_SORT`). When omitted, use [`HistoryHandle::set_sort`].
    #[prop(optional)]
    sort: Option<Signal<HistorySort>>,
    /// Max additional pages to fetch during `scroll_to_entry_or_load` (default 20).
    #[prop(optional, default = 20)]
    max_scroll_load_pages: u32,
    /// Page size for Client + [`HistoryPagingMode::Paged`] windowing (default 20).
    #[prop(optional, default = 20)]
    client_page_size: u32,
    /// Newest entries from live events, merged above Server pages (deduped by `id`). Ignored for Client source.
    #[prop(optional)]
    live_head: Option<Signal<Vec<HistoryEntry>>>,
    /// Kind options for built-in filter chrome chips.
    #[prop(optional)]
    filter_kinds: Option<Signal<Vec<String>>>,
    /// Actor options for built-in filter chrome chips.
    #[prop(optional)]
    filter_actors: Option<Signal<Vec<HistoryFilterActorOption>>>,
    /// Estimated row height for virtualized lists (default 72).
    #[prop(optional, default = DEFAULT_HISTORY_ROW_HEIGHT_PX as u32)]
    virtual_row_height: u32,
    /// Scroll behavior when Server live entries merge (default preserve offset).
    #[prop(optional, default = HistoryLiveScrollPolicy::Preserve)]
    live_scroll_policy: HistoryLiveScrollPolicy,
    /// Entries newer than this instant render as unread when `UNREAD_HIGHLIGHT` is enabled.
    #[prop(optional)]
    read_watermark: Option<RwSignal<Option<chrono::DateTime<Utc>>>>,
    /// Group consecutive entries by actor or kind when `GROUP_COLLAPSE` is enabled.
    #[prop(optional)]
    group_by: Option<Signal<HistoryGroupBy>>,
    #[prop(optional)] events: HistoryEvents,
    #[prop(optional)] renderers: Option<HistoryRenderers>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] history_header: Option<HistoryHeader>,
    #[prop(optional)] history_empty_view: Option<HistoryEmptyView>,
    #[prop(optional)] history_loading_view: Option<HistoryLoadingView>,
    #[prop(optional)] history_loading_more_view: Option<HistoryLoadingMoreView>,
    #[prop(optional)] history_error_view: Option<HistoryErrorView>,
    #[prop(optional)] history_end_view: Option<HistoryEndView>,
    #[prop(optional)] history_pagination_view: Option<HistoryPaginationView>,
    #[prop(optional)] history_entry_slot: Option<HistoryEntrySlot>,
    #[prop(optional)] history_change_slot: Option<HistoryChangeSlot>,
) -> impl IntoView {
    inject_style("orbital-history", history_styles());

    let slots = HistorySlots::from_slot_props(
        history_header,
        history_empty_view,
        history_loading_view,
        history_loading_more_view,
        history_error_view,
        history_end_view,
        history_pagination_view,
        history_entry_slot,
        history_change_slot,
    );

    let pagination_render = StoredValue::new(slots.pagination.map(|slot| slot.children));
    let header_slot = slots.header;
    let empty_slot = slots.empty;
    let loading_slot = slots.loading;
    let loading_more_slot = slots.loading_more;
    let error_slot = slots.error;
    let end_slot = slots.end;

    let mut merged_renderers = renderers.unwrap_or_default();
    if let Some(slot) = slots.entry_slot {
        merged_renderers.entry_view = Some(slot.render);
    }
    if let Some(slot) = slots.change_slot {
        merged_renderers.change_view = Some(slot.render);
    }

    let locale_signal = RwSignal::new(resolve_history_locale(locale));
    let display_timezone =
        display_timezone.unwrap_or_else(|| Signal::derive(|| DatetimeTimezone::Utc));

    let filter_controlled = filter.is_some();
    let internal_filter = RwSignal::new(HistoryFilter::default());
    let filter_signal: Signal<HistoryFilter> = filter.unwrap_or_else(|| internal_filter.into());

    let sort_controlled = sort.is_some();
    let internal_sort = RwSignal::new(HistorySort::NewestFirst);
    let sort_signal: Signal<HistorySort> = sort.unwrap_or_else(|| internal_sort.into());

    let live_head_controlled = live_head.is_some();
    let internal_live_head = RwSignal::new(Vec::<HistoryEntry>::new());
    let live_head_signal: Signal<Vec<HistoryEntry>> =
        live_head.unwrap_or_else(|| Signal::derive(move || internal_live_head.get()));

    let read_watermark_rw = read_watermark;
    let internal_read_watermark = RwSignal::new(None::<chrono::DateTime<Utc>>);
    let read_watermark_signal: Signal<Option<chrono::DateTime<Utc>>> = match read_watermark_rw {
        Some(rw) => Signal::derive(move || rw.get()),
        None => Signal::derive(move || internal_read_watermark.get()),
    };

    let group_by_signal: Signal<HistoryGroupBy> =
        group_by.unwrap_or_else(|| Signal::derive(|| HistoryGroupBy::None));
    let expanded_groups = RwSignal::new(HashSet::<String>::new());
    let toggle_group = Callback::new({
        let expanded_groups = expanded_groups;
        move |(key,): (String,)| {
            expanded_groups.update(|set| {
                if set.contains(&key) {
                    set.remove(&key);
                } else {
                    set.insert(key);
                }
            });
        }
    });

    let filter_kind_options: Signal<Vec<String>> =
        filter_kinds.unwrap_or_else(|| Signal::derive(Vec::new));
    let filter_actor_options: Signal<Vec<HistoryFilterActorOption>> =
        filter_actors.unwrap_or_else(|| Signal::derive(Vec::new));

    let row_height = virtual_row_height.max(1) as f64;
    let row_height_cache: HistoryRowHeightCache = RwSignal::new(std::collections::HashMap::new());
    let list_layout_keys = RwSignal::new(Vec::<String>::new());
    let is_paged = paging == HistoryPagingMode::Paged;
    let merged_entry_ids = RwSignal::new(Vec::<String>::new());
    let pending_scroll_restore = RwSignal::new(None::<f64>);

    let scroll_el = NodeRef::<Div>::new();
    let scroll_top = RwSignal::new(0.0);
    attach_scroll_top_listener(scroll_el, scroll_top);
    let refresh_trigger = RwSignal::new(0u32);
    let server_query_gen = RwSignal::new(0u32);
    // 1-based page for Pagination UI / Paged mode.
    let page_ui = RwSignal::new(1usize);
    let page_count = RwSignal::new(1usize);
    let is_server = data_source.is_server();
    let is_client = !is_server;
    let is_paged_server = is_server && paging == HistoryPagingMode::Paged;
    let is_paged_client = is_client && paging == HistoryPagingMode::Paged;
    let is_infinite = is_server && paging == HistoryPagingMode::InfiniteScroll;

    let infinite_state: StoredValue<Option<InfiniteScrollState>> = StoredValue::new(None);
    let hunt_generation = RwSignal::new(0u32);

    let handle = HistoryHandle {
        scroll_to_entry: Callback::new({
            let scroll_el = scroll_el;
            let merged_entry_ids = merged_entry_ids;
            let list_layout_keys = list_layout_keys;
            let row_height_cache = row_height_cache;
            let row_height = row_height;
            let features = features;
            move |(id,): (String,)| {
                if features.contains(HistoryFeatures::VIRTUALIZE) && !entry_in_dom(&id) {
                    let keys = list_layout_keys.get_untracked();
                    if let Some(idx) = keys.iter().position(|k| k == &id) {
                        let offset = if features.contains(HistoryFeatures::VARIABLE_ROW_HEIGHT) {
                            let cache = row_height_cache.get_untracked();
                            let heights: Vec<f64> = keys
                                .iter()
                                .map(|k| cache.get(k).copied().unwrap_or(row_height))
                                .collect();
                            scroll_offset_for_index(&heights, idx)
                        } else {
                            idx as f64 * row_height
                        };
                        scroll_container_to_offset(scroll_el, offset);
                    } else if let Some(idx) = merged_entry_ids
                        .get_untracked()
                        .iter()
                        .position(|x| x == &id)
                    {
                        scroll_container_to_offset(scroll_el, idx as f64 * row_height);
                    }
                }
                schedule_scroll_entry_into_view(id);
            }
        }),
        scroll_to_entry_or_load: Callback::new({
            let infinite_state = infinite_state;
            let page_ui = page_ui;
            let page_count = page_count;
            let live_head_signal = live_head_signal;
            move |(id,): (String,)| {
                if entry_in_dom(&id) {
                    schedule_scroll_entry_into_view(id.clone());
                    return;
                }
                if live_head_signal.get_untracked().iter().any(|e| e.id == id) {
                    schedule_scroll_entry_into_view(id.clone());
                    return;
                }
                if is_infinite {
                    let Some(state) = infinite_state.get_value() else {
                        schedule_scroll_entry_into_view(id);
                        return;
                    };
                    let gen = {
                        let next = hunt_generation.get_untracked().saturating_add(1);
                        hunt_generation.set(next);
                        next
                    };
                    let max_pages = max_scroll_load_pages;
                    leptos::task::spawn_local_scoped(async move {
                        let mut pages_loaded = 0u32;
                        loop {
                            if hunt_generation.get_untracked() != gen {
                                return;
                            }
                            if state.items.get_untracked().iter().any(|e| e.id == id)
                                || entry_in_dom(&id)
                            {
                                schedule_scroll_entry_into_view(id.clone());
                                return;
                            }
                            if !state.has_more.get_untracked() || pages_loaded >= max_pages {
                                return;
                            }
                            state.loading.set(true);
                            let offset = state.next_offset.get_untracked();
                            let result = (state.fetcher)(HistoryFetchParams::new(
                                PageRequest::new(offset, state.page_size),
                                state.filter.get_untracked(),
                                state.sort.get_untracked(),
                            ))
                            .await;
                            if hunt_generation.get_untracked() != gen {
                                state.loading.set(false);
                                return;
                            }
                            match result {
                                Ok(page) => {
                                    let len = page.items.len() as u32;
                                    state.items.update(|v| v.extend(page.items));
                                    state.has_more.set(page.has_more);
                                    state.next_offset.set(
                                        page.next_request_offset
                                            .unwrap_or(offset.saturating_add(len)),
                                    );
                                    pages_loaded += 1;
                                    state.loading.set(false);
                                }
                                Err(_) => {
                                    state.loading.set(false);
                                    state.has_more.set(false);
                                    return;
                                }
                            }
                        }
                    });
                    return;
                }
                if is_paged_server {
                    let gen = {
                        let next = hunt_generation.get_untracked().saturating_add(1);
                        hunt_generation.set(next);
                        next
                    };
                    let max_pages = max_scroll_load_pages;
                    leptos::task::spawn_local_scoped(async move {
                        for _ in 0..max_pages {
                            if hunt_generation.get_untracked() != gen {
                                return;
                            }
                            if entry_in_dom(&id) {
                                schedule_scroll_entry_into_view(id.clone());
                                return;
                            }
                            let current = page_ui.get_untracked();
                            let count = page_count.get_untracked().max(1);
                            if current >= count {
                                return;
                            }
                            page_ui.set(current.saturating_add(1));
                            for _ in 0..40 {
                                if hunt_generation.get_untracked() != gen {
                                    return;
                                }
                                if entry_in_dom(&id) {
                                    schedule_scroll_entry_into_view(id.clone());
                                    return;
                                }
                                #[cfg(feature = "hydrate")]
                                {
                                    gloo_timers::future::TimeoutFuture::new(50).await;
                                }
                                #[cfg(not(feature = "hydrate"))]
                                break;
                            }
                        }
                    });
                    return;
                }
                schedule_scroll_entry_into_view(id);
            }
        }),
        scroll_to_top: Callback::new({
            let scroll_el = scroll_el;
            move |_| {
                scroll_container_to_top(scroll_el);
            }
        }),
        refresh: Callback::new(move |_| {
            if is_server {
                if is_paged_server {
                    page_ui.set(1);
                }
                refresh_trigger.update(|n| *n += 1);
            }
        }),
        set_filter: Callback::new(move |(f,): (HistoryFilter,)| {
            if !filter_controlled {
                internal_filter.set(f);
            }
        }),
        set_sort: Callback::new(move |(s,): (HistorySort,)| {
            if is_client && !sort_controlled {
                internal_sort.set(s);
            }
        }),
        go_to_page: Callback::new(move |(page_0,): (usize,)| {
            if !is_paged_server && !is_paged_client {
                return;
            }
            let count = page_count.get_untracked().max(1);
            let clamped = page_0.min(count.saturating_sub(1));
            page_ui.set(clamped.saturating_add(1));
        }),
        prepend_live: Callback::new(move |(entries,): (Vec<HistoryEntry>,)| {
            if is_server && !live_head_controlled {
                internal_live_head.update(|list| {
                    for entry in entries {
                        if list.iter().any(|e| e.id == entry.id) {
                            continue;
                        }
                        list.insert(0, entry);
                    }
                });
            }
        }),
        export_state: Callback::new({
            move |_| HistorySerializedState {
                filter: filter_signal.get_untracked(),
                sort: sort_signal.get_untracked(),
                page: is_paged.then(|| page_ui.get_untracked()),
                scroll_top: Some(scroll_top.get_untracked()),
                read_watermark: read_watermark_signal.get_untracked(),
            }
        }),
        restore_state: Callback::new({
            move |(state,): (HistorySerializedState,)| {
                if !filter_controlled {
                    internal_filter.set(state.filter);
                }
                if is_client && !sort_controlled {
                    internal_sort.set(state.sort);
                }
                if let Some(rw) = read_watermark_rw {
                    rw.set(state.read_watermark);
                } else {
                    internal_read_watermark.set(state.read_watermark);
                }
                if is_paged {
                    if let Some(page) = state.page {
                        page_ui.set(page.max(1));
                    }
                }
                if is_server {
                    refresh_trigger.update(|n| *n += 1);
                }
                if let Some(top) = state.scroll_top {
                    pending_scroll_restore.set(Some(top));
                }
            }
        }),
        set_read_watermark: Callback::new(move |(wm,): (chrono::DateTime<Utc>,)| {
            if let Some(rw) = read_watermark_rw {
                rw.set(Some(wm));
            } else {
                internal_read_watermark.set(Some(wm));
            }
        }),
        mark_all_read: Callback::new(move |_| {
            let now = Some(Utc::now());
            if let Some(rw) = read_watermark_rw {
                rw.set(now);
            } else {
                internal_read_watermark.set(now);
            }
        }),
        expand_group: Callback::new({
            let expanded_groups = expanded_groups;
            move |(key,): (String,)| {
                expanded_groups.update(|set| {
                    set.insert(key);
                });
            }
        }),
        collapse_group: Callback::new({
            let expanded_groups = expanded_groups;
            move |(key,): (String,)| {
                expanded_groups.update(|set| {
                    set.remove(&key);
                });
            }
        }),
        expand_all_groups: Callback::new({
            let expanded_groups = expanded_groups;
            let list_layout_keys = list_layout_keys;
            move |_| {
                expanded_groups.update(|set| {
                    for key in list_layout_keys.get_untracked() {
                        if let Some(group_key) = key.strip_prefix("group-") {
                            set.insert(group_key.to_string());
                        }
                    }
                });
            }
        }),
    };

    let handle_delivered = StoredValue::new(false);
    Effect::new({
        let events = events.clone();
        let handle = handle.clone();
        move |_| {
            if handle_delivered.get_value() {
                return;
            }
            handle_delivered.set_value(true);
            events.notify_handle(handle.clone());
        }
    });

    let history_ctx = HistoryContext {
        locale: locale_signal.into(),
        features,
        layout,
        events: events.clone(),
        renderers: merged_renderers,
        display_timezone,
        filter: filter_signal,
        sort: sort_signal,
        is_client,
        set_filter: handle.set_filter.clone(),
        set_sort: handle.set_sort.clone(),
        scroll_top: scroll_top.into(),
        filter_kind_options,
        filter_actor_options,
        virtual_row_height: row_height,
        read_watermark: read_watermark_signal,
        row_height_cache,
        list_layout_keys,
        group_by: group_by_signal,
        expanded_groups,
        toggle_group,
        page: is_paged.then(|| Signal::derive(move || page_ui.get())),
        page_count: is_paged.then(|| Signal::derive(move || page_count.get())),
        go_to_page: is_paged.then(|| handle.go_to_page.clone()),
    };

    Effect::new({
        let scroll_el = scroll_el;
        move |_| {
            if let Some(top) = pending_scroll_restore.get() {
                scroll_container_to_offset(scroll_el, top);
                pending_scroll_restore.set(None);
            }
        }
    });

    Effect::new({
        let skip_initial = StoredValue::new(true);
        move |_| {
            if !is_server {
                return;
            }
            if !(features.contains(HistoryFeatures::SERVER_FILTER)
                || features.contains(HistoryFeatures::SERVER_SORT))
            {
                return;
            }
            let _ = filter_signal.get();
            let _ = sort_signal.get();
            if skip_initial.get_value() {
                skip_initial.set_value(false);
                return;
            }
            server_query_gen.update(|n| *n += 1);
            if is_paged_server {
                page_ui.set(1);
            }
            refresh_trigger.update(|n| *n += 1);
        }
    });

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());

    let root_class = Memo::new(move |_| {
        let mut classes = vec!["orbital-history".to_string()];
        let density = density_class.get();
        if !density.is_empty() {
            classes.push(density);
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    });

    let scroll_style = max_height.map(|h| format!("max-height:{h};"));

    let header_view = move || {
        if let Some(slot) = &header_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultHeader /> }.into_any()
        }
    };

    match data_source {
        HistorySource::Client(items) => {
            let entries = Signal::derive(move || items.get());
            view! {
                <Provider value=history_ctx>
                    <div class=move || root_class.get() data-orbital-history data-testid="history-timeline">
                        {header_view()}
                        <HistoryClientPanel
                            entries=entries
                            paging=paging
                            client_page_size=client_page_size
                            page_ui=page_ui
                            page_count=page_count
                            loading=loading
                            skeleton_row_count=skeleton_row_count
                            scroll_style=scroll_style
                            scroll_el=scroll_el
                            empty_slot=empty_slot
                            loading_slot=loading_slot
                            pagination_render=pagination_render
                            merged_entry_ids=merged_entry_ids
                        />
                    </div>
                </Provider>
            }
            .into_any()
        }
        HistorySource::Server { fetcher, page_size } => view! {
            <Provider value=history_ctx>
                <div class=move || root_class.get() data-orbital-history data-testid="history-timeline">
                    {header_view()}
                    <HistoryServerPanel
                    fetcher=fetcher
                    page_size=page_size
                    paging=paging
                    features=features
                    filter_signal=filter_signal
                    sort_signal=sort_signal
                    server_query_gen=server_query_gen
                    loading=loading
                    skeleton_row_count=skeleton_row_count
                    scroll_style=scroll_style
                    scroll_el=scroll_el
                    refresh_trigger=refresh_trigger
                    page_ui=page_ui
                    page_count=page_count
                    infinite_state=infinite_state
                    live_head_signal=live_head_signal
                    live_scroll_policy=live_scroll_policy
                    scroll_top=scroll_top
                    events=events
                    empty_slot=empty_slot
                    loading_slot=loading_slot
                    loading_more_slot=loading_more_slot
                    error_slot=error_slot
                    end_slot=end_slot
                    pagination_render=pagination_render
                    merged_entry_ids=merged_entry_ids
                />
                </div>
            </Provider>
        }
        .into_any(),
    }
}

#[component]
fn HistoryClientPanel(
    entries: Signal<Vec<HistoryEntry>>,
    paging: HistoryPagingMode,
    client_page_size: u32,
    page_ui: RwSignal<usize>,
    page_count: RwSignal<usize>,
    loading: Option<Signal<bool>>,
    skeleton_row_count: u32,
    scroll_style: Option<String>,
    scroll_el: NodeRef<Div>,
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
    pagination_render: StoredValue<Option<ChildrenFn>>,
    merged_entry_ids: RwSignal<Vec<String>>,
) -> impl IntoView {
    let ctx = use_history_context();
    let is_paged = paging == HistoryPagingMode::Paged;
    let page_size = client_page_size.max(1) as usize;

    let is_loading = Memo::new(move |_| loading.map(|s| s.get()).unwrap_or(false));
    let source_empty = Memo::new(move |_| entries.get().is_empty());

    let projected_all = Memo::new(move |_| {
        project_entries(
            &entries.get(),
            true,
            ctx.features,
            ctx.sort.get(),
            &ctx.filter.get(),
            &ctx.locale.get(),
        )
    });

    Effect::new({
        move |_| {
            if !is_paged {
                return;
            }
            let total = projected_all.get().len();
            let pages = total.div_ceil(page_size).max(1);
            page_count.set(pages);
            if page_ui.get_untracked() > pages {
                page_ui.set(pages);
            }
        }
    });

    let windowed = Memo::new(move |_| {
        let all = projected_all.get();
        if !is_paged {
            return all;
        }
        let p0 = page_ui.get().saturating_sub(1);
        let start = p0 * page_size;
        all.into_iter().skip(start).take(page_size).collect()
    });

    let projected_empty = Memo::new(move |_| windowed.get().is_empty());
    let filter_active = Memo::new(move |_| ctx.filter.get().is_active());

    let show_initial = Memo::new(move |_| is_loading.get() && source_empty.get());
    let show_empty = Memo::new(move |_| !is_loading.get() && source_empty.get());
    let show_no_matches = Memo::new(move |_| {
        !is_loading.get() && !source_empty.get() && projected_empty.get() && filter_active.get()
    });
    let show_list = Memo::new(move |_| !projected_empty.get());

    let display_entries = Signal::derive(move || windowed.get());

    Effect::new({
        move |_| {
            let ids: Vec<_> = display_entries
                .get()
                .iter()
                .map(|entry| entry.id.clone())
                .collect();
            merged_entry_ids.set(ids);
        }
    });

    let pagination_view = move || {
        if let Some(render) = pagination_render.get_value().as_ref() {
            render().into_any()
        } else {
            view! {
                <HistoryDefaultPagination
                    page=page_ui
                    page_count=Signal::derive(move || page_count.get())
                />
            }
            .into_any()
        }
    };

    let loading_view = move || {
        if let Some(slot) = &loading_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryTimelineSkeleton row_count=skeleton_row_count /> }.into_any()
        }
    };

    let empty_view = move || {
        if let Some(slot) = &empty_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultEmptyView /> }.into_any()
        }
    };

    view! {
        <div class="orbital-history__client-panel" style="display:flex;flex-direction:column;min-height:0;flex:1;">
            <ScrollArea
                class="orbital-history__scroll".to_string()
                style=scroll_style.unwrap_or_default()
                node_ref=scroll_el
            >
                <Show when=move || show_initial.get() fallback=|| ()>
                    {loading_view()}
                </Show>
                <Show when=move || show_empty.get() fallback=|| ()>
                    {empty_view()}
                </Show>
                <Show when=move || show_no_matches.get() fallback=|| ()>
                    <HistoryDefaultNoMatchesView />
                </Show>
                <Show when=move || show_list.get() fallback=|| ()>
                    <HistoryEntryList
                        entries=display_entries
                        pre_projected=true
                        scrollport=scroll_el
                    />
                </Show>
            </ScrollArea>
            <Show when=move || is_paged fallback=|| ()>
                {pagination_view()}
            </Show>
        </div>
    }
}

#[component]
fn HistoryServerPanel(
    fetcher: HistoryPageFetcher,
    page_size: u32,
    paging: HistoryPagingMode,
    features: HistoryFeatures,
    filter_signal: Signal<HistoryFilter>,
    sort_signal: Signal<HistorySort>,
    server_query_gen: RwSignal<u32>,
    loading: Option<Signal<bool>>,
    skeleton_row_count: u32,
    scroll_style: Option<String>,
    scroll_el: NodeRef<Div>,
    refresh_trigger: RwSignal<u32>,
    page_ui: RwSignal<usize>,
    page_count: RwSignal<usize>,
    infinite_state: StoredValue<Option<InfiniteScrollState>>,
    live_head_signal: Signal<Vec<HistoryEntry>>,
    live_scroll_policy: HistoryLiveScrollPolicy,
    scroll_top: RwSignal<f64>,
    events: HistoryEvents,
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
    loading_more_slot: Option<HistoryLoadingMoreView>,
    error_slot: Option<HistoryErrorView>,
    end_slot: Option<HistoryEndView>,
    pagination_render: StoredValue<Option<ChildrenFn>>,
    merged_entry_ids: RwSignal<Vec<String>>,
) -> impl IntoView {
    let ctx = use_history_context();
    let load_error = RwSignal::new(false);
    let on_load_error = events.on_load_error.clone();

    let wrap_fetch = {
        let fetcher = fetcher.clone();
        move |req: PageRequest| {
            let fetcher = fetcher.clone();
            let filter = if features.contains(HistoryFeatures::SERVER_FILTER) {
                filter_signal.get_untracked()
            } else {
                HistoryFilter::default()
            };
            let sort = if features.contains(HistoryFeatures::SERVER_SORT) {
                sort_signal.get_untracked()
            } else {
                HistorySort::NewestFirst
            };
            async move { (fetcher)(HistoryFetchParams::new(req, filter, sort)).await }
        }
    };

    let fetch = wrap_fetch.clone();

    let (entries, hook_loading, ever_loaded, has_more, show_error, show_pagination) = match paging {
        HistoryPagingMode::InfiniteScroll => {
            let hook =
                use_paged_infinite_scroll(scroll_el, page_size, refresh_trigger.into(), fetch);
            infinite_state.set_value(Some(InfiniteScrollState {
                items: hook.items,
                has_more: hook.has_more,
                next_offset: hook.next_request_offset,
                loading: hook.loading,
                fetcher: {
                    let fetcher = fetcher.clone();
                    Arc::new(move |params: HistoryFetchParams| {
                        let fetcher = fetcher.clone();
                        Box::pin((fetcher)(params))
                            as Pin<
                                Box<
                                    dyn Future<Output = Result<Page<HistoryEntry>, ServerFnError>>
                                        + Send,
                                >,
                            >
                    })
                },
                page_size,
                filter: filter_signal,
                sort: sort_signal,
            }));
            (
                hook.items,
                Signal::derive(move || hook.loading.get()),
                Signal::derive(move || hook.ever_loaded.get()),
                Signal::derive(move || hook.has_more.get()),
                Signal::derive(move || load_error.get()),
                false,
            )
        }
        HistoryPagingMode::None => {
            let items = RwSignal::new(Vec::<HistoryEntry>::new());
            let loading_sig = RwSignal::new(true);
            let ever = RwSignal::new(false);
            let has_more_sig = RwSignal::new(false);
            let fetcher = fetcher.clone();
            let first_page = Resource::new(
                move || (refresh_trigger.get(), server_query_gen.get()),
                move |_| {
                    let fetcher = fetcher.clone();
                    let filter = if features.contains(HistoryFeatures::SERVER_FILTER) {
                        filter_signal.get_untracked()
                    } else {
                        HistoryFilter::default()
                    };
                    let sort = if features.contains(HistoryFeatures::SERVER_SORT) {
                        sort_signal.get_untracked()
                    } else {
                        HistorySort::NewestFirst
                    };
                    async move {
                        (fetcher)(HistoryFetchParams::new(
                            PageRequest::new(0, page_size),
                            filter,
                            sort,
                        ))
                        .await
                    }
                },
            );
            Effect::new(move |_| {
                loading_sig.set(first_page.get().is_none());
                match first_page.get() {
                    Some(Ok(page)) => {
                        items.set(page.items.clone());
                        has_more_sig.set(page.has_more);
                        ever.set(true);
                        load_error.set(false);
                    }
                    Some(Err(err)) => {
                        ever.set(true);
                        load_error.set(true);
                        if let Some(cb) = &on_load_error {
                            cb.run(err);
                        }
                    }
                    None => {}
                }
            });
            (
                items,
                Signal::derive(move || loading_sig.get()),
                Signal::derive(move || ever.get()),
                Signal::derive(move || has_more_sig.get()),
                Signal::derive(move || load_error.get()),
                false,
            )
        }
        HistoryPagingMode::Paged => {
            let items = RwSignal::new(Vec::<HistoryEntry>::new());
            let loading_sig = RwSignal::new(true);
            let ever = RwSignal::new(false);
            let has_more_sig = RwSignal::new(false);
            let fetcher = fetcher.clone();
            let page_resource = Resource::new(
                move || (refresh_trigger.get(), page_ui.get(), server_query_gen.get()),
                move |(_, page_1, _)| {
                    let fetcher = fetcher.clone();
                    let page_0 = page_1.saturating_sub(1) as u32;
                    let offset = page_0.saturating_mul(page_size);
                    let filter = if features.contains(HistoryFeatures::SERVER_FILTER) {
                        filter_signal.get_untracked()
                    } else {
                        HistoryFilter::default()
                    };
                    let sort = if features.contains(HistoryFeatures::SERVER_SORT) {
                        sort_signal.get_untracked()
                    } else {
                        HistorySort::NewestFirst
                    };
                    async move {
                        (fetcher)(HistoryFetchParams::new(
                            PageRequest::new(offset, page_size),
                            filter,
                            sort,
                        ))
                        .await
                    }
                },
            );
            Effect::new(move |_| {
                let had_items = !items.get_untracked().is_empty() || ever.get_untracked();
                loading_sig.set(page_resource.get().is_none());
                match page_resource.get() {
                    Some(Ok(page)) => {
                        items.set(page.items.clone());
                        has_more_sig.set(page.has_more);
                        ever.set(true);
                        load_error.set(false);
                        if let Some(total) = page.total_count {
                            let count = ((total as u32).div_ceil(page_size)).max(1) as usize;
                            page_count.set(count);
                        } else {
                            let current = page_ui.get_untracked().max(1);
                            page_count.set(if page.has_more {
                                current.saturating_add(1)
                            } else {
                                current
                            });
                        }
                        let _ = had_items;
                    }
                    Some(Err(err)) => {
                        ever.set(true);
                        load_error.set(true);
                        if let Some(cb) = &on_load_error {
                            cb.run(err);
                        }
                    }
                    None => {}
                }
            });
            (
                items,
                Signal::derive(move || loading_sig.get()),
                Signal::derive(move || ever.get()),
                Signal::derive(move || has_more_sig.get()),
                Signal::derive(move || load_error.get()),
                true,
            )
        }
    };

    let is_loading = Memo::new(move |_| {
        loading
            .map(|s| s.get())
            .unwrap_or_else(|| hook_loading.get())
    });

    let entry_signal = Signal::derive(move || entries.get());
    let merged_signal =
        Signal::derive(move || merge_live_head(&entries.get(), &live_head_signal.get()));
    let source_has_data =
        Memo::new(move |_| !entry_signal.get().is_empty() || !live_head_signal.get().is_empty());
    let source_empty = Memo::new(move |_| !source_has_data.get());
    let projected = Memo::new(move |_| {
        project_entries(
            &merged_signal.get(),
            false,
            ctx.features,
            ctx.sort.get(),
            &ctx.filter.get(),
            &ctx.locale.get(),
        )
    });

    Effect::new({
        move |_| {
            let ids: Vec<_> = merged_signal
                .get()
                .iter()
                .map(|entry| entry.id.clone())
                .collect();
            merged_entry_ids.set(ids);
        }
    });
    let projected_empty = Memo::new(move |_| projected.get().is_empty());
    let filter_active = Memo::new(move |_| ctx.filter.get().is_active());

    let show_initial =
        Memo::new(move |_| is_loading.get() && source_empty.get() && !ever_loaded.get());
    let show_incremental =
        Memo::new(move |_| is_loading.get() && (ever_loaded.get() || !source_empty.get()));
    let show_empty = Memo::new(move |_| {
        !is_loading.get()
            && source_empty.get()
            && ever_loaded.get()
            && !show_error.get()
            && !filter_active.get()
    });
    let show_no_matches = Memo::new(move |_| {
        !is_loading.get()
            && source_has_data.get()
            && projected_empty.get()
            && filter_active.get()
            && !show_error.get()
    });
    let show_list = Memo::new(move |_| !projected_empty.get());
    let show_end = Memo::new(move |_| {
        ever_loaded.get()
            && !has_more.get()
            && !source_empty.get()
            && !is_loading.get()
            && !show_pagination
    });

    let loading_view = move || {
        if let Some(slot) = &loading_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryTimelineSkeleton row_count=skeleton_row_count /> }.into_any()
        }
    };
    let loading_more_view = move || {
        if let Some(slot) = &loading_more_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultLoadingMoreView /> }.into_any()
        }
    };
    let empty_view = move || {
        if let Some(slot) = &empty_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultEmptyView /> }.into_any()
        }
    };
    let error_view = move || {
        if let Some(slot) = &error_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultErrorView /> }.into_any()
        }
    };
    let end_view = move || {
        if let Some(slot) = &end_slot {
            (slot.children)().into_any()
        } else {
            view! { <HistoryDefaultEndView /> }.into_any()
        }
    };

    let pagination_view = move || {
        if let Some(render) = pagination_render.get_value().as_ref() {
            render().into_any()
        } else {
            view! {
                <HistoryDefaultPagination
                    page=page_ui
                    page_count=Signal::derive(move || page_count.get())
                />
            }
            .into_any()
        }
    };

    let prev_live_len = StoredValue::new(0usize);
    Effect::new({
        let scroll_el = scroll_el;
        move |_| {
            let live = live_head_signal.get();
            let prev = prev_live_len.get_value();
            if live.len() > prev
                && live_scroll_policy.should_scroll_on_live_update(scroll_top.get_untracked())
            {
                match live_scroll_policy {
                    HistoryLiveScrollPolicy::ScrollToTop
                    | HistoryLiveScrollPolicy::ScrollIfNearTop { .. } => {
                        scroll_container_to_top(scroll_el);
                    }
                    HistoryLiveScrollPolicy::ScrollToFirstNew => {
                        if let Some(entry) = live.first() {
                            schedule_scroll_entry_into_view(entry.id.clone());
                        }
                    }
                    HistoryLiveScrollPolicy::Preserve => {}
                }
            }
            prev_live_len.set_value(live.len());
        }
    });

    view! {
        <div class="orbital-history__server-panel" style="display:flex;flex-direction:column;min-height:0;flex:1;">
            <ScrollArea
                class="orbital-history__scroll".to_string()
                style=scroll_style.unwrap_or_default()
                node_ref=scroll_el
            >
                <Show when=move || show_error.get() fallback=|| ()>
                    {error_view()}
                </Show>
                <Show when=move || show_initial.get() fallback=|| ()>
                    {loading_view()}
                </Show>
                <Show when=move || show_empty.get() fallback=|| ()>
                    {empty_view()}
                </Show>
                <Show when=move || show_no_matches.get() fallback=|| ()>
                    <HistoryDefaultNoMatchesView />
                </Show>
                <Show when=move || show_list.get() fallback=|| ()>
                    <HistoryEntryList entries=merged_signal scrollport=scroll_el />
                </Show>
                <Show when=move || show_incremental.get() fallback=|| ()>
                    {loading_more_view()}
                </Show>
                <Show when=move || show_end.get() fallback=|| ()>
                    {end_view()}
                </Show>
            </ScrollArea>
            <Show when=move || show_pagination fallback=|| ()>
                {pagination_view()}
            </Show>
        </div>
    }
}
