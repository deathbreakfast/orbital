use std::time::Duration;

use leptos::ev;
use leptos::leptos_dom::helpers::{
    request_animation_frame, set_timeout_with_handle, TimeoutHandle,
};
use leptos::prelude::*;
use leptos::tachys::view::any_view::AnyView;
use wasm_bindgen::JsCast;

use super::drag::{
    announce_moved, announce_over, announce_picked, handle_key, BoardDrag, BoardKeyOutcome,
    BoardMove, ANNOUNCE_CANCELLED,
};
use super::BoardCard;
use super::BoardColumn;

/// How long a touch must rest on a card before it picks up, so a swipe still scrolls.
const LONG_PRESS: Duration = Duration::from_millis(200);
/// Finger travel (px) that turns a pending long-press into a scroll.
const TOUCH_SLOP: i32 = 10;

/// State shared by the board, its lanes and its cards.
#[derive(Clone, Copy)]
pub(super) struct BoardCtx {
    pub drag: RwSignal<Option<BoardDrag>>,
    pub announcement: RwSignal<String>,
    pub columns: Signal<Vec<BoardColumn>>,
    pub on_move: Option<Callback<(String, String)>>,
    pub testid: StoredValue<String>,
    pub root: NodeRef<leptos::html::Section>,
}

impl BoardCtx {
    pub fn movable(&self) -> bool {
        self.on_move.is_some()
    }

    pub fn column_ids(&self) -> Vec<String> {
        self.columns
            .with_untracked(|cols| cols.iter().map(|c| c.id.clone()).collect())
    }

    pub fn lane_index(&self, column_id: &str) -> Option<usize> {
        self.columns
            .with_untracked(|cols| cols.iter().position(|c| c.id == column_id))
    }

    fn lane_label(&self, index: usize) -> String {
        self.columns
            .with_untracked(|cols| cols.get(index).map(|c| c.label.clone()))
            .unwrap_or_default()
    }

    fn announce(&self, text: String) {
        self.announcement.set(text);
    }

    /// Point the active drag at lane `index`, if it isn't there already.
    pub fn hover(&self, index: usize) {
        let lanes = self.columns.with_untracked(Vec::len);
        let changed = self
            .drag
            .with_untracked(|d| d.as_ref().is_some_and(|d| d.over != index));
        if changed {
            self.drag.update(|d| {
                if let Some(d) = d.as_mut() {
                    d.hover(index, lanes);
                }
            });
        }
    }

    /// Drop `drag`: report the move, or announce a cancel when it lands on its own lane.
    pub fn finish(&self, drag: BoardDrag, refocus: bool) {
        match drag.drop_on(&self.column_ids()) {
            Some(mv) => self.commit(mv, &drag.card_label, refocus),
            None => self.announce(ANNOUNCE_CANCELLED.to_string()),
        }
    }

    /// Report `mv` through `on_move` and announce it. `refocus` moves focus to the card
    /// once the app re-renders it in its new lane.
    fn commit(&self, mv: BoardMove, card_label: &str, refocus: bool) {
        let lane = self
            .lane_index(&mv.to_column)
            .map(|i| self.lane_label(i))
            .unwrap_or_default();
        if let Some(on_move) = self.on_move {
            on_move.run((mv.card_id.clone(), mv.to_column));
        }
        self.announce(announce_moved(card_label, &lane));
        if refocus {
            self.focus_card(mv.card_id);
        }
    }

    fn focus_card(&self, card_id: String) {
        let root = self.root;
        request_animation_frame(move || {
            let Some(root) = root.get_untracked() else {
                return;
            };
            let Ok(nodes) = root.query_selector_all("[data-board-card]") else {
                return;
            };
            for i in 0..nodes.length() {
                let Some(el) = nodes
                    .item(i)
                    .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                else {
                    continue;
                };
                if el.get_attribute("data-board-card").as_deref() == Some(card_id.as_str()) {
                    let _ = el.focus();
                    return;
                }
            }
        });
    }
}

