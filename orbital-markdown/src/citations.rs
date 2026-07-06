use regex::Regex;

use crate::citation_style::CitationLinkStyle;

/// Citation id + 1-based display index for ref resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CitationRef<'a> {
    pub id: &'a str,
    pub display_index: usize,
}

/// Replace `[^id]` in HTML output with superscript anchor links.
pub fn replace_citation_refs(
    html: &str,
    citations: &[CitationRef<'_>],
    style: CitationLinkStyle,
) -> String {
    if citations.is_empty() {
        return html.to_string();
    }

    let mut result = html.to_string();
    for citation in citations {
        let pattern = format!(r"\[\^{}\]", regex::escape(citation.id));
        let Ok(re) = Regex::new(&pattern) else {
            continue;
        };
        let row_anchor = format!("{}{}", style.row_anchor_prefix, citation.id);
        let ref_id = format!("{}{}", style.ref_id_prefix, citation.id);
        let replacement = format!(
            r##"<sup class="{class}"><a href="#{row_anchor}" id="{ref_id}" data-citation-id="{id}">{index}</a></sup>"##,
            class = style.anchor_class,
            row_anchor = row_anchor,
            ref_id = ref_id,
            id = citation.id,
            index = citation.display_index,
        );
        result = re.replace_all(&result, replacement.as_str()).to_string();
    }
    result
}

/// Strip markdown image syntax for URLs already rendered as File parts.
pub fn strip_duplicate_images(markdown: &str, attachment_urls: &[&str]) -> String {
    if attachment_urls.is_empty() {
        return markdown.to_string();
    }

    let mut result = markdown.to_string();
    for url in attachment_urls {
        let escaped = regex::escape(url);
        let pattern = format!(r"!\[[^\]]*\]\({escaped}\)");
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, "").to_string();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_discussion_citation_ref_in_html() {
        let html = "<p>See [^cit-1] for details.</p>";
        let citations = vec![CitationRef {
            id: "cit-1",
            display_index: 1,
        }];
        let out = replace_citation_refs(html, &citations, CitationLinkStyle::discussion());
        assert!(out.contains("discussion-citation-row-cit-1"));
        assert!(out.contains(">1</a>"));
        assert!(!out.contains("[^cit-1]"));
    }

    #[test]
    fn replaces_history_citation_ref_in_html() {
        let html = "<p>See [^audit-1] for details.</p>";
        let citations = vec![CitationRef {
            id: "audit-1",
            display_index: 2,
        }];
        let out = replace_citation_refs(html, &citations, CitationLinkStyle::history());
        assert!(out.contains("history-citation-row-audit-1"));
        assert!(out.contains("history-citation-ref-audit-1"));
        assert!(out.contains("orbital-history__citation-ref"));
    }
}
