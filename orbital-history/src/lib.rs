//! Orbital History — composable audit timeline for Leptos.
//!
//! Host applications own data fetch and domain models; Orbital owns presentation,
//! formatting defaults, layout, slots, and render extensibility.

#![recursion_limit = "512"]

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

mod context;
mod engine;
mod format;
mod products;
mod types;

pub use context::*;
pub use engine::*;
pub use format::*;
pub use products::history::*;
pub use types::*;
