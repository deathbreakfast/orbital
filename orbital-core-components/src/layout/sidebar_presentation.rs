//! How [`Layout`](super::Layout) presents the sidebar column.

/// Sidebar presentation mode for [`crate::Layout`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarPresentation {
    /// Always render an inline side column (historical default).
    #[default]
    Inline,
    /// Always present sidebar content in an [`crate::OverlayDrawer`].
    Overlay,
    /// Inline above [`super::sidebar_presentation::DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT`];
    /// overlay drawer below that breakpoint.
    Auto,
}

/// Default breakpoint for [`SidebarPresentation::Auto`] (below = overlay drawer).
pub const DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT: orbital_theme::Breakpoint =
    orbital_theme::Breakpoint::Md;
