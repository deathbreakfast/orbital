//! Doc-section preview for timeline resources (SC-18).

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

/// Assign events to nested resource lanes on [`SchedulerTimeline`].
///
/// [`ScheduleResource`] supports hierarchical `children` — each row renders as an indented lane label.
///
/// # When to use
///
/// - Team, room, or equipment hierarchies where events belong to a specific row
/// - Gantt views where every bar must map to a resource id
///
/// # Usage
///
/// 1. Bind `resources: RwSignal<Vec<ScheduleResource>>` — required on timeline (unlike calendar).
/// 2. Nest sub-resources via `children` on each [`ScheduleResource`].
/// 3. Set `PlannedEvent.resource_id` to the target resource `id`.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep resource `id` values stable across fetches so event assignments survive reloads.
///
/// ## Don'ts
///
/// - Do not render a timeline without resources — lanes are mandatory for this product.
///
/// # Examples
///
/// ## Team rows
/// Nested building and room resources with events scoped to each lane.
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
///     <div data-testid="scheduler-timeline-resources-preview">
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
    preview_slug = "scheduler-timeline-resources",
    preview_label = "Scheduler Event Timeline Resources",
    preview_icon = icondata::AiTeamOutlined,
)]
#[component]
pub fn SchedulerTimelineResources(
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
            <div data-testid="scheduler-timeline-resources-preview" class=move || class.get().unwrap_or_default()>
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
