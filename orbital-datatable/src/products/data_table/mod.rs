mod aggregation_footer;
mod body;
mod cell;
mod cell_selection_handlers;
mod column_menu;
mod column_picker;
mod column_reorder;
mod column_styles;
mod controlled;
mod docs;
mod edit;
mod export_menu;
mod filter_panel;
mod filter_rule_editor;
mod footer;
mod grid_keyboard;
mod group_row;
mod header;
mod header_cell;
mod header_filters;
mod header_groups;
mod infinite_scroll;
mod leading_columns;
mod list_body;
mod overlays;
mod pagination_bar;
#[cfg(feature = "preview")]
mod phase9_previews;
mod pivot_panel;
mod quick_search;
mod root;
mod row;
mod row_interaction;
mod row_reorder;
mod styles;
mod toolbar;
mod toolbar_overflow;
mod tree_grouping_cell;

pub use body::DataTableBody;
pub use cell::data_table_cell_view;
pub use footer::DataTableFooter;
pub use header::DataTableHeader;
pub use header_cell::DataTableHeaderCell;
pub use overlays::DataTableOverlays;
pub use pagination_bar::DataTablePaginationBar;
pub use quick_search::DataTableQuickSearch;
pub use root::DataTableRoot;
pub use toolbar::DataTableToolbar;

#[cfg(feature = "preview")]
pub use docs::{
    DATATABLEADVANCEDDOC_PREVIEW_REGISTRATION, DATATABLECHARTSINTEGRATIONDOC_PREVIEW_REGISTRATION,
    DATATABLECOLUMNDEFINITIONDOC_PREVIEW_REGISTRATION, DATATABLECOLUMNSDOC_PREVIEW_REGISTRATION,
    DATATABLEDATASOURCEDOC_PREVIEW_REGISTRATION, DATATABLEEDITINGDOC_PREVIEW_REGISTRATION,
    DATATABLEEXPORTDOC_PREVIEW_REGISTRATION, DATATABLERENDERINGDOC_PREVIEW_REGISTRATION,
    DATATABLEROWSDOC_PREVIEW_REGISTRATION, DATATABLESELECTIONDOC_PREVIEW_REGISTRATION,
    DATATABLESLOTSDOC_PREVIEW_REGISTRATION, DATATABLESORTINGFILTERINGDOC_PREVIEW_REGISTRATION,
    DATATABLESTATEDOC_PREVIEW_REGISTRATION,
};

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::Direction;

use crate::types::{
    AggregationModel, AggregationPosition, DataTableColumnDef, DataTableColumnGroupDef,
    DataTableEmptyView, DataTableEvents, DataTableFeatures, DataTableFilter, DataTableFooterSlot,
    DataTableHandle, DataTableHeaderChromeConfig, DataTableInitialState, DataTableLoadingView,
    DataTableLocale, DataTableNoResultsView, DataTablePivotModel, DataTableRowDetail,
    DataTableRowGrouping, DataTableRowModel, DataTableSelectionMode, DataTableSlots, DataTableSort,
    DataTableSource, DataTableToolbarConfig, DataTableToolbarSlot, EditMode, GetRowId, GetTreePath,
    ListViewConfig, PaginationDisplayFormat, PaginationState, PagingMode, RowDetailView,
    ServerFetchPolicy,
};

fn default_page_size_options() -> Option<Vec<u32>> {
    Some(vec![10, 25, 50, 100])
}

