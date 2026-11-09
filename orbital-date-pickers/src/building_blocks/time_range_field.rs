//! [`TimeRangeField`] — segmented start/end time input bound to [`DateTimeRange`].

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{picker_style_sheet, range_field_root_classes, use_range_coordinator};
use crate::{use_datetime_locale, TimeField, TimeFieldAppearance, TimeFieldBind};

use super::field_types::{TimeRangeFieldAppearance, TimeRangeFieldBind};

/// Segmented start/end time input bound to [`DateTimeRange`].
///
/// TimeRangeField renders two [`TimeField`](crate::TimeField) segment groups separated by an
/// en-dash. See the crate README for range control selection.
///
/// # When to use
///
/// - Same-day shift or hours windows entered by keyboard
/// - Compact toolbar filters for time-of-day spans
///
/// # Usage
///
/// 1. Bind `Option<DateTimeRange>` through [`TimeRangeFieldBind`].
/// 2. Set [`TimeRangeFieldAppearance`] for 12-hour or 24-hour segments.
/// 3. Share [`DatetimeLocale`] with sibling pickers on the page.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use a shared reference date on both endpoints when the range never crosses midnight.
///
/// ## Don'ts
///
/// - Do not use for multi-day spans — use [`DateRangeField`](crate::DateRangeField) or [`DateTimeRangeField`](crate::DateTimeRangeField).
///
/// # Examples
///
/// ## Time range segments
/// Default 12-hour start/end segments with bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="time-range-field-preview">
///         <PickerPreviewKnobs />
///         <TimeRangeField bind=value />
///         <div data-testid="time-range-field-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour format
/// Hour/minute segments in 24-hour layout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="TRF-02">
///         <TimeRangeField bind=value appearance=TimeRangeFieldAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "time-range-field",
    preview_label = "Time Range Field",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn TimeRangeField(
    /// Value binding for the segmented time range input.
    #[prop(optional, into)]
    bind: TimeRangeFieldBind,
    /// Format, reference date, and disabled state.
    #[prop(optional, into)]
    appearance: TimeRangeFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let TimeRangeFieldBind { value, id, name } = bind;
    let TimeRangeFieldAppearance {
        format,
        reference_date,
        timezone,
        minute_step,
        disabled,
    } = appearance;

    let locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();
    let coordinator = use_range_coordinator(value);
    let resolved_reference = Signal::derive(move || {
        let explicit = reference_date.get();
        if explicit.start_of_day() == locale.reference_date {
            locale.reference_date
        } else {
            explicit
        }
    });

    let field_appearance_start = TimeFieldAppearance {
        format,
        reference_date: resolved_reference,
        timezone,
        minute_step,
        disabled,
    };
    let field_appearance_end = TimeFieldAppearance {
        format,
        reference_date: resolved_reference,
        timezone,
        minute_step,
        disabled,
    };

    let root_class = move || {
        let mut parts = vec![range_field_root_classes(
            "orb-time-range-field",
            theme_options.get().density,
        )];
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
            <TimeField
                bind=TimeFieldBind { value: coordinator.start.into(), id, name }
                appearance=field_appearance_start
            />
            <span class="orb-picker-range-field__separator">" – "</span>
            <TimeField
                bind=TimeFieldBind { value: coordinator.end.into(), ..Default::default() }
                appearance=field_appearance_end
            />
        </div>
    }
}
