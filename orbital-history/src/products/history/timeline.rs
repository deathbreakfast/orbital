use leptos::html::Div;
use leptos::prelude::*;
use orbital_core_components::ScrollArea;
use orbital_macros::component_doc;
use orbital_paging::{use_paged_infinite_scroll, PageRequest};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::context::{provide_history_context, HistoryContext};
use crate::types::{
    resolve_history_locale, HistoryChangeSlot, HistoryEmptyView, HistoryEndView, HistoryEntry,
    HistoryEntrySlot, HistoryErrorView, HistoryEvents, HistoryFeatures, HistoryHeader,
    HistoryLoadingMoreView, HistoryLoadingView, HistoryLocale, HistoryOrientation,
    HistoryPageFetcher, HistoryPagingMode, HistoryRenderers, HistorySlots, HistorySource,
};

use super::styles::{density_modifier_class, history_styles};
use super::{
    HistoryDefaultEmptyView, HistoryDefaultEndView, HistoryDefaultErrorView,
    HistoryDefaultHeader, HistoryDefaultLoadingMoreView, HistoryEntryList,
    HistoryTimelineSkeleton,
};

/// Scrollable audit timeline from a client signal or server page fetcher.
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

    provide_history_context(HistoryContext {
        locale: locale_signal.into(),
        features,
        orientation,
        events: events.clone(),
        renderers: merged_renderers,
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
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
) -> impl IntoView {
    let is_loading = Memo::new(move |_| loading.map(|s| s.get()).unwrap_or(false));
    let is_empty = Memo::new(move |_| entries.get().is_empty());
    let show_initial = Memo::new(move |_| is_loading.get() && is_empty.get());
    let show_empty = Memo::new(move |_| !is_loading.get() && is_empty.get());

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
        <ScrollArea class="orbital-history__scroll".to_string() style=scroll_style.unwrap_or_default()>
            <Show when=move || show_initial.get() fallback=|| ()>
                {loading_view()}
            </Show>
            <Show when=move || show_empty.get() fallback=|| ()>
                {empty_view()}
            </Show>
            <Show when=move || !is_empty.get() fallback=|| ()>
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
    events: HistoryEvents,
    empty_slot: Option<HistoryEmptyView>,
    loading_slot: Option<HistoryLoadingView>,
    loading_more_slot: Option<HistoryLoadingMoreView>,
    error_slot: Option<HistoryErrorView>,
    end_slot: Option<HistoryEndView>,
) -> impl IntoView {
    let scroll_el = NodeRef::<Div>::new();
    let refresh = RwSignal::new(0u32);
    let load_error = RwSignal::new(false);

    let on_load_error = events.on_load_error.clone();

    let fetch = {
        let fetcher = fetcher.clone();
        move |req: PageRequest| {
            let fetcher = fetcher.clone();
            async move { (fetcher)(req).await }
        }
    };

    let (entries, hook_loading, ever_loaded, has_more, show_error) = match paging {
        HistoryPagingMode::InfiniteScroll => {
            let hook = use_paged_infinite_scroll(scroll_el, page_size, refresh.into(), fetch);
            Effect::new(move |_| {
                // Surface fetch errors via Resource — hook stores items on success only.
                // Track loading transitions; errors are reported if items stay empty after load.
                let _ = hook.loading.get();
                let _ = hook.ever_loaded.get();
            });
            (
                hook.items,
                Signal::derive(move || hook.loading.get()),
                Signal::derive(move || hook.ever_loaded.get()),
                Signal::derive(move || hook.has_more.get()),
                Signal::derive(move || load_error.get()),
            )
        }
        HistoryPagingMode::None => {
            let items = RwSignal::new(Vec::<HistoryEntry>::new());
            let loading_sig = RwSignal::new(true);
            let ever = RwSignal::new(false);
            let has_more_sig = RwSignal::new(false);
            let fetcher = fetcher.clone();
            let first_page = Resource::new(
                || (),
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
            )
        }
    };

    let is_loading = Memo::new(move |_| {
        loading
            .map(|s| s.get())
            .unwrap_or_else(|| hook_loading.get())
    });

    let entry_signal = Signal::derive(move || entries.get());
    let is_empty = Memo::new(move |_| entry_signal.get().is_empty());

    let show_initial = Memo::new(move |_| {
        is_loading.get() && is_empty.get() && !ever_loaded.get()
    });
    let show_incremental = Memo::new(move |_| {
        is_loading.get() && (ever_loaded.get() || !is_empty.get())
    });
    let show_empty = Memo::new(move |_| {
        !is_loading.get() && is_empty.get() && ever_loaded.get() && !show_error.get()
    });
    let show_end = Memo::new(move |_| {
        ever_loaded.get() && !has_more.get() && !is_empty.get() && !is_loading.get()
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
            <Show when=move || !is_empty.get() fallback=|| ()>
                <HistoryEntryList entries=entry_signal />
            </Show>
            <Show when=move || show_incremental.get() fallback=|| ()>
                {loading_more_view()}
            </Show>
            <Show when=move || show_end.get() fallback=|| ()>
                {end_view()}
            </Show>
        </ScrollArea>
    }
}
