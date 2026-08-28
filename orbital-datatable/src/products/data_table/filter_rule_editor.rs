use leptos::prelude::*;
use orbital_base_components::InputType;
use orbital_core_components::{Input, InputAppearance, Select, SelectSize};
use orbital_data::DataValue;

use crate::engine::operator_to_wire;
use crate::types::{ColumnType, DataTableColumnDef, FilterOperator, FilterRule};

/// Human-readable label for a filter operator.
pub fn operator_label(operator: FilterOperator) -> &'static str {
    match operator {
        FilterOperator::IsEmpty => "Is empty",
        FilterOperator::IsNotEmpty => "Is not empty",
        FilterOperator::Contains => "Contains",
        FilterOperator::NotContains => "Does not contain",
        FilterOperator::Equals => "Equals",
        FilterOperator::NotEquals => "Does not equal",
        FilterOperator::StartsWith => "Starts with",
        FilterOperator::EndsWith => "Ends with",
        FilterOperator::GreaterThan => "Greater than",
        FilterOperator::GreaterThanOrEqual => "Greater than or equal",
        FilterOperator::LessThan => "Less than",
        FilterOperator::LessThanOrEqual => "Less than or equal",
        FilterOperator::Is => "Is",
        FilterOperator::IsNot => "Is not",
    }
}

pub fn default_operator_for(col_type: ColumnType) -> FilterOperator {
    match col_type {
        ColumnType::Number | ColumnType::Boolean | ColumnType::Date => FilterOperator::Equals,
        _ => FilterOperator::Contains,
    }
}

pub fn operator_requires_value(operator: FilterOperator) -> bool {
    !matches!(
        operator,
        FilterOperator::IsEmpty | FilterOperator::IsNotEmpty
    )
}

pub fn format_filter_value(value: &DataValue) -> String {
    match value {
        DataValue::Text(s) | DataValue::Category(s) => s.clone(),
        DataValue::Number(n) => n.to_string(),
        DataValue::Bool(b) => b.to_string(),
        DataValue::Date(d) => d.to_string(),
        DataValue::Null => String::new(),
    }
}

pub fn parse_filter_value(text: &str, col_type: ColumnType, operator: FilterOperator) -> DataValue {
    if !operator_requires_value(operator) {
        return DataValue::Null;
    }
    let trimmed = text.trim();
    match col_type {
        ColumnType::Number => trimmed
            .parse::<f64>()
            .map(DataValue::Number)
            .unwrap_or(DataValue::Null),
        ColumnType::Boolean => match trimmed.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => DataValue::Bool(true),
            "false" | "0" | "no" => DataValue::Bool(false),
            _ => DataValue::Null,
        },
        ColumnType::Date => DataValue::Text(trimmed.to_string()),
        _ => DataValue::Text(trimmed.to_string()),
    }
}

pub fn rule_to_draft(
    rule: &FilterRule,
    columns: &[DataTableColumnDef],
) -> (String, String, String) {
    let _col_type = columns
        .iter()
        .find(|c| c.field == rule.field)
        .map(|c| c.col_type)
        .unwrap_or(ColumnType::Text);
    (
        rule.field.clone(),
        operator_to_wire(rule.operator).to_string(),
        format_filter_value(&rule.value),
    )
}

pub fn draft_to_rule(
    field: &str,
    operator_wire: &str,
    value_text: &str,
    columns: &[DataTableColumnDef],
) -> Option<FilterRule> {
    let col = columns.iter().find(|c| c.field == field)?;
    let operator = crate::engine::operator_from_wire(operator_wire)?;
    let value = parse_filter_value(value_text, col.col_type, operator);
    if operator_requires_value(operator) && value.is_empty_value() && !value_text.trim().is_empty()
    {
        return None;
    }
    Some(FilterRule {
        field: field.to_string(),
        operator,
        value,
    })
}

/// Shared filter rule editor row (column, operator, value).
#[component]
pub fn DataTableFilterRuleEditor(
    columns: StoredValue<Vec<DataTableColumnDef>>,
    field: RwSignal<String>,
    operator_wire: RwSignal<String>,
    value_text: RwSignal<String>,
    #[prop(optional, into)] rule_index: MaybeProp<usize>,
) -> impl IntoView {
    let col_type = Memo::new(move |_| {
        let field = field.get();
        columns
            .get_value()
            .iter()
            .find(|c| c.field == field)
            .map(|c| c.col_type)
            .unwrap_or(ColumnType::Text)
    });

    let operators = Memo::new(move |_| FilterOperator::allowed_for(col_type.get()).to_vec());

    Effect::new(move || {
        let ops = operators.get();
        let current = operator_wire.get();
        if !ops
            .iter()
            .any(|op| operator_to_wire(*op) == current.as_str())
        {
            if let Some(first) = ops.first() {
                operator_wire.set(operator_to_wire(*first).to_string());
            }
        }
    });

    let idx = rule_index.get().unwrap_or(0);
    let field_testid = format!("data-table-filter-rule-{idx}-field");
    let operator_testid = format!("data-table-filter-rule-{idx}-operator");

    view! {
        <div class="orbital-data-table__filter-rule-editor">
            <Select bind=field attr:data-testid=field_testid appearance=SelectSize::Small>
                {move || {
                    columns
                        .get_value()
                        .iter()
                        .filter(|c| c.filterable && c.col_type != ColumnType::Actions)
                        .map(|col| {
                            let value = col.field.clone();
                            let label = col.header_name.clone();
                            view! { <option value=value>{label}</option> }
                        })
                        .collect_view()
                }}
            </Select>
            <Select bind=operator_wire attr:data-testid=operator_testid appearance=SelectSize::Small>
                {move || {
                    operators
                        .get()
                        .iter()
                        .map(|op| {
                            let wire = operator_to_wire(*op).to_string();
                            let label = operator_label(*op);
                            view! { <option value=wire.clone()>{label}</option> }
                        })
                        .collect_view()
                }}
            </Select>
            <Show
                when=move || {
                    operator_from_wire_local(operator_wire.get()).is_some_and(operator_requires_value)
                }
                fallback=|| view! { <span class="orbital-data-table__filter-rule-spacer" /> }
            >
                {move || {
                    let col = col_type.get();
                    let input_type = match col {
                        ColumnType::Number => InputType::Number,
                        ColumnType::Date => InputType::Date,
                        _ => InputType::Text,
                    };
                    let value_testid = format!("data-table-filter-rule-{idx}-value");
                    if col == ColumnType::Boolean {
                        view! {
                            <Select bind=value_text attr:data-testid=value_testid appearance=SelectSize::Small>
                                <option value="true">"True"</option>
                                <option value="false">"False"</option>
                            </Select>
                        }.into_any()
                    } else {
                        view! {
                            <Input
                                bind=value_text
                                appearance=InputAppearance {
                                    input_type: Signal::from(input_type),
                                    ..Default::default()
                                }
                                attr:data-testid=value_testid
                            />
                        }.into_any()
                    }
                }}
            </Show>
        </div>
    }
}

fn operator_from_wire_local(value: String) -> Option<FilterOperator> {
    crate::engine::operator_from_wire(&value)
}
