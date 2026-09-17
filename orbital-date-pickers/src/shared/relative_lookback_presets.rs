//! Default relative lookback presets for [`DateTimeRangePicker`](crate::DateTimeRangePicker).

use chrono::{Duration, Months};
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use crate::DateTimeRange;

/// One lookback option shown in the DateTimeRange popover rail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelativeLookbackPreset {
    /// Operator-facing label (e.g. "−1 hour").
    pub label: &'static str,
    pub kind: RelativeLookbackKind,
}

/// How far back from `now` the lookback start is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelativeLookbackKind {
    Hours(i64),
    Days(i64),
    Months(u32),
}

/// Ship defaults: −1h, −12h, −24h, −1w, −2w, −1 month.
pub fn default_relative_lookback_presets() -> &'static [RelativeLookbackPreset] {
    &[
        RelativeLookbackPreset {
            label: "−1 hour",
            kind: RelativeLookbackKind::Hours(1),
        },
        RelativeLookbackPreset {
            label: "−12 hours",
            kind: RelativeLookbackKind::Hours(12),
        },
        RelativeLookbackPreset {
            label: "−24 hours",
            kind: RelativeLookbackKind::Hours(24),
        },
        RelativeLookbackPreset {
            label: "−1 week",
            kind: RelativeLookbackKind::Days(7),
        },
        RelativeLookbackPreset {
            label: "−2 weeks",
            kind: RelativeLookbackKind::Days(14),
        },
        RelativeLookbackPreset {
            label: "−1 month",
            kind: RelativeLookbackKind::Months(1),
        },
    ]
}

/// `end = now`, `start = now − duration` in `timezone`.
pub fn lookback_range_now(
    kind: RelativeLookbackKind,
    timezone: DatetimeTimezone,
) -> Option<DateTimeRange> {
    lookback_range_at(kind, OrbitalDateTime::utc_now(timezone))
}

/// Whether `range` matches a lookback duration within `tolerance` of wall-clock now.
pub fn range_matches_lookback(
    range: &DateTimeRange,
    kind: RelativeLookbackKind,
    now: OrbitalDateTime,
    tolerance: Duration,
) -> bool {
    let Some(expected) = lookback_range_at(kind, now) else {
        return false;
    };
    let start_ok = (range.start.instant() - expected.start.instant())
        .num_milliseconds()
        .abs()
        <= tolerance.num_milliseconds();
    let end_ok = (range.end.instant() - expected.end.instant())
        .num_milliseconds()
        .abs()
        <= tolerance.num_milliseconds();
    start_ok && end_ok
}

fn lookback_range_at(kind: RelativeLookbackKind, end: OrbitalDateTime) -> Option<DateTimeRange> {
    let start = match kind {
        RelativeLookbackKind::Hours(h) => {
            OrbitalDateTime::from_instant(end.instant() - Duration::hours(h), end.timezone())
        }
        RelativeLookbackKind::Days(d) => {
            OrbitalDateTime::from_instant(end.instant() - Duration::days(d), end.timezone())
        }
        RelativeLookbackKind::Months(m) => {
            let date = end.wall_date()?;
            let prev = date.checked_sub_months(Months::new(m))?;
            let (h, mi, s) = end.hour_minute_second().unwrap_or((0, 0, 0));
            OrbitalDateTime::from_naive_date(prev, end.timezone())?.apply_hms(h, mi, s)?
        }
    };
    Some(DateTimeRange::new(start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{DatetimeTimezone, TryFromUnixSeconds};

    #[test]
    fn lookback_one_hour_happy() {
        let tz = DatetimeTimezone::Utc;
        let end = OrbitalDateTime::try_from_unix_seconds(3_600, tz).expect("valid");
        let range = lookback_range_at(RelativeLookbackKind::Hours(1), end).expect("range");
        assert_eq!(range.end.instant().timestamp(), 3_600);
        assert_eq!(range.start.instant().timestamp(), 0);
    }

    #[test]
    fn range_matches_lookback_within_tolerance() {
        let tz = DatetimeTimezone::Utc;
        let end = OrbitalDateTime::try_from_unix_seconds(10_000, tz).expect("valid");
        let range = lookback_range_at(RelativeLookbackKind::Hours(1), end).expect("range");
        assert!(range_matches_lookback(
            &range,
            RelativeLookbackKind::Hours(1),
            end,
            Duration::seconds(30),
        ));
        assert!(!range_matches_lookback(
            &range,
            RelativeLookbackKind::Hours(12),
            end,
            Duration::seconds(30),
        ));
    }
}
