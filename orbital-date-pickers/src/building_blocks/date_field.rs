//! [`DateField`] — segmented date input bound to [`OrbitalDateTime`].

use leptos::prelude::*;
use orbital_base_components::DatetimeFormat;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::picker_style_sheet;
use crate::{use_datetime_locale, SegmentedDatetimeField};

use super::field_types::{DateFieldAppearance, DateFieldBind};

/// Segmented date input with locale-aware section masks, bound to [`OrbitalDateTime`].
///
/// DateField renders editable month/day/year (or ISO) segments instead of a single text box.
/// Values are stored as start-of-day [`OrbitalDateTime`] in the chosen timezone. Convert at
/// API boundaries via `ToUnixSeconds` or `ToIso8601`. For a text field with popover calendar,
/// use core [`DatePicker`](orbital_core_components::DatePicker) instead.
///
/// # When to use
///
/// - Dense forms that benefit from section-wise date entry
/// - Keyboards-first flows where users tab through date parts
/// - Flows that need segmented date entry without a popover calendar
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`DateFieldBind`].
/// 2. Set `appearance.format` to `IsoDate` or `UsDate`.
/// 3. Wrap in [`Field`](orbital_core_components::Field) when a visible label is required.
///
/// # Lifecycle
///
/// - **Value:** each segment commits the parsed date on blur when all required sections are complete.
/// - **Open state:** no popover — segments are always editable inline.
///
/// # Timezone
///
/// `appearance.timezone` controls start-of-day normalization for parsed segment values.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind with [`OrbitalDateTime`], not raw unix seconds
/// * Wrap preview examples in a native element with `data-testid`
///
/// ## Don'ts
///
/// * Do not use for time-of-day — prefer [`TimeField`](crate::TimeField)
/// * Do not put `data-testid` on the component itself
///
/// # Examples
///
/// ## Segmented input
/// Default US-format month/day/year segments with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-field-preview">
///         <PickerPreviewKnobs />
///         <DateField bind=value />
///         <div data-testid="date-field-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Typing complete segments updates the bound [`OrbitalDateTime`].
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DF-02">
///         <DateField bind=value />
///         <div data-testid="DF-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## ISO format
/// Year-month-day segment order for ISO locales.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, OrbitalDateTime};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DF-03">
///         <DateField bind=value appearance=DateFieldAppearance { format: Signal::from(DatetimeFormat::IsoDate), ..Default::default() } />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-field",
    preview_label = "Date Field",
    preview_icon = icondata::AiFieldBinaryOutlined,
)]
#[component]
pub fn DateField(
    /// Value binding for the segmented date input.
    #[prop(optional, into)]
    bind: DateFieldBind,
    /// Format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateFieldBind { value, id, name } = bind;
    let DateFieldAppearance {
        format,
        timezone,
        disabled,
    } = appearance;

    let locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();
    let resolved_format = Signal::derive(move || match format.get() {
        DatetimeFormat::Time24 | DatetimeFormat::Time12 => locale.default_format,
        other => other,
    });
    let resolved_timezone = Signal::derive(move || timezone.get());
    let reference_date = Signal::derive(move || locale.reference_date);

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
                reference_date=reference_date
                disabled=disabled
                testid_prefix="date-field"
                id=id
                name=name
            />
        </div>
    }
}
