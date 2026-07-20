use leptos::prelude::*;
use orbital_base_components::input_event_value;
use orbital_core_components::{Checkbox, DatePicker, DatePickerAppearance, Select, SelectSize};

use crate::engine::{date_value_to_unix, unix_to_date_text};
use crate::types::{
    ColumnType, DataTableColumnDef, DataTableRowModel, DataTableTableState, EditCellProps,
};

/// Default typed editor view (inline, not a nested component — avoids SSR hydration issues).
pub fn data_table_default_editor_view(
    state: DataTableTableState,
    row_id: String,
    field: String,
    column: DataTableColumnDef,
    row: DataTableRowModel,
) -> AnyView {
    let drafts = state.edit_session.drafts.get_value();
    let Some(draft_state) = drafts.get(&field).cloned() else {
        return view! { <span></span> }.into_any();
    };
    let draft = draft_state.draft;
    let error = draft_state.error;
    let original = draft_state.original.clone();

    if let Some(edit_view) = column.edit_view.clone() {
        let row_id_commit = row_id.clone();
        let on_commit = Callback::new(move |_| state.commit_edit(&row_id_commit));
        let on_cancel = Callback::new(move |_| state.cancel_edit());
        return edit_view(EditCellProps {
            row: row.record.clone(),
            column: column.clone(),
            draft: draft_state.draft,
            error: draft_state.error,
            on_commit,
            on_cancel,
        })
        .into_any();
    }

    let input_testid = format!("data-table-cell-{row_id}-{field}-input");
    let error_testid = format!("data-table-edit-error-{row_id}-{field}");

    match column.col_type {
        ColumnType::Boolean => {
            let row_id = row_id.clone();
            let checked = RwSignal::new(matches!(
                draft.get_untracked().as_str(),
                "true" | "1" | "yes"
            ));
            view! {
                <div class="orbital-data-table__edit-cell" data-testid=input_testid.clone()>
                    <Checkbox
                        checked=checked
                        on_change=Callback::new({
                            let row_id = row_id.clone();
                            move |value| {
                                draft.set(if value { "true" } else { "false" }.to_string());
                                state.commit_edit(&row_id);
                            }
                        })
                    />
                    <Show when=move || error.get().is_some()>
                        <span class="orbital-data-table__edit-error" data-testid=error_testid.clone()>
                            {move || error.get().unwrap_or_default()}
                        </span>
                    </Show>
                </div>
            }
            .into_any()
        }
        ColumnType::SingleSelect => {
            let row_id = row_id.clone();
            let options = column
                .edit_options
                .clone()
                .unwrap_or_else(|| vec!["Admin".into(), "Member".into()]);
            view! {
                <div
                    class="orbital-data-table__edit-cell"
                    data-testid=input_testid.clone()
                    on:change=move |_| state.commit_edit(&row_id)
                >
                    <Select bind=draft appearance=SelectSize::Small>
                        {options
                            .into_iter()
                            .map(|opt| {
                                let label = opt.clone();
                                view! { <option value=opt>{label}</option> }
                            })
                            .collect_view()}
                    </Select>
                    <Show when=move || error.get().is_some()>
                        <span class="orbital-data-table__edit-error" data-testid=error_testid.clone()>
                            {move || error.get().unwrap_or_default()}
                        </span>
                    </Show>
                </div>
            }
            .into_any()
        }
        ColumnType::Date => {
            let row_id = row_id.clone();
            let unix = RwSignal::new(date_value_to_unix(&original));
            view! {
                <div
                    class="orbital-data-table__edit-cell"
                    data-testid=input_testid.clone()
                    on:change=move |_| {
                        if let Some(ts) = unix.get() {
                            draft.set(unix_to_date_text(ts));
                        }
                        state.commit_edit(&row_id);
                    }
                >
                    <DatePicker bind=unix appearance=DatePickerAppearance { ..Default::default() } />
                    <Show when=move || error.get().is_some()>
                        <span class="orbital-data-table__edit-error" data-testid=error_testid.clone()>
                            {move || error.get().unwrap_or_default()}
                        </span>
                    </Show>
                </div>
            }
            .into_any()
        }
        ColumnType::Number | ColumnType::Text | ColumnType::Actions => {
            let row_id = row_id.clone();
            let row_id_tab = row_id.clone();
            let row_id_blur = row_id.clone();
            let field_blur = field.clone();
            let input_type = if column.col_type == ColumnType::Number {
                "number"
            } else {
                "text"
            };
            view! {
                <div class="orbital-data-table__edit-cell" data-testid=input_testid.clone()>
                    <span class="orbital-input orbital-input--medium">
                        <input
                            class="orbital-input__input"
                            type=input_type
                            prop:value=move || draft.get()
                            on:input=move |ev| {
                                if let Some(value) = input_event_value(&ev) {
                                    draft.set(value);
                                }
                            }
                            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    state.commit_edit(&row_id);
                                } else if ev.key() == "Escape" {
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    state.cancel_edit();
                                } else if ev.key() == "Tab" && !ev.shift_key() {
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    state.commit_and_advance(&row_id_tab, &field);
                                }
                            }
                            on:blur=move |_| {
                                if state.edit_session.is_editing_field(&row_id_blur, &field_blur) {
                                    state.commit_edit(&row_id_blur);
                                }
                            }
                            autofocus
                        />
                    </span>
                    <Show when=move || error.get().is_some()>
                        <span class="orbital-data-table__edit-error" data-testid=error_testid.clone()>
                            {move || error.get().unwrap_or_default()}
                        </span>
                    </Show>
                </div>
            }
            .into_any()
        }
    }
}
