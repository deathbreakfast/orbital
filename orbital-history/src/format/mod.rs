//! Pure formatting helpers (no I/O).

mod markdown;

pub use markdown::render_history_markdown;

use chrono::{DateTime, Utc};
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use crate::types::{HistoryChange, HistoryDateBucket, HistoryEntry, HistoryListItem, HistoryLocale};

/// Default max display length for substituted values.
pub const DEFAULT_TRUNCATE_LEN: usize = 80;

/// Truncate a display string with a Unicode ellipsis when over `max_len`.
pub fn truncate_display_value(value: &str, max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }
    let count = value.chars().count();
    if count <= max_len {
        return value.to_string();
    }
    let keep = max_len.saturating_sub(1);
    let mut out: String = value.chars().take(keep).collect();
    out.push('…');
    out
}

/// Format a change payload using locale templates.
pub fn format_change(change: &HistoryChange, locale: &HistoryLocale) -> String {
    match change {
        HistoryChange::Created => locale.format_created(),
        HistoryChange::Deleted { label } => locale.format_deleted(label),
        HistoryChange::FieldDiff {
            field,
            old_value,
            new_value,
        } => locale.format_field_diff(field, old_value, new_value),
        HistoryChange::FieldDiffs { fields } => {
            if fields.is_empty() {
                return locale.format_field_diffs_header(0);
            }
            let header = locale.format_field_diffs_header(fields.len());
            let lines: Vec<String> = fields
                .iter()
                .map(|f| locale.format_field_diff(&f.field, &f.old_value, &f.new_value))
                .collect();
            format!("{header}: {}", lines.join("; "))
        }
        HistoryChange::Custom { summary } => {
            truncate_display_value(summary, DEFAULT_TRUNCATE_LEN)
        }
        HistoryChange::Markdown { body, .. } => truncate_display_value(body, DEFAULT_TRUNCATE_LEN),
    }
}

/// Assign a relative date bucket for `changed_at` relative to `now` (UTC calendar days).
pub fn history_date_bucket(changed_at: DateTime<Utc>, now: DateTime<Utc>) -> HistoryDateBucket {
    history_date_bucket_in_tz(changed_at, now, DatetimeTimezone::Utc)
}

/// Assign a relative date bucket using wall-clock calendar days in `tz`.
pub fn history_date_bucket_in_tz(
    changed_at: DateTime<Utc>,
    now: DateTime<Utc>,
    tz: DatetimeTimezone,
) -> HistoryDateBucket {
    let today = OrbitalDateTime::from_instant(now, tz)
        .wall_date()
        .unwrap_or_else(|| now.date_naive());
    let day = OrbitalDateTime::from_instant(changed_at, tz)
        .wall_date()
        .unwrap_or_else(|| changed_at.date_naive());
    let delta = (today - day).num_days();
    match delta {
        0 => HistoryDateBucket::Today,
        1 => HistoryDateBucket::Yesterday,
        2..=7 => HistoryDateBucket::Last7Days,
        8..=30 => HistoryDateBucket::Last30Days,
        _ => HistoryDateBucket::Older,
    }
}

/// Project entries into a list with date-bucket dividers (newest-first input, UTC days).
pub fn with_date_dividers(entries: &[HistoryEntry], now: DateTime<Utc>) -> Vec<HistoryListItem> {
    with_date_dividers_in_tz(entries, now, DatetimeTimezone::Utc)
}

