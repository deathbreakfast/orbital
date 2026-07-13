use serde::{Deserialize, Serialize};

/// Citation metadata for markdown `[^id]` refs in history change bodies.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryCitation {
    pub id: String,
    pub display_index: usize,
}
