//! `matchMedia` hook — SSR-safe (`false` until the client can evaluate).

use leptos::prelude::*;

/// Subscribe to a CSS media query.
///
/// On SSR / non-wasm targets the signal stays `false` until hydration can run
/// `window.matchMedia`. Pass a full query such as `"(min-width: 1024px)"`.
pub fn use_media_query(query: impl Into<String>) -> ReadSignal<bool> {
    let query = query.into();
    let (matches, set_matches) = signal(false);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (query, set_matches);
    }

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::web_sys::window;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let Some(win) = window() else {
                return;
            };
            let Ok(Some(mql)) = win.match_media(&query) else {
                return;
            };
            set_matches.set(mql.matches());

            let set_matches = set_matches;
            let mql_for_listener = mql.clone();
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                set_matches.set(mql_for_listener.matches());
            }) as Box<dyn FnMut(web_sys::Event)>);

            let _ = mql.add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
            // Keep the listener for the component lifetime (same pattern as reduced motion).
            closure.forget();
        });
    }

    matches
}
