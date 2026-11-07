//! Doc-section preview for timeline markers: today line, point events, tone,
//! progress and collapsible rows.

use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};

#[cfg(feature = "preview")]
use crate::preview::fixtures::{sample_point_events, sample_schedule_resources};
#[cfg(feature = "preview")]
use crate::preview_anchor_date;

/// Draw a project plan on [`SchedulerTimeline`]: a today line, diamonds for
/// one-day events, bars coloured by tone with a progress fill, and parent rows
/// that fold their children away.
///
/// A milestone is a parent [`ScheduleResource`](crate::ScheduleResource) with
/// its tasks as children. Give the milestone's own bar a `progress`, give each
/// task a bar from start to due, and give a task that only has a due date an
/// event with `start == end` so it draws as a diamond.
///
/// # When to use
///
/// - Project timelines with milestones, tasks and deadlines
/// - Any timeline where late or at-risk work needs to stand out
///
/// # Usage
///
/// 1. Pass `today` from the server so the line is in the first paint and tests
///    can pin the date. Without it the line appears after hydration.
/// 2. Set [`PlannedEvent::tone`](crate::PlannedEvent::tone) with
///    [`PlannedEvent::with_tone`](crate::PlannedEvent::with_tone) and progress
///    with [`PlannedEvent::with_progress`](crate::PlannedEvent::with_progress).
/// 3. Nest tasks under milestones in `resources`; bind `collapsed_resources`
///    to keep the folded rows outside the timeline.
/// 4. Use [`TimelinePreset::Month`] or [`TimelinePreset::Quarter`] for plans
///    longer than a week.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use [`EventTone::Danger`](crate::EventTone::Danger) for late work and keep
///   the other tones for status the reader acts on.
/// - Put counts such as "9 of 12" in `scheduler_event_content`; `progress` only draws the fill.
///
/// ## Don'ts
///
/// - Do not set `color` when you want a tone: an explicit `color` wins.
/// - Do not expect point events to drag or resize; they open the edit dialog only.
///
/// # Examples
///
/// ## Today line and point events
/// A fixed `today` draws the red line in server-rendered HTML too. Events whose
/// start equals their end draw as diamonds with their title beside them.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_point_events, sample_schedule_resources};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_point_events());
/// let resources = RwSignal::new(sample_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-markers-today">
///         <SchedulerTimeline
///             events=events
///             resources=resources
///             visible_date=visible_date
///             display_timezone=display_timezone
///             preset=preset
///             today=preview_anchor_date()
///             features=SchedulerFeatures::TIMELINE
///         />
///     </div>
/// }
/// ```
///
/// ## Tone and progress
/// Each bar takes its colours from its tone; the darker share is its progress.
/// "Custom colour" sets `color`, which wins over its danger tone.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_schedule_resources, sample_toned_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_toned_events());
/// let resources = RwSignal::new(sample_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-markers-tone">
///         <SchedulerTimeline
///             events=events
///             resources=resources
///             visible_date=visible_date
///             display_timezone=display_timezone
///             preset=preset
///             today=preview_anchor_date()
///             features=SchedulerFeatures::TIMELINE
///         />
///     </div>
/// }
/// ```
///
/// ## Collapsible rows
/// Milestones are parent rows with their tasks as children. The chevron folds a
/// milestone's tasks away; the milestone's own bar stays. The folded ids live in
/// a signal the host owns.
/// <!-- preview -->
/// ```rust
/// use std::collections::HashSet;
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_milestone_events, sample_milestone_resources};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Month);
/// let events = RwSignal::new(sample_milestone_events());
/// let resources = RwSignal::new(sample_milestone_resources());
/// let collapsed = RwSignal::new(HashSet::<String>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-markers-collapse">
///         <SchedulerTimeline
///             events=events
///             resources=resources
///             visible_date=visible_date
///             display_timezone=display_timezone
///             preset=preset
///             today=preview_anchor_date()
///             collapsed_resources=collapsed
///             features=SchedulerFeatures::TIMELINE
///         />
///     </div>
/// }
/// ```
///
/// ## Month and quarter zoom
/// The quarter preset shows one column per week starting Monday; the toolbar
/// select switches to a month of day columns. On a narrow screen the lanes
/// scroll sideways and the label column stays.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use crate::{SchedulerFeatures, SchedulerTimeline, TimelinePreset};
/// use crate::preview::fixtures::{sample_milestone_events, sample_milestone_resources};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Quarter);
/// let events = RwSignal::new(sample_milestone_events());
/// let resources = RwSignal::new(sample_milestone_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-markers-zoom">
///         <SchedulerTimeline
///             events=events
///             resources=resources
///             visible_date=visible_date
///             display_timezone=display_timezone
///             preset=preset
///             today=preview_anchor_date()
///             features=SchedulerFeatures::TIMELINE
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-markers",
    preview_label = "Scheduler Event Timeline Markers",
    preview_icon = icondata::AiFlagOutlined,
)]
#[component]
pub fn SchedulerTimelineMarkers(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(sample_point_events());
        let resources = RwSignal::new(sample_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div data-testid="scheduler-timeline-markers-today" class=move || class.get().unwrap_or_default()>
                <SchedulerTimeline
                    events=events
                    resources=resources
                    visible_date=visible_date
                    display_timezone=display_timezone
                    preset=preset
                    today=preview_anchor_date()
                    features=SchedulerFeatures::TIMELINE
                />
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
