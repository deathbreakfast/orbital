//! Preset-aware navigation stepping and range labels.

use chrono::{Datelike, Local, Months, NaiveDate, Utc};
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};

use crate::timeline::engine::{
    align_anchor_to_window, timeline_visible_range, BUSINESS_WEEK_DAYS, QUARTER_PRESET_MONTHS,
    WEEK_PRESET_DAYS,
};
use crate::NavDirection;
use crate::TimelinePreset;

fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => chrono::FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

fn add_months(dt: OrbitalDateTime, months: i64) -> Option<OrbitalDateTime> {
    let date = dt.wall_date()?;
    let step = Months::new(months.unsigned_abs() as u32);
    let next = if months >= 0 {
        date.checked_add_months(step)?
    } else {
        date.checked_sub_months(step)?
    };
    orbital_from_date(next, dt.timezone())
}

fn add_days(dt: OrbitalDateTime, days: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::{ToUnixSeconds, TryFromUnixSeconds};
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + days * 86_400, dt.timezone()).ok()
}

/// Advance or retreat the navigation anchor by one preset page.
pub fn advance_visible_date_by_preset(
    date: OrbitalDateTime,
    preset: TimelinePreset,
    direction: NavDirection,
) -> Option<OrbitalDateTime> {
    let tz = date.timezone();
    let aligned = align_anchor_to_window(date, preset, tz)?;
    let signed = match direction {
        NavDirection::Next => 1_i64,
        NavDirection::Previous => -1,
    };

    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => add_days(aligned, signed),
        TimelinePreset::Week => add_days(aligned, signed * WEEK_PRESET_DAYS as i64),
        TimelinePreset::BusinessWeek => add_days(aligned, signed * BUSINESS_WEEK_DAYS as i64),
        TimelinePreset::Month => add_months(aligned, signed),
        TimelinePreset::Quarter => add_months(aligned, signed * QUARTER_PRESET_MONTHS as i64),
    }
}

/// Format the toolbar title for the current preset and anchor date.
pub fn format_timeline_range_label(date: OrbitalDateTime, preset: TimelinePreset) -> String {
    let Some(range) = timeline_visible_range(date, preset, date.timezone(), true) else {
        return String::new();
    };

    match preset {
        TimelinePreset::Day => format_datetime(range.range_start, DatetimeFormat::IsoDate),
        TimelinePreset::BusinessDay => format!(
            "{} · Business day",
            format_datetime(range.range_start, DatetimeFormat::IsoDate)
        ),
        TimelinePreset::Week | TimelinePreset::BusinessWeek => format!(
            "{} – {}",
            format_datetime(range.range_start, DatetimeFormat::IsoDate),
            format_datetime(range.range_end, DatetimeFormat::IsoDate)
        ),
        TimelinePreset::Month => range
            .range_start
            .wall_date()
            .map(|d| d.format("%B %Y").to_string())
            .unwrap_or_default(),
        TimelinePreset::Quarter => {
            match (range.range_start.wall_date(), range.range_end.wall_date()) {
                (Some(start), Some(end)) => format!(
                    "{} – {} {}",
                    start.format("%b"),
                    end.format("%b"),
                    end.year()
                ),
                _ => String::new(),
            }
        }
    }
}

