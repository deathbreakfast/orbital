//! [`SchedulerCalendarFiltering`] — calendar filtering (not yet shipped).

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::DeferredFeatureNotice;

/// Built-in calendar event filtering is not yet available in Orbital.
///
/// # When to use
///
/// - Planning a future filter-by-resource or filter-by-category UX
/// - Documenting the intended catalog entry until the feature ships
///
/// # Usage
///
/// Until filtering ships, filter [`PlannedEvent`] rows in your store before binding `events`
/// on [`SchedulerCalendar`]. Combine with [`SchedulerDataSource`] when lazy loading so queries
/// stay scoped to the visible range.
///
/// # Best Practices
///
/// ## Do's
///
/// - Apply filters in the same layer that owns your event fetch — keep the calendar props simple.
///
/// ## Don'ts
///
/// - Do not expect toolbar filter controls on [`SchedulerCalendar`] today — they are not implemented.
///
/// # Examples
///
/// ## Not yet available
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::DeferredFeatureNotice;
/// view! {
///     <div data-testid="scheduler-calendar-filtering-preview">
///         <DeferredFeatureNotice
///             sc_id="SC-12"
///             feature_name="Calendar filtering"
///             hint="Filter PlannedEvent rows in your data layer before binding events on SchedulerCalendar. Built-in toolbar filtering is planned for a future release."
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-filtering",
    preview_label = "Scheduler Event Calendar Filtering",
    preview_icon = icondata::AiFilterOutlined,
)]
#[component]
pub fn SchedulerCalendarFiltering(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-testid="scheduler-calendar-filtering-preview" class=move || class.get().unwrap_or_default()>
            <DeferredFeatureNotice
                sc_id="SC-12"
                feature_name="Calendar filtering"
                hint="Filter PlannedEvent rows in your data layer before binding events on SchedulerCalendar. Built-in toolbar filtering is planned for a future release."
            />
            {children.map(|c| c())}
        </div>
    }
}
