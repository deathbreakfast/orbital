//! Per-component style injection and SSR style collection for Orbital.

mod inject;
#[cfg(all(test, feature = "hydrate", not(feature = "ssr")))]
mod inject_tests;
mod sr_only;
mod style_registry;

pub use inject::{inject_dynamic_style, inject_style};
pub use sr_only::{ensure_sr_only_styles, SR_ONLY_CSS};
pub use style_registry::{StyleRegistry, StyleRegistryContext};