/// Focusable, draggable wrapper around one rendered card.
#[component]
pub(super) fn BoardCardItem(
    ctx: BoardCtx,
    card: BoardCard,
    render_card: Callback<BoardCard, AnyView>,
    item_class: &'static str,
    picked_class: &'static str,
    hint_id: String,
) -> impl IntoView {
    let movable = ctx.movable();
    let id = StoredValue::new(card.id.clone());
    let label = StoredValue::new(card.label.clone());
    let column_id = StoredValue::new(card.column_id.clone());
    let testid = format!("{}-card-{}", ctx.testid.get_value(), card.id);
    let card_attr = card.id.clone();

    let is_picked = move || {
        ctx.drag.with(|d| {
            d.as_ref()
                .is_some_and(|d| id.with_value(|id| &d.card_id == id))
        })
    };
    let picked_untracked = move || {
        ctx.drag.with_untracked(|d| {
            d.as_ref()
                .is_some_and(|d| id.with_value(|id| &d.card_id == id))
        })
    };
    let class = move || {
        if is_picked() {
            format!("{item_class} {picked_class}")
        } else {
            item_class.to_string()
        }
    };
    let pick = move || {
        let lane = column_id.with_value(|c| ctx.lane_index(c))?;
        Some(BoardDrag::pick(id.get_value(), label.get_value(), lane))
    };

    let on_keydown = move |event: ev::KeyboardEvent| {
        if !movable {
            return;
        }
        let Some(lane) = column_id.with_value(|c| ctx.lane_index(c)) else {
            return;
        };
        let mut state = ctx.drag.get_untracked();
        let card_label = state
            .as_ref()
            .map(|d| d.card_label.clone())
            .unwrap_or_else(|| label.get_value());
        let outcome = handle_key(
            &mut state,
            &event.key(),
            &id.get_value(),
            &label.get_value(),
            lane,
            &ctx.column_ids(),
        );
        if outcome == BoardKeyOutcome::Ignored {
            return;
        }
        event.prevent_default();
        ctx.drag.set(state);
        match outcome {
            BoardKeyOutcome::Picked => ctx.announce(announce_picked(&card_label)),
            BoardKeyOutcome::Over(index) => ctx.announce(announce_over(&ctx.lane_label(index))),
            BoardKeyOutcome::Moved(mv) => ctx.commit(mv, &card_label, true),
            BoardKeyOutcome::Cancelled => ctx.announce(ANNOUNCE_CANCELLED.to_string()),
            BoardKeyOutcome::Ignored => {}
        }
    };

    // Tabbing away from a picked-up card cancels the move.
    let on_focusout = move |_: ev::FocusEvent| {
        if picked_untracked() {
            ctx.drag.set(None);
            ctx.announce(ANNOUNCE_CANCELLED.to_string());
        }
    };

    let on_dragstart = move |event: ev::DragEvent| {
        if !movable {
            return;
        }
        if let Some(dt) = event.data_transfer() {
            let _ = dt.set_data("text/plain", &id.get_value());
            dt.set_effect_allowed("move");
        }
        ctx.drag.set(pick());
    };
    let on_dragend = move |_: ev::DragEvent| {
        if picked_untracked() {
            ctx.drag.set(None);
        }
    };

    // Touch fallback: a 200 ms long-press picks the card up, then the finger drags it.
    let press_start = StoredValue::new(None::<(i32, i32)>);
    let press_timer = StoredValue::new(None::<TimeoutHandle>);
    let clear_timer = move || {
        if let Some(handle) = press_timer.get_value() {
            handle.clear();
        }
        press_timer.set_value(None);
        press_start.set_value(None);
    };
    let on_pointerdown = move |event: ev::PointerEvent| {
        if !movable || event.pointer_type() != "touch" {
            return;
        }
        clear_timer();
        press_start.set_value(Some((event.client_x(), event.client_y())));
        let handle = set_timeout_with_handle(
            move || {
                press_timer.set_value(None);
                if let Some(drag) = pick() {
                    ctx.announce(announce_picked(&drag.card_label));
                    ctx.drag.set(Some(drag));
                }
            },
            LONG_PRESS,
        );
        press_timer.set_value(handle.ok());
    };
    let on_pointermove = move |event: ev::PointerEvent| {
        if event.pointer_type() != "touch" {
            return;
        }
        if !picked_untracked() {
            let moved = press_start.get_value().is_some_and(|(x, y)| {
                (event.client_x() - x).abs() > TOUCH_SLOP
                    || (event.client_y() - y).abs() > TOUCH_SLOP
            });
            if moved {
                clear_timer();
            }
            return;
        }
        let lane = document()
            .element_from_point(event.client_x() as f32, event.client_y() as f32)
            .and_then(|el| el.closest("[data-board-lane]").ok().flatten())
            .and_then(|el| el.get_attribute("data-board-lane"))
            .and_then(|lane_id| ctx.lane_index(&lane_id));
        if let Some(index) = lane {
            ctx.hover(index);
        }
    };
    let on_pointerup = move |event: ev::PointerEvent| {
        if event.pointer_type() != "touch" {
            return;
        }
        clear_timer();
        if picked_untracked() {
            if let Some(drag) = ctx.drag.get_untracked() {
                ctx.drag.set(None);
                ctx.finish(drag, false);
            }
        }
    };
    let on_pointercancel = move |_: ev::PointerEvent| {
        clear_timer();
        if picked_untracked() {
            ctx.drag.set(None);
            ctx.announce(ANNOUNCE_CANCELLED.to_string());
        }
    };
    // Stop the page from scrolling under a picked-up card.
    let on_touchmove = move |event: ev::TouchEvent| {
        if picked_untracked() {
            event.prevent_default();
        }
    };

    view! {
        <div
            class=class
            data-board-card=card_attr
            data-testid=testid
            tabindex=movable.then_some("0")
            draggable=movable.then_some("true")
            aria-describedby=movable.then_some(hint_id)
            on:keydown=on_keydown
            on:focusout=on_focusout
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            on:pointerdown=on_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=on_pointerup
            on:pointercancel=on_pointercancel
            on:touchmove=on_touchmove
        >
            {render_card.run(card)}
        </div>
    }
}
