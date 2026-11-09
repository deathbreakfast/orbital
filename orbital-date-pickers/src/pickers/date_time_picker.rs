//! [`DateTimePicker`] — combined date and time pickers on one [`OrbitalDateTime`] bind.

use leptos::prelude::*;
use orbital_core_components::{DatePicker, DatePickerAppearance, TimePicker, TimePickerAppearance};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::building_blocks::{DateTimePickerAppearance, DateTimePickerBind};
use crate::shared::{
    datetime_picker_root_classes, datetime_picker_row_class, picker_style_sheet,
    use_datetime_coordinator,
};

/// Side-by-side date and time pickers sharing one [`OrbitalDateTime`] bind.
///
/// DateTimePicker composes core [`DatePicker`](orbital_core_components::DatePicker) and
/// [`TimePicker`](orbital_core_components::TimePicker). Changing the calendar day preserves the
/// existing time-of-day. For keyboard segment entry without popovers, use [`DateTimeField`](crate::DateTimeField).
///
/// # When to use
///
/// - Event scheduling forms (start/end datetime)
/// - Flows that need both calendar and scroll-column time selection
/// - Combined date and time selection with calendar and scroll-column pickers
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`DateTimePickerBind`].
/// 2. Set `appearance.date_format` and `appearance.time_format` for locale masks.
/// 3. Wrap in [`DatetimeLocale`](crate::DatetimeLocale) for shared defaults.
///
/// # Examples
///
/// ## Date and time pickers
/// Default US date + 12-hour time with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-time-picker-preview">
///         <PickerPreviewKnobs />
///         <DateTimePicker bind=value />
///         <div data-testid="date-time-picker-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Selecting date and time updates the bound [`OrbitalDateTime`].
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DTP-02">
///         <DateTimePicker bind=value />
///         <div data-testid="DTP-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour time
/// Date picker with 24-hour scroll columns.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DTP-03">
///         <DateTimePicker bind=value appearance=DateTimePickerAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Disabled
/// Both pickers are non-interactive.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DTP-04">
///         <DateTimePicker bind=value appearance=DateTimePickerAppearance { disabled: Signal::from(true), ..Default::default() } />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-time-picker",
    preview_label = "Date Time Picker",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateTimePicker(
    /// Value binding for the combined date-time pickers.
    #[prop(optional, into)]
    bind: DateTimePickerBind,
    /// Date format, time format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateTimePickerAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateTimePickerBind { value, id, name } = bind;
    let DateTimePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        placement,
    } = appearance;

    let locale = untrack(crate::use_datetime_locale);
    let theme_options = use_theme_options();
    let fallback_reference = Signal::derive(move || locale.reference_date);
    let coordinator = use_datetime_coordinator(value, id, name, fallback_reference);

    let date_appearance = DatePickerAppearance {
        format: date_format,
        timezone,
        disabled,
        placement,
        ..Default::default()
    };
    let time_appearance = TimePickerAppearance {
        format: time_format,
        reference_date: coordinator.reference_date,
        timezone,
        disabled,
    };

    let root_class = move || {
        let mut parts = vec![datetime_picker_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <div class=datetime_picker_row_class()>
                <DatePicker bind=coordinator.date_bind appearance=date_appearance />
                <TimePicker bind=coordinator.time_bind appearance=time_appearance />
            </div>
        </div>
    }
}
