use leptos::prelude::*;
use orbital_base_components::{
    DatePickerRule, DatetimeFormat, DatetimeTimezone, OptionBind, OrbitalDateTime, PickerShortcut,
    Placement,
};

use crate::forms::datetime_bridge::orbital_from_i64;

/// Value binding, identity, and validation for [`DatePicker`](super::component::DatePicker).
#[derive(Default)]
pub struct DatePickerBind {
    /// Two-way [`OrbitalDateTime`] date value (`None` when empty).
    pub value: OptionBind<OrbitalDateTime>,
    /// Explicit `id` for the input; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
    /// Validation rules that update the parent Field validation state.
    pub rules: Vec<DatePickerRule>,
}

impl DatePickerBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for DatePickerBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<OptionBind<OrbitalDateTime>> for DatePickerBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

impl From<Option<OrbitalDateTime>> for DatePickerBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for DatePickerBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for DatePickerBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for DatePickerBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

impl From<RwSignal<Option<i64>>> for DatePickerBind {
    fn from(value: RwSignal<Option<i64>>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}

impl From<OptionBind<i64>> for DatePickerBind {
    fn from(value: OptionBind<i64>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}

use crate::forms::calendar::CalendarMonthButtonRenderer;

/// Formatting, timezone, and visual options for [`DatePicker`](super::component::DatePicker).
pub struct DatePickerAppearance {
    pub format: Signal<DatetimeFormat>,
    pub timezone: Signal<DatetimeTimezone>,
    pub placeholder: MaybeProp<String>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub placement: Signal<Placement>,
    /// Earliest selectable day (inclusive); disables out-of-range days in the panel.
    pub min_date: Signal<Option<OrbitalDateTime>>,
    /// Latest selectable day (inclusive); disables out-of-range days in the panel.
    pub max_date: Signal<Option<OrbitalDateTime>>,
    /// Shortcut preset buttons shown below the calendar grid.
    pub shortcuts: Signal<Vec<PickerShortcut>>,
    /// Optional custom month button renderer for the month selection panel.
    pub month_button: Option<CalendarMonthButtonRenderer>,
}

impl Default for DatePickerAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::default()),
            timezone: Signal::from(DatetimeTimezone::Local),
            placeholder: MaybeProp::from("Select date".to_string()),
            disabled: Signal::from(false),
            readonly: Signal::from(false),
            placement: Signal::from(Placement::BottomStart),
            min_date: Signal::from(None),
            max_date: Signal::from(None),
            shortcuts: Signal::from(Vec::new()),
            month_button: None,
        }
    }
}

impl From<DatetimeFormat> for DatePickerAppearance {
    fn from(format: DatetimeFormat) -> Self {
        Self {
            format: Signal::from(format),
            ..Default::default()
        }
    }
}

impl From<DatetimeTimezone> for DatePickerAppearance {
    fn from(timezone: DatetimeTimezone) -> Self {
        Self {
            timezone: Signal::from(timezone),
            ..Default::default()
        }
    }
}
