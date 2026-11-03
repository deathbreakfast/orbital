use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_core_components::{
    Input, InputAppearance, InputEvents, Select, SelectSize, TableCell, TableRow,
};

use super::filter_rule_editor::{default_operator_for, format_filter_value, parse_filter_value};
use super::leading_columns::leading_header_cells;
use crate::core::use_data_table_context;
use crate::types::{ColumnType, DataTableTableState, FilterOperator, FilterRule};

/// Inline filter row rendered below column headers when [`DataTableFeatures::HEADER_FILTERS`] is set.
#[component]
pub fn DataTableHeaderFilterRow(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let layout = Memo::new(move |_| super::leading_columns::leading_column_layout(state, ctx));

    view! {
        <TableRow attr:data-testid="data-table-header-filters">
            {move || {
                let layout = layout.get();
                leading_header_cells(layout, None)
            }}
            <For
                each=move || {
                    let _ = state.render_key.get();
                    state.column_layout.get().columns
                }
                key=|col| col.def.field.clone()
                children=move |col| {
                    view! {
                        <DataTableHeaderFilterCell state=state field=col.def.field.clone() column=col.def.clone() />
                    }
                }
            />
        </TableRow>
    }
}

#[component]
fn DataTableHeaderFilterCell(
    state: DataTableTableState,
    field: String,
    column: crate::types::DataTableColumnDef,
) -> impl IntoView {
    let filterable = column.filterable && column.col_type != ColumnType::Actions;
    let value_text = RwSignal::new(String::new());

    Effect::new({
        let field = field.clone();
        move || {
            let current = state.filter.get();
            if let Some(rule) = current.items.iter().find(|r| r.field == field) {
                value_text.set(format_filter_value(&rule.value));
            } else {
                value_text.set(String::new());
            }
        }
    });

    let apply = Callback::new({
        let field = field.clone();
        let col_type = column.col_type;
        let has_choices = column.filter_choices().is_some();
        move |_| {
            let text = value_text.get().trim().to_string();
            let mut filter = state.filter.get();
            filter.items.retain(|r| r.field != field);
            if !text.is_empty() {
                let operator = if has_choices {
                    FilterOperator::Equals
                } else {
                    default_operator_for(col_type)
                };
                filter.items.push(FilterRule {
                    field: field.clone(),
                    operator,
                    value: parse_filter_value(&text, col_type, operator),
                });
            }
            state.set_filter(filter);
        }
    });

    let testid = format!("data-table-header-filter-{field}");

    view! {
        <TableCell class="orbital-data-table__header-filter-cell">
            {filterable.then(|| {
                if let Some(choices) = column.filter_choices() {
                    view! {
                        <Select
                            bind=value_text
                            appearance=SelectSize::Small
                            attr:data-testid=testid.clone()
                            on:change=move |_| apply.run(())
                        >
                            <option value="">"All"</option>
                            {choices
                                .into_iter()
                                .map(|c| view! { <option value=c.value>{c.label}</option> })
                                .collect_view()}
                        </Select>
                    }.into_any()
                } else if column.col_type == ColumnType::Boolean {
                    view! {
                        <Select
                            bind=value_text
                            appearance=SelectSize::Small
                            attr:data-testid=testid.clone()
                            on:change=move |_| apply.run(())
                        >
                            <option value="">"All"</option>
                            <option value="true">"True"</option>
                            <option value="false">"False"</option>
                        </Select>
                    }.into_any()
                } else {
                    view! {
                        <Input
                            bind=value_text
                            appearance=InputAppearance::with_placeholder("Filter")
                            events=InputEvents {
                                on_blur: Some(Handler::on({
                                    let apply = apply;
                                    move |_ev: leptos::ev::FocusEvent| apply.run(())
                                })),
                                ..Default::default()
                            }
                            attr:data-testid=testid
                        />
                    }.into_any()
                }
            })}
        </TableCell>
    }
}
