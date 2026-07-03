bitflags::bitflags! {
    /// Opt-in / opt-out capabilities (runtime; not Cargo features).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct HistoryFeatures: u32 {
        /// Render user actors as links when `href` is present.
        const ACTOR_LINKS = 1 << 0;
        /// Insert relative date-bucket dividers (Today / Yesterday / Last 7 days / …).
        const DATE_DIVIDERS = 1 << 1;
        /// Make the entry row activatable and fire `HistoryEvents::on_entry_click`.
        const ENTRY_CLICK = 1 << 2;
    }
}

impl HistoryFeatures {
    pub fn default_enabled() -> Self {
        Self::ACTOR_LINKS | Self::DATE_DIVIDERS
    }
}

impl Default for HistoryFeatures {
    fn default() -> Self {
        Self::default_enabled()
    }
}
