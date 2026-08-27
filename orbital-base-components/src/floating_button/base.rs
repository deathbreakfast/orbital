use leptos::{ev, prelude::*};

use crate::Handler;

/// Headless floating action button — semantics and fixed positioning only.
#[component]
pub fn BaseFloatingButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(default = false.into(), into)] disabled: Signal<bool>,
    #[prop(optional)] on_click: Option<Handler<ev::MouseEvent>>,
    #[prop(optional, into)] testid: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let on_click = move |event: ev::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_click) = on_click.as_ref() {
            on_click.run(event);
        }
    };

    view! {
        <button
            type="button"
            class=move || class.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            aria-label=move || aria_label.get()
            disabled=move || disabled.get().then_some("")
            data-testid=move || testid.get()
            on:click=on_click
        >
            {children()}
        </button>
    }
}
