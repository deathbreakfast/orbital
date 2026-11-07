//! Board (kanban) pattern: labelled lanes of cards that move between lanes by mouse,
//! touch or keyboard. See [`Board`].

mod board;
mod card;
pub mod drag;

pub use board::Board;
#[cfg(feature = "preview")]
pub use board::{BoardPreview, BOARD_DOC, BOARD_PREVIEW_REGISTRATION, BOARD_PROPS};

/// One lane of a [`Board`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoardColumn {
    /// Stable lane id; [`Board`]'s `on_move` reports it as the target lane.
    pub id: String,
    /// Lane heading, such as "Doing".
    pub label: String,
    /// Replaces the card count in the heading's parentheses, such as "3 of 12".
    pub count_label: Option<String>,
}

impl BoardColumn {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            count_label: None,
        }
    }
}

/// One card on a [`Board`]. The board renders it through `render_card` in the lane whose
/// id matches `column_id`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoardCard {
    /// Stable card id; `on_move` reports it.
    pub id: String,
    /// Id of the lane the card sits in.
    pub column_id: String,
    /// Short name read out by screen readers while the card moves ("Picked up …").
    pub label: String,
}

impl BoardCard {
    pub fn new(
        id: impl Into<String>,
        column_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            column_id: column_id.into(),
            label: label.into(),
        }
    }
}
