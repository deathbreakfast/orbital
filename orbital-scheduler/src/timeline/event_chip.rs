//! Horizontal timeline event bar rendering (SC-17, SC-20), point-event
//! diamonds, tone colours and progress fill.

use leptos::prelude::*;
use orbital_base_components::ToUnixSeconds;

use crate::shared::interaction::{
    event_is_draggable, event_is_editable, event_is_resizable, scheduler_drag_active,
    scheduler_drag_session, use_scheduler_interaction,
};
use crate::timeline::drag::{begin_timeline_event_drag, timeline_drag_mode_from_pointer};
use crate::timeline::engine::{TimelineEventLayoutRect, TimelineEventPlacement};
use crate::{
    render_event_content, use_scheduler_chrome, EventDialogMode, EventDialogRequest,
    EventRenderSurface, PlannedEvent,
};

/// Inline style for a timeline bar: position, then `color` if set, else `tone`.
pub fn timeline_chip_style(event: &PlannedEvent, layout: TimelineEventLayoutRect) -> String {
    let mut parts = vec![
        format!("left: {}%", layout.left_pct),
        format!("width: {}%", layout.width_pct),
    ];
    if let Some(color) = &event.color {
        parts.push(format!(
            "background: color-mix(in srgb, {color} 18%, var(--orb-color-surface-canvas, #fff)); border-inline-start: 3px solid {color};"
        ));
    } else if let Some(tone) = event.tone {
        let tokens = tone.tokens();
        parts.push(format!(
            "background: {}; border-inline-start: 3px solid {}; color: {};",
            tokens.background, tokens.border, tokens.foreground
        ));
    }
    parts.join("; ")
}

/// Colour for the progress fill and the point diamond: `color` if set, else the
/// tone's border token, else the default event accent.
pub fn timeline_chip_strong_color(event: &PlannedEvent) -> String {
    match (&event.color, event.tone) {
        (Some(color), _) => color.clone(),
        (None, Some(tone)) => tone.tokens().border.to_string(),
        (None, None) => "var(--orb-color-accent-primary, #2563eb)".to_string(),
    }
}

/// Whether a pointer down on this chip may start a drag or resize.
///
/// Point events never drag or resize; they only open the edit dialog.
pub fn timeline_chip_can_drag(
    event: &PlannedEvent,
    global_drag: bool,
    global_resize: bool,
) -> bool {
    !event.is_point()
        && (event_is_draggable(event, global_drag) || event_is_resizable(event, global_resize))
}

/// Progress as a whole percentage for display, when the event is a bar with progress.
fn progress_percent(event: &PlannedEvent) -> Option<u32> {
    if event.is_point() {
        return None;
    }
    event
        .progress
        .map(|p| (p.clamp(0.0, 1.0) * 100.0).round() as u32)
}

/// Positioned event inside a timeline resource lane: a bar, or a diamond for a
/// point event.
#[component]
pub fn TimelineEventChip(
    event: PlannedEvent,
    placement: TimelineEventPlacement,
    resource_id: Option<String>,
) -> impl IntoView {
    match placement {
        TimelineEventPlacement::Bar(layout) => {
            view! { <TimelineEventBar event=event layout=layout resource_id=resource_id /> }
                .into_any()
        }
        TimelineEventPlacement::Point { left_pct } => {
            view! { <TimelinePointEvent event=event left_pct=left_pct resource_id=resource_id /> }
                .into_any()
        }
    }
}

