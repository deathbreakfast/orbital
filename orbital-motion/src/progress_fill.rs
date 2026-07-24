//! Continuous progress / width fill motion (property transitions, not presence).
//!
//! Use for bars that animate toward a known progress value. Enter/exit still belongs
//! to [`PresenceMotion`](crate::PresenceMotion).

use orbital_style::inject_style;

use crate::tokens::{MotionCurve, MotionDuration};

/// CSS class applied to elements whose `width` (or `transform: scaleX`) should follow
/// progress-fill timing tokens.
pub const PROGRESS_FILL_CLASS: &str = "orbital-motion-progress-fill";

/// CSS class that also respects `prefers-reduced-motion` (near-instant transitions).
pub const PROGRESS_FILL_RESPECTS_REDUCED_CLASS: &str = "orbital-motion-progress-fill-respects-reduced";

/// Default recipe for snappy measurable progress (ProgressBar-like).
pub const DEFAULT_PROGRESS_FILL: ProgressFillMotion = ProgressFillMotion {
    duration: MotionDuration::Slower,
    curve: MotionCurve::EasyEase,
};

/// Default recipe for slow ambient loops (Coming Soon, marketing fills).
pub const GLACIAL_PROGRESS_FILL: ProgressFillMotion = ProgressFillMotion {
    duration: MotionDuration::Glacial,
    curve: MotionCurve::DecelerateMid,
};

/// Default recipe for Coming Soon one-shot fills (~10s to `fill_to`).
pub const EPIC_PROGRESS_FILL: ProgressFillMotion = ProgressFillMotion {
    duration: MotionDuration::Epic,
    curve: MotionCurve::DecelerateMid,
};

/// Duration + curve for a continuous progress fill transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProgressFillMotion {
    /// How long the fill takes to reach the target width.
    pub duration: MotionDuration,
    /// Easing applied to the width / scale transition.
    pub curve: MotionCurve,
}

impl ProgressFillMotion {
    /// Build a recipe from duration and curve tokens.
    #[must_use]
    pub const fn new(duration: MotionDuration, curve: MotionCurve) -> Self {
        Self { duration, curve }
    }

    /// CSS `transition` shorthand for `width` (property included).
    ///
    /// Duration / curve use theme vars with literal fallbacks so fills still
    /// animate when tokens are not yet on the theme provider.
    #[must_use]
    pub fn width_transition(self) -> String {
        format!(
            "width var({}, {}) var({}, {})",
            self.duration.css_var_name(),
            self.duration.ms(),
            self.curve.css_var_name(),
            self.curve.cubic_bezier(),
        )
    }

    /// CSS `transition` shorthand for `transform` (scaleX fills).
    #[must_use]
    pub fn transform_transition(self) -> String {
        format!(
            "transform var({}, {}) var({}, {})",
            self.duration.css_var_name(),
            self.duration.ms(),
            self.curve.css_var_name(),
            self.curve.cubic_bezier(),
        )
    }

    /// Inject shared progress-fill stylesheet once (idempotent via [`inject_style`]).
    pub fn ensure_styles() {
        inject_style("orbital-motion-progress-fill", progress_fill_styles());
    }
}

/// Stylesheet for [`PROGRESS_FILL_CLASS`] / [`PROGRESS_FILL_RESPECTS_REDUCED_CLASS`].
///
/// Defaults use [`DEFAULT_PROGRESS_FILL`] tokens via theme CSS variables.
pub fn progress_fill_styles() -> &'static str {
    r#"
.orbital-motion-progress-fill {
  transition-property: width, transform;
  transition-duration: var(--orb-motion-duration-2xl, 400ms);
  transition-timing-function: var(--orb-motion-ease-standard, cubic-bezier(0.33, 0, 0.67, 1));
}
.orbital-motion-progress-fill-respects-reduced {
  transition-property: width, transform;
  transition-duration: var(--orb-motion-duration-2xl, 400ms);
  transition-timing-function: var(--orb-motion-ease-standard, cubic-bezier(0.33, 0, 0.67, 1));
}
@media (prefers-reduced-motion: reduce) {
  .orbital-motion-progress-fill-respects-reduced {
    transition-duration: 0.001ms !important;
    transition-delay: 0ms !important;
  }
}
"#
}

/// Clamp a unit progress value into `0.0..=1.0`.
#[must_use]
pub fn clamp_unit_progress(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Format a unit progress value as a CSS width percentage string (e.g. `"45%"`).
#[must_use]
pub fn progress_width_percent(value: f64) -> String {
    format!("{}%", clamp_unit_progress(value) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_unit_progress_bounds() {
        assert!((clamp_unit_progress(-1.0) - 0.0).abs() < f64::EPSILON);
        assert!((clamp_unit_progress(0.5) - 0.5).abs() < f64::EPSILON);
        assert!((clamp_unit_progress(2.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn progress_width_percent_formats() {
        assert_eq!(progress_width_percent(0.0), "0%");
        assert_eq!(progress_width_percent(0.45), "45%");
        assert_eq!(progress_width_percent(1.0), "100%");
    }

    #[test]
    fn glacial_uses_theme_var() {
        assert_eq!(
            MotionDuration::Glacial.css_var_name(),
            "--orb-motion-duration-4xl"
        );
        let t = GLACIAL_PROGRESS_FILL.width_transition();
        assert!(t.contains("--orb-motion-duration-4xl"));
        assert!(t.contains("4000ms"));
    }

    #[test]
    fn epic_uses_theme_var_with_fallback() {
        let t = EPIC_PROGRESS_FILL.width_transition();
        assert!(t.contains("--orb-motion-duration-5xl"));
        assert!(t.contains("10000ms"));
    }
}
