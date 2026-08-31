//! Doc-section preview for timeline events (SC-17).

use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};

#[cfg(feature = "preview")]
use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
#[cfg(feature = "preview")]
use crate::preview_anchor_date;

/// Render planned events as horizontal bars on resource timeline lanes.
///
/// Each [`PlannedEvent`] spans from `start` to `end` across the active [`TimelinePreset`].
/// Events without a matching `resource_id` are hidden on the timeline.
///
/// # When to use
///
/// - Showing multi-day projects, releases, or bookings on resource rows
/// - Coloring bars by category or status via event styling props
///
/// # Usage
///
/// 1. Bind `events: RwSignal<Vec<PlannedEvent>>` on [`SchedulerTimeline`].
/// 2. Set each event's `resource_id` to a [`ScheduleResource`] `id` in the bound `resources` list.
/// 3. Enable [`SchedulerFeatures::TIMELINE`] in `features`.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use [`OrbitalDateTime`] for `start` and `end` — the bar width is computed from instants.
/// - Assign distinct `color` or `class` on events when lanes are dense.
///
/// ## Don'ts
///
/// - Do not omit `resource_id` for timeline events — they will not render on any lane.
///
/// # Examples
///
/// ## Multi-day bars
/// Week preset with a spanning release window on nested resource lanes.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-events-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::TIMELINE
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-events",
    preview_label = "Scheduler Event Timeline Events",
    preview_icon = icondata::AiScheduleOutlined,
)]
#[component]
pub fn SchedulerTimelineEvents(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(sample_timeline_events());
        let resources = RwSignal::new(sample_nested_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div data-testid="scheduler-timeline-events-preview" class=move || class.get().unwrap_or_default()>
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <SchedulerTimeline
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        preset=preset
                        features=SchedulerFeatures::TIMELINE
                    />
                </Flex>
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = &class;
        ().into_any()
    }
}
