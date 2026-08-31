//! [`SchedulerCalendarImportExport`] — ICS import/export (not yet shipped).

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::DeferredFeatureNotice;

/// ICS import and export for calendar events is not yet available in Orbital.
///
/// # When to use
///
/// - Planning interoperability with calendar clients that speak iCalendar (.ics)
/// - Documenting the intended catalog entry until the feature ships
///
/// # Usage
///
/// Until ICS support ships, convert between your API format and [`PlannedEvent`] at boundaries.
/// Use [`OrbitalDateTime`] and optional `recurrence_rule` strings when round-tripping RRULE data manually.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep canonical events in your backend — treat any ICS bridge as an import/export adapter.
///
/// ## Don'ts
///
/// - Do not expect file upload or download controls on [`SchedulerCalendar`] today.
///
/// # Examples
///
/// ## Not yet available
/// <!-- preview -->
/// ```rust
/// use crate::SchedulerCalendarImportExport;
/// view! {
///     <SchedulerCalendarImportExport />
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-import-export",
    preview_label = "Scheduler Event Calendar Import Export",
    preview_icon = icondata::AiExportOutlined,
)]
#[component]
pub fn SchedulerCalendarImportExport(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-testid="scheduler-calendar-import-export-preview" class=move || class.get().unwrap_or_default()>
            <DeferredFeatureNotice
                sc_id="SC-13"
                feature_name="Calendar import/export"
                hint="Convert between your API and PlannedEvent at boundaries until ICS import/export ships on SchedulerCalendar."
            />
            {children.map(|c| c())}
        </div>
    }
}
