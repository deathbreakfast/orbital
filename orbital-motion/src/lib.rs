#![recursion_limit = "256"]

//! Shared motion vocabulary for Orbital components.
//!
//! Tokens, atoms, presence transitions, and reduced-motion utilities consumed by
//! `orbital-base-components` and `orbital-core-components`.

pub mod atom;
pub mod callback;
pub mod collapse;
pub mod group;
pub mod presence;
pub mod progress_fill;
mod preview_registration;
pub mod reduced_motion;
pub mod slot;
pub mod tokens;

pub use preview_registration::PreviewRegistration;

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

pub use atom::{MotionAtom, SlideFrom};
pub use callback::MotionElementCallback;
pub use group::{MotionGroupContext, OrbitalPresenceGroup, OrbitalPresenceGroupItem};
pub use presence::{presence_styles, OrbitalPresence, PresenceMotion};
pub use progress_fill::{
    clamp_unit_progress, progress_fill_styles, progress_width_percent, ProgressFillMotion,
    DEFAULT_PROGRESS_FILL, EPIC_PROGRESS_FILL, GLACIAL_PROGRESS_FILL, PROGRESS_FILL_CLASS,
    PROGRESS_FILL_RESPECTS_REDUCED_CLASS,
};
pub use reduced_motion::use_reduced_motion;
pub use slot::{resolve_presence_motion, resolve_presence_motion_derived, MotionSlot};
pub use tokens::{MotionCurve, MotionDuration};
