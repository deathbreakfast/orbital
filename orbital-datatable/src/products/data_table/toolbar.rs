use leptos::prelude::*;
use orbital_core_components::{ButtonGroup, Toolbar};

use super::column_picker::DataTableColumnPicker;
use super::export_menu::DataTableExportMenu;
use super::filter_panel::DataTableFilterPanel;
use super::pivot_panel::DataTablePivotPanel;
use super::quick_search::DataTableQuickSearch;
use super::toolbar_overflow::DataTableToolbarOverflow;
use crate::core::use_data_table_context;
use crate::core::DataTableTableStateProvider;
use crate::types::{DataTableFeatures, DataTableTableState, DataTableToolbarSlot};

#[cfg(feature = "hydrate")]
fn click_toolbar_trigger(testid: &str) {
    use wasm_bindgen::JsCast;
    if let Some(doc) = window().document() {
        if let Ok(Some(el)) = doc.query_selector(&format!("[data-testid=\"{testid}\"]")) {
            if let Ok(btn) = el.dyn_into::<web_sys::HtmlElement>() {
                btn.click();
            }
        }
    }
}

#[cfg(not(feature = "hydrate"))]
fn click_toolbar_trigger(_testid: &str) {}

/// Toolbar shell for quick search, filter panel, column picker, and export.
#[component]
pub fn DataTableToolbar(
    state: DataTableTableState,
    #[prop(optional)] toolbar_slot: Option<DataTableToolbarSlot>,
) -> impl IntoView {
    if let Some(slot) = toolbar_slot {
        return view! {
            <DataTableTableStateProvider state=state>
                {(slot.children)()}
            </DataTableTableStateProvider>
        }
        .into_any();
    }

    let ctx = use_data_table_context();
    let config = ctx.toolbar_config.get_value();
    let pivoting = state.features.contains(DataTableFeatures::PIVOTING);

    let show_filter = config.filter_panel;
    let show_columns = config.column_picker;
    let show_pivot = config.pivot && pivoting;
    let show_export = config.export_menu;
    let show_actions = show_filter || show_columns || show_pivot || show_export;

    let on_overflow_select = Callback::new(move |action: String| match action.as_str() {
        "filter" => click_toolbar_trigger("data-table-filter-panel-trigger"),
        "columns" => click_toolbar_trigger("data-table-column-picker-trigger"),
        "pivot" => click_toolbar_trigger("data-table-pivot-trigger"),
        "export" => click_toolbar_trigger("data-table-export-menu"),
        _ => {}
    });

    view! {
        {move || {
            view! {
                <div class="orbital-data-table__toolbar" data-testid="data-table-toolbar">
                    {config.quick_search.then(|| view! {
                        <div class="orbital-data-table__toolbar-search">
                            <DataTableQuickSearch state=state />
                        </div>
                    })}
                    {show_actions.then(|| view! {
                        <div class="orbital-data-table__toolbar-actions">
                            <Toolbar attr:aria-label="Table actions">
                                <DataTableToolbarOverflow
                                    on_select=on_overflow_select
                                    show_filter=show_filter
                                    show_columns=show_columns
                                    show_pivot=show_pivot
                                    show_export=show_export
                                >
                                    <ButtonGroup>
                                        {show_filter.then(|| view! { <DataTableFilterPanel state=state /> })}
                                        {show_columns.then(|| view! { <DataTableColumnPicker state=state /> })}
                                        {show_pivot.then(|| view! { <DataTablePivotPanel state=state /> })}
                                        {show_export.then(|| view! { <DataTableExportMenu state=state /> })}
                                    </ButtonGroup>
                                </DataTableToolbarOverflow>
                            </Toolbar>
                        </div>
                    })}
                </div>
            }
            .into_any()
        }}
    }
    .into_any()
}
