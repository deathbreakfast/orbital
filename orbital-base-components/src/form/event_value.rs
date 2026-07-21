//! Non-panicking helpers for reading values from DOM form events.
//!
//! Prefer these over [`leptos::prelude::event_target_value`] /
//! [`leptos::prelude::event_target`], which can panic when the event has no
//! target or the target is not a value-bearing control.

use wasm_bindgen::JsCast;

/// Read `.value` from an `<input>` event target.
pub fn input_event_value(ev: &web_sys::Event) -> Option<String> {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value())
}

/// Read `.value` from a `<textarea>` event target.
pub fn textarea_event_value(ev: &web_sys::Event) -> Option<String> {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
        .map(|el| el.value())
}

/// Cast an event's target to [`web_sys::HtmlElement`] when present.
pub fn event_html_element(ev: &web_sys::Event) -> Option<web_sys::HtmlElement> {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
}
