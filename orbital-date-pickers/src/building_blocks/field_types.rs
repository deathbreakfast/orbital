use leptos::prelude::*;
use orbital_base_components::{
    DatePickerRule, DatetimeFormat, DatetimeTimezone, OptionBind, OrbitalDateTime, PickerShortcut,
    Placement, TryFromUnixSeconds,
};
use orbital_core_components::CalendarDayRenderer;

use crate::DateTimeRange;

const LEGACY_BIND_TZ: DatetimeTimezone = DatetimeTimezone::Local;

pub fn orbital_from_i64(secs: i64) -> Option<OrbitalDateTime> {
    OrbitalDateTime::try_from_unix_seconds(secs, LEGACY_BIND_TZ).ok()
}

/// Value binding for [`super::date_field::DateField`].
#[derive(Default)]
pub struct DateFieldBind {
    /// Two-way [`OrbitalDateTime`] date value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` for the first segment; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl DateFieldBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DateFieldBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DateFieldBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for DateFieldBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DateFieldBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for DateFieldBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for DateFieldBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Formatting and interaction options for [`super::date_field::DateField`].
pub struct DateFieldAppearance {
    /// Segmented date mask (`IsoDate` or `UsDate`).
    pub format: Signal<DatetimeFormat>,
    /// Timezone used when parsing typed segments.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for DateFieldAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::default()),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl From<DatetimeFormat> for DateFieldAppearance {
    fn from(format: DatetimeFormat) -> Self {
        Self {
            format: Signal::from(format),
            ..Default::default()
        }
    }
}

impl From<DatetimeTimezone> for DateFieldAppearance {
    fn from(timezone: DatetimeTimezone) -> Self {
        Self {
            timezone: Signal::from(timezone),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::time_field::TimeField`].
#[derive(Default)]
pub struct TimeFieldBind {
    /// Two-way [`OrbitalDateTime`] time value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` for the first segment; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl TimeFieldBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for TimeFieldBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for TimeFieldBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for TimeFieldBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for TimeFieldBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for TimeFieldBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for TimeFieldBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Formatting and interaction options for [`super::time_field::TimeField`].
pub struct TimeFieldAppearance {
    /// Segmented time mask (`Time24` or `Time12`).
    pub format: Signal<DatetimeFormat>,
    /// Calendar day anchor when resolving time-only segments.
    pub reference_date: Signal<OrbitalDateTime>,
    /// Wall-clock timezone for segment parsing and display.
    pub timezone: Signal<DatetimeTimezone>,
    /// Minute increment hint (rounding deferred; default `1`).
    pub minute_step: Signal<u32>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for TimeFieldAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time12),
            reference_date: Signal::from(
                OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
            ),
            timezone: Signal::from(DatetimeTimezone::Local),
            minute_step: Signal::from(1),
            disabled: Signal::from(false),
        }
    }
}

impl TimeFieldAppearance {
    pub fn time24() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::date_calendar::DateCalendar`].
#[derive(Default)]
pub struct DateCalendarBind {
    /// Two-way selected calendar day.
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name when nested in Field.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<DatePickerRule>,
}

impl DateCalendarBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DateCalendarBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DateCalendarBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for DateCalendarBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DateCalendarBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

/// Visual and range options for [`super::date_calendar::DateCalendar`].
pub struct DateCalendarAppearance {
    /// Timezone for start-of-day conversion.
    pub timezone: Signal<DatetimeTimezone>,
    /// Earliest selectable day (inclusive).
    pub min_date: Signal<Option<OrbitalDateTime>>,
    /// Latest selectable day (inclusive).
    pub max_date: Signal<Option<OrbitalDateTime>>,
    /// Shortcut preset buttons shown above the calendar grid.
    pub shortcuts: Signal<Vec<PickerShortcut>>,
    /// Disables day selection.
    pub disabled: Signal<bool>,
    /// Optional custom day cell renderer forwarded to core [`Calendar`].
    pub day: Option<CalendarDayRenderer>,
}

impl Default for DateCalendarAppearance {
    fn default() -> Self {
        Self {
            timezone: Signal::from(DatetimeTimezone::Local),
            min_date: Signal::from(None),
            max_date: Signal::from(None),
            shortcuts: Signal::from(Vec::new()),
            disabled: Signal::from(false),
            day: None,
        }
    }
}

impl From<DatetimeTimezone> for DateCalendarAppearance {
    fn from(timezone: DatetimeTimezone) -> Self {
        Self {
            timezone: Signal::from(timezone),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::date_time_field::DateTimeField`].
#[derive(Default)]
pub struct DateTimeFieldBind {
    /// Two-way [`OrbitalDateTime`] date-time value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` for the first segment; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl DateTimeFieldBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DateTimeFieldBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DateTimeFieldBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for DateTimeFieldBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DateTimeFieldBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for DateTimeFieldBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for DateTimeFieldBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Formatting and interaction options for [`super::date_time_field::DateTimeField`].
pub struct DateTimeFieldAppearance {
    /// Segmented date mask (`IsoDate` or `UsDate`).
    pub date_format: Signal<DatetimeFormat>,
    /// Segmented time mask (`Time24` or `Time12`).
    pub time_format: Signal<DatetimeFormat>,
    /// Timezone used when parsing typed segments.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for DateTimeFieldAppearance {
    fn default() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::default()),
            time_format: Signal::from(DatetimeFormat::Time12),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl DateTimeFieldAppearance {
    pub fn iso_time24() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::IsoDate),
            time_format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`crate::DateTimePicker`].
#[derive(Default)]
pub struct DateTimePickerBind {
    /// Two-way [`OrbitalDateTime`] date-time value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` forwarded to the date field input.
    pub id: MaybeProp<String>,
    /// Form field name forwarded to the date field input.
    pub name: MaybeProp<String>,
}

impl DateTimePickerBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DateTimePickerBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DateTimePickerBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for DateTimePickerBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DateTimePickerBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for DateTimePickerBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for DateTimePickerBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Formatting and panel options for [`crate::DateTimePicker`].
pub struct DateTimePickerAppearance {
    /// Display format for the date field (`IsoDate` or `UsDate`).
    pub date_format: Signal<DatetimeFormat>,
    /// Display format for the time picker (`Time24` or `Time12`).
    pub time_format: Signal<DatetimeFormat>,
    /// Timezone for date parsing and display.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables both date and time pickers.
    pub disabled: Signal<bool>,
    /// Popover placement for the date picker panel.
    pub placement: Signal<Placement>,
}

impl Default for DateTimePickerAppearance {
    fn default() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::default()),
            time_format: Signal::from(DatetimeFormat::Time12),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
            placement: Signal::from(Placement::BottomStart),
        }
    }
}

