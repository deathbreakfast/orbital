//! Preview registration for scheduler documentation pages.

pub mod static_registrations;

#[cfg(feature = "preview")]
pub mod fixtures;

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};

#[cfg(feature = "preview")]
pub use fixtures::{sample_planned_events, sample_schedule_resources};
