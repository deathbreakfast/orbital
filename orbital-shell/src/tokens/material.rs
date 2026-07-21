//! Surface material (solid vs translucent).

/// Material treatment for marketing and shell surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Material {
    /// Default — highest readability in dark mode.
    Solid,
    /// Top bar, transient popovers, coachmarks.
    Frost,
    /// Reserved for signed-in shells (not default on public marketing).
    Shell,
    /// Blocking scrim behind dialogs / blocking coachmarks.
    Scrim,
}

impl Material {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Solid => "orbital-token-material-solid",
            Self::Frost => "orbital-token-material-frost",
            Self::Shell => "orbital-token-material-shell",
            Self::Scrim => "orbital-token-material-scrim",
        }
    }

    /// CSS `backdrop-filter` / opacity hints; callers combine with elevation.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Solid => "none",
            Self::Frost => "saturate(108%) blur(12px)",
            Self::Shell => "saturate(128%) blur(16px)",
            Self::Scrim => "none",
        }
    }
}
