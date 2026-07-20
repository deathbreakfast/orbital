//! Pointer handlers for calendar event drag and resize.
//!
//! Some helpers remain for upcoming drag-hit wiring.
#![allow(dead_code)]

use chrono::NaiveDate;
use leptos::{ev, leptos_dom::helpers::window_event_listener, prelude::*};

use orbital_base_components::DatetimeTimezone;

use super::state::{
    event_drag_ghost_at_pointer, move_event_drag_ghost, DragMode, EventDragSession,
};
use crate::shared::editing::{EventDialogMode, EventDialogRequest};
use crate::shared::interaction::{
    event_is_editable, patch_event, scheduler_drag_ghost, scheduler_drag_handlers_enabled,
    scheduler_drag_session, scheduler_end_drag_session, scheduler_start_drag,
    scheduler_update_drag_ghost, scheduler_update_drag_session, SchedulerInteractionContext,
};
use crate::PlannedEvent;
use crate::{minutes_from_column_y, move_event_to, resize_event_end, resize_event_start};

const RESIZE_HANDLE_PX: f64 = 6.0;
const DRAG_THRESHOLD_PX: f64 = 4.0;

/// Remove any window listeners registered for an in-progress drag session.
pub fn clear_drag_listeners(
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>>,
) {
    listeners.update_value(|handles| {
        for handle in handles.drain(..) {
            handle.remove();
        }
    });
}

#[cfg(feature = "hydrate")]
fn column_at_point(x: f32, y: f32) -> Option<(NaiveDate, Option<String>)> {
    let document = window().document()?;
    let el = document.element_from_point(x, y)?;
    let mut current = Some(el);
    while let Some(node) = current {
        if let Some(day_str) = node.get_attribute("data-day") {
            let day = NaiveDate::parse_from_str(&day_str, "%Y-%m-%d").ok()?;
            let resource_id = node
                .get_attribute("data-resource-id")
                .filter(|s| !s.is_empty());
            return Some((day, resource_id));
        }
        current = node.parent_element();
    }
    None
}

#[cfg(not(feature = "hydrate"))]
fn column_at_point(_x: f32, _y: f32) -> Option<(NaiveDate, Option<String>)> {
    None
}

#[cfg(feature = "hydrate")]
fn column_rect_at_point(x: f32, y: f32) -> Option<web_sys::DomRect> {
    let document = window().document()?;
    let el = document.element_from_point(x, y)?;
    let mut current = Some(el);
    while let Some(node) = current {
        if node.has_attribute("data-day") {
            return node.get_bounding_client_rect().into();
        }
        current = node.parent_element();
    }
    None
}

fn minutes_in_column(client_y: f64, rect_top: f64, rect_height: f64) -> f64 {
    let ratio = if rect_height > 0.0 {
        (client_y - rect_top) / rect_height
    } else {
        0.0
    };
    minutes_from_column_y(ratio)
}

fn update_draft_from_pointer(
    session: &mut EventDragSession,
    day: NaiveDate,
    minutes: f64,
    display_tz: DatetimeTimezone,
    resource_id: Option<String>,
) {
    match session.mode {
        DragMode::Move => {
            if let Some((start, end)) = move_event_to(&session.draft, day, minutes, display_tz) {
                session.draft.start = start;
                session.draft.end = end;
            }
            if let Some(rid) = resource_id {
                if rid != "__unassigned__" {
                    session.draft.resource_id = Some(rid);
                } else {
                    session.draft.resource_id = None;
                }
            }
        }
        DragMode::ResizeEnd => {
            if let Some(end) = resize_event_end(&session.draft, day, minutes, display_tz) {
                session.draft.end = end;
            }
        }
        DragMode::ResizeStart => {
            if let Some(start) = resize_event_start(&session.draft, day, minutes, display_tz) {
                session.draft.start = start;
            }
        }
    }
}

/// Attach window listeners for an in-progress drag session.
pub fn attach_drag_listeners(
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>>,
    ctx: SchedulerInteractionContext,
    start_x: f32,
    start_y: f32,
) {
    listeners.update_value(|handles| {
        for handle in handles.drain(..) {
            handle.remove();
        }
    });

    let start_x = start_x as f64;
    let start_y = start_y as f64;

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
                if let (Some((day, resource_id)), Some(rect)) =
                    (column_at_point(x, y), column_rect_at_point(x, y))
                {
                    let minutes =
                        minutes_in_column(ev.client_y() as f64, rect.top(), rect.height());
                    let display_tz = ctx.display_timezone.get_untracked();
                    if let Some(mut session) = scheduler_drag_session(&ctx) {
                        update_draft_from_pointer(
                            &mut session,
                            day,
                            minutes,
                            display_tz,
                            resource_id,
                        );
                        scheduler_update_drag_session(&ctx, session);
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

/// Begin dragging or resizing an event from a pointer down on its chip.
pub fn begin_event_drag(
    ctx: SchedulerInteractionContext,
    listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>>,
    event: PlannedEvent,
    day: NaiveDate,
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
    attach_drag_listeners(listeners, ctx, client_x, client_y);
}

/// Hit-test pointer Y within a chip to choose drag mode.
pub fn drag_mode_from_pointer(
    offset_y: f64,
    chip_height: f64,
    resizable: bool,
    draggable: bool,
) -> Option<DragMode> {
    if resizable && offset_y <= RESIZE_HANDLE_PX {
        return Some(DragMode::ResizeStart);
    }
    if resizable && offset_y >= chip_height - RESIZE_HANDLE_PX {
        return Some(DragMode::ResizeEnd);
    }
    if draggable {
        return Some(DragMode::Move);
    }
    None
}
