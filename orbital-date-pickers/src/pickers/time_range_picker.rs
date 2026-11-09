//! [`TimeRangePicker`] — time range field with dual time picker panels.

use leptos::prelude::*;
use orbital_core_components::{TimePicker, TimePickerAppearance, TimePickerBind};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::building_blocks::{
    TimeRangeField, TimeRangeFieldAppearance, TimeRangeFieldBind, TimeRangePickerAppearance,
    TimeRangePickerBind,
};
use crate::shared::{
    datetime_range_picker_row_class, layout_root_classes, picker_style_sheet, use_range_coordinator,
};

/// Time range field with side-by-side start/end [`TimePicker`](orbital_core_components::TimePicker)
/// panels, bound to [`DateTimeRange`].
///
/// See the crate README for range control selection.
///
/// # When to use
///
/// - Business hours, shift windows, or same-day time spans
/// - Forms where users pick start/end times with scroll columns instead of typing
///
/// # Usage
///
/// 1. Bind `Option<DateTimeRange>` through [`TimeRangePickerBind`].
/// 2. Set [`TimeRangePickerAppearance`] for 12/24-hour format and reference date.
/// 3. Enable [`DatePickerFeatures::RANGE_PICKERS`] in docs — no runtime license check.
///
/// # Best Practices
///
/// ## Do's
///
/// - Anchor times to a shared `reference_date` when the range is always same-day.
///
/// ## Don'ts
///
/// - Do not mix timezone binds — keep start and end in the same display zone from [`DatetimeLocale`].
///
/// # Examples
///
/// ## Time window
/// Default 12-hour range field with dual time pickers and bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="time-range-picker-preview">
///         <PickerPreviewKnobs />
///         <TimeRangePicker bind=value />
///         <div data-testid="time-range-picker-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour panels
/// Scroll-column pickers in 24-hour format.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="TRP-02">
///         <TimeRangePicker bind=value appearance=TimeRangePickerAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "time-range-picker",
    preview_label = "Time Range Picker",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn TimeRangePicker(
    /// Value binding for the combined time range pickers.
    #[prop(optional, into)]
    bind: TimeRangePickerBind,
    /// Time format, reference date, and disabled state.
    #[prop(optional, into)]
    appearance: TimeRangePickerAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let TimeRangePickerBind { value, id, name } = bind;
    let TimeRangePickerAppearance {
        format,
        reference_date,
        timezone,
        disabled,
    } = appearance;

    let locale = untrack(crate::use_datetime_locale);
    let theme_options = use_theme_options();
    let value_stored = StoredValue::new(value);
    let coordinator = use_range_coordinator(value_stored.with_value(|v| v.clone()));
    let resolved_reference = Signal::derive(move || reference_date.get());

    let field_bind = TimeRangeFieldBind {
        value: value_stored.with_value(|v| v.clone()),
        id,
        name,
    };
    let field_appearance = TimeRangeFieldAppearance {
        format,
        reference_date: resolved_reference,
        timezone,
        minute_step: Signal::from(1),
        disabled,
    };

    let start_picker_appearance = TimePickerAppearance {
        format,
        reference_date: Signal::derive(move || Some(reference_date.get())),
        timezone,
        disabled,
    };
    let end_picker_appearance = TimePickerAppearance {
        format,
        reference_date: Signal::derive(move || Some(reference_date.get())),
        timezone,
        disabled,
    };

    let root_class = move || {
        let mut parts = vec![layout_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        let _ = locale.reference_date;
        parts.join(" ")
    };

    view! {
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <TimeRangeField bind=field_bind appearance=field_appearance />
            <div class=datetime_range_picker_row_class()>
                <TimePicker bind=TimePickerBind::new(coordinator.start) appearance=start_picker_appearance />
                <TimePicker bind=TimePickerBind::new(coordinator.end) appearance=end_picker_appearance />
            </div>
        </div>
    }
}
