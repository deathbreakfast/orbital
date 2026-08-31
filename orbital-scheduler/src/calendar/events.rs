//! Event chip rendering for timed calendar grids (SC-05).

use chrono::NaiveDate;
use leptos::prelude::*;
use orbital_base_components::ToUnixSeconds;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use std::sync::Arc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{
    Badge, BadgeAppearance, Flex, FlexAlign, FlexGap, ThemeDensityStepper,
};

use crate::calendar::drag::{begin_event_drag, drag_mode_from_pointer};
use crate::calendar::engine::EventLayoutRect;
#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date, preview::fixtures::sample_planned_events,
    EventContentView, ScheduleResource, SchedulerCalendar, SchedulerEventContent,
    SchedulerFeatures, SchedulerView,
};
use crate::{
    event_is_editable, render_event_content, scheduler_drag_active, scheduler_drag_session,
    use_scheduler_interaction, EventRenderSurface,
};
use crate::{EventDialogMode, EventDialogRequest, PlannedEvent};

/// Positioned event block inside a timed day column.
#[component]
pub fn SchedulerEventChip(
    event: PlannedEvent,
    layout: EventLayoutRect,
    day: NaiveDate,
    resource_id: Option<String>,
) -> impl IntoView {
    let ctx = use_scheduler_interaction();
    let test_id = format!("scheduler-event-{}", event.id);
    let start_unix = event.start.to_unix_seconds().to_string();
    let end_unix = event.end.to_unix_seconds().to_string();
    let event_id = event.id.clone();
    let event_color = event.color.clone();
    let event_id_for_drag = event_id.clone();
    let is_draggable_override = event.is_draggable;
    let is_resizable_override = event.is_resizable;
    let resource_id_for_render = resource_id.clone();

    let class_list = {
        let ctx = ctx.clone();
        let event_id = event_id_for_drag.clone();
        move || {
            let ctx = ctx.clone();
            let mut parts = vec!["orb-scheduler-event".to_string()];
            let can_drag =
                ctx.are_events_draggable.get_untracked() && is_draggable_override.unwrap_or(true);
            if can_drag {
                parts.push("orb-scheduler-event--draggable".to_string());
            }
            let dragging = scheduler_drag_active(&ctx)
                && scheduler_drag_session(&ctx)
                    .map(|s| s.event_id == event_id)
                    .unwrap_or(false);
            if dragging {
                parts.push("orb-scheduler-event--dragging".to_string());
            }
            parts.join(" ")
        }
    };

    let style = move || {
        let mut parts = vec![
            format!("top: {}%", layout.top_pct),
            format!("height: {}%", layout.height_pct),
        ];
        if let Some(color) = &event_color {
            parts.push(format!(
                "background: color-mix(in srgb, {color} 18%, var(--orb-color-surface-canvas, #fff)); border-inline-start: 3px solid {color};"
            ));
        }
        parts.join("; ")
    };

    let on_pointer_down = {
        let event = event.clone();
        let resource_id = resource_id.clone();
        let ctx = ctx.clone();
        move |ev: leptos::ev::PointerEvent| {
            ev.stop_propagation();
            #[cfg(feature = "hydrate")]
            let (chip_height, chip_width, offset_y) = {
                use wasm_bindgen::JsCast;
                if let Some(el) = ev.current_target() {
                    if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                        let rect = html.get_bounding_client_rect();
                        let offset_y = ev.client_y() as f64 - rect.top();
                        (rect.height(), rect.width(), offset_y)
                    } else {
                        (40.0, 120.0, 0.0)
                    }
                } else {
                    (40.0, 120.0, 0.0)
                }
            };
            #[cfg(not(feature = "hydrate"))]
            let (chip_height, chip_width, offset_y) = (40.0_f64, 120.0_f64, 0.0_f64);

            let resizable =
                ctx.are_events_resizable.get_untracked() && is_resizable_override.unwrap_or(true);
            let draggable =
                ctx.are_events_draggable.get_untracked() && is_draggable_override.unwrap_or(true);

            let mode = drag_mode_from_pointer(offset_y, chip_height, resizable, draggable);
            let Some(mode) = mode else {
                if event_is_editable(&event) {
                    ctx.open_dialog.run(EventDialogRequest {
                        mode: EventDialogMode::Edit {
                            event_id: event.id.clone(),
                        },
                    });
                }
                return;
            };

            begin_event_drag(
                ctx.clone(),
                ctx.drag_listeners,
                event.clone(),
                day,
                resource_id.clone(),
                mode,
                ev.client_x() as f32,
                ev.client_y() as f32,
                chip_width as f32,
                chip_height as f32,
            );
        }
    };

    view! {
        <div
            class=class_list
            data-testid=test_id
            data-event-id=event_id.clone()
            data-start-unix=start_unix
            data-end-unix=end_unix
            style=style
            on:pointerdown=on_pointer_down
        >
            {{
                let ctx = ctx.clone();
                let _event = event.clone();
                move || {
                let show_handles = ctx.are_events_resizable.get_untracked()
                    && is_resizable_override.unwrap_or(true);
                if show_handles {
                    view! {
                        <div class="orb-scheduler-event__resize-handle orb-scheduler-event__resize-handle--top" />
                        <div class="orb-scheduler-event__resize-handle orb-scheduler-event__resize-handle--bottom" />
                    }
                    .into_any()
                } else {
                    ().into_any()
                }
            }
            }}
            {move || {
                let ctx = ctx.clone();
                ctx.renderers.with_value(|renderers| {
                    render_event_content(
                        renderers,
                        &event,
                        resource_id_for_render.as_deref(),
                        EventRenderSurface::TimedGrid,
                    )
                })
            }}
        </div>
    }
}

/// Render timed and all-day events on [`SchedulerCalendar`] week, month, day, and agenda views.
///
/// Event chips are positioned from each [`PlannedEvent`] `start` and `end` instants in the active view grid.
///
/// # When to use
///
/// - Showing meetings, shifts, or bookings on a time grid
/// - Coloring events by team, status, or category
///
/// # Usage
///
/// 1. Bind `events: RwSignal<Vec<PlannedEvent>>` on [`SchedulerCalendar`].
/// 2. Set `view: RwSignal<SchedulerView>` to pick week, month, day, or agenda layout.
/// 3. Optionally assign `color` or custom classes on each event for visual grouping.
///
/// # Best Practices
///
/// ## Do's
///
/// - Store `start` / `end` as [`OrbitalDateTime`] — grid placement uses typed instants.
///
/// ## Don'ts
///
/// - Do not mix all-day and timed semantics without setting the event's all-day flag when your model supports it.
///
/// # Examples
///
/// ## Colored week events
/// Week view with sample planned events and no resource columns.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-events-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <div data-testid="scheduler-calendar-week-preview">
///                 <SchedulerCalendar
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     view=view
///                     features=SchedulerFeatures::empty()
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-events",
    preview_label = "Scheduler Calendar Events",
    preview_icon = icondata::AiScheduleOutlined,
)]
#[component]
pub fn SchedulerCalendarEvents(
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
        let events = RwSignal::new(sample_planned_events());
        let resources = RwSignal::new(Vec::<ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        let event_content: EventContentView = Arc::new(|ctx| {
            Some(view! {
                <Badge appearance=BadgeAppearance::Outline attr:data-testid="scheduler-event-content-badge">
                    {ctx.event.title.clone()}
                </Badge>
            }
            .into_any())
        });

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-events-preview">
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
                        >
                            <SchedulerEventContent slot render=event_content />
                        </SchedulerCalendar>
                    </div>
                </Flex>
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
