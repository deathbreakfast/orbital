//! Static preview registrations for the graph components.

use crate::preview::PreviewRegistration;

#[cfg(feature = "preview")]
use crate::{GRAPHLEGEND_PREVIEW_REGISTRATION, GRAPH_PREVIEW_REGISTRATION};

orbital_macros::preview_registrations! {
    &GRAPH_PREVIEW_REGISTRATION,
    &GRAPHLEGEND_PREVIEW_REGISTRATION,
}
