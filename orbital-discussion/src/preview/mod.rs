//! Preview registration for discussion components.

pub mod static_registrations;

#[cfg(feature = "preview")]
pub mod fixtures;

#[cfg(feature = "preview")]
pub mod mock_adapter;

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};

#[cfg(feature = "preview")]
pub use fixtures::{empty_thread, sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
