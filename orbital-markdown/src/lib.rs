//! Shared markdown → HTML rendering with citation refs and image support.

mod citation_style;
mod citations;
mod links;
mod mention_style;
mod mentions;
mod options;
mod render_html;
mod sanitize;

pub use citation_style::CitationLinkStyle;
pub use citations::CitationRef;
pub use links::{style_markdown_links, ORBITAL_LINK_INLINE_CLASS};
pub use mention_style::MentionLinkStyle;
pub use mentions::MentionRef;
pub use options::OrbitalMarkdownOptions;
pub use render_html::{render_to_html, RenderContext};
