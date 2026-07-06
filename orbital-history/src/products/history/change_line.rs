use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::format::{format_change, render_history_markdown};
use crate::types::{HistoryChange, HistoryFeatures};

/// Single-line formatted change summary (or markdown body when enabled).
#[component]
pub fn HistoryChangeLine(change: HistoryChange) -> impl IntoView {
    let ctx = use_history_context();
    let markdown_enabled =
        Memo::new(move |_| ctx.features.contains(HistoryFeatures::MARKDOWN_BODIES));

    view! {
        {move || {
            let locale = ctx.locale.get();
            match &change {
                HistoryChange::Markdown { body } if markdown_enabled.get() => {
                    let html = render_history_markdown(body);
                    view! {
                        <div
                            class="orbital-history__change orbital-history__markdown"
                            inner_html=html
                        />
                    }
                    .into_any()
                }
                HistoryChange::Custom { summary } if markdown_enabled.get() => {
                    let html = render_history_markdown(summary);
                    view! {
                        <div
                            class="orbital-history__change orbital-history__markdown"
                            inner_html=html
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
