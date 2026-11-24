use leptos::prelude::*;
use orbital_base_components::{FormBind, InputSize};

use super::format::CurrencyLocale;

/// Value binding and identity for [`CurrencyInput`](super::currency_input::CurrencyInput).
#[derive(Default)]
pub struct CurrencyInputBind {
    /// Two-way amount in major units as plain text: optional `-`, digits and a `.`
    /// decimal point (`"1234.56"`), whatever the locale. Empty while there is no number.
    pub value: FormBind<String>,
    /// Explicit `id` for the `<input>`; auto-linked when nested in Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl CurrencyInputBind {
    pub fn new(value: impl Into<FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for CurrencyInputBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

impl From<FormBind<String>> for CurrencyInputBind {
    fn from(value: FormBind<String>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Locale, sign and visual options for [`CurrencyInput`](super::currency_input::CurrencyInput).
#[derive(Default)]
pub struct CurrencyInputAppearance {
    /// Grouping, decimal mark and symbol side.
    pub locale: Signal<CurrencyLocale>,
    /// When true, a leading minus is accepted.
    pub allow_negative: bool,
    /// Placeholder shown while empty; defaults to zero in the locale's format.
    pub placeholder: MaybeProp<String>,
    /// Orbital visual size (small, medium, large).
    pub size: Signal<InputSize>,
    /// When true, the amount and the currency select cannot be edited.
    pub disabled: Signal<bool>,
}
