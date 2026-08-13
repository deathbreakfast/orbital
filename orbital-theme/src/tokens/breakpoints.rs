//! Viewport breakpoint tokens for responsive composition.

/// Named viewport width thresholds used by Orbital responsive hooks and CSS vars.
///
/// Values are **minimum** widths in CSS pixels (`min-width`). Use
/// [`Breakpoint::down_max_px`] for the matching `max-width` query boundary
/// (one pixel below this tier's minimum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Breakpoint {
    /// 640px and up.
    Sm,
    /// 1024px and up (aligns with `ContentWithAside` stacking below 1024).
    Md,
    /// 1280px and up.
    Lg,
    /// 1536px and up.
    Xl,
}

impl Breakpoint {
    /// Minimum width in CSS pixels for this breakpoint (`min-width`).
    pub const fn min_width_px(self) -> u32 {
        match self {
            Self::Sm => 640,
            Self::Md => 1024,
            Self::Lg => 1280,
            Self::Xl => 1536,
        }
    }

    /// Exclusive upper bound for "below this breakpoint" (`max-width: Npx`).
    pub const fn down_max_px(self) -> u32 {
        self.min_width_px().saturating_sub(1)
    }

    /// CSS custom property name for this breakpoint minimum (e.g. `--orb-breakpoint-md`).
    pub const fn css_var(self) -> &'static str {
        match self {
            Self::Sm => "--orb-breakpoint-sm",
            Self::Md => "--orb-breakpoint-md",
            Self::Lg => "--orb-breakpoint-lg",
            Self::Xl => "--orb-breakpoint-xl",
        }
    }

    /// All breakpoints in ascending order.
    pub const fn all() -> [Self; 4] {
        [Self::Sm, Self::Md, Self::Lg, Self::Xl]
    }

    /// `min-width` media query string for this breakpoint.
    pub fn up_query(self) -> String {
        format!("(min-width: {}px)", self.min_width_px())
    }

    /// `max-width` media query string for viewports below this breakpoint.
    pub fn down_query(self) -> String {
        format!("(max-width: {}px)", self.down_max_px())
    }
}

/// Appends `--orb-breakpoint-*` CSS custom properties.
pub fn write_orb_breakpoint_css_vars(css_vars: &mut String) {
    for bp in Breakpoint::all() {
        css_vars.push_str(&format!("{}: {}px;", bp.css_var(), bp.min_width_px()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breakpoint_tokens_emit() {
        let mut css = String::new();
        write_orb_breakpoint_css_vars(&mut css);
        assert!(css.contains("--orb-breakpoint-sm: 640px;"));
        assert!(css.contains("--orb-breakpoint-md: 1024px;"));
        assert!(css.contains("--orb-breakpoint-lg: 1280px;"));
        assert!(css.contains("--orb-breakpoint-xl: 1536px;"));
    }

    #[test]
    fn down_max_is_one_below_min() {
        assert_eq!(Breakpoint::Md.down_max_px(), 1023);
        assert_eq!(Breakpoint::Sm.down_query(), "(max-width: 639px)");
        assert_eq!(Breakpoint::Md.up_query(), "(min-width: 1024px)");
    }
}
