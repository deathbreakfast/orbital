pub mod board;
pub mod coachmark;
pub mod identity_card;
pub mod marketing;

// Re-export patterns
pub use board::{Board, BoardCard, BoardColumn};
#[cfg(feature = "preview")]
pub use board::{BoardPreview, BOARD_DOC, BOARD_PREVIEW_REGISTRATION, BOARD_PROPS};
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
