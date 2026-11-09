mod anchored_counter_badge;
mod position;
mod styles;

pub use anchored_counter_badge::AnchoredCounterBadge;
pub use position::BadgeCorner;

#[cfg(feature = "preview")]
pub use anchored_counter_badge::{
    ANCHOREDCOUNTERBADGE_DESCRIPTION, ANCHOREDCOUNTERBADGE_DOC,
    ANCHOREDCOUNTERBADGE_PREVIEW_REGISTRATION, ANCHOREDCOUNTERBADGE_PROPS,
};
