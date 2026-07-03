//! Preview catalog registration for orbital-history.

#[cfg(feature = "preview")]
pub mod fixtures;
#[cfg(feature = "preview")]
pub mod static_registrations;

pub use orbital_core_components::preview::{ComponentPreviewCard, OrbitalComponentView};

#[cfg(feature = "preview")]
pub use orbital_core_components::preview::PreviewRegistration;
