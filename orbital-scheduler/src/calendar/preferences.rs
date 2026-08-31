//! [`SchedulerCalendarPreferences`] — preferences menu preview (SC-14).

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
    calendar::navigation::preview_anchor_date, preview::fixtures::sample_planned_events,
    scheduler_preferences_from_signals, ScheduleResource, SchedulerCalendar, SchedulerFeatures,
    SchedulerPreferencesMenu, SchedulerView,
};

/// Let users toggle weekend visibility, 12/24-hour time, and week start day on [`SchedulerCalendar`].
///
/// # When to use
///
/// - Personalization menus in scheduling apps
/// - Persisting display settings alongside user profiles
///
/// # Usage
///
/// 1. Build [`SchedulerPreferences`] from signals or a snapshot via `scheduler_preferences_from_signals`.
/// 2. Pass `preferences` to [`SchedulerCalendar`].
/// 3. Optionally listen to `on_preferences_change` to persist toggles.
///
/// # Best Practices
///
/// ## Do's
///
/// - Mirror preference signals in your own store if users expect settings to survive reloads.
///
/// ## Don'ts
///
/// - Do not confuse display preferences with `display_timezone` — timezone is a separate bind.
///
/// # Examples
///
/// ## Preferences menu
/// Toggle switches above the week grid update grid chrome immediately.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use leptos::html::Div;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     scheduler_preferences_from_signals, SchedulerCalendar, SchedulerFeatures,
///     SchedulerPreferencesMenu, SchedulerView,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(Vec::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let show_weekends = RwSignal::new(true);
/// let ampm = RwSignal::new(true);
/// let week_starts_on = RwSignal::new(0u8);
/// let preferences = scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on);
/// let popover_mount = NodeRef::<Div>::new();
/// view! {
///     <div data-testid="scheduler-calendar-preferences-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <div node_ref=popover_mount style="position: relative;" />
///             <SchedulerPreferencesMenu
///                 show_weekends=show_weekends
///                 ampm=ampm
///                 week_starts_on=week_starts_on
///                 mount=Some(popover_mount)
///             />
///             <div data-testid="scheduler-calendar-week-preview">
///                 <SchedulerCalendar
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     view=view
///                     features=SchedulerFeatures::empty()
///                     preferences=preferences
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-preferences",
    preview_label = "Scheduler Event Calendar Preferences",
    preview_icon = icondata::AiSettingOutlined,
)]
#[component]
pub fn SchedulerCalendarPreferences(
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
        let show_weekends = RwSignal::new(true);
        let ampm = RwSignal::new(true);
        let week_starts_on = RwSignal::new(0u8);
        let preferences = scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on);
        let popover_mount = NodeRef::<Div>::new();

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-preferences-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <div node_ref=popover_mount style="position: relative;" />
                    <SchedulerPreferencesMenu
                        show_weekends=show_weekends
                        ampm=ampm
                        week_starts_on=week_starts_on
                        mount=Some(popover_mount)
                    />
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
