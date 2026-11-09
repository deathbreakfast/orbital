//! [`DateTimeRangeField`] — segmented start/end datetime input bound to [`DateTimeRange`].

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{picker_style_sheet, range_field_root_classes, use_range_coordinator};
use crate::{use_datetime_locale, DateTimeField, DateTimeFieldAppearance, DateTimeFieldBind};

use super::field_types::{DateTimeRangeFieldAppearance, DateTimeRangeFieldBind};

/// Segmented start/end datetime input bound to [`DateTimeRange`].
///
/// DateTimeRangeField renders two [`DateTimeField`](crate::DateTimeField) segment groups
/// separated by an en-dash. See the crate README for range control selection.
///
/// # When to use
///
/// - Forms that need typed date+time on both endpoints without popover panels
/// - Accessibility-first flows where segmented fields beat mouse-driven pickers
///
/// # Usage
///
/// 1. Bind `Option<DateTimeRange>` through [`DateTimeRangeFieldBind`].
/// 2. Configure [`DateTimeRangeFieldAppearance`] for date and time format independently.
/// 3. Wrap in [`DatetimeLocale`](crate::DatetimeLocale) for default timezone.
///
/// # Best Practices
///
/// ## Do's
///
/// - Pair with [`DateTimeRangePicker`](crate::DateTimeRangePicker) when users want both typing and panels.
///
/// ## Don'ts
///
/// - Do not convert to unix in the bind — keep [`OrbitalDateTime`] endpoints inside [`DateTimeRange`].
///
/// # Examples
///
/// ## Datetime range field
/// Default US date + 12-hour time segments with bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-time-range-field-preview">
///         <PickerPreviewKnobs />
///         <DateTimeRangeField bind=value />
///         <div data-testid="date-time-range-field-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## ISO date and 24-hour time
/// Year-month-day segments with 24-hour hour/minute endpoints.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DTRF-02">
///         <DateTimeRangeField bind=value appearance=DateTimeRangeFieldAppearance::iso_time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-time-range-field",
    preview_label = "DateTime Range Field",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn DateTimeRangeField(
    /// Value binding for the segmented datetime range input.
    #[prop(optional, into)]
    bind: DateTimeRangeFieldBind,
    /// Date format, time format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateTimeRangeFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateTimeRangeFieldBind { value, id, name } = bind;
    let DateTimeRangeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    } = appearance;

    let _locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();
    let coordinator = use_range_coordinator(value);

    let field_appearance_start = DateTimeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    };
    let field_appearance_end = DateTimeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    };

    let root_class = move || {
        let mut parts = vec![range_field_root_classes(
            "orb-datetime-range-field",
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
            <DateTimeField
                bind=DateTimeFieldBind { value: coordinator.start.into(), id, name }
                appearance=field_appearance_start
            />
            <span class="orb-picker-range-field__separator">" – "</span>
            <DateTimeField
                bind=DateTimeFieldBind { value: coordinator.end.into(), ..Default::default() }
                appearance=field_appearance_end
            />
        </div>
    }
}
