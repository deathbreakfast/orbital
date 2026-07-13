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
    Deleted {
        label: String,
    },
    FieldDiff {
        field: String,
        old_value: String,
        new_value: String,
    },
    FieldDiffs {
        fields: Vec<HistoryFieldDiff>,
    },
    /// Plain summary when no structured change applies; often paired with a custom renderer.
    Custom {
        summary: String,
    },
    /// Markdown body (rendered when [`HistoryFeatures::MARKDOWN_BODIES`] is enabled).
    Markdown {
        body: String,
        #[serde(default)]
        citations: Vec<super::HistoryCitation>,
        #[serde(default)]
        mentions: Vec<super::HistoryMention>,
        #[serde(default)]
        attachments: Vec<super::HistoryAttachment>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{HistoryAttachment, HistoryCitation, HistoryMention};

    #[test]
    fn markdown_change_round_trip_with_mentions_and_attachments() {
        use leptos::serde_json;
        let change = HistoryChange::Markdown {
            body: "Hi @[Jordan](u1)".into(),
            citations: vec![HistoryCitation {
                id: "c1".into(),
                display_index: 1,
            }],
            mentions: vec![HistoryMention {
                id: "u1".into(),
                display_name: "Jordan".into(),
                avatar_src: None,
                subtitle: Some("Engineer".into()),
            }],
            attachments: vec![HistoryAttachment {
                url: "https://example.com/a.png".into(),
                name: Some("a.png".into()),
                mime: Some("image/png".into()),
            }],
        };
        let json = serde_json::to_string(&change).expect("serialize");
        let restored: HistoryChange = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(change, restored);
    }
}
