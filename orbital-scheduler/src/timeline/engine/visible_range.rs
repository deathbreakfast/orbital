//! Visible time range and column ticks from navigation anchor + preset.

use chrono::{Datelike, Months, NaiveDate, Weekday};
use orbital_base_components::{
    format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds,
};

use crate::start_of_week;
use crate::DateTimeRange;
use crate::TimelinePreset;

/// Hours shown in the single-day preset.
pub const DAY_PRESET_HOURS: usize = 24;
/// Days shown in the single-week preset.
pub const WEEK_PRESET_DAYS: usize = 7;
/// Hours shown in the business-day preset (08:00–18:00).
pub const BUSINESS_DAY_HOURS: usize = 10;
/// Hour of day when the business-day window starts.
pub const BUSINESS_DAY_START_HOUR: i64 = 8;
/// Weekdays shown in the business-week preset.
pub const BUSINESS_WEEK_DAYS: usize = 5;
/// Calendar months shown in the quarter preset.
pub const QUARTER_PRESET_MONTHS: u32 = 3;
/// Width reserved for the resource label column (matches default 8rem CSS).
pub const DEFAULT_RESOURCE_COLUMN_WIDTH_PX: f64 = 128.0;

/// Fixed pixel width per column for virtual scroll math.
pub fn preset_column_width(preset: TimelinePreset) -> f64 {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => 56.0,
        TimelinePreset::Week | TimelinePreset::BusinessWeek => 72.0,
        TimelinePreset::Month => 32.0,
        // Width of a full seven-day week; clipped weeks at the quarter edges are narrower.
        TimelinePreset::Quarter => 72.0,
    }
}

/// Number of time columns shown for a preset zoom level.
///
/// `window_start` is the aligned window start (see [`align_anchor_to_window`]).
/// It matters for [`TimelinePreset::Month`] (28–31 day columns) and
/// [`TimelinePreset::Quarter`] (13–15 week columns); the other presets ignore it.
pub fn preset_column_count(preset: TimelinePreset, window_start: OrbitalDateTime) -> usize {
    match preset {
        TimelinePreset::Day => DAY_PRESET_HOURS,
        TimelinePreset::BusinessDay => BUSINESS_DAY_HOURS,
        TimelinePreset::Week => WEEK_PRESET_DAYS,
        TimelinePreset::BusinessWeek => BUSINESS_WEEK_DAYS,
        TimelinePreset::Month | TimelinePreset::Quarter => window_start
            .wall_date()
            .and_then(|start| calendar_column_dates(start, preset))
            .map(|dates| dates.len().saturating_sub(1))
            .unwrap_or(0),
    }
}

/// Whether the lanes keep their preset column widths and scroll sideways when
/// the scrollport is narrower. Hour and day presets shrink to fit instead.
pub fn preset_scrolls_horizontally(preset: TimelinePreset) -> bool {
    match preset {
        TimelinePreset::Month | TimelinePreset::Quarter => true,
        TimelinePreset::Day
        | TimelinePreset::BusinessDay
        | TimelinePreset::Week
        | TimelinePreset::BusinessWeek => false,
    }
}

/// First day of the month containing `date`.
pub fn start_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

/// First day of the calendar quarter containing `date` (Jan 1, Apr 1, Jul 1 or Oct 1).
pub fn start_of_quarter(date: NaiveDate) -> NaiveDate {
    let month = (date.month0() / QUARTER_PRESET_MONTHS) * QUARTER_PRESET_MONTHS + 1;
    NaiveDate::from_ymd_opt(date.year(), month, 1).unwrap_or(date)
}

