//! RFC 5545 RRULE subset parser and recurrence expansion (SC-04).
//!
//! # Supported RRULE subset
//!
//! | Token | Support | Notes |
//! |-------|---------|-------|
//! | `FREQ` | `DAILY`, `WEEKLY` | Required |
//! | `INTERVAL` | integer ≥ 1 | Default 1 |
//! | `BYDAY` | `MO,TU,WE,TH,FR,SA,SU` | For weekly; defaults to master start weekday |
//! | `COUNT` | integer | Mutually exclusive with `UNTIL` |
//! | `UNTIL` | `YYYYMMDD` | Inclusive end date in master event timezone |
//!
//! Unsupported tokens (`MONTHLY`, `YEARLY`, `BYMONTHDAY`, `BYSETPOS`, `WKST`, `EXDATE`, `RDATE`)
//! cause parse failure; the master event renders once without expansion.

use chrono::{Datelike, NaiveDate, TimeZone, Weekday};
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, ToUnixSeconds};

use super::visible_range::visible_range;
use crate::{DateTimeRange, PlannedEvent, SchedulerFeatures, SchedulerView};

const INSTANCE_SEPARATOR: &str = "::";
const MAX_INSTANCES: u32 = 500;

/// Recurrence frequency.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecurrenceFreq {
    Daily,
    Weekly,
}

/// Parsed recurrence rule (supported subset only).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecurrenceRule {
    pub freq: RecurrenceFreq,
    pub interval: u32,
    pub by_day: Vec<Weekday>,
    pub count: Option<u32>,
    pub until: Option<NaiveDate>,
}

/// Recurrence parse failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecurrenceError {
    Empty,
    MissingFreq,
    UnsupportedFreq,
    InvalidToken,
    CountAndUntil,
}

impl std::fmt::Display for RecurrenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "recurrence rule is empty"),
            Self::MissingFreq => write!(f, "recurrence rule is missing FREQ"),
            Self::UnsupportedFreq => write!(f, "unsupported recurrence FREQ"),
            Self::InvalidToken => write!(f, "invalid recurrence rule token"),
            Self::CountAndUntil => write!(f, "recurrence rule cannot set both COUNT and UNTIL"),
        }
    }
}

impl std::error::Error for RecurrenceError {}

/// Build a synthetic instance id from master id and occurrence start instant.
pub fn instance_id(master_id: &str, start: OrbitalDateTime) -> String {
    format!("{master_id}{INSTANCE_SEPARATOR}{}", start.to_unix_seconds())
}

/// Strip synthetic instance suffix; returns the id unchanged when not synthetic.
pub fn master_event_id(id: &str) -> &str {
    id.split_once(INSTANCE_SEPARATOR)
        .map(|(master, _)| master)
        .unwrap_or(id)
}

/// Parse occurrence start from a synthetic instance id.
pub fn occurrence_start_from_instance(
    id: &str,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let (_, unix_str) = id.split_once(INSTANCE_SEPARATOR)?;
    let secs: i64 = unix_str.parse().ok()?;
    chrono::Utc
        .timestamp_opt(secs, 0)
        .single()
        .map(|instant| OrbitalDateTime::from_instant(instant, timezone))
}

/// Parse a supported RRULE string (semicolon-separated key=value pairs).
pub fn parse_recurrence_rule(rule: &str) -> Result<RecurrenceRule, RecurrenceError> {
    let trimmed = rule.trim();
    if trimmed.is_empty() {
        return Err(RecurrenceError::Empty);
    }

    let mut freq = None;
    let mut interval = 1_u32;
    let mut by_day = Vec::new();
    let mut count = None;
    let mut until = None;

    for part in trimmed.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (key, value) = part.split_once('=').ok_or(RecurrenceError::InvalidToken)?;
        match key.trim().to_uppercase().as_str() {
            "FREQ" => {
                freq = Some(match value.trim().to_uppercase().as_str() {
                    "DAILY" => RecurrenceFreq::Daily,
                    "WEEKLY" => RecurrenceFreq::Weekly,
                    _ => return Err(RecurrenceError::UnsupportedFreq),
                });
            }
            "INTERVAL" => {
                interval = value
                    .trim()
                    .parse()
                    .map_err(|_| RecurrenceError::InvalidToken)?;
                if interval == 0 {
                    return Err(RecurrenceError::InvalidToken);
                }
            }
            "BYDAY" => {
                for token in value.split(',') {
                    by_day.push(parse_weekday(token.trim())?);
                }
            }
            "COUNT" => {
                count = Some(
                    value
                        .trim()
                        .parse()
                        .map_err(|_| RecurrenceError::InvalidToken)?,
                );
            }
            "UNTIL" => {
                until = Some(parse_until(value.trim())?);
            }
            _ => return Err(RecurrenceError::InvalidToken),
        }
    }

    let freq = freq.ok_or(RecurrenceError::MissingFreq)?;
    if count.is_some() && until.is_some() {
        return Err(RecurrenceError::CountAndUntil);
    }

    Ok(RecurrenceRule {
        freq,
        interval,
        by_day,
        count,
        until,
    })
}

