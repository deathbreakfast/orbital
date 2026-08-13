//! Adaptive menu — popover/menu on wide viewports, overlay drawer on narrow.

mod adaptive_menu;
mod styles;
mod types;

pub use adaptive_menu::AdaptiveMenu;
pub use types::{AdaptiveMenuTrigger, DEFAULT_ADAPTIVE_MENU_BREAKPOINT};

#[cfg(feature = "preview")]
pub use adaptive_menu::ADAPTIVEMENU_PREVIEW_REGISTRATION;