/// Presents sortable, filterable tabular data with built-in toolbar, selection, and pagination.
///
/// Bind `columns` and `items` (or `data_source`) to get a working table. Enable advanced capabilities
/// via [`DataTableFeatures`] and tune toolbar/header chrome with [`DataTableToolbarConfig`] and
/// [`DataTableHeaderChromeConfig`]. For static HTML tables without a data engine, use
/// [`Table`](orbital_core_components::Table) instead.
///
/// # When to use
///
/// - Interactive grids over medium client-side datasets with sort, search, and pagination
/// - Server-driven paging, sort, and filter via [`DataTableSource::Server`]
/// - Admin views that need selection, editing, export, or grouping
///
/// # When to use Table instead
///
/// Use [`Table`](orbital_core_components::Table) for static or lightly interactive content
/// without a built-in data engine.
///
/// # Usage
///
/// 1. Define columns with [`DataTableColumnDef`] — see [Column Definition](/data-table-column-definition).
/// 2. Supply rows via `items` or `data_source`.
/// 3. Enable feature flags on `features` as needed (pinning, virtualization, pivot, etc.).
/// 4. Customize toolbar and header chrome with `toolbar_config` and `header_chrome` without replacing slots.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind columns to [`FieldDef`](crate::FieldDef) keys via [`DataTableColumnDef::field`]
/// * Wrap rows as [`DataTableRowModel`] over typed [`DataRecord`](crate::DataRecord) values
/// * Prefer `data_source` for explicit client/server mode; `items` is sugar for client signals
/// * Use topic pages under Data Table for column, row, editing, and state patterns
///
/// ## Don'ts
///
/// * Do not replace the entire toolbar when you only need to hide one control — use `toolbar_config`
/// * Do not use raw HTML for toolbar/footer controls — the built-in chrome uses Orbital primitives
///
/// # DataTable topic guide
///
/// - **Columns** — [Column Definition](/data-table-column-definition), [Column Features](/data-table-columns)
/// - **Rows** — [Rows](/data-table-rows)
/// - **Editing** — [Editing](/data-table-editing)
/// - **Sort & filter** — [Sorting & Filtering](/data-table-sorting-filtering)
/// - **Data & paging** — [Data Source & Pagination](/data-table-data-source)
/// - **Selection & export** — [Selection](/data-table-selection), [Export & Clipboard](/data-table-export)
/// - **UX** — [Rendering & UX](/data-table-rendering)
/// - **Advanced** — [Tree, Grouping & Pivot](/data-table-advanced), [Charts Integration](/data-table-charts-integration)
/// - **State** — [State & Handle](/data-table-state)
///
/// # Toolbar and header chrome
///
/// | Field | Type | Default | Description |
/// |-------|------|---------|-------------|
/// | `quick_search` | `bool` | `true` | Quick-search field in the toolbar |
/// | `filter_panel` | `bool` | `true` | Structured filter panel trigger |
/// | `column_picker` | `bool` | `true` | Column visibility picker trigger |
/// | `pivot` | `bool` | `true` | Pivot panel trigger (requires `PIVOTING`) |
/// | `export_menu` | `bool` | `true` | Export/print menu trigger |
///
/// [`DataTableHeaderChromeConfig`] gates per-header menu, filter button, and hide-column UX.
/// See [Column Features](/data-table-columns) for header chrome interaction with column menus.
///
/// # Examples
///
/// ## Default data table
/// <!-- default -->
/// Sortable columns with quick search and pagination.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel};
/// use std::collections::HashMap;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-preview">
///         <DataTable
///             sortable=true
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Row selection
/// Multiselect with checkboxes.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, DataTableSelectionMode};
/// use std::collections::HashMap;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("a", HashMap::from([("name".into(), "Alpha".into())])),
///     DataTableRowModel::from_text_cells("b", HashMap::from([("name".into(), "Beta".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-selection">
///         <DataTable
///             selection_mode=DataTableSelectionMode::Multiselect
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Density variants
/// Row and header heights respond to theme density.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel};
/// use orbital_core_components::{Flex, FlexGap, ThemeDensityStepper};
/// use std::collections::HashMap;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-density">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <ThemeDensityStepper />
///             <DataTable
///                 columns=vec![DataTableColumnDef::new("name", "Name")]
///                 items=items
///             />
///         </Flex>
///     </div>
/// }
/// ```
///
///
/// ## Layout
/// Fixed height and flex-fill in a bounded parent.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// let items = RwSignal::new((0..30).map(|i| {
///     DataTableRowModel::from_text_cells(&i.to_string(), HashMap::from([("name".into(), format!("Row {i}"))]))
/// }).collect::<Vec<_>>());
/// view! {
///     <div data-testid="data-table-layout-preview" style="display: flex; flex-direction: column; height: 350px;">
///         <DataTable
///             flex=true
///             max_height=280.0
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Custom slots
/// Replace toolbar, footer, and empty views with custom content via Leptos slot children.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     DataTable, DataTableColumnDef, DataTableEmptyView, DataTableFooterSlot,
///     DataTableRowModel, DataTableToolbarSlot, PagingMode,
/// };
/// use orbital_core_components::{Toolbar, ToolbarButton};
/// let empty: RwSignal<Vec<DataTableRowModel>> = RwSignal::new(vec![]);
/// view! {
///     <div data-testid="data-table-slots-preview">
///         <DataTable
///             paging=PagingMode::None
///             max_height=200.0
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=empty
///         >
///             <DataTableToolbarSlot slot>
///                 <div data-testid="custom-toolbar">
///                     <Toolbar><ToolbarButton>"Custom toolbar"</ToolbarButton></Toolbar>
///                 </div>
///             </DataTableToolbarSlot>
///             <DataTableFooterSlot slot>
///                 <div data-testid="custom-footer">"Custom footer"</div>
///             </DataTableFooterSlot>
///             <DataTableEmptyView slot>
///                 <div data-testid="custom-empty">"No data yet"</div>
///             </DataTableEmptyView>
///         </DataTable>
///     </div>
/// }
/// ```
///
///
/// ## Toolbar and header chrome
/// Toggle built-in toolbar controls and column-header actions without replacing the whole toolbar.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     DataTable, DataTableColumnDef, DataTableHeaderChromeConfig, DataTableRowModel,
///     DataTableToolbarConfig, PagingMode,
/// };
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-chrome-config-preview">
///         <DataTable
///             paging=PagingMode::None
///             max_height=200.0
///             toolbar_config=DataTableToolbarConfig {
///                 quick_search: true,
///                 filter_panel: false,
///                 column_picker: false,
///                 pivot: false,
///                 export_menu: true,
///             }
///             header_chrome=DataTableHeaderChromeConfig {
///                 column_menu: false,
///                 column_filter_button: false,
///                 column_hide: false,
///             }
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
#[component_doc(
    category = "Data Table",
    group_priority = 10,
    preview_slug = "data-table",
    preview_label = "Data Table",
    preview_icon = icondata::AiTableOutlined,
)]
#[component]
pub fn DataTable(
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Column definitions (bind to dataset schema field keys).
    #[prop(optional)]
    columns: Option<Vec<DataTableColumnDef>>,
    /// Unified data source (client signal or server fetcher).
    #[prop(optional)]
    data_source: Option<DataTableSource>,
    /// Pagination presentation (`Paged`, `InfiniteScroll`, or `None`).
    #[prop(default = PagingMode::Paged)]
    paging: PagingMode,
    /// Reactive row data (sugar for `DataTableSource::Client` when `data_source` is omitted).
    #[prop(optional)]
    items: Option<RwSignal<Vec<DataTableRowModel>>>,
    /// Inline edit scope: single cell or whole row.
    #[prop(default = EditMode::Cell)]
    edit_mode: EditMode,
    /// Show undo/redo toolbar (typically enabled in undo preview).
    #[prop(default = false)]
    show_undo_toolbar: bool,
    /// Opt-in capability flags.
    #[prop(default = DataTableFeatures::empty())]
    features: DataTableFeatures,
    /// Built-in toolbar control visibility (ignored when a custom toolbar slot is provided).
    #[prop(default = DataTableToolbarConfig::default())]
    toolbar_config: DataTableToolbarConfig,
    /// Column header chrome visibility (menu, filter button, hide-column UX).
    #[prop(default = DataTableHeaderChromeConfig::default())]
    header_chrome: DataTableHeaderChromeConfig,
    /// Enable column header sorting.
    #[prop(default = true)]
    sortable: bool,
    /// Enable drag resize on column headers.
    #[prop(default = false)]
    resizable_columns: bool,
    /// Optional nested column groups for multi-row headers.
    #[prop(optional)]
    column_groups: Option<Vec<DataTableColumnGroupDef>>,
    /// Optional override for header row height in pixels.
    #[prop(optional)]
    header_height: Option<f64>,
    /// Fixed height for the scroll body in pixels (enables vertical scroll).
    #[prop(optional)]
    height: Option<f64>,
    /// Optional max height for the scroll body (enables vertical scroll).
    #[prop(optional)]
    max_height: Option<f64>,
    /// Fill available height in a flex parent (`flex: 1; min-height: 0`).
    #[prop(default = false)]
    flex: bool,
    /// Allow rows to grow taller than the density-mapped minimum height.
    #[prop(default = false)]
    auto_row_height: bool,
    /// Localized UI strings (footer, overlays, search placeholder).
    #[prop(optional)]
    locale: Option<DataTableLocale>,
    /// Footer pagination label format (`Locale` range vs legacy `Plain` count).
    #[prop(default = PaginationDisplayFormat::Locale)]
    pagination_display: PaginationDisplayFormat,
    /// Rows-per-page options for footer Select (`None` hides the selector).
    #[prop(default = default_page_size_options())]
    page_size_options: Option<Vec<u32>>,
    /// Custom toolbar — nest with `<DataTableToolbarSlot slot>`.
    #[prop(optional)]
    data_table_toolbar: Option<DataTableToolbarSlot>,
    /// Deprecated — use [`data_table_toolbar`].
    #[prop(optional)]
    data_table_toolbar_slot: Option<DataTableToolbarSlot>,
    /// Custom footer — nest with `<DataTableFooterSlot slot>`.
    #[prop(optional)]
    data_table_footer: Option<DataTableFooterSlot>,
    /// Deprecated — use [`data_table_footer`].
    #[prop(optional)]
    data_table_footer_slot: Option<DataTableFooterSlot>,
    /// Custom empty-state overlay — nest with `<DataTableEmptyView slot>`.
    #[prop(optional)]
    data_table_empty_view: Option<DataTableEmptyView>,
    /// Custom no-results overlay — nest with `<DataTableNoResultsView slot>`.
    #[prop(optional)]
    data_table_no_results_view: Option<DataTableNoResultsView>,
    /// Custom loading overlay — nest with `<DataTableLoadingView slot>`.
    #[prop(optional)]
    data_table_loading_view: Option<DataTableLoadingView>,
    /// Client-controlled loading state for overlay display.
    #[prop(optional)]
    loading: Option<RwSignal<bool>>,
    /// Text direction override (defaults to theme direction).
    #[prop(optional)]
    dir: Option<Direction>,
    /// Per-row CSS class callback.
    #[prop(optional)]
    get_row_class: Option<Callback<(DataTableRowModel, usize), String>>,
    /// Custom row id resolver (default: [`DataRecord::id`]).
    #[prop(optional)]
    get_row_id: Option<GetRowId>,
    /// Hierarchical path resolver for tree data (`TREE_DATA`).
    #[prop(optional)]
    get_tree_path: Option<GetTreePath>,
    /// Row grouping model (`ROW_GROUPING`).
    #[prop(optional)]
    row_grouping: Option<DataTableRowGrouping>,
    /// Aggregation rules for footer/group summaries (`AGGREGATION`).
    #[prop(optional)]
    aggregation: Option<AggregationModel>,
    /// Where aggregate values render (footer or inline on groups).
    #[prop(default = AggregationPosition::Footer)]
    aggregation_position: AggregationPosition,
    /// Pivot configuration (`PIVOTING`).
    #[prop(optional)]
    pivot: Option<DataTablePivotModel>,
    /// List view card layout config (`LIST_VIEW`).
    #[prop(optional)]
    list_view: Option<ListViewConfig>,
    /// Custom row detail panel — nest with `<DataTableRowDetail slot render=... />`.
    #[prop(optional)]
    data_table_row_detail: Option<DataTableRowDetail>,
    /// Deprecated — use [`DataTableRowDetail`] slot or [`data_table_row_detail`].
    #[prop(optional)]
    row_detail: Option<RowDetailView>,
    /// Row selection mode (`Single` or `Multiselect`).
    #[prop(optional)]
    selection_mode: Option<DataTableSelectionMode>,
    /// One-time initial state (sort, search, pagination, selection).
    #[prop(optional)]
    initial_state: Option<DataTableInitialState>,
    /// Controlled sort model (`None` = uncontrolled).
    #[prop(optional)]
    sort: Option<Signal<Option<DataTableSort>>>,
    /// Controlled filter model (`None` = uncontrolled).
    #[prop(optional)]
    filter: Option<Signal<Option<DataTableFilter>>>,
    /// Controlled pagination (`None` = uncontrolled).
    #[prop(optional)]
    pagination: Option<Signal<Option<PaginationState>>>,
    /// Controlled selection ids (`None` = uncontrolled).
    #[prop(optional)]
    selection: Option<Signal<Option<std::collections::HashSet<String>>>>,
    /// Server fetch invalidation: drop stale in-flight responses; optional [`ServerFetchPolicy::dedupe_key`].
    #[prop(default = ServerFetchPolicy::default())]
    server_fetch_policy: ServerFetchPolicy,
    /// External refresh nonce — reloads server data in place (no remount / empty flash).
    #[prop(optional)]
    refresh_signal: Option<Signal<u32>>,
    /// Side-effect callbacks for table integration.
    #[prop(optional, default = DataTableEvents::default())]
    data_table_events: DataTableEvents,
    /// Deprecated — prefer [`data_table_events`].
    #[prop(optional)]
    events: Option<DataTableEvents>,
    /// Deprecated — prefer `data_table_events.on_handle`.
    #[prop(optional)]
    on_handle: Option<Callback<DataTableHandle, ()>>,
    /// Additional children (provider context, etc.).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let columns = StoredValue::new(columns.unwrap_or_default());
    let column_groups = StoredValue::new(column_groups);
    let initial_state = StoredValue::new(initial_state);
    let resolved_events = StoredValue::new(Some(DataTableEvents::resolve(
        data_table_events,
        events,
        on_handle,
    )));
    let items = items.unwrap_or_else(|| RwSignal::new(Vec::new()));

    let resolved_source = data_source.unwrap_or_else(|| DataTableSource::client_signal(items));
    let data_source = StoredValue::new(resolved_source);

    let selection_mode = StoredValue::new(selection_mode);
    let selection_mode = Signal::derive(move || selection_mode.get_value());
    let get_row_id = StoredValue::new(get_row_id);
    let get_tree_path = StoredValue::new(get_tree_path);
    let row_grouping = StoredValue::new(row_grouping.unwrap_or_default());
    let aggregation = StoredValue::new(aggregation.unwrap_or_default());
    let aggregation_position = StoredValue::new(aggregation_position);
    let pivot = StoredValue::new(pivot.unwrap_or_default());
    let list_view = StoredValue::new(list_view);
    let toolbar_slot = data_table_toolbar.or(data_table_toolbar_slot);
    let footer_slot = data_table_footer.or(data_table_footer_slot);
    let slots = DataTableSlots::from_slot_props(
        toolbar_slot,
        footer_slot,
        data_table_empty_view,
        data_table_no_results_view,
        data_table_loading_view,
        data_table_row_detail,
    );
    let row_detail = StoredValue::new(slots.row_detail_view(row_detail));
    let slots_for_root = slots;
    let get_row_class = StoredValue::new(get_row_class);
    let locale = StoredValue::new(locale.unwrap_or_default());
    let pagination_display = StoredValue::new(pagination_display);
    let page_size_options = StoredValue::new(page_size_options);
    let direction_override = StoredValue::new(dir);
    let toolbar_config = StoredValue::new(toolbar_config);
    let header_chrome = StoredValue::new(header_chrome);
    let client_loading = loading.unwrap_or_else(|| RwSignal::new(false));
    let sort = sort.unwrap_or_else(|| Signal::derive(|| None));
    let filter = filter.unwrap_or_else(|| Signal::derive(|| None));
    let pagination = pagination.unwrap_or_else(|| Signal::derive(|| None));
    let selection = selection.unwrap_or_else(|| Signal::derive(|| None));
    let server_fetch_policy = StoredValue::new(server_fetch_policy);
    let refresh_signal = refresh_signal.unwrap_or_else(|| Signal::derive(|| 0u32));

    view! {
        <DataTableRoot
            class=class
            columns=columns
            column_groups=column_groups
            data_source=data_source
            paging=paging
            features=features
            sortable=sortable
            resizable_columns=resizable_columns
            header_height=header_height
            height=height
            max_height=max_height
            flex=flex
            auto_row_height=auto_row_height
            get_row_id=get_row_id
            get_tree_path=get_tree_path
            row_grouping=row_grouping
            aggregation=aggregation
            aggregation_position=aggregation_position
            pivot=pivot
            list_view=list_view
            row_detail=row_detail
            locale=locale
            pagination_display=pagination_display
            page_size_options=page_size_options
            slots=slots_for_root
            get_row_class=get_row_class
            direction_override=direction_override
            client_loading=client_loading
            selection_mode=selection_mode
            initial_state=initial_state
            controlled_sort=sort
            controlled_filter=filter
            controlled_pagination=pagination
            controlled_selection=selection
            events=resolved_events
            edit_mode=edit_mode
            show_undo_toolbar=show_undo_toolbar
            toolbar_config=toolbar_config
            header_chrome=header_chrome
            server_fetch_policy=server_fetch_policy
            refresh_signal=refresh_signal
            children=children
        />
    }
}
