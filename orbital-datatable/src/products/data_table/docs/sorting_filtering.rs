use leptos::prelude::*;
use orbital_macros::component_doc;

/// Client-side and controlled sort/filter models with multi-column sort, typed operators, and header filter rows.
///
/// Bind [`DataTableSort`] and [`DataTableFilter`] for controlled mode, or let the table manage state internally.
///
/// # When to use
///
/// - Multi-key sort (role then name)
/// - Structured filter panel with per-type operators
/// - Inline filters under column headers
///
/// # Usage
///
/// 1. Enable `MULTI_COLUMN_SORT`, `MULTI_FILTER`, or `HEADER_FILTERS` on `features`.
/// 2. Pass controlled `sort` / `filter` signals for parent-owned state.
/// 3. Use quick search on the Overview page for simple text search.
///
/// # Best Practices
///
/// ## Do's
///
/// * Match [`FilterOperator`] to [`ColumnType`] for correct comparisons
/// * Use `FilterLogic::Or` for optional match-any rules
///
/// ## Don'ts
///
/// * Do not mix uncontrolled filter state with server fetchers without syncing query params
///
/// # Sort & filter reference
///
/// | Type | Field | Description |
/// |------|-------|-------------|
/// | [`DataTableSort`] | `items` | Ordered [`SortRule`] list (index 0 = primary) |
/// | [`SortRule`] | `field`, `direction` | Column key and asc/desc |
/// | [`DataTableFilter`] | `items`, `logic` | Filter rules and `And` / `Or` combination |
/// | [`FilterRule`] | `field`, `operator`, `value` | Single column constraint |
/// | `quick_search` | — | Cross-column text search (Overview) |
///
///
/// # Examples
///
///
/// ## Multi-column sort
/// Ctrl/Cmd+click column headers to stack sort priorities (`MULTI_COLUMN_SORT`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("role".into(), "Admin".into()), ("name".into(), "Zed".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("role".into(), "Admin".into()), ("name".into(), "Ada".into())])),
///     DataTableRowModel::from_text_cells("3", HashMap::from([("role".into(), "Member".into()), ("name".into(), "Bob".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-multi-sort-preview">
///         <DataTable
///             features=DataTableFeatures::MULTI_COLUMN_SORT
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("role", "Role"),
///                 DataTableColumnDef::new("name", "Name"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Typed filtering
/// Filter panel with operators per column type.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([("name".into(), DataValue::Text("Alpha".into())), ("score".into(), DataValue::Number(10.0))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("name".into(), DataValue::Text("Beta".into())), ("score".into(), DataValue::Number(42.0))]))),
/// ]);
/// view! {
///     <div data-testid="data-table-filtering-preview">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("score", "Score").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Filter panel
/// Popover UI for structured column filters.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel, FilterChoice, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into()), ("status".into(), "Active".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into()), ("status".into(), "Archived".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-filter-panel-preview">
///         <DataTable paging=PagingMode::None columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role"), DataTableColumnDef::new("status", "Status").with_col_type(ColumnType::SingleSelect).with_filter_options(["Active", "Archived"].map(FilterChoice::same))] items=items />
///     </div>
/// }
/// ```
///
///
/// ## Filter logic AND/OR
/// Combine multiple filter rules with AND or OR (`MULTI_FILTER`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new((1..=6).map(|i| {
///     let role = if i % 2 == 0 { "Admin" } else { "Member" };
///     DataTableRowModel::from_text_cells(i.to_string(), HashMap::from([("name".into(), format!("User {i}")), ("role".into(), role.into())]))
/// }).collect());
/// view! {
///     <div data-testid="data-table-filter-logic-preview">
///         <DataTable features=DataTableFeatures::MULTI_FILTER paging=PagingMode::None columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")] items=items />
///     </div>
/// }
/// ```
///
///
/// ## Header filters
/// Inline filter row under column headers (`HEADER_FILTERS`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, FilterChoice, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into()), ("status".into(), "Active".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into()), ("status".into(), "Archived".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-header-filters-preview">
///         <DataTable features=DataTableFeatures::HEADER_FILTERS paging=PagingMode::None columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role"), DataTableColumnDef::new("status", "Status").with_col_type(ColumnType::SingleSelect).with_filter_options(["Active", "Archived"].map(FilterChoice::same))] items=items />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 50,
    preview_slug = "data-table-sorting-filtering",
    preview_label = "Sorting & Filtering",
    preview_icon = icondata::AiFilterOutlined,
)]
#[component]
pub fn DataTableSortingFilteringDoc() -> impl IntoView {
    view! { () }
}
