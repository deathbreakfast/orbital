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
        /// Allow client-side newest/oldest sort (Client source only; default off).
        const CLIENT_SORT = 1 << 3;
        /// Built-in filter search input above the list (default off).
        const FILTER_CHROME = 1 << 4;
        /// Built-in newest/oldest sort toggle (requires `CLIENT_SORT`; default off).
        const SORT_CHROME = 1 << 5;
        /// Pass active filter to Server fetcher (host applies server-side; default off).
        const SERVER_FILTER = 1 << 6;
        /// Pass active sort to Server fetcher (host applies server-side; default off).
        const SERVER_SORT = 1 << 7;
        /// Windowed rendering for long lists (default off).
        const VIRTUALIZE = 1 << 8;
        /// Render `Custom` summaries and markdown change bodies as HTML (default off).
        const MARKDOWN_BODIES = 1 << 9;
        /// Visual old/new highlighting for field diff values (default off).
        const DIFF_HIGHLIGHT = 1 << 10;
        /// Highlight entries newer than the read watermark (default off).
        const UNREAD_HIGHLIGHT = 1 << 11;
        /// Resolve `[^id]` citation refs in markdown bodies (default off).
        const MARKDOWN_CITATIONS = 1 << 12;
        /// Measure row heights for virtualized lists (requires `VIRTUALIZE`; default off).
        const VARIABLE_ROW_HEIGHT = 1 << 13;
        /// Resolve `@[label](id)` mention refs in markdown bodies (default off).
        const MARKDOWN_MENTIONS = 1 << 14;
        /// Render inline markdown images with attachment dedup (default off).
        const MARKDOWN_IMAGES = 1 << 15;
        /// Collapse consecutive entries by actor or kind (default off).
        const GROUP_COLLAPSE = 1 << 16;
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
