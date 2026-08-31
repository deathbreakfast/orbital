//! Doc-section preview for timeline virtualization (SC-24).

use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::ThemeDensityStepper;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};

#[cfg(feature = "preview")]
use crate::preview::fixtures::{large_schedule_resources, sample_timeline_events};
#[cfg(feature = "preview")]
use crate::preview_anchor_date;

/// Scroll large resource lists efficiently on [`SchedulerTimeline`].
///
/// Only visible resource rows mount in the DOM — suitable for hundreds of lanes in a fixed-height container.
///
/// # When to use
///
/// - Operations dashboards with many teams, rooms, or machines
/// - Embedded timelines inside dialogs or split panes with bounded height
///
/// # Usage
///
/// 1. Place [`SchedulerTimeline`] inside a flex child with `min-height: 0` and a fixed or max height.
/// 2. Bind a large `resources` vector — virtualization activates automatically for timeline body rows.
/// 3. Keep `events` scoped to visible resources to minimize layout work.
///
/// # Best Practices
///
/// ## Do's
///
/// - Give the timeline container an explicit height so the scroll region can calculate window size.
///
/// ## Don'ts
///
/// - Do not rely on page-level scrolling alone — the timeline needs a bounded flex column.
///
/// # Examples
///
/// ## Large resource set
/// One hundred twenty resource rows in a bounded timeline container.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{large_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(large_schedule_resources(120));
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div
///         data-testid="scheduler-timeline-virtualization-preview"
///         style="height: 480px; display: flex; flex-direction: column; min-height: 0;"
///     >
///         <ThemeDensityStepper />
///         <div style="flex: 1 1 auto; min-height: 0; display: flex; flex-direction: column; overflow: hidden;">
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::TIMELINE
///             />
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-virtualization",
    preview_label = "Scheduler Event Timeline Virtualization",
    preview_icon = icondata::AiBlockOutlined,
)]
#[component]
pub fn SchedulerTimelineVirtualization(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(sample_timeline_events());
        let resources = RwSignal::new(large_schedule_resources(120));
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div
                data-testid="scheduler-timeline-virtualization-preview"
                class=move || class.get().unwrap_or_default()
                style="height: 480px; display: flex; flex-direction: column; min-height: 0;"
            >
                <ThemeDensityStepper />
                <div style="flex: 1 1 auto; min-height: 0; display: flex; flex-direction: column; overflow: hidden;">
                    <SchedulerTimeline
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        preset=preset
                        features=SchedulerFeatures::TIMELINE
                    />
                </div>
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
