pub mod coachmark;
pub mod identity_card;
pub mod marketing;

// Re-export patterns
pub use coachmark::{OrbitalCoachmark, RememberMode};
pub use identity_card::IdentityCard;
#[cfg(feature = "preview")]
pub use identity_card::{
    IdentityCardPreview, IDENTITYCARD_DOC, IDENTITYCARD_PREVIEW_REGISTRATION, IDENTITYCARD_PROPS,
};
pub use marketing::*;
#[cfg(feature = "preview")]
pub use orbital_core_components::preview::{
    ComponentDocMarkdown, ComponentPreviewCard, OrbitalComponentView,
};