fn parse_weekday(token: &str) -> Result<Weekday, RecurrenceError> {
    match token.to_uppercase().as_str() {
        "MO" => Ok(Weekday::Mon),
        "TU" => Ok(Weekday::Tue),
        "WE" => Ok(Weekday::Wed),
        "TH" => Ok(Weekday::Thu),
        "FR" => Ok(Weekday::Fri),
        "SA" => Ok(Weekday::Sat),
        "SU" => Ok(Weekday::Sun),
        _ => Err(RecurrenceError::InvalidToken),
    }
}

fn parse_until(value: &str) -> Result<NaiveDate, RecurrenceError> {
    if value.len() != 8 {
        return Err(RecurrenceError::InvalidToken);
    }
    let year: i32 = value[0..4]
        .parse()
        .map_err(|_| RecurrenceError::InvalidToken)?;
    let month: u32 = value[4..6]
        .parse()
        .map_err(|_| RecurrenceError::InvalidToken)?;
    let day: u32 = value[6..8]
        .parse()
        .map_err(|_| RecurrenceError::InvalidToken)?;
    NaiveDate::from_ymd_opt(year, month, day).ok_or(RecurrenceError::InvalidToken)
}

fn event_overlaps_range(event: &PlannedEvent, range: &DateTimeRange) -> bool {
    event.start <= range.end && event.end >= range.start
}

fn duration_between(start: OrbitalDateTime, end: OrbitalDateTime) -> chrono::TimeDelta {
    end.instant() - start.instant()
}

fn occurrence_end(start: OrbitalDateTime, master: &PlannedEvent) -> OrbitalDateTime {
    let delta = duration_between(master.start, master.end);
    OrbitalDateTime::from_instant(start.instant() + delta, master.end.timezone())
}

fn make_instance(master: &PlannedEvent, occurrence_start: OrbitalDateTime) -> PlannedEvent {
    PlannedEvent {
        id: instance_id(&master.id, occurrence_start),
        title: master.title.clone(),
        start: occurrence_start,
        end: occurrence_end(occurrence_start, master),
        resource_id: master.resource_id.clone(),
        recurrence_rule: None,
        color: master.color.clone(),
        is_draggable: master.is_draggable,
        is_resizable: master.is_resizable,
        is_editable: master.is_editable,
    }
}

fn within_until(occurrence_start: OrbitalDateTime, until: NaiveDate) -> bool {
    occurrence_start.wall_date().is_some_and(|d| d <= until)
}

fn week_start_for(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_sunday();
    date.checked_sub_days(chrono::Days::new(weekday as u64))
        .unwrap_or(date)
}

fn date_for_weekday(week_start: NaiveDate, weekday: Weekday) -> NaiveDate {
    let offset = weekday.num_days_from_sunday();
    week_start
        .checked_add_days(chrono::Days::new(offset as u64))
        .unwrap_or(week_start)
}

fn occurrence_at(
    date: NaiveDate,
    hour: u32,
    minute: u32,
    second: u32,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)?.apply_hms(hour, minute, second)
}

/// Expand a single recurring master into display instances within `range`.
pub fn expand_recurring_event(master: &PlannedEvent, range: &DateTimeRange) -> Vec<PlannedEvent> {
    let Some(rule_str) = master.recurrence_rule.as_deref() else {
        return if event_overlaps_range(master, range) {
            vec![master.clone()]
        } else {
            Vec::new()
        };
    };

    let Ok(rule) = parse_recurrence_rule(rule_str) else {
        return if event_overlaps_range(master, range) {
            vec![master.clone()]
        } else {
            Vec::new()
        };
    };

    expand_with_rule(master, &rule, range)
}

