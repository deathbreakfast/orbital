use leptos::prelude::*;
use orbital_base_components::{Handler, OverlayDismiss};
use orbital_core_components::{
    Button, ButtonAppearance, Popover, PopoverLifecycle, PopoverPosition, PopoverSize,
    PopoverTrigger, PopoverTriggerType, Select, SelectSize,
};

use super::filter_rule_editor::{
    default_operator_for, draft_to_rule, rule_to_draft, DataTableFilterRuleEditor,
};
use crate::engine::operator_to_wire;
use crate::types::{
    ColumnType, DataTableColumnDef, DataTableFeatures, DataTableFilter, DataTableTableState,
    FilterLogic,
};

/// Filter panel trigger + popover UI for structured column filters.
#[component]
pub fn DataTableFilterPanel(state: DataTableTableState) -> impl IntoView {
    let multi_filter = state.features.contains(DataTableFeatures::MULTI_FILTER);
    let logic = RwSignal::new(FilterLogic::And);
    let rule_count = RwSignal::new(1usize);

    let (field0, op0, val0) = new_draft_signals(&state.columns.get_value(), 0);
    let (field1, op1, val1) = new_draft_signals(&state.columns.get_value(), 1);
    let (field2, op2, val2) = new_draft_signals(&state.columns.get_value(), 2);

    let logic_value = RwSignal::new("and".to_string());
    Effect::new(move || {
        logic.set(match logic_value.get().as_str() {
            "or" => FilterLogic::Or,
            _ => FilterLogic::And,
        });
    });

    let load_draft = move || {
        let current = state.filter.get();
        logic.set(current.logic);
        logic_value.set(match current.logic {
            FilterLogic::Or => "or".into(),
            FilterLogic::And => "and".into(),
        });
        let columns = state.columns.get_value();
        let count = if current.items.is_empty() {
            1
        } else {
            current.items.len().max(1)
        };
        rule_count.set(if multi_filter { count } else { 1 });
        let slots = [
            (&field0, &op0, &val0),
            (&field1, &op1, &val1),
            (&field2, &op2, &val2),
        ];
        for (idx, item) in current.items.iter().enumerate().take(3) {
            let (field, operator, value) = rule_to_draft(item, &columns);
            slots[idx].0.set(field);
            slots[idx].1.set(operator);
            slots[idx].2.set(value);
        }
        if current.items.is_empty() {
            let (field, operator, value) = new_draft_values(&columns, 0);
            field0.set(field);
            op0.set(operator);
            val0.set(value);
        }
    };

    let collect_rules = move || {
        let columns = state.columns.get_value();
        let count = rule_count.get().min(if multi_filter { 3 } else { 1 });
        let slots = [
            (&field0, &op0, &val0),
            (&field1, &op1, &val1),
            (&field2, &op2, &val2),
        ];
        (0..count)
            .filter_map(|idx| {
                draft_to_rule(
                    &slots[idx].0.get(),
                    &slots[idx].1.get(),
                    &slots[idx].2.get(),
                    &columns,
                )
            })
            .collect::<Vec<_>>()
    };

    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::BottomEnd
            size=PopoverSize::Large
            lifecycle=PopoverLifecycle {
                on_open: Some(Handler::new(load_draft)),
                ..Default::default()
            }
        >
            <PopoverTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    attr:data-testid="data-table-filter-panel-trigger"
                >
                    "Filters"
                </Button>
            </PopoverTrigger>
            <DataTableFilterPanelContent
                state=state
                multi_filter=multi_filter
                logic=logic
                logic_value=logic_value
                rule_count=rule_count
                field0=field0
                op0=op0
                val0=val0
                field1=field1
                op1=op1
                val1=val1
                field2=field2
                op2=op2
                val2=val2
                collect_rules=collect_rules
            />
        </Popover>
    }
}

