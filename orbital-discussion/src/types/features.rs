use bitflags::bitflags;

bitflags! {
    /// Feature flags gating optional Discussion surfaces.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DiscussionFeatures: u32 {
        const ATTACHMENTS = 1 << 0;
        const MARKDOWN = 1 << 1;
        const CITATIONS = 1 << 2;
        const CUSTOM_PARTS = 1 << 3;
        const FOCUS_NAVIGATION = 1 << 4;
        const AGENT_PARTS = 1 << 5;
        /// The host draws its own Reply control (for example in the footer slot), so the
        /// built-in Reply button is hidden. Use it when only some replies accept replies,
        /// such as a locked thread.
        const HOST_REPLY_ACTION = 1 << 6;
    }
}

impl DiscussionFeatures {
    /// Default feature set for Phase 0+ thread navigation.
    pub fn default_enabled() -> Self {
        Self::FOCUS_NAVIGATION
    }
}

impl Default for DiscussionFeatures {
    fn default() -> Self {
        Self::default_enabled()
    }
}
