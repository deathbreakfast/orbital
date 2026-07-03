//! Browser scroll helpers for history entry anchors.

use leptos::html::Div;
use leptos::prelude::*;

/// Scroll an entry row into view. No-op when the id is not in the DOM.
#[cfg(feature = "hydrate")]
pub fn scroll_entry_into_view(entry_id: &str) {
    use wasm_bindgen::JsCast;

    let document = document();
    let selector = format!("[data-history-entry-id=\"{entry_id}\"]");
    let Ok(Some(element)) = document.query_selector(&selector) else {
        return;
    };
    let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() else {
        return;
    };
    element.scroll_into_view();
}

/// Schedule scroll after the next paint so newly mounted rows exist in the DOM.
#[cfg(feature = "hydrate")]
pub fn schedule_scroll_entry_into_view(entry_id: String) {
    request_animation_frame(move || {
        request_animation_frame(move || {
            scroll_entry_into_view(&entry_id);
        });
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn schedule_scroll_entry_into_view(_entry_id: String) {}

/// Scroll the timeline scrollport to the top.
#[cfg(feature = "hydrate")]
pub fn scroll_container_to_top(scroll_el: NodeRef<Div>) {
    if let Some(el) = scroll_el.get_untracked() {
        el.set_scroll_top(0);
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn scroll_container_to_top(_scroll_el: NodeRef<Div>) {}
