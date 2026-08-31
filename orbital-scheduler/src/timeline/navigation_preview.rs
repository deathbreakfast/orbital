//! Doc-section preview for timeline navigation (SC-19).

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexGap, ThemeDensityStepper,
};
use orbital_macros::component_doc;

use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
use crate::preview_anchor_date;
use crate::{SchedulerFeatures, SchedulerTimeline, SchedulerTimelineHandle, TimelinePreset};

/// Move the visible timeline range with toolbar controls or an imperative handle.
///
/// Navigation steps follow the active [`TimelinePreset`] — day presets shift by hours, week presets by days.
///
/// # When to use
///
/// - Wiring external "today" or paging buttons outside the timeline toolbar
/// - Syncing `visible_date` with URL query params or parent view state
///
/// # Usage
///
/// 1. Bind `visible_date: RwSignal<OrbitalDateTime>` on [`SchedulerTimeline`].
/// 2. Bind `preset: RwSignal<TimelinePreset>` so prev/next step sizes match zoom level.
/// 3. Capture [`SchedulerTimelineHandle`] via `on_handle` for imperative `go_to_today`, `go_to_previous`, and `go_to_next`.
///
/// # Best Practices
///
/// ## Do's
///
/// - Store `visible_date` at start-of-day in your display timezone for stable titles.
///
/// ## Don'ts
///
/// - Do not mix preset changes without updating `visible_date` — align both signals when jumping views.
///
/// # Examples
///
/// ## External paging buttons
/// Buttons outside the toolbar call handle callbacks to move the visible range.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Button, ButtonAppearance, Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     SchedulerFeatures, SchedulerTimeline, SchedulerTimelineHandle, TimelinePreset,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let handle = RwSignal::new(None::<SchedulerTimelineHandle>);
/// view! {
///     <div data-testid="scheduler-timeline-navigation-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Flex gap=FlexGap::Small align=FlexAlign::Center>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_previous.run(()); }
///                 }>"Previous"</Button>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_today.run(()); }
///                 }>"Today"</Button>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_next.run(()); }
///                 }>"Next"</Button>
///             </Flex>
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::TIMELINE
///                 on_handle=Callback::new(move |h| handle.set(Some(h)))
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-navigation",
    preview_label = "Scheduler Event Timeline Navigation",
    preview_icon = icondata::AiCompassOutlined,
)]
#[component]
pub fn SchedulerTimelineNavigation(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let visible_date = RwSignal::new(preview_anchor_date());
    let preset = RwSignal::new(TimelinePreset::Week);
    let events = RwSignal::new(sample_timeline_events());
    let resources = RwSignal::new(sample_nested_schedule_resources());
    let display_timezone = RwSignal::new(DatetimeTimezone::Local);
    let handle = RwSignal::new(None::<SchedulerTimelineHandle>);

    view! {
        <div data-testid="scheduler-timeline-navigation-preview" class=move || class.get().unwrap_or_default()>
            <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                <ThemeDensityStepper />
                <Flex gap=FlexGap::Small align=FlexAlign::Center>
                    <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
                        if let Some(h) = handle.get() { h.go_to_previous.run(()); }
                    }>"External Previous"</Button>
                    <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
                        if let Some(h) = handle.get() { h.go_to_today.run(()); }
                    }>"External Today"</Button>
                    <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
                        if let Some(h) = handle.get() { h.go_to_next.run(()); }
                    }>"External Next"</Button>
                </Flex>
                <SchedulerTimeline
                    events=events
                    resources=resources
                    visible_date=visible_date
                    display_timezone=display_timezone
                    preset=preset
                    features=SchedulerFeatures::TIMELINE
                    on_handle=Callback::new(move |h| handle.set(Some(h)))
                />
            </Flex>
        </div>
    }
}
