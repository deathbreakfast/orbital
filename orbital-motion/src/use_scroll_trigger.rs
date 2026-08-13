//! Hydrate scroll listener that drives a hide/show signal.

use leptos::html::Div;
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use crate::scroll_trigger::resolve_scroll_trigger;
use crate::scroll_trigger::ScrollTriggerOptions;

/// Listen for scroll on `target` (or the window when `target` is unset / empty).
///
/// Returns whether chrome should be **hidden** (tucked). Defaults to `false` on SSR
/// and when no DOM target is available yet.
///
/// Pass an explicit [`NodeRef`] for a page [`ScrollArea`] scrollport. Product shells
/// should resolve Layout page-scroll context in the caller and pass that ref here.
pub fn use_scroll_trigger(
    options: ScrollTriggerOptions,
    target: Option<NodeRef<Div>>,
) -> ReadSignal<bool> {
    let (hidden, set_hidden) = signal(false);

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = (options, target, set_hidden);
    }

    #[cfg(feature = "hydrate")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let last_y = StoredValue::new(0.0_f64);
        let detach = StoredValue::new(None::<SendWrapper<Box<dyn FnOnce()>>>);

        Effect::new(move |_| {
            detach.update_value(|slot| {
                if let Some(d) = slot.take() {
                    (d.take())();
                }
            });

            let element: Option<web_sys::Element> =
                target.and_then(|node_ref| node_ref.get()).map(|html| {
                    let el: web_sys::HtmlElement = html.into();
                    el.into()
                });

            let read_y = {
                let element = element.clone();
                move || -> f64 {
                    if let Some(el) = element.as_ref() {
                        el.scroll_top() as f64
                    } else {
                        leptos::prelude::window().scroll_y().unwrap_or(0.0)
                    }
                }
            };

            let initial = read_y();
            last_y.set_value(initial);
            set_hidden.set(false);

            let event_target: web_sys::EventTarget = match &element {
                Some(el) => el.clone().into(),
                None => leptos::prelude::window().into(),
            };

            let closure = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
                let y = read_y();
                let prev = last_y.get_value();
                let next = resolve_scroll_trigger(prev, y, hidden.get_untracked(), options);
                last_y.set_value(y);
                set_hidden.set(next);
            }) as Box<dyn FnMut(web_sys::Event)>);

            let _ = event_target
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());

            let event_target = SendWrapper::new(event_target);
            let closure = SendWrapper::new(closure);
            detach.set_value(Some(SendWrapper::new(Box::new(move || {
                let _ = event_target.remove_event_listener_with_callback(
                    "scroll",
                    closure.as_ref().unchecked_ref(),
                );
            }) as Box<dyn FnOnce()>)));
        });

        on_cleanup(move || {
            detach.update_value(|slot| {
                if let Some(d) = slot.take() {
                    (d.take())();
                }
            });
        });
    }

    hidden
}
