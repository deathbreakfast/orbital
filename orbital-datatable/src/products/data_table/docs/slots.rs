use std::collections::HashMap;

use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, Select, SelectBind, Toolbar, ToolbarButton,
};
use orbital_data::{DataRecord, DataValue};
use orbital_macros::component_doc;

use crate::{
    DataTable, DataTableColumnDef, DataTableEmptyView, DataTableFooterSlot, DataTableRowModel,
    DataTableToolbarEnd, DataTableToolbarSlot, PagingMode,
};

/// Leptos slot regions for DataTable composition.
///
/// # When to use
///
/// - Adding a page's own controls (saved searches, a filter button, quick toggles) to the
///   default toolbar's search row with [`DataTableToolbarEnd`].
/// - Replacing the default toolbar, footer, or loading/empty overlays.
/// - Custom expandable row detail panels via [`DataTableRowDetail`].
///
/// # Usage
///
/// Nest slot children on [`DataTable`]: [`DataTableToolbarEnd`], [`DataTableToolbarSlot`], [`DataTableFooterSlot`],
/// [`DataTableEmptyView`], [`DataTableNoResultsView`], [`DataTableLoadingView`], and
/// [`DataTableRowDetail`]. Wire side effects through [`DataTableEvents`] and read table
/// state with [`use_data_table_table_state`].
///
/// # Examples
///
/// ## Saved searches on the search row
/// [`DataTableToolbarEnd`] content sits on the quick search row, at the end beside the
/// built-in Filters button. Search keeps the start of the row; the row wraps on narrow
/// screens.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, DataTableToolbarEnd, PagingMode};
/// use orbital_core_components::{Button, ButtonAppearance, Select, SelectBind};
/// use orbital_data::{DataRecord, DataValue};
/// let row = |id: &str, title: &str, status: &str| {
///     DataTableRowModel::new(DataRecord::new(id, HashMap::from([
///         ("title".into(), DataValue::Text(title.into())),
///         ("status".into(), DataValue::Text(status.into())),
///     ])))
/// };
/// let items = RwSignal::new(vec![
///     row("1", "Migrate card vault", "Doing"),
///     row("2", "Write card vault runbook", "To do"),
/// ]);
/// let saved = RwSignal::new("mine".to_string());
/// view! {
///     <div data-testid="data-table-toolbar-end-preview">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("title", "Title"), DataTableColumnDef::new("status", "Status")]
///             items=items
///         >
///             <DataTableToolbarEnd slot>
///                 <Button appearance=ButtonAppearance::Subtle>"Advanced"</Button>
///                 <Select bind=SelectBind { value: saved.into(), ..Default::default() }>
///                     <option value="all">"All (unsaved)"</option>
///                     <option value="mine">"My open tasks"</option>
///                 </Select>
///             </DataTableToolbarEnd>
///         </DataTable>
///     </div>
/// }
/// ```
///
/// ## Busy search row
/// With many controls the slot shrinks to its widest control and wraps onto more lines,
/// so the search box keeps its place at the start of the row.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, DataTableToolbarEnd, PagingMode};
/// use orbital_core_components::{Button, ButtonAppearance, Select, SelectBind};
/// use orbital_data::{DataRecord, DataValue};
/// let items = RwSignal::new(vec![DataTableRowModel::new(DataRecord::new("1", HashMap::from([
///     ("title".into(), DataValue::Text("Migrate card vault".into())),
///     ("status".into(), DataValue::Text("Doing".into())),
/// ])))]);
/// let saved = RwSignal::new("mine".to_string());
/// view! {
///     <div data-testid="data-table-toolbar-end-busy-preview" style="width: min(720px, 100%);">
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("title", "Title"), DataTableColumnDef::new("status", "Status")]
///             items=items
///         >
///             <DataTableToolbarEnd slot>
///                 <Button appearance=ButtonAppearance::Subtle>"Advanced"</Button>
///                 <Button appearance=ButtonAppearance::Subtle>"Group by"</Button>
///                 <Button appearance=ButtonAppearance::Subtle>"Columns"</Button>
///                 <Button appearance=ButtonAppearance::Subtle>"Share view"</Button>
///                 <Select bind=SelectBind { value: saved.into(), ..Default::default() }>
///                     <option value="all">"All (unsaved)"</option>
///                     <option value="mine">"My open tasks"</option>
///                 </Select>
///             </DataTableToolbarEnd>
///         </DataTable>
///     </div>
/// }
/// ```
///
/// ## Custom toolbar, footer, and empty state
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
#[component_doc(
    category = "Data Table",
    preview_slug = "data-table-slots",
    preview_label = "DataTable Slots",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn DataTableSlotsDoc() -> impl IntoView {
    let empty: RwSignal<Vec<DataTableRowModel>> = RwSignal::new(vec![]);
    let row = |id: &str, title: &str, status: &str| {
        DataTableRowModel::new(DataRecord::new(
            id,
            HashMap::from([
                ("title".into(), DataValue::Text(title.into())),
                ("status".into(), DataValue::Text(status.into())),
            ]),
        ))
    };
    let items = RwSignal::new(vec![
        row("1", "Migrate card vault", "Doing"),
        row("2", "Write card vault runbook", "To do"),
    ]);
    let saved = RwSignal::new("mine".to_string());

    view! {
        <div data-testid="data-table-toolbar-end-preview">
            <DataTable
                paging=PagingMode::None
                columns=vec![
                    DataTableColumnDef::new("title", "Title"),
                    DataTableColumnDef::new("status", "Status"),
                ]
                items=items
            >
                <DataTableToolbarEnd slot>
                    <Button appearance=ButtonAppearance::Subtle>"Advanced"</Button>
                    <Select bind=SelectBind { value: saved.into(), ..Default::default() }>
                        <option value="all">"All (unsaved)"</option>
                        <option value="mine">"My open tasks"</option>
                    </Select>
                </DataTableToolbarEnd>
            </DataTable>
        </div>
        <div data-testid="data-table-slots-preview">
            <DataTable
                paging=PagingMode::None
                max_height=200.0
                columns=vec![DataTableColumnDef::new("name", "Name")]
                items=empty
            >
                <DataTableToolbarSlot slot>
                    <div data-testid="custom-toolbar">
                        <Toolbar>
                            <ToolbarButton>"Custom toolbar"</ToolbarButton>
                        </Toolbar>
                    </div>
                </DataTableToolbarSlot>
                <DataTableFooterSlot slot>
                    <div data-testid="custom-footer">"Custom footer"</div>
                </DataTableFooterSlot>
                <DataTableEmptyView slot>
                    <div data-testid="custom-empty">"No data yet"</div>
                </DataTableEmptyView>
            </DataTable>
        </div>
    }
}
