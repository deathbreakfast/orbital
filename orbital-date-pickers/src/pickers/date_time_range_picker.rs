//! [`DateTimeRangePicker`] — datetime range field with popover start/end panels.

use leptos::prelude::*;
use orbital_base_components::OverlayDismiss;
use orbital_core_components::{
    Icon, Popover, PopoverPosition, PopoverSize, PopoverTrigger, PopoverTriggerType,
};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::building_blocks::{
    DateTimeRangeField, DateTimeRangeFieldAppearance, DateTimeRangeFieldBind,
    DateTimeRangePickerAppearance, DateTimeRangePickerBind,
};
use crate::pickers::DateTimePicker;
use crate::shared::{
    datetime_range_picker_row_class, layout_root_classes, picker_style_sheet, use_range_coordinator,
};
use crate::{DateTimePickerAppearance, DateTimePickerBind};

/// Datetime range field with a popover of start/end [`DateTimePicker`](crate::DateTimePicker)
/// panels, bound to [`DateTimeRange`].
///
/// See the crate README for range control selection.
///
/// # When to use
///
/// - Meeting slot booking with full date and time on both endpoints
/// - Maintenance windows that span partial days
///
/// # Usage
///
/// 1. Wrap in [`DatetimeLocale`](crate::DatetimeLocale) for timezone and format defaults.
/// 2. Bind `Option<DateTimeRange>` through [`DateTimeRangePickerBind`].
/// 3. Tune [`DateTimeRangePickerAppearance`] for date masks and 12/24-hour time columns.
///
/// # Best Practices
///
/// ## Do's
///
/// - Validate `range.end` is after `range.start` before submit — the picker allows partial entry while editing.
///
/// ## Don'ts
///
/// - Do not store unix seconds in the bind — use [`DateTimeRange`] with [`OrbitalDateTime`] endpoints.
///
/// # Examples
///
/// ## Meeting slot range
/// Default US date + 12-hour time range with bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-time-range-picker-preview">
///         <PickerPreviewKnobs />
///         <DateTimeRangePicker bind=value />
///         <div data-testid="date-time-range-picker-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## ISO date and 24-hour time
/// Combined pickers with ISO date masks and 24-hour time columns.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DTRP-02">
///         <DateTimeRangePicker bind=value appearance=DateTimeRangePickerAppearance::iso_time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-time-range-picker",
    preview_label = "DateTime Range Picker",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateTimeRangePicker(
    /// Value binding for the combined datetime range pickers.
    #[prop(optional, into)]
    bind: DateTimeRangePickerBind,
    /// Date format, time format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateTimeRangePickerAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateTimeRangePickerBind { value, id, name } = bind;
    let DateTimeRangePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        close_on_select,
        placement,
    } = appearance;

    let theme_options = use_theme_options();
    let value_stored = StoredValue::new(value);
    let coordinator = use_range_coordinator(value_stored.with_value(|v| v.clone()));

    let field_bind = DateTimeRangeFieldBind {
        value: value_stored.with_value(|v| v.clone()),
        id,
        name,
    };
    let field_appearance = DateTimeRangeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    };

    let picker_appearance_start = DateTimePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        ..Default::default()
    };
    let picker_appearance_end = DateTimePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        ..Default::default()
    };

    let root_class = move || {
        let mut parts = vec![layout_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <style>{date_time_range_picker_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <Popover
                trigger_type=PopoverTriggerType::Click
                position=placement_to_popover_position(placement.get_untracked())
                size=Signal::from(PopoverSize::Large)
            >
                <PopoverTrigger slot>
                    <div class="orb-picker-datetime-range-picker__trigger">
                        <DateTimeRangeField bind=field_bind appearance=field_appearance />
                        <button
                            type="button"
                            class="orb-picker-datetime-range-picker__open-btn"
                            aria-label="Open date and time range"
                            disabled=move || disabled.get()
                        >
                            <Icon icon=icondata::AiCalendarOutlined />
                        </button>
                    </div>
                </PopoverTrigger>
                <DateTimeRangePickerPanel
                    coordinator_start=coordinator.start
                    coordinator_end=coordinator.end
                    appearance_start=picker_appearance_start
                    appearance_end=picker_appearance_end
                    value=value_stored
                    close_on_select=close_on_select
                    id=id
                    name=name
                />
            </Popover>
        </div>
    }
}

#[component]
fn DateTimeRangePickerPanel(
    coordinator_start: RwSignal<Option<crate::OrbitalDateTime>>,
    coordinator_end: RwSignal<Option<crate::OrbitalDateTime>>,
    appearance_start: DateTimePickerAppearance,
    appearance_end: DateTimePickerAppearance,
    value: StoredValue<orbital_base_components::OptionBind<crate::DateTimeRange>>,
    close_on_select: Signal<bool>,
    id: MaybeProp<String>,
    name: MaybeProp<String>,
) -> impl IntoView {
    let dismiss = use_context::<OverlayDismiss>();
    let prev_complete = RwSignal::new(None::<bool>);

    Effect::new(move |_| {
        if !close_on_select.get() {
            return;
        }
        let complete = value.with_value(|v| v.get()).is_some();
        if complete && prev_complete.get_untracked() == Some(false) {
            if let Some(dismiss) = dismiss {
                dismiss.close.run(());
            }
        }
        prev_complete.set(Some(complete));
    });

    view! {
        <div class="orb-picker-datetime-range-picker__panel">
            <div class=datetime_range_picker_row_class()>
                <DateTimePicker
                    bind=DateTimePickerBind {
                        value: coordinator_start.into(),
                        id,
                        name,
                    }
                    appearance=appearance_start
                />
                <DateTimePicker
                    bind=DateTimePickerBind {
                        value: coordinator_end.into(),
                        ..Default::default()
                    }
                    appearance=appearance_end
                />
            </div>
        </div>
    }
}

fn date_time_range_picker_styles() -> &'static str {
    r#"
.orb-picker-datetime-range-picker__trigger {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.orb-picker-datetime-range-picker__open-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    cursor: pointer;
}

.orb-picker-datetime-range-picker__open-btn:hover:not(:disabled) {
    border-color: var(--orb-color-border-default-hover);
    background: var(--orb-color-surface-canvas-hover);
}

.orb-picker-datetime-range-picker__open-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.orb-picker-datetime-range-picker__panel {
    box-sizing: border-box;
    max-width: min(calc(100vw - 24px), 720px);
    padding: var(--orb-space-inset-md);
}

.orb-picker-datetime-range-picker__panel .orb-picker-datetime-range-picker__row {
    margin-top: 0;
}

.orbital-positioning-content:has(.orb-picker-datetime-range-picker__panel) {
    max-height: calc(100dvh - 16px);
    overflow-y: auto;
    overscroll-behavior: contain;
    -webkit-overflow-scrolling: touch;
}
"#
}

fn placement_to_popover_position(placement: orbital_base_components::Placement) -> PopoverPosition {
    use orbital_base_components::Placement;
    match placement {
        Placement::Top => PopoverPosition::Top,
        Placement::Bottom => PopoverPosition::Bottom,
        Placement::Left => PopoverPosition::Left,
        Placement::Right => PopoverPosition::Right,
        Placement::TopStart => PopoverPosition::TopStart,
        Placement::TopEnd => PopoverPosition::TopEnd,
        Placement::LeftStart => PopoverPosition::LeftStart,
        Placement::LeftEnd => PopoverPosition::LeftEnd,
        Placement::RightStart => PopoverPosition::RightStart,
        Placement::RightEnd => PopoverPosition::RightEnd,
        Placement::BottomStart => PopoverPosition::BottomStart,
        Placement::BottomEnd => PopoverPosition::BottomEnd,
    }
}
