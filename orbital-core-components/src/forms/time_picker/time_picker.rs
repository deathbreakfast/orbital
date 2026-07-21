use leptos::prelude::*;
use orbital_base_components::{
    BaseTimePicker, DatetimeFormat, DatetimeTimezone, OptionBind, OrbitalDateTime,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::forms::datetime_bridge::{
    bridge_reference_date_to_unix, orbital_from_i64, use_unix_bridge,
};

use super::styles::time_picker_styles;

/// TimePicker captures time-of-day as [`OrbitalDateTime`] anchored to a calendar day.
///
/// Users open the trigger and scroll hour, minute, and second columns (with AM/PM when using 12-hour format). The bound value is an [`OrbitalDateTime`] on a calendar day — convert at API boundaries via `ToUnixSeconds`. Set `reference_date` in [`TimePickerAppearance`] when the field starts empty but must resolve to a specific day; common in scheduling forms paired with [`DatePicker`](crate::DatePicker). For date-only fields, use [`DatePicker`](crate::DatePicker) instead.
///
/// # When to use
///
/// - Time-of-day fields on forms and filters
/// - Scheduling flows paired with [`DatePicker`](crate::DatePicker)
/// - 12-hour or 24-hour locale-specific time entry
/// - Empty time fields that must anchor to a known calendar day via `reference_date`
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`TimePickerBind`].
/// 2. Set format, `timezone`, and `reference_date` in [`TimePickerAppearance`].
/// 3. Wrap in [`Field`](crate::Field) for labeled form layouts.
///
/// # Lifecycle
///
/// - **Open:** the scroll-column panel opens on trigger click and closes after a time slot is selected.
/// - **Value:** panel selection commits immediately to the bound signal.
///
/// # Timezone
///
/// `appearance.timezone` anchors wall-clock display and value construction. The bound
/// [`OrbitalDateTime`] carries the same timezone context on its calendar day.
///
/// # Best Practices
///
/// ## Do's
///
/// * Set `reference_date` when the time should anchor to a specific calendar day and the field may start empty
/// * Use 12-hour appearance when locale expects AM/PM
/// * Pair with [`DatePicker`](crate::DatePicker) for full date-and-time scheduling forms
///
/// ## Don'ts
///
/// * Do not use for date selection — prefer [`DatePicker`](crate::DatePicker)
/// * Do not treat the bound value as a clock string — persist [`OrbitalDateTime`] and convert at boundaries
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// 12-hour picker with hour/minute/second columns and AM/PM controls.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="time-picker-preview">
///     <div data-testid="TP-01">
///         <TimePicker bind=value />
///     </div>
///     </div>
/// }
/// ```
///
/// ## 24-hour format
/// Uses a 0–23 hour column without AM/PM.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="TP-02">
///         <TimePicker bind=value appearance=TimePickerAppearance::time24() />
///     </div>
/// }
/// ```
///
/// ## Preselected value
/// Existing [`OrbitalDateTime`] renders as initial trigger value.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1_738_412_400_i64, DatetimeTimezone::Local)
///         .expect("valid time"),
/// ));
/// view! {
///     <div data-testid="TP-03">
///         <TimePicker bind=value />
///     </div>
/// }
/// ```
///
/// ## Reference date fallback
/// Uses provided reference date when value is currently `None`.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="TP-04">
///         <TimePicker
///             bind=value
///             appearance=TimePickerAppearance {
///                 reference_date: Signal::from(Some(
///                     OrbitalDateTime::try_from_unix_seconds(1_735_603_200_i64, DatetimeTimezone::Local)
///                         .expect("valid reference date"),
///                 )),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Trigger cannot open when disabled.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="TP-05">
///         <TimePicker
///             bind=value
///             appearance=TimePickerAppearance::disabled()
///         />
///     </div>
/// }
/// ```
///
/// ## Field composition
/// Typical labeled usage with `Field`.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="TP-06">
///         <Field label="Start time">
///             <TimePicker bind=value />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## OptionBind from plain value
/// Preview ergonomics for static initialization.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// view! {
///     <div data-testid="TP-07">
///         <TimePicker bind=OrbitalDateTime::try_from_unix_seconds(1_738_412_400_i64, DatetimeTimezone::Local).expect("valid time") />
///     </div>
/// }
/// ```
///
/// ## Empty placeholder
/// Shows placeholder text when no value is selected.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <div data-testid="TP-08">
///         <TimePicker bind=value />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "time-picker",
    preview_label = "Time Picker",
    preview_icon = icondata::AiClockCircleOutlined,
)]
#[component]
pub fn TimePicker(
    /// Value binding and optional field identity.
    #[prop(optional, into)]
    bind: TimePickerBind,
    /// Format, reference date, placeholder, disabled state, and panel placement.
    #[prop(optional, into)]
    appearance: TimePickerAppearance,
    /// Extra CSS class names merged onto the picker root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-time-picker", time_picker_styles());

    let TimePickerBind { value, id, name } = bind;
    let TimePickerAppearance {
        format,
        reference_date,
        timezone,
        disabled,
    } = appearance;

    let unix_value = use_unix_bridge(value, timezone);
    let unix_reference = bridge_reference_date_to_unix(reference_date);

    view! {
        <BaseTimePicker
            class=class
            id=id
            name=name
            value=unix_value
            format=format
            reference_date=unix_reference
            disabled=disabled
        />
    }
}

#[derive(Default)]
pub struct TimePickerBind {
    pub value: OptionBind<OrbitalDateTime>,
    pub id: MaybeProp<String>,
    pub name: MaybeProp<String>,
}

impl TimePickerBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<OptionBind<OrbitalDateTime>> for TimePickerBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for TimePickerBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<Option<OrbitalDateTime>> for TimePickerBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for TimePickerBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for TimePickerBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for TimePickerBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

impl From<RwSignal<Option<i64>>> for TimePickerBind {
    fn from(value: RwSignal<Option<i64>>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}

impl From<OptionBind<i64>> for TimePickerBind {
    fn from(value: OptionBind<i64>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}

/// Panel format and behavior options for [`TimePicker`].
pub struct TimePickerAppearance {
    /// Display format for the trigger and scroll columns.
    pub format: Signal<DatetimeFormat>,
    /// Calendar-day anchor when the bound value is `None`.
    pub reference_date: Signal<Option<OrbitalDateTime>>,
    /// Wall-clock timezone for display, parsing, and value construction.
    pub timezone: Signal<DatetimeTimezone>,
    /// When true, the trigger cannot open the panel.
    pub disabled: Signal<bool>,
}

impl Default for TimePickerAppearance {
    fn default() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time12),
            reference_date: Signal::from(None),
            timezone: Signal::from(DatetimeTimezone::Local),
            disabled: Signal::from(false),
        }
    }
}

impl TimePickerAppearance {
    pub fn time24() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time24),
            ..Default::default()
        }
    }

    pub fn time12() -> Self {
        Self {
            format: Signal::from(DatetimeFormat::Time12),
            ..Default::default()
        }
    }

    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
            ..Default::default()
        }
    }
}

impl From<DatetimeFormat> for TimePickerAppearance {
    fn from(format: DatetimeFormat) -> Self {
        Self {
            format: Signal::from(format),
            ..Default::default()
        }
    }
}

impl From<DatetimeTimezone> for TimePickerAppearance {
    fn from(timezone: DatetimeTimezone) -> Self {
        Self {
            timezone: Signal::from(timezone),
            ..Default::default()
        }
    }
}
