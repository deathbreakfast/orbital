use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::format::{format_change, render_history_markdown};
use crate::types::{HistoryChange, HistoryFeatures};

/// Highlighted field diff line when [`HistoryFeatures::DIFF_HIGHLIGHT`] is enabled.
#[component]
fn HistoryFieldDiffHighlight(
    field: String,
    old_value: String,
    new_value: String,
) -> impl IntoView {
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
    let diff_highlight =
        Memo::new(move |_| ctx.features.contains(HistoryFeatures::DIFF_HIGHLIGHT));
    let citations_enabled =
        Memo::new(move |_| ctx.features.contains(HistoryFeatures::MARKDOWN_CITATIONS));

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
                HistoryChange::Markdown { body, citations } if markdown_enabled.get() => {
                    let cite_list = if citations_enabled.get() {
                        citations.clone()
                    } else {
                        vec![]
                    };
                    let html = render_history_markdown(body, &cite_list);
                    let citation_events = ctx.events.clone();
                    view! {
                        <div
                            class="orbital-history__change orbital-history__markdown"
                            inner_html=html
                            on:click=move |ev| {
                                #[cfg(feature = "hydrate")]
                                {
                                    let citation_cb = citation_events.on_citation_click.clone();
                                    use wasm_bindgen::JsCast;
                                    if let Some(target) = ev.target() {
                                        if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                            let anchor = if element
                                                .class_list()
                                                .contains("orbital-history__citation-ref")
                                            {
                                                Some(element)
                                            } else {
                                                element
                                                    .closest(".orbital-history__citation-ref")
                                                    .ok()
                                                    .flatten()
                                            };
                                            if let Some(anchor) = anchor {
                                                ev.prevent_default();
                                                if let Some(id) =
                                                    anchor.get_attribute("data-citation-id")
                                                {
                                                    if let Some(cb) = &citation_cb {
                                                        cb.run(id);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                #[cfg(not(feature = "hydrate"))]
                                {
                                    let _ = (&ev, &citation_events);
                                }
                            }
                        />
                    }
                    .into_any()
                }
                HistoryChange::Custom { summary } if markdown_enabled.get() => {
                    let html = render_history_markdown(summary, &[]);
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
