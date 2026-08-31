//! [`SchedulerTimelineImportExport`] — ICS import/export (not yet shipped).

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::DeferredFeatureNotice;

/// ICS import and export for timeline events is not yet available in Orbital.
///
/// # When to use
///
/// - Planning Gantt interchange with tools that export iCalendar (.ics)
/// - Documenting the intended catalog entry until the feature ships
///
/// # Usage
///
/// Map external ICS data to [`PlannedEvent`] with `resource_id` set for each bar until
/// built-in import/export ships on [`SchedulerTimeline`].
///
/// # Best Practices
///
/// ## Do's
///
/// - Validate resource ids when importing — timeline bars require matching lanes.
///
/// ## Don'ts
///
/// - Do not expect file controls on [`SchedulerTimeline`] today.
///
/// # Examples
///
/// ## Not yet available
/// <!-- preview -->
/// ```rust
/// use crate::SchedulerTimelineImportExport;
/// view! {
///     <SchedulerTimelineImportExport />
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-import-export",
    preview_label = "Scheduler Event Timeline Import Export",
    preview_icon = icondata::AiExportOutlined,
)]
#[component]
pub fn SchedulerTimelineImportExport(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-testid="scheduler-timeline-import-export-preview" class=move || class.get().unwrap_or_default()>
            <DeferredFeatureNotice
                sc_id="SC-26"
                feature_name="Timeline import/export"
                hint="Convert between your API and PlannedEvent at boundaries until ICS import/export ships on SchedulerTimeline."
            />
            {children.map(|c| c())}
        </div>
    }
}
