use std::time::Duration;

use leptos::{leptos_dom::helpers::set_timeout, prelude::*, task::spawn_local};
use orbital_macros::component_doc;

use crate::{Button, ButtonAppearance, ButtonSize};

/// How long the "Copied" confirmation stays before the button reverts.
const COPIED_RESET: Duration = Duration::from_millis(2000);

/// Writes the browser clipboard; returns `false` when the write is refused or unavailable.
#[cfg(target_arch = "wasm32")]
async fn write_clipboard(text: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    wasm_bindgen_futures::JsFuture::from(window.navigator().clipboard().write_text(text))
        .await
        .is_ok()
}

#[cfg(not(target_arch = "wasm32"))]
async fn write_clipboard(_text: &str) -> bool {
    false
}

/// Copies a string to the clipboard and confirms it in place — share links, API keys, and command snippets.
///
/// The label switches to "Copied" with a check icon for two seconds, announced through a polite live region. A refused write (permissions, insecure context) leaves the label unchanged and calls `on_error`.
///
/// # When to use
///
/// - Share URLs, tokens, and identifiers the user will paste elsewhere - Next to a [`Code`](crate::Code) block holding a command
///
/// # Examples
///
/// ## Copy button
/// Copies the given text and shows a short confirmation.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="copy-button-preview">
///         <CopyButton text="https://example.com/share/abc123" />
///     </div>
/// }
/// ```
///
/// ## Custom label
/// Override the idle label to match the surrounding copy.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="copy-button-label">
///         <CopyButton text="cargo leptos watch" label="Copy command" appearance=ButtonAppearance::Subtle />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "copy-button",
    preview_label = "Copy button",
    preview_icon = icondata::AiCopyOutlined,
)]
#[component]
pub fn CopyButton(
    /// Text written to the clipboard on click.
    #[prop(into)]
    text: Signal<String>,
    /// Idle label.
    #[prop(optional, into, default = "Copy".into())]
    label: Signal<String>,
    /// Label shown briefly after a successful copy.
    #[prop(optional, into, default = "Copied".into())]
    copied_label: Signal<String>,
    /// Visual emphasis forwarded to the underlying [`Button`].
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// Size forwarded to the underlying [`Button`].
    #[prop(optional, into)]
    size: Signal<ButtonSize>,
    /// Extra CSS class names merged onto the button.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Disables the button.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Called after the clipboard write succeeds.
    #[prop(optional)]
    on_copy: Option<Callback<()>>,
    /// Called when the clipboard write fails.
    #[prop(optional)]
    on_error: Option<Callback<()>>,
) -> impl IntoView {
    let copied = RwSignal::new(false);

    let on_click = Callback::new(move |_| {
        let value = text.get_untracked();
        spawn_local(async move {
            if write_clipboard(&value).await {
                copied.set(true);
                set_timeout(move || copied.set(false), COPIED_RESET);
                if let Some(cb) = on_copy {
                    cb.run(());
                }
            } else if let Some(cb) = on_error {
                cb.run(());
            }
        });
    });

    let icon = Signal::derive(move || {
        Some(if copied.get() {
            icondata::AiCheckOutlined
        } else {
            icondata::AiCopyOutlined
        })
    });

    view! {
        <span role="status" aria-live="polite">
            <Button
                class=class
                appearance=appearance
                size=size
                disabled=disabled
                icon=icon
                on_click=on_click
            >
                {move || if copied.get() { copied_label.get() } else { label.get() }}
            </Button>
        </span>
    }
}
