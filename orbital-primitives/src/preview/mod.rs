//! Primitive preview registration and host catalog composition.
//!
//! External products should not patch Orbital leaf crates to appear in a preview
//! host. Annotate with `#[component_doc]`, export via `preview_registrations!`,
//! then [`PreviewCatalog::orbital`].`extend` the product table.

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};

mod catalog;
mod collect;
pub mod static_registrations;

pub use catalog::{preview_registration_cmp, PreviewCatalog};
pub use collect::collect_all_preview_registrations;