impl DateTimePickerAppearance {
    pub fn time24() -> Self {
        Self {
            time_format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::time_clock::TimeClock`].
#[derive(Default)]
pub struct TimeClockBind {
    /// Two-way [`OrbitalDateTime`] time value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
}

impl TimeClockBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for TimeClockBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for TimeClockBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self { value }
    }
}

impl From<Option<OrbitalDateTime>> for TimeClockBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for TimeClockBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for TimeClockBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for TimeClockBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Visual and interaction options for [`super::time_clock::TimeClock`].
pub struct TimeClockAppearance {
    /// When true, show a 12-hour dial with AM/PM meridiem controls.
    pub ampm: Signal<bool>,
    /// Minute increment for the minute-selection view (minimum 1).
    pub minute_step: Signal<u32>,
    /// Calendar day anchor when resolving time-only values.
    pub reference_date: Signal<OrbitalDateTime>,
    /// Timezone for wall-clock interpretation.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables dial interaction.
    pub disabled: Signal<bool>,
}

impl Default for TimeClockAppearance {
    fn default() -> Self {
        Self {
            ampm: Signal::from(true),
            minute_step: Signal::from(1),
            reference_date: Signal::from(
                OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
            ),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl TimeClockAppearance {
    pub fn time24() -> Self {
        Self {
            ampm: Signal::from(false),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::digital_clock::DigitalClock`].
#[derive(Default)]
pub struct DigitalClockBind {
    /// Two-way [`OrbitalDateTime`] time value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
}

impl DigitalClockBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DigitalClockBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DigitalClockBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self { value }
    }
}

impl From<Option<OrbitalDateTime>> for DigitalClockBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DigitalClockBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for DigitalClockBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for DigitalClockBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

/// Visual and interaction options for [`super::digital_clock::DigitalClock`].
pub struct DigitalClockAppearance {
    /// Minute increment between scrollable list items (minimum 1; default 30).
    pub time_step: Signal<u32>,
    /// When true, format list labels in 12-hour time with AM/PM.
    pub ampm: Signal<bool>,
    /// Calendar day anchor when resolving time-only values.
    pub reference_date: Signal<OrbitalDateTime>,
    /// Timezone for wall-clock interpretation.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables list selection.
    pub disabled: Signal<bool>,
}

