//! Horizontal pointer math for timeline drag (SC-20).

use orbital_base_components::{
    DatetimeTimezone, OrbitalDateTime, ToUnixSeconds, TryFromUnixSeconds,
};

use crate::timeline::engine::TimelineVisibleRange;
use crate::PlannedEvent;
use crate::{MIN_EVENT_DURATION_MINUTES, SNAP_STEP_MINUTES};

fn span_seconds(range: &TimelineVisibleRange) -> f64 {
    let start = range.range_start.to_unix_seconds();
    let end = range.range_end.to_unix_seconds();
    (end - start).max(1) as f64
}

/// Map a horizontal ratio within the visible range to an instant.
pub fn datetime_from_range_ratio(
    range: &TimelineVisibleRange,
    ratio: f64,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let ratio = ratio.clamp(0.0, 1.0);
    let offset_secs = (span_seconds(range) * ratio).round() as i64;
    let unix = range.range_start.to_unix_seconds() + offset_secs;
    orbital_base_components::OrbitalDateTime::try_from_unix_seconds(unix, display_tz).ok()
}

fn snap_offset_seconds(offset_secs: f64) -> i64 {
    let step = (SNAP_STEP_MINUTES * 60.0).round() as i64;
    ((offset_secs / step as f64).round() * step as f64) as i64
}

/// Move event preserving duration; `start_ratio` is position in visible range.
pub fn move_event_horizontally(
    event: &PlannedEvent,
    range: &TimelineVisibleRange,
    start_ratio: f64,
    display_tz: DatetimeTimezone,
) -> Option<(OrbitalDateTime, OrbitalDateTime)> {
    let duration_secs = (event.end.to_unix_seconds() - event.start.to_unix_seconds())
        .max((MIN_EVENT_DURATION_MINUTES * 60.0) as i64);
    let offset = snap_offset_seconds(span_seconds(range) * start_ratio.clamp(0.0, 1.0));
    let start_unix = range.range_start.to_unix_seconds() + offset;
    let end_unix = start_unix + duration_secs;
    let start =
        orbital_base_components::OrbitalDateTime::try_from_unix_seconds(start_unix, display_tz)
            .ok()?;
    let end = orbital_base_components::OrbitalDateTime::try_from_unix_seconds(end_unix, display_tz)
        .ok()?;
    Some((start, end))
}

/// Resize start edge to ratio within visible range.
pub fn resize_timeline_event_start(
    event: &PlannedEvent,
    range: &TimelineVisibleRange,
    start_ratio: f64,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let start = datetime_from_range_ratio(range, start_ratio, display_tz)?;
    let min_end = start.to_unix_seconds() + (MIN_EVENT_DURATION_MINUTES * 60.0) as i64;
    if event.end.to_unix_seconds() < min_end {
        return orbital_base_components::OrbitalDateTime::try_from_unix_seconds(
            min_end, display_tz,
        )
        .ok();
    }
    if start.instant() >= event.end.instant() {
        return None;
    }
    Some(start)
}

/// Resize end edge to ratio within visible range.
pub fn resize_timeline_event_end(
    event: &PlannedEvent,
    range: &TimelineVisibleRange,
    end_ratio: f64,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let end = datetime_from_range_ratio(range, end_ratio, display_tz)?;
    let min_start = end.to_unix_seconds() - (MIN_EVENT_DURATION_MINUTES * 60.0) as i64;
    if event.start.to_unix_seconds() > min_start {
        return orbital_base_components::OrbitalDateTime::try_from_unix_seconds(
            min_start.max(event.start.to_unix_seconds()),
            display_tz,
        )
        .ok();
    }
    if end.instant() <= event.start.instant() {
        return None;
    }
    Some(end)
}

/// Horizontal offset ratio from pointer X within a lane rect.
pub fn ratio_from_lane_x(client_x: f64, rect_left: f64, rect_width: f64) -> f64 {
    if rect_width <= 0.0 {
        return 0.0;
    }
    ((client_x - rect_left) / rect_width).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TimelinePreset;
    use orbital_base_components::TryFromUnixSeconds;

    fn sample_range() -> TimelineVisibleRange {
        crate::timeline_visible_range(
            OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Local)
                .unwrap()
                .start_of_day(),
            TimelinePreset::Day,
            DatetimeTimezone::Local,
            true,
        )
        .unwrap()
    }

    #[test]
    fn move_preserves_duration() {
        let range = sample_range();
        let start =
            OrbitalDateTime::try_from_unix_seconds(1_735_693_200, DatetimeTimezone::Local).unwrap();
        let end =
            OrbitalDateTime::try_from_unix_seconds(1_735_696_800, DatetimeTimezone::Local).unwrap();
        let event = PlannedEvent::new("evt-1".to_string(), "Test".to_string(), start, end);
        let (new_start, new_end) =
            move_event_horizontally(&event, &range, 0.25, DatetimeTimezone::Local).unwrap();
        assert_eq!(
            new_end.to_unix_seconds() - new_start.to_unix_seconds(),
            3600
        );
    }
}
