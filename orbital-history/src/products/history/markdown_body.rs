use leptos::prelude::*;
use orbital_core_components::{Persona, PersonaConfig, PersonaSecondaryText, PersonaSize};

use crate::context::use_history_context;
use crate::format::{render_history_markdown, HistoryMarkdownRenderOptions};
use crate::types::{HistoryAttachment, HistoryCitation, HistoryFeatures, HistoryMention};

/// Read-only markdown body with citation click and mention hover card.
#[component]
pub fn HistoryMarkdownBody(
    body: String,
    citations: Vec<HistoryCitation>,
    mentions: Vec<HistoryMention>,
    attachments: Vec<HistoryAttachment>,
) -> impl IntoView {
    let ctx = use_history_context();
    let features = ctx.features;
    let events = ctx.events.clone();

    let html = Memo::new({
        let body = body.clone();
        let citations = citations.clone();
        let mentions = mentions.clone();
        let attachments = attachments.clone();
        move |_| {
            render_history_markdown(
                &body,
                if features.contains(HistoryFeatures::MARKDOWN_CITATIONS) {
                    &citations
                } else {
                    &[]
                },
                if features.contains(HistoryFeatures::MARKDOWN_MENTIONS) {
                    &mentions
                } else {
                    &[]
                },
                if features.contains(HistoryFeatures::MARKDOWN_IMAGES) {
                    &attachments
                } else {
                    &[]
                },
                HistoryMarkdownRenderOptions {
                    enable_citations: features.contains(HistoryFeatures::MARKDOWN_CITATIONS),
                    enable_mentions: features.contains(HistoryFeatures::MARKDOWN_MENTIONS),
                    enable_images: features.contains(HistoryFeatures::MARKDOWN_IMAGES),
                },
            )
        }
    });

    let active_mention = RwSignal::new(None::<(HistoryMention, f64, f64)>);

    view! {
        <div class="orbital-history__markdown-surface">
            <div
                class="orbital-history__change orbital-history__markdown"
                inner_html=move || html.get()
                on:click=move |ev| {
                    #[cfg(feature = "hydrate")]
                    {
                        let citation_cb = events.on_citation_click.clone();
                        let mention_cb = events.on_mention_click.clone();
                        use wasm_bindgen::JsCast;
                        if let Some(target) = ev.target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                if let Some(anchor) = element
                                    .closest(".orbital-history__citation-ref")
                                    .ok()
                                    .flatten()
                                {
                                    ev.prevent_default();
                                    if let Some(id) = anchor.get_attribute("data-citation-id") {
                                        if let Some(cb) = &citation_cb {
                                            cb.run(id);
                                        }
                                    }
                                    return;
                                }
                                if let Some(anchor) = element
                                    .closest(".orbital-history__mention-ref")
                                    .ok()
                                    .flatten()
                                {
                                    ev.prevent_default();
                                    if let Some(id) = anchor.get_attribute("data-mention-id") {
                                        if let Some(cb) = &mention_cb {
                                            cb.run(id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(not(feature = "hydrate"))]
                    {
                        let _ = (&ev, &events);
                    }
                }
                on:mouseover=move |ev| {
                    #[cfg(feature = "hydrate")]
                    {
                        use wasm_bindgen::JsCast;
                        if !features.contains(HistoryFeatures::MARKDOWN_MENTIONS) {
                            return;
                        }
                        if let Some(target) = ev.target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                if let Some(anchor) = element
                                    .closest(".orbital-history__mention-ref")
                                    .ok()
                                    .flatten()
                                {
                                    if let Some(id) = anchor.get_attribute("data-mention-id") {
                                        if let Some(mention) =
                                            mentions.iter().find(|m| m.id == id).cloned()
                                        {
                                            let rect = anchor.get_bounding_client_rect();
                                            active_mention.set(Some((
                                                mention,
                                                rect.left(),
                                                rect.bottom() + 4.0,
                                            )));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                on:mouseout=move |ev| {
                    #[cfg(feature = "hydrate")]
                    {
                        use wasm_bindgen::JsCast;
                        if let Some(target) = ev.target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                if element.closest(".orbital-history__mention-ref").ok().flatten()
                                    .is_some()
                                {
                                    active_mention.set(None);
                                }
                            }
                        }
                    }
                }
            />
            <Show when=move || active_mention.get().is_some() fallback=|| ()>
                {move || {
                    let Some((mention, left, top)) = active_mention.get() else {
                        return ().into_any();
                    };
                    let mut config = PersonaConfig::named(mention.display_name.clone());
                    config.size = PersonaSize::Small;
                    config.avatar_src = mention.avatar_src.clone();
                    let style = format!("left: {left}px; top: {top}px;");
                    match mention.subtitle {
                        Some(subtitle) => view! {
                            <div
                                class="orbital-history__mention-popover"
                                role="tooltip"
                                style=style
                            >
                                <Persona config=config>
                                    <PersonaSecondaryText slot>{subtitle}</PersonaSecondaryText>
                                </Persona>
                            </div>
                        }
                        .into_any(),
                        None => view! {
                            <div
                                class="orbital-history__mention-popover"
                                role="tooltip"
                                style=style
                            >
                                <Persona config=config />
                            </div>
                        }
                        .into_any(),
                    }
                }}
            </Show>
        </div>
    }
}