/// Project entries into a list with date-bucket dividers in `tz`.
pub fn with_date_dividers_in_tz(
    entries: &[HistoryEntry],
    now: DateTime<Utc>,
    tz: DatetimeTimezone,
) -> Vec<HistoryListItem> {
    let mut out = Vec::with_capacity(entries.len().saturating_mul(2));
    let mut prev_bucket: Option<HistoryDateBucket> = None;
    for entry in entries {
        let bucket = history_date_bucket_in_tz(entry.changed_at, now, tz);
        if prev_bucket != Some(bucket) {
            out.push(HistoryListItem::Divider(bucket));
            prev_bucket = Some(bucket);
        }
        out.push(HistoryListItem::Entry(entry.clone()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    use crate::types::HistoryFieldDiff;

    fn ts(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
    }

    #[test]
    fn truncate_short_unchanged() {
        assert_eq!(truncate_display_value("hello", 80), "hello");
    }

    #[test]
    fn truncate_long_with_ellipsis() {
        let long = "a".repeat(90);
        let out = truncate_display_value(&long, 80);
        assert_eq!(out.chars().count(), 80);
        assert!(out.ends_with('…'));
    }

    #[test]
    fn format_field_diff_quotes() {
        let locale = HistoryLocale::english();
        let s = format_change(
            &HistoryChange::FieldDiff {
                field: "name".into(),
                old_value: "A".into(),
                new_value: "B".into(),
            },
            &locale,
        );
        assert_eq!(s, "changed name from \"A\" to \"B\"");
    }

    #[test]
    fn format_deleted_quotes() {
        let locale = HistoryLocale::english();
        let s = format_change(
            &HistoryChange::Deleted {
                label: "Widget".into(),
            },
            &locale,
        );
        assert_eq!(s, "deleted \"Widget\"");
    }

    #[test]
    fn format_field_diffs_summary() {
        let locale = HistoryLocale::english();
        let s = format_change(
            &HistoryChange::FieldDiffs {
                fields: vec![
                    HistoryFieldDiff {
                        field: "a".into(),
                        old_value: "1".into(),
                        new_value: "2".into(),
                    },
                    HistoryFieldDiff {
                        field: "b".into(),
                        old_value: "3".into(),
                        new_value: "4".into(),
                    },
                ],
            },
            &locale,
        );
        assert!(s.starts_with("changed 2 fields:"));
        assert!(s.contains("changed a from \"1\" to \"2\""));
    }

    #[test]
    fn buckets_today_yesterday_older() {
        let now = ts(2026, 7, 3, 12, 0);
        assert_eq!(
            history_date_bucket(ts(2026, 7, 3, 8, 0), now),
            HistoryDateBucket::Today
        );
        assert_eq!(
            history_date_bucket(ts(2026, 7, 2, 8, 0), now),
            HistoryDateBucket::Yesterday
        );
        assert_eq!(
            history_date_bucket(ts(2026, 6, 28, 8, 0), now),
            HistoryDateBucket::Last7Days
        );
        assert_eq!(
            history_date_bucket(ts(2026, 6, 10, 8, 0), now),
            HistoryDateBucket::Last30Days
        );
        assert_eq!(
            history_date_bucket(ts(2026, 1, 1, 8, 0), now),
            HistoryDateBucket::Older
        );
    }

    #[test]
    fn with_date_dividers_inserts_on_transition() {
        let now = ts(2026, 7, 3, 12, 0);
        let entries = vec![
            HistoryEntry {
                id: "1".into(),
                kind: "field_diff".into(),
                changed_at: ts(2026, 7, 3, 10, 0),
                actor: crate::types::HistoryActor::System,
                change: HistoryChange::Created,
            },
            HistoryEntry {
                id: "2".into(),
                kind: "field_diff".into(),
                changed_at: ts(2026, 7, 2, 10, 0),
                actor: crate::types::HistoryActor::System,
                change: HistoryChange::Created,
            },
        ];
        let items = with_date_dividers(&entries, now);
        assert!(matches!(
            items[0],
            HistoryListItem::Divider(HistoryDateBucket::Today)
        ));
        assert!(matches!(items[1], HistoryListItem::Entry(_)));
        assert!(matches!(
            items[2],
            HistoryListItem::Divider(HistoryDateBucket::Yesterday)
        ));
    }

    #[test]
    fn compact_time_respects_timezone() {
        let locale = HistoryLocale::english();
        let at = ts(2026, 7, 3, 18, 0);
        let utc = locale.format_compact_time(at, DatetimeTimezone::Utc);
        let west = locale.format_compact_time(at, DatetimeTimezone::FixedOffset(-8 * 3600));
        assert_ne!(utc, west);
    }

    #[test]
    fn buckets_respect_fixed_offset_wall_clock() {
        // UTC-8 (e.g. US Pacific standard).
        let pacific = DatetimeTimezone::FixedOffset(-8 * 3600);
        let now = ts(2026, 7, 3, 12, 0); // 04:00 Pacific on July 3
        let evening_utc = ts(2026, 7, 3, 2, 0); // 18:00 Pacific on July 2

        assert_eq!(
            history_date_bucket(evening_utc, now),
            HistoryDateBucket::Today
        );
        assert_eq!(
            history_date_bucket_in_tz(evening_utc, now, pacific),
            HistoryDateBucket::Yesterday
        );
    }
}
