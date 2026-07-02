use leptos::prelude::*;
use orbital_macros::component_doc;

/// Hierarchical tree rows, group-by headers, footer aggregates, pivot cross-tabs, and responsive list view.
///
/// Each capability is opt-in via [`DataTableFeatures`] and its matching config struct.
///
/// # When to use
///
/// - Org charts and nested categories (`TREE_DATA`)
/// - Group-by reports with subtotals (`ROW_GROUPING`, `AGGREGATION`)
/// - Cross-tab analysis (`PIVOTING`)
/// - Mobile-friendly card lists (`LIST_VIEW`)
///
/// # Usage
///
/// 1. Enable the relevant flag on `features`.
/// 2. Pass `get_tree_path`, `row_grouping`, `aggregation`, `pivot`, or `list_view` props.
/// 3. Read processed output via [`DataTableHandle::get_processed_dataset`] for downstream charts
///    — see [Charts Integration](/data-table-charts-integration) for the live binding pattern.
///
/// # Best Practices
///
/// ## Do's
///
/// * Start with row grouping before pivot for simpler mental models
/// * Use list view only when horizontal columns do not fit
///
/// ## Don'ts
///
/// * Do not combine pivot and tree data without testing performance on your dataset size
///
///
/// # Examples
///
///
/// ## Tree data
/// Hierarchical rows with expand/collapse grouping column (`TREE_DATA`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, GetTreePath, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([("name".into(), DataValue::Text("Engineering".into()))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("name".into(), DataValue::Text("Frontend".into()))]))),
///     DataTableRowModel::new(DataRecord::new("3", HashMap::from([("name".into(), DataValue::Text("Backend".into()))]))),
/// ]);
/// let get_tree_path = GetTreePath::new(|(record,)| match record.id.as_str() {
///     "1" => vec!["Org".into()],
///     "2" => vec!["Org".into(), "Eng".into()],
///     "3" => vec!["Org".into(), "Eng".into(), "BE".into()],
///     _ => vec![record.id.clone()],
/// });
/// view! {
///     <div data-testid="data-table-tree-preview">
///         <DataTable
///             features=DataTableFeatures::TREE_DATA
///             get_tree_path=get_tree_path
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Row grouping
/// Group rows by column with expand/collapse headers (`ROW_GROUPING`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowGrouping, DataTableRowModel, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///         ("company".into(), DataValue::Text("Acme".into())),
///         ("name".into(), DataValue::Text("Ada".into())),
///     ]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([
///         ("company".into(), DataValue::Text("Acme".into())),
///         ("name".into(), DataValue::Text("Bob".into())),
///     ]))),
///     DataTableRowModel::new(DataRecord::new("3", HashMap::from([
///         ("company".into(), DataValue::Text("Globex".into())),
///         ("name".into(), DataValue::Text("Carol".into())),
///     ]))),
/// ]);
/// view! {
///     <div data-testid="data-table-grouping-preview">
///         <DataTable
///             features=DataTableFeatures::ROW_GROUPING
///             row_grouping=DataTableRowGrouping::new(vec!["company".into()])
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("company", "Company"),
///                 DataTableColumnDef::new("name", "Name"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Summaries
/// Footer aggregate totals for numeric columns (`AGGREGATION`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{AggregationFn, AggregationModel, AggregationRule, DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, ColumnType, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([("item".into(), DataValue::Text("A".into())), ("amount".into(), DataValue::Number(400.0))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("item".into(), DataValue::Text("B".into())), ("amount".into(), DataValue::Number(350.0))]))),
///     DataTableRowModel::new(DataRecord::new("3", HashMap::from([("item".into(), DataValue::Text("C".into())), ("amount".into(), DataValue::Number(450.0))]))),
/// ]);
/// view! {
///     <div data-testid="data-table-summaries-preview">
///         <DataTable
///             features=DataTableFeatures::AGGREGATION
///             aggregation=AggregationModel::new(vec![AggregationRule { field: "amount".into(), func: AggregationFn::Sum }])
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("item", "Item"),
///                 DataTableColumnDef::new("amount", "Amount").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Pivot
/// Cross-tab pivot with dynamic columns and configuration panel (`PIVOTING`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, ColumnType, PagingMode};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([("region".into(), DataValue::Text("East".into())), ("amount".into(), DataValue::Number(100.0))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("region".into(), DataValue::Text("West".into())), ("amount".into(), DataValue::Number(200.0))]))),
/// ]);
/// view! {
///     <div data-testid="data-table-pivot-preview">
///         <DataTable
///             features=DataTableFeatures::PIVOTING
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("region", "Region"),
///                 DataTableColumnDef::new("amount", "Amount").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## List view
/// Responsive card layout instead of table rows (`LIST_VIEW`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableEvents, DataTableFeatures, DataTableRowModel, DataTableSelectionMode, ListViewConfig, PagingMode};
/// use leptos::prelude::*;
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///         ("name".into(), DataValue::Text("Ada Lovelace".into())),
///         ("role".into(), DataValue::Text("Engineer".into())),
///     ]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([
///         ("name".into(), DataValue::Text("Grace Hopper".into())),
///         ("role".into(), DataValue::Text("Scientist".into())),
///     ]))),
/// ]);
/// let log = RwSignal::new(String::new());
/// let events = DataTableEvents {
///     on_row_click: Some(Callback::new(move |(id,)| log.set(format!("row_click:{id}")))),
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="data-table-list-view-preview">
///         <div data-testid="data-table-list-view-log">{move || log.get()}</div>
///         <DataTable
///             features=DataTableFeatures::LIST_VIEW
///             list_view=ListViewConfig::new("name").with_secondary_fields(vec!["role".into()])
///             paging=PagingMode::None
///             selection_mode=DataTableSelectionMode::Multiselect
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
    group_priority = 90,
    preview_slug = "data-table-advanced",
    preview_label = "Tree, Grouping & Pivot",
    preview_icon = icondata::AiApartmentOutlined,
)]
#[component]
pub fn DataTableAdvancedDoc() -> impl IntoView {
    view! { () }
}
