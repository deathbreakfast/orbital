//! Static preview registrations — Tree preview lives in orbital-core-components.

use crate::preview::PreviewRegistration;

/// No local registrations; TreeView is registered in `orbital-core-components`.
pub fn all() -> &'static [&'static PreviewRegistration] {
    &[]
}
