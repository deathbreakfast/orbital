//! Types for [`super::AdaptiveMenu`].

use leptos::prelude::*;
use orbital_theme::Breakpoint;

/// Default breakpoint for adaptive presentation (below = drawer).
pub const DEFAULT_ADAPTIVE_MENU_BREAKPOINT: Breakpoint = Breakpoint::Md;

/// Trigger slot for [`super::AdaptiveMenu`].
#[slot]
pub struct AdaptiveMenuTrigger {
    /// Trigger control — typically a [`crate::Button`].
    pub children: ChildrenFn,
}
