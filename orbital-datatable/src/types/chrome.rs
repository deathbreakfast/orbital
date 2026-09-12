/// Built-in toolbar control visibility when no custom [`crate::DataTableToolbarSlot`] is provided.
///
/// Set individual flags to `false` to hide Filters, Columns, Pivot, or Export while keeping
/// quick search. Ignored when a custom toolbar slot replaces the default chrome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTableToolbarConfig {
    /// Show the quick-search field in the toolbar. Default `true`.
    pub quick_search: bool,
    /// Show the structured filter panel trigger. Default `true`.
    pub filter_panel: bool,
    /// Show the column visibility picker trigger. Default `true`.
    pub column_picker: bool,
    /// Show the pivot configuration trigger (requires [`crate::DataTableFeatures::PIVOTING`]). Default `true`.
    pub pivot: bool,
    /// Show the export/print menu trigger. Default `true`.
    pub export_menu: bool,
}

impl Default for DataTableToolbarConfig {
    fn default() -> Self {
        Self {
            quick_search: true,
            filter_panel: true,
            column_picker: true,
            pivot: true,
            export_menu: true,
        }
    }
}

/// Per-column-header chrome toggles (menu, inline filter button, hide-column UX).
///
/// Gate header actions without replacing [`crate::DataTableHeaderCell`] rendering.
/// Feature flags still control underlying capability; these flags control visible chrome only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTableHeaderChromeConfig {
    /// Show the column header actions menu (sort, pin, hide). Default `true`.
    pub column_menu: bool,
    /// Show the per-header filter popover button. Default `true`.
    pub column_filter_button: bool,
    /// Allow hiding columns via menu and picker. Default `true`.
    pub column_hide: bool,
    /// When `false`, omit thead/tbody/scroll host (toolbar-only chrome for chart panels).
    /// Default `true`.
    pub show_table_grid: bool,
}

impl Default for DataTableHeaderChromeConfig {
    fn default() -> Self {
        Self {
            column_menu: true,
            column_filter_button: true,
            column_hide: true,
            show_table_grid: true,
        }
    }
}
