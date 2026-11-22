use leptos::prelude::*;
use orbital_macros::component_doc;

/// Controlled sort/filter/pagination/selection, JSON state round-trip, imperative handle, and event callbacks.
///
/// Use [`DataTableInitialState`] for one-time setup and controlled props for parent-owned models.
///
/// # When to use
///
/// - Parent components that own table state
/// - Save/restore user grid preferences
/// - Toolbar buttons that drive sort/filter/scroll imperatively
///
/// # Usage
///
/// 1. Pass `sort`, `filter`, `pagination`, and `selection` signals for controlled mode.
/// 2. Wire [`DataTableEvents`] callbacks to keep parent state in sync.
/// 3. Capture [`DataTableHandle`] via `data_table_events.on_handle` for imperative methods.
/// 4. Call `export_state` / `restore_state` on [`DataTableState`] for persistence.
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer event callbacks over polling handle state
/// * Serialize state with [`SerializedState`] for stable JSON keys
///
/// ## Don'ts
///
/// * Do not mix `initial_state` and controlled props for the same field without a clear precedence rule
///
/// # State reference
///
/// | Type | Field | Description |
/// |------|-------|-------------|
/// | [`DataTableInitialState`] | `sort`, `filter`, `quick_search`, `pagination` | One-time defaults applied at mount |
/// | | `column_visibility`, `pinned_columns`, `pinned_rows`, `selection` | Layout and selection defaults |
/// | [`DataTableState`] | same fields | Full snapshot for export/restore |
/// | `quick_search` prop | `RwSignal<String>` | Binds the built-in search box to your signal (URL sync) |
/// | [`DataTableHandle`] | `sort_column`, `set_filter`, `set_quick_search` | Imperative sort/filter/search |
/// | | `export_state`, `restore_state`, `export_csv` | Persistence and export |
/// | | `scroll_to_row`, `scroll_to_column`, `get_processed_dataset` | Scroll and processed data |
/// | [`DataTableEvents`] | `on_sort_change`, `on_filter_change`, `on_selection_change` | Controlled sync callbacks |
///
///
/// # Examples
///
///
/// ## Controlled models
/// Parent-owned sort, filter, pagination, and selection with two-way sync via event callbacks.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::{HashMap, HashSet};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use orbital_data::DataValue;
/// use crate::engine::SortDirection;
/// use crate::products::data_table::phase9_previews::DataTablePreviewControls;
/// use crate::types::{
///     DataTableColumnDef, DataTableEvents, DataTableFilter, DataTableRowModel, DataTableSort,
///     FilterLogic, FilterOperator, FilterRule, PaginationState, SortRule,
/// };
/// use crate::{DataTable, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells(
///         "1",
///         HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "2",
///         HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "3",
///         HashMap::from([("name".into(), "Bob".into()), ("role".into(), "Admin".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "4",
///         HashMap::from([("name".into(), "Carol".into()), ("role".into(), "Admin".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "5",
///         HashMap::from([("name".into(), "Dan".into()), ("role".into(), "Admin".into())]),
///     ),
/// ]);
/// let sort = RwSignal::new(Some(DataTableSort::default()));
/// let filter = RwSignal::new(Some(DataTableFilter::default()));
/// let pagination = RwSignal::new(Some(PaginationState { page: 0, page_size: 2 }));
/// let selection = RwSignal::new(Some(HashSet::<String>::new()));
/// let sort_label = RwSignal::new(String::from("none"));
/// let events = DataTableEvents {
///     on_sort_change: Some(Callback::new(move |s: DataTableSort| {
///         sort.set(Some(s.clone()));
///         if let Some(r) = s.items.first() {
///             let dir = if r.direction == SortDirection::Asc { "asc" } else { "desc" };
///             sort_label.set(format!("{} {}", r.field, dir));
///         }
///     })),
///     on_filter_change: Some(Callback::new(move |f: DataTableFilter| filter.set(Some(f)))),
///     on_pagination_change: Some(Callback::new(move |p: PaginationState| pagination.set(Some(p)))),
///     on_selection_change: Some(Callback::new(move |s: HashSet<String>| selection.set(Some(s)))),
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="data-table-controlled-preview">
///         <div class="orbital-data-table__controlled-toolbar">
///             <span data-testid="controlled-sort-label">{move || sort_label.get()}</span>
///             <DataTablePreviewControls>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="controlled-sort-desc"
///                     on_click=Callback::new(move |_| {
///                         sort.set(Some(DataTableSort {
///                             items: vec![SortRule {
///                                 field: "name".into(),
///                                 direction: SortDirection::Desc,
///                             }],
///                         }));
///                         sort_label.set("name desc".into());
///                     })
///                 >
///                     "Sort Name DESC"
///                 </Button>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="controlled-filter-admin"
///                     on_click=Callback::new(move |_| {
///                         filter.set(Some(DataTableFilter {
///                             items: vec![FilterRule {
///                                 field: "role".into(),
///                                 operator: FilterOperator::Equals,
///                                 value: DataValue::Text("Admin".into()),
///                             }],
///                             logic: FilterLogic::And,
///                         }));
///                     })
///                 >
///                     "Filter Admin"
///                 </Button>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="controlled-page-2"
///                     on_click=Callback::new(move |_| {
///                         pagination.set(Some(PaginationState { page: 1, page_size: 2 }));
///                     })
///                 >
///                     "Page 2"
///                 </Button>
///             </DataTablePreviewControls>
///         </div>
///         <DataTable
///             sort=Signal::derive(move || sort.get())
///             filter=Signal::derive(move || filter.get())
///             pagination=Signal::derive(move || pagination.get())
///             selection=Signal::derive(move || selection.get())
///             events=events
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
/// ## State export and restore
/// Capture the full table state as JSON, reset, and restore for round-trip persistence.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use crate::engine::SortDirection;
/// use crate::products::data_table::phase9_previews::DataTablePreviewControls;
/// use crate::types::{
///     DataTableColumnDef, DataTableEvents, DataTableFilter, DataTableHandle, DataTableRowModel, SerializedState,
/// };
/// use crate::DataTable;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells(
///         "1",
///         HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "2",
///         HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())]),
///     ),
/// ]);
/// let handle = RwSignal::new(None::<DataTableHandle>);
/// let saved = RwSignal::new(None::<SerializedState>);
/// let snapshot_label = RwSignal::new(String::new());
/// view! {
///     <div data-testid="data-table-state-export-preview">
///         <div>
///             <DataTablePreviewControls>
///                 <Button
///                     appearance=ButtonAppearance::Primary
///                     attr:data-testid="export-state"
///                     on_click=Callback::new(move |_| {
///                         if let Some(h) = handle.get() {
///                             let snap = h.export_state.run(());
///                             snapshot_label.set(snap.0.clone());
///                             saved.set(Some(snap));
///                         }
///                     })
///                 >
///                     "Export state"
///                 </Button>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="reset-state"
///                     on_click=Callback::new(move |_| {
///                         if let Some(h) = handle.get() {
///                             h.set_quick_search.run(("".into(),));
///                             h.sort_column.run(("name".into(), SortDirection::Asc));
///                             h.set_filter.run((DataTableFilter::default(),));
///                         }
///                     })
///                 >
///                     "Reset"
///                 </Button>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="restore-state"
///                     on_click=Callback::new(move |_| {
///                         if let (Some(h), Some(snap)) = (handle.get(), saved.get()) {
///                             h.restore_state.run((snap,));
///                         }
///                     })
///                 >
///                     "Restore"
///                 </Button>
///             </DataTablePreviewControls>
///             <span data-testid="serialized-state">{move || snapshot_label.get()}</span>
///         </div>
///         <DataTable
///             sortable=true
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///             data_table_events=DataTableEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
///
/// ## Programmatic handle
/// Imperative sort, filter, search, and scroll via [`DataTableHandle`].
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use orbital_data::DataValue;
/// use crate::engine::SortDirection;
/// use crate::products::data_table::phase9_previews::DataTablePreviewControls;
/// use crate::types::{
///     DataTableColumnDef, DataTableEvents, DataTableFilter, DataTableHandle, DataTableRowModel, FilterLogic,
///     FilterOperator, FilterRule,
/// };
/// use crate::{DataTable, PagingMode};
/// let items = RwSignal::new(
///     (0..30)
///         .map(|i| {
///             DataTableRowModel::from_text_cells(
///                 &i.to_string(),
///                 HashMap::from([
///                     ("name".into(), format!("Row {i}")),
///                     ("email".into(), format!("row{i}@example.com")),
///                 ]),
///             )
///         })
///         .collect::<Vec<_>>(),
/// );
/// let handle = RwSignal::new(None::<DataTableHandle>);
/// view! {
///     <div data-testid="data-table-handle-preview">
///         <DataTablePreviewControls>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="handle-sort-name"
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.sort_column.run(("name".into(), SortDirection::Desc));
///                     }
///                 })
///             >
///                 "Sort name DESC"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="handle-filter-admin"
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.set_filter.run((
///                             DataTableFilter {
///                                 items: vec![FilterRule {
///                                     field: "name".into(),
///                                     operator: FilterOperator::Contains,
///                                     value: DataValue::Text("Row 1".into()),
///                                 }],
///                                 logic: FilterLogic::And,
///                             },
///                         ));
///                     }
///                 })
///             >
///                 "Filter Row 1"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="handle-search-ada"
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.set_quick_search.run(("Row 5".into(),));
///                     }
///                 })
///             >
///                 "Search Row 5"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="handle-scroll-row-25"
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.scroll_to_row.run(("25".into(),));
///                     }
///                 })
///             >
///                 "Scroll row 25"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="handle-scroll-column-email"
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.scroll_to_column.run(("email".into(),));
///                     }
///                 })
///             >
///                 "Scroll email column"
///             </Button>
///         </DataTablePreviewControls>
///         <DataTable
///             max_height=200.0
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("email", "Email"),
///             ]
///             items=items
///             data_table_events=DataTableEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
///
/// ## Bound quick search
/// Pass `quick_search` to own the search text, for example to keep it in the URL.
/// Typing in the table writes your signal, and setting the signal filters the table.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use crate::products::data_table::phase9_previews::DataTablePreviewControls;
/// use crate::types::{DataTableColumnDef, DataTableRowModel};
/// use crate::{DataTable, PagingMode};
/// let items = RwSignal::new(
///     (0..12)
///         .map(|i| {
///             DataTableRowModel::from_text_cells(
///                 &i.to_string(),
///                 HashMap::from([("name".into(), format!("Row {i}"))]),
///             )
///         })
///         .collect::<Vec<_>>(),
/// );
/// let search = RwSignal::new(String::new());
/// view! {
///     <div data-testid="data-table-bound-search-preview">
///         <DataTablePreviewControls>
///             <span data-testid="bound-search-value">{move || format!("q={}", search.get())}</span>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="bound-search-set"
///                 on_click=Callback::new(move |_| search.set("Row 7".into()))
///             >
///                 "Search Row 7"
///             </Button>
///         </DataTablePreviewControls>
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///             quick_search=search
///         />
///     </div>
/// }
/// ```
///
///
/// ## Event callbacks
/// All [`DataTableEvents`] fire on real state changes; this preview logs each event.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::{HashMap, HashSet};
/// use leptos::prelude::*;
/// use crate::types::{
///     DataTableColumnDef, DataTableEvents, DataTableFilter, DataTableRowModel, DataTableSort,
///     PaginationState,
/// };
/// use crate::{DataTable, DataTableSelectionMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells(
///         "1",
///         HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())]),
///     ),
///     DataTableRowModel::from_text_cells(
///         "2",
///         HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())]),
///     ),
/// ]);
/// let log = RwSignal::new(Vec::<String>::new());
/// let events = DataTableEvents {
///     on_sort_change: Some(Callback::new(move |s: DataTableSort| {
///         log.update(|v| v.push(format!("sort: name")));
///         let _ = s;
///     })),
///     on_filter_change: Some(Callback::new(move |f: DataTableFilter| {
///         log.update(|v| v.push(format!("filter: {} rules", f.items.len())));
///     })),
///     on_pagination_change: Some(Callback::new(move |p: PaginationState| {
///         log.update(|v| v.push(format!("pagination: page {}", p.page + 1)));
///     })),
///     on_selection_change: Some(Callback::new(move |s: HashSet<String>| {
///         log.update(|v| v.push(format!("selection: {} rows", s.len())));
///     })),
///     on_row_click: Some(Callback::new(move |(id,)| {
///         log.update(|v| v.push(format!("row_click: {id}")));
///     })),
///     on_cell_click: Some(Callback::new(move |(row, field)| {
///         log.update(|v| v.push(format!("cell_click: {row}-{field}")));
///     })),
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="data-table-events-preview">
///         <div data-testid="event-log">{move || log.get().join("|")}</div>
///         <DataTable
///             selection_mode=DataTableSelectionMode::Multiselect
///             sortable=true
///             events=events
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 100,
    preview_slug = "data-table-state",
    preview_label = "State & Handle",
    preview_icon = icondata::AiControlOutlined,
)]
#[component]
pub fn DataTableStateDoc() -> impl IntoView {
    view! { () }
}
