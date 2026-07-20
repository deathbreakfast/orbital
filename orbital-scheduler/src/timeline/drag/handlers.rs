//! Pointer handlers for horizontal timeline event drag (SC-20).
//!
//! Some helpers remain for upcoming lane-hit wiring.
#![allow(dead_code)]

use leptos::{ev, leptos_dom::helpers::window_event_listener, prelude::*};

use orbital_base_components::{DatetimeTimezone, TryFromUnixSeconds};

use crate::shared::editing::{EventDialogMode, EventDialogRequest};
use crate::shared::interaction::{
    event_is_editable, patch_event, scheduler_drag_ghost, scheduler_drag_handlers_enabled,
    scheduler_drag_session, scheduler_end_drag_session, scheduler_start_drag,
    scheduler_update_drag_ghost, scheduler_update_drag_session, SchedulerInteractionContext,
};
#[cfg(feature = "hydrate")]
use crate::timeline::engine::ratio_from_lane_x;
use crate::timeline::engine::{
    move_event_horizontally, resize_timeline_event_end, resize_timeline_event_start,
    TimelineVisibleRange,
};
use crate::PlannedEvent;
use crate::{event_drag_ghost_at_pointer, move_event_drag_ghost, DragMode, EventDragSession};

const RESIZE_HANDLE_PX: f64 = 6.0;

#[derive(Clone)]
struct TimelineLaneHit {
    resource_id: String,
    range_start_unix: i64,
    range_end_unix: i64,
    rect_left: f64,
    rect_width: f64,
}

#[cfg(feature = "hydrate")]
fn lane_at_point(x: f32, y: f32) -> Option<TimelineLaneHit> {
    let document = window().document()?;
    let el = document.element_from_point(x, y)?;
    let mut current = Some(el);
    let mut resource_id = None;
    let mut range_start_unix = None;
    let mut range_end_unix = None;
    while let Some(node) = current {
        if resource_id.is_none() {
            if let Some(id) = node.get_attribute("data-resource-id") {
                if !id.is_empty() {
                    resource_id = Some(id);
                }
            }
        }
        if range_start_unix.is_none() {
            if let Some(v) = node.get_attribute("data-range-start-unix") {
                range_start_unix = v.parse().ok();
            }
        }
        if range_end_unix.is_none() {
            if let Some(v) = node.get_attribute("data-range-end-unix") {
                range_end_unix = v.parse().ok();
            }
        }
        current = node.parent_element();
    }
    let resource_id = resource_id?;
    let range_start_unix = range_start_unix?;
    let range_end_unix = range_end_unix?;

    let mut lane_el = document.element_from_point(x, y)?;
    let mut current = Some(lane_el.clone());
    while let Some(node) = current {
        if node.has_attribute("data-timeline-lane") {
            lane_el = node;
            break;
        }
        current = node.parent_element();
    }
    let rect = lane_el.get_bounding_client_rect();
    Some(TimelineLaneHit {
        resource_id,
        range_start_unix,
        range_end_unix,
        rect_left: rect.left(),
        rect_width: rect.width(),
    })
}

#[cfg(not(feature = "hydrate"))]
fn lane_at_point(_x: f32, _y: f32) -> Option<TimelineLaneHit> {
    None
}

fn range_from_hit(
    hit: &TimelineLaneHit,
    display_tz: DatetimeTimezone,
) -> Option<TimelineVisibleRange> {
    let range_start = orbital_base_components::OrbitalDateTime::try_from_unix_seconds(
        hit.range_start_unix,
        display_tz,
    )
    .ok()?;
    let range_end = orbital_base_components::OrbitalDateTime::try_from_unix_seconds(
        hit.range_end_unix,
        display_tz,
    )
    .ok()?;
    Some(TimelineVisibleRange {
        query: crate::DateTimeRange::new(range_start, range_end),
        range_start,
        range_end,
        columns: Vec::new(),
    })
}

fn update_timeline_draft(
    session: &mut EventDragSession,
    range: &TimelineVisibleRange,
    ratio: f64,
    display_tz: DatetimeTimezone,
    resource_id: Option<String>,
) {
    match session.mode {
        DragMode::Move => {
            if let Some((start, end)) =
                move_event_horizontally(&session.draft, range, ratio, display_tz)
            {
                session.draft.start = start;
                session.draft.end = end;
            }
            if let Some(rid) = resource_id {
                session.draft.resource_id = Some(rid);
            }
        }
        DragMode::ResizeEnd => {
            if let Some(end) = resize_timeline_event_end(&session.draft, range, ratio, display_tz) {
                session.draft.end = end;
            }
        }
        DragMode::ResizeStart => {
            if let Some(start) =
                resize_timeline_event_start(&session.draft, range, ratio, display_tz)
            {
                session.draft.start = start;
            }
        }
    }
}

