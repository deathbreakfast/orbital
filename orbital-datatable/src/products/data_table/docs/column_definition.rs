use leptos::prelude::*;
use orbital_macros::component_doc;

/// Defines how each column binds to dataset fields, formats values, and renders cells.
///
/// Use [`DataTableColumnDef`] for every column in the `columns` prop. Set [`ColumnType`] for typed sort and filter behavior, and attach custom views with `cell_view` or `header_view`.
///
/// # When to use
///
/// - Defining the column schema for a new table
/// - Custom cell rendering (badges, links, formatted numbers)
/// - Typed sort and filter operators per column
///
/// # When to use Column Features instead
///
/// Column sizing, pinning, reorder, groups, and visibility live on the [Column Features](/data-table-columns) page.
///
/// # Usage
///
/// 1. Create one [`DataTableColumnDef`] per visible column with `field` matching a [`FieldDef`](crate::FieldDef) key.
/// 2. Set [`ColumnType`] when the column is not plain text.
/// 3. Attach `cell_view` or `header_view` for custom rendering.
/// 4. Pass the vec to `DataTable` via the `columns` prop.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind `field` to stable schema keys from your [`DataSchema`](crate::DataSchema)
/// * Use [`ColumnType::Number`] for numeric sort (2 before 10)
/// * Keep `cell_view` closures lightweight — defer heavy work to async data
///
/// ## Don'ts
///
/// * Do not use raw HTML in cells when an Orbital component (`Badge`, `Link`) fits
/// * Do not duplicate column defs across tables — extract shared builders
///
/// # Column definition reference
///
/// | Field | Type | Default | Description |
/// |-------|------|---------|-------------|
/// | `field` | `String` | required | Schema key this column displays |
/// | `header` | `String` | required | Column header label |
/// | `col_type` | `ColumnType` | `Text` | Typed sort, filter, and edit behavior |
/// | `width` | `ColumnWidth` | `Flex(1.0)` | Fixed, flex (fills the table, resizable), or auto width |
/// | `editable` | `bool` | `false` | Allow inline editing |
/// | `cell_view` | `Option<Arc<...>>` | `None` | Custom cell renderer |
/// | `header_view` | `Option<Arc<...>>` | `None` | Custom header renderer |
///
/// See [`DataTableColumnDef`] rustdoc for the full field list.
///
///
/// # Examples
///
///
/// ## Custom status column
/// <!-- default -->
/// Status cells render as themed badges via `cell_view`.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel};
/// use orbital_core_components::{Badge, BadgeAppearance, BadgeColor};
/// use orbital_data::DataValue;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("status".into(), "Active".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("status".into(), "Pending".into())])),
/// ]);
/// let status_view = Arc::new(|record: orbital_data::DataRecord| {
///     let label = record.get("status").map(DataValue::display_string).unwrap_or_default();
///     let color = if label == "Active" { BadgeColor::Success } else { BadgeColor::Warning };
///     view! {
///         <Badge appearance=BadgeAppearance::Filled color=color>{label}</Badge>
///     }.into_any()
/// });
/// view! {
///     <div data-testid="data-table-custom-columns-preview">
///         <DataTable
///             columns=vec![
///                 DataTableColumnDef::new("status", "Status")
///                     .with_col_type(ColumnType::SingleSelect)
///                     .with_cell_view(status_view),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Typed numeric sort
/// Number columns sort numerically (`2` before `10`), not lexicographically.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnType, DataTable, DataTableColumnDef, DataTableRowModel};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::new(DataRecord::new("10", HashMap::from([("amount".into(), DataValue::Number(10.0))]))),
///     DataTableRowModel::new(DataRecord::new("2", HashMap::from([("amount".into(), DataValue::Number(2.0))]))),
/// ]);
/// view! {
///     <div data-testid="data-table-typed-sort-preview">
///         <DataTable
///             columns=vec![
///                 DataTableColumnDef::new("amount", "Amount").with_col_type(ColumnType::Number),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Custom column header
/// Custom `header_view` and description tooltip on a column.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel};
/// use orbital_core_components::Badge;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("score".into(), "95".into())])),
/// ]);
/// let header = Arc::new(|| view! { <Badge>"Score %"</Badge> }.into_any());
/// view! {
///     <div data-testid="data-table-column-header-preview">
///         <DataTable
///             columns=vec![
///                 DataTableColumnDef::new("score", "Score")
///                     .with_description("Percentage score out of 100")
///                     .with_header_view(header),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group = "Columns",
    group_priority = 20,
    preview_slug = "data-table-column-definition",
    preview_label = "Column Definition",
    preview_icon = icondata::AiColumnWidthOutlined,
)]
#[component]
pub fn DataTableColumnDefinitionDoc() -> impl IntoView {
    view! { () }
}
