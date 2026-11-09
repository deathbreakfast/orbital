use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, TableCell};

use crate::core::DataTableContext;
use crate::engine::branch_key_for_path;
use crate::engine::build_tree_index;
use crate::types::{DataTableRowModel, DataTableTableState};

const TREE_INDENT_PX: f64 = 20.0;

/// Tree grouping column cell with depth indent and expand/collapse chevron.
pub fn tree_grouping_cell(
    row: DataTableRowModel,
    _layout: crate::engine::LeadingColumnLayout,
    state: DataTableTableState,
    _ctx: DataTableContext,
    extra_style: Option<String>,
) -> impl IntoView {
    let get_tree_path = state.get_tree_path.get_value().expect("tree path required");
    let all_rows = state.client_items.get_untracked();
    let row_id = state.resolve_id(&row);
    let index = build_tree_index(
        &all_rows,
        &get_tree_path,
        state.get_row_id.get_value().as_ref(),
    );
    let meta = index.row_meta.get(&row_id).cloned();
    let depth = meta.as_ref().map(|m| m.depth).unwrap_or(0);
    let is_branch = meta.as_ref().is_some_and(|m| m.is_branch);
    let path_key = meta
        .as_ref()
        .map(|m| m.path_key.clone())
        .unwrap_or_default();

    let mut style = format!("padding-left: {}px;", depth as f64 * TREE_INDENT_PX);
    if let Some(extra) = extra_style {
        style.push(' ');
        style.push_str(&extra);
    }

    let path = meta.as_ref().map(|m| m.path.clone()).unwrap_or_default();
    let toggle_key =
        branch_key_for_path(&path, &index.branch_keys).unwrap_or_else(|| path_key.clone());

    let expanded = RwSignal::new(false);
    let pk = toggle_key.clone();
    Effect::new(move |_| {
        expanded.set(state.expanded_tree_nodes.with(|s| s.contains(&pk)));
    });

    let testid = format!("data-table-tree-toggle-{toggle_key}");

    let row_for_label = row.clone();
    view! {
        <TableCell style=style>
            {is_branch.then(|| {
                let key = toggle_key.clone();
                let testid = testid.clone();
                view! {
                    <span data-testid=testid>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            attr:aria-label=format!("Toggle {toggle_key}")
                            attr:aria-expanded=move || if expanded.get() { "true" } else { "false" }
                            on:click=move |_| state.toggle_tree_node(&key)
                        >
                            {move || if expanded.get() { "▼" } else { "▶" }}
                        </Button>
                    </span>
                }
            })}
            {move || {
                let field = state
                    .column_layout
                    .get()
                    .columns
                    .first()
                    .map(|c| c.def.field.clone())
                    .unwrap_or_else(|| "name".into());
                row_for_label
                    .get(&field)
                    .map(|v| v.display_string())
                    .unwrap_or_default()
            }}
        </TableCell>
    }
}