/// Diamond for an event with `start == end`. Opens the edit dialog on click; never drags.
#[component]
fn TimelinePointEvent(
    event: PlannedEvent,
    left_pct: f64,
    resource_id: Option<String>,
) -> impl IntoView {
    let ctx = use_scheduler_interaction();
    let test_id = format!("scheduler-event-{}", event.id);
    let diamond_test_id = format!("scheduler-event-diamond-{}", event.id);
    let start_unix = event.start.to_unix_seconds().to_string();
    let end_unix = event.end.to_unix_seconds().to_string();
    let tone = event.tone.map(|t| t.wire_value());
    let diamond_style = format!(
        "background: {}; border-color: {};",
        event
            .color
            .clone()
            .or_else(|| event.tone.map(|t| t.tokens().background.to_string()))
            .unwrap_or_else(|| "var(--orb-color-surface-canvas, #fff)".to_string()),
        timeline_chip_strong_color(&event)
    );
    let title = event.title.clone();
    let event_id = event.id.clone();

    let on_pointer_down = {
        let event = event.clone();
        let ctx = ctx.clone();
        move |ev: leptos::ev::PointerEvent| {
            ev.stop_propagation();
            if event_is_editable(&event) {
                ctx.open_dialog.run(EventDialogRequest {
                    mode: EventDialogMode::Edit {
                        event_id: event.id.clone(),
                    },
                });
            }
        }
    };

    view! {
        <div
            class="orb-scheduler-event orb-scheduler-event--point"
            data-testid=test_id
            data-event-id=event_id
            data-start-unix=start_unix
            data-end-unix=end_unix
            data-tone=tone
            style=format!("left: {left_pct}%;")
            on:pointerdown=on_pointer_down
        >
            <span
                class="orb-scheduler-event__diamond"
                role="img"
                aria-label=title
                data-testid=diamond_test_id
                style=diamond_style
            />
            <span class="orb-scheduler-event__point-label" aria-hidden="true">
                {move || {
                    let ctx = ctx.clone();
                    ctx.renderers.with_value(|renderers| {
                        render_event_content(
                            renderers,
                            &event,
                            resource_id.as_deref(),
                            EventRenderSurface::TimelineBar,
                        )
                    })
                }}
            </span>
        </div>
    }
}

