//! Brand tone tokens for marketing and accent surfaces.

/// Cross-page accent selection mapped to Orbital brand CSS variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrandTone {
    /// Global marketing CTA / hero accent.
    Brand,
    Neutral,
    Subtle,
    Accent,
}

impl BrandTone {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Brand => "orbital-token-tone-brand",
            Self::Neutral => "orbital-token-tone-neutral",
            Self::Subtle => "orbital-token-tone-subtle",
            Self::Accent => "orbital-token-tone-accent",
        }
    }

    /// Primary CSS token for accent (background or glow source).
    pub const fn accent_token(self) -> &'static str {
        match self {
            Self::Brand => "var(--orb-color-brand-bg)",
            Self::Neutral => "var(--orb-color-surface-subtle)",
            Self::Subtle => "var(--orb-color-surface-overlay)",
            Self::Accent => "var(--orb-color-brand-bg-subtle)",
        }
    }
}
