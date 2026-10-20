use leptos::prelude::*;
use orbital_macros::component_doc;

/// Column layout and interaction features: sizing, visibility, pinning, reorder, groups, and spanning.
///
/// Enable capability flags via [`DataTableFeatures`] and configure header chrome with [`DataTableHeaderChromeConfig`] (filter button, menu, hide).
///
/// # When to use
///
/// - Resizable or pinned columns in wide datasets
/// - User-controlled column visibility via the picker
/// - Multi-level grouped headers
///
/// # Usage
///
/// 1. Enable `COLUMN_PINNING`, `COLUMN_REORDER`, or related flags on `features`.
/// 2. Set column widths with [`ColumnWidth::Fixed`] or [`ColumnWidth::Flex`].
///    Columns default to `Flex(1.0)`: the table measures its container and shares
///    the width left over by fixed columns by weight, refilling on window resize.
///    Until the first measurement (server render) it assumes 800px.
/// 3. Gate toolbar picker and header menu via `toolbar_config` and `header_chrome`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `resizable_columns=true`; dragging a flex column pins it at the dragged
///   width while the other flex columns keep filling the table
/// * Pin identifier columns left in horizontally scrollable tables
/// * Use [`DataTableColumnGroupDef`] for two-level headers only when needed
///
/// ## Don'ts
///
/// * Do not pin every column — leave at least one scrollable region
/// * Do not enable `column_hide: false` and expect the picker to show hide toggles
///
/// # Column features reference
///
/// | Type / prop | Description |
/// |-------------|-------------|
/// | [`DataTableHeaderChromeConfig`] | Per-header filter, menu, and hide toggles |
/// | [`DataTableColumnGroupDef`] | Two-level grouped header labels |
/// | `resizable_columns` | Drag resize handles on column headers |
/// | `COLUMN_PINNING`, `COLUMN_REORDER` | Pin and drag-reorder columns |
///
/// See the [Overview](/data-table) page for [`DataTableToolbarConfig`] (picker, export, quick search).
///
///
/// # Examples
///
///
/// ## Column sizing and resize
/// Fixed and flex columns with drag resize handles on headers.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnWidth, DataTable, DataTableColumnDef, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada Lovelace".into()),
///         ("role".into(), "Admin".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-column-sizing-preview">
///         <DataTable
///             resizable_columns=true
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_width(ColumnWidth::Fixed(180.0)),
///                 DataTableColumnDef::new("role", "Role").with_width(ColumnWidth::Flex(1.0)),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column visibility
/// Hide columns via initial state (`column_visibility`).
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableInitialState, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("role".into(), "Admin".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-column-visibility-preview">
///         <DataTable
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///             initial_state=DataTableInitialState {
///                 column_visibility: HashMap::from([("role".into(), false)]),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column picker
/// Toggle column visibility from the toolbar Columns popover.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("email".into(), "ada@example.com".into()),
///         ("role".into(), "Admin".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-column-picker-preview">
///         <DataTable
///             features=DataTableFeatures::COLUMN_REORDER
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("email", "Email"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column menu
/// Per-header menu for sort, filter, pin, and hide actions.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("role".into(), "Editor".into()),
///     ])),
///     DataTableRowModel::from_text_cells("2", HashMap::from([
///         ("name".into(), "Grace".into()),
///         ("role".into(), "Admin".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-column-menu-preview">
///         <DataTable
///             features=DataTableFeatures::COLUMN_PINNING
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
/// ## Column pinning
/// Sticky left/right columns when scrolling horizontally.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     ColumnWidth, DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PinSide,
/// };
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("dept".into(), "Engineering".into()),
///         ("location".into(), "London".into()),
///         ("notes".into(), "Long notes column for scroll".into()),
///     ])),
/// ]);
/// view! {
///     <div
///         data-testid="data-table-column-pinning-preview"
///         style="max-width: 420px; width: 100%;"
///     >
///         <DataTable
///             features=DataTableFeatures::COLUMN_PINNING
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_pinned(PinSide::Left).with_width(ColumnWidth::Fixed(120.0)),
///                 DataTableColumnDef::new("dept", "Department").with_width(ColumnWidth::Fixed(160.0)),
///                 DataTableColumnDef::new("location", "Location").with_width(ColumnWidth::Fixed(160.0)),
///                 DataTableColumnDef::new("notes", "Notes").with_width(ColumnWidth::Fixed(320.0)),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column reorder
/// Drag column headers to reorder when `COLUMN_REORDER` is enabled.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("role".into(), "Admin".into()),
///         ("dept".into(), "Eng".into()),
///     ])),
/// ]);
/// view! {
///     <div data-testid="data-table-column-reorder-preview">
///         <DataTable
///             features=DataTableFeatures::COLUMN_REORDER
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///                 DataTableColumnDef::new("dept", "Department"),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column groups
/// Two-level grouped headers over leaf columns.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     ColumnWidth, DataTable, DataTableColumnDef, DataTableColumnGroupChild,
///     DataTableColumnGroupDef, DataTableRowModel,
/// };
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("email".into(), "ada@example.com".into()),
///         ("dept".into(), "Eng".into()),
///         ("title".into(), "Lead".into()),
///     ])),
/// ]);
/// let groups = vec![
///     DataTableColumnGroupDef::new("personal", "Personal").with_children(vec![
///         DataTableColumnGroupChild::column("name"),
///         DataTableColumnGroupChild::column("email"),
///     ]),
///     DataTableColumnGroupDef::new("work", "Work").with_children(vec![
///         DataTableColumnGroupChild::column("dept"),
///         DataTableColumnGroupChild::column("title"),
///     ]),
/// ];
/// view! {
///     <div
///         data-testid="data-table-column-groups-preview"
///         style="max-width: 480px; width: 100%;"
///     >
///         <DataTable
///             column_groups=groups
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name").with_width(ColumnWidth::Fixed(120.0)),
///                 DataTableColumnDef::new("email", "Email").with_width(ColumnWidth::Fixed(180.0)),
///                 DataTableColumnDef::new("dept", "Department").with_width(ColumnWidth::Fixed(150.0)),
///                 DataTableColumnDef::new("title", "Title").with_width(ColumnWidth::Fixed(150.0)),
///             ]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Column spanning
/// Body cells span multiple columns via `col_span`.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{ColumnWidth, DataTable, DataTableColumnDef, DataTableRowModel};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         (
///             "summary".into(),
///             "Merged summary cell spanning Summary and Detail A".into(),
///         ),
///         ("detail_a".into(), "A".into()),
///         ("detail_b".into(), "B".into()),
///     ])),
/// ]);
/// view! {
///     <div
///         data-testid="data-table-col-span-preview"
///         style="max-width: 480px; width: 100%;"
///     >
///         <DataTable
///             columns=vec![
///                 DataTableColumnDef::new("summary", "Summary")
///                     .with_col_span(2)
///                     .with_width(ColumnWidth::Fixed(200.0)),
///                 DataTableColumnDef::new("detail_a", "Detail A").with_width(ColumnWidth::Fixed(180.0)),
///                 DataTableColumnDef::new("detail_b", "Detail B").with_width(ColumnWidth::Fixed(140.0)),
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
    preview_slug = "data-table-columns",
    preview_label = "Column Features",
    preview_icon = icondata::AiColumnHeightOutlined,
)]
#[component]
pub fn DataTableColumnsDoc() -> impl IntoView {
    view! { () }
}
