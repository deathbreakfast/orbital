use leptos::prelude::*;
use orbital_base_components::{OptionBind, OrbitalDateTime};

use crate::forms::datetime_bridge::orbital_from_i64;

/// Value binding contract for [`Calendar`](super::calendar::Calendar).
#[derive(Default)]
pub struct CalendarBind {
    pub value: OptionBind<OrbitalDateTime>,
}

impl CalendarBind {
    pub fn new(value: impl Into<OptionBind<OrbitalDateTime>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<OptionBind<OrbitalDateTime>> for CalendarBind {
    fn from(value: OptionBind<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<RwSignal<Option<OrbitalDateTime>>> for CalendarBind {
    fn from(value: RwSignal<Option<OrbitalDateTime>>) -> Self {
        Self::new(value)
    }
}

impl From<Option<OrbitalDateTime>> for CalendarBind {
    fn from(value: Option<OrbitalDateTime>) -> Self {
        Self::new(value)
    }
}

impl From<OrbitalDateTime> for CalendarBind {
    fn from(value: OrbitalDateTime) -> Self {
        Self::new(Some(value))
    }
}

impl From<i64> for CalendarBind {
    fn from(value: i64) -> Self {
        Self::new(orbital_from_i64(value))
    }
}

impl From<Option<i64>> for CalendarBind {
    fn from(value: Option<i64>) -> Self {
        Self::new(value.and_then(orbital_from_i64))
    }
}

impl From<RwSignal<Option<i64>>> for CalendarBind {
    fn from(value: RwSignal<Option<i64>>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}

impl From<OptionBind<i64>> for CalendarBind {
    fn from(value: OptionBind<i64>) -> Self {
        Self::new(value.get_untracked().and_then(orbital_from_i64))
    }
}
