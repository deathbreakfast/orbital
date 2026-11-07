use leptos::prelude::*;

/// Backdrop and keyboard dismiss behavior for [`Dialog`](super::dialog::Dialog).
#[derive(Clone, Copy)]
pub struct DialogDismissConfig {
    /// When true, clicking the backdrop sets `open` to `false`.
    pub mask_closeable: Signal<bool>,
    /// When true, pressing Esc sets `open` to `false`.
    pub close_on_esc: bool,
}

impl Default for DialogDismissConfig {
    fn default() -> Self {
        Self {
            mask_closeable: Signal::from(true),
            close_on_esc: true,
        }
    }
}

/// Width of a [`DialogSurface`](super::surface::DialogSurface).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogSize {
    /// Up to 400px, for confirmations.
    Small,
    /// Up to 600px, for short forms.
    #[default]
    Medium,
    /// Up to 960px, for forms with two columns.
    Large,
    /// The viewport less a 24px margin, for editors that need the screen.
    Full,
}

impl DialogSize {
    /// Suffix of the `orbital-dialog-surface--<suffix>` class.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Full => "full",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_size_default_is_medium_happy() {
        assert_eq!(DialogSize::default(), DialogSize::Medium);
    }

    #[test]
    fn dialog_size_class_suffixes_happy() {
        assert_eq!(DialogSize::Small.class_suffix(), "small");
        assert_eq!(DialogSize::Medium.class_suffix(), "medium");
        assert_eq!(DialogSize::Large.class_suffix(), "large");
        assert_eq!(DialogSize::Full.class_suffix(), "full");
    }
}
