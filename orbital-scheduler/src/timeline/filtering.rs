//! [`SchedulerTimelineFiltering`] — timeline filtering (not yet shipped).

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::DeferredFeatureNotice;

/// Built-in timeline event filtering is not yet available in Orbital.
///
/// # When to use
///
/// - Planning filter-by-resource or filter-by-date-range UX on Gantt timelines
/// - Documenting the intended catalog entry until the feature ships
///
/// # Usage
///
/// Filter [`PlannedEvent`] and [`ScheduleResource`] collections before binding them to
/// [`SchedulerTimeline`], or push predicates into your [`SchedulerDataSource`] fetch.
///
/// # Best Practices
///
/// ## Do's
///
/// - Narrow `resources` first when users pick a team — drop unrelated lanes before render.
///
/// ## Don'ts
///
/// - Do not expect built-in filter chips on [`SchedulerTimeline`] today.
///
/// # Examples
///
/// ## Not yet available
/// <!-- preview -->
/// ```rust
/// use crate::SchedulerTimelineFiltering;
/// view! {
///     <SchedulerTimelineFiltering />
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-filtering",
    preview_label = "Scheduler Event Timeline Filtering",
    preview_icon = icondata::AiFilterOutlined,
)]
#[component]
pub fn SchedulerTimelineFiltering(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-testid="scheduler-timeline-filtering-preview" class=move || class.get().unwrap_or_default()>
            <DeferredFeatureNotice
                sc_id="SC-25"
                feature_name="Timeline filtering"
                hint="Filter PlannedEvent and ScheduleResource rows before binding SchedulerTimeline. Built-in toolbar filtering is planned for a future release."
            />
            {children.map(|c| c())}
        </div>
    }
}
