//! [`DateTimeField`] — combined date and time segmented input.

use leptos::prelude::*;
use orbital_base_components::DatetimeFormat;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use super::datetime_field_coordinator::use_datetime_field_coordinator;
use crate::building_blocks::{
    DateField, DateFieldAppearance, DateTimeFieldAppearance, DateTimeFieldBind, TimeField,
    TimeFieldAppearance,
};
use crate::shared::{datetime_field_row_class, picker_style_sheet};

/// Combined date and time segmented input bound to [`OrbitalDateTime`].
///
/// DateTimeField composes [`DateField`](crate::DateField) and [`TimeField`](crate::TimeField) on
/// one shared bind. Changing the date preserves the existing time-of-day. For popover pickers,
/// use [`DateTimePicker`](crate::DateTimePicker) instead.
///
/// # When to use
///
/// - Dense forms needing keyboard-friendly date and time entry
/// - Flows that prefer segmented input over scroll-column pickers
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`DateTimeFieldBind`].
/// 2. Set `appearance.date_format` and `appearance.time_format`.
/// 3. Wrap in [`Field`](orbital_core_components::Field) when a visible label is required.
///
/// # Examples
///
/// ## Combined segments
/// Default US date + 12-hour time with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-time-field-preview">
///         <PickerPreviewKnobs />
///         <DateTimeField bind=value />
///         <div data-testid="date-time-field-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Completed segments update the bound [`OrbitalDateTime`].
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DTF-02">
///         <DateTimeField bind=value />
///         <div data-testid="DTF-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## ISO date and 24-hour time
/// Year-month-day segments followed by 24-hour hour/minute.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DTF-03">
///         <DateTimeField bind=value appearance=DateTimeFieldAppearance::iso_time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-time-field",
    preview_label = "Date Time Field",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn DateTimeField(
    /// Value binding for the combined segmented input.
    #[prop(optional, into)]
    bind: DateTimeFieldBind,
    /// Date format, time format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateTimeFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateTimeFieldBind { value, id, name } = bind;
    let DateTimeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    } = appearance;

    let locale = untrack(crate::use_datetime_locale);
    let theme_options = use_theme_options();
    let fallback_reference = Signal::derive(move || locale.reference_date);
    let coordinator = use_datetime_field_coordinator(value, id, name, fallback_reference);

    let resolved_date_format = Signal::derive(move || match date_format.get() {
        DatetimeFormat::Time24 | DatetimeFormat::Time12 => locale.default_format,
        other => other,
    });

    let date_appearance = DateFieldAppearance {
        format: resolved_date_format,
        timezone,
        disabled,
    };
    let time_appearance = TimeFieldAppearance {
        format: time_format,
        reference_date: coordinator.reference_date,
        timezone,
        disabled,
        ..Default::default()
    };

    let root_class = move || {
        let mut parts = Vec::new();
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        let _ = theme_options.get();
        parts.join(" ")
    };

    view! {
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <div class=datetime_field_row_class()>
                <DateField bind=coordinator.date_bind appearance=date_appearance />
                <TimeField bind=coordinator.time_bind appearance=time_appearance />
            </div>
        </div>
    }
}
