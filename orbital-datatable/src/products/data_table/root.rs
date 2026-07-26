use std::collections::HashMap;
use std::sync::Arc;

use leptos::{html::Div, prelude::*};
use orbital_core_components::{ScrollArea, Table};
use orbital_data::{DataRecord, Dataset};
use orbital_style::inject_style;
use orbital_theme::{use_theme_options, Direction};

use super::body::DataTableBody;
use super::column_reorder::DataTableColumnDragGhost;
use super::controlled::wire_controlled_models;
use super::footer::DataTableFooter;
use super::grid_keyboard::handle_grid_keydown;
use super::header::DataTableHeader;
use super::infinite_scroll::{
    infinite_scroll_needs_bounded_scroll, DataTableInfiniteScrollController,
};
use super::list_body::DataTableListBody;
use super::overlays::DataTableOverlays;
use super::row_reorder::DataTableRowDragGhost;
use super::styles::{data_table_styles, density_modifier_class};
use super::toolbar::DataTableToolbar;
use crate::core::{provide_chart_binding, use_data_table_context, DataTableProvider};
use crate::engine::build_page_request;
use crate::engine::ExportRowScope;
use crate::engine::{coordinator_begin, coordinator_is_current, ServerFetchCoordinator};
#[cfg(feature = "hydrate")]
use crate::engine::{
    scroll_dimensions, scroll_offset_for_column, scroll_offset_for_row, set_scroll_left,
    set_scroll_top, ScrollAlignment,
};
use crate::io::download_bytes;
use crate::types::ChartBinding;
use crate::types::{
    create_column_layout_memo, AggregationModel, AggregationPosition, CellSelection,
    DataTableColumnDef, DataTableColumnGroupDef, DataTableEvents, DataTableFeatures,
    DataTableFilter, DataTableHandle, DataTableInitialState, DataTableLocale, DataTablePivotModel,
    DataTableRowGrouping, DataTableRowModel, DataTableSelectionMode, DataTableSlots, DataTableSort,
    DataTableSource, DataTableSourceKind, DataTableTableState, EditHistory, EditMode,
    EditSessionStore, GetRowId, GetTreePath, ListViewConfig, PaginationDisplayFormat,
    PaginationState, PagingMode, ServerFetchPolicy,
};

use super::edit::{DataTableEditErrorDialog, DataTableEditUndoToolbar};

