use leptos::html::Div;
use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::ScrollArea;
use orbital_macros::component_doc;
use orbital_paging::{use_paged_infinite_scroll, PageRequest};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::context::{provide_history_context, use_history_context, HistoryContext};
use crate::products::history::list::project_entries;
use crate::types::{
    resolve_history_locale, HistoryChangeSlot, HistoryEmptyView, HistoryEndView, HistoryEntry,
    HistoryEntrySlot, HistoryErrorView, HistoryEvents, HistoryFeatures, HistoryFilter,
    HistoryHandle, HistoryHeader, HistoryLoadingMoreView, HistoryLoadingView, HistoryLocale,
    HistoryOrientation, HistoryPageFetcher, HistoryPagingMode, HistoryRenderers, HistorySlots,
    HistorySort, HistorySource,
};

use super::scroll::{
    entry_in_dom, schedule_scroll_entry_into_view, scroll_container_to_top,
};
use super::styles::{density_modifier_class, history_styles};
use super::{
    HistoryDefaultEmptyView, HistoryDefaultEndView, HistoryDefaultErrorView,
    HistoryDefaultHeader, HistoryDefaultLoadingMoreView, HistoryDefaultNoMatchesView,
    HistoryDefaultPagination, HistoryEntryList, HistoryTimelineSkeleton,
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
}

