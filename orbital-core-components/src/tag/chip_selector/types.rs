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
    /// Small badge after the text on the chip and on the option row, such as a
    /// task's status or a kind ("Team", "Guest").
    pub badge: Option<ChipBadge>,
    /// Shown in the option list but can't be picked, such as someone who is already
    /// a member. Arrow keys skip it and screen readers announce it as unavailable.
    pub disabled: bool,
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
            disabled: false,
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

    /// Adds a badge after the text on the chip and on the option row.
    pub fn badge(mut self, text: impl Into<String>, color: BadgeColor) -> Self {
        self.badge = Some(ChipBadge {
            text: text.into(),
            color,
        });
        self
    }

    /// Keeps the option in the list but stops it being picked. Say why on the
    /// second line, such as "Already a member".
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A titled section of a [`ChipSelector`](super::ChipSelector) option list, such as
/// "Recent" or one directory a search came from.
#[derive(Clone, PartialEq)]
pub struct ChipOptionGroup {
    /// Heading above the section, and its accessible name.
    pub header: String,
    /// Muted line under the heading, such as where the results came from.
    pub caption: Option<String>,
    /// The section's choices.
    pub options: Vec<ChipOption>,
}

impl ChipOptionGroup {
    /// A section with a heading and its choices.
    pub fn new(header: impl Into<String>, options: Vec<ChipOption>) -> Self {
        Self {
            header: header.into(),
            caption: None,
            options,
        }
    }

    /// Adds a muted line under the heading.
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
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
