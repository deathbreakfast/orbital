//! [`DigitalClock`] — scrollable time list bound to [`OrbitalDateTime`].

mod styles;

use leptos::prelude::*;
use orbital_base_components::{ListNavigationMode, ListSelectionMode};
use orbital_core_components::{List, ListItem, ScrollArea};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{
    commit_time, format_slot_label, generate_time_slots, picker_style_sheet, resolve_anchor,
    use_datetime_locale,
};

use super::field_types::{DigitalClockAppearance, DigitalClockBind};
use styles::digital_clock_styles;

/// Scrollable time list for selecting time-of-day, bound to [`OrbitalDateTime`].
///
/// DigitalClock renders a vertical list of time slots at `appearance.time_step` increments.
/// Values anchor to `appearance.reference_date` or the nearest [`DatetimeLocale`](crate::DatetimeLocale)
/// default. Clock surfaces are optional product features documented via
/// [`DatePickerFeatures::CLOCK_VIEWS`] — there is no runtime license check.
///
/// # When to use
///
/// - Compact time selection in picker panels
/// - Alternatives to scroll-column [`TimePicker`](orbital_core_components::TimePicker) surfaces
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` via [`DigitalClockBind`].
/// 2. Set `appearance.time_step` for list increment (default 30 minutes).
/// 3. Wrap preview examples in a native element with `data-testid`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Enable clock views in product docs with [`DatePickerFeatures::CLOCK_VIEWS`]
/// * Use 15- or 30-minute steps for scheduling UIs
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Digital list
/// Scroll a 30-minute slot list; the readout above shows your selection (preview also prints unix for E2E).
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, OrbitalDateTime, ToUnixSeconds, format_datetime};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="digital-clock-preview">
///         <PickerPreviewKnobs />
///         <DigitalClock bind=value />
///         <div data-testid="digital-clock-preview-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///         <div data-testid="digital-clock-preview-LABEL">{move || value.get().map(|v| format_datetime(v, DatetimeFormat::Time12)).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## List selection
/// Click a time slot to commit the bound value.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, OrbitalDateTime, ToUnixSeconds, format_datetime};
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DC-02">
///         <DigitalClock bind=value />
///         <div data-testid="DC-02-VALUE">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///         <div data-testid="DC-02-LABEL">{move || value.get().map(|v| format_datetime(v, DatetimeFormat::Time12)).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Fifteen-minute steps
/// List items appear every 15 minutes.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DC-03">
///         <DigitalClock
///             bind=value
///             appearance=DigitalClockAppearance {
///                 time_step: Signal::from(15),
///                 ..Default::default()
///             }
///         />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## 24-hour labels
/// Slots display 24-hour formatted labels.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::OrbitalDateTime;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="DC-04">
///         <DigitalClock bind=value appearance=DigitalClockAppearance::time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "digital-clock",
    preview_label = "Digital Clock",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn DigitalClock(
    /// Value binding for the selected time-of-day.
    #[prop(optional, into)]
    bind: DigitalClockBind,
    /// List step, label format, reference day, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DigitalClockAppearance,
    /// Optional CSS class merged onto the layout root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DigitalClockBind { value } = bind;
    let DigitalClockAppearance {
        time_step,
        ampm,
        reference_date,
        timezone,
        disabled,
    } = appearance;

    let value = StoredValue::new(value);
    let locale = untrack(use_datetime_locale);
    let theme_options = use_theme_options();

    let resolved_timezone = Signal::derive(move || timezone.get());
    let resolved_reference = Signal::derive(move || reference_date.get().start_of_day());

    let slots = Signal::derive(move || generate_time_slots(time_step.get().max(1)));

    let selected_slot = Signal::derive(move || {
        value
            .get_value()
            .get()
            .and_then(|dt| dt.hour_minute_second())
            .map(|(h, m, _)| (h, m))
    });

    let selection_label = Signal::derive(move || {
        value
            .get_value()
            .get()
            .and_then(|dt| dt.hour_minute_second())
            .map(|(hour, minute, _)| format_slot_label(hour, minute, ampm.get()))
    });

    let root_class = move || {
        let mut parts = vec!["orb-picker-digital-clock".to_string()];
        match theme_options.get().density {
            orbital_theme::Density::Compact => {
                parts.push("orb-picker-digital-clock--density-compact".to_string())
            }
            orbital_theme::Density::Spacious => {
                parts.push("orb-picker-digital-clock--density-spacious".to_string())
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

    let select_slot = move |hour: u32, minute: u32| {
        if disabled.get_untracked() {
            return;
        }
        let anchor = resolve_anchor(
            value.get_value().get_untracked(),
            resolved_reference.get_untracked(),
            resolved_timezone.get_untracked(),
        );
        if let Some(committed) =
            commit_time(anchor, hour, minute, 0, resolved_timezone.get_untracked())
        {
            value.get_value().set(Some(committed));
        }
    };

    view! {
        <style>{digital_clock_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="" role="group" aria-label="Digital clock">
            <div
                class=move || {
                    if selection_label.get().is_some() {
                        "orb-picker-digital-clock__readout".to_string()
                    } else {
                        "orb-picker-digital-clock__readout orb-picker-digital-clock__readout--placeholder".to_string()
                    }
                }
                aria-live="polite"
            >
                {move || selection_label.get().unwrap_or_else(|| "Select a time".to_string())}
            </div>
            <ScrollArea class="orb-picker-digital-clock__scroll">
                <List
                    navigation_mode=ListNavigationMode::Nav
                    selection_mode=ListSelectionMode::Single
                >
                    {move || {
                        let use_ampm = ampm.get();
                        slots
                            .get()
                            .into_iter()
                            .map(|(hour, minute)| {
                                let label = format_slot_label(hour, minute, use_ampm);
                                let is_selected = selected_slot.get() == Some((hour, minute));
                                view! {
                                    <ListItem
                                        class="orb-picker-digital-clock__item"
                                        selected=Signal::from(is_selected)
                                        on_click=Callback::new(move |_| select_slot(hour, minute))
                                    >
                                        {label}
                                    </ListItem>
                                }
                            })
                            .collect_view()
                    }}
                </List>
            </ScrollArea>
        </div>
    }
}
