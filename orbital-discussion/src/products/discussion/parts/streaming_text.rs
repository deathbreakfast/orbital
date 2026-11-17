use leptos::prelude::*;
use orbital_core_components::inject_markdown_styles;

use crate::render_markdown;

/// Markdown text with an optional streaming cursor suffix.
#[component]
pub fn DiscussionStreamingTextView(
    markdown: String,
    streaming: bool,
    markdown_enabled: bool,
    citations: Vec<crate::DiscussionCitation>,
    attachment_urls: Vec<String>,
) -> impl IntoView {
    let url_refs: Vec<&str> = attachment_urls.iter().map(String::as_str).collect();

    view! {
        {if markdown_enabled {
            inject_markdown_styles();
            let html = render_markdown(&markdown, &citations, &url_refs);
            view! {
                <div class="orbital-discussion__markdown orbital-discussion__markdown--streaming">
                    <span inner_html=html />
                    {streaming.then(|| view! {
                        <span
                            class="orbital-discussion__streaming-cursor"
                            data-testid="discussion-streaming-cursor"
                            aria-hidden="true"
                        >
                            "▌"
                        </span>
                    })}
                </div>
            }
            .into_any()
        } else {
            view! {
                <span class="orbital-discussion__reply-text">
                    {markdown.clone()}
                    {streaming.then(|| view! {
                        <span
                            class="orbital-discussion__streaming-cursor"
                            data-testid="discussion-streaming-cursor"
                            aria-hidden="true"
                        >
                            "▌"
                        </span>
                    })}
                </span>
            }
            .into_any()
        }}
    }
}