/// Internal shell: provider, state, processed rows memo, density class.
#[component]
pub fn DataTableRoot(
    #[prop(optional, into)] class: MaybeProp<String>,
    columns: StoredValue<Vec<DataTableColumnDef>>,
    column_groups: StoredValue<Option<Vec<DataTableColumnGroupDef>>>,
    data_source: StoredValue<DataTableSource>,
    paging: PagingMode,
    features: DataTableFeatures,
    sortable: bool,
    resizable_columns: bool,
    header_height: Option<f64>,
    height: Option<f64>,
    max_height: Option<f64>,
    flex: bool,
    auto_row_height: bool,
    get_row_id: StoredValue<Option<GetRowId>>,
    row_detail: StoredValue<Option<Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>>>,
    locale: StoredValue<DataTableLocale>,
    pagination_display: StoredValue<PaginationDisplayFormat>,
    page_size_options: StoredValue<Option<Vec<u32>>>,
    slots: DataTableSlots,
    get_row_class: StoredValue<Option<Callback<(DataTableRowModel, usize), String>>>,
    direction_override: StoredValue<Option<Direction>>,
    client_loading: RwSignal<bool>,
    selection_mode: Signal<Option<DataTableSelectionMode>>,
    initial_state: StoredValue<Option<DataTableInitialState>>,
    controlled_sort: Signal<Option<DataTableSort>>,
    controlled_filter: Signal<Option<DataTableFilter>>,
    controlled_pagination: Signal<Option<PaginationState>>,
    controlled_selection: Signal<Option<std::collections::HashSet<String>>>,
    events: StoredValue<Option<DataTableEvents>>,
    edit_mode: EditMode,
    show_undo_toolbar: bool,
    get_tree_path: StoredValue<Option<GetTreePath>>,
    row_grouping: StoredValue<DataTableRowGrouping>,
    aggregation: StoredValue<AggregationModel>,
    aggregation_position: StoredValue<AggregationPosition>,
    pivot: StoredValue<DataTablePivotModel>,
    list_view: StoredValue<Option<ListViewConfig>>,
    toolbar_config: StoredValue<crate::types::DataTableToolbarConfig>,
    header_chrome: StoredValue<crate::types::DataTableHeaderChromeConfig>,
    server_fetch_policy: StoredValue<ServerFetchPolicy>,
    refresh_signal: Signal<u32>,
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-data-table", data_table_styles());

    let DataTableSlots {
        toolbar: toolbar_slot,
        footer: footer_slot,
        empty_view,
        no_results_view,
        loading_view,
        row_detail: _,
    } = slots;

    let init = initial_state.get_value().unwrap_or_default();

    let quick_search = RwSignal::new(init.quick_search.clone().unwrap_or_default());
    let filter = RwSignal::new(init.filter.clone().unwrap_or_default());
    let sort = RwSignal::new(init.sort.clone().unwrap_or_default());
    let page = RwSignal::new(init.pagination.as_ref().map(|p| p.page).unwrap_or(0));
    let page_size = RwSignal::new(
        init.pagination
            .as_ref()
            .map(|p| p.page_size as usize)
            .unwrap_or(10),
    );
    let server_offset = RwSignal::new(0u32);
    let server_total = RwSignal::new(None::<u64>);
    let server_loading = RwSignal::new(false);
    let server_ever_loaded = RwSignal::new(false);
    let selected = RwSignal::new(init.selection.clone());
    let selection_anchor = RwSignal::new(init.selection.iter().next().cloned());
    let cell_selection = RwSignal::new(CellSelection::default());
    let cell_dragging = RwSignal::new(false);
    let processed = RwSignal::new(Vec::<DataTableRowModel>::new());
    let processed_dataset = RwSignal::new(Dataset::default());
    let chart_x_field = RwSignal::new(None::<String>);
    let chart_y_fields = RwSignal::new(Vec::<String>::new());
    let render_key = RwSignal::new(0u32);
    let column_visibility = RwSignal::new(init.column_visibility.clone());
    let column_widths = RwSignal::new(HashMap::<String, f64>::new());
    let column_order = RwSignal::new(Vec::<String>::new());
    let pinned_columns = RwSignal::new(init.pinned_columns.clone());
    let pinned_rows = RwSignal::new(init.pinned_rows.clone());
    let row_order = RwSignal::new(Vec::<String>::new());
    let expanded_rows = RwSignal::new(std::collections::HashSet::<String>::new());
    let expanded_tree_nodes = RwSignal::new(std::collections::HashSet::<String>::new());
    let expanded_groups = RwSignal::new(std::collections::HashSet::<String>::new());
    let pivot_columns = RwSignal::new(Vec::<DataTableColumnDef>::new());
    let footer_row = RwSignal::new(None::<DataTableRowModel>);
    let row_grouping_signal = RwSignal::new(row_grouping.get_value());
    let aggregation_signal = RwSignal::new(aggregation.get_value());
    let pivot_signal = RwSignal::new(pivot.get_value());
    let edit_session = EditSessionStore::new();
    let edit_error_dialog = RwSignal::new(None::<String>);
    let edit_history = RwSignal::new(EditHistory::default());
    let scroll_el = NodeRef::<Div>::new();
    let scroll_top = RwSignal::new(0.0f64);
    let scroll_left = RwSignal::new(0.0f64);
    let focus_cell = RwSignal::new(None);
    let virtual_scroll_target = RwSignal::new(None::<usize>);
    let horizontal_scroll_target = RwSignal::new(None::<String>);

    let column_layout = create_column_layout_memo(
        columns,
        pivot_columns,
        column_order,
        column_visibility,
        column_widths,
        pinned_columns,
        features,
    );

    let source = data_source.get_value();
    let source_kind = if source.is_server() {
        DataTableSourceKind::Server
    } else {
        DataTableSourceKind::Client
    };

    if let Some(size) = source.server_page_size() {
        page_size.set(size as usize);
    }

    let client_items = source
        .client_items()
        .unwrap_or_else(|| RwSignal::new(Vec::<DataTableRowModel>::new()));

    let row_pinning_enabled = features.contains(DataTableFeatures::ROW_PINNING);
    let force_bounded_scroll =
        infinite_scroll_needs_bounded_scroll(paging, source_kind == DataTableSourceKind::Server);
    let bounded_scroll =
        row_pinning_enabled || height.is_some() || max_height.is_some() || force_bounded_scroll;
    let list_view_enabled =
        features.contains(DataTableFeatures::LIST_VIEW) && list_view.get_value().is_some();
    let horizontal_scroll = !bounded_scroll && !list_view_enabled;

    let table_state = DataTableTableState {
        columns,
        column_groups,
        source_kind,
        client_items,
        processed,
        processed_dataset,
        chart_x_field,
        chart_y_fields,
        total_rows: Memo::new(move |_| {
            if source_kind == DataTableSourceKind::Server {
                server_total
                    .get()
                    .map(|t| t as usize)
                    .unwrap_or(processed.get().len())
            } else {
                let quick_filtered = crate::engine::filter_rows(
                    &client_items.get(),
                    &columns.get_value(),
                    &quick_search.get(),
                );
                crate::engine::filter_by_rules(&quick_filtered, &columns.get_value(), &filter.get())
                    .len()
            }
        }),
        page_count: Memo::new(move |_| {
            if source_kind == DataTableSourceKind::Server {
                server_total
                    .get()
                    .map(|total| {
                        let size = page_size.get().max(1);
                        ((total as f64) / (size as f64)).ceil() as usize
                    })
                    .unwrap_or(1)
                    .max(1)
            } else {
                let quick_filtered = crate::engine::filter_rows(
                    &client_items.get(),
                    &columns.get_value(),
                    &quick_search.get(),
                );
                let total = crate::engine::filter_by_rules(
                    &quick_filtered,
                    &columns.get_value(),
                    &filter.get(),
                )
                .len();
                let size = page_size.get();
                if size == 0 {
                    1
                } else {
                    total.div_ceil(size).max(1)
                }
            }
        }),
        quick_search,
        filter,
        sort,
        page,
        page_size,
        server_offset,
        server_total,
        server_loading,
        server_ever_loaded,
        paging,
        selected,
        selection_anchor,
        cell_selection,
        cell_dragging,
        column_visibility,
        column_widths,
        column_order,
        pinned_columns,
        pinned_rows,
        row_order,
        expanded_rows,
        expanded_tree_nodes,
        expanded_groups,
        get_tree_path,
        row_grouping: row_grouping_signal,
        aggregation: aggregation_signal,
        aggregation_position,
        pivot: pivot_signal,
        list_view,
        pivot_columns,
        footer_row,
        get_row_id,
        selection_mode,
        features,
        resizable_columns,
        header_height,
        sortable,
        events,
        column_layout,
        render_key,
        edit_mode,
        edit_session,
        edit_error_dialog,
        edit_history,
        scroll_top,
        scroll_left,
        client_loading,
        focus_cell,
        virtual_scroll_target,
        horizontal_scroll_target,
        bounded_scroll,
        auto_row_height,
    };

    wire_controlled_models(
        table_state,
        controlled_sort,
        controlled_filter,
        controlled_pagination,
        controlled_selection,
    );

    if table_state.is_server() {
        if let Some(p) = &init.pagination {
            let size = table_state.page_size.get() as u32;
            table_state
                .server_offset
                .set((p.page as u32).saturating_mul(size.max(1)));
        }
    } else {
        table_state.recompute_client_processed();
    }

    Effect::new(move |_| {
        if virtual_scroll_target.get().is_none() {
            return;
        }
        #[cfg(feature = "hydrate")]
        {
            let target = virtual_scroll_target.get_untracked();
            virtual_scroll_target.set(None);
            if let Some(target) = target {
                if let Some(el) = scroll_el.get() {
                    let row_height = table_state.row_height_px();
                    let (_, viewport_h) = scroll_dimensions(&el);
                    let top = scroll_offset_for_row(
                        target,
                        row_height,
                        viewport_h,
                        ScrollAlignment::Start,
                    );
                    set_scroll_top(&el, top);
                    table_state.scroll_top.set(top);
                }
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            virtual_scroll_target.set(None);
        }
    });

    Effect::new(move |_| {
        if horizontal_scroll_target.get().is_none() {
            return;
        }
        #[cfg(feature = "hydrate")]
        if let Some(field) = horizontal_scroll_target.get_untracked() {
            if let Some(el) = scroll_el.get() {
                let fields = table_state.visible_data_fields();
                if let Some(index) = fields.iter().position(|f| f == &field) {
                    let layout = table_state.column_layout.get();
                    let widths: Vec<f64> = layout.columns.iter().map(|c| c.width_px).collect();
                    let offsets = crate::engine::column_left_offsets(&widths);
                    let (_, viewport_w) = crate::engine::scroll_dimensions_horizontal(&el);
                    let left = scroll_offset_for_column(
                        index,
                        &offsets,
                        &widths,
                        viewport_w,
                        ScrollAlignment::Start,
                    );
                    set_scroll_left(&el, left);
                    scroll_left.set(left);
                }
            }
        }
        horizontal_scroll_target.set(None);
    });

    events.with_value(|events| {
        let Some(events) = events else {
            return;
        };
        let export_csv = Callback::new(move |()| {
            let csv = table_state.export_csv(ExportRowScope::AllMatching);
            download_bytes("export.csv", csv.as_bytes(), "text/csv;charset=utf-8");
        });
        let sort_column = Callback::new(
            move |(field, direction): (String, crate::engine::SortDirection)| {
                table_state.sort_column(&field, direction);
            },
        );
        let set_filter = Callback::new(move |(filter,): (DataTableFilter,)| {
            table_state.set_filter(filter);
        });
        let set_quick_search = Callback::new(move |(text,): (String,)| {
            table_state.set_quick_search(text);
        });
        let scroll_to_row = Callback::new(move |row_id: (String,)| {
            table_state.scroll_to_row_id(&row_id.0);
        });
        let scroll_to_column = Callback::new(move |field: (String,)| {
            table_state.scroll_to_column_field(&field.0);
        });
        let get_processed_dataset = Callback::new(move |()| table_state.processed_dataset());
        let export_state = Callback::new(move |()| table_state.capture_state().export());
        let restore_state = Callback::new(move |(snapshot,): (crate::types::SerializedState,)| {
            if let Ok(state) = crate::types::DataTableState::restore(snapshot) {
                table_state.apply_state(&state);
            }
        });
        events.notify_handle(DataTableHandle {
            sort_column,
            set_filter,
            set_quick_search,
            export_state,
            restore_state,
            export_csv,
            scroll_to_row,
            scroll_to_column,
            get_processed_dataset,
        });
    });

    #[cfg(feature = "hydrate")]
    {
        use leptos::ev;
        use wasm_bindgen::JsCast;

        let _mouseup = window_event_listener(ev::mouseup, move |_ev| {
            if cell_dragging.try_get_untracked() == Some(true) {
                cell_dragging.set(false);
            }
        });

        let scroll_el_for_listener = scroll_el;
        let scroll_top_signal = scroll_top;
        let scroll_left_signal = scroll_left;
        Effect::new(move |_| {
            let Some(el) = scroll_el_for_listener.get() else {
                return;
            };
            let on_scroll =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_ev: web_sys::Event| {
                    if let Some(el) = scroll_el_for_listener.get() {
                        scroll_top_signal.set(el.scroll_top() as f64);
                        scroll_left_signal.set(el.scroll_left() as f64);
                    }
                }) as Box<dyn FnMut(_)>);
            let _ = el.add_event_listener_with_callback(
                "scroll",
                on_scroll.as_ref().unchecked_ref::<js_sys::Function>(),
            );
            on_scroll.forget();
        });
    }

    Effect::new(move |_| {
        if source_kind != DataTableSourceKind::Client {
            return;
        }
        let _ = quick_search.get();
        let _ = filter.get();
        let _ = sort.get();
        let _ = page.get();
        let _ = page_size.get();
        let _ = client_items.get();
        let _ = column_visibility.get();
        let _ = expanded_tree_nodes.get();
        let _ = expanded_groups.get();
        let _ = row_grouping_signal.get();
        let _ = aggregation_signal.get();
        table_state.recompute_client_processed();
    });

    Effect::new(move |_| {
        if source_kind == DataTableSourceKind::Client {
            let _ = quick_search.get();
            let _ = filter.get();
            let _ = sort.get();
            page.set(0);
        }
    });

    let server_infinite =
        source_kind == DataTableSourceKind::Server && matches!(paging, PagingMode::InfiniteScroll);
    let server_paged = source_kind == DataTableSourceKind::Server
        && matches!(paging, PagingMode::Paged | PagingMode::None);

    let fetch_coordinator = RwSignal::new(ServerFetchCoordinator::default());

    if server_paged {
        if let Some(fetcher) = data_source.get_value().server_fetcher() {
            let fetcher_stored = StoredValue::new(fetcher);

            Effect::new(move |_| {
                let _ = quick_search.get();
                let _ = filter.get();
                server_total.set(None);
                fetch_coordinator.update(|c| c.clear_dedupe());
            });

            Effect::new(move |_| {
                let _ = quick_search.get();
                let _ = filter.get();
                let _ = sort.get();
                server_offset.set(0);
            });

            Effect::new(move |_| {
                let _ = sort.get();
                let _ = filter.get();
                let _ = quick_search.get();
                let _ = page_size.get();
                let _ = refresh_signal.get();
                let offset = server_offset.get();
                let limit = page_size.get() as u32;
                let fetcher = fetcher_stored.get_value();
                let policy = server_fetch_policy.get_value();
                let request = build_page_request(
                    &sort.get_untracked(),
                    &filter.get_untracked(),
                    &quick_search.get_untracked(),
                    offset,
                    limit,
                );
                let gen = coordinator_begin(fetch_coordinator, &request, &policy);
                let Some(gen) = gen else {
                    return;
                };
                server_loading.set(true);
                leptos::task::spawn_local_scoped(async move {
                    match (fetcher)(request).await {
                        Ok(page_result) if coordinator_is_current(fetch_coordinator, gen) => {
                            if let Some(count) = page_result.total_count {
                                server_total.set(Some(count));
                            }
                            processed.set(
                                page_result
                                    .items
                                    .into_iter()
                                    .map(DataTableRowModel::new)
                                    .collect(),
                            );
                            table_state.sync_processed_dataset_server_page();
                            server_ever_loaded.set(true);
                            render_key.update(|k| *k += 1);
                        }
                        Err(_) if coordinator_is_current(fetch_coordinator, gen) => {
                            processed.set(Vec::new());
                            table_state.sync_processed_dataset_server_page();
                            server_ever_loaded.set(true);
                        }
                        _ => {}
                    }
                    if coordinator_is_current(fetch_coordinator, gen) {
                        server_loading.set(false);
                    }
                });
            });
        }
    }

    let infinite_fetcher = server_infinite
        .then(|| data_source.get_value().server_fetcher())
        .flatten();

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());

    let column_order_label = Memo::new(move |_| {
        let order = table_state.column_order.get();
        if order.is_empty() {
            table_state
                .columns
                .get_value()
                .iter()
                .map(|c| c.field.clone())
                .collect::<Vec<_>>()
                .join(",")
        } else {
            order.join(",")
        }
    });

    let direction = Signal::derive(move || {
        direction_override
            .get_value()
            .unwrap_or(theme_options.get().direction)
    });

    let header_height_style = header_height
        .map(|h| format!("--orbital-data-table-header-height: {h}px;"))
        .unwrap_or_default();

    let scroll_area_style = if bounded_scroll {
        if let Some(h) = height {
            format!("display: block; width: 100%; height: {h}px;")
        } else if let Some(h) = max_height {
            format!("display: block; width: 100%; max-height: {h}px;")
        } else {
            "display: block; width: 100%;".to_string()
        }
    } else {
        "display: block; width: 100%;".to_string()
    };

    let sticky_header = bounded_scroll;

    view! {
        <DataTableProvider
            features=features
            selection_mode=selection_mode
            events=events
            get_row_id=get_row_id
            row_detail=row_detail
            auto_row_height=auto_row_height
            locale=locale
            pagination_display=pagination_display
            page_size_options=page_size_options
            get_row_class=get_row_class
            direction=direction
            toolbar_config=toolbar_config
            header_chrome=header_chrome
        >
            <DataTableChartBindingScope
                table_state=table_state
                features=features
            >
            <div
                class=move || {
                    let mut parts = vec!["orbital-data-table".to_string()];
                    let density = density_class.get();
                    if !density.is_empty() {
                        parts.push(density);
                    }
                    if flex {
                        parts.push("orbital-data-table--flex-fill".to_string());
                    }
                    if auto_row_height {
                        parts.push("orbital-data-table--auto-row-height".to_string());
                    }
                    if row_pinning_enabled {
                        parts.push("orbital-data-table--row-pinning".to_string());
                    }
                    if sticky_header {
                        parts.push("orbital-data-table--sticky-header".to_string());
                    }
                    if direction.get() == Direction::Rtl {
                        parts.push("orbital-data-table--rtl".to_string());
                    }
                    if force_bounded_scroll {
                        parts.push("orbital-data-table--infinite-scroll".to_string());
                    }
                    if list_view_enabled {
                        parts.push("orbital-data-table--list-view".to_string());
                    }
                    if let Some(extra) = class.get() {
                        if !extra.is_empty() {
                            parts.push(extra);
                        }
                    }
                    parts.join(" ")
                }
                style=header_height_style.clone()
                dir=move || direction.get().as_str()
                data-testid="data-table-root"
            >
                {match toolbar_slot {
                    Some(toolbar_slot) => {
                        view! { <DataTableToolbar state=table_state toolbar_slot=toolbar_slot /> }.into_any()
                    }
                    None => view! { <DataTableToolbar state=table_state /> }.into_any(),
                }}
                <Show when=move || {
                    show_undo_toolbar && table_state.features.contains(DataTableFeatures::UNDO_REDO)
                }>
                    <DataTableEditUndoToolbar state=table_state />
                </Show>
                <div
                    class=move || {
                        let mut parts = vec!["orbital-data-table__scroll-host".to_string()];
                        if bounded_scroll {
                            parts.push("orbital-data-table__scroll-host--bounded".to_string());
                        }
                        parts.join(" ")
                    }
                    tabindex="-1"
                    data-testid="data-table-grid-focus"
                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                        let ctx = use_data_table_context();
                        handle_grid_keydown(table_state, ctx, ev);
                    }
                >
                    <ScrollArea
                        horizontal=horizontal_scroll
                        class="orbital-data-table__scroll"
                        style=scroll_area_style.clone()
                        node_ref=scroll_el
                        scroll_testid="data-table-scroll"
                        scroll_data_column_order=column_order_label
                    >
                        <DataTableMain state=table_state />
                        {infinite_fetcher.map(|fetcher| view! {
                            <DataTableInfiniteScrollController
                                state=table_state
                                scroll_el=scroll_el
                                fetcher=fetcher
                                page_size=page_size.get() as u32
                                fetch_coordinator=fetch_coordinator
                                server_fetch_policy=server_fetch_policy
                                refresh_signal=refresh_signal
                            />
                        })}
                    </ScrollArea>
                    <DataTableOverlays
                        state=table_state
                        empty_view=empty_view
                        no_results_view=no_results_view
                        loading_view=loading_view
                    />
                </div>
                {match footer_slot {
                    Some(footer_slot) => {
                        view! { <DataTableFooter state=table_state footer_slot=footer_slot /> }.into_any()
                    }
                    None => view! { <DataTableFooter state=table_state /> }.into_any(),
                }}
                <DataTableColumnDragGhost />
                <DataTableRowDragGhost />
                <DataTableEditErrorDialog state=table_state />
                {children.map(|c| c())}
            </div>
            </DataTableChartBindingScope>
        </DataTableProvider>
    }
}

/// Provides [`ChartBinding`] when [`DataTableFeatures::CHARTS_INTEGRATION`] is enabled.
#[component]
fn DataTableChartBindingScope(
    table_state: DataTableTableState,
    features: DataTableFeatures,
    children: Children,
) -> impl IntoView {
    if features.contains(DataTableFeatures::CHARTS_INTEGRATION) {
        provide_chart_binding(ChartBinding {
            dataset: table_state.processed_dataset,
            x_field: Signal::derive(move || table_state.chart_x_field.get()),
            y_fields: Signal::derive(move || table_state.chart_y_fields.get()),
        });
    }
    children()
}

/// Header + body table; must render under [`DataTableProvider`] for context.
#[component]
fn DataTableMain(state: DataTableTableState) -> impl IntoView {
    view! {
        {move || {
            if state.list_view_enabled() {
                return view! { <DataTableListBody state=state /> }.into_any();
            }
            view! {
                <Table>
                    <DataTableHeader state=state />
                    <DataTableBody state=state />
                </Table>
            }
            .into_any()
        }}
    }
}