fn expand_with_rule(
    master: &PlannedEvent,
    rule: &RecurrenceRule,
    range: &DateTimeRange,
) -> Vec<PlannedEvent> {
    let tz = master.start.timezone();
    let (hour, minute, second) = master.start.hour_minute_second().unwrap_or((0, 0, 0));

    let mut out = Vec::new();
    let mut emitted = 0_u32;

    match rule.freq {
        RecurrenceFreq::Daily => {
            let mut cursor = master
                .start
                .wall_date()
                .unwrap_or_else(|| master.start.instant().date_naive());
            while emitted < MAX_INSTANCES {
                if let Some(count) = rule.count {
                    if emitted >= count {
                        break;
                    }
                }
                if let Some(until) = rule.until {
                    if cursor > until {
                        break;
                    }
                }

                if let Some(occurrence_start) = occurrence_at(cursor, hour, minute, second, tz) {
                    if occurrence_start >= master.start
                        && rule.until.is_none_or(|u| within_until(occurrence_start, u))
                    {
                        let instance = make_instance(master, occurrence_start);
                        if event_overlaps_range(&instance, range) {
                            out.push(instance);
                        }
                        emitted += 1;
                        if rule.count.is_some_and(|c| emitted >= c) {
                            break;
                        }
                    }
                    if occurrence_start > range.end {
                        break;
                    }
                }

                cursor = match cursor.checked_add_days(chrono::Days::new(rule.interval as u64)) {
                    Some(next) => next,
                    None => break,
                };
            }
        }
        RecurrenceFreq::Weekly => {
            let by_day = if rule.by_day.is_empty() {
                vec![master
                    .start
                    .wall_date()
                    .map(|d| d.weekday())
                    .unwrap_or(Weekday::Mon)]
            } else {
                rule.by_day.clone()
            };

            let master_week = week_start_for(
                master
                    .start
                    .wall_date()
                    .unwrap_or_else(|| master.start.instant().date_naive()),
            );
            let mut week_index = 0_u32;

            'weeks: while emitted < MAX_INSTANCES {
                if let Some(count) = rule.count {
                    if emitted >= count {
                        break;
                    }
                }

                let week_start = master_week
                    .checked_add_days(chrono::Days::new((week_index * rule.interval * 7) as u64))
                    .unwrap_or(master_week);

                if let Some(until) = rule.until {
                    if week_start > until {
                        break;
                    }
                }

                for weekday in &by_day {
                    let date = date_for_weekday(week_start, *weekday);
                    if let Some(until) = rule.until {
                        if date > until {
                            continue;
                        }
                    }

                    if let Some(occurrence_start) = occurrence_at(date, hour, minute, second, tz) {
                        if occurrence_start < master.start {
                            continue;
                        }
                        if occurrence_start > range.end {
                            break 'weeks;
                        }

                        let instance = make_instance(master, occurrence_start);
                        if event_overlaps_range(&instance, range) {
                            out.push(instance);
                        }
                        emitted += 1;
                        if rule.count.is_some_and(|c| emitted >= c) {
                            break 'weeks;
                        }
                        if emitted >= MAX_INSTANCES {
                            break 'weeks;
                        }
                    }
                }

                week_index += 1;
                if week_index > 520 {
                    break;
                }
            }
        }
    }

    out
}

/// Expand all masters for display; gated by [`SchedulerFeatures::RECURRING_EVENTS`].
pub fn expand_recurring_events(
    masters: &[PlannedEvent],
    range: &DateTimeRange,
    features: SchedulerFeatures,
) -> Vec<PlannedEvent> {
    let mut out = Vec::new();
    for master in masters {
        if master.recurrence_rule.is_some()
            && features.contains(SchedulerFeatures::RECURRING_EVENTS)
        {
            out.extend(expand_recurring_event(master, range));
        } else if event_overlaps_range(master, range) {
            out.push(master.clone());
        }
    }
    out
}

