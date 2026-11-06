//! Orbital DataTable — interactive tabular data product for the Orbital component library.
#![recursion_limit = "512"]

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

mod core;
mod engine;
mod io;
mod products;
mod types;

pub use core::*;
pub use engine::{SortDirection, SortState};
pub use products::*;
pub use types::*;
