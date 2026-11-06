//! Table width measurement via ResizeObserver.

use leptos::{html::Div, prelude::*};

/// Observe an element's content width and publish it once it is non-zero.
///
/// Stays `None` on the server and until the first measurement, so layout falls
/// back to its default table width until then.
pub fn use_table_width(node_ref: NodeRef<Div>) -> Signal<Option<f64>> {
    let width = RwSignal::new(None::<f64>);

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
            if el.client_width() > 0 {
                width.set(Some(f64::from(el.client_width())));
            }

            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                let entry = entries.get(0);
                if let Ok(obs_entry) = entry.dyn_into::<web_sys::ResizeObserverEntry>() {
                    let w = obs_entry.content_rect().width();
                    if w > 0.0 && width.get_untracked() != Some(w) {
                        width.set(Some(w));
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
            on_cleanup(move || observer.disconnect());
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = &node_ref;
    }

    width.into()
}
