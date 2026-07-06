/// HTML anchor template for resolved `@[label](id)` mention refs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MentionLinkStyle {
    pub anchor_class: &'static str,
}

impl MentionLinkStyle {
    pub fn history() -> Self {
        Self {
            anchor_class: "orbital-history__mention-ref",
        }
    }
}

impl Default for MentionLinkStyle {
    fn default() -> Self {
        Self::history()
    }
}
