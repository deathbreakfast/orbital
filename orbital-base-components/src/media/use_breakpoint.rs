//! Breakpoint convenience hooks built on [`super::use_media_query`].

use leptos::prelude::*;
use orbital_theme::Breakpoint;

use super::use_media_query::use_media_query;

/// `true` when the viewport width is at least the breakpoint minimum.
pub fn use_breakpoint_up(breakpoint: Breakpoint) -> ReadSignal<bool> {
    use_media_query(breakpoint.up_query())
}

/// `true` when the viewport width is below the breakpoint minimum.
pub fn use_breakpoint_down(breakpoint: Breakpoint) -> ReadSignal<bool> {
    use_media_query(breakpoint.down_query())
}

/// Active breakpoint tier for the current viewport (largest matching `min-width`).
///
/// Returns `None` when below [`Breakpoint::Sm`] (or on SSR until hydrate).
pub fn use_breakpoint() -> Signal<Option<Breakpoint>> {
    let sm = use_breakpoint_up(Breakpoint::Sm);
    let md = use_breakpoint_up(Breakpoint::Md);
    let lg = use_breakpoint_up(Breakpoint::Lg);
    let xl = use_breakpoint_up(Breakpoint::Xl);
    Signal::derive(move || {
        if xl.get() {
            Some(Breakpoint::Xl)
        } else if lg.get() {
            Some(Breakpoint::Lg)
        } else if md.get() {
            Some(Breakpoint::Md)
        } else if sm.get() {
            Some(Breakpoint::Sm)
        } else {
            None
        }
    })
}
