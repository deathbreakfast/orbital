use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_markdown::{markdown_styles, render_to_html, OrbitalMarkdownOptions, RenderContext};
use orbital_style::inject_style;

use crate::data::image::image_styles;
use crate::data::table::table_styles;
use crate::feedback::message_bar::message_bar_styles;
use crate::forms::checkbox::checkbox_styles;
use crate::link_styles;

/// Inject every stylesheet rendered Markdown draws on: the prose sheet plus
/// the components whose markup it reuses (Link, Table, Checkbox, MessageBar,
/// Image), under the ids those components use themselves.
pub(crate) fn inject_markdown_styles() {
    inject_style("orbital-markdown", markdown_styles());
    inject_style("orbital-link", link_styles());
    inject_style("orbital-table", table_styles());
    inject_style("orbital-checkbox", checkbox_styles());
    inject_style("orbital-message-bar", message_bar_styles());
    inject_style("orbital-image", image_styles());
}

/// MarkdownView shows saved Markdown as formatted, sanitized text: headings,
/// lists and task lists, links (bare URLs too), code, images, quotes, GitHub
/// alerts (`> [!NOTE]`), collapsible `<details>`, rules and GFM tables.
///
/// Tables, task boxes, alerts, images and links come out as Orbital `Table`,
/// `Checkbox`, `MessageBar`, `Image` and `Link` markup, and headings follow the
/// `Title3` / `Subtitle1` / `Body1Strong` ramp, so a description reads like
/// the rest of the page.
///
/// It renders with `orbital-markdown`, the same renderer as the
/// [`MarkdownEditor`](crate::MarkdownEditor) Preview tab, so what people
/// preview is what they get. Raw HTML in the source is sanitized: scripts,
/// event handlers and unknown tags are dropped. Wide tables scroll sideways
/// on their own instead of widening the page.
///
/// # When to use
///
/// - Read views of descriptions, scopes, notes and other bodies stored as Markdown
///
/// # Best Practices
///
/// ## Do's
///
/// * Pair it with [`MarkdownEditor`](crate::MarkdownEditor) for the edit view of the same text
/// * Show your own empty state when the text is blank; the view renders nothing for it
///
/// ## Don'ts
///
/// * Do not set `inner_html` on a `div` with `render_to_html` output; tables, quotes and headings lose their styles
/// * Do not use for plain text that is never Markdown; use [`Text`](crate::Text)
///
/// # Examples
///
/// ## Default
/// Headings, a blockquote, a rule and a table with links, code and a centered column.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="markdown-view-preview">
///         <MarkdownView markdown="> **Product UI in Rust.**\n\n## Features\n\n| Feature | Crate | Status |\n|---|---|:---:|\n| [Tables](https://example.com) | `orbital-datatable` | Built |\n| Charts | `orbital-charts` | Built |\n| Scheduler | `orbital-scheduler` | Planned |\n\n---\n\n*Updated weekly.*" />
///     </div>
/// }
/// ```
///
/// ## Lists, alerts and details
/// Task lists, a bare link, a GitHub alert, a code block and a collapsible section.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="markdown-view-blocks">
///         <MarkdownView markdown="- [x] Write the scope\n- [ ] Review it\n\nDocs live at https://example.com/docs.\n\n> [!WARNING]\n> Deleting a project can't be undone.\n\n```\ncargo leptos watch\n```\n\n<details>\n<summary>Release notes</summary>\n\nShipped `MarkdownView`.\n\n</details>" />
///     </div>
/// }
/// ```
///
/// ## Wide table
/// A table wider than its container scrolls inside its own box.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="markdown-view-wide" style="max-width: 320px">
///         <MarkdownView markdown="| Name | Owner | Repository | Status | Updated |\n|---|---|---|---|---|\n| Valence | Platform foundations | unified-field-dev/valence | Built | 2026-11-16 |" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Typography",
    preview_slug = "markdown-view",
    preview_label = "Markdown View",
    preview_icon = icondata::AiFileMarkdownOutlined,
)]
#[component]
pub fn MarkdownView(
    /// Markdown source to render.
    #[prop(into)]
    markdown: Signal<String>,
    /// Markdown features; defaults to images on, with no citations or mentions.
    #[prop(optional)]
    render_options: Option<OrbitalMarkdownOptions>,
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// `data-testid` on the root element.
    #[prop(optional, into)]
    testid: MaybeProp<String>,
) -> impl IntoView {
    inject_markdown_styles();

    let render_options = StoredValue::new(render_options.unwrap_or(OrbitalMarkdownOptions {
        enable_images: true,
        ..Default::default()
    }));
    let html = Memo::new(move |_| {
        markdown.with(|text| {
            render_options
                .with_value(|options| render_to_html(text, options, &RenderContext::default()))
        })
    });
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-markdown {extra}"),
        _ => "orbital-markdown".to_string(),
    };

    view! {
        <div class=root_class data-testid=move || testid.get() inner_html=move || html.get() />
    }
}
