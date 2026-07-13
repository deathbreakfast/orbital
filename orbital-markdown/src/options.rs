use crate::{CitationLinkStyle, MentionLinkStyle};

/// Feature flags for markdown rendering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OrbitalMarkdownOptions {
    /// Resolve `[^id]` citation reference syntax to superscript links.
    pub enable_citation_refs: bool,
    /// Resolve `@[label](id)` mention syntax to styled anchor links.
    pub enable_mention_refs: bool,
    /// Render `![alt](url)` as `<img>` (deduped against attachment URLs in context).
    pub enable_images: bool,
    /// Anchor/id template for citation superscript links.
    pub citation_style: CitationLinkStyle,
    /// Anchor template for mention links.
    pub mention_style: MentionLinkStyle,
}

impl OrbitalMarkdownOptions {
    /// Discussion reply body defaults.
    pub fn discussion_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_mention_refs: false,
            enable_images: true,
            citation_style: CitationLinkStyle::discussion(),
            mention_style: MentionLinkStyle::default(),
        }
    }

    /// History change-body defaults.
    pub fn history_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_mention_refs: true,
            enable_images: true,
            citation_style: CitationLinkStyle::history(),
            mention_style: MentionLinkStyle::history(),
        }
    }
}
