//! Shared markdown → HTML rendering with citation refs and image support.

mod citation_style;
mod citations;
mod options;
mod render_html;
mod sanitize;

pub use citation_style::CitationLinkStyle;
pub use citations::CitationRef;
pub use options::OrbitalMarkdownOptions;
pub use render_html::{render_to_html, RenderContext};
