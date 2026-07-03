use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One audit-timeline row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    /// Host-defined kind string. Suggested defaults: `"field_diff"`, `"created"`, `"deleted"`, `"custom"`.
    pub kind: String,
    pub changed_at: DateTime<Utc>,
    pub actor: HistoryActor,
    pub change: HistoryChange,
}

/// Who performed the change.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryActor {
    System,
    User {
        id: String,
        display_name: String,
        /// Host-provided route or URL. Orbital never invents app routes.
        href: Option<String>,
    },
}

/// One field change inside a multi-field card.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryFieldDiff {
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

/// Structured change payload for default formatting.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryChange {
    Created,
    Deleted { label: String },
    FieldDiff {
        field: String,
        old_value: String,
        new_value: String,
    },
    FieldDiffs { fields: Vec<HistoryFieldDiff> },
    /// Plain summary when no structured change applies; often paired with a custom renderer.
    Custom { summary: String },
}
