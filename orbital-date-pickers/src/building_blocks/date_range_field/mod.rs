//! [`DateRangeField`] — segmented start/end date input bound to [`DateTimeRange`].

mod styles;

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{picker_style_sheet, range_field_root_classes, use_range_coordinator};
use crate::{use_datetime_locale, DateField, DateFieldAppearance, DateFieldBind};

use super::field_types::{DateRangeFieldAppearance, DateRangeFieldBind};
use styles::date_range_field_styles;

/// Segmented start/end date input bound to [`DateTimeRange`].
///
/// DateRangeField renders two [`DateField`](crate::DateField) segment groups separated by an
/// en-dash. Values commit when both endpoints parse successfully. See
/// See the crate README for field vs picker choice.
///
/// # When to use
///
/// - Dense forms where users type dates instead of opening a calendar panel
/// - Tables or filters that need compact start/end segments
///
/// # Usage
///
/// 1. Bind `Option<DateTimeRange>` through [`DateRangeFieldBind`].
/// 2. Inherit format and timezone from [`DatetimeLocale`] when wrapped.
/// 3. Pair with [`DateRangePicker`](crate::DateRangePicker) when users also need a calendar popover.
///
/// # Best Practices
///
/// ## Do's
///
/// - Show validation when end precedes start — the field commits only when both sides parse.
///
/// ## Don'ts
///
/// - Do not use for time-only spans — use [`TimeRangeField`](crate::TimeRangeField) instead.
///
/// # Examples
///
/// ## Range segments
/// Default US-format start/end segments with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-range-field-preview">
///         <PickerPreviewKnobs />
///         <DateRangeField bind=value />
///         <div data-testid="date-range-field-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Completed start and end segments update the bound range.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DRF-02">
///         <DateRangeField bind=value />
///         <div data-testid="DRF-02-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-range-field",
    preview_label = "Date Range Field",
    preview_icon = icondata::AiFieldBinaryOutlined,
)]
#[component]
pub fn DateRangeField(
    /// Value binding for the segmented range input.
    #[prop(optional, into)]
    bind: DateRangeFieldBind,
    /// Format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateRangeFieldAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateRangeFieldBind { value, id, name } = bind;
    let DateRangeFieldAppearance {
        format,
        timezone,
        disabled,
    } = appearance;

    let locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();
    let coordinator = use_range_coordinator(value);

    let field_appearance_start = DateFieldAppearance {
        format,
        timezone,
        disabled,
    };
    let field_appearance_end = DateFieldAppearance {
        format,
        timezone,
        disabled,
    };

    let root_class = move || {
        let mut parts = vec![range_field_root_classes(
            "orb-date-range-field",
            theme_options.get().density,
        )];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        let _ = locale.default_format;
        parts.join(" ")
    };

    view! {
        <style>{date_range_field_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <DateField
                bind=DateFieldBind { value: coordinator.start.into(), id, name }
                appearance=field_appearance_start
            />
            <span class="orb-picker-range-field__separator">" – "</span>
            <DateField
                bind=DateFieldBind { value: coordinator.end.into(), ..Default::default() }
                appearance=field_appearance_end
            />
        </div>
    }
}
