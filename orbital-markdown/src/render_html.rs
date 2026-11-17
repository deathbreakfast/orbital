use pulldown_cmark::{html, Options, Parser, TextMergeStream};

use crate::citations::{replace_citation_refs, strip_duplicate_images, CitationRef};
use crate::events::{expand_placeholders, strip_placeholders, transform};
use crate::links::style_markdown_links;
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

    let markdown = strip_placeholders(markdown);
    let mut source = if options.enable_images {
        strip_duplicate_images(&markdown, ctx.attachment_urls)
    } else {
        markdown
    };

    if options.enable_mention_refs {
        source = prepare_mention_markdown(&source);
    }

    let mut parse_options = Options::empty();
    parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    parse_options.insert(Options::ENABLE_TABLES);
    parse_options.insert(Options::ENABLE_TASKLISTS);
    parse_options.insert(Options::ENABLE_GFM);

    let parser = TextMergeStream::new(Parser::new_ext(&source, parse_options));
    let mut html_output = String::new();
    html::push_html(&mut html_output, transform(parser).into_iter());

    let mut html = html_output;
    if options.enable_mention_refs {
        html = replace_mention_links(&html, options.mention_style);
    }

    html = sanitize_html(
        &html,
        options.enable_images,
        options.image_url_schemes.as_deref(),
    );

    html = expand_placeholders(&html);
    html = add_block_classes(&html);

    if options.enable_images {
        html = html.replace(
            "<img src=",
            "<img class=\"orbital-image orbital-image--rounded orbital-markdown__image\" loading=\"lazy\" src=",
        );
    }

    if options.enable_citation_refs {
        html = replace_citation_refs(&html, ctx.citations, options.citation_style);
    }

    style_markdown_links(&html)
}

