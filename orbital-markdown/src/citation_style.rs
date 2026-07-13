/// HTML anchor template for resolved `[^id]` citation refs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CitationLinkStyle {
    pub row_anchor_prefix: &'static str,
    pub ref_id_prefix: &'static str,
    pub anchor_class: &'static str,
}

impl CitationLinkStyle {
    pub fn discussion() -> Self {
        Self {
            row_anchor_prefix: "discussion-citation-row-",
            ref_id_prefix: "discussion-citation-ref-",
            anchor_class: "orbital-markdown__citation-ref",
        }
    }

    pub fn history() -> Self {
        Self {
            row_anchor_prefix: "history-citation-row-",
            ref_id_prefix: "history-citation-ref-",
            anchor_class: "orbital-history__citation-ref",
        }
    }
}

impl Default for CitationLinkStyle {
    fn default() -> Self {
        Self::discussion()
    }
}