impl Default for DigitalClockAppearance {
    fn default() -> Self {
        Self {
            time_step: Signal::from(30),
            ampm: Signal::from(true),
            reference_date: Signal::from(
                OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
            ),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl DigitalClockAppearance {
    pub fn time24() -> Self {
        Self {
            ampm: Signal::from(false),
            ..Default::default()
        }
    }
}

fn range_bind_from_master(
    value: impl Into<OptionBind<DateTimeRange>>,
) -> OptionBind<DateTimeRange> {
    value.into()
}

/// Value binding for [`super::date_range_calendar::DateRangeCalendar`].
#[derive(Default)]
pub struct DateRangeCalendarBind {
    /// Two-way selected date range.
    pub value: OptionBind<DateTimeRange>,
}

impl DateRangeCalendarBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for DateRangeCalendarBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for DateRangeCalendarBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self { value }
    }
}

impl From<Option<DateTimeRange>> for DateRangeCalendarBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for DateRangeCalendarBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Visual and range options for [`super::date_range_calendar::DateRangeCalendar`].
pub struct DateRangeCalendarAppearance {
    /// Timezone for start-of-day conversion.
    pub timezone: Signal<DatetimeTimezone>,
    /// Earliest selectable day (inclusive).
    pub min_date: Signal<Option<OrbitalDateTime>>,
    /// Latest selectable day (inclusive).
    pub max_date: Signal<Option<OrbitalDateTime>>,
    /// Disables day selection.
    pub disabled: Signal<bool>,
    /// Number of month panels to render (1 or 2).
    pub calendars: Signal<u8>,
    /// Optional custom day cell renderer for range calendar cells.
    pub day: Option<CalendarDayRenderer>,
}

impl Default for DateRangeCalendarAppearance {
    fn default() -> Self {
        Self {
            timezone: Signal::from(DatetimeTimezone::Local),
            min_date: Signal::from(None),
            max_date: Signal::from(None),
            disabled: Signal::from(false),
            calendars: Signal::from(2),
            day: None,
        }
    }
}

/// Value binding for [`super::date_range_field::DateRangeField`].
#[derive(Default)]
pub struct DateRangeFieldBind {
    /// Two-way date range value (`None` when incomplete or empty).
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the first start segment.
    pub id: MaybeProp<String>,
    /// Form field name for the start segment group.
    pub name: MaybeProp<String>,
}

impl DateRangeFieldBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for DateRangeFieldBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for DateRangeFieldBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for DateRangeFieldBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for DateRangeFieldBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Formatting options for [`super::date_range_field::DateRangeField`].
pub struct DateRangeFieldAppearance {
    /// Segmented date mask for both endpoints.
    pub format: Signal<DatetimeFormat>,
    /// Timezone used when parsing typed segments.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for DateRangeFieldAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::default()),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

/// Value binding for [`super::time_range_field::TimeRangeField`].
#[derive(Default)]
pub struct TimeRangeFieldBind {
    /// Two-way time range value (`None` when incomplete or empty).
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the first start segment.
    pub id: MaybeProp<String>,
    /// Form field name for the start segment group.
    pub name: MaybeProp<String>,
}

impl TimeRangeFieldBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for TimeRangeFieldBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for TimeRangeFieldBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for TimeRangeFieldBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for TimeRangeFieldBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Formatting options for [`super::time_range_field::TimeRangeField`].
pub struct TimeRangeFieldAppearance {
    /// Segmented time mask for both endpoints.
    pub format: Signal<DatetimeFormat>,
    /// Calendar day anchor when resolving time-only segments.
    pub reference_date: Signal<OrbitalDateTime>,
    /// Wall-clock timezone for segment parsing and display.
    pub timezone: Signal<DatetimeTimezone>,
    /// Minute increment hint (default `1`).
    pub minute_step: Signal<u32>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for TimeRangeFieldAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time12),
            reference_date: Signal::from(
                OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
            ),
            timezone: Signal::from(DatetimeTimezone::Local),
            minute_step: Signal::from(1),
            disabled: Signal::from(false),
        }
    }
}

impl TimeRangeFieldAppearance {
    pub fn time24() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`super::date_time_range_field::DateTimeRangeField`].
#[derive(Default)]
pub struct DateTimeRangeFieldBind {
    /// Two-way datetime range value (`None` when incomplete or empty).
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the first start segment.
    pub id: MaybeProp<String>,
    /// Form field name for the start segment group.
    pub name: MaybeProp<String>,
}

impl DateTimeRangeFieldBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for DateTimeRangeFieldBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for DateTimeRangeFieldBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for DateTimeRangeFieldBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for DateTimeRangeFieldBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Formatting options for [`super::date_time_range_field::DateTimeRangeField`].
pub struct DateTimeRangeFieldAppearance {
    /// Segmented date mask for both endpoints.
    pub date_format: Signal<DatetimeFormat>,
    /// Segmented time mask for both endpoints.
    pub time_format: Signal<DatetimeFormat>,
    /// Timezone used when parsing typed segments.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables all segments.
    pub disabled: Signal<bool>,
}

