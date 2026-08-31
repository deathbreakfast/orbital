//! Doc-section preview for timeline presets (SC-22).

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

/// Switch timeline zoom between day, business-day, week, and business-week presets.
///
/// [`TimelinePreset`] controls column width and navigation step size on [`SchedulerTimeline`].
///
/// # When to use
///
/// - Hourly detail for single-day planning
/// - Multi-day or multi-week overviews for capacity planning
///
/// # Usage
///
/// 1. Bind `preset: RwSignal<TimelinePreset>` on [`SchedulerTimeline`].
/// 2. Let users change preset via the built-in toolbar select or set the signal from your own controls.
/// 3. Keep `visible_date` aligned when switching presets so the anchor day stays in view.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use `TimelinePreset::Week` as the default for resource planning over several days.
///
/// ## Don'ts
///
/// - Do not assume all presets show the same events — very short events may be hidden at coarse zoom.
///
/// # Examples
///
/// ## Day and week zoom
/// Preset select switches between hourly day view and daily week view.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Day);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-presets-preview">
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
    preview_slug = "scheduler-timeline-presets",
    preview_label = "Scheduler Event Timeline Presets",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn SchedulerTimelinePresets(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Day);
        let events = RwSignal::new(sample_timeline_events());
        let resources = RwSignal::new(sample_nested_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div data-testid="scheduler-timeline-presets-preview" class=move || class.get().unwrap_or_default()>
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
