use leptos::html::Div;
use leptos::prelude::*;
use orbital_core_components::Spinner;
use orbital_data::DataRecord;
use orbital_paging::{use_paged_infinite_scroll, Page, PageRequest};

use crate::core::use_data_table_context;
use crate::engine::build_page_request;
use crate::engine::{coordinator_begin, coordinator_is_current, ServerFetchCoordinator};
use crate::types::{DataTableRowModel, DataTableTableState, PagingMode, ServerFetchPolicy};
use crate::PageFetcher;

fn stale_page(request_offset: u32) -> Page<DataRecord> {
    Page {
        items: vec![],
        has_more: true,
        total_count: None,
        next_request_offset: Some(request_offset),
    }
}

/// Wires server infinite scroll fetching into table state.
#[component]
pub fn DataTableInfiniteScrollController(
    state: DataTableTableState,
    scroll_el: NodeRef<Div>,
    fetcher: PageFetcher,
    page_size: u32,
    fetch_coordinator: RwSignal<ServerFetchCoordinator>,
    server_fetch_policy: StoredValue<ServerFetchPolicy>,
    /// External refresh nonce — bumps first-page reload without remounting the table.
    #[prop(optional)]
    refresh_signal: Option<Signal<u32>>,
) -> impl IntoView {
    let refresh = RwSignal::new(0u32);
    let refresh_in_flight = RwSignal::new(false);
    let external_refresh = refresh_signal.unwrap_or_else(|| Signal::derive(|| 0u32));
    let skip_external = StoredValue::new(true);

    Effect::new(move || {
        let _ = external_refresh.get();
        if skip_external.get_value() {
            skip_external.set_value(false);
            return;
        }
        if refresh_in_flight.get_untracked() {
            return;
        }
        fetch_coordinator.update(|c| c.clear_dedupe());
        refresh.update(|v| *v += 1);
        state.reset_pagination();
    });

    let fetch = {
        let fetcher = fetcher.clone();
        move |req: PageRequest| {
            let fetcher = fetcher.clone();
            let sort = state.sort.get_untracked();
            let filter = state.filter.get_untracked();
            let quick = state.quick_search.get_untracked();
            let request = build_page_request(&sort, &filter, &quick, req.offset, req.limit);
            let request_offset = request.offset;
            async move {
                let policy = server_fetch_policy.get_value();
                let gen = coordinator_begin(fetch_coordinator, &request, &policy);
                let Some(gen) = gen else {
                    return Ok(stale_page(request_offset));
                };
                match (fetcher)(request).await {
                    Ok(page) if coordinator_is_current(fetch_coordinator, gen) => Ok(page),
                    Ok(_) => Ok(stale_page(request_offset)),
                    Err(err) if coordinator_is_current(fetch_coordinator, gen) => Err(err),
                    Err(_) => Ok(stale_page(request_offset)),
                }
            }
        }
    };

    let hook = use_paged_infinite_scroll(scroll_el, page_size, refresh.into(), fetch);

    let skip_initial_refresh = StoredValue::new(true);
    Effect::new(move || {
        let _ = state.sort.get();
        let _ = state.filter.get();
        let _ = state.quick_search.get();
        if skip_initial_refresh.get_value() {
            skip_initial_refresh.set_value(false);
            return;
        }
        if refresh_in_flight.get_untracked() {
            return;
        }
        fetch_coordinator.update(|c| c.clear_dedupe());
        refresh.update(|v| *v += 1);
        state.reset_pagination();
    });

    Effect::new(move || {
        refresh_in_flight.set(hook.loading.get());
    });

    Effect::new(move || {
        let rows: Vec<DataTableRowModel> = hook
            .items
            .get()
            .into_iter()
            .map(DataTableRowModel::new)
            .collect();
        // Update rows in place. Do **not** bump `render_key` — that remounts TableBody
        // and causes a multi-Hz empty/flash loop with infinite scroll.
        state.processed.set(rows);
        state.sync_processed_dataset_server_page();
    });

    Effect::new(move || {
        state.server_loading.set(hook.loading.get());
    });

    Effect::new(move || {
        state.server_ever_loaded.set(hook.ever_loaded.get());
    });

    Effect::new(move || {
        state.server_total.set(hook.total_count.get());
    });

    view! {
        <DataTableInfiniteScrollFooter
            loading=hook.loading
            ever_loaded=hook.ever_loaded
            has_more=hook.has_more
            item_count=hook.items
        />
    }
}

/// Loading, empty, and end-of-list chrome for infinite scroll mode.
#[component]
fn DataTableInfiniteScrollFooter(
    loading: RwSignal<bool>,
    ever_loaded: RwSignal<bool>,
    has_more: RwSignal<bool>,
    item_count: RwSignal<Vec<DataRecord>>,
) -> impl IntoView {
    let ctx = use_data_table_context();
    let locale = StoredValue::new(ctx.locale.get_value());

    view! {
        <div class="orbital-data-table__infinite-scroll-footer">
            <Show when=move || !ever_loaded.get()>
                <div data-testid="data-table-infinite-loading">
                    <Spinner label=locale.get_value().loading.clone() />
                </div>
            </Show>
            <Show when=move || ever_loaded.get() && !has_more.get() && !item_count.get().is_empty()>
                <div class="orbital-data-table__infinite-end" data-testid="data-table-infinite-end">
                    {locale.get_value().infinite_end.clone()}
                </div>
            </Show>
            <Show when=move || loading.get() && ever_loaded.get() && has_more.get()>
                <div data-testid="data-table-infinite-loading-more">
                    <Spinner label=locale.get_value().loading.clone() />
                </div>
            </Show>
        </div>
    }
}

/// Whether infinite scroll should force a bounded scroll host.
pub fn infinite_scroll_needs_bounded_scroll(paging: PagingMode, is_server: bool) -> bool {
    is_server && matches!(paging, PagingMode::InfiniteScroll)
}
