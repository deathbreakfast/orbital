use crate::CitationLinkStyle;

/// Feature flags for markdown rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrbitalMarkdownOptions {
    /// Resolve `[^id]` citation reference syntax to superscript links.
    pub enable_citation_refs: bool,
    /// Render `![alt](url)` as `<img>` (deduped against attachment URLs in context).
    pub enable_images: bool,
    /// Anchor/id template for citation superscript links.
    pub citation_style: CitationLinkStyle,
}

impl Default for OrbitalMarkdownOptions {
    fn default() -> Self {
        Self {
            enable_citation_refs: false,
            enable_images: false,
            citation_style: CitationLinkStyle::default(),
        }
    }
}

impl OrbitalMarkdownOptions {
    /// Discussion reply body defaults.
    pub fn discussion_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_images: true,
            citation_style: CitationLinkStyle::discussion(),
        }
    }

    /// History change-body defaults (citation refs only; no inline images).
    pub fn history_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_images: false,
            citation_style: CitationLinkStyle::history(),
        }
    }
}
