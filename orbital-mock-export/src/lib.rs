//! Turns SSR-rendered Orbital previews into static stylesheets for HTML mocks.
//!
//! Orbital ships component CSS as Rust strings that reach the page through
//! `inject_style` during SSR. HTML mocks have no SSR, so this crate renders every
//! preview registration once, pulls the `<style>` blocks out of the output, and
//! writes them as plain files:
//!
//! - `orbital.css`: component styles, deduplicated, in first-seen order.
//! - `tokens.css`: `--orb-*` variables for the light and dark themes.
//!
//! The pure pieces live here so they can be tested without rendering.

use std::collections::HashSet;

/// One `<style>` block found in rendered HTML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleBlock {
    /// The `id` attribute, when the block had one (`orbital-style-*` for injected styles).
    pub id: Option<String>,
    /// The CSS text between the tags.
    pub css: String,
}

/// Returns every `<style>` block in `html`, in document order.
pub fn extract_style_blocks(html: &str) -> Vec<StyleBlock> {
    let mut blocks = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("<style") {
        let after_tag = &rest[start + "<style".len()..];
        // `<style>` or `<style ...>`; skip lookalikes such as `<styled-thing>`.
        if !after_tag.starts_with('>') && !after_tag.starts_with(char::is_whitespace) {
            rest = after_tag;
            continue;
        }
        let Some(open_end) = after_tag.find('>') else {
            break;
        };
        let attrs = &after_tag[..open_end];
        let body = &after_tag[open_end + 1..];
        let Some(close) = body.find("</style>") else {
            break;
        };
        blocks.push(StyleBlock {
            id: attr_value(attrs, "id"),
            css: body[..close].to_string(),
        });
        rest = &body[close + "</style>".len()..];
    }
    blocks
}

fn attr_value(attrs: &str, name: &str) -> Option<String> {
    let needle = format!("{name}=\"");
    let start = attrs.find(&needle)? + needle.len();
    let len = attrs[start..].find('"')?;
    Some(attrs[start..start + len].to_string())
}

/// Style ids that carry theme variables or font faces. `tokens.css` covers those,
/// so they stay out of `orbital.css`.
pub fn is_theme_block(block: &StyleBlock) -> bool {
    match block.id.as_deref() {
        Some(id) => {
            id.starts_with("orbital-style-orbital-theme-")
                || id == "orbital-style-orbital-font-faces"
        }
        None => false,
    }
}

/// Collects component CSS across many rendered pages.
///
/// Blocks are keyed by id when present and by content otherwise, so a component
/// rendered on fifty preview pages contributes its stylesheet once.
#[derive(Debug, Default)]
pub struct StyleCollector {
    seen: HashSet<String>,
    blocks: Vec<StyleBlock>,
}

impl StyleCollector {
    /// Adds the component style blocks from one rendered page.
    pub fn add_html(&mut self, html: &str) {
        for block in extract_style_blocks(html) {
            if is_theme_block(&block) || block.css.trim().is_empty() {
                continue;
            }
            let key = match &block.id {
                Some(id) => format!("id:{id}"),
                None => format!("css:{}", block.css),
            };
            if self.seen.insert(key) {
                self.blocks.push(block);
            }
        }
    }

    /// Number of distinct blocks collected so far.
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Whether nothing has been collected.
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Writes the collected blocks as one stylesheet, each under a comment naming its source.
    pub fn to_css(&self, header: &str) -> String {
        let mut out = String::from(header);
        for block in &self.blocks {
            let label = block.id.as_deref().unwrap_or("inline");
            out.push_str(&format!("\n/* {label} */\n{}\n", block.css.trim()));
        }
        out
    }
}

/// Strips the selector from `theme_scoped_vars_css` output, leaving the declarations.
///
/// Input looks like `.orbital-theme-provider[data-orbital-theme-id="0"] {--a: 1;--b: 2;}`.
pub fn declarations_of(scoped_css: &str) -> &str {
    let open = scoped_css.find('{').map_or(0, |i| i + 1);
    let close = scoped_css.rfind('}').unwrap_or(scoped_css.len());
    scoped_css[open..close].trim()
}

/// Builds `tokens.css`: fonts, light variables on the root and every theme provider,
/// and dark variables behind both the OS preference and an explicit `data-theme`.
pub fn tokens_css(header: &str, font_faces: &str, light_decls: &str, dark_decls: &str) -> String {
    format!(
        "{header}\n{font_faces}\n\
:root, .orbital-theme-provider, [data-theme=\"light\"] {{{light_decls}}}\n\
@media (prefers-color-scheme: dark) {{\n  :root:not([data-theme=\"light\"]), :root:not([data-theme=\"light\"]) .orbital-theme-provider {{{dark_decls}}}\n}}\n\
:root[data-theme=\"dark\"], :root[data-theme=\"dark\"] .orbital-theme-provider, [data-theme=\"dark\"] {{{dark_decls}}}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_blocks_with_and_without_ids() {
        let html = r#"<head><style id="orbital-style-orbital-button">.orbital-button{color:red}</style></head>
<body><styled-box></styled-box><style>.Wrapper-a1-b2{padding:4px}</style></body>"#;
        let blocks = extract_style_blocks(html);
        assert_eq!(
            blocks,
            vec![
                StyleBlock {
                    id: Some("orbital-style-orbital-button".into()),
                    css: ".orbital-button{color:red}".into(),
                },
                StyleBlock {
                    id: None,
                    css: ".Wrapper-a1-b2{padding:4px}".into(),
                },
            ]
        );
    }

    #[test]
    fn unterminated_style_stops_without_panicking() {
        assert!(extract_style_blocks("<style>.a{}").is_empty());
        assert!(extract_style_blocks("<style").is_empty());
    }

    #[test]
    fn collector_dedupes_and_drops_theme_blocks() {
        let page = r#"<style id="orbital-style-orbital-theme-0">.orbital-theme-provider{--x:1}</style>
<style id="orbital-style-orbital-font-faces">@font-face{}</style>
<style id="orbital-style-orbital-card">.orbital-card{}</style>
<style>.Inline-1{}</style><style>   </style>"#;
        let mut collector = StyleCollector::default();
        collector.add_html(page);
        collector.add_html(page);
        assert_eq!(collector.len(), 2);
        let css = collector.to_css("/* h */");
        assert!(css.contains(".orbital-card{}"));
        assert!(css.contains(".Inline-1{}"));
        assert!(!css.contains("--x:1"));
        assert!(!css.contains("@font-face"));
    }

    #[test]
    fn declarations_strip_the_scoped_selector() {
        let scoped = r#".orbital-theme-provider[data-orbital-theme-id="0"] {--a: 1;--b: 2;}"#;
        assert_eq!(declarations_of(scoped), "--a: 1;--b: 2;");
    }

    #[test]
    fn tokens_cover_light_dark_and_explicit_theme() {
        let css = tokens_css("/* h */", "@font-face{}", "--c:#fff;", "--c:#000;");
        assert!(css.contains(":root, .orbital-theme-provider, [data-theme=\"light\"] {--c:#fff;}"));
        assert!(css.contains("prefers-color-scheme: dark"));
        assert!(css.contains(":root[data-theme=\"dark\"]"));
        assert_eq!(css.matches("--c:#000;").count(), 2);
    }
}
