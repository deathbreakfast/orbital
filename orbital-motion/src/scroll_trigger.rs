//! Pure scroll-direction trigger used by hide-on-scroll chrome.

/// Options for [`resolve_scroll_trigger`] / [`crate::use_scroll_trigger`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollTriggerOptions {
    /// Scroll position (px) below which the trigger is always inactive (bar visible).
    pub threshold: f64,
    /// Minimum |delta| (px) required to flip hidden state — dampens 1px flicker.
    pub hysteresis: f64,
}

impl Default for ScrollTriggerOptions {
    fn default() -> Self {
        Self {
            // ~compact AppBar height
            threshold: 48.0,
            hysteresis: 8.0,
        }
    }
}

/// Compute the next hidden flag from scroll positions.
///
/// - `y < threshold` → always shown (`false`)
/// - scroll down by at least `hysteresis` while past threshold → hidden (`true`)
/// - scroll up by at least `hysteresis` → shown (`false`)
/// - smaller deltas keep the previous `hidden` value
pub fn resolve_scroll_trigger(
    last_y: f64,
    y: f64,
    hidden: bool,
    opts: ScrollTriggerOptions,
) -> bool {
    let threshold = opts.threshold.max(0.0);
    let hysteresis = opts.hysteresis.max(0.0);

    if y < threshold {
        return false;
    }

    let delta = y - last_y;
    if delta >= hysteresis {
        true
    } else if delta <= -hysteresis {
        false
    } else {
        hidden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> ScrollTriggerOptions {
        ScrollTriggerOptions {
            threshold: 48.0,
            hysteresis: 8.0,
        }
    }

    #[test]
    fn resolve_hides_on_scroll_down_past_threshold() {
        assert!(resolve_scroll_trigger(50.0, 80.0, false, opts()));
    }

    #[test]
    fn resolve_shows_on_scroll_up() {
        assert!(!resolve_scroll_trigger(200.0, 160.0, true, opts()));
    }

    #[test]
    fn resolve_stays_visible_below_threshold() {
        assert!(!resolve_scroll_trigger(10.0, 20.0, true, opts()));
        assert!(!resolve_scroll_trigger(40.0, 47.0, true, opts()));
    }

    #[test]
    fn resolve_hysteresis_ignores_small_delta() {
        assert!(!resolve_scroll_trigger(100.0, 104.0, false, opts()));
        assert!(resolve_scroll_trigger(100.0, 96.0, true, opts()));
    }
}
