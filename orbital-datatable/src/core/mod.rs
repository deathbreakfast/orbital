//! Shared DataTable context and helpers for product + subcomponents.

mod chart_binding;
mod column_drag;
mod row_drag;

use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::DataRecord;
use orbital_theme::Direction;

use crate::types::{
    DataTableEvents, DataTableFeatures, DataTableHeaderChromeConfig, DataTableLocale,
    DataTableRowModel, DataTableSelectionMode, DataTableTableState, DataTableToolbarConfig,
    GetRowId, PaginationDisplayFormat,
};
pub use chart_binding::{derive_schema_hints, provide_chart_binding, use_chart_binding};
pub use column_drag::{column_drag_ghost_at_pointer, move_column_drag_ghost, ColumnDragGhost};
pub use row_drag::{move_row_drag_ghost, row_drag_ghost_at_pointer, RowDragGhost};

/// Reactive table state for custom toolbar/footer slot content.
pub fn use_data_table_table_state() -> DataTableTableState {
    expect_context::<DataTableTableState>()
}

/// Read shared context from the nearest [`crate::DataTable`].
pub fn use_data_table_context() -> DataTableContext {
    expect_context::<DataTableContext>()
}

/// Provides [`DataTableTableState`] to custom toolbar/footer slot children.
#[component]
pub fn DataTableTableStateProvider(
    state: DataTableTableState,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(state);
    children()
}

/// Context shared by [`crate::DataTable`] and composable subcomponents.
#[derive(Clone, Copy)]
pub struct DataTableContext {
    pub features: DataTableFeatures,
    pub selection_mode: Signal<Option<DataTableSelectionMode>>,
    pub events: StoredValue<Option<DataTableEvents>>,
    pub get_row_id: StoredValue<Option<GetRowId>>,
    pub row_detail: StoredValue<Option<Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>>>,
    pub auto_row_height: bool,
    pub locale: StoredValue<DataTableLocale>,
    pub pagination_display: StoredValue<PaginationDisplayFormat>,
    pub page_size_options: StoredValue<Option<Vec<u32>>>,
    pub get_row_class: StoredValue<Option<Callback<(DataTableRowModel, usize), String>>>,
    pub direction: Signal<Direction>,
    /// Id of the row to highlight as active (independent of selection).
    pub active_row_id: Signal<Option<String>>,
    pub toolbar_config: StoredValue<DataTableToolbarConfig>,
    pub header_chrome: StoredValue<DataTableHeaderChromeConfig>,
    /// Fallback drag source when `dataTransfer` is empty (e.g. synthetic DnD in tests).
    pub drag_column_field: RwSignal<Option<String>>,
    /// Floating label shown while a column is being drag-reordered.
    pub column_drag_ghost: RwSignal<Option<ColumnDragGhost>>,
    /// Full-row preview shown while a row is being drag-reordered.
    pub row_drag_ghost: RwSignal<Option<RowDragGhost>>,
    /// Fallback drag source for row reorder when `dataTransfer` is empty.
    pub drag_row_id: RwSignal<Option<String>>,
}

#[component]
pub fn DataTableProvider(
    features: DataTableFeatures,
    selection_mode: Signal<Option<DataTableSelectionMode>>,
    events: StoredValue<Option<DataTableEvents>>,
    get_row_id: StoredValue<Option<GetRowId>>,
    row_detail: StoredValue<Option<Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>>>,
    auto_row_height: bool,
    locale: StoredValue<DataTableLocale>,
    pagination_display: StoredValue<PaginationDisplayFormat>,
    page_size_options: StoredValue<Option<Vec<u32>>>,
    get_row_class: StoredValue<Option<Callback<(DataTableRowModel, usize), String>>>,
    direction: Signal<Direction>,
    active_row_id: Signal<Option<String>>,
    toolbar_config: StoredValue<DataTableToolbarConfig>,
    header_chrome: StoredValue<DataTableHeaderChromeConfig>,
    children: Children,
) -> impl IntoView {
    provide_context(DataTableContext {
        features,
        selection_mode,
        events,
        get_row_id,
        row_detail,
        auto_row_height,
        locale,
        pagination_display,
        page_size_options,
        get_row_class,
        direction,
        active_row_id,
        toolbar_config,
        header_chrome,
        drag_column_field: RwSignal::new(None),
        column_drag_ghost: RwSignal::new(None),
        row_drag_ghost: RwSignal::new(None),
        drag_row_id: RwSignal::new(None),
    });
    children()
}
