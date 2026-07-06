use leptos::prelude::*;
use orbital_core_components::{Body1, Body1Strong};

use crate::context::use_history_context;
use crate::types::{HistoryFeatures, HistoryFieldDiff};

#[component]
fn HistoryFieldDiffCardRow(
    field: String,
    old_value: String,
    new_value: String,
) -> impl IntoView {
    let ctx = use_history_context();
    let highlight = Memo::new(move |_| ctx.features.contains(HistoryFeatures::DIFF_HIGHLIGHT));
    let plain = Memo::new({
        let field = field.clone();
        let old_value = old_value.clone();
        let new_value = new_value.clone();
        move |_| ctx.locale.get().format_field_diff(&field, &old_value, &new_value)
    });
    let segments = Memo::new(move |_| {
        ctx.locale
            .get()
            .field_diff_segments(&field, &old_value, &new_value)
    });

    view! {
        <Show
            when=move || highlight.get()
            fallback=move || view! {
                <Body1 class="orbital-history__change-card-row".to_string()>
                    {move || plain.get()}
                </Body1>
            }
        >
            <Body1 class="orbital-history__change-card-row".to_string()>
                {move || segments.get().0.clone()}
                <span class="orbital-history__diff-old">{move || segments.get().1.clone()}</span>
                {move || segments.get().2.clone()}
                <span class="orbital-history__diff-new">{move || segments.get().3.clone()}</span>
            </Body1>
        </Show>
    }
}

/// Multi-field change card for `HistoryChange::FieldDiffs`.
#[component]
pub fn HistoryChangeCard(fields: Vec<HistoryFieldDiff>) -> impl IntoView {
    let ctx = use_history_context();
    let n = fields.len();
    let header = Memo::new(move |_| ctx.locale.get().format_field_diffs_header(n));

    let rows: Vec<_> = fields
        .into_iter()
        .map(|field| {
            view! {
                <HistoryFieldDiffCardRow
                    field=field.field
                    old_value=field.old_value
                    new_value=field.new_value
                />
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
