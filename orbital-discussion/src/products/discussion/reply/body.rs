use leptos::prelude::*;
use orbital_core_components::{link_styles, Link};
use orbital_style::inject_style;
use std::sync::Arc;

use crate::{
    format_file_size, is_image_mime, render_markdown, use_discussion, DiscussionFeatures,
    DiscussionPart, DiscussionReply, ReplyRenderContext,
};

use super::super::parts::{
    DiscussionReasoningPartView, DiscussionStepPartView, DiscussionStreamingTextView,
    DiscussionToolPartView,
};

/// Reply body with part dispatch and optional markdown rendering.
#[component]
pub fn DiscussionReplyBody(reply: DiscussionReply, features: DiscussionFeatures) -> impl IntoView {
    inject_style("orbital-link", link_styles());

    let ctx = use_discussion();
    let markdown_enabled = features.contains(DiscussionFeatures::MARKDOWN);
    let custom_parts = features.contains(DiscussionFeatures::CUSTOM_PARTS);
    let agent_parts = features.contains(DiscussionFeatures::AGENT_PARTS);

    let citations = reply.citations.clone();
    let attachment_urls: Arc<[String]> = reply
        .parts
        .iter()
        .filter_map(|part| {
            if let DiscussionPart::File { url, .. } = part {
                Some(url.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into();

    let reply_parts = reply.parts.clone();
    let reply_id = reply.id.clone();

    view! {
        <div class="orbital-discussion__reply-body">
            <For
                each=move || reply_parts.clone()
                key=|part| part_key(part)
                children=move |part| {
                    let url_refs: Vec<&str> = attachment_urls.iter().map(String::as_str).collect();
                    render_part(
                        reply.clone(),
                        reply_id.clone(),
                        part,
                        markdown_enabled,
                        custom_parts,
                        agent_parts,
                        &citations,
                        &url_refs,
                        ctx,
                    )
                }
            />
        </div>
    }
}

fn part_key(part: &DiscussionPart) -> String {
    match part {
        DiscussionPart::Text {
            markdown,
            streaming,
        } => {
            format!("text:{markdown}:stream={streaming}")
        }
        DiscussionPart::File { name, .. } => format!("file:{name}"),
        DiscussionPart::Custom { kind, .. } => format!("custom:{kind}"),
        DiscussionPart::Tool(tool) => format!("tool:{}", tool.tool_call_id),
        DiscussionPart::Reasoning(reasoning) => format!("reasoning:{}", reasoning.text),
        DiscussionPart::Step(step) => format!("step:{}", step.step_number),
    }
}

fn render_part(
    reply: DiscussionReply,
    reply_id: String,
    part: DiscussionPart,
    markdown_enabled: bool,
    custom_parts: bool,
    agent_parts: bool,
    citations: &[crate::DiscussionCitation],
    attachment_urls: &[&str],
    ctx: crate::DiscussionContext,
) -> AnyView {
    if custom_parts {
        if let Some(view) = ctx.renderers.with_value(|renderers| {
            renderers
                .part_view
                .as_ref()
                .and_then(|view| view(ReplyRenderContext::new(reply.clone()), part.clone()))
        }) {
            return view;
        }
    }

    match part {
        DiscussionPart::Text {
            markdown,
            streaming,
        } => {
            if agent_parts && streaming {
                return view! {
                    <DiscussionStreamingTextView
                        markdown=markdown
                        streaming=true
                        markdown_enabled=markdown_enabled
                        citations=citations.to_vec()
                        attachment_urls=attachment_urls.iter().map(|s| (*s).to_string()).collect()
                    />
                }
                .into_any();
            }

            if markdown_enabled {
                let html = render_markdown(&markdown, citations, attachment_urls);
                view! {
                    <div class="orbital-discussion__markdown" inner_html=html />
                }
                .into_any()
            } else {
                view! {
                    <span class="orbital-discussion__reply-text">{markdown}</span>
                }
                .into_any()
            }
        }
        DiscussionPart::File {
            name,
            url,
            mime,
            size_bytes,
        } => {
            let size_label = format_file_size(size_bytes);
            if is_image_mime(mime.as_deref()) {
                view! {
                    <div
                        class="orbital-discussion__file-part orbital-discussion__file-part--image"
                        data-testid="discussion-file-part"
                        data-file-kind="image"
                    >
                        <img
                            class="orbital-discussion__file-image"
                            src=url.clone()
                            alt=name.clone()
                            loading="lazy"
                        />
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div
                        class="orbital-discussion__file-part orbital-discussion__file-part--download"
                        data-testid="discussion-file-part"
                        data-file-kind="download"
                    >
                        <Link href=url.clone() class="orbital-discussion__file-link".to_string()>
                            {name.clone()}
                            {size_label.map(|label| format!(" ({label})"))}
                        </Link>
                    </div>
                }
                .into_any()
            }
        }
        DiscussionPart::Custom { kind, .. } => view! {
            <span class="orbital-discussion__reply-text">{format!("[custom: {kind}]")}</span>
        }
        .into_any(),
        DiscussionPart::Tool(tool) => {
            if agent_parts {
                view! {
                    <DiscussionToolPartView reply_id=reply_id part=tool />
                }
                .into_any()
            } else {
                view! {
                    <span class="orbital-discussion__reply-text">
                        {format!("[tool: {}]", tool.tool_name)}
                    </span>
                }
                .into_any()
            }
        }
        DiscussionPart::Reasoning(reasoning) => {
            if agent_parts {
                view! { <DiscussionReasoningPartView part=reasoning /> }.into_any()
            } else {
                view! {
                    <span class="orbital-discussion__reply-text">"[reasoning]"</span>
                }
                .into_any()
            }
        }
        DiscussionPart::Step(step) => {
            if agent_parts {
                view! { <DiscussionStepPartView part=step /> }.into_any()
            } else {
                view! {
                    <span class="orbital-discussion__reply-text">
                        {format!("[step: {}]", step.label)}
                    </span>
                }
                .into_any()
            }
        }
    }
}
