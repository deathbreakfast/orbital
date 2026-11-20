mod format;
mod styles;
mod timestamp;
mod value;

pub use styles::{timestamp_styles, TimestampStyle};
pub use timestamp::{Timestamp, TimestampHoverCard};
pub use value::{ResolvedTimestamp, TimestampValue};

#[cfg(feature = "preview")]
pub use timestamp::{
    TIMESTAMP_DESCRIPTION, TIMESTAMP_DOC, TIMESTAMP_PREVIEW_REGISTRATION, TIMESTAMP_PROPS,
};

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use orbital_base_components::DatetimeTimezone;

    use super::format::{
        format_label, format_relative, iso_row, local_row, machine_datetime, offset_label, utc_row,
        zone_for, zone_row, ZoneInfo,
    };
    use super::{TimestampStyle, TimestampValue};

    const PDT: i32 = -7 * 3600;

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 27, 8, 42, 17).unwrap()
    }

    fn pdt() -> ZoneInfo {
        ZoneInfo {
            short: "PDT".to_string(),
            iana: Some("America/Los_Angeles".to_string()),
            ..ZoneInfo::fixed(PDT)
        }
    }

    #[test]
    fn rfc3339_with_micros_and_offset() {
        let value = TimestampValue::from("2026-10-27T01:42:17.204913-07:00");
        let resolved = value.resolve().expect("parses");
        assert_eq!(resolved.utc.timestamp(), at().timestamp());
        assert_eq!(resolved.utc.timestamp_subsec_micros(), 204_913);
        assert!(resolved.has_subsec);
        assert_eq!(
            resolved.original.as_deref(),
            Some("2026-10-27T01:42:17.204913-07:00")
        );
    }

    #[test]
    fn rfc3339_zulu() {
        let resolved = TimestampValue::from("2026-10-27T08:42:17Z")
            .resolve()
            .expect("parses");
        assert_eq!(resolved.utc, at());
        assert!(!resolved.has_subsec);
    }

    #[test]
    fn unix_seconds_never_show_millis() {
        let resolved = TimestampValue::UnixSeconds(at().timestamp())
            .resolve()
            .expect("valid");
        assert_eq!(resolved.utc, at());
        assert!(!resolved.has_subsec);
        assert_eq!(
            utc_row(resolved.utc, resolved.has_subsec),
            "2026-10-27 08:42:17 UTC"
        );
    }

    #[test]
    fn unix_millis_show_millis_only_when_present() {
        let whole = TimestampValue::UnixMillis(at().timestamp_millis())
            .resolve()
            .expect("valid");
        assert!(!whole.has_subsec);
        let frac = TimestampValue::UnixMillis(at().timestamp_millis() + 204)
            .resolve()
            .expect("valid");
        assert!(frac.has_subsec);
        assert_eq!(
            utc_row(frac.utc, frac.has_subsec),
            "2026-10-27 08:42:17.204 UTC"
        );
        assert_eq!(
            machine_datetime(frac.utc, frac.has_subsec),
            "2026-10-27T08:42:17.204Z"
        );
    }

    #[test]
    fn utc_datetime_converts() {
        assert_eq!(TimestampValue::from(at()), TimestampValue::Utc(at()));
        assert_eq!(
            TimestampValue::from(at()).resolve().expect("valid").utc,
            at()
        );
        assert_eq!(
            TimestampValue::from("x".to_string()),
            TimestampValue::Rfc3339("x".to_string())
        );
    }

    #[test]
    fn unparseable_string_passes_through() {
        let value = TimestampValue::from("last Tuesday");
        assert!(value.resolve().is_none());
        assert_eq!(value.raw_text(), "last Tuesday");
        assert!(TimestampValue::from("2026-10-27").resolve().is_none());
    }

    #[test]
    fn date_time_style() {
        let label = format_label(at(), TimestampStyle::DateTime, &pdt(), at());
        assert_eq!(label, "Oct 27, 2026, 1:42 AM PDT");
        let label = format_label(at(), TimestampStyle::DateTime, &ZoneInfo::utc(), at());
        assert_eq!(label, "Oct 27, 2026, 8:42 AM UTC");
    }

    #[test]
    fn date_style_keeps_the_year_in_the_current_year() {
        let label = format_label(at(), TimestampStyle::Date, &pdt(), at());
        assert_eq!(label, "Oct 27, 2026");
    }

    #[test]
    fn time_style() {
        let zone = zone_for(DatetimeTimezone::FixedOffset(PDT), at());
        assert_eq!(
            format_label(at(), TimestampStyle::Time, &zone, at()),
            "1:42 AM GMT-7"
        );
        assert_eq!(
            format_label(at(), TimestampStyle::Time, &pdt(), at()),
            "1:42 AM PDT"
        );
    }

    #[test]
    fn relative_style() {
        let now = at() + chrono::Duration::hours(3);
        assert_eq!(
            format_label(at(), TimestampStyle::Relative, &pdt(), now),
            "3h ago"
        );
        assert_eq!(
            format_relative(at(), at() + chrono::Duration::seconds(30)),
            "just now"
        );
        assert_eq!(
            format_relative(at(), at() - chrono::Duration::minutes(5)),
            "in 5m"
        );
        assert_eq!(
            format_relative(at(), at() + chrono::Duration::days(2)),
            "2d ago"
        );
        assert_eq!(
            format_relative(at(), at() + chrono::Duration::days(21)),
            "3w ago"
        );
        assert_eq!(
            format_relative(at(), at() + chrono::Duration::days(400)),
            "1y ago"
        );
    }

    #[test]
    fn fixed_offset_zone_names() {
        assert_eq!(ZoneInfo::fixed(2 * 3600).short, "GMT+2");
        assert_eq!(ZoneInfo::fixed(19_800).short, "GMT+5:30");
        assert_eq!(zone_for(DatetimeTimezone::Utc, at()).short, "UTC");
    }

    #[test]
    fn card_rows() {
        assert_eq!(
            local_row(at(), &pdt(), false),
            "Tue, Oct 27, 2026, 1:42:17 AM PDT"
        );
        let micros = at() + chrono::Duration::microseconds(204_913);
        assert_eq!(
            local_row(micros, &pdt(), true),
            "Tue, Oct 27, 2026, 1:42:17.204 AM PDT"
        );
        assert_eq!(zone_row(&pdt()), "America/Los_Angeles · UTC\u{2212}07:00");
        assert_eq!(offset_label(ZoneInfo::utc().offset), "UTC+00:00");
        assert_eq!(utc_row(micros, true), "2026-10-27 08:42:17.204 UTC");
    }

    #[test]
    fn iso_row_prefers_the_original_text() {
        assert_eq!(
            iso_row(at(), Some("2026-10-27T01:42:17-07:00"), false),
            "2026-10-27T01:42:17-07:00"
        );
        assert_eq!(iso_row(at(), None, false), "2026-10-27T08:42:17Z");
    }
}
