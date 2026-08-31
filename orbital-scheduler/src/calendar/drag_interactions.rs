//! [`SchedulerCalendarDragInteractions`] — drag and resize preview (SC-08).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date, preview::fixtures::sample_planned_events,
    ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView,
};

/// Drag and resize timed events within the [`SchedulerCalendar`] week grid.
///
/// # When to use
///
/// - Rescheduling meetings by dragging chips vertically in the time grid
/// - Adjusting duration with top or bottom resize handles
///
/// # Usage
///
/// 1. Bind `events` as `RwSignal<Vec<PlannedEvent>>`.
/// 2. Set `are_events_draggable` and `are_events_resizable` on [`SchedulerCalendar`].
/// 3. Optionally set per-event `is_draggable` / `is_resizable` to lock specific chips.
/// 4. Listen to `on_events_change` to persist updated times.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep drag updates in the same `RwSignal` you pass to the calendar.
///
/// ## Don'ts
///
/// - Do not enable drag on read-only calendars without an `on_events_change` handler — users expect persistence feedback.
///
/// # Examples
///
/// ## Drag reschedule
/// Week view with draggable sample events; drag chips to move times or resize edges.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let mut sample = sample_planned_events();
/// if let Some(first) = sample.first_mut() {
///     first.is_draggable = Some(true);
/// }
/// let events = RwSignal::new(sample);
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-drag-interactions-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <div data-testid="scheduler-calendar-week-preview">
///                 <SchedulerCalendar
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     view=view
///                     features=SchedulerFeatures::empty()
///                     are_events_draggable=Signal::from(true)
///                     are_events_resizable=Signal::from(true)
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-drag-interactions",
    preview_label = "Scheduler Calendar Drag Interactions",
    preview_icon = icondata::AiDragOutlined,
)]
#[component]
pub fn SchedulerCalendarDragInteractions(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let mut sample = sample_planned_events();
        if let Some(first) = sample.first_mut() {
            first.is_draggable = Some(true);
        }
        let events = RwSignal::new(sample);
        let resources = RwSignal::new(Vec::<ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-drag-interactions-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <div data-testid="scheduler-calendar-week-preview">
                        <SchedulerCalendar
                            events=events
                            resources=resources
                            visible_date=visible_date
                            display_timezone=display_timezone
                            view=view
                            features=SchedulerFeatures::empty()
                            are_events_draggable=Signal::from(true)
                            are_events_resizable=Signal::from(true)
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
