use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

use super::timezone::DatetimeTimezone;

/// Canonical instant + explicit timezone context for picker/scheduler logic.
///
/// # Timezone
///
/// Each value stores a UTC [`instant`](Self::instant) and an explicit [`timezone`](Self::timezone)
/// for wall-clock interpretation. Calendar operations (`start_of_day`, `same_calendar_day`,
/// `apply_hms`) use the value's timezone. Set
/// `appearance.timezone` on pickers to control display and parsing; the bound value should
/// use a matching timezone for consistent round-trips.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrbitalDateTime {
    /// UTC instant — validated chrono `DateTime<Utc>` internally.
    instant: DateTime<Utc>,
    /// Wall-clock / display interpretation from the stored timezone.
    timezone: DatetimeTimezone,
}

impl OrbitalDateTime {
    pub fn from_instant(instant: DateTime<Utc>, timezone: DatetimeTimezone) -> Self {
        Self { instant, timezone }
    }

    pub fn utc_now(timezone: DatetimeTimezone) -> Self {
        Self {
            instant: Utc::now(),
            timezone,
        }
    }

    /// Calendar day start at 00:00:00 in this value's timezone.
    pub fn start_of_day(self) -> Self {
        let date = self.wall_date().unwrap_or_else(|| Utc::now().date_naive());
        Self::from_naive_date(date, self.timezone).unwrap_or(self)
    }

    /// Replace time-of-day in this value's timezone, keeping the calendar day.
    pub fn apply_hms(self, hour: u32, minute: u32, second: u32) -> Option<Self> {
        let date = self.wall_date()?;
        let naive = date.and_hms_opt(hour, minute, second)?;
        Self::from_naive_datetime(naive, self.timezone)
    }

    /// Whether both instants fall on the same calendar day in `self.timezone`.
    pub fn same_calendar_day(self, other: Self) -> bool {
        match (self.wall_date(), other.wall_date_in(self.timezone)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    pub fn hour_minute_second(self) -> Option<(u32, u32, u32)> {
        let naive = self.wall_naive()?;
        Some((naive.hour(), naive.minute(), naive.second()))
    }

    pub fn instant(&self) -> DateTime<Utc> {
        self.instant
    }

    pub fn timezone(&self) -> DatetimeTimezone {
        self.timezone
    }

    pub fn from_naive_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<Self> {
        Self::from_naive_datetime(date.and_hms_opt(0, 0, 0)?, timezone)
    }

    pub(crate) fn from_naive_datetime(
        naive: NaiveDateTime,
        timezone: DatetimeTimezone,
    ) -> Option<Self> {
        let instant = match timezone {
            DatetimeTimezone::Local => Local
                .from_local_datetime(&naive)
                .single()?
                .with_timezone(&Utc),
            DatetimeTimezone::Utc => Utc.from_utc_datetime(&naive),
            DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)?
                .from_local_datetime(&naive)
                .single()?
                .with_timezone(&Utc),
        };
        Some(Self { instant, timezone })
    }

    fn wall_naive(&self) -> Option<NaiveDateTime> {
        match self.timezone {
            DatetimeTimezone::Local => Some(self.instant.with_timezone(&Local).naive_local()),
            DatetimeTimezone::Utc => Some(self.instant.naive_utc()),
            DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
                .map(|offset| self.instant.with_timezone(&offset).naive_local()),
        }
    }

    pub fn wall_date(&self) -> Option<NaiveDate> {
        self.wall_naive().map(|dt| dt.date())
    }

    fn wall_date_in(&self, timezone: DatetimeTimezone) -> Option<NaiveDate> {
        Self {
            instant: self.instant,
            timezone,
        }
        .wall_date()
    }
}

#[cfg(test)]
mod tests {
    use super::OrbitalDateTime;
    use crate::form::DatetimeTimezone;
    use chrono::{TimeZone, Utc};

    #[test]
    fn start_of_day_utc_resets_time() {
        let source = Utc
            .with_ymd_and_hms(2025, 1, 2, 18, 45, 12)
            .single()
            .expect("valid dt");
        let dt = OrbitalDateTime::from_instant(source, DatetimeTimezone::Utc);
        let sod = dt.start_of_day();
        let expected = Utc
            .with_ymd_and_hms(2025, 1, 2, 0, 0, 0)
            .single()
            .expect("valid dt");
        assert_eq!(sod.instant(), expected);
        assert_eq!(sod.timezone(), DatetimeTimezone::Utc);
    }

    #[test]
    fn apply_hms_utc_replaces_time_component() {
        let source = Utc
            .with_ymd_and_hms(2025, 2, 11, 1, 2, 3)
            .single()
            .expect("valid dt");
        let dt = OrbitalDateTime::from_instant(source, DatetimeTimezone::Utc);
        let updated = dt.apply_hms(22, 10, 9).expect("updated dt");
        let expected = Utc
            .with_ymd_and_hms(2025, 2, 11, 22, 10, 9)
            .single()
            .expect("valid dt");
        assert_eq!(updated.instant(), expected);
    }

    #[test]
    fn same_calendar_day_utc_checks_date_only() {
        let a = Utc
            .with_ymd_and_hms(2025, 6, 4, 0, 0, 1)
            .single()
            .expect("valid dt");
        let b = Utc
            .with_ymd_and_hms(2025, 6, 4, 23, 59, 59)
            .single()
            .expect("valid dt");
        let c = Utc
            .with_ymd_and_hms(2025, 6, 5, 0, 0, 0)
            .single()
            .expect("valid dt");

        let tz = DatetimeTimezone::Utc;
        let a_dt = OrbitalDateTime::from_instant(a, tz);
        let b_dt = OrbitalDateTime::from_instant(b, tz);
        let c_dt = OrbitalDateTime::from_instant(c, tz);

        assert!(a_dt.same_calendar_day(b_dt));
        assert!(!a_dt.same_calendar_day(c_dt));
    }

    #[test]
    fn fixed_offset_start_of_day() {
        let tz = DatetimeTimezone::FixedOffset(-5 * 3600);
        let source = Utc
            .with_ymd_and_hms(2025, 3, 15, 10, 30, 0)
            .single()
            .expect("valid dt");
        let dt = OrbitalDateTime::from_instant(source, tz);
        let sod = dt.start_of_day();
        assert_eq!(sod.hour_minute_second(), Some((0, 0, 0)));
    }
}