/// Positioned event bar inside a timeline resource lane.
#[component]
fn TimelineEventBar(
    event: PlannedEvent,
    layout: TimelineEventLayoutRect,
    resource_id: Option<String>,
) -> impl IntoView {
    let ctx = use_scheduler_interaction();
    let chrome = use_scheduler_chrome();
    let test_id = format!("scheduler-event-{}", event.id);
    let start_unix = event.start.to_unix_seconds().to_string();
    let end_unix = event.end.to_unix_seconds().to_string();
    let event_id = event.id.clone();
    let tone = event.tone.map(|t| t.wire_value());
    let chip_style = timeline_chip_style(&event, layout);
    let progress = progress_percent(&event);
    let progress_fill = progress.map(|pct| {
        let fill_style = format!(
            "width: {pct}%; background: {};",
            timeline_chip_strong_color(&event)
        );
        let fill_test_id = format!("scheduler-event-progress-{}", event.id);
        let done_word = move || {
            chrome
                .and_then(|c| c.locale_text.try_get().map(|l| l.progress_done.clone()))
                .unwrap_or_else(|| "done".to_string())
        };
        view! {
            <span
                class="orb-scheduler-event__progress"
                data-testid=fill_test_id
                data-progress=pct.to_string()
                style=fill_style
                aria-hidden="true"
            />
            <span class="orb-scheduler-event__sr">{move || format!(", {pct}% {}", done_word())}</span>
        }
    });
    let event_id_for_drag = event_id.clone();
    let is_draggable_override = event.is_draggable;
    let is_resizable_override = event.is_resizable;
    let resource_id_for_render = resource_id.clone();

    let class_list = {
        let ctx = ctx.clone();
        let event_id = event_id_for_drag.clone();
        let event_for_class = event.clone();
        move || {
            let ctx = ctx.clone();
            let mut parts = vec!["orb-scheduler-event".to_string()];
            let can_drag = timeline_chip_can_drag(
                &event_for_class,
                ctx.are_events_draggable.get_untracked(),
                ctx.are_events_resizable.get_untracked(),
            );
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

    let on_pointer_down = {
        let event_for_drag = event.clone();
        let event_for_edit = event.clone();
        let resource_id = resource_id.clone();
        let ctx = ctx.clone();
        move |ev: leptos::ev::PointerEvent| {
            ev.stop_propagation();
            #[cfg(feature = "hydrate")]
            let (chip_height, chip_width, offset_x) = {
                use wasm_bindgen::JsCast;
                if let Some(el) = ev.current_target() {
                    if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                        let rect = html.get_bounding_client_rect();
                        let offset_x = ev.client_x() as f64 - rect.left();
                        (rect.height(), rect.width(), offset_x)
                    } else {
                        (24.0, 80.0, 0.0)
                    }
                } else {
                    (24.0, 80.0, 0.0)
                }
            };
            #[cfg(not(feature = "hydrate"))]
            let (chip_height, chip_width, offset_x) = (24.0_f64, 80.0_f64, 0.0_f64);

            let draggable =
                event_is_draggable(&event_for_drag, ctx.are_events_draggable.get_untracked())
                    && is_draggable_override.unwrap_or(true);
            let resizable =
                event_is_resizable(&event_for_drag, ctx.are_events_resizable.get_untracked())
                    && is_resizable_override.unwrap_or(true);

            if let Some(mode) =
                timeline_drag_mode_from_pointer(offset_x, chip_width, resizable, draggable)
            {
                begin_timeline_event_drag(
                    ctx.clone(),
                    ctx.drag_listeners,
                    event_for_drag.clone(),
                    resource_id.clone(),
                    mode,
                    ev.client_x() as f32,
                    ev.client_y() as f32,
                    chip_width as f32,
                    chip_height as f32,
                );
            } else if event_is_editable(&event_for_edit) {
                ctx.open_dialog.run(EventDialogRequest {
                    mode: EventDialogMode::Edit {
                        event_id: event_for_edit.id.clone(),
                    },
                });
            }
        }
    };

    view! {
        <div
            class=class_list
            data-testid=test_id
            data-event-id=event_id
            data-start-unix=start_unix
            data-end-unix=end_unix
            data-tone=tone
            style=chip_style
            on:pointerdown=on_pointer_down
        >
            {progress_fill}
            <span class="orb-scheduler-event__content">
            {move || {
                let ctx = ctx.clone();
                ctx.renderers.with_value(|renderers| {
                    render_event_content(
                        renderers,
                        &event,
                        resource_id_for_render.as_deref(),
                        EventRenderSurface::TimelineBar,
                    )
                })
            }}
            </span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventTone;
    use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};

    fn at(secs: i64) -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(secs, DatetimeTimezone::Utc).expect("valid")
    }

    fn rect() -> TimelineEventLayoutRect {
        TimelineEventLayoutRect {
            left_pct: 10.0,
            width_pct: 20.0,
        }
    }

    #[test]
    fn tone_style_uses_danger_tokens_happy() {
        let event = PlannedEvent::new("e", "Late", at(0), at(3600)).with_tone(EventTone::Danger);
        let style = timeline_chip_style(&event, rect());
        assert!(style.contains("left: 10%"));
        assert!(style.contains("background: var(--orb-color-status-danger-bg)"));
        assert!(style.contains("solid var(--orb-color-status-danger-border)"));
        assert!(style.contains("color: var(--orb-color-status-danger-fg)"));
        assert_eq!(
            timeline_chip_strong_color(&event),
            "var(--orb-color-status-danger-border)"
        );
    }

    #[test]
    fn color_overrides_tone_happy() {
        let mut event =
            PlannedEvent::new("e", "Late", at(0), at(3600)).with_tone(EventTone::Danger);
        event.color = Some("#123456".into());
        let style = timeline_chip_style(&event, rect());
        assert!(style.contains("#123456"));
        assert!(!style.contains("status-danger"));
        assert_eq!(timeline_chip_strong_color(&event), "#123456");

        let plain = PlannedEvent::new("p", "Plain", at(0), at(3600));
        let style = timeline_chip_style(&plain, rect());
        assert!(!style.contains("background"));
    }

    #[test]
    fn point_event_not_draggable_sad() {
        let point = PlannedEvent::new("p", "Due", at(3600), at(3600));
        assert!(!timeline_chip_can_drag(&point, true, true));
        let mut forced = point.clone();
        forced.is_draggable = Some(true);
        forced.is_resizable = Some(true);
        assert!(!timeline_chip_can_drag(&forced, true, true));
        let bar = PlannedEvent::new("b", "Work", at(0), at(3600));
        assert!(timeline_chip_can_drag(&bar, true, false));
        assert!(!timeline_chip_can_drag(&bar, false, false));
    }

    #[test]
    fn point_event_ignores_progress_sad() {
        let point = PlannedEvent::new("p", "Due", at(3600), at(3600)).with_progress(0.5);
        assert_eq!(progress_percent(&point), None);
        let bar = PlannedEvent::new("b", "Work", at(0), at(3600)).with_progress(0.444);
        assert_eq!(progress_percent(&bar), Some(44));
    }
}
