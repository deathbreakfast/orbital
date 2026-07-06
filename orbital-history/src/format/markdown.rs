use orbital_markdown::{render_to_html, CitationRef, MentionLinkStyle, OrbitalMarkdownOptions, RenderContext};

use crate::types::{HistoryAttachment, HistoryCitation, HistoryMention};

/// Feature toggles for history markdown rendering.
#[derive(Clone, Copy, Debug, Default)]
pub struct HistoryMarkdownRenderOptions {
    pub enable_citations: bool,
    pub enable_mentions: bool,
    pub enable_images: bool,
}

/// Render a history change body as sanitized HTML (read-only).
pub fn render_history_markdown(
    markdown: &str,
    citations: &[HistoryCitation],
    _mentions: &[HistoryMention],
    attachments: &[HistoryAttachment],
    options: HistoryMarkdownRenderOptions,
) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }
    let cite_refs: Vec<CitationRef<'_>> = citations
        .iter()
        .map(|c| CitationRef {
            id: &c.id,
            display_index: c.display_index,
        })
        .collect();
    let attachment_urls: Vec<&str> = attachments.iter().map(|a| a.url.as_str()).collect();
    let mut html = render_to_html(
        markdown,
        &OrbitalMarkdownOptions {
            enable_citation_refs: options.enable_citations && !citations.is_empty(),
            enable_mention_refs: options.enable_mentions,
            enable_images: options.enable_images,
            citation_style: orbital_markdown::CitationLinkStyle::history(),
            mention_style: MentionLinkStyle::history(),
        },
        &RenderContext {
            citations: &cite_refs,
            attachment_urls: &attachment_urls,
        },
    );

    if options.enable_images {
        html = append_attachment_images(&html, attachments);
    }

    html
}

fn is_image_attachment(att: &HistoryAttachment) -> bool {
    if att
        .mime
        .as_deref()
        .is_some_and(|mime| mime.starts_with("image/"))
    {
        return true;
    }
    att.url
        .rsplit('/')
        .next()
        .is_some_and(|name| {
            matches!(
                name.rsplit_once('.').map(|(_, ext)| ext.to_ascii_lowercase()),
                Some(ext) if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg")
            )
        })
}

/// Render image attachments not already present in markdown HTML (discussion File-part parity).
fn append_attachment_images(html: &str, attachments: &[HistoryAttachment]) -> String {
    let mut out = html.to_string();
    for att in attachments {
        if !is_image_attachment(att) || out.contains(&att.url) {
            continue;
        }
        let alt = att.name.as_deref().unwrap_or("");
        out.push_str(&format!(
            "<p><img class=\"orbital-markdown__image\" loading=\"lazy\" src=\"{}\" alt=\"{alt}\"></p>",
            att.url
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_markdown() {
        let html = render_history_markdown(
            "**bold** text",
            &[],
            &[],
            &[],
            HistoryMarkdownRenderOptions::default(),
        );
        assert!(html.contains("<strong>") || html.contains("<b>"));
    }

    #[test]
    fn history_citation_anchors() {
        let citations = vec![HistoryCitation {
            id: "audit-1".into(),
            display_index: 1,
        }];
        let html = render_history_markdown(
            "See [^audit-1] here.",
            &citations,
            &[],
            &[],
            HistoryMarkdownRenderOptions {
                enable_citations: true,
                ..Default::default()
            },
        );
        assert!(html.contains("history-citation-ref-audit-1"));
    }

    #[test]
    fn history_mention_anchors() {
        let html = render_history_markdown(
            "Hi @[Jordan Lee](u1)!",
            &[],
            &[],
            &[],
            HistoryMarkdownRenderOptions {
                enable_mentions: true,
                ..Default::default()
            },
        );
        assert!(html.contains("data-mention-id=\"u1\""));
        assert!(html.contains("orbital-history__mention-ref"));
    }

    #[test]
    fn history_markdown_image() {
        let html = render_history_markdown(
            "![shot](https://example.com/a.png)",
            &[],
            &[],
            &[],
            HistoryMarkdownRenderOptions {
                enable_images: true,
                ..Default::default()
            },
        );
        assert!(html.contains("orbital-markdown__image"));
    }

    #[test]
    fn dedupes_inline_image_when_attachment_url_matches() {
        let attachments = vec![HistoryAttachment {
            url: "https://example.com/a.png".into(),
            name: Some("a.png".into()),
            mime: Some("image/png".into()),
        }];
        let html = render_history_markdown(
            "![shot](https://example.com/a.png)",
            &[],
            &[],
            &attachments,
            HistoryMarkdownRenderOptions {
                enable_images: true,
                ..Default::default()
            },
        );
        assert_eq!(html.matches("orbital-markdown__image").count(), 1);
    }
}
