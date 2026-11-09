//! [`TimeField`] — segmented time input bound to [`OrbitalDateTime`].

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::picker_style_sheet;
use crate::{use_datetime_locale, SegmentedDatetimeField};

use super::field_types::{TimeFieldAppearance, TimeFieldBind};

/// Segmented time input bound to [`OrbitalDateTime`] on a reference calendar day.
///
/// TimeField renders hour/minute (and meridiem for 12-hour) segments. Parsed values are
/// anchored to `appearance.reference_date` or the nearest [`DatetimeLocale`](crate::DatetimeLocale)
/// default. For scroll-column time selection, use core [`TimePicker`](orbital_core_components::TimePicker).
///
/// # When to use
///
/// - Time-of-day entry with keyboard-friendly segments
/// - Scheduling forms paired with a date field on the same reference day
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`TimeFieldBind`].
/// 2. Set `appearance.format` to `Time12` or `Time24`.
/// 3. Set `appearance.timezone` and `reference_date` when the anchor day or zone differs from defaults.
///
/// # Lifecycle
///
/// - **Value:** completed time segments commit on blur via [`SegmentedDatetimeField`].
/// - **Open state:** no popover — segments are always editable inline.
///
/// # Timezone
///
/// `appearance.timezone` controls how hour/minute segments resolve to [`OrbitalDateTime`]
/// on `appearance.reference_date`.
///
/// # Examples
///
/// ## Time segments
/// Default 12-hour segmented input with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="time-field-preview">
///         <PickerPreviewKnobs />
///         <TimeField bind=value />
///         <div data-testid="time-field-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour format
/// Hour and minute segments without meridiem.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="TF-02">
///         <TimeField bind=value appearance=TimeFieldAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Completed segments update the bound value readout.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="TF-03">
///         <TimeField bind=value />
///         <div data-testid="TF-03-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "time-field",
    preview_label = "Time Field",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn TimeField(
    /// Value binding for the segmented time input.
    #[prop(optional, into)]
    bind: TimeFieldBind,
    /// Format, reference day, and disabled state.
    #[prop(optional, into)]
    appearance: TimeFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let TimeFieldBind { value, id, name } = bind;
    let TimeFieldAppearance {
        format,
        reference_date,
        timezone,
        disabled,
        minute_step: _,
    } = appearance;

    let _locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();
    let resolved_format = Signal::derive(move || format.get());
    let resolved_timezone = Signal::derive(move || timezone.get());
    let resolved_reference = Signal::derive(move || reference_date.get().start_of_day());

    let root_class = move || {
        let mut parts = vec!["orb-picker-segmented-field-host".to_string()];
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
            <SegmentedDatetimeField
                value=value
                format=resolved_format
                timezone=resolved_timezone
                reference_date=resolved_reference
                disabled=disabled
                testid_prefix="time-field"
                is_time=true
                id=id
                name=name
            />
        </div>
    }
}
