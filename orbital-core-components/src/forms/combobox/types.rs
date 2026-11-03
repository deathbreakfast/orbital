use leptos::prelude::*;
use orbital_base_components::{ComboboxSize, FormBind};

/// Value and identity bindings for [`Combobox`](super::combobox::Combobox).
#[derive(Default)]
pub struct ComboboxBind {
    /// Two-way text displayed in the combobox input.
    pub value: FormBind<String>,
    /// Selected option values.
    pub selected_options: FormBind<Vec<String>>,
    /// Explicit `id` for the input; auto-linked inside Field.
    pub id: MaybeProp<String>,
    /// Form field name submitted with native form posts.
    pub name: MaybeProp<String>,
}

impl ComboboxBind {
    pub fn new(
        value: impl Into<FormBind<String>>,
        selected_options: impl Into<FormBind<Vec<String>>>,
    ) -> Self {
        Self {
            value: value.into(),
            selected_options: selected_options.into(),
            ..Default::default()
        }
    }
}

/// Visual and interaction attributes for [`Combobox`](super::combobox::Combobox).
#[derive(Default)]
pub struct ComboboxAppearance {
    /// Placeholder text shown when the value is empty.
    pub placeholder: MaybeProp<String>,
    /// Disables typing, opening, and selecting options.
    pub disabled: Signal<bool>,
    /// Shows a clear icon when there is a selection.
    pub clearable: bool,
    /// Accessible name of the clear button. Defaults to "Clear".
    pub clear_label: MaybeProp<String>,
    /// Supports toggling multiple selected options.
    pub multiselect: Signal<bool>,
    /// Visual size of the input control.
    pub size: Signal<ComboboxSize>,
}

impl ComboboxAppearance {
    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
            ..Default::default()
        }
    }
}

impl From<ComboboxSize> for ComboboxAppearance {
    fn from(size: ComboboxSize) -> Self {
        Self {
            size: Signal::from(size),
            ..Default::default()
        }
    }
}
