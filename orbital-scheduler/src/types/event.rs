//! Planned event types for the scheduler.
//!
//! `SchedulerEvent` was renamed to [`PlannedEvent`] before API freeze (design §3.1).

use orbital_base_components::OrbitalDateTime;

/// A scheduled event with typed start/end datetimes.
#[derive(Clone, Debug, PartialEq)]
pub struct PlannedEvent {
    pub id: String,
    pub title: String,
    pub start: OrbitalDateTime,
    pub end: OrbitalDateTime,
    pub resource_id: Option<String>,
    /// RFC 5545 recurrence rule when [`SchedulerFeatures::RECURRING_EVENTS`](crate::SchedulerFeatures::RECURRING_EVENTS) is enabled.
    pub recurrence_rule: Option<String>,
    pub color: Option<String>,
    /// Semantic colour for the timeline bar, used when `color` is `None`.
    ///
    /// Use [`EventTone::Danger`] for late work. Calendar views ignore it.
    pub tone: Option<EventTone>,
    /// Share of the work done, from `0.0` to `1.0`. The timeline bar fills
    /// this share of its width; point events ignore it.
    pub progress: Option<f32>,
    pub is_draggable: Option<bool>,
    pub is_resizable: Option<bool>,
    pub is_editable: Option<bool>,
}

impl PlannedEvent {
    /// Creates an event with the required fields; optional fields default to `None`.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        start: OrbitalDateTime,
        end: OrbitalDateTime,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            start,
            end,
            resource_id: None,
            recurrence_rule: None,
            color: None,
            tone: None,
            progress: None,
            is_draggable: None,
            is_resizable: None,
            is_editable: None,
        }
    }

    /// Sets [`tone`](Self::tone).
    pub fn with_tone(mut self, tone: EventTone) -> Self {
        self.tone = Some(tone);
        self
    }

    /// Sets [`progress`](Self::progress), clamped to `0.0..=1.0`. `NaN` becomes `0.0`.
    pub fn with_progress(mut self, progress: f32) -> Self {
        let clamped = if progress.is_nan() {
            0.0
        } else {
            progress.clamp(0.0, 1.0)
        };
        self.progress = Some(clamped);
        self
    }

    /// Whether the event happens at one instant (`start == end`).
    ///
    /// The timeline draws these as a diamond instead of a bar.
    pub fn is_point(&self) -> bool {
        self.start == self.end
    }
}

/// Semantic colour for a timeline bar (see [`PlannedEvent::tone`]).
///
/// Each tone maps to Orbital colour tokens, so it follows the light and dark themes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventTone {
    /// Subtle surface with the default border.
    #[default]
    Neutral,
    /// Brand colour, for highlighted or primary work.
    Brand,
    /// Done or on track.
    Success,
    /// At risk.
    Warning,
    /// Late or blocked.
    Danger,
}

/// CSS custom properties a tone draws with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventToneTokens {
    /// Bar background.
    pub background: &'static str,
    /// Bar border, and the progress fill.
    pub border: &'static str,
    /// Text colour.
    pub foreground: &'static str,
}

impl EventTone {
    /// Stable wire value (`"neutral"`, `"brand"`, …).
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Brand => "brand",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    /// The Orbital colour tokens for this tone.
    pub fn tokens(self) -> EventToneTokens {
        match self {
            Self::Neutral => EventToneTokens {
                background: "var(--orb-color-surface-subtle)",
                border: "var(--orb-color-border-default)",
                foreground: "var(--orb-color-text-primary)",
            },
            Self::Brand => EventToneTokens {
                background: "var(--orb-color-brand-bg)",
                border: "var(--orb-color-brand-stroke)",
                // brand-bg is the solid brand fill; body text on it fails contrast.
                foreground: "var(--orb-color-text-on-brand)",
            },
            Self::Success => EventToneTokens {
                background: "var(--orb-color-status-success-bg)",
                border: "var(--orb-color-status-success-border)",
                foreground: "var(--orb-color-status-success-fg)",
            },
            Self::Warning => EventToneTokens {
                background: "var(--orb-color-status-warning-bg)",
                border: "var(--orb-color-status-warning-border)",
                foreground: "var(--orb-color-status-warning-fg)",
            },
            Self::Danger => EventToneTokens {
                background: "var(--orb-color-status-danger-bg)",
                border: "var(--orb-color-status-danger-border)",
                foreground: "var(--orb-color-status-danger-fg)",
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{DatetimeTimezone, TryFromUnixSeconds};

    fn at(secs: i64) -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(secs, DatetimeTimezone::Utc).expect("valid")
    }

    #[test]
    fn tone_maps_to_tokens_happy() {
        assert_eq!(
            EventTone::Danger.tokens().background,
            "var(--orb-color-status-danger-bg)"
        );
        assert_eq!(
            EventTone::Brand.tokens().border,
            "var(--orb-color-brand-stroke)"
        );
        assert_eq!(
            EventTone::Brand.tokens().foreground,
            "var(--orb-color-text-on-brand)"
        );
        assert_eq!(
            EventTone::Neutral.tokens().background,
            "var(--orb-color-surface-subtle)"
        );
        assert_eq!(EventTone::default(), EventTone::Neutral);
        assert_eq!(EventTone::Warning.wire_value(), "warning");
    }

    #[test]
    fn progress_clamped_to_unit_range_sad() {
        let event = PlannedEvent::new("e", "E", at(0), at(3600));
        assert_eq!(event.clone().with_progress(1.4).progress, Some(1.0));
        assert_eq!(event.clone().with_progress(-0.2).progress, Some(0.0));
        assert_eq!(event.with_progress(f32::NAN).progress, Some(0.0));
    }

    #[test]
    fn new_leaves_tone_and_progress_unset_happy() {
        let event = PlannedEvent::new("e", "E", at(0), at(0));
        assert_eq!(event.tone, None);
        assert_eq!(event.progress, None);
        assert!(event.is_point());
        assert_eq!(
            event.with_tone(EventTone::Success).tone,
            Some(EventTone::Success)
        );
    }
}
