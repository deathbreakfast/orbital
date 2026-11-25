//! The height grip in the bottom-right corner of the Write panel.

use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;

use super::height::HeightControl;

/// A focusable separator that changes the writing area's height, by pointer
/// or keyboard, plus the chip that shows the height in lines while dragging.
#[component]
pub(super) fn HeightGrip(
    height: HeightControl,
    /// The least height, in lines; also `aria-valuemin`.
    rows: u32,
    disabled: Signal<bool>,
) -> impl IntoView {
    let dragging = RwSignal::new(false);
    // Pointer y and textarea height when the drag started.
    let drag_start = StoredValue::new(None::<(f64, f64)>);

    let on_pointerdown = move |ev: ev::PointerEvent| {
        if disabled.get_untracked() || ev.button() != 0 {
            return;
        }
        ev.prevent_default();
        if let Some(grip) = ev
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            _ = grip.set_pointer_capture(ev.pointer_id());
            _ = grip.focus();
        }
        drag_start.set_value(Some((f64::from(ev.client_y()), height.current())));
        dragging.set(true);
    };
    let on_pointermove = move |ev: ev::PointerEvent| {
        if let Some((start_y, start_height)) = drag_start.get_value() {
            height.resize_to(start_height + f64::from(ev.client_y()) - start_y);
        }
    };
    let end_drag = move || {
        drag_start.set_value(None);
        dragging.set(false);
    };
    let on_keydown = move |ev: ev::KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(m) = height.read_metrics() else {
            return;
        };
        let step = if ev.shift_key() { 5.0 } else { 1.0 } * m.line;
        match ev.key().as_str() {
            "ArrowDown" => height.resize_to(height.current() + step),
            "ArrowUp" => height.resize_to(height.current() - step),
            "Home" => height.resize_to(m.min),
            "End" => height.resize_to(m.max),
            "Enter" => height.fit_text(),
            _ => return,
        }
        ev.prevent_default();
        ev.stop_propagation();
    };

    view! {
        <div
            class="orbital-markdown-editor__grip"
            role="separator"
            aria-orientation="horizontal"
            aria-label="Resize editor height"
            tabindex=move || if disabled.get() { "-1" } else { "0" }
            aria-disabled=move || disabled.get().then_some("true")
            aria-valuemin=rows.to_string()
            aria-valuemax=move || height.metrics().map(|m| m.lines(m.max).to_string())
            aria-valuenow=move || height.lines().map(|lines| lines.to_string())
            aria-valuetext=move || height.lines().map(|lines| format!("{lines} lines"))
            on:pointerdown=on_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=move |_| end_drag()
            on:pointercancel=move |_| end_drag()
            on:keydown=on_keydown
            on:dblclick=move |_| {
                if !disabled.get_untracked() {
                    height.fit_text();
                }
            }
        ></div>
        <Show when=move || dragging.get()>
            <span class="orbital-markdown-editor__grip-chip" aria-hidden="true">
                {move || height.lines().map(|lines| format!("{lines} lines"))}
            </span>
        </Show>
    }
}
