use leptos::prelude::*;
use orbital_base_components::BaseDialogSurface;

use super::types::DialogSize;

/// Animated modal panel (`role="dialog"`, `aria-modal="true"`).
///
/// Required child of [`Dialog`](super::dialog::Dialog). Wraps [`DialogBody`] and its title/content/actions slots.
#[component]
pub fn DialogSurface(
    /// Optional CSS class on the surface panel.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Panel width. Use `Small` for confirmations, `Medium` (default) for short forms, `Large` for forms with two columns, and `Full` for editors that need the screen.
    #[prop(optional)]
    size: DialogSize,
    /// Panel contents — typically [`DialogBody`].
    children: Children,
) -> impl IntoView {
    let size_class = format!("orbital-dialog-surface--{}", size.class_suffix());
    let class = Signal::derive(move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("{size_class} {extra}"),
        _ => size_class.clone(),
    });
    view! {
        <BaseDialogSurface class=class>
            {children()}
        </BaseDialogSurface>
    }
}
