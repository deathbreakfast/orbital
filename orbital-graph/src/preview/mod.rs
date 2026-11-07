//! Preview registration for the graph documentation pages.

pub mod static_registrations;

#[cfg(feature = "preview")]
pub mod fixtures;

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};