#[component]
fn DataTableFilterPanelContent(
    state: DataTableTableState,
    multi_filter: bool,
    logic: RwSignal<FilterLogic>,
    logic_value: RwSignal<String>,
    rule_count: RwSignal<usize>,
    field0: RwSignal<String>,
    op0: RwSignal<String>,
    val0: RwSignal<String>,
    field1: RwSignal<String>,
    op1: RwSignal<String>,
    val1: RwSignal<String>,
    field2: RwSignal<String>,
    op2: RwSignal<String>,
    val2: RwSignal<String>,
    collect_rules: impl Fn() -> Vec<crate::types::FilterRule> + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let dismiss = expect_context::<OverlayDismiss>();

    let apply_filter = {
        let collect_rules = collect_rules.clone();
        Callback::new(move |_| {
            state.set_filter(DataTableFilter {
                items: collect_rules(),
                logic: logic.get(),
            });
            dismiss.close.run(());
        })
    };

    let clear_filter = Callback::new(move |_| {
        state.set_filter(DataTableFilter::default());
        dismiss.close.run(());
    });

    view! {
        <div
            class="orbital-data-table__filter-panel"
            data-testid="data-table-filter-panel"
        >
            <div class="orbital-data-table__filter-panel-title">"Filter rows"</div>
            {multi_filter.then(|| view! {
                <div class="orbital-data-table__filter-logic">
                    <span>"Match"</span>
                    <Select
                        bind=logic_value
                        attr:data-testid="data-table-filter-logic"
                        appearance=SelectSize::Small
                    >
                        <option value="and">"All (AND)"</option>
                        <option value="or">"Any (OR)"</option>
                    </Select>
                </div>
            })}
            <DataTableFilterRuleEditor
                columns=state.columns
                field=field0
                operator_wire=op0
                value_text=val0
                rule_index=0
            />
            <Show when=move || multi_filter && (rule_count.get() >= 2)>
                <DataTableFilterRuleEditor
                    columns=state.columns
                    field=field1
                    operator_wire=op1
                    value_text=val1
                    rule_index=1
                />
            </Show>
            <Show when=move || multi_filter && (rule_count.get() >= 3)>
                <DataTableFilterRuleEditor
                    columns=state.columns
                    field=field2
                    operator_wire=op2
                    value_text=val2
                    rule_index=2
                />
            </Show>
            {multi_filter.then(|| view! {
                <Button
                    appearance=ButtonAppearance::Subtle
                    disabled=Signal::derive(move || rule_count.get() >= 3)
                    on_click=Callback::new(move |_| rule_count.update(|c| *c = (*c + 1).min(3)))
                    attr:data-testid="data-table-filter-add-rule"
                >
                    "Add filter"
                </Button>
            })}
            <div class="orbital-data-table__filter-panel-actions">
                <Button appearance=ButtonAppearance::Primary on_click=apply_filter>
                    "Apply"
                </Button>
                <Button appearance=ButtonAppearance::Subtle on_click=clear_filter>
                    "Clear"
                </Button>
            </div>
        </div>
    }
}

fn new_draft_signals(
    columns: &[DataTableColumnDef],
    idx: usize,
) -> (RwSignal<String>, RwSignal<String>, RwSignal<String>) {
    let (field, operator, value) = new_draft_values(columns, idx);
    (
        RwSignal::new(field),
        RwSignal::new(operator),
        RwSignal::new(value),
    )
}

fn new_draft_values(columns: &[DataTableColumnDef], idx: usize) -> (String, String, String) {
    let filterable: Vec<_> = columns
        .iter()
        .filter(|c| c.filterable && c.col_type != ColumnType::Actions)
        .collect();
    let col = filterable.get(idx).or_else(|| filterable.first());
    let field = col
        .map(|c| c.field.clone())
        .unwrap_or_else(|| "name".into());
    let col_type = col.map(|c| c.col_type).unwrap_or(ColumnType::Text);
    let op = default_operator_for(col_type);
    (field, operator_to_wire(op).to_string(), String::new())
}
