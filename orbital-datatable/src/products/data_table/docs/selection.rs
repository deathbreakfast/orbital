use leptos::prelude::*;
use orbital_macros::component_doc;

/// Row multiselect with shift-click ranges and spreadsheet-style cell range selection.
///
/// Set `selection_mode` for row checkboxes; enable `CELL_SELECTION` on `features` for cell ranges.
///
/// # When to use
///
/// - Bulk actions on selected rows
/// - Copying rectangular cell regions
///
/// # Usage
///
/// 1. Set `selection_mode=DataTableSelectionMode::Multiselect` for row selection.
/// 2. Enable `DataTableFeatures::CELL_SELECTION` for cell ranges.
/// 3. Bind `selection` signal or listen to `on_selection_change` for controlled mode.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use shift-click for contiguous row ranges in long lists
/// * Pair cell selection with `CLIPBOARD` for copy/paste workflows on the [Export](/data-table-export) page
///
/// ## Don'ts
///
/// * Do not use `selection_mode` just to highlight the open row; `active_row_id` does that without a checkbox column
/// * Do not enable cell selection on mobile-only layouts without keyboard alternatives
///
/// # Selection reference
///
/// | Type / prop | Description |
/// |-------------|-------------|
/// | `selection_mode` | `Single` or `Multiselect` row checkboxes |
/// | `selection` | Controlled set of selected row ids |
/// | `active_row_id` | Row to highlight and mark `aria-current`, independent of selection |
/// | `CELL_SELECTION` | Spreadsheet-style rectangular cell ranges |
/// | [`CellSelection`] | Anchor and focus coords for cell range |
///
///
/// # Examples
///
///
/// ## Shift range selection
/// Multiselect with shift-click to select a contiguous row range.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, DataTableSelectionMode};
/// let items = RwSignal::new((1..=8).map(|i| {
///     DataTableRowModel::from_text_cells(
///         i.to_string(),
///         HashMap::from([("name".into(), format!("Row {i}"))]),
///     )
/// }).collect());
/// view! {
///     <div data-testid="data-table-range-select-preview">
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
/// ## Cell range selection
/// Spreadsheet-style cell ranges with shift-click and arrow keys (`CELL_SELECTION`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into()), ("role".into(), "Admin".into())])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([("name".into(), "Grace".into()), ("role".into(), "Editor".into())])),
///     DataTableRowModel::from_text_cells("3", HashMap::from([("name".into(), "Bob".into()), ("role".into(), "Member".into())])),
/// ]);
/// view! {
///     <div data-testid="data-table-range-selection-preview">
///         <DataTable
///             features=DataTableFeatures::CELL_SELECTION
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Active row
/// Highlight the row open in a side panel without a selection column. Press `j` / `k` to move it.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use orbital_base_components::{use_hotkeys, Hotkey};
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel};
/// let items = RwSignal::new((1..=5).map(|i| {
///     DataTableRowModel::from_text_cells(
///         i.to_string(),
///         HashMap::from([("name".into(), format!("Row {i}"))]),
///     )
/// }).collect());
/// let active = RwSignal::new(Some("1".to_string()));
/// let step = move |delta: i32| {
///     let next = active.get_untracked().and_then(|id| id.parse::<i32>().ok()).map_or(1, |n| (n + delta).clamp(1, 5));
///     active.set(Some(next.to_string()));
/// };
/// use_hotkeys(vec![Hotkey::new("j", move || step(1)), Hotkey::new("k", move || step(-1))], true);
/// view! {
///     <div data-testid="data-table-active-row-preview">
///         <DataTable
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///             active_row_id=active
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group = "Selection & IO",
    group_priority = 70,
    preview_slug = "data-table-selection",
    preview_label = "Selection",
    preview_icon = icondata::AiCheckSquareOutlined,
)]
#[component]
pub fn DataTableSelectionDoc() -> impl IntoView {
    view! { () }
}
