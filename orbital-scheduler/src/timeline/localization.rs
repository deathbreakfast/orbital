//! [`SchedulerTimelineLocalization`] — locale text preview (SC-28).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    locale_text_signal, preview::fixtures::sample_nested_schedule_resources, preview_anchor_date,
    DatetimeLocale, SchedulerChromeContext, SchedulerFeatures, SchedulerLocaleText,
    SchedulerTimeline, TimelinePreset,
};

/// Override timeline toolbar and overlay strings for localization.
///
/// # When to use
///
/// - Localized prev/next/today labels on timeline chrome
/// - Matching calendar and timeline copy in multilingual apps
///
/// # Usage
///
/// 1. Provide [`SchedulerLocaleText`] via [`SchedulerChromeContext`] or the `locale_text` prop.
/// 2. Wrap the tree in [`DatetimeLocale`] when editing uses pickers.
/// 3. Update the locale text signal when the user picks a language.
///
/// # Best Practices
///
/// ## Do's
///
/// - Share one locale text signal when calendar and timeline sit on the same page.
///
/// ## Don'ts
///
/// - Do not hard-code toolbar strings in app chrome — use `SchedulerLocaleText` fields for consistency.
///
/// # Examples
///
/// ## English and French toolbar
/// Language select switches built-in English and French timeline labels.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, Select, ThemeDensityStepper};
/// use crate::{
///     locale_text_signal, scheduler_preferences_from_signals, SchedulerChromeContext,
///     SchedulerFeatures, SchedulerLocaleText, SchedulerTimeline, TimelinePreset, DatetimeLocale,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
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
///     <div data-testid="scheduler-timeline-localization-preview">
///         <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
///             <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///                 <ThemeDensityStepper />
///                 <Select bind=locale_pick attr:data-testid="scheduler-locale-select">
///                     <option value="en">"English"</option>
///                     <option value="fr">"Français"</option>
///                 </Select>
///                 <SchedulerTimeline
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     preset=preset
///                     features=SchedulerFeatures::TIMELINE
///                 />
///             </Flex>
///         </DatetimeLocale>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-localization",
    preview_label = "Scheduler Event Timeline Localization",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn SchedulerTimelineLocalization(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(crate::preview::fixtures::sample_timeline_events());
        let resources = RwSignal::new(sample_nested_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);
        let locale_pick = RwSignal::new("en".to_string());
        let (locale_rw, _) = locale_text_signal(SchedulerLocaleText::english());

        Effect::new(move |_| {
            locale_rw.set(match locale_pick.get().as_str() {
                "fr" => SchedulerLocaleText::french(),
                _ => SchedulerLocaleText::english(),
            });
        });

        SchedulerChromeContext {
            preferences: crate::scheduler_preferences_from_signals(
                RwSignal::new(true),
                RwSignal::new(true),
                RwSignal::new(0),
            ),
            locale_text: locale_rw,
        }
        .provide();

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-timeline-localization-preview">
                <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
                    <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                        <ThemeDensityStepper />
                        <div data-testid="scheduler-locale-select">
                            <Select bind=locale_pick>
                                <option value="en">"English"</option>
                                <option value="fr">"Français"</option>
                            </Select>
                        </div>
                        <SchedulerTimeline
                            events=events
                            resources=resources
                            visible_date=visible_date
                            display_timezone=display_timezone
                            preset=preset
                            features=SchedulerFeatures::TIMELINE
                        />
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
