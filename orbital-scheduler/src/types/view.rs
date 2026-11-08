/// Calendar view modes for [`SchedulerCalendar`](crate::SchedulerCalendar).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulerView {
    Day,
    Week,
    Month,
    Agenda,
}

impl SchedulerView {
    /// User-facing label for toolbar and select options.
    pub fn label(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::Week => "Week",
            Self::Month => "Month",
            Self::Agenda => "Agenda",
        }
    }

    /// Stable select / wire value.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Agenda => "agenda",
        }
    }

    /// Parse a select wire value.
    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "day" => Some(Self::Day),
            "week" => Some(Self::Week),
            "month" => Some(Self::Month),
            "agenda" => Some(Self::Agenda),
            _ => None,
        }
    }

    /// All views in display order.
    pub fn all() -> &'static [Self] {
        &[Self::Day, Self::Week, Self::Month, Self::Agenda]
    }
}

/// Timeline zoom presets for [`SchedulerTimeline`](crate::SchedulerTimeline).
///
/// Hour and day presets cover up to one week. [`Month`](Self::Month) and
/// [`Quarter`](Self::Quarter) zoom out for project plans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelinePreset {
    /// One day at hourly resolution.
    Day,
    /// One business day (08:00–18:00) at hourly resolution.
    BusinessDay,
    /// One week at daily resolution.
    Week,
    /// One business week (Mon–Fri) at daily resolution.
    BusinessWeek,
    /// One calendar month, one column per day.
    Month,
    /// Three calendar months (a quarter), one column per week starting Monday.
    ///
    /// The first and last columns are clipped to the quarter, so they can be
    /// shorter than seven days; their width shrinks to match.
    Quarter,
}

impl TimelinePreset {
    /// User-facing label for toolbar and select options.
    pub fn label(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::BusinessDay => "Business day",
            Self::Week => "Week",
            Self::BusinessWeek => "Business week",
            Self::Month => "Month",
            Self::Quarter => "Quarter",
        }
    }

    /// Stable select / wire value.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::BusinessDay => "business_day",
            Self::Week => "week",
            Self::BusinessWeek => "business_week",
            Self::Month => "month",
            Self::Quarter => "quarter",
        }
    }

    /// Parse a select wire value.
    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "day" => Some(Self::Day),
            "business_day" => Some(Self::BusinessDay),
            "week" => Some(Self::Week),
            "business_week" => Some(Self::BusinessWeek),
            "month" => Some(Self::Month),
            "quarter" => Some(Self::Quarter),
            // Legacy wire values from removed presets.
            "hour" => Some(Self::Day),
            "year" => Some(Self::Quarter),
            _ => None,
        }
    }

    /// All presets in display order.
    pub fn all() -> &'static [Self] {
        &[
            Self::Day,
            Self::BusinessDay,
            Self::Week,
            Self::BusinessWeek,
            Self::Month,
            Self::Quarter,
        ]
    }
}
