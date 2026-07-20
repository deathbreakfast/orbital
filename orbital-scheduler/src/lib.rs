//! Orbital Scheduler — calendar and timeline scheduling products.
//!
//! ## Migration
//!
//! - `SchedulerEvent` → [`PlannedEvent`]
//! - `SchedulerResource` → [`ScheduleResource`]

#![recursion_limit = "512"]

/// Label text for gap placeholder stubs.
pub fn placeholder_label(name: &str) -> String {
    format!("Todo: {name}")
}

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

mod calendar;
mod products;
mod shared;
mod timeline;
mod types;

pub use calendar::*;
pub use orbital_base_components::OrbitalDateTime;
pub use orbital_date_pickers::{DateTimePicker, DateTimeRange, DatetimeLocale};
pub use products::*;
pub use shared::*;
pub use timeline::*;
pub use types::*;

#[cfg(feature = "preview")]
pub use calendar::preview_anchor_date;
