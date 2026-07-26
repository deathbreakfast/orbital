//! Scrollport size measurement via ResizeObserver.

use leptos::html::Div;
use leptos::prelude::*;

/// Observe an element's content box height (px).
pub fn use_scrollport_height(node_ref: NodeRef<Div>, fallback: f64) -> ReadSignal<f64> {
    let height = RwSignal::new(fallback);

    #[cfg(feature = "hydrate")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let Some(element) = node_ref.get() else {
                return;
            };
            let el: web_sys::Element = element.into();
            if el.client_height() > 0 {
                height.set(el.client_height() as f64);
            }

            let height_signal = height;
            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                let entry = entries.get(0);
                if let Ok(obs_entry) = entry.dyn_into::<web_sys::ResizeObserverEntry>() {
                    let h = obs_entry.content_rect().height();
                    if h > 0.0 {
                        height_signal.set(h);
                    }
                }
            }) as Box<dyn FnMut(js_sys::Array)>);

            let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
            else {
                return;
            };
            observer.observe(&el);
            callback.forget();

            let observer = SendWrapper::new(observer);
            on_cleanup(move || {
                observer.disconnect();
            });
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = &node_ref;
    }

    height.read_only()
}
