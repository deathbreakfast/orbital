//! Primitive preview registration.

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};

mod collect;
pub mod static_registrations;

pub use collect::collect_all_preview_registrations;
