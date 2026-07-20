mod convert;
mod day_bounds;
mod error;
mod format;
mod orbital_datetime;
mod picker_shortcut;
mod timezone;

pub use convert::{
    ToDataValue, ToIso8601, ToUnixSeconds, TryFromDataValue, TryFromIso8601, TryFromUnixSeconds,
};
pub use day_bounds::{is_datetime_out_of_range, is_day_disabled};
pub use error::DatetimeError;
pub use format::{format_datetime, format_unix, parse_datetime, parse_to_unix, DatetimeFormat};
pub use orbital_datetime::OrbitalDateTime;
pub use picker_shortcut::PickerShortcut;
pub use timezone::DatetimeTimezone;

#[cfg(test)]
mod format_tests {
    use super::{
        format_datetime, parse_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime,
        TryFromUnixSeconds,
    };

    #[test]
    fn format_parse_datetime_iso_round_trip() {
        let dt = OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Utc)
            .expect("valid");
        let formatted = format_datetime(dt, DatetimeFormat::IsoDate);
        let parsed = parse_datetime(&formatted, DatetimeFormat::IsoDate, DatetimeTimezone::Utc)
            .expect("parsed");
        assert!(dt.same_calendar_day(parsed));
    }

    #[test]
    fn format_parse_datetime_us_round_trip() {
        let dt = OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Utc)
            .expect("valid");
        let formatted = format_datetime(dt, DatetimeFormat::UsDate);
        let parsed = parse_datetime(&formatted, DatetimeFormat::UsDate, DatetimeTimezone::Utc)
            .expect("parsed");
        assert!(dt.same_calendar_day(parsed));
    }
}