impl Default for DateTimeRangeFieldAppearance {
    fn default() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::default()),
            time_format: Signal::from(DatetimeFormat::Time12),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl DateTimeRangeFieldAppearance {
    pub fn iso_time24() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::IsoDate),
            time_format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`crate::DateRangePicker`].
#[derive(Default)]
pub struct DateRangePickerBind {
    /// Two-way date range value.
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the start field input.
    pub id: MaybeProp<String>,
    /// Form field name for the start field input.
    pub name: MaybeProp<String>,
}

impl DateRangePickerBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for DateRangePickerBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for DateRangePickerBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for DateRangePickerBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for DateRangePickerBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Panel and field options for [`crate::DateRangePicker`].
pub struct DateRangePickerAppearance {
    /// Display format for the range field segments.
    pub format: Signal<DatetimeFormat>,
    /// Timezone for date parsing and display.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables the field and calendar panel.
    pub disabled: Signal<bool>,
    /// Number of month panels in the calendar (1 or 2).
    pub calendars: Signal<u8>,
    /// Close the popover when the range selection completes.
    pub close_on_select: Signal<bool>,
    /// Popover placement for the calendar panel.
    pub placement: Signal<Placement>,
}

impl Default for DateRangePickerAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::default()),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
            calendars: Signal::from(2),
            close_on_select: Signal::from(true),
            placement: Signal::from(Placement::BottomStart),
        }
    }
}

/// Value binding for [`crate::TimeRangePicker`].
#[derive(Default)]
pub struct TimeRangePickerBind {
    /// Two-way time range value.
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the start field input.
    pub id: MaybeProp<String>,
    /// Form field name for the start field input.
    pub name: MaybeProp<String>,
}

impl TimeRangePickerBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for TimeRangePickerBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for TimeRangePickerBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for TimeRangePickerBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for TimeRangePickerBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Panel and field options for [`crate::TimeRangePicker`].
pub struct TimeRangePickerAppearance {
    /// Display format for the time field segments.
    pub format: Signal<DatetimeFormat>,
    /// Calendar day anchor when resolving time-only values.
    pub reference_date: Signal<OrbitalDateTime>,
    /// Wall-clock timezone for time panel and field values.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables the field and time panels.
    pub disabled: Signal<bool>,
}

impl Default for TimeRangePickerAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time12),
            reference_date: Signal::from(
                OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
            ),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl TimeRangePickerAppearance {
    pub fn time24() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}

/// Value binding for [`crate::DateTimeRangePicker`].
#[derive(Default)]
pub struct DateTimeRangePickerBind {
    /// Two-way datetime range value.
    pub value: OptionBind<DateTimeRange>,
    /// Explicit `id` for the start field input.
    pub id: MaybeProp<String>,
    /// Form field name for the start field input.
    pub name: MaybeProp<String>,
}

impl DateTimeRangePickerBind {
    pub fn new(value: impl Into<OptionBind<DateTimeRange>>) -> Self {
        Self {
            value: range_bind_from_master(value),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<DateTimeRange>>> for DateTimeRangePickerBind {
    fn from(value: RwSignal<Option<DateTimeRange>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<DateTimeRange>> for DateTimeRangePickerBind {
    fn from(value: OptionBind<DateTimeRange>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<DateTimeRange>> for DateTimeRangePickerBind {
    fn from(value: Option<DateTimeRange>) -> Self {
        Self::new(value)
    }
}

impl From<DateTimeRange> for DateTimeRangePickerBind {
    fn from(value: DateTimeRange) -> Self {
        Self::new(Some(value))
    }
}

/// Panel and field options for [`crate::DateTimeRangePicker`].
pub struct DateTimeRangePickerAppearance {
    /// Display format for the date field segments.
    pub date_format: Signal<DatetimeFormat>,
    /// Display format for the time field segments.
    pub time_format: Signal<DatetimeFormat>,
    /// Timezone for date parsing and display.
    pub timezone: Signal<DatetimeTimezone>,
    /// Disables the field and datetime panels.
    pub disabled: Signal<bool>,
    /// Close the popover when the range selection completes.
    pub close_on_select: Signal<bool>,
    /// Popover placement for the start/end datetime panels.
    pub placement: Signal<Placement>,
}

impl Default for DateTimeRangePickerAppearance {
    fn default() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::default()),
            time_format: Signal::from(DatetimeFormat::Time12),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
            close_on_select: Signal::from(true),
            placement: Signal::from(Placement::BottomStart),
        }
    }
}

impl DateTimeRangePickerAppearance {
    pub fn iso_time24() -> Self {
        Self {
            date_format: Signal::from(DatetimeFormat::IsoDate),
            time_format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }
}
