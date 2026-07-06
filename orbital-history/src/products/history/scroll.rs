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

/// Whether an entry with `entry_id` is currently in the DOM.
#[cfg(feature = "hydrate")]
pub fn entry_in_dom(entry_id: &str) -> bool {
    let document = document();
    let selector = format!("[data-history-entry-id=\"{entry_id}\"]");
    document
        .query_selector(&selector)
        .ok()
        .flatten()
        .is_some()
}

#[cfg(not(feature = "hydrate"))]
pub fn entry_in_dom(_entry_id: &str) -> bool {
    false
}

/// Track scroll offset on the timeline scrollport (for virtualization).
#[cfg(feature = "hydrate")]
pub fn attach_scroll_top_listener(scroll_el: NodeRef<Div>, scroll_top: RwSignal<f64>) {
    use leptos::ev;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let Some(el) = scroll_el.get() else {
            return;
        };
        let scroll_top = scroll_top;
        let listener = Closure::<dyn Fn(ev::Event)>::new(move |_ev: ev::Event| {
            scroll_top.set(el.scroll_top() as f64);
        });
        el.add_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
            .ok();
        on_cleanup(move || {
            el.remove_event_listener_with_callback("scroll", listener.as_ref().unchecked_ref())
                .ok();
        });
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn attach_scroll_top_listener(_scroll_el: NodeRef<Div>, _scroll_top: RwSignal<f64>) {}
