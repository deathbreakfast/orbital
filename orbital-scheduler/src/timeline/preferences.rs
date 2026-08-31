//! [`SchedulerTimelinePreferences`] — timeline preferences preview (SC-27).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use leptos::html::Div;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    preview::fixtures::sample_schedule_resources, preview_anchor_date,
    scheduler_preferences_from_signals, SchedulerFeatures, SchedulerPreferencesMenu,
    SchedulerTimeline, TimelinePreset,
};

/// Let users toggle weekend visibility, 12/24-hour time, and week start day on [`SchedulerTimeline`].
///
/// # When to use
///
/// - Personalization for Gantt-style timelines shared with calendar preferences
/// - Day preset views where AM/PM vs 24-hour labels matter
///
/// # Usage
///
/// 1. Build [`SchedulerPreferences`] via `scheduler_preferences_from_signals` or a snapshot.
/// 2. Pass `preferences` to [`SchedulerTimeline`].
/// 3. Wire [`SchedulerPreferencesMenu`] toggles to the same signals for live updates.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use the same preference model on calendar and timeline when both appear in one app.
///
/// ## Don'ts
///
/// - Do not store preferences only in local component state if users expect account-level settings.
///
/// # Examples
///
/// ## Timeline preferences menu
/// Toggle switches update time gutter and header chrome on a day preset timeline.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use leptos::html::Div;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     scheduler_preferences_from_signals, SchedulerFeatures, SchedulerPreferencesMenu,
///     SchedulerTimeline, TimelinePreset,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Day);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let show_weekends = RwSignal::new(true);
/// let ampm = RwSignal::new(true);
/// let week_starts_on = RwSignal::new(0u8);
/// let preferences = scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on);
/// let popover_mount = NodeRef::<Div>::new();
/// view! {
///     <div data-testid="scheduler-timeline-preferences-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <div
///                 data-testid="scheduler-timeline-preferences-timeline-host"
///                 style="height: 420px; display: flex; flex-direction: column; min-height: 0;"
///             >
///                 <SchedulerTimeline
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     preset=preset
///                     features=SchedulerFeatures::TIMELINE
///                     preferences=preferences
///                 />
///             </div>
///             <div node_ref=popover_mount style="position: relative;" />
///             <SchedulerPreferencesMenu
///                 show_weekends=show_weekends
///                 ampm=ampm
///                 week_starts_on=week_starts_on
///                 mount=Some(popover_mount)
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-preferences",
    preview_label = "Scheduler Event Timeline Preferences",
    preview_icon = icondata::AiSettingOutlined,
)]
#[component]
pub fn SchedulerTimelinePreferences(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Day);
        let events = RwSignal::new(crate::preview::fixtures::sample_timeline_events());
        let resources = RwSignal::new(sample_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);
        let show_weekends = RwSignal::new(true);
        let ampm = RwSignal::new(true);
        let week_starts_on = RwSignal::new(0u8);
        let preferences = scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on);
        let popover_mount = NodeRef::<Div>::new();

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-timeline-preferences-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <div node_ref=popover_mount style="position: relative;" />
                    <SchedulerPreferencesMenu
                        show_weekends=show_weekends
                        ampm=ampm
                        week_starts_on=week_starts_on
                        mount=Some(popover_mount)
                    />
                    <div
                        data-testid="scheduler-timeline-preferences-timeline-host"
                        style="height: 420px; display: flex; flex-direction: column; min-height: 0;"
                    >
                        <SchedulerTimeline
                            events=events
                            resources=resources
                            visible_date=visible_date
                            display_timezone=display_timezone
                            preset=preset
                            features=SchedulerFeatures::TIMELINE
                            preferences=preferences
                        />
                    </div>
                </Flex>
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