/// Scrollable audit timeline from a client signal or server page fetcher.
///
/// # Live updates
///
/// - **Client:** prepend or replace entries on the host `RwSignal`; the timeline reacts.
/// - **Server:** call [`HistoryHandle::refresh`] after the host's own subscription/poll.
///
/// Capture the handle via [`HistoryEvents::on_handle`].
///
/// # Examples
///
/// ## Client signal list
/// Newest-first field-diff entries in the default vertical orientation.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(sample_entries());
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
    #[prop(optional, default = HistoryOrientation::Vertical)] orientation: HistoryOrientation,
    #[prop(optional, default = HistoryFeatures::default_enabled())] features: HistoryFeatures,
    #[prop(optional)] locale: Option<HistoryLocale>,
    /// e.g. Some("320px"). None = flex-fill (`min-height: 0`) in parent.
    #[prop(optional)] max_height: Option<String>,
    #[prop(optional, default = HistoryPagingMode::InfiniteScroll)] paging: HistoryPagingMode,
    /// Host override for loading. When `None`, Server derives from the paging hook.
    #[prop(optional)] loading: Option<Signal<bool>>,
    /// Placeholder rows in the initial skeleton (default 5).
    #[prop(optional, default = 5)] skeleton_row_count: u32,
    /// Wall-clock timezone for date-bucket boundaries and compact timestamps. `None` uses UTC.
    #[prop(optional)] display_timezone: Option<Signal<DatetimeTimezone>>,
    /// Controlled filter. When omitted, use [`HistoryHandle::set_filter`].
    #[prop(optional)] filter: Option<Signal<HistoryFilter>>,
    /// Controlled sort (Client + `CLIENT_SORT`). When omitted, use [`HistoryHandle::set_sort`].
    #[prop(optional)] sort: Option<Signal<HistorySort>>,
    /// Max additional pages to fetch during `scroll_to_entry_or_load` (default 20).
    #[prop(optional, default = 20)] max_scroll_load_pages: u32,
    #[prop(optional)] events: HistoryEvents,
    #[prop(optional)] renderers: Option<HistoryRenderers>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] history_header: Option<HistoryHeader>,
    #[prop(optional)] history_empty_view: Option<HistoryEmptyView>,
    #[prop(optional)] history_loading_view: Option<HistoryLoadingView>,
    #[prop(optional)] history_loading_more_view: Option<HistoryLoadingMoreView>,
    #[prop(optional)] history_error_view: Option<HistoryErrorView>,
    #[prop(optional)] history_end_view: Option<HistoryEndView>,
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
        history_entry_slot,
        history_change_slot,
    );

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

    let scroll_el = NodeRef::<Div>::new();
    let refresh_trigger = RwSignal::new(0u32);
    // 1-based page for Pagination UI / Paged mode.
    let page_ui = RwSignal::new(1usize);
    let page_count = RwSignal::new(1usize);
    let is_server = data_source.is_server();
    let is_client = !is_server;
    let is_paged = is_server && paging == HistoryPagingMode::Paged;
    let is_infinite = is_server && paging == HistoryPagingMode::InfiniteScroll;

    let infinite_state: StoredValue<Option<InfiniteScrollState>> = StoredValue::new(None);
    let hunt_generation = RwSignal::new(0u32);

    let handle = HistoryHandle {
        scroll_to_entry: Callback::new(|(id,): (String,)| {
            schedule_scroll_entry_into_view(id);
        }),
        scroll_to_entry_or_load: Callback::new({
            let infinite_state = infinite_state;
            move |(id,): (String,)| {
                if entry_in_dom(&id) || !is_infinite {
                    schedule_scroll_entry_into_view(id.clone());
                    return;
                }
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
                leptos::task::spawn_local(async move {
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
                        let result =
                            (state.fetcher)(PageRequest::new(offset, state.page_size)).await;
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
                if is_paged {
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
            if !is_paged {
                return;
            }
            let count = page_count.get_untracked().max(1);
            let clamped = page_0.min(count.saturating_sub(1));
            page_ui.set(clamped.saturating_add(1));
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

    provide_history_context(HistoryContext {
        locale: locale_signal.into(),
        features,
        orientation,
        events: events.clone(),
        renderers: merged_renderers,
        display_timezone,
        filter: filter_signal,
        sort: sort_signal,
        is_client,
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
                <div class=move || root_class.get() data-orbital-history data-testid="history-timeline">
                    {header_view()}
                    <HistoryClientPanel
                        entries=entries
                        loading=loading
                        skeleton_row_count=skeleton_row_count
                        scroll_style=scroll_style
                        scroll_el=scroll_el
                        empty_slot=empty_slot
                        loading_slot=loading_slot
                    />
                </div>
            }
            .into_any()
        }
        HistorySource::Server { fetcher, page_size } => view! {
            <div class=move || root_class.get() data-orbital-history data-testid="history-timeline">
                {header_view()}
                <HistoryServerPanel
                    fetcher=fetcher
                    page_size=page_size
                    paging=paging
                    loading=loading
                    skeleton_row_count=skeleton_row_count
                    scroll_style=scroll_style
                    scroll_el=scroll_el
                    refresh_trigger=refresh_trigger
                    page_ui=page_ui
                    page_count=page_count
                    infinite_state=infinite_state
                    events=events
                    empty_slot=empty_slot
                    loading_slot=loading_slot
                    loading_more_slot=loading_more_slot
                    error_slot=error_slot
                    end_slot=end_slot
                />
            </div>
        }
        .into_any(),
    }
}

#[component]
fn HistoryClientPanel(
    entries: Signal<Vec<HistoryEntry>>,
    loading: Option<Signal<bool>>,
    skeleton_row_count: u32,
    scroll_style: Option<String>,
    scroll_el: NodeRef<Div>,
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
) -> impl IntoView {
    let ctx = use_history_context();
    let is_loading = Memo::new(move |_| loading.map(|s| s.get()).unwrap_or(false));
    let source_empty = Memo::new(move |_| entries.get().is_empty());
    let projected = Memo::new(move |_| {
        project_entries(
            &entries.get(),
            true,
            ctx.features,
            ctx.sort.get(),
            &ctx.filter.get(),
            &ctx.locale.get(),
        )
    });
    let projected_empty = Memo::new(move |_| projected.get().is_empty());
    let filter_active = Memo::new(move |_| ctx.filter.get().is_active());

    let show_initial = Memo::new(move |_| is_loading.get() && source_empty.get());
    let show_empty = Memo::new(move |_| {
        !is_loading.get() && source_empty.get()
    });
    let show_no_matches = Memo::new(move |_| {
        !is_loading.get() && !source_empty.get() && projected_empty.get() && filter_active.get()
    });
    let show_list = Memo::new(move |_| !projected_empty.get());

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
                <HistoryEntryList entries=entries />
            </Show>
        </ScrollArea>
    }
}

#[component]
fn HistoryServerPanel(
    fetcher: HistoryPageFetcher,
    page_size: u32,
    paging: HistoryPagingMode,
    loading: Option<Signal<bool>>,
    skeleton_row_count: u32,
    scroll_style: Option<String>,
    scroll_el: NodeRef<Div>,
    refresh_trigger: RwSignal<u32>,
    page_ui: RwSignal<usize>,
    page_count: RwSignal<usize>,
    infinite_state: StoredValue<Option<InfiniteScrollState>>,
    events: HistoryEvents,
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
    loading_more_slot: Option<HistoryLoadingMoreView>,
    error_slot: Option<HistoryErrorView>,
    end_slot: Option<HistoryEndView>,
) -> impl IntoView {
    let ctx = use_history_context();
    let load_error = RwSignal::new(false);
    let on_load_error = events.on_load_error.clone();

    let fetch = {
        let fetcher = fetcher.clone();
        move |req: PageRequest| {
            let fetcher = fetcher.clone();
            async move { (fetcher)(req).await }
        }
    };

    let (entries, hook_loading, ever_loaded, has_more, show_error, show_pagination) = match paging {
        HistoryPagingMode::InfiniteScroll => {
            let hook =
                use_paged_infinite_scroll(scroll_el, page_size, refresh_trigger.into(), fetch);
            infinite_state.set_value(Some(InfiniteScrollState {
                items: hook.items,
                has_more: hook.has_more,
                next_offset: hook.next_request_offset,
                loading: hook.loading,
                fetcher: fetcher.clone(),
                page_size,
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
                move || refresh_trigger.get(),
                move |_| {
                    let fetcher = fetcher.clone();
                    async move { (fetcher)(PageRequest::new(0, page_size)).await }
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
                move || (refresh_trigger.get(), page_ui.get()),
                move |(_, page_1)| {
                    let fetcher = fetcher.clone();
                    let page_0 = page_1.saturating_sub(1) as u32;
                    let offset = page_0.saturating_mul(page_size);
                    async move { (fetcher)(PageRequest::new(offset, page_size)).await }
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
    let source_empty = Memo::new(move |_| entry_signal.get().is_empty());
    let projected = Memo::new(move |_| {
        project_entries(
            &entry_signal.get(),
            false,
            ctx.features,
            ctx.sort.get(),
            &ctx.filter.get(),
            &ctx.locale.get(),
        )
    });
    let projected_empty = Memo::new(move |_| projected.get().is_empty());
    let filter_active = Memo::new(move |_| ctx.filter.get().is_active());

    let show_initial = Memo::new(move |_| {
        is_loading.get() && source_empty.get() && !ever_loaded.get()
    });
    let show_incremental = Memo::new(move |_| {
        is_loading.get() && (ever_loaded.get() || !source_empty.get())
    });
    let show_empty = Memo::new(move |_| {
        !is_loading.get()
            && source_empty.get()
            && ever_loaded.get()
            && !show_error.get()
            && !filter_active.get()
    });
    let show_no_matches = Memo::new(move |_| {
        !is_loading.get()
            && !source_empty.get()
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
                    <HistoryEntryList entries=entry_signal />
                </Show>
                <Show when=move || show_incremental.get() fallback=|| ()>
                    {loading_more_view()}
                </Show>
                <Show when=move || show_end.get() fallback=|| ()>
                    {end_view()}
                </Show>
            </ScrollArea>
            <Show when=move || show_pagination fallback=|| ()>
                <HistoryDefaultPagination
                    page=page_ui
                    page_count=Signal::derive(move || page_count.get())
                />
            </Show>
        </div>
    }
}
