//! [`SchedulerTimelineDragInteractions`] — horizontal drag preview (SC-20).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events},
    preview_anchor_date, SchedulerFeatures, SchedulerTimeline, TimelinePreset,
};

/// Drag timeline event bars to reschedule, or pull start/end handles to change duration.
///
/// For the full timeline setup see [`SchedulerTimeline`]. This page covers drag and resize props only.
///
/// # When to use
///
/// - Rescheduling work across days or hours on a resource timeline
/// - Adjusting event duration by dragging bar edges
///
/// # Usage
///
/// 1. Bind `events` as `RwSignal<Vec<PlannedEvent>>` on [`SchedulerTimeline`].
/// 2. Set `are_events_draggable` and `are_events_resizable` to enable interactions globally.
/// 3. Optionally set `is_draggable` / `is_resizable` on individual [`PlannedEvent`] rows to opt out.
/// 4. Pass `on_events_change` to persist updated `start` / `end` [`OrbitalDateTime`] values.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep event times as [`OrbitalDateTime`] in your model — convert at API boundaries only.
/// - Wrap editing flows in [`DatetimeLocale`] when users also open the event dialog.
///
/// ## Don'ts
///
/// - Do not store unix seconds on `PlannedEvent` — use the typed datetime primitive.
///
/// # Examples
///
/// ## Move and resize bars
/// Week preset with global drag and resize enabled on sample events.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     PlannedEvent, ScheduleResource, SchedulerFeatures, SchedulerTimeline, TimelinePreset,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let mut sample = sample_timeline_events();
/// if let Some(first) = sample.first_mut() {
///     first.is_draggable = Some(true);
///     first.is_resizable = Some(true);
/// }
/// let events = RwSignal::new(sample);
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-drag-interactions-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::TIMELINE
///                 are_events_draggable=Signal::from(true)
///                 are_events_resizable=Signal::from(true)
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-drag-interactions",
    preview_label = "Scheduler Timeline Drag Interactions",
    preview_icon = icondata::AiDragOutlined,
)]
#[component]
pub fn SchedulerTimelineDragInteractions(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let mut sample = sample_timeline_events();
        if let Some(first) = sample.first_mut() {
            first.is_draggable = Some(true);
            first.is_resizable = Some(true);
        }
        let events = RwSignal::new(sample);
        let resources = RwSignal::new(sample_nested_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-timeline-drag-interactions-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <SchedulerTimeline
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        preset=preset
                        features=SchedulerFeatures::TIMELINE
                        are_events_draggable=Signal::from(true)
                        are_events_resizable=Signal::from(true)
                    />
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