/// Begin horizontal drag or resize on a timeline event chip.
pub fn begin_timeline_event_drag(
    ctx: SchedulerInteractionContext,
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>>,
    event: PlannedEvent,
    resource_id: Option<String>,
    mode: DragMode,
    client_x: f32,
    client_y: f32,
    chip_width: f32,
    chip_height: f32,
) {
    let ghost = event_drag_ghost_at_pointer(
        event.title.clone(),
        chip_width,
        chip_height,
        client_x,
        client_y,
    );
    let day = event
        .start
        .wall_date()
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let session = EventDragSession {
        event_id: event.id.clone(),
        mode,
        origin_day: day,
        origin_resource_id: resource_id,
        origin_start: event.start,
        origin_end: event.end,
        draft: event,
        moved: false,
    };
    scheduler_start_drag(&ctx, session, ghost);

    listeners.update_value(|handles| {
        for handle in handles.drain(..) {
            handle.remove();
        }
    });

    let start_x = client_x as f64;
    let start_y = client_y as f64;
    const DRAG_THRESHOLD_PX: f64 = 4.0;

    let on_pointer_move = window_event_listener(ev::pointermove, {
        let ctx = ctx.clone();
        move |ev: ev::PointerEvent| {
            if !scheduler_drag_handlers_enabled(&ctx) {
                return;
            }

            let dx = (ev.client_x() as f64 - start_x).abs();
            let dy = (ev.client_y() as f64 - start_y).abs();
            if dx > DRAG_THRESHOLD_PX || dy > DRAG_THRESHOLD_PX {
                if let Some(mut session) = scheduler_drag_session(&ctx) {
                    session.moved = true;
                    scheduler_update_drag_session(&ctx, session);
                }
            }

            #[cfg(feature = "hydrate")]
            {
                let x = ev.client_x() as f32;
                let y = ev.client_y() as f32;
                if let Some(hit) = lane_at_point(x, y) {
                    let display_tz = ctx.display_timezone.get_untracked();
                    if let Some(range) = range_from_hit(&hit, display_tz) {
                        let ratio =
                            ratio_from_lane_x(ev.client_x() as f64, hit.rect_left, hit.rect_width);
                        if let Some(mut session) = scheduler_drag_session(&ctx) {
                            update_timeline_draft(
                                &mut session,
                                &range,
                                ratio,
                                display_tz,
                                Some(hit.resource_id.clone()),
                            );
                            scheduler_update_drag_session(&ctx, session);
                        }
                    }
                }
            }

            if let Some(mut ghost) = scheduler_drag_ghost(&ctx) {
                move_event_drag_ghost(&mut ghost, ev.client_x() as f32, ev.client_y() as f32);
                scheduler_update_drag_ghost(&ctx, ghost);
            }
        }
    });

    let on_pointer_up = window_event_listener(ev::pointerup, {
        let ctx = ctx.clone();
        move |_ev: ev::PointerEvent| {
            if !scheduler_drag_handlers_enabled(&ctx) {
                return;
            }

            let session = scheduler_drag_session(&ctx);
            let Some(session) = session else {
                return;
            };

            listeners.update_value(|handles| {
                for handle in handles.drain(..) {
                    handle.remove();
                }
            });

            if session.moved {
                let draft = session.draft.clone();
                if scheduler_drag_handlers_enabled(&ctx) {
                    patch_event(
                        ctx.events,
                        ctx.scheduler_events,
                        ctx.lazy_context.as_ref(),
                        &session.event_id,
                        |event| {
                            event.start = draft.start;
                            event.end = draft.end;
                            event.resource_id = draft.resource_id.clone();
                        },
                    );
                }
            } else if event_is_editable(&session.draft) && scheduler_drag_handlers_enabled(&ctx) {
                ctx.open_dialog.run(EventDialogRequest {
                    mode: EventDialogMode::Edit {
                        event_id: session.event_id.clone(),
                    },
                });
            }

            scheduler_end_drag_session(&ctx);
        }
    });

    listeners.update_value(|handles| {
        handles.push(on_pointer_move);
        handles.push(on_pointer_up);
    });
}

/// Hit-test pointer X within a horizontal chip to choose drag mode.
pub fn timeline_drag_mode_from_pointer(
    offset_x: f64,
    chip_width: f64,
    resizable: bool,
    draggable: bool,
) -> Option<DragMode> {
    if resizable && offset_x <= RESIZE_HANDLE_PX {
        return Some(DragMode::ResizeStart);
    }
    if resizable && offset_x >= chip_width - RESIZE_HANDLE_PX {
        return Some(DragMode::ResizeEnd);
    }
    if draggable {
        return Some(DragMode::Move);
    }
    None
}
