//! Navigation-oriented hooks shared by Orbital shell components.
//!
//! These helpers are intentionally small and router-focused. They let UI components
//! such as side navigation or tabs reactively determine whether a
//! route should be considered active.
//!
//! ## Example
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use orbital::nav::use_route_active;
//!
//! #[component]
//! fn CounterNavLink() -> impl IntoView {
//!     let is_active = use_route_active("/counter", false);
//!
//!     view! {
//!         <a class:active=move || is_active.get() href="/counter">
//!             "Counter"
//!         </a>
//!     }
//! }
//! ```

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_router::NavigateOptions;

/// Reactive hook that returns whether the given path matches the current location.
///
/// When `exact` is true, only an exact match counts. When `exact` is false (default), any path that starts with the given prefix matches.
///
/// The hook normalises the current pathname by stripping a trailing slash so that `/counter` and `/counter/` are treated identically.
///
/// This is primarily intended for navigation items that should stay highlighted while the user browses nested routes under the same section.
pub fn use_route_active(path: &str, exact: bool) -> Memo<bool> {
    let location = use_location();
    let path = path.to_string();
    Memo::new(move |_| {
        let raw = location.pathname.get();
        let normalized = if raw.len() > 1 {
            raw.trim_end_matches('/').to_string()
        } else {
            raw
        };
        if exact {
            normalized == path
        } else {
            normalized.starts_with(&path)
        }
    })
}

/// Navigate back in the browser history when possible; otherwise go to `fallback`.
///
/// Prefers `history.back()` whenever the session has a prior entry (`length > 1`),
/// including cross-origin referrers (return to wherever the user came from).
/// On SSR or when history is unavailable / empty, calls `navigate(fallback, …)`.
pub fn navigate_back_or(fallback: &str, navigate: &impl Fn(&str, NavigateOptions)) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                if history.length().unwrap_or(0) > 1 {
                    let _ = history.back();
                    return;
                }
            }
            // Same-tab navigations sometimes report length == 1 while still having
            // a document referrer (e.g. opened from an external link that replaced).
            let referrer = window.document().map(|d| d.referrer()).unwrap_or_default();
            if !referrer.is_empty() {
                if let Ok(history) = window.history() {
                    let _ = history.back();
                    return;
                }
            }
        }
    }
    navigate(fallback, NavigateOptions::default());
}
