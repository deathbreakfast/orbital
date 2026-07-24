/// Motion duration tokens aligned with Orbital `--orb-motion-duration-*` theme variables.
///
/// Values in [`ms`](MotionDuration::ms) match [`orbital-theme`](../../orbital-theme) defaults where the theme defines them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionDuration {
    UltraFast,
    Faster,
    Fast,
    Normal,
    Gentle,
    Slow,
    Slower,
    UltraSlow,
    /// Ambient / marketing fills (~4s). Prefer for Coming Soon loops, not snappy bars.
    Glacial,
    /// Long ambient approach fills (~10s). Prefer for Coming Soon one-shot holds.
    Epic,
}

impl MotionDuration {
    pub const ALL: [Self; 10] = [
        Self::UltraFast,
        Self::Faster,
        Self::Fast,
        Self::Normal,
        Self::Gentle,
        Self::Slow,
        Self::Slower,
        Self::UltraSlow,
        Self::Glacial,
        Self::Epic,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::UltraFast => 0,
            Self::Faster => 1,
            Self::Fast => 2,
            Self::Normal => 3,
            Self::Gentle => 4,
            Self::Slow => 5,
            Self::Slower => 6,
            Self::UltraSlow => 7,
            Self::Glacial => 8,
            Self::Epic => 9,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::UltraFast,
            1 => Self::Faster,
            2 => Self::Fast,
            3 => Self::Normal,
            4 => Self::Gentle,
            5 => Self::Slow,
            6 => Self::Slower,
            7 => Self::UltraSlow,
            8 => Self::Glacial,
            _ => Self::Epic,
        }
    }

    /// Returns a delay token offset by `steps` from `self` (for stagger choreography).
    pub const fn stagger_offset(self, steps: usize) -> Self {
        Self::from_index(self.index() + steps)
    }

    /// Numeric duration in milliseconds for this token.
    pub const fn ms_value(self) -> u32 {
        match self {
            Self::UltraFast => 50,
            Self::Faster => 100,
            Self::Fast => 150,
            Self::Normal => 200,
            Self::Gentle => 250,
            Self::Slow => 300,
            Self::Slower => 400,
            Self::UltraSlow => 500,
            Self::Glacial => 4000,
            Self::Epic => 10000,
        }
    }

    /// Map a millisecond value to the nearest duration token (rounded up).
    pub const fn from_ms_approx(ms: u32) -> Self {
        if ms <= 50 {
            Self::UltraFast
        } else if ms <= 100 {
            Self::Faster
        } else if ms <= 150 {
            Self::Fast
        } else if ms <= 200 {
            Self::Normal
        } else if ms <= 250 {
            Self::Gentle
        } else if ms <= 300 {
            Self::Slow
        } else if ms <= 400 {
            Self::Slower
        } else if ms <= 500 {
            Self::UltraSlow
        } else if ms <= 4000 {
            Self::Glacial
        } else {
            Self::Epic
        }
    }

    /// Stagger enter delay for list index `n` using `self` as the per-step spacing.
    pub const fn stagger_step_delay(self, index: usize) -> Self {
        Self::from_ms_approx(self.ms_value().saturating_mul(index as u32))
    }

    pub const fn css_var(self) -> &'static str {
        match self {
            Self::UltraFast => "var(--orb-motion-duration-2xs)",
            Self::Faster => "var(--orb-motion-duration-xs)",
            Self::Fast => "var(--orb-motion-duration-sm)",
            Self::Normal => "var(--orb-motion-duration-md)",
            Self::Gentle => "var(--orb-motion-duration-lg)",
            Self::Slow => "var(--orb-motion-duration-xl)",
            Self::Slower => "var(--orb-motion-duration-2xl)",
            Self::UltraSlow => "var(--orb-motion-duration-3xl)",
            Self::Glacial => "var(--orb-motion-duration-4xl)",
            Self::Epic => "var(--orb-motion-duration-5xl)",
        }
    }

    /// CSS variable name (without `var()`) for this duration token.
    pub const fn css_var_name(self) -> &'static str {
        match self {
            Self::UltraFast => "--orb-motion-duration-2xs",
            Self::Faster => "--orb-motion-duration-xs",
            Self::Fast => "--orb-motion-duration-sm",
            Self::Normal => "--orb-motion-duration-md",
            Self::Gentle => "--orb-motion-duration-lg",
            Self::Slow => "--orb-motion-duration-xl",
            Self::Slower => "--orb-motion-duration-2xl",
            Self::UltraSlow => "--orb-motion-duration-3xl",
            Self::Glacial => "--orb-motion-duration-4xl",
            Self::Epic => "--orb-motion-duration-5xl",
        }
    }

    /// Short human-readable scale label (2xs … 5xl).
    pub const fn scale_label(self) -> &'static str {
        match self {
            Self::UltraFast => "2xs",
            Self::Faster => "xs",
            Self::Fast => "sm",
            Self::Normal => "md",
            Self::Gentle => "lg",
            Self::Slow => "xl",
            Self::Slower => "2xl",
            Self::UltraSlow => "3xl",
            Self::Glacial => "4xl",
            Self::Epic => "5xl",
        }
    }

    pub const fn ms(self) -> &'static str {
        match self {
            Self::UltraFast => "50ms",
            Self::Faster => "100ms",
            Self::Fast => "150ms",
            Self::Normal => "200ms",
            Self::Gentle => "250ms",
            Self::Slow => "300ms",
            Self::Slower => "400ms",
            Self::UltraSlow => "500ms",
            Self::Glacial => "4000ms",
            Self::Epic => "10000ms",
        }
    }
}

