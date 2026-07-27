use leptos::{either::EitherOf3, prelude::*};
use orbital_macros::component_doc;
use orbital_style::inject_style;

const CODE_CSS: &str = include_str!("code.css");

/// Monospace code block for CLI commands, API identifiers, and short config samples.
///
/// Pass `text` for plain content. Pass `inner_html` only when you control the markup (for example pre-highlighted output from a trusted highlighter) — do not pass unsanitized user input to `inner_html`.
///
/// # When to use
///
/// - Single-line or multi-line code snippets in docs and settings panels - Inline monospace copy inside prose (wrap in a short `text` value)
///
/// # Examples
///
/// ## Code snippet
/// A plain code block with a distinct code surface color from the theme.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="code-preview">
///         <Code text="cargo leptos watch --split".to_string() />
///     </div>
/// }
/// ```
///
/// ## Multi-line block
/// Multi-line plain text for shell commands or config samples.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="code-multiline">
///         <Code text="cargo leptos build --split --release\ncargo leptos serve --split".to_string() />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Typography",
    preview_slug = "code",
    preview_label = "Code",
    preview_icon = icondata::AiCodeOutlined,
)]
#[component]
pub fn Code(
    /// Extra CSS class names merged onto the root `<code>` element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Plain-text snippet rendered inside `<pre>`.
    #[prop(optional, into)]
    text: Option<String>,
    /// Pre-rendered markup for syntax highlighting; takes precedence over `text`.
    #[prop(optional, into)]
    inner_html: Option<String>,
) -> impl IntoView {
    inject_style("orbital-code", CODE_CSS);

    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-code".to_string()
        } else {
            format!("orbital-code {extra}")
        }
    });

    view! {
        <code class=root_class>
            {if let Some(inner_html) = inner_html {
                EitherOf3::A(view! { <pre inner_html=inner_html></pre> })
            } else if let Some(text) = text {
                EitherOf3::B(view! { <pre>{text}</pre> })
            } else {
                EitherOf3::C(())
            }}
        </code>
    }
}
