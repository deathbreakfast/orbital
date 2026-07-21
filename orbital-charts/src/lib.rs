//! Orbital Charts — chart components and engine.

#![recursion_limit = "512"]
#![allow(clippy::if_same_then_else, clippy::ptr_arg)]

pub use orbital_data::{
    ChartFieldBinding, DataRecord, DataSchema, DataValue, Dataset, FieldDef, ProjectionError,
};

/// Label text for gap placeholder stubs.
pub fn placeholder_label(name: &str) -> String {
    format!("Todo: {name}")
}

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub use preview::{DatasetIntegration, DATASETINTEGRATION_PREVIEW_REGISTRATION};

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

mod charts;
mod context;
mod engine;
mod shared;
mod types;

pub use charts::*;
pub use context::*;
pub use engine::*;
pub use shared::*;
pub use types::*;
