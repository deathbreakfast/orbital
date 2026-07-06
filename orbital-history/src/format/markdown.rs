use orbital_markdown::{render_to_html, CitationRef, OrbitalMarkdownOptions, RenderContext};

use crate::types::HistoryCitation;

/// Render a history change body as sanitized HTML (read-only).
pub fn render_history_markdown(markdown: &str, citations: &[HistoryCitation]) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }
    let refs: Vec<CitationRef<'_>> = citations
        .iter()
        .map(|c| CitationRef {
            id: &c.id,
            display_index: c.display_index,
        })
        .collect();
    let enable_citations = !citations.is_empty();
    render_to_html(
        markdown,
        &OrbitalMarkdownOptions {
            enable_citation_refs: enable_citations,
            enable_images: false,
            citation_style: orbital_markdown::CitationLinkStyle::history(),
        },
        &RenderContext {
            citations: &refs,
            attachment_urls: &[],
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HistoryCitation;

    #[test]
    fn bold_markdown() {
        let html = render_history_markdown("**bold** text", &[]);
        assert!(html.contains("<strong>") || html.contains("<b>"));
    }

    #[test]
    fn history_citation_anchors() {
        let citations = vec![HistoryCitation {
            id: "audit-1".into(),
            display_index: 1,
        }];
        let html = render_history_markdown("See [^audit-1] here.", &citations);
        assert!(html.contains("history-citation-ref-audit-1"));
        assert!(html.contains("orbital-history__citation-ref"));
    }
}
