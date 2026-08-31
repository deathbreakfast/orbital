//! Doc-section preview for calendar views (SC-10).

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
use orbital_macros::component_doc;

use crate::calendar::navigation::preview_anchor_date;
use crate::{PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView};

/// Switch between day, week, month, and agenda layouts on [`SchedulerCalendar`].
///
/// # When to use
///
/// - Week grid for operational scheduling
/// - Month overview for capacity planning
/// - Agenda list for mobile or narrow layouts
///
/// # Usage
///
/// 1. Bind `view: RwSignal<SchedulerView>` on [`SchedulerCalendar`].
/// 2. Use the toolbar view select or set the signal from your own tabs.
/// 3. Keep `visible_date` at start-of-day when switching views for consistent titles.
///
/// # Best Practices
///
/// ## Do's
///
/// - Use `SchedulerView::Week` as the default for timed event editing flows.
///
/// ## Don'ts
///
/// - Do not assume month view shows the same detail as week — timed chips simplify at coarse zoom.
///
/// # Examples
///
/// ## View switcher
/// Use the toolbar select to switch between calendar views.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// let visible_date = RwSignal::new(
///     OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Local)
///         .expect("valid anchor")
///         .start_of_day(),
/// );
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-views-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::empty()
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-views",
    preview_label = "Scheduler Calendar Views",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn SchedulerCalendarViews(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let visible_date = RwSignal::new(preview_anchor_date());
    let view = RwSignal::new(SchedulerView::Week);
    let events = RwSignal::new(Vec::<PlannedEvent>::new());
    let resources = RwSignal::new(Vec::<ScheduleResource>::new());
    let display_timezone = RwSignal::new(DatetimeTimezone::Local);

    #[cfg(feature = "preview")]
    {
        events.set(crate::preview::fixtures::sample_planned_events());
        resources.set(crate::preview::fixtures::sample_schedule_resources());
    }

    view! {
        <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-views-preview">
            <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                <ThemeDensityStepper />
                <SchedulerCalendar
                    events=events
                    resources=resources
                    visible_date=visible_date
                    display_timezone=display_timezone
                    view=view
                    features=SchedulerFeatures::empty()
                />
            </Flex>
            {children.map(|c| c())}
        </div>
    }
}
