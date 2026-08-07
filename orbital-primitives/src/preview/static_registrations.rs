//! Static preview registrations for gap placeholders in this crate.

use crate::preview::PreviewRegistration;

/// No local gap placeholders yet; leaf crates own their tables.
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}
