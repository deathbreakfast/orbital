use orbital_markdown::{render_to_html, OrbitalMarkdownOptions, RenderContext};

/// Render a history change body as sanitized HTML (read-only).
pub fn render_history_markdown(markdown: &str) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }
    render_to_html(
        markdown,
        &OrbitalMarkdownOptions::discussion_body(),
        &RenderContext {
            citations: &[],
            attachment_urls: &[],
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_markdown() {
        let html = render_history_markdown("**bold** text");
        assert!(html.contains("<strong>") || html.contains("<b>"));
    }
}