/// Class the block elements the sanitizer kept. Tables take Orbital `Table`
/// classes and sit in a scroll box, so a wide table scrolls sideways on its
/// own; the box takes focus so keyboard users can scroll it too.
///
/// Runs on sanitized output, where every tag is lowercase and only `th`/`td`
/// (`style`) and `details` (`open`) carry attributes, so plain string matches
/// are exact. `<th` is matched with its `>` or ` ` so `<thead` is left alone.
fn add_block_classes(html: &str) -> String {
    html.replace(
        "<table>",
        "<div class=\"orbital-markdown__table-scroll\" tabindex=\"0\"><table class=\"orbital-table orbital-markdown__table\">",
    )
    .replace("</table>", "</table></div>")
    .replace("<thead>", "<thead class=\"orbital-table-header\">")
    .replace("<tbody>", "<tbody class=\"orbital-table-body\">")
    .replace("<tr>", "<tr class=\"orbital-table-row\">")
    .replace("<th>", "<th class=\"orbital-table-header-cell\">")
    .replace("<th style=", "<th class=\"orbital-table-header-cell\" style=")
    .replace("<td>", "<td class=\"orbital-table-cell\">")
    .replace("<td style=", "<td class=\"orbital-table-cell\" style=")
    .replace("<blockquote>", "<blockquote class=\"orbital-markdown__blockquote\">")
    .replace("<hr>", "<hr class=\"orbital-markdown__rule\">")
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
                image_url_schemes: None,
                citation_style: CitationLinkStyle::discussion(),
                mention_style: Default::default(),
            },
            &ctx,
        );
        assert!(html.contains("discussion-citation-ref-cit-1"));
        assert!(html.contains("orbital-link orbital-link--inline"));
    }

    #[test]
    fn markdown_link_uses_orbital_link_classes() {
        let html = render_to_html(
            "See [design doc](https://example.com).",
            &OrbitalMarkdownOptions::default(),
            &RenderContext::default(),
        );
        assert!(html.contains("orbital-link orbital-link--inline"));
        assert!(html.contains("design doc"));
    }

    #[test]
    fn mention_ref_in_body() {
        let html = render_to_html(
            "Hi @[Jordan Lee](u1)!",
            &OrbitalMarkdownOptions {
                enable_citation_refs: false,
                enable_mention_refs: true,
                enable_images: false,
                image_url_schemes: None,
                citation_style: CitationLinkStyle::history(),
                mention_style: crate::MentionLinkStyle::history(),
            },
            &RenderContext::default(),
        );
        assert!(html.contains("data-mention-id=\"u1\""));
        assert!(html.contains("orbital-history__mention-ref"));
        assert!(html.contains("orbital-link orbital-link--inline"));
    }

    fn render_images(md: &str, schemes: Option<&[&str]>) -> String {
        render_to_html(
            md,
            &OrbitalMarkdownOptions {
                enable_images: true,
                image_url_schemes: schemes.map(|s| s.iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            },
            &RenderContext::default(),
        )
    }

    #[test]
    fn image_schemes_none_keeps_default_set() {
        assert!(render_images("![a](https://x.test/a.png)", None).contains("<img"));
        assert!(render_images("![a](mailto:a@x.test)", None).contains("src=\"mailto:"));
    }

    #[test]
    fn image_schemes_https_only() {
        let s = Some(&["https"][..]);
        assert!(render_images("![a](https://x.test/a.png)", s).contains("src=\"https://"));
        assert!(render_images("![a](HTTPS://x.test/a.png)", s).contains("<img"));
        for url in [
            "http://x.test/a.png",
            "mailto:a@x.test",
            "data:image/png;base64,AA",
        ] {
            let html = render_images(&format!("![a]({url})"), s);
            assert!(!html.contains("src="), "{url} kept: {html}");
        }
    }

    #[test]
    fn image_schemes_keep_relative_src() {
        let s = Some(&["https"][..]);
        assert!(render_images("![a](/files/a.png)", s).contains("src=\"/files/a.png\""));
    }

    #[test]
    fn image_schemes_leave_links_alone() {
        let html = render_images("[l](http://x.test)", Some(&["https"][..]));
        assert!(html.contains("href=\"http://x.test\""));
    }

    #[test]
    fn image_schemes_cannot_widen() {
        let html = render_images("![a](ftp://x.test/a.png)", Some(&["ftp"][..]));
        assert!(!html.contains("src="));
    }

    fn render_plain(md: &str) -> String {
        render_to_html(
            md,
            &OrbitalMarkdownOptions::default(),
            &RenderContext::default(),
        )
    }

    const TABLE: &str = "| Feature | Status |\n|---|:---:|\n| [Rich content](/magnon/projects/1) | `Built` |\n| Charts | Planned |\n";

    #[test]
    fn gfm_table_renders_with_head_body_and_scroll_wrapper() {
        let html = render_plain(TABLE);
        assert!(html.contains(
            "<div class=\"orbital-markdown__table-scroll\" tabindex=\"0\"><table class=\"orbital-table orbital-markdown__table\"><thead class=\"orbital-table-header\">"
        ), "{html}");
        assert!(
            html.contains("<tbody class=\"orbital-table-body\">\n<tr class=\"orbital-table-row\">"),
            "{html}"
        );
        assert!(html.contains("</table></div>"), "{html}");
        assert!(
            html.contains("<th class=\"orbital-table-header-cell\">Feature</th>"),
            "{html}"
        );
    }

    #[test]
    fn gfm_table_cells_keep_links_code_and_alignment() {
        let html = render_plain(TABLE);
        assert!(html.contains("href=\"/magnon/projects/1\""), "{html}");
        assert!(html.contains("orbital-link orbital-link--inline"), "{html}");
        assert!(html.contains("<code>Built</code>"), "{html}");
        assert!(
            html.contains("<td class=\"orbital-table-cell\" style=\"text-align: center\">"),
            "{html}"
        );
    }

    #[test]
    fn table_cell_style_other_than_alignment_is_dropped() {
        let html = render_plain(
            "<table><tr><td style=\"background:url(javascript:alert(1))\">x</td><td style=\"text-align: right; color: red\">y</td></tr></table>",
        );
        assert!(!html.contains("style="), "{html}");
        assert!(
            html.contains("<td class=\"orbital-table-cell\">x</td>"),
            "{html}"
        );
    }

    #[test]
    fn pipe_text_without_delimiter_row_stays_a_paragraph() {
        let html = render_plain("a | b\nc | d");
        assert!(!html.contains("<table"), "{html}");
        assert!(html.starts_with("<p>"), "{html}");
    }

    #[test]
    fn headings_blockquotes_and_rules_render() {
        let html = render_plain("## Vision\n\n> **Tagline**\n\n---\n\n#### Small");
        assert!(html.contains("<h2>Vision</h2>"), "{html}");
        assert!(html.contains(
            "<blockquote class=\"orbital-markdown__blockquote\">\n<p><strong>Tagline</strong></p>"
        ), "{html}");
        assert!(
            html.contains("<hr class=\"orbital-markdown__rule\">"),
            "{html}"
        );
        assert!(html.contains("<h4>Small</h4>"), "{html}");
    }

    #[test]
    fn raw_html_block_tags_get_no_classes_or_handlers() {
        let html = render_plain(
            "<blockquote onclick=\"x()\">q</blockquote>\n\n<h2 id=\"spoof\" class=\"evil\">t</h2>",
        );
        assert!(!html.contains("onclick"), "{html}");
        assert!(!html.contains("spoof"), "{html}");
        assert!(!html.contains("evil"), "{html}");
    }

    #[test]
    fn task_list_items_render_orbital_checkboxes() {
        let html = render_plain("- [x] Done\n- [ ] Open\n");
        assert!(html.contains(
            "<li><span class=\"orbital-checkbox orbital-checkbox--checked orbital-checkbox--medium orbital-markdown__task\"><input class=\"orbital-checkbox__input\" type=\"checkbox\" disabled checked aria-label=\"Task\"><span aria-hidden=\"true\" class=\"orbital-checkbox__indicator\"><svg class=\"orbital-checkbox__check-icon\""
        ), "{html}");
        assert!(html.contains(
            "<li><span class=\"orbital-checkbox orbital-checkbox--medium orbital-markdown__task\"><input class=\"orbital-checkbox__input\" type=\"checkbox\" disabled aria-label=\"Task\"><span aria-hidden=\"true\" class=\"orbital-checkbox__indicator\"></span></span>"
        ), "{html}");
        assert!(
            !html.contains("<div"),
            "task boxes must not put a div in a list item: {html}"
        );
    }

    #[test]
    fn raw_inputs_and_placeholder_characters_do_not_render() {
        let html = render_plain("<input type=\"text\" value=\"x\"> a\u{E000}b \u{E013}c");
        assert!(!html.contains("<input"), "{html}");
        assert!(html.contains("ab c"), "{html}");
    }

    #[test]
    fn github_alert_renders_as_message_bar() {
        let html = render_plain("> [!WARNING]\n> Deleting can't be undone.\n\nAfter.");
        assert!(html.contains(
            "<div class=\"orbital-message-bar orbital-message-bar--warning orbital-message-bar--multiline orbital-markdown__alert\" role=\"group\"><div class=\"orbital-message-bar__icon\"><svg"
        ), "{html}");
        assert!(html.contains(
            "<div class=\"orbital-message-bar-title\">Warning</div><blockquote class=\"orbital-message-bar-body orbital-markdown__alert-body\">"
        ), "{html}");
        assert!(
            html.contains("</blockquote></div>\n<p>After.</p>"),
            "{html}"
        );
        assert!(!html.contains("[!WARNING]"), "{html}");
    }

    #[test]
    fn alert_kinds_map_to_message_bar_intents() {
        for (kind, intent, title) in [
            ("NOTE", "info", "Note"),
            ("TIP", "success", "Tip"),
            ("IMPORTANT", "info", "Important"),
            ("CAUTION", "error", "Caution"),
        ] {
            let html = render_plain(&format!("> [!{kind}]\n> x\n"));
            assert!(
                html.contains(&format!("orbital-message-bar--{intent} ")),
                "{kind}: {html}"
            );
            assert!(html.contains(&format!(">{title}</div>")), "{kind}: {html}");
        }
    }

    #[test]
    fn quote_nested_in_an_alert_closes_in_order() {
        let html = render_plain("> [!NOTE]\n> outer\n>\n> > inner\n\nAfter.");
        assert_eq!(html.matches("<blockquote").count(), 2, "{html}");
        assert!(
            html.contains("inner</p>\n</blockquote>\n</blockquote></div>"),
            "{html}"
        );
        assert!(html.ends_with("<p>After.</p>\n"), "{html}");
    }

    #[test]
    fn raw_alert_class_stays_a_plain_quote() {
        let html = render_plain("<blockquote class=\"markdown-alert-note\">x</blockquote>");
        assert!(!html.contains("alert"), "{html}");
        assert!(html.contains("orbital-markdown__blockquote"), "{html}");
    }

    #[test]
    fn details_and_summary_render_without_handlers() {
        let html = render_plain(
            "<details open ontoggle=\"x()\">\n<summary>More</summary>\n\nHidden **text**\n\n</details>",
        );
        assert!(html.contains("<details open=\"\">"), "{html}");
        assert!(html.contains("<summary>More</summary>"), "{html}");
        assert!(html.contains("<strong>text</strong>"), "{html}");
        assert!(!html.contains("ontoggle"), "{html}");
    }

    #[test]
    fn bare_urls_become_links() {
        let html = render_plain("Docs at https://example.com/docs?a=1&b=2, and more.");
        assert!(
            html.contains("href=\"https://example.com/docs?a=1&amp;b=2\""),
            "{html}"
        );
        assert!(html.contains("orbital-link orbital-link--inline"), "{html}");
        assert!(
            html.contains(">https://example.com/docs?a=1&amp;b=2</a>, and more."),
            "{html}"
        );
    }

    #[test]
    fn urls_in_code_links_and_other_schemes_stay_text() {
        let html = render_plain(
            "`https://a.test` and [https://b.test](https://c.test)\n\n```\nhttps://d.test\n```\n\njavascript:alert(1) ftp://e.test",
        );
        assert!(html.contains("<code>https://a.test</code>"), "{html}");
        assert_eq!(html.matches("<a ").count(), 1, "{html}");
        assert!(html.contains("https://d.test\n</code></pre>"), "{html}");
        assert!(!html.contains("href=\"javascript"), "{html}");
        assert!(!html.contains("href=\"ftp"), "{html}");
    }
}
