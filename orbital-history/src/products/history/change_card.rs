use leptos::prelude::*;
use orbital_core_components::{Body1, Body1Strong};

use crate::context::use_history_context;
use crate::types::HistoryFieldDiff;

/// Multi-field change card for `HistoryChange::FieldDiffs`.
#[component]
pub fn HistoryChangeCard(fields: Vec<HistoryFieldDiff>) -> impl IntoView {
    let ctx = use_history_context();
    let n = fields.len();
    let header = Memo::new(move |_| ctx.locale.get().format_field_diffs_header(n));

    let rows: Vec<_> = fields
        .into_iter()
        .map(|field| {
            let locale = ctx.locale;
            let line = Memo::new(move |_| {
                locale.get().format_field_diff(
                    &field.field,
                    &field.old_value,
                    &field.new_value,
                )
            });
            view! {
                <Body1 class="orbital-history__change-card-row".to_string()>
                    {move || line.get()}
                </Body1>
            }
        })
        .collect();

    view! {
        <div class="orbital-history__change-card" data-testid="history-change-card">
            <Body1Strong class="orbital-history__change-card-header".to_string()>
                {move || header.get()}
            </Body1Strong>
            {rows}
        </div>
    }
}
