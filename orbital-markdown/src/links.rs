use regex::Regex;

use crate::mentions::MENTION_HREF_PREFIX;

/// CSS classes applied to theme-aware inline links in markdown HTML.
pub const ORBITAL_LINK_INLINE_CLASS: &str = "orbital-link orbital-link--inline";

fn is_special_link(attrs: &str) -> bool {
    attrs.contains("data-citation-id")
        || attrs.contains("data-mention-id")
        || attrs.contains("orbital-history__mention-ref")
        || attrs.contains("orbital-history__citation-ref")
        || attrs.contains("orbital-markdown__citation-ref")
        || attrs.contains("discussion-citation-ref")
        || attrs.contains(MENTION_HREF_PREFIX)
}

fn add_link_class(attrs: &str) -> String {
    if attrs.contains("orbital-link") {
        return attrs.to_string();
    }

    if let Ok(class_re) = Regex::new(r#"class="([^"]*)""#) {
        if class_re.is_match(attrs) {
            return class_re
                .replace(attrs, |caps: &regex::Captures| {
                    let existing = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                    format!(r#"class="{existing} {ORBITAL_LINK_INLINE_CLASS}""#)
                })
                .to_string();
        }
    }

    let trimmed = attrs.trim();
    if trimmed.is_empty() {
        format!(r#"class="{ORBITAL_LINK_INLINE_CLASS}""#)
    } else {
        format!(r#"class="{ORBITAL_LINK_INLINE_CLASS}" {trimmed}"#)
    }
}

/// Apply Orbital [`Link`](https://docs.rs/orbital-core-components/latest/orbital_core_components/struct.Link.html) classes to plain markdown anchors.
pub fn style_markdown_links(html: &str) -> String {
    let Ok(re) = Regex::new(r#"(?i)<a\s+([^>]*?)>"#) else {
        return html.to_string();
    };

    re.replace_all(html, |caps: &regex::Captures| {
        let attrs = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        if is_special_link(attrs) {
            return format!("<a {attrs}>");
        }
        let styled = add_link_class(attrs);
        format!("<a {styled}>")
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styles_plain_markdown_link() {
        let html =
            r#"<p>See <a href="https://example.com" rel="noopener noreferrer">design doc</a>.</p>"#;
        let out = style_markdown_links(html);
        assert!(out.contains(r#"class="orbital-link orbital-link--inline""#));
        assert!(out.contains(r#"href="https://example.com""#));
    }

    #[test]
    fn citation_ref_links_include_orbital_link_class() {
        let html = "<sup class=\"orbital-markdown__citation-ref\"><a href=\"#row\" data-citation-id=\"c1\">1</a></sup>";
        let out = style_markdown_links(html);
        assert!(!out.contains("orbital-link"));
    }

    #[test]
    fn skips_mention_ref_links() {
        let html = "<a href=\"#mention-u1\" class=\"orbital-history__mention-ref\" data-mention-id=\"u1\">@Jordan</a>";
        let out = style_markdown_links(html);
        assert!(!out.contains("orbital-link"));
    }
}
