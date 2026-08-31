//! Doc-section preview for calendar navigation (SC-07).

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexGap, ThemeDensityStepper,
};
use orbital_macros::component_doc;

use crate::calendar::navigation::{
    format_visible_range_label, preview_anchor_date, SchedulerCalendarHandle,
};
use crate::{
    PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerEvents, SchedulerFeatures,
    SchedulerView,
};

/// Move the visible calendar range with toolbar controls or an imperative handle.
///
/// # When to use
///
/// - Wiring external paging or "today" buttons outside the calendar toolbar
/// - Syncing `visible_date` with routing or parent tabs
///
/// # Usage
///
/// 1. Bind `visible_date: RwSignal<OrbitalDateTime>` and `view: RwSignal<SchedulerView>`.
/// 2. Capture [`SchedulerCalendarHandle`] via `scheduler_events.on_calendar_handle` for `go_to_today`, `go_to_previous`, and `go_to_next`.
/// 3. Display the formatted range label from `visible_date` + `view` in your own chrome if needed.
///
/// # Best Practices
///
/// ## Do's
///
/// - Anchor `visible_date` at start-of-day in the user's display timezone.
///
/// ## Don'ts
///
/// - Do not call handle methods before `on_calendar_handle` fires — store the handle in a signal first.
///
/// # Examples
///
/// ## Go to today
/// Toolbar prev/next/today and external handle buttons update the visible range title.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use orbital_core_components::{Button, ButtonAppearance, Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerCalendarHandle, SchedulerEvents,
///     SchedulerFeatures, SchedulerView,
/// };
/// let visible_date = RwSignal::new(
///     OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Local)
///         .expect("valid anchor")
///         .start_of_day(),
/// );
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let handle = RwSignal::new(None::<SchedulerCalendarHandle>);
/// view! {
///     <div data-testid="scheduler-calendar-navigation-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Flex gap=FlexGap::Small align=FlexAlign::Center>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_previous.run(()); }
///                 }>"External Previous"</Button>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_today.run(()); }
///                 }>"External Today"</Button>
///                 <Button appearance=ButtonAppearance::Subtle on:click=move |_| {
///                     if let Some(h) = handle.get() { h.go_to_next.run(()); }
///                 }>"External Next"</Button>
///             </Flex>
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::empty()
///                 scheduler_events=SchedulerEvents {
///                     on_calendar_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                     ..Default::default()
///                 }
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-navigation",
    preview_label = "Scheduler Calendar Navigation",
    preview_icon = icondata::AiCompassOutlined,
)]
#[component]
pub fn SchedulerCalendarNavigation(
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
    let handle = RwSignal::new(None::<SchedulerCalendarHandle>);

    #[cfg(feature = "preview")]
    {
        events.set(crate::preview::fixtures::sample_planned_events());
        resources.set(crate::preview::fixtures::sample_schedule_resources());
    }

    view! {
        <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-navigation-preview">
            <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                <ThemeDensityStepper />
                <Flex gap=FlexGap::Small align=FlexAlign::Center>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on:click=move |_| {
                            if let Some(h) = handle.get() {
                                h.go_to_previous.run(());
                            }
                        }
                    >
                        "External Previous"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on:click=move |_| {
                            if let Some(h) = handle.get() {
                                h.go_to_today.run(());
                            }
                        }
                    >
                        "External Today"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on:click=move |_| {
                            if let Some(h) = handle.get() {
                                h.go_to_next.run(());
                            }
                        }
                    >
                        "External Next"
                    </Button>
                </Flex>
                <SchedulerCalendar
                    events=events
                    resources=resources
                    visible_date=visible_date
                    display_timezone=display_timezone
                    view=view
                    features=SchedulerFeatures::empty()
                    scheduler_events=SchedulerEvents {
                        on_calendar_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
                        ..Default::default()
                    }
                />
                <div data-testid="scheduler-calendar-navigation-visible-date">
                    {move || format_visible_range_label(visible_date.get(), view.get())}
                </div>
            </Flex>
            {children.map(|c| c())}
        </div>
    }
}
