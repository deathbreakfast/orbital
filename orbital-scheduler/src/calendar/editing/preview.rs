//! Event editing preview (SC-09).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Button, Flex, FlexAlign, FlexGap, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date, open_create_default, DatetimeLocale, PlannedEvent,
    ScheduleResource, SchedulerCalendar, SchedulerEditingTools, SchedulerFeatures, SchedulerView,
};

#[cfg(feature = "preview")]
#[component]
fn SchedulerEditingOpenButton(
    visible_date: RwSignal<orbital_base_components::OrbitalDateTime>,
) -> impl IntoView {
    let Some(ctx) = use_context::<crate::SchedulerInteractionContext>() else {
        return ().into_any();
    };
    let on_click = Callback::new(move |_ev: leptos::ev::MouseEvent| {
        open_create_default(&ctx, visible_date.get_untracked());
    });
    view! {
        <Button on_click=on_click attr:data-testid="scheduler-event-dialog-open">
            "New event"
        </Button>
    }
    .into_any()
}

/// Create and edit calendar events through the shared dialog with [`DateTimePicker`].
///
/// # When to use
///
/// - Adding events from empty timed slots or a toolbar button
/// - Editing title, start/end, and resource assignment on existing chips
///
/// # Usage
///
/// 1. Wrap [`SchedulerCalendar`] in [`DatetimeLocale`] for picker timezone and format.
/// 2. Set `event_creation=Signal::from(true)` to enable click-to-create on empty slots.
/// 3. Provide [`SchedulerEditingTools`] slot content that opens the dialog via [`use_scheduler_interaction`].
/// 4. Bind `events: RwSignal<Vec<PlannedEvent>>` for the saved collection.
///
/// # Best Practices
///
/// ## Do's
///
/// - Reuse [`DateTimePicker`](orbital_date_pickers::DateTimePicker) from Calendar & Time — the dialog composes it internally.
///
/// ## Don'ts
///
/// - Do not mount editing without `DatetimeLocale` — picker segments need locale context.
///
/// # Examples
///
/// ## Create on click
/// Week view with click-to-create enabled and a New event toolbar button.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Button, Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     open_create_default, PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerEditingTools,
///     SchedulerFeatures, SchedulerView, DatetimeLocale, use_scheduler_interaction,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// #[component]
/// fn OpenButton(visible_date: RwSignal<orbital_base_components::OrbitalDateTime>) -> impl IntoView {
///     view! {
///         <Button
///             attr:data-testid="scheduler-event-dialog-open"
///             on:click=move |_| {
///                 open_create_default(&use_scheduler_interaction(), visible_date.get_untracked());
///             }
///         >
///             "New event"
///         </Button>
///     }
/// }
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-editing-preview">
///         <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
///             <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///                 <ThemeDensityStepper />
///                 <SchedulerCalendar
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     view=view
///                     features=SchedulerFeatures::empty()
///                     event_creation=Signal::from(true)
///                 >
///                 <SchedulerEditingTools slot>
///                     <Button
///                         attr:data-testid="scheduler-event-dialog-open"
///                         on:click=move |_| {
///                             open_create_default(&use_scheduler_interaction(), visible_date.get_untracked());
///                         }
///                     >
///                         "New event"
///                     </Button>
///                 </SchedulerEditingTools>
///             </SchedulerCalendar>
///             </Flex>
///         </DatetimeLocale>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-editing",
    preview_label = "Scheduler Calendar Editing",
    preview_icon = icondata::AiEditOutlined,
)]
#[component]
pub fn SchedulerCalendarEditing(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(Vec::<PlannedEvent>::new());
        let resources = RwSignal::new(Vec::<ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-editing-preview">
                <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
                    <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                        <ThemeDensityStepper />
                        <div data-testid="scheduler-calendar-week-preview">
                            <SchedulerCalendar
                                events=events
                                resources=resources
                                visible_date=visible_date
                                display_timezone=display_timezone
                                view=view
                                features=SchedulerFeatures::empty()
                                event_creation=Signal::from(true)
                            >
                                <SchedulerEditingTools slot>
                                    <SchedulerEditingOpenButton visible_date=visible_date />
                                </SchedulerEditingTools>
                            </SchedulerCalendar>
                        </div>
                    </Flex>
                </DatetimeLocale>
                {children.map(|c| c())}
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = (&class, &children);
        ().into_any()
    }
}
