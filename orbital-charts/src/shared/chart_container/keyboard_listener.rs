//! Native keydown listener for hydrated chart roots.

use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use crate::context::apply_chart_keyboard_key;
use crate::context::ChartInteractionContext;
use crate::shared::chart_container::DrawingArea;

/// Attach a DOM keydown listener when keyboard navigation is enabled.
#[cfg(feature = "hydrate")]
pub fn use_chart_keyboard_listener(
    root_ref: NodeRef<leptos::html::Div>,
    keyboard_nav: bool,
    interaction: ChartInteractionContext,
    area: DrawingArea,
) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        if !keyboard_nav {
            return;
        }
        let Some(element) = root_ref.get() else {
            return;
        };
        let el: web_sys::HtmlElement = element.into();

        let callback = Closure::wrap(Box::new(move |ev: web_sys::Event| {
            let ev: web_sys::KeyboardEvent = ev.dyn_into().unwrap();
            let key = ev.key();
            if key == "Escape" {
                interaction.pointer_plot.set(None);
                interaction.axis_data_index.set(None);
                if let Some(cb) = interaction.on_highlight_change.as_ref() {
                    cb.run((None,));
                }
                interaction.hovered_item.set(None);
                interaction.highlighted_item.set(None);
                interaction.keyboard_focus_item.set(None);
                return;
            }
            let active = interaction.hovered_item.get_untracked();
            if apply_chart_keyboard_key(&key, interaction, area, active) {
                ev.prevent_default();
            }
        }) as Box<dyn FnMut(web_sys::Event)>);

        let _ = el.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());
        let el = SendWrapper::new(el);
        let handler = SendWrapper::new(callback);
        on_cleanup(move || {
            let _ =
                el.remove_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        });
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn use_chart_keyboard_listener(
    _root_ref: NodeRef<leptos::html::Div>,
    _keyboard_nav: bool,
    _interaction: ChartInteractionContext,
    _area: DrawingArea,
) {
}