/// Today as an anchor for timeline navigation.
pub fn today_anchor(timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    orbital_from_date(today_for_timezone(timezone), timezone).map(|d| d.start_of_day())
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

    #[test]
    fn advance_day_moves_one_day() {
        let next =
            advance_visible_date_by_preset(anchor(), TimelinePreset::Day, NavDirection::Next)
                .expect("next");
        let prev =
            advance_visible_date_by_preset(next, TimelinePreset::Day, NavDirection::Previous)
                .expect("prev");
        assert_eq!(prev, anchor());
    }

    #[test]
    fn advance_week_moves_one_week() {
        let week_start =
            align_anchor_to_window(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc)
                .expect("start");
        let next =
            advance_visible_date_by_preset(week_start, TimelinePreset::Week, NavDirection::Next)
                .expect("next");
        let label = format_timeline_range_label(next, TimelinePreset::Week);
        assert!(!label.is_empty());
    }

    #[test]
    fn advance_business_day_moves_one_day() {
        let aligned =
            align_anchor_to_window(anchor(), TimelinePreset::BusinessDay, DatetimeTimezone::Utc)
                .expect("aligned");
        let next = advance_visible_date_by_preset(
            anchor(),
            TimelinePreset::BusinessDay,
            NavDirection::Next,
        )
        .expect("next");
        let prev = advance_visible_date_by_preset(
            next,
            TimelinePreset::BusinessDay,
            NavDirection::Previous,
        )
        .expect("prev");
        assert_eq!(prev, aligned);
    }

    #[test]
    fn advance_business_week_moves_five_days() {
        let week_start = align_anchor_to_window(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
        )
        .expect("start");
        let next = advance_visible_date_by_preset(
            week_start,
            TimelinePreset::BusinessWeek,
            NavDirection::Next,
        )
        .expect("next");
        let label = format_timeline_range_label(next, TimelinePreset::BusinessWeek);
        assert!(!label.is_empty());
    }

    fn utc_date(y: i32, m: u32, d: u32) -> OrbitalDateTime {
        OrbitalDateTime::from_naive_date(
            NaiveDate::from_ymd_opt(y, m, d).expect("date"),
            DatetimeTimezone::Utc,
        )
        .expect("valid")
    }

    #[test]
    fn preset_month_align_and_step_happy() {
        let next = advance_visible_date_by_preset(
            utc_date(2027, 1, 31),
            TimelinePreset::Month,
            NavDirection::Next,
        )
        .expect("next");
        assert_eq!(next, utc_date(2027, 2, 1));
        let range =
            timeline_visible_range(next, TimelinePreset::Month, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.range_start, utc_date(2027, 2, 1));
        assert_eq!(range.columns.len(), 28);
        let prev = advance_visible_date_by_preset(
            utc_date(2027, 1, 15),
            TimelinePreset::Month,
            NavDirection::Previous,
        )
        .expect("prev");
        assert_eq!(prev, utc_date(2026, 12, 1));
        assert_eq!(
            format_timeline_range_label(utc_date(2026, 11, 20), TimelinePreset::Month),
            "November 2026"
        );
    }

    #[test]
    fn preset_quarter_range_label_happy() {
        assert_eq!(
            format_timeline_range_label(utc_date(2026, 11, 20), TimelinePreset::Quarter),
            "Oct – Dec 2026"
        );
        let next = advance_visible_date_by_preset(
            utc_date(2026, 11, 20),
            TimelinePreset::Quarter,
            NavDirection::Next,
        )
        .expect("next");
        assert_eq!(next, utc_date(2027, 1, 1));
        assert_eq!(
            format_timeline_range_label(next, TimelinePreset::Quarter),
            "Jan – Mar 2027"
        );
        let prev = advance_visible_date_by_preset(
            utc_date(2026, 11, 20),
            TimelinePreset::Quarter,
            NavDirection::Previous,
        )
        .expect("prev");
        assert_eq!(prev, utc_date(2026, 7, 1));
    }

    #[test]
    fn preset_wire_values_round_trip_happy() {
        for preset in TimelinePreset::all() {
            assert_eq!(
                TimelinePreset::from_wire_value(preset.wire_value()),
                Some(*preset)
            );
        }
        assert_eq!(TimelinePreset::Month.label(), "Month");
        assert_eq!(TimelinePreset::Quarter.label(), "Quarter");
        let all = TimelinePreset::all();
        let bw = all
            .iter()
            .position(|p| *p == TimelinePreset::BusinessWeek)
            .expect("business week");
        assert_eq!(
            &all[bw + 1..],
            &[TimelinePreset::Month, TimelinePreset::Quarter]
        );
    }

    #[test]
    fn preset_legacy_month_year_map_to_new_presets_happy() {
        assert_eq!(
            TimelinePreset::from_wire_value("month"),
            Some(TimelinePreset::Month)
        );
        assert_eq!(
            TimelinePreset::from_wire_value("year"),
            Some(TimelinePreset::Quarter)
        );
        assert_eq!(
            TimelinePreset::from_wire_value("hour"),
            Some(TimelinePreset::Day)
        );
    }

    #[test]
    fn preset_unknown_wire_value_none_sad() {
        assert_eq!(TimelinePreset::from_wire_value("decade"), None);
        assert_eq!(TimelinePreset::from_wire_value(""), None);
        assert_eq!(TimelinePreset::from_wire_value("Month"), None);
    }

    #[test]
    fn labels_differ_by_preset() {
        let date = anchor();
        let day = format_timeline_range_label(date, TimelinePreset::Day);
        let week = format_timeline_range_label(date, TimelinePreset::Week);
        assert!(!day.is_empty());
        assert!(!week.is_empty());
        assert_ne!(day, week);
    }
}
