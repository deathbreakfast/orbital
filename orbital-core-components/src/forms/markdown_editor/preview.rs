//! The Preview tab of [`MarkdownEditor`](super::MarkdownEditor).

use leptos::prelude::*;
use orbital_markdown::{render_to_html, OrbitalMarkdownOptions, RenderContext};

/// The Preview tab: the rendered Markdown at the Write panel's last height,
/// so switching tabs doesn't move the page.
#[component]
pub(super) fn PreviewPanel(
    value: RwSignal<String>,
    options: OrbitalMarkdownOptions,
    height: RwSignal<f64>,
) -> impl IntoView {
    let options = StoredValue::new(options);
    let html = Memo::new(move |_| {
        value.with(|text| {
            options.with_value(|options| render_to_html(text, options, &RenderContext::default()))
        })
    });
    let style = move || {
        let height = height.get();
        (height > 0.0).then(|| format!("height: {height}px; min-height: 0"))
    };
    view! {
        <Show
            when=move || !html.with(String::is_empty)
            fallback=move || {
                view! {
                    <div class="orbital-markdown-editor__preview" style=style>
                        <span class="orbital-markdown-editor__empty">"Nothing to preview"</span>
                    </div>
                }
            }
        >
            <div
                class="orbital-markdown-editor__preview orbital-markdown"
                style=style
                inner_html=move || html.get()
            />
        </Show>
    }
}
