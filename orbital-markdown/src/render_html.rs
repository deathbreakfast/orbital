use pulldown_cmark::{html, Options, Parser};

use crate::citations::{replace_citation_refs, strip_duplicate_images, CitationRef};
use crate::mentions::{prepare_mention_markdown, replace_mention_links};
use crate::options::OrbitalMarkdownOptions;
use crate::sanitize::sanitize_html;

/// Context for resolving citation refs and deduping images.
#[derive(Clone, Debug, Default)]
pub struct RenderContext<'a> {
    pub citations: &'a [CitationRef<'a>],
    pub attachment_urls: &'a [&'a str],
}

/// Render markdown to sanitized HTML.
pub fn render_to_html(
    markdown: &str,
    options: &OrbitalMarkdownOptions,
    ctx: &RenderContext<'_>,
) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }

    let mut source = if options.enable_images {
        strip_duplicate_images(markdown, ctx.attachment_urls)
    } else {
        markdown.to_string()
    };

    if options.enable_mention_refs {
        source = prepare_mention_markdown(&source);
    }

    let mut parse_options = Options::empty();
    parse_options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&source, parse_options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut html = html_output;
    if options.enable_mention_refs {
        html = replace_mention_links(&html, options.mention_style);
    }

    html = sanitize_html(&html, options.enable_images);

    if options.enable_images {
        html = html.replace(
            "<img src=",
            "<img class=\"orbital-markdown__image\" loading=\"lazy\" src=",
        );
    }

    if options.enable_citation_refs {
        html = replace_citation_refs(&html, ctx.citations, options.citation_style);
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CitationLinkStyle;

    #[test]
    fn bold_renders() {
        let html = render_to_html(
            "Hello **bold**",
            &OrbitalMarkdownOptions::default(),
            &RenderContext::default(),
        );
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn citation_ref_in_body() {
        let citations = [CitationRef {
            id: "cit-1",
            display_index: 1,
        }];
        let ctx = RenderContext {
            citations: &citations,
            attachment_urls: &[],
        };
        let html = render_to_html(
            "See [^cit-1] here.",
            &OrbitalMarkdownOptions {
                enable_citation_refs: true,
                enable_mention_refs: false,
                enable_images: false,
                citation_style: CitationLinkStyle::discussion(),
                mention_style: Default::default(),
            },
            &ctx,
        );
        assert!(html.contains("discussion-citation-ref-cit-1"));
    }

    #[test]
    fn mention_ref_in_body() {
        let html = render_to_html(
            "Hi @[Jordan Lee](u1)!",
            &OrbitalMarkdownOptions {
                enable_citation_refs: false,
                enable_mention_refs: true,
                enable_images: false,
                citation_style: CitationLinkStyle::history(),
                mention_style: crate::MentionLinkStyle::history(),
            },
            &RenderContext::default(),
        );
        assert!(html.contains("data-mention-id=\"u1\""));
        assert!(html.contains("orbital-history__mention-ref"));
    }
}
