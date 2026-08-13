//! Page scrollport context for overlay [`Layout`](super::Layout) shells.

use leptos::html::Div;
use leptos::prelude::*;

/// Node ref to the Layout page [`ScrollArea`](crate::ScrollArea) when
/// `overlay_header` + default `page_scrollport` are active.
///
/// Consumers such as [`HideOnScroll`](crate::HideOnScroll) listen on this
/// element instead of `window`.
#[derive(Clone, Copy)]
pub struct LayoutPageScrollport(pub NodeRef<Div>);
