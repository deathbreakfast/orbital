/// Scroll behavior when live entries merge above server pages.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum HistoryLiveScrollPolicy {
    /// Preserve scroll offset when live entries arrive (default).
    #[default]
    Preserve,
    /// Scroll to top after new live entries merge.
    ScrollToTop,
    /// Scroll so the first newly prepended row is visible.
    ScrollToFirstNew,
    /// Scroll to top only when the user is within `near_top_px` of the top.
    ScrollIfNearTop { near_top_px: f64 },
}

impl HistoryLiveScrollPolicy {
    /// Whether live-scroll actions should run for the current scroll offset.
    pub fn should_scroll_on_live_update(self, scroll_top: f64) -> bool {
        match self {
            Self::Preserve => false,
            Self::ScrollToTop | Self::ScrollToFirstNew => true,
            Self::ScrollIfNearTop { near_top_px } => scroll_top <= near_top_px.max(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_never_scrolls() {
        assert!(!HistoryLiveScrollPolicy::Preserve.should_scroll_on_live_update(0.0));
        assert!(!HistoryLiveScrollPolicy::Preserve.should_scroll_on_live_update(100.0));
    }

    #[test]
    fn scroll_if_near_top_threshold() {
        let policy = HistoryLiveScrollPolicy::ScrollIfNearTop { near_top_px: 48.0 };
        assert!(policy.should_scroll_on_live_update(0.0));
        assert!(policy.should_scroll_on_live_update(48.0));
        assert!(!policy.should_scroll_on_live_update(49.0));
    }
}