/// Resolve the event list shown in the calendar for the active view window.
pub fn resolve_display_events(
    masters: &[PlannedEvent],
    visible_date: OrbitalDateTime,
    view: SchedulerView,
    display_tz: DatetimeTimezone,
    features: SchedulerFeatures,
) -> Vec<PlannedEvent> {
    let Some(visible) = visible_range(visible_date, view, display_tz) else {
        return Vec::new();
    };
    expand_recurring_events(masters, &visible.query, features)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn master_on(
        day: NaiveDate,
        start_hms: (u32, u32, u32),
        end_hms: (u32, u32, u32),
    ) -> PlannedEvent {
        let tz = DatetimeTimezone::Utc;
        let start = OrbitalDateTime::from_naive_date(day, tz)
            .unwrap()
            .apply_hms(start_hms.0, start_hms.1, start_hms.2)
            .expect("start");
        let end = OrbitalDateTime::from_naive_date(day, tz)
            .unwrap()
            .apply_hms(end_hms.0, end_hms.1, end_hms.2)
            .expect("end");
        PlannedEvent::new("weekly-standup", "Standup", start, end)
    }

    fn week_range() -> DateTimeRange {
        let tz = DatetimeTimezone::Utc;
        let anchor =
            OrbitalDateTime::from_naive_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), tz)
                .expect("valid");
        visible_range(anchor, SchedulerView::Week, tz)
            .expect("range")
            .query
    }

    #[test]
    fn parse_daily_rule() {
        let rule = parse_recurrence_rule("FREQ=DAILY;INTERVAL=2;COUNT=3").expect("parsed");
        assert_eq!(rule.freq, RecurrenceFreq::Daily);
        assert_eq!(rule.interval, 2);
        assert_eq!(rule.count, Some(3));
    }

    #[test]
    fn parse_weekly_byday() {
        let rule = parse_recurrence_rule("FREQ=WEEKLY;BYDAY=MO,WE,FR").expect("parsed");
        assert_eq!(rule.freq, RecurrenceFreq::Weekly);
        assert_eq!(rule.by_day.len(), 3);
    }

    #[test]
    fn reject_unsupported_freq() {
        assert_eq!(
            parse_recurrence_rule("FREQ=MONTHLY"),
            Err(RecurrenceError::UnsupportedFreq)
        );
    }

    #[test]
    fn daily_expansion_respects_count() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let mut master = master_on(day, (9, 0, 0), (9, 30, 0));
        master.recurrence_rule = Some("FREQ=DAILY;INTERVAL=2;COUNT=3".into());
        let range = week_range();
        let extended_end = OrbitalDateTime::from_naive_date(
            NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(),
            DatetimeTimezone::Utc,
        )
        .expect("valid")
        .apply_hms(23, 59, 59)
        .expect("end");
        let extended = DateTimeRange::new(range.start, extended_end);
        let instances = expand_recurring_event(&master, &extended);
        assert_eq!(instances.len(), 3);
        assert!(instances
            .iter()
            .all(|e| e.id.starts_with("weekly-standup::")));
    }

    #[test]
    fn weekly_expansion_emits_byday_instances() {
        let monday = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let mut master = master_on(monday, (9, 0, 0), (9, 30, 0));
        master.recurrence_rule = Some("FREQ=WEEKLY;BYDAY=MO,WE,FR".into());
        let instances = expand_recurring_event(&master, &week_range());
        assert!(instances.len() >= 3);
    }

    #[test]
    fn instance_id_round_trip() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let master = master_on(day, (9, 0, 0), (9, 30, 0));
        let id = instance_id(&master.id, master.start);
        assert_eq!(master_event_id(&id), "weekly-standup");
        let parsed = occurrence_start_from_instance(&id, master.start.timezone()).expect("parsed");
        assert_eq!(parsed, master.start);
    }

    #[test]
    fn expansion_gated_without_feature_flag() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let mut master = master_on(day, (9, 0, 0), (9, 30, 0));
        master.recurrence_rule = Some("FREQ=WEEKLY;BYDAY=MO,WE,FR".into());
        let out =
            expand_recurring_events(&[master.clone()], &week_range(), SchedulerFeatures::empty());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, master.id);
    }

    #[test]
    fn until_boundary_excludes_later_dates() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let mut master = master_on(day, (9, 0, 0), (9, 30, 0));
        master.recurrence_rule = Some("FREQ=DAILY;UNTIL=20250102".into());
        let range = week_range();
        let instances = expand_recurring_event(&master, &range);
        assert!(instances.len() <= 2);
    }
}
