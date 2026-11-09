mod clock_number;
mod dial;
mod interaction;
mod pointer;
mod styles;

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{
    commit_time, now_on_anchor, picker_style_sheet, resolve_anchor, snap_minute,
    use_datetime_locale,
};

use super::field_types::{TimeClockAppearance, TimeClockBind};

pub use dial::{ClockView, TimeClockDial};
use styles::time_clock_styles;

/// Analog clock surface for selecting time-of-day, bound to [`OrbitalDateTime`].
///
/// TimeClock renders a two-step SVG dial: pick an hour, then pick a minute. Values anchor
/// to `appearance.reference_date` or the nearest [`DatetimeLocale`](crate::DatetimeLocale) default.
/// Clock surfaces are optional product features documented via [`DatePickerFeatures::CLOCK_VIEWS`]
/// — there is no runtime license check.
///
/// # When to use
///
/// - Visual time selection in picker panels or dialogs
/// - Alternatives to scroll-column [`TimePicker`](orbital_core_components::TimePicker) surfaces
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`TimeClockBind`].
/// 2. Set `appearance.ampm` for 12-hour vs 24-hour dial.
/// 3. Wrap preview examples in a native element with `data-testid`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Enable clock views in product docs with [`DatePickerFeatures::CLOCK_VIEWS`]
/// * Provide `reference_date` when the anchor day differs from locale defaults
///
/// ## Don'ts
///
/// * Do not use for date selection — prefer [`DateCalendar`](crate::DateCalendar)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Analog clock
/// Default 12-hour dial with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="time-clock-preview">
///         <PickerPreviewKnobs />
///         <TimeClock bind=value />
///         <div data-testid="time-clock-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Hour and minute selection
/// Click or drag on the dial to pick an hour, then a minute. Minute labels show five-minute increments.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{OrbitalDateTime, ToUnixSeconds};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="TC-02">
///         <TimeClock bind=value />
///         <div data-testid="TC-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour dial
/// Hour markers run 00–23 without meridiem controls.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="TC-03">
///         <TimeClock bind=value appearance=TimeClockAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Five-minute steps
/// Five-minute minute labels with coarser snap via `minute_step`.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="TC-04">
///         <TimeClock
///             bind=value
///             appearance=TimeClockAppearance {
///                 minute_step: Signal::from(5),
///                 ..Default::default()
///             }
///         />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "time-clock",
    preview_label = "Time Clock",
    preview_icon = icondata::AiClockCircleOutlined,
)]
#[component]
pub fn TimeClock(
    /// Value binding for the selected time-of-day.
    #[prop(optional, into)]
    bind: TimeClockBind,
    /// Dial format, minute step, reference day, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: TimeClockAppearance,
    /// Optional CSS class merged onto the layout root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let TimeClockBind { value } = bind;
    let TimeClockAppearance {
        ampm,
        minute_step,
        reference_date,
        timezone,
        disabled,
    } = appearance;

    let value = StoredValue::new(value);

    let locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();

    let resolved_timezone = Signal::derive(move || timezone.get());
    let resolved_reference = Signal::derive(move || reference_date.get().start_of_day());
    let anchor = move || {
        resolve_anchor(
            value.get_value().get(),
            resolved_reference.get(),
            resolved_timezone.get(),
        )
    };

    let view = RwSignal::new(ClockView::Hours);
    let draft_hour_24 = RwSignal::new(0u32);
    let draft_minute = RwSignal::new(0u32);
    let is_pm = RwSignal::new(false);

    Effect::new(move |_| {
        let tz = resolved_timezone.get();
        let anchor_day = anchor();
        if let Some(current) = value.get_value().get() {
            if let Some((hour, minute, _)) = current.hour_minute_second() {
                draft_hour_24.set(hour);
                draft_minute.set(minute);
                is_pm.set(hour >= 12);
                return;
            }
        }
        let (hour, minute, pm) = now_on_anchor(anchor_day);
        draft_hour_24.set(hour);
        draft_minute.set(minute);
        is_pm.set(pm);
        let _ = tz;
    });

    let on_minute_selected = Callback::new(move |(hour, minute): (u32, u32)| {
        if disabled.get_untracked() {
            return;
        }
        let step = minute_step.get_untracked().max(1);
        let snapped = snap_minute(minute, step);
        if let Some(committed) = commit_time(
            anchor(),
            hour,
            snapped,
            0,
            resolved_timezone.get_untracked(),
        ) {
            value.get_value().set(Some(committed));
        }
    });

    let root_class = move || {
        let mut parts = vec!["orb-picker-time-clock".to_string()];
        match theme_options.get().density {
            orbital_theme::Density::Compact => {
                parts.push("orb-picker-time-clock--density-compact".to_string())
            }
            orbital_theme::Density::Spacious => {
                parts.push("orb-picker-time-clock--density-spacious".to_string())
            }
            orbital_theme::Density::Default => {}
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        let _ = locale.locale;
        parts.join(" ")
    };

    view! {
        <style>{time_clock_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="" role="group" aria-label="Time clock">
            <TimeClockDial
                view=view
                draft_hour_24=draft_hour_24
                draft_minute=draft_minute
                is_pm=is_pm
                ampm=ampm
                minute_step=minute_step
                disabled=disabled
                on_minute_selected=on_minute_selected
            />
        </div>
    }
}