/// Column start dates for the month and quarter presets, followed by the
/// (exclusive) window end date. `window_start` must already be aligned.
fn calendar_column_dates(
    window_start: NaiveDate,
    preset: TimelinePreset,
) -> Option<Vec<NaiveDate>> {
    match preset {
        TimelinePreset::Month => {
            let end = window_start.checked_add_months(Months::new(1))?;
            Some(window_start.iter_days().take_while(|d| *d <= end).collect())
        }
        TimelinePreset::Quarter => {
            let end = window_start.checked_add_months(Months::new(QUARTER_PRESET_MONTHS))?;
            let mut dates = vec![window_start];
            dates.extend(
                window_start
                    .iter_days()
                    .skip(1)
                    .take_while(|d| *d < end)
                    .filter(|d| d.weekday() == Weekday::Mon),
            );
            dates.push(end);
            Some(dates)
        }
        TimelinePreset::Day
        | TimelinePreset::BusinessDay
        | TimelinePreset::Week
        | TimelinePreset::BusinessWeek => None,
    }
}

/// One tick in the timeline time header.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineColumn {
    pub start: OrbitalDateTime,
    pub label: String,
    pub width_px: f64,
}

/// Query window and time columns for a timeline preset.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineVisibleRange {
    pub query: DateTimeRange,
    pub range_start: OrbitalDateTime,
    pub range_end: OrbitalDateTime,
    pub columns: Vec<TimelineColumn>,
}

fn rezone(anchor: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(anchor.instant(), display_tz)
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

fn add_hours(dt: OrbitalDateTime, hours: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + hours * 3600, dt.timezone()).ok()
}

fn add_days(dt: OrbitalDateTime, days: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + days * 86_400, dt.timezone()).ok()
}

/// Monday of the calendar week containing `date`.
pub fn start_of_business_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date.checked_sub_days(chrono::Days::new(weekday as u64))
        .unwrap_or(date)
}

/// Align the navigation anchor to the start of the visible window for a preset.
pub fn align_anchor_to_window(
    anchor: OrbitalDateTime,
    preset: TimelinePreset,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let zoned = rezone(anchor, display_tz);
    let current = zoned.wall_date()?;

    match preset {
        TimelinePreset::Day => Some(zoned.start_of_day()),
        TimelinePreset::BusinessDay => add_hours(zoned.start_of_day(), BUSINESS_DAY_START_HOUR),
        TimelinePreset::Week => orbital_from_date(start_of_week(current), display_tz),
        TimelinePreset::BusinessWeek => {
            orbital_from_date(start_of_business_week(current), display_tz)
        }
        TimelinePreset::Month => orbital_from_date(start_of_month(current), display_tz),
        TimelinePreset::Quarter => orbital_from_date(start_of_quarter(current), display_tz),
    }
}

fn column_start_at(
    window_start: OrbitalDateTime,
    preset: TimelinePreset,
    index: usize,
) -> Option<OrbitalDateTime> {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => add_hours(window_start, index as i64),
        TimelinePreset::Week | TimelinePreset::BusinessWeek => add_days(window_start, index as i64),
        TimelinePreset::Month | TimelinePreset::Quarter => {
            let dates = calendar_column_dates(window_start.wall_date()?, preset)?;
            orbital_from_date(*dates.get(index)?, window_start.timezone())
        }
    }
}

fn column_label(start: OrbitalDateTime, preset: TimelinePreset, ampm: bool) -> String {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => {
            let format = if ampm {
                DatetimeFormat::Time12
            } else {
                DatetimeFormat::Time24
            };
            format_datetime(start, format)
        }
        TimelinePreset::Week | TimelinePreset::BusinessWeek | TimelinePreset::Quarter => {
            format_datetime(start, DatetimeFormat::IsoDate)
        }
        TimelinePreset::Month => start
            .wall_date()
            .map(|d| d.day().to_string())
            .unwrap_or_default(),
    }
}

fn column_width(start: OrbitalDateTime, next: OrbitalDateTime, preset: TimelinePreset) -> f64 {
    let width = preset_column_width(preset);
    match preset {
        TimelinePreset::Quarter => match (start.wall_date(), next.wall_date()) {
            (Some(a), Some(b)) => width * (b - a).num_days() as f64 / WEEK_PRESET_DAYS as f64,
            _ => width,
        },
        TimelinePreset::Day
        | TimelinePreset::BusinessDay
        | TimelinePreset::Week
        | TimelinePreset::BusinessWeek
        | TimelinePreset::Month => width,
    }
}

