mod content_with_aside;
mod content_with_aside_slots;
mod context;
mod header_inset;
mod layout;
mod main;
mod overlay;
mod sidebar;
mod sidebar_presentation;
mod sidebar_toggle;
mod slots;
mod styles;

pub use content_with_aside::ContentWithAside;
#[cfg(feature = "preview")]
pub use content_with_aside::CONTENTWITHASIDE_PREVIEW_REGISTRATION;
pub use content_with_aside_slots::{Aside, Content};
pub use context::LayoutSidebarOpen;
pub use header_inset::LayoutHeaderInset;
pub use layout::Layout;
#[cfg(feature = "preview")]
pub use layout::LAYOUT_PREVIEW_REGISTRATION;
pub use sidebar_presentation::{SidebarPresentation, DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT};
pub use sidebar_toggle::LayoutSidebarToggle;
pub use slots::{LayoutHeader, LayoutMain, LayoutSidebar};

pub use orbital_base_components::{AppBarInset, LayoutPosition};
pub use styles::layout_styles;
