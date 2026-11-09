use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, TableCell, TableCellLayout};

use crate::engine::format_display;
use crate::types::{DataTableRowKind, DataTableRowModel, DataTableTableState};

const GROUP_INDENT_PX: f64 = 16.0;

/// Render cells for a group header row.
pub fn group_header_cells(
    row: &DataTableRowModel,
    state: DataTableTableState,
    _col_count: usize,
) -> impl IntoView {
    let DataTableRowKind::GroupHeader {
        group_key,
        field,
        depth,
        child_count: _,
        aggregates,
    } = &row.kind
    else {
        return ().into_any();
    };

    let group_key = group_key.clone();
    let field = field.clone();
    let depth = *depth;
    let label = row
        .get(field.as_str())
        .map(|v| v.display_string())
        .unwrap_or_default();
    let expanded = RwSignal::new(false);
    let gk = group_key.clone();
    Effect::new(move |_| {
        expanded.set(state.expanded_groups.with(|s| s.contains(&gk)));
    });

    let toggle_key = group_key.clone();
    let testid = format!("data-table-group-toggle-{group_key}");
    let indent = depth as f64 * GROUP_INDENT_PX;

    let resolved_cols = state.column_layout.get_untracked().columns;
    let mut cells = Vec::new();

    for (i, col) in resolved_cols.iter().enumerate() {
        let is_group_col = col.def.field == field
            || state
                .row_grouping
                .get()
                .grouping_field()
                .is_some_and(|f| f == col.def.field);

        if i == 0 || is_group_col {
            let testid_btn = testid.clone();
            let toggle = toggle_key.clone();
            let label_text = label.clone();
            cells.push(
                view! {
                    <TableCell
                        class="orbital-data-table__group-cell"
                        style=format!("padding-left: {indent}px; font-weight: 600;")
                    >
                        <Button
                            appearance=ButtonAppearance::Subtle
                            attr:data-testid=testid_btn
                            attr:aria-expanded=move || if expanded.get() { "true" } else { "false" }
                            on:click=move |_| state.toggle_group(&toggle)
                        >
                            {move || if expanded.get() { "▼" } else { "▶" }}
                        </Button>
                        " "
                        {label_text.clone()}
                    </TableCell>
                }
                .into_any(),
            );
        } else if let Some(agg) = aggregates.get(&col.def.field) {
            let text = format_display(&col.def, agg);
            cells.push(
                view! {
                    <TableCell class="orbital-data-table__group-aggregate">
                        <TableCellLayout>{text}</TableCellLayout>
                    </TableCell>
                }
                .into_any(),
            );
        } else {
            cells.push(
                view! {
                    <TableCell></TableCell>
                }
                .into_any(),
            );
        }
    }

    view! { {cells} }.into_any()
}