fn range_end_from_columns(
    window_start: OrbitalDateTime,
    preset: TimelinePreset,
    count: usize,
) -> Option<OrbitalDateTime> {
    let next_window = column_start_at(window_start, preset, count)?;
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(
        next_window.to_unix_seconds() - 1,
        window_start.timezone(),
    )
    .ok()
}

/// Compute the visible query range and time columns for the active preset.
pub fn timeline_visible_range(
    anchor: OrbitalDateTime,
    preset: TimelinePreset,
    display_tz: DatetimeTimezone,
    ampm: bool,
) -> Option<TimelineVisibleRange> {
    let range_start = align_anchor_to_window(anchor, preset, display_tz)?;
    let count = preset_column_count(preset, range_start);
    let range_end = range_end_from_columns(range_start, preset, count)?;

    let mut columns = Vec::with_capacity(count);
    let mut start = column_start_at(range_start, preset, 0)?;
    for i in 0..count {
        let next = column_start_at(range_start, preset, i + 1)?;
        columns.push(TimelineColumn {
            label: column_label(start, preset, ampm),
            start,
            width_px: column_width(start, next, preset),
        });
        start = next;
    }

    Some(TimelineVisibleRange {
        query: DateTimeRange::new(range_start, range_end),
        range_start,
        range_end,
        columns,
    })
}

/// Total content width in pixels for all columns.
pub fn timeline_total_width(range: &TimelineVisibleRange) -> f64 {
    range.columns.iter().map(|c| c.width_px).sum()
}

