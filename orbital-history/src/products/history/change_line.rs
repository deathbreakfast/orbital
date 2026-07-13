use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::format::format_change;
use crate::types::{HistoryChange, HistoryFeatures};

use super::HistoryMarkdownBody;

/// Highlighted field diff line when [`HistoryFeatures::DIFF_HIGHLIGHT`] is enabled.
#[component]
fn HistoryFieldDiffHighlight(field: String, old_value: String, new_value: String) -> impl IntoView {
    let ctx = use_history_context();
    let segments = Memo::new(move |_| {
        ctx.locale
            .get()
            .field_diff_segments(&field, &old_value, &new_value)
    });

    view! {
        <Body1 class="orbital-history__change".to_string()>
            {move || segments.get().0.clone()}
            <span class="orbital-history__diff-old">{move || segments.get().1.clone()}</span>
            {move || segments.get().2.clone()}
            <span class="orbital-history__diff-new">{move || segments.get().3.clone()}</span>
        </Body1>
    }
}

/// Single-line formatted change summary (or markdown body when enabled).
#[component]
pub fn HistoryChangeLine(change: HistoryChange) -> impl IntoView {
    let ctx = use_history_context();
    let markdown_enabled =
        Memo::new(move |_| ctx.features.contains(HistoryFeatures::MARKDOWN_BODIES));
    let diff_highlight = Memo::new(move |_| ctx.features.contains(HistoryFeatures::DIFF_HIGHLIGHT));

    view! {
        {move || {
            let locale = ctx.locale.get();
            match &change {
                HistoryChange::FieldDiff {
                    field,
                    old_value,
                    new_value,
                } if diff_highlight.get() => {
                    view! {
                        <HistoryFieldDiffHighlight
                            field=field.clone()
                            old_value=old_value.clone()
                            new_value=new_value.clone()
                        />
                    }
                    .into_any()
                }
                HistoryChange::Markdown {
                    body,
                    citations,
                    mentions,
                    attachments,
                } if markdown_enabled.get() => {
                    view! {
                        <HistoryMarkdownBody
                            body=body.clone()
                            citations=citations.clone()
                            mentions=mentions.clone()
                            attachments=attachments.clone()
                        />
                    }
                    .into_any()
                }
                HistoryChange::Custom { summary } if markdown_enabled.get() => {
                    view! {
                        <HistoryMarkdownBody
                            body=summary.clone()
                            citations=vec![]
                            mentions=vec![]
                            attachments=vec![]
                        />
                    }
                    .into_any()
                }
                _ => {
                    let text = format_change(&change, &locale);
                    view! {
                        <Body1 class="orbital-history__change".to_string()>
                            {text}
                        </Body1>
                    }
                    .into_any()
                }
            }
        }}
    }
}
