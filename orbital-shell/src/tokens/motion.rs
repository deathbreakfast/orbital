//! Motion duration, easing, and named presets.

/// Durations aligned with the Orbital motion scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionDuration {
    Ultrafast,
    Faster,
    Fast,
    Normal,
    Slow,
    Slower,
}

impl MotionDuration {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Ultrafast => "orbital-token-motion-ultrafast",
            Self::Faster => "orbital-token-motion-faster",
            Self::Fast => "orbital-token-motion-fast",
            Self::Normal => "orbital-token-motion-normal",
            Self::Slow => "orbital-token-motion-slow",
            Self::Slower => "orbital-token-motion-slower",
        }
    }

    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Ultrafast => "50ms",
            Self::Faster => "100ms",
            Self::Fast => "200ms",
            Self::Normal => "250ms",
            Self::Slow => "400ms",
            Self::Slower => "600ms",
        }
    }
}

/// Easing curves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionEasing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Decelerate,
    Accelerate,
}

impl MotionEasing {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Linear => "orbital-token-ease-linear",
            Self::EaseIn => "orbital-token-ease-in",
            Self::EaseOut => "orbital-token-ease-out",
            Self::EaseInOut => "orbital-token-ease-in-out",
            Self::Decelerate => "orbital-token-ease-decelerate",
            Self::Accelerate => "orbital-token-ease-accelerate",
        }
    }

    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::EaseIn => "ease-in",
            Self::EaseOut => "ease-out",
            Self::EaseInOut => "ease-in-out",
            Self::Decelerate => "cubic-bezier(0, 0, 0, 1)",
            Self::Accelerate => "cubic-bezier(1, 0, 1, 1)",
        }
    }
}

/// Named motion recipes (duration + easing) for marketing surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionPreset {
    /// Coachmarks, info-label popovers.
    EnterPopover,
    ExitPopover,
    /// Family card on intersection enter.
    RevealCard,
    /// Card hover lift (`translateY(-1px)`).
    HoverLift,
    /// Route transitions between marketing pages.
    TopLevelPageFade,
    /// Capability tab underline slide.
    TabIndicatorSlide,
    /// Sponsor progress bar fill on mount.
    ProgressFill,
}

impl MotionPreset {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::EnterPopover => "orbital-token-motion-enter-popover",
            Self::ExitPopover => "orbital-token-motion-exit-popover",
            Self::RevealCard => "orbital-token-motion-reveal-card",
            Self::HoverLift => "orbital-token-motion-hover-lift",
            Self::TopLevelPageFade => "orbital-token-motion-page-fade",
            Self::TabIndicatorSlide => "orbital-token-motion-tab-slide",
            Self::ProgressFill => "orbital-token-motion-progress-fill",
        }
    }

    /// CSS `transition` shorthand fragment (property left to caller).
    pub const fn transition_timing(self) -> &'static str {
        match self {
            Self::EnterPopover => "100ms ease-out",
            Self::ExitPopover => "100ms ease-in",
            Self::RevealCard => "250ms ease-out",
            Self::HoverLift => "50ms ease-out",
            Self::TopLevelPageFade => "200ms ease-in-out",
            Self::TabIndicatorSlide => "200ms ease-in-out",
            // Aligned with orbital-motion `DEFAULT_PROGRESS_FILL` (Slower + EasyEase).
            Self::ProgressFill => {
                "var(--orb-motion-duration-2xl, 400ms) var(--orb-motion-ease-standard, ease-in-out)"
            }
        }
    }
}
