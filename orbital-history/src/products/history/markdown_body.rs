use leptos::html::Div;
use leptos::prelude::*;
use orbital_base_components::OverlayAppearance;
use orbital_core_components::{
    link_styles, navigation::popover::popover_styles, overlay::overlay_surface_class,
    overlay::FloatingPanel, overlay::ThemedPortal, Persona, PersonaConfig, PersonaSecondaryText,
    PersonaSize, PopoverSize,
};
use orbital_style::inject_style;

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
    inject_style("orbital-link", link_styles());
    inject_style("orbital-popover", popover_styles());

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
    let markdown_ref = NodeRef::<Div>::new();
    let mentions_store = StoredValue::new(mentions.clone());
    let features_store = StoredValue::new(features);

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let Some(el) = markdown_ref.get() else {
                return;
            };
            let active = active_mention;
            let features = features_store.get_value();
            let mentions = mentions_store.get_value();

            let on_over = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(
                move |ev: web_sys::MouseEvent| {
                    if !features.contains(HistoryFeatures::MARKDOWN_MENTIONS) {
                        return;
                    }
                    let Some(target) = ev.target() else {
                        return;
                    };
                    let Ok(element) = target.dyn_into::<web_sys::Element>() else {
                        return;
                    };
                    let Some(anchor) = element
                        .closest(".orbital-history__mention-ref")
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let Some(id) = anchor.get_attribute("data-mention-id") else {
                        return;
                    };
                    let Some(mention) = mentions.iter().find(|m| m.id == id).cloned() else {
                        return;
                    };
                    let rect = anchor.get_bounding_client_rect();
                    active.set(Some((mention, rect.left(), rect.bottom() + 4.0)));
                },
            );
            let on_out = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(
                move |ev: web_sys::MouseEvent| {
                    let Some(target) = ev.target() else {
                        return;
                    };
                    let Ok(element) = target.dyn_into::<web_sys::Element>() else {
                        return;
                    };
                    let Some(anchor) = element
                        .closest(".orbital-history__mention-ref")
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    if let Some(related) = ev.related_target() {
                        if let Ok(related_el) = related.dyn_into::<web_sys::Element>() {
                            if anchor.contains(Some(&related_el)) {
                                return;
                            }
                        }
                    }
                    active.set(None);
                },
            );
            el.add_event_listener_with_callback("mouseover", on_over.as_ref().unchecked_ref())
                .ok();
            el.add_event_listener_with_callback("mouseout", on_out.as_ref().unchecked_ref())
                .ok();
            on_over.forget();
            on_out.forget();
        });
    }

    view! {
        <div class="orbital-history__markdown-surface">
            <div
                node_ref=markdown_ref
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
                    let surface_class = Signal::derive(move || {
                        overlay_surface_class(
                            "orbital-popover-surface",
                            OverlayAppearance::Default,
                            Some(PopoverSize::Small.as_str()),
                        )
                    });
                    match mention.subtitle {
                        Some(subtitle) => view! {
                            <ThemedPortal immediate=true>
                                <div
                                    class="orbital-popover-shell orbital-history__mention-popover-anchor"
                                    style=style
                                >
                                    <FloatingPanel
                                        class=surface_class
                                        body_class="orbital-popover-body"
                                        role="tooltip"
                                    >
                                        <Persona config=config>
                                            <PersonaSecondaryText slot>{subtitle}</PersonaSecondaryText>
                                        </Persona>
                                    </FloatingPanel>
                                </div>
                            </ThemedPortal>
                        }
                        .into_any(),
                        None => view! {
                            <ThemedPortal immediate=true>
                                <div
                                    class="orbital-popover-shell orbital-history__mention-popover-anchor"
                                    style=style
                                >
                                    <FloatingPanel
                                        class=surface_class
                                        body_class="orbital-popover-body"
                                        role="tooltip"
                                    >
                                        <Persona config=config />
                                    </FloatingPanel>
                                </div>
                            </ThemedPortal>
                        }
                        .into_any(),
                    }
                }}
            </Show>
        </div>
    }
}
