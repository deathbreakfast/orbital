use leptos::prelude::*;
use orbital_core_components::{Checkbox, Material, MaterialVariant};

use super::row_interaction::{click_target_is_interactive, keyboard_activates_row};
use crate::core::use_data_table_context;
use crate::engine::format_display;
use crate::types::DataTableTableState;

/// Card list body for responsive list view layout.
#[component]
pub fn DataTableListBody(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();
    let config = state.list_view.get_value().expect("list view config");
    let row_click_enabled = ctx.events.with_value(|events| {
        events
            .as_ref()
            .and_then(|e| e.on_row_click.as_ref())
            .is_some()
    });
    let list_card_class = if row_click_enabled {
        "orbital-data-table__list-card orbital-data-table__list-card--clickable"
    } else {
        "orbital-data-table__list-card"
    };

    view! {
        {move || {
            let _ = state.render_key.get();
            let rows = state.processed.get();
            let columns = state.columns.get_value();
            let primary = config.primary_field.clone();
            let secondary = config.secondary_fields.clone();

            view! {
                <div class="orbital-data-table__list-view" data-testid="data-table-list-view">
                    {rows
                        .into_iter()
                        .filter(|r| r.is_data_row())
                        .map(|row| {
                            let row_id = state.resolve_id(&row);
                            let testid = format!("data-table-list-card-{row_id}");
                            let title = row
                                .get(&primary)
                                .map(|v| v.display_string())
                                .unwrap_or_default();
                            let row_checked = RwSignal::new(false);
                            let id = row_id.clone();
                            Effect::new(move |_| {
                                row_checked.set(state.selected.with(|s| s.contains(&id)));
                            });
                            let select_id = row_id.clone();
                            let secondary_cells: Vec<(String, String)> = secondary
                                .iter()
                                .filter_map(|field| {
                                    columns.iter().find(|c| c.field == *field).map(|col| {
                                        let text = row
                                            .get(field)
                                            .map(|v| format_display(col, v))
                                            .unwrap_or_default();
                                        (col.header_name.clone(), text)
                                    })
                                })
                                .collect();
                            let has_secondary_fields = !secondary_cells.is_empty();
                            let row_click_id_click = row_id.clone();
                            let row_click_id_key = row_id.clone();
                            view! {
                                <div
                                    class=list_card_class
                                    data-testid=testid
                                    role=if row_click_enabled { "button" } else { "" }
                                    tabindex=if row_click_enabled { "0" } else { "-1" }
                                    on:click=move |ev: leptos::ev::MouseEvent| {
                                        if !row_click_enabled || click_target_is_interactive(&ev) {
                                            return;
                                        }
                                        if let Some(events) = ctx.events.get_value() {
                                            events.notify_row_click(&row_click_id_click);
                                        }
                                    }
                                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                        if !row_click_enabled || !keyboard_activates_row(&ev) {
                                            return;
                                        }
                                        ev.prevent_default();
                                        if let Some(events) = ctx.events.get_value() {
                                            events.notify_row_click(&row_click_id_key);
                                        }
                                    }
                                >
                                    <Material
                                        variant=MaterialVariant::Outlined
                                        class="orbital-data-table__list-card-surface"
                                    >
                                        <div class="orbital-data-table__list-card-inner">
                                            <Show when=move || state.selection_mode.get().is_some()>
                                                <Checkbox
                                                    checked=row_checked
                                                    attr:data-testid=format!("data-table-list-select-{row_id}")
                                                    on:change={
                                                        let select_id = select_id.clone();
                                                        move |_| {
                                                            state.toggle_row_selection(&select_id, false);
                                                        }
                                                    }
                                                />
                                            </Show>
                                            <div class="orbital-data-table__list-card-body">
                                                <div class="orbital-data-table__list-card-title">{title}</div>
                                                <Show when=move || has_secondary_fields>
                                                    <div class="orbital-data-table__list-card-fields">
                                                        {secondary_cells
                                                            .iter()
                                                            .map(|(header, text)| {
                                                                view! {
                                                                    <div class="orbital-data-table__list-card-field">
                                                                        <span class="orbital-data-table__list-card-label">
                                                                            {header.clone()}
                                                                        </span>
                                                                        <span class="orbital-data-table__list-card-value">
                                                                            {text.clone()}
                                                                        </span>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>
                                                </Show>
                                            </div>
                                        </div>
                                    </Material>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            }
        }}
    }
}