/// Stretch or shrink columns so the full preset fits the time scrollport width.
///
/// Columns keep their relative widths, so clipped quarter weeks stay narrower.
pub fn fit_timeline_columns_to_viewport(range: &mut TimelineVisibleRange, time_area_width_px: f64) {
    let total = timeline_total_width(range);
    if range.columns.is_empty() || time_area_width_px <= 0.0 || total <= 0.0 {
        return;
    }

    let scale = time_area_width_px / total;
    for col in &mut range.columns {
        col.width_px *= scale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::TryFromUnixSeconds;

    fn anchor() -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day()
    }

    fn utc_date(y: i32, m: u32, d: u32) -> OrbitalDateTime {
        OrbitalDateTime::from_naive_date(
            NaiveDate::from_ymd_opt(y, m, d).expect("date"),
            DatetimeTimezone::Utc,
        )
        .expect("valid")
    }

    #[test]
    fn preset_month_columns_one_per_day_happy() {
        let nov = timeline_visible_range(
            utc_date(2026, 11, 17),
            TimelinePreset::Month,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(nov.columns.len(), 30);
        assert_eq!(nov.range_start, utc_date(2026, 11, 1));
        assert_eq!(nov.columns[0].label, "1");
        assert_eq!(nov.columns[29].label, "30");
        assert_eq!(
            nov.range_end.wall_date(),
            NaiveDate::from_ymd_opt(2026, 11, 30)
        );
        assert!(nov.columns.iter().all(|c| (c.width_px - 32.0).abs() < 1e-9));

        let feb = timeline_visible_range(
            utc_date(2027, 2, 3),
            TimelinePreset::Month,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(feb.columns.len(), 28);
        let jan = timeline_visible_range(
            utc_date(2027, 1, 3),
            TimelinePreset::Month,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(jan.columns.len(), 31);
    }

    #[test]
    fn preset_quarter_columns_one_per_week_happy() {
        // Q4 2026: Oct 1 is a Thursday, Dec 31 is a Thursday.
        let range = timeline_visible_range(
            utc_date(2026, 11, 5),
            TimelinePreset::Quarter,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(range.range_start, utc_date(2026, 10, 1));
        assert_eq!(
            range.range_end.wall_date(),
            NaiveDate::from_ymd_opt(2026, 12, 31)
        );
        // Oct 1 (Thu), then Mondays Oct 5 … Dec 28.
        assert_eq!(range.columns.len(), 14);
        assert_eq!(range.columns[0].label, "2026-10-01");
        assert_eq!(range.columns[1].label, "2026-10-05");
        assert_eq!(range.columns[13].label, "2026-12-28");
        assert!(range.columns[1..]
            .iter()
            .all(|c| c.start.wall_date().unwrap().weekday() == Weekday::Mon));
        // Oct 1–4 is four days wide; Dec 28–31 is four days wide.
        assert!((range.columns[0].width_px - 72.0 * 4.0 / 7.0).abs() < 1e-9);
        assert!((range.columns[1].width_px - 72.0).abs() < 1e-9);
        assert!((range.columns[13].width_px - 72.0 * 4.0 / 7.0).abs() < 1e-9);
        let total_days = 92.0;
        assert!((timeline_total_width(&range) - 72.0 * total_days / 7.0).abs() < 1e-6);
    }

    #[test]
    fn fit_columns_keeps_relative_widths_happy() {
        let mut range = timeline_visible_range(
            utc_date(2026, 11, 5),
            TimelinePreset::Quarter,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        fit_timeline_columns_to_viewport(&mut range, 920.0);
        assert!((timeline_total_width(&range) - 920.0).abs() < 1e-6);
        assert!(range.columns[0].width_px < range.columns[1].width_px);
    }

    #[test]
    fn day_preset_is_single_day_hourly() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), DAY_PRESET_HOURS);
        assert_eq!(range.range_start, anchor());
        let end_day = range.range_end.wall_date().expect("end day");
        assert_eq!(end_day, anchor().wall_date().expect("start day"));
    }

    #[test]
    fn business_day_preset_is_eight_am_to_six_pm() {
        let range = timeline_visible_range(
            anchor(),
            TimelinePreset::BusinessDay,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(range.columns.len(), BUSINESS_DAY_HOURS);
        assert_eq!(range.range_start.hour_minute_second(), Some((8, 0, 0)));
        assert_eq!(range.columns.first().unwrap().label, "08:00 AM");
        assert_eq!(range.columns.last().unwrap().label, "05:00 PM");
    }

    #[test]
    fn week_preset_spans_seven_days() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), WEEK_PRESET_DAYS);
    }

    #[test]
    fn business_week_preset_spans_five_weekdays() {
        let range = timeline_visible_range(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(range.columns.len(), BUSINESS_WEEK_DAYS);
        let start = range.range_start.wall_date().expect("start");
        assert_eq!(start.weekday(), chrono::Weekday::Mon);
    }

    #[test]
    fn align_week_starts_on_sunday() {
        let aligned = align_anchor_to_window(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc)
            .expect("aligned");
        let date = aligned.wall_date().expect("date");
        assert_eq!(date, start_of_week(anchor().wall_date().unwrap()));
    }

    #[test]
    fn align_business_week_starts_on_monday() {
        let aligned = align_anchor_to_window(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
        )
        .expect("aligned");
        let date = aligned.wall_date().expect("date");
        assert_eq!(date, start_of_business_week(anchor().wall_date().unwrap()));
    }

    #[test]
    fn day_column_labels_are_times() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert!(range.columns[9].label.contains(':'));
        assert!(!range.columns[9].label.contains('-'));
    }

    #[test]
    fn day_preset_includes_full_day_through_23_00() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), DAY_PRESET_HOURS);
        assert_eq!(range.columns.first().unwrap().label, "12:00 AM");
        assert_eq!(range.columns.last().unwrap().label, "11:00 PM");
    }

    #[test]
    fn fit_columns_use_available_time_area() {
        let mut range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        fit_timeline_columns_to_viewport(&mut range, 800.0);
        assert!((timeline_total_width(&range) - 800.0).abs() < 0.01);
        assert!((range.columns[0].width_px - 800.0 / 24.0).abs() < 0.01);
    }
}
