//! Pure drag state for [`Board`](super::Board): which card is picked up, which lane it
//! started in and which lane it hovers. Keyboard, HTML5 drag and touch drags all drive
//! the same [`BoardDrag`] so they share one set of rules (clamping, same-lane no-op).

/// A card in flight between lanes. Lanes are addressed by their index in the board's
/// column list at pickup time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardDrag {
    /// Id of the card being moved.
    pub card_id: String,
    /// Human label of the card, used in announcements.
    pub card_label: String,
    /// Lane the card started in.
    pub from: usize,
    /// Lane the card would drop into right now.
    pub over: usize,
}

/// A committed cross-lane move: the card and the id of the lane it lands in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardMove {
    pub card_id: String,
    pub to_column: String,
}

impl BoardDrag {
    /// Pick up `card_id` from lane `from`. The target starts on the source lane.
    pub fn pick(card_id: impl Into<String>, card_label: impl Into<String>, from: usize) -> Self {
        Self {
            card_id: card_id.into(),
            card_label: card_label.into(),
            from,
            over: from,
        }
    }

    /// Move the target lane by `delta`, clamped to `0..lane_count`.
    pub fn move_by(&mut self, delta: isize, lane_count: usize) {
        let last = lane_count.saturating_sub(1);
        self.over = self.over.saturating_add_signed(delta).min(last);
    }

    /// Point the target at lane `index` (used by pointer and HTML5 drags).
    pub fn hover(&mut self, index: usize, lane_count: usize) {
        self.over = index.min(lane_count.saturating_sub(1));
    }

    /// Drop the card. Returns the move to report, or `None` when the card is still over
    /// its own lane (or the lane no longer exists).
    pub fn drop_on(&self, column_ids: &[String]) -> Option<BoardMove> {
        if self.over == self.from {
            return None;
        }
        column_ids.get(self.over).map(|to| BoardMove {
            card_id: self.card_id.clone(),
            to_column: to.clone(),
        })
    }
}

pub fn announce_picked(card_label: &str) -> String {
    format!("Picked up {card_label}")
}

pub fn announce_over(lane_label: &str) -> String {
    format!("Over {lane_label}")
}

pub fn announce_moved(card_label: &str, lane_label: &str) -> String {
    format!("Moved {card_label} to {lane_label}")
}

pub const ANNOUNCE_CANCELLED: &str = "Move cancelled";

/// What a key press on a focused card did to the drag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoardKeyOutcome {
    /// Not a board key; let the event through.
    Ignored,
    /// Space picked the focused card up.
    Picked,
    /// An arrow key moved the target to this lane index.
    Over(usize),
    /// Space or Enter dropped the card on another lane.
    Moved(BoardMove),
    /// Escape, or a drop back on the source lane.
    Cancelled,
}

/// Apply one key press (`KeyboardEvent.key`) on the card `card_id`, which sits in lane
/// `lane`. Space picks up; Left/Right move the target; Space or Enter drop; Escape cancels.
pub fn handle_key(
    state: &mut Option<BoardDrag>,
    key: &str,
    card_id: &str,
    card_label: &str,
    lane: usize,
    column_ids: &[String],
) -> BoardKeyOutcome {
    let Some(drag) = state.as_mut() else {
        if key == " " {
            *state = Some(BoardDrag::pick(card_id, card_label, lane));
            return BoardKeyOutcome::Picked;
        }
        return BoardKeyOutcome::Ignored;
    };
    match key {
        "ArrowRight" | "ArrowLeft" => {
            let delta = if key == "ArrowRight" { 1 } else { -1 };
            drag.move_by(delta, column_ids.len());
            BoardKeyOutcome::Over(drag.over)
        }
        " " | "Enter" => match state.take().and_then(|drag| drag.drop_on(column_ids)) {
            Some(mv) => BoardKeyOutcome::Moved(mv),
            None => BoardKeyOutcome::Cancelled,
        },
        "Escape" => {
            *state = None;
            BoardKeyOutcome::Cancelled
        }
        _ => BoardKeyOutcome::Ignored,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lanes() -> Vec<String> {
        ["todo", "doing", "review", "done"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn press(state: &mut Option<BoardDrag>, key: &str, lane: usize) -> BoardKeyOutcome {
        handle_key(state, key, "c1", "Write spec", lane, &lanes())
    }

    #[test]
    fn keyboard_pick_move_drop_happy() {
        let mut state = None;
        assert_eq!(press(&mut state, " ", 0), BoardKeyOutcome::Picked);
        assert_eq!(announce_picked("Write spec"), "Picked up Write spec");
        assert_eq!(press(&mut state, "ArrowRight", 0), BoardKeyOutcome::Over(1));
        assert_eq!(
            press(&mut state, " ", 0),
            BoardKeyOutcome::Moved(BoardMove {
                card_id: "c1".into(),
                to_column: "doing".into(),
            })
        );
        assert!(state.is_none());
        assert_eq!(
            announce_moved("Write spec", "Doing"),
            "Moved Write spec to Doing"
        );

        // Enter drops too.
        press(&mut state, " ", 1);
        press(&mut state, "ArrowRight", 1);
        assert!(
            matches!(press(&mut state, "Enter", 1), BoardKeyOutcome::Moved(mv) if mv.to_column == "review")
        );
    }

    #[test]
    fn escape_cancels_sad() {
        let mut state = None;
        assert_eq!(press(&mut state, "Escape", 0), BoardKeyOutcome::Ignored);
        press(&mut state, " ", 0);
        press(&mut state, "ArrowRight", 0);
        assert_eq!(press(&mut state, "Escape", 0), BoardKeyOutcome::Cancelled);
        assert!(state.is_none());
        // Enter without a pickup does nothing.
        assert_eq!(press(&mut state, "Enter", 0), BoardKeyOutcome::Ignored);
    }

    #[test]
    fn drop_on_same_lane_is_noop_sad() {
        let mut state = None;
        press(&mut state, " ", 1);
        press(&mut state, "ArrowRight", 1);
        press(&mut state, "ArrowLeft", 1);
        assert_eq!(press(&mut state, " ", 1), BoardKeyOutcome::Cancelled);
        assert!(state.is_none());

        let mut drag = BoardDrag::pick("c1", "Write spec", 2);
        drag.hover(2, 4);
        assert_eq!(drag.drop_on(&lanes()), None);
    }

    #[test]
    fn move_past_last_lane_clamps_sad() {
        let mut state = None;
        press(&mut state, " ", 3);
        assert_eq!(press(&mut state, "ArrowRight", 3), BoardKeyOutcome::Over(3));
        for _ in 0..6 {
            press(&mut state, "ArrowLeft", 3);
        }
        assert_eq!(state.as_ref().map(|d| d.over), Some(0));

        let mut drag = BoardDrag::pick("c1", "Write spec", 0);
        drag.hover(9, 4);
        assert_eq!(drag.over, 3);
    }
}
