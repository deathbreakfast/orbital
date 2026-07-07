use regex::Regex;

use crate::links::ORBITAL_LINK_INLINE_CLASS;
use crate::mention_style::MentionLinkStyle;

/// Mention id + display label for `@[label](id)` resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MentionRef<'a> {
    pub id: &'a str,
    pub display_label: &'a str,
}

/// Sentinel href prefix for mention links produced by [`prepare_mention_markdown`].
pub const MENTION_HREF_PREFIX: &str = "mention-ref:";

/// Rewrite `@[label](id)` to markdown links the parser understands.
pub fn prepare_mention_markdown(markdown: &str) -> String {
    let Ok(re) = Regex::new(r"@\[([^\]]+)\]\(([^)]+)\)") else {
        return markdown.to_string();
    };
    re.replace_all(markdown, |caps: &regex::Captures| {
        let label = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let id = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        format!("[@{label}]({MENTION_HREF_PREFIX}{id})")
    })
    .to_string()
}

/// Style mention links injected during markdown preparation.
pub fn replace_mention_links(html: &str, style: MentionLinkStyle) -> String {
    let Ok(re) = Regex::new(&format!(
        r#"(?i)<a href="{prefix}([^"]+)">([^<]*)</a>"#,
        prefix = regex::escape(MENTION_HREF_PREFIX)
    )) else {
        return html.to_string();
    };

    re.replace_all(html, |caps: &regex::Captures| {
        let id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let label = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        format!(
            r##"<a href="#mention-{id}" class="{class} {link_class}" data-mention-id="{id}">{label}</a>"##,
            class = style.anchor_class,
            link_class = ORBITAL_LINK_INLINE_CLASS,
            id = id,
            label = label,
        )
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_and_replaces_mention_links() {
        let md = prepare_mention_markdown("Hi @[Jordan Lee](u1)!");
        assert!(md.contains("mention-ref:u1"));
        let html = format!("<p>Hi <a href=\"mention-ref:u1\">@Jordan Lee</a>!</p>");
        let out = replace_mention_links(&html, MentionLinkStyle::history());
        assert!(out.contains("data-mention-id=\"u1\""));
        assert!(out.contains("orbital-history__mention-ref"));
        assert!(out.contains("orbital-link orbital-link--inline"));
    }
}
