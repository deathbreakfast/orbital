use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::DataRecord;

/// Custom row detail panel body (`ROW_DETAIL`).
pub type RowDetailView = Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>;

/// Internal slot content consumed by toolbar, footer, overlay, and row-detail subcomponents.
#[derive(Default)]
pub struct DataTableSlots {
    pub toolbar: Option<DataTableToolbarSlot>,
    pub toolbar_end: Option<DataTableToolbarEnd>,
    pub footer: Option<DataTableFooterSlot>,
    pub empty_view: Option<DataTableEmptyView>,
    pub no_results_view: Option<DataTableNoResultsView>,
    pub loading_view: Option<DataTableLoadingView>,
    pub row_detail: Option<DataTableRowDetail>,
}

impl DataTableSlots {
    #[allow(clippy::too_many_arguments)]
    pub fn from_slot_props(
        toolbar: Option<DataTableToolbarSlot>,
        toolbar_end: Option<DataTableToolbarEnd>,
        footer: Option<DataTableFooterSlot>,
        empty_view: Option<DataTableEmptyView>,
        no_results_view: Option<DataTableNoResultsView>,
        loading_view: Option<DataTableLoadingView>,
        row_detail: Option<DataTableRowDetail>,
    ) -> Self {
        Self {
            toolbar,
            toolbar_end,
            footer,
            empty_view,
            no_results_view,
            loading_view,
            row_detail,
        }
    }

    /// Resolve row detail callback from slot or deprecated prop.
    pub fn row_detail_view(&self, legacy: Option<RowDetailView>) -> Option<RowDetailView> {
        if let Some(slot) = &self.row_detail {
            return Some(slot.render.clone());
        }
        legacy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn row_detail_view_prefers_slot() {
        let legacy: RowDetailView = Arc::new(|_| ().into_any());
        let slot_view: RowDetailView = Arc::new(|_| ().into_any());
        let slots = DataTableSlots {
            row_detail: Some(DataTableRowDetail { render: slot_view }),
            ..Default::default()
        };
        let resolved = slots.row_detail_view(Some(legacy));
        assert!(resolved.is_some());
    }
}

/// Custom toolbar region. Pass `<DataTableToolbarSlot slot>` as a child of [`crate::DataTable`].
///
/// When omitted, the default toolbar renders quick search, filter panel, column picker, pivot, and export controls.
///
/// Use [`crate::use_data_table_table_state`] inside the slot to read reactive table state
/// (pagination, selection, filters, etc.).
#[slot]
pub struct DataTableToolbarSlot {
    pub(crate) children: ChildrenFn,
}

/// Product controls on the default toolbar's search row. Pass `<DataTableToolbarEnd slot>` as a
/// child of [`crate::DataTable`].
///
/// The content renders on the same row as quick search: search stays at the start of the
/// row, and this content sits at the end beside the built-in buttons (Filters, Columns,
/// Pivot, Export), ahead of them. Use it for a page's own filter button, saved searches or
/// quick toggles, so they share the search row instead of adding rows above the table. The
/// row wraps on narrow screens. Ignored when [`DataTableToolbarSlot`] replaces the toolbar.
///
/// Use [`crate::use_data_table_table_state`] inside the slot to read reactive table state.
#[slot]
pub struct DataTableToolbarEnd {
    pub(crate) children: ChildrenFn,
}

/// Custom footer region. Pass `<DataTableFooterSlot slot>` as a child of [`crate::DataTable`].
///
/// When omitted, the default footer renders row count and pagination controls.
///
/// Use [`crate::use_data_table_table_state`] inside the slot to read reactive table state.
#[slot]
pub struct DataTableFooterSlot {
    pub(crate) children: ChildrenFn,
}

/// Custom empty-state overlay. Pass `<DataTableEmptyView slot>` as a child of [`crate::DataTable`].
///
/// Shown when the data source has zero rows and no filter/search is active.
#[slot]
pub struct DataTableEmptyView {
    pub(crate) children: ChildrenFn,
}

/// Custom no-results overlay. Pass `<DataTableNoResultsView slot>` as a child of [`crate::DataTable`].
///
/// Shown when filters or quick search eliminate all rows.
#[slot]
pub struct DataTableNoResultsView {
    pub(crate) children: ChildrenFn,
}

/// Custom loading overlay. Pass `<DataTableLoadingView slot>` as a child of [`crate::DataTable`].
///
/// Shown while server data is fetching or when the client `loading` prop is true.
#[slot]
pub struct DataTableLoadingView {
    pub(crate) children: ChildrenFn,
}

/// Custom expandable row detail panel. Pass `<DataTableRowDetail slot render=... />`.
#[slot]
pub struct DataTableRowDetail {
    #[prop(into)]
    pub render: RowDetailView,
}
