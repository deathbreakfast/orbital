use leptos::prelude::*;
use orbital_core_components::{TableHeader, TableRow};

use super::header_cell::DataTableHeaderCell;
use super::header_filters::DataTableHeaderFilterRow;
use super::header_groups::{build_group_header_row, DataTableGroupHeaderRow};
use super::leading_columns::{leading_column_layout, leading_header_cells};
use crate::core::use_data_table_context;
use crate::engine::{DETAIL_COLUMN_WIDTH_PX, REORDER_COLUMN_WIDTH_PX, SELECTION_COLUMN_WIDTH_PX};
use crate::types::{DataTableFeatures, DataTableTableState};

/// `<col>` widths for the fixed table layout, one per header column.
///
/// The browser sizes fixed-layout columns from these instead of from header content or a
/// column-group row, so a refilled column can't push the table past its container.
/// Plain function (not a component) so the `<colgroup>` sits directly under `<table>` on SSR.
pub fn data_table_colgroup(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    move || {
        let _ = state.render_key.get();
        let leading = leading_column_layout(state, ctx);
        let leading_widths = [
            (leading.reorder, REORDER_COLUMN_WIDTH_PX),
            (leading.detail, DETAIL_COLUMN_WIDTH_PX),
            (leading.selection, SELECTION_COLUMN_WIDTH_PX),
        ];
        let widths: Vec<f64> = leading_widths
            .into_iter()
            .filter_map(|(shown, width)| shown.then_some(width))
            .chain(
                state
                    .column_layout
                    .get()
                    .columns
                    .iter()
                    .map(|col| col.width_px),
            )
            .collect();
        view! {
            <colgroup>
                {widths
                    .into_iter()
                    .map(|width| view! { <col style=format!("width: {width:.2}px") /> })
                    .collect_view()}
            </colgroup>
        }
    }
}

/// Header row(s) with optional selection column, groups, and sortable headers.
#[component]
pub fn DataTableHeader(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let layout = Memo::new(move |_| leading_column_layout(state, ctx));

    view! {
        <TableHeader>
            {move || {
                build_group_header_row(state).map(|cells| {
                    view! {
                        <DataTableGroupHeaderRow
                            cells=cells
                            leading_count=layout.get().count()
                        />
                    }
                })
            }}
            <TableRow>
                {move || {
                    let layout = layout.get();
                    let rowspan = build_group_header_row(state).is_some().then_some(2u32);
                    leading_header_cells(layout, rowspan)
                }}
                <For
                    each=move || {
                        let _ = state.render_key.get();
                        state.column_layout.get().columns
                    }
                    key=|col| format!("{}:{:.0}", col.def.field, col.width_px)
                    children=move |col| {
                        view! { <DataTableHeaderCell state=state resolved=col /> }
                    }
                />
            </TableRow>
            {move || {
                state
                    .features
                    .contains(DataTableFeatures::HEADER_FILTERS)
                    .then(|| view! { <DataTableHeaderFilterRow state=state /> })
            }}
        </TableHeader>
    }
}
