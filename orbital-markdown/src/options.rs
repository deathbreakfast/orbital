use crate::{CitationLinkStyle, MentionLinkStyle};

/// Feature flags for markdown rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrbitalMarkdownOptions {
    /// Resolve `[^id]` citation reference syntax to superscript links.
    pub enable_citation_refs: bool,
    /// Resolve `@[label](id)` mention syntax to styled anchor links.
    pub enable_mention_refs: bool,
    /// Render `![alt](url)` as `<img>` (deduped against attachment URLs in context).
    pub enable_images: bool,
    /// Restrict `<img src>` to these URL schemes (compared case-insensitively).
    ///
    /// `None` applies the same schemes as links (`http`, `https`, `mailto`,
    /// `mention-ref`). `Some` can only narrow that set: a scheme outside it is
    /// still dropped. Scheme-less (relative) sources are kept unless
    /// `allow_relative_images` is false.
    pub image_url_schemes: Option<Vec<String>>,
    /// Keep scheme-less (relative) `<img src>` values such as `/a.png`.
    ///
    /// Defaults to `true`. Set it to `false` when every image must be an
    /// absolute URL, for example with `image_url_schemes` set to `https` only.
    pub allow_relative_images: bool,
    /// Anchor/id template for citation superscript links.
    pub citation_style: CitationLinkStyle,
    /// Anchor template for mention links.
    pub mention_style: MentionLinkStyle,
}

impl Default for OrbitalMarkdownOptions {
    fn default() -> Self {
        Self {
            enable_citation_refs: false,
            enable_mention_refs: false,
            enable_images: false,
            image_url_schemes: None,
            allow_relative_images: true,
            citation_style: CitationLinkStyle::default(),
            mention_style: MentionLinkStyle::default(),
        }
    }
}

impl OrbitalMarkdownOptions {
    /// Discussion reply body defaults.
    pub fn discussion_body() -> Self {
        Self {
            enable_citation_refs: true,
            enable_mention_refs: false,
            enable_images: true,
            image_url_schemes: None,
            allow_relative_images: true,
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
            image_url_schemes: None,
            allow_relative_images: true,
            citation_style: CitationLinkStyle::history(),
            mention_style: MentionLinkStyle::history(),
        }
    }
}
