use crate::BadgeColor;

/// One choice in a [`ChipSelector`](super::ChipSelector): the key stored in the bound
/// list, the text on its chip and option row, and optional extras.
#[derive(Clone, PartialEq)]
pub struct ChipOption {
    /// Key stored in the selection.
    pub value: String,
    /// Text on the chip and in the option list.
    pub text: String,
    /// When set, the chip's text links to the record (a project, a task, a tag page).
    pub href: Option<String>,
    /// Second line under the name in the Persona variant, such as "Person" or "Team".
    pub secondary: Option<String>,
    /// Small badge after the chip's text, such as a task's status.
    pub badge: Option<ChipBadge>,
}

impl ChipOption {
    /// A choice with a key and the text people see.
    pub fn new(value: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            text: text.into(),
            href: None,
            secondary: None,
            badge: None,
        }
    }

    /// Links the chip's text to `href`.
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Sets the Persona variant's second line.
    pub fn secondary(mut self, secondary: impl Into<String>) -> Self {
        self.secondary = Some(secondary.into());
        self
    }

    /// Adds a badge after the chip's text.
    pub fn badge(mut self, text: impl Into<String>, color: BadgeColor) -> Self {
        self.badge = Some(ChipBadge {
            text: text.into(),
            color,
        });
        self
    }
}

/// Badge shown after a chip's text.
#[derive(Clone, PartialEq)]
pub struct ChipBadge {
    /// Badge text, such as "In review".
    pub text: String,
    /// Badge colour.
    pub color: BadgeColor,
}

/// How many values a [`ChipSelector`](super::ChipSelector) holds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChipSelectorMode {
    /// Any number of chips (Tags, Projects, Blocked by).
    #[default]
    Multiple,
    /// At most one value; picking another replaces it (Owner, Parent).
    Single,
}

/// How a [`ChipSelector`](super::ChipSelector) draws options and selected values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChipSelectorVariant {
    /// Text chips and text option rows.
    #[default]
    Chip,
    /// People and teams: option rows are Personas (avatar, name, second line). In
    /// single mode the selected value is a Persona; in multiple mode each chip has
    /// an avatar.
    Persona,
}
