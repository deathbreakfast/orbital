//! Corner math for [`AnchoredCounterBadge`](super::AnchoredCounterBadge), kept free of the DOM so it can be unit-tested.

/// Corner of the anchor element the badge is centred on.
///
/// `End` is the right edge and `Start` the left edge; the corner does not flip in
/// right-to-left layouts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeCorner {
    /// Top-right corner.
    #[default]
    TopEnd,
    /// Top-left corner.
    TopStart,
    /// Bottom-right corner.
    BottomEnd,
    /// Bottom-left corner.
    BottomStart,
}

/// Viewport-space rectangle (the shape of `getBoundingClientRect()`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Rect {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Overlap of two rects; an empty overlap has `right < left` or `bottom < top`.
    pub fn intersect(self, other: Rect) -> Rect {
        Rect {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    /// True when the rect has no area, as for `display: none` elements.
    pub fn is_empty(self) -> bool {
        self.right - self.left <= 0.0 && self.bottom - self.top <= 0.0
    }
}

/// Slack for sub-pixel layout so a corner sitting exactly on a clip edge still counts as visible.
const EDGE_TOLERANCE: f64 = 1.0;

/// Viewport point for `corner` of `rect`.
pub(crate) fn corner_point(rect: Rect, corner: BadgeCorner) -> (f64, f64) {
    match corner {
        BadgeCorner::TopEnd => (rect.right, rect.top),
        BadgeCorner::TopStart => (rect.left, rect.top),
        BadgeCorner::BottomEnd => (rect.right, rect.bottom),
        BadgeCorner::BottomStart => (rect.left, rect.bottom),
    }
}

/// Where to centre the badge, or `None` when the anchor is empty or its corner is clipped.
///
/// `clip` is the viewport intersected with every scroll container around the anchor.
pub(crate) fn badge_point(anchor: Rect, corner: BadgeCorner, clip: Rect) -> Option<(f64, f64)> {
    if anchor.is_empty() {
        return None;
    }
    let (x, y) = corner_point(anchor, corner);
    let inside = x >= clip.left - EDGE_TOLERANCE
        && x <= clip.right + EDGE_TOLERANCE
        && y >= clip.top - EDGE_TOLERANCE
        && y <= clip.bottom + EDGE_TOLERANCE;
    inside.then_some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARD: Rect = Rect {
        left: 10.0,
        top: 20.0,
        right: 110.0,
        bottom: 70.0,
    };
    const VIEWPORT: Rect = Rect {
        left: 0.0,
        top: 0.0,
        right: 800.0,
        bottom: 600.0,
    };

    #[test]
    fn corner_point_picks_each_corner() {
        assert_eq!(corner_point(CARD, BadgeCorner::TopEnd), (110.0, 20.0));
        assert_eq!(corner_point(CARD, BadgeCorner::TopStart), (10.0, 20.0));
        assert_eq!(corner_point(CARD, BadgeCorner::BottomEnd), (110.0, 70.0));
        assert_eq!(corner_point(CARD, BadgeCorner::BottomStart), (10.0, 70.0));
    }

    #[test]
    fn default_corner_is_top_end() {
        assert_eq!(BadgeCorner::default(), BadgeCorner::TopEnd);
    }

    #[test]
    fn badge_point_inside_viewport() {
        assert_eq!(
            badge_point(CARD, BadgeCorner::TopEnd, VIEWPORT),
            Some((110.0, 20.0))
        );
    }

    #[test]
    fn badge_point_hidden_for_zero_size_anchor() {
        let hidden = Rect::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(badge_point(hidden, BadgeCorner::TopEnd, VIEWPORT), None);
    }

    #[test]
    fn badge_point_kept_for_zero_height_line() {
        // A horizontal rule has width but no height; it still has corners.
        let rule = Rect::new(10.0, 40.0, 210.0, 40.0);
        assert_eq!(
            badge_point(rule, BadgeCorner::TopEnd, VIEWPORT),
            Some((210.0, 40.0))
        );
    }

    #[test]
    fn badge_point_shown_on_clip_edge() {
        // Card flush with the top of its scroller.
        let scroller = Rect::new(0.0, 20.0, 400.0, 300.0);
        let clip = VIEWPORT.intersect(scroller);
        assert_eq!(
            badge_point(CARD, BadgeCorner::TopEnd, clip),
            Some((110.0, 20.0))
        );
    }

    #[test]
    fn badge_point_hidden_when_corner_scrolled_out_of_container() {
        // Top edge scrolled above the scroller; the bottom corners are still visible.
        let scroller = Rect::new(0.0, 50.0, 400.0, 300.0);
        let clip = VIEWPORT.intersect(scroller);
        assert_eq!(badge_point(CARD, BadgeCorner::TopEnd, clip), None);
        assert_eq!(
            badge_point(CARD, BadgeCorner::BottomEnd, clip),
            Some((110.0, 70.0))
        );
    }

    #[test]
    fn badge_point_hidden_outside_viewport() {
        let below = Rect::new(10.0, 700.0, 110.0, 750.0);
        assert_eq!(badge_point(below, BadgeCorner::TopEnd, VIEWPORT), None);
    }

    #[test]
    fn badge_point_hidden_when_containers_do_not_overlap() {
        let a = Rect::new(0.0, 0.0, 100.0, 100.0);
        let b = Rect::new(200.0, 0.0, 300.0, 100.0);
        let clip = a.intersect(b);
        assert_eq!(badge_point(CARD, BadgeCorner::TopStart, clip), None);
    }
}