impl std::fmt::Display for MotionDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.css_var())
    }
}

/// Motion easing curve tokens aligned with Orbital `--orb-motion-ease-*` theme variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionCurve {
    Linear,
    AccelerateMax,
    AccelerateMid,
    AccelerateMin,
    DecelerateMax,
    DecelerateMid,
    DecelerateMin,
    EasyEase,
    EasyEaseMax,
}

impl MotionCurve {
    pub const fn css_var(self) -> &'static str {
        match self {
            Self::Linear => "var(--orb-motion-ease-linear)",
            Self::AccelerateMax => "var(--orb-motion-ease-accelerate-strong)",
            Self::AccelerateMid => "var(--orb-motion-ease-accelerate)",
            Self::AccelerateMin => "var(--orb-motion-ease-accelerate-subtle)",
            Self::DecelerateMax => "var(--orb-motion-ease-decelerate-strong)",
            Self::DecelerateMid => "var(--orb-motion-ease-decelerate)",
            Self::DecelerateMin => "var(--orb-motion-ease-decelerate-subtle)",
            Self::EasyEase => "var(--orb-motion-ease-standard)",
            Self::EasyEaseMax => "var(--orb-motion-ease-emphasis)",
        }
    }

    /// CSS variable name (without `var()`) for this curve token.
    pub const fn css_var_name(self) -> &'static str {
        match self {
            Self::Linear => "--orb-motion-ease-linear",
            Self::AccelerateMax => "--orb-motion-ease-accelerate-strong",
            Self::AccelerateMid => "--orb-motion-ease-accelerate",
            Self::AccelerateMin => "--orb-motion-ease-accelerate-subtle",
            Self::DecelerateMax => "--orb-motion-ease-decelerate-strong",
            Self::DecelerateMid => "--orb-motion-ease-decelerate",
            Self::DecelerateMin => "--orb-motion-ease-decelerate-subtle",
            Self::EasyEase => "--orb-motion-ease-standard",
            Self::EasyEaseMax => "--orb-motion-ease-emphasis",
        }
    }

    /// Short human-readable label for demos and docs.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::AccelerateMax => "Accelerate (strong)",
            Self::AccelerateMid => "Accelerate",
            Self::AccelerateMin => "Accelerate (subtle)",
            Self::DecelerateMax => "Decelerate (strong)",
            Self::DecelerateMid => "Decelerate",
            Self::DecelerateMin => "Decelerate (subtle)",
            Self::EasyEase => "Standard ease",
            Self::EasyEaseMax => "Emphasis ease",
        }
    }

    pub const fn cubic_bezier(self) -> &'static str {
        match self {
            Self::Linear => "cubic-bezier(0, 0, 1, 1)",
            Self::AccelerateMax => "cubic-bezier(0.9, 0.1, 1, 0.2)",
            Self::AccelerateMid => "cubic-bezier(1, 0, 1, 1)",
            Self::AccelerateMin => "cubic-bezier(0.8, 0, 0.78, 1)",
            Self::DecelerateMax => "cubic-bezier(0.1, 0.9, 0.2, 1)",
            Self::DecelerateMid => "cubic-bezier(0, 0, 0, 1)",
            Self::DecelerateMin => "cubic-bezier(0.33, 0, 0.1, 1)",
            Self::EasyEase => "cubic-bezier(0.33, 0, 0.67, 1)",
            Self::EasyEaseMax => "cubic-bezier(0.8, 0, 0.2, 1)",
        }
    }
}

impl std::fmt::Display for MotionCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.css_var())
    }
}
