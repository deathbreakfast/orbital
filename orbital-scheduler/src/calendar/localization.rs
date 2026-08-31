//! [`SchedulerCalendarLocalization`] — locale text preview (SC-15).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date, locale_text_signal,
    preview::fixtures::sample_planned_events, scheduler_preferences_from_signals, DatetimeLocale,
    ScheduleResource, SchedulerCalendar, SchedulerChromeContext, SchedulerFeatures,
    SchedulerLocaleText, SchedulerView,
};

/// Override scheduler toolbar and overlay strings for localization.
///
/// # When to use
///
/// - French, English, or custom label sets on calendar chrome
/// - Product copy that differs from default English strings
///
/// # Usage
///
/// 1. Provide [`SchedulerLocaleText`] through [`SchedulerChromeContext`] or the `locale_text` prop.
/// 2. Wrap pickers in [`DatetimeLocale`] when editing uses Calendar & Time components.
/// 3. Switch locale by updating the locale text signal — toolbar labels react immediately.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep datetime parsing on [`OrbitalDateTime`] — locale text affects labels only, not instants.
///
/// ## Don'ts
///
/// - Do not confuse scheduler locale strings with [`DatetimeLocale`] format/timezone — they compose together.
///
/// # Examples
///
/// ## English and French toolbar
/// Select switches between built-in English and French label sets.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};
/// use crate::{
///     locale_text_signal, scheduler_preferences_from_signals, SchedulerCalendar, SchedulerChromeContext,
///     SchedulerFeatures, SchedulerLocaleText, SchedulerView, DatetimeLocale,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(Vec::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let locale_pick = RwSignal::new("en".to_string());
/// let (locale_rw, _) = locale_text_signal(SchedulerLocaleText::english());
/// let preferences = scheduler_preferences_from_signals(RwSignal::new(true), RwSignal::new(true), RwSignal::new(0));
/// Effect::new(move |_| {
///     locale_rw.set(match locale_pick.get().as_str() {
///         "fr" => SchedulerLocaleText::french(),
///         _ => SchedulerLocaleText::english(),
///     });
/// });
/// SchedulerChromeContext { preferences, locale_text: locale_rw }.provide();
/// view! {
///     <div data-testid="scheduler-calendar-localization-preview">
///         <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
///             <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///                 <ThemeDensityStepper />
///                 <Select bind=locale_pick attr:data-testid="scheduler-locale-select">
///                     <option value="en">"English"</option>
///                     <option value="fr">"Français"</option>
///                 </Select>
///                 <div data-testid="scheduler-calendar-week-preview">
///                     <SchedulerCalendar
///                         events=events
///                         resources=resources
///                         visible_date=visible_date
///                         display_timezone=display_timezone
///                         view=view
///                         features=SchedulerFeatures::empty()
///                     />
///                 </div>
///             </Flex>
///         </DatetimeLocale>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-localization",
    preview_label = "Scheduler Event Calendar Localization",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn SchedulerCalendarLocalization(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(sample_planned_events());
        let resources = RwSignal::new(Vec::<ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);
        let locale_pick = RwSignal::new("en".to_string());
        let (locale_rw, _) = locale_text_signal(SchedulerLocaleText::english());
        let show_weekends = RwSignal::new(true);
        let ampm = RwSignal::new(true);
        let week_starts_on = RwSignal::new(0u8);
        let preferences = scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on);

        Effect::new(move |_| {
            locale_rw.set(match locale_pick.get().as_str() {
                "fr" => SchedulerLocaleText::french(),
                _ => SchedulerLocaleText::english(),
            });
        });

        SchedulerChromeContext {
            preferences,
            locale_text: locale_rw,
        }
        .provide();

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-localization-preview">
                <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
                    <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                        <ThemeDensityStepper />
                        <div data-testid="scheduler-locale-select">
                            <Select bind=locale_pick>
                                <option value="en">"English"</option>
                                <option value="fr">"Français"</option>
                            </Select>
                        </div>
                        <div data-testid="scheduler-calendar-week-preview">
                            <SchedulerCalendar
                                events=events
                                resources=resources
                                visible_date=visible_date
                                display_timezone=display_timezone
                                view=view
                                features=SchedulerFeatures::empty()
                                preferences=preferences
                            />
                        </div>
                    </Flex>
                </DatetimeLocale>
                {children.map(|c| c())}
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = (&class, &children);
        ().into_any()
    }
}
