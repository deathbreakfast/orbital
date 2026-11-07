use leptos::prelude::*;
use orbital_base_components::OverlayDismiss;
use orbital_core_components::{
    Button, ButtonAppearance, Input, Menu, MenuItem, MenuTrigger, Popover, PopoverTrigger,
    PopoverTriggerType, Select, SelectSize,
};

use super::filter_rule_editor::{operator_label, parse_filter_value};
use crate::core::use_data_table_context;
use crate::engine::{operator_to_wire, SortDirection};
use crate::types::{DataTableFeatures, DataTableTableState, FilterOperator, FilterRule, PinSide};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ColumnMenuAction {
    SortAsc,
    SortDesc,
    PinLeft,
    PinRight,
    Unpin,
    Hide,
}

/// Per-header column menu (sort, filter, pin, hide).
#[component]
pub fn DataTableColumnMenu(
    state: DataTableTableState,
    field: String,
    sortable: bool,
    filterable: bool,
    hideable: bool,
) -> impl IntoView {
    let ctx = use_data_table_context();
    let chrome = ctx.header_chrome.get_value();
    let show_filter = chrome.column_filter_button && filterable;
    let show_menu = chrome.column_menu;
    let allow_hide = chrome.column_hide && hideable;

    if !show_filter && !show_menu {
        return view! { <span></span> }.into_any();
    }

    let field_stored = StoredValue::new(field.clone());
    let filter_value = RwSignal::new(String::new());
    let has_choices = state
        .columns
        .get_value()
        .iter()
        .any(|c| c.field == field && c.filter_choices().is_some());
    let operator_wire = RwSignal::new(
        operator_to_wire(if has_choices {
            FilterOperator::Equals
        } else {
            FilterOperator::Contains
        })
        .to_string(),
    );

    let on_menu_select = Callback::new(move |action: ColumnMenuAction| {
        let field = field_stored.get_value();
        match action {
            ColumnMenuAction::SortAsc => {
                state.sort_column(&field, SortDirection::Asc);
            }
            ColumnMenuAction::SortDesc => {
                state.sort_column(&field, SortDirection::Desc);
            }
            ColumnMenuAction::PinLeft => state.pin_column(&field, PinSide::Left),
            ColumnMenuAction::PinRight => state.pin_column(&field, PinSide::Right),
            ColumnMenuAction::Unpin => state.unpin_column(&field),
            ColumnMenuAction::Hide => state.set_column_visible(&field, false),
        }
    });

    let menu_testid = format!("data-table-column-menu-{field}");
    let filter_testid = format!("data-table-column-filter-{field}");
    let pinning_enabled = ctx.features.contains(DataTableFeatures::COLUMN_PINNING);

    view! {
        <div class="orbital-data-table__column-menu">
            {show_filter.then(|| view! {
                <Popover trigger_type=PopoverTriggerType::Click>
                    <PopoverTrigger slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiFilterOutlined
                            attr:data-testid=filter_testid
                            attr:aria-label="Filter column"
                        />
                    </PopoverTrigger>
                    <DataTableColumnFilterPanel
                        state=state
                        field=field.clone()
                        filter_value=filter_value
                        operator_wire=operator_wire
                    />
                </Popover>
            })}
            {show_menu.then(|| view! {
                <Menu on_select=move |action: ColumnMenuAction| on_menu_select.run(action)>
                    <MenuTrigger slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiMoreOutlined
                            attr:data-testid=menu_testid
                            attr:aria-label="Column menu"
                        />
                    </MenuTrigger>
                    {sortable.then(|| view! {
                        <>
                            <MenuItem value=ColumnMenuAction::SortAsc>"Sort ascending"</MenuItem>
                            <MenuItem value=ColumnMenuAction::SortDesc>"Sort descending"</MenuItem>
                        </>
                    })}
                    {pinning_enabled.then(|| view! {
                        <>
                            <MenuItem value=ColumnMenuAction::PinLeft>"Pin left"</MenuItem>
                            <MenuItem value=ColumnMenuAction::PinRight>"Pin right"</MenuItem>
                            <MenuItem value=ColumnMenuAction::Unpin>"Unpin"</MenuItem>
                        </>
                    })}
                    {allow_hide.then(|| view! {
                        <MenuItem value=ColumnMenuAction::Hide>"Hide column"</MenuItem>
                    })}
                </Menu>
            })}
        </div>
    }
    .into_any()
}

#[component]
fn DataTableColumnFilterPanel(
    state: DataTableTableState,
    field: String,
    filter_value: RwSignal<String>,
    operator_wire: RwSignal<String>,
) -> impl IntoView {
    let dismiss = expect_context::<OverlayDismiss>();
    let column = state
        .columns
        .get_value()
        .iter()
        .find(|c| c.field == field)
        .cloned();
    let col_type = column
        .as_ref()
        .map(|c| c.col_type)
        .unwrap_or(crate::types::ColumnType::Text);
    let choices = column.and_then(|c| c.filter_choices());
    let field_id = field.clone();

    let apply_filter = {
        let field = field.clone();
        Callback::new(move |_| {
            let value = filter_value.get().trim().to_string();
            let operator = crate::engine::operator_from_wire(&operator_wire.get())
                .unwrap_or(FilterOperator::Contains);
            let mut filter = state.filter.get();
            if value.is_empty()
                && !matches!(
                    operator,
                    FilterOperator::IsEmpty | FilterOperator::IsNotEmpty
                )
            {
                filter.items.retain(|r| r.field != field);
            } else {
                filter.items.retain(|r| r.field != field);
                filter.items.push(FilterRule {
                    field: field.clone(),
                    operator,
                    value: parse_filter_value(&value, col_type, operator),
                });
            }
            state.set_filter(filter);
            dismiss.close.run(());
        })
    };

    view! {
        <div
            class="orbital-data-table__column-menu-filter-panel"
            data-testid=format!("data-table-column-filter-panel-{field_id}")
        >
            <Select bind=operator_wire appearance=SelectSize::Small>
                {FilterOperator::allowed_for(col_type)
                    .iter()
                    .map(|op| {
                        let wire = operator_to_wire(*op).to_string();
                        let label = operator_label(*op);
                        view! { <option value=wire>{label}</option> }
                    })
                    .collect_view()}
            </Select>
            {match choices {
                Some(choices) => view! {
                    <Select
                        bind=filter_value
                        appearance=SelectSize::Small
                        attr:data-testid=format!("data-table-column-filter-value-{field}")
                    >
                        <option value="">"Any value"</option>
                        {choices
                            .into_iter()
                            .map(|c| view! { <option value=c.value>{c.label}</option> })
                            .collect_view()}
                    </Select>
                }.into_any(),
                None => view! { <Input bind=filter_value /> }.into_any(),
            }}
            <Button appearance=ButtonAppearance::Primary on_click=apply_filter>
                "Apply"
            </Button>
        </div>
    }
}
