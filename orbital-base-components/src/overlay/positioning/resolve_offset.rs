//! Anchored overlay placement — flip, shift, and clamp into the viewport.

use leptos::prelude::window;
use web_sys::DomRect;

use crate::overlay::placement::Placement;

/// Minimum remaining space (px) required to accept a clamped preferred placement
/// instead of flipping to another side.
const MIN_CLAMP_SPACE: f64 = 48.0;

pub struct AnchorOffset {
    pub top: f64,
    pub left: f64,
    pub transform: String,
    pub placement: Placement,
    pub max_height: Option<f64>,
}

/// Axis-aligned rect used by the pure placement solver (and tests).
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn bottom(self) -> f64 {
        self.top + self.height
    }

    pub fn right(self) -> f64 {
        self.left + self.width
    }

    fn from_dom(rect: &DomRect) -> Self {
        Self {
            top: rect.top(),
            left: rect.left(),
            width: rect.width(),
            height: rect.height(),
        }
    }
}

pub fn resolve_anchor_offset(
    placement: Placement,
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_height: Option<f64>,
) -> Option<AnchorOffset> {
    let vw = window_inner_width()?;
    let vh = window_inner_height()?;
    resolve_anchor_offset_in_viewport(
        placement,
        Rect::from_dom(target_rect),
        Rect::from_dom(content_rect),
        arrow_height,
        vw,
        vh,
    )
}

/// Pure placement solver — preferred clamp/shift, then flip, then viewport-safe must.
pub fn resolve_anchor_offset_in_viewport(
    preferred: Placement,
    target: Rect,
    content: Rect,
    arrow_height: Option<f64>,
    viewport_w: f64,
    viewport_h: f64,
) -> Option<AnchorOffset> {
    use Placement::*;
    let candidates = match preferred {
        TopStart => [TopStart, BottomStart, Right, Left, BottomStart],
        Top => [Top, Bottom, Right, Left, Bottom],
        TopEnd => [TopEnd, BottomEnd, Right, Left, BottomEnd],
        BottomStart => [BottomStart, TopStart, Right, Left, BottomStart],
        Bottom => [Bottom, Top, Right, Left, Bottom],
        BottomEnd => [BottomEnd, TopEnd, Right, Left, BottomEnd],
        RightStart => [RightStart, LeftStart, Top, Bottom, RightStart],
        Right => [Right, Left, Top, Bottom, Right],
        RightEnd => [RightEnd, LeftEnd, Top, Bottom, RightEnd],
        LeftStart => [LeftStart, RightStart, Top, Bottom, LeftStart],
        Left => [Left, Right, Top, Bottom, Left],
        LeftEnd => [LeftEnd, RightEnd, Top, Bottom, LeftEnd],
    };

    let len = candidates.len();
    for (index, placement) in candidates.into_iter().enumerate() {
        let must = index + 1 == len;
        if let Some(offset) = try_placement(
            placement,
            target,
            content,
            arrow_height,
            viewport_w,
            viewport_h,
            must,
        ) {
            return Some(offset);
        }
    }
    None
}

fn try_placement(
    placement: Placement,
    target: Rect,
    content: Rect,
    arrow_height: Option<f64>,
    vw: f64,
    vh: f64,
    must: bool,
) -> Option<AnchorOffset> {
    use Placement::*;
    match placement {
        TopStart | Top | TopEnd => {
            placement_vertical(placement, target, content, arrow_height, vw, vh, must, true)
        }
        BottomStart | Bottom | BottomEnd => {
            placement_vertical(placement, target, content, arrow_height, vw, vh, must, false)
        }
        RightStart | Right | RightEnd => {
            placement_horizontal(placement, target, content, arrow_height, vw, vh, must, true)
        }
        LeftStart | Left | LeftEnd => {
            placement_horizontal(placement, target, content, arrow_height, vw, vh, must, false)
        }
    }
}

fn placement_vertical(
    placement: Placement,
    target: Rect,
    content: Rect,
    arrow_height: Option<f64>,
    vw: f64,
    vh: f64,
    must: bool,
    prefer_top: bool,
) -> Option<AnchorOffset> {
    let arrow = arrow_height.unwrap_or_default();
    let (top, max_height) = if prefer_top {
        let content_height = content.height + arrow;
        let raw_top = target.top - content_height;
        let space_above = target.top.max(0.0);
        if raw_top < 0.0 {
            if !must && space_above < MIN_CLAMP_SPACE {
                return None;
            }
            // Clamp into the viewport rather than placing above y=0.
            (0.0, Some(space_above.max(MIN_CLAMP_SPACE.min(space_above))))
        } else {
            (raw_top, Some(space_above))
        }
    } else {
        let target_bottom = target.bottom() + arrow;
        let space_below = (vh - target_bottom).max(0.0);
        if target_bottom + content.height > vh {
            if !must && space_below < MIN_CLAMP_SPACE {
                return None;
            }
            (target_bottom, Some((space_below - 1.0).max(0.0)))
        } else {
            (target_bottom, Some((space_below - 1.0).max(0.0)))
        }
    };

    // When must would still open Top with no space above a top-edge anchor, refuse
    // so the candidate list can land on Bottom (last Top* preferred lists end on Bottom*).
    if prefer_top && must && top <= 0.0 && target.top < MIN_CLAMP_SPACE {
        return None;
    }

    let (left, transform) = shift_horizontal(placement, target, content, vw)?;

    Some(AnchorOffset {
        top: top.max(0.0),
        left,
        transform,
        placement,
        max_height,
    })
}

fn shift_horizontal(
    placement: Placement,
    target: Rect,
    content: Rect,
    vw: f64,
) -> Option<(f64, String)> {
    use Placement::*;
    let w = content.width;
    match placement {
        TopStart | BottomStart => {
            let mut left = target.left;
            if left + w > vw {
                left = (vw - w).max(0.0);
            }
            if left < 0.0 {
                left = 0.0;
            }
            Some((left, String::new()))
        }
        Top | Bottom => {
            let half = w / 2.0;
            let mut center = target.left + target.width / 2.0;
            if center - half < 0.0 {
                center = half;
            }
            if center + half > vw {
                center = vw - half;
            }
            if w > vw {
                center = vw / 2.0;
            }
            Some((center, String::from("translateX(-50%)")))
        }
        TopEnd | BottomEnd => {
            let mut left = target.right();
            // Panel extends left via translateX(-100%).
            if left - w < 0.0 {
                left = w.min(vw);
            }
            if left > vw {
                left = vw;
            }
            Some((left, String::from("translateX(-100%)")))
        }
        _ => None,
    }
}

fn placement_horizontal(
    placement: Placement,
    target: Rect,
    content: Rect,
    arrow_height: Option<f64>,
    vw: f64,
    vh: f64,
    must: bool,
    prefer_right: bool,
) -> Option<AnchorOffset> {
    let arrow = arrow_height.unwrap_or_default();
    let left = if prefer_right {
        let raw = target.right() + arrow;
        if raw + content.width > vw {
            if !must && (vw - target.right()).max(0.0) < MIN_CLAMP_SPACE {
                return None;
            }
            (vw - content.width).max(0.0)
        } else {
            raw
        }
    } else {
        let content_width = content.width + arrow;
        let raw = target.left - content_width;
        if raw < 0.0 {
            if !must && target.left < MIN_CLAMP_SPACE {
                return None;
            }
            0.0
        } else {
            raw
        }
    };

    let (top, transform) = shift_vertical(placement, target, content, vh)?;

    Some(AnchorOffset {
        top,
        left: left.max(0.0),
        transform,
        placement,
        max_height: None,
    })
}

fn shift_vertical(
    placement: Placement,
    target: Rect,
    content: Rect,
    vh: f64,
) -> Option<(f64, String)> {
    use Placement::*;
    let h = content.height;
    match placement {
        RightStart | LeftStart => {
            let mut top = target.top;
            if top + h > vh {
                top = (vh - h).max(0.0);
            }
            if top < 0.0 {
                top = 0.0;
            }
            Some((top, String::new()))
        }
        Right | Left => {
            let half = h / 2.0;
            let mut center = target.top + target.height / 2.0;
            if center - half < 0.0 {
                center = half;
            }
            if center + half > vh {
                center = vh - half;
            }
            if h > vh {
                center = vh / 2.0;
            }
            Some((center, String::from("translateY(-50%)")))
        }
        RightEnd | LeftEnd => {
            let mut top = target.bottom();
            if top - h < 0.0 {
                top = h.min(vh);
            }
            if top > vh {
                top = vh;
            }
            Some((top, String::from("translateY(-100%)")))
        }
        _ => None,
    }
}

fn window_inner_width() -> Option<f64> {
    let Ok(inner_width) = window().inner_width() else {
        return None;
    };
    inner_width.as_f64()
}

fn window_inner_height() -> Option<f64> {
    let Ok(inner_height) = window().inner_height() else {
        return None;
    };
    inner_height.as_f64()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Placement::*;

    fn target_top_bar_trailing() -> Rect {
        Rect {
            top: 8.0,
            left: 300.0,
            width: 80.0,
            height: 32.0,
        }
    }

    fn tall_panel() -> Rect {
        Rect {
            top: 0.0,
            left: 0.0,
            width: 320.0,
            height: 500.0,
        }
    }

    #[test]
    fn top_preferred_near_top_flips_to_bottom_with_clamp() {
        let offset = resolve_anchor_offset_in_viewport(
            Top,
            target_top_bar_trailing(),
            tall_panel(),
            None,
            390.0,
            844.0,
        )
        .expect("placement");
        assert_eq!(offset.placement, Bottom);
        assert!(offset.top >= 0.0);
        let max_h = offset.max_height.expect("max_height");
        assert!(max_h > 0.0);
        assert!(offset.top + 1.0 < 844.0);
    }

    #[test]
    fn bottom_tall_panel_clamps_instead_of_forcing_top() {
        let offset = resolve_anchor_offset_in_viewport(
            Bottom,
            target_top_bar_trailing(),
            tall_panel(),
            None,
            390.0,
            600.0,
        )
        .expect("placement");
        assert_eq!(offset.placement, Bottom);
        assert!(offset.top >= 40.0);
        assert!(offset.max_height.unwrap() < 600.0);
    }

    #[test]
    fn wide_centered_panel_shifts_into_viewport() {
        let offset = resolve_anchor_offset_in_viewport(
            Bottom,
            Rect {
                top: 8.0,
                left: 320.0,
                width: 60.0,
                height: 32.0,
            },
            Rect {
                top: 0.0,
                left: 0.0,
                width: 320.0,
                height: 120.0,
            },
            None,
            390.0,
            844.0,
        )
        .expect("placement");
        assert_eq!(offset.placement, Bottom);
        // Center with translateX(-50%) must keep panel within [0, vw].
        let half = 160.0;
        assert!(offset.left - half >= -1.0);
        assert!(offset.left + half <= 390.0 + 1.0);
    }

    #[test]
    fn must_never_returns_negative_top_for_top_placement() {
        let offset = resolve_anchor_offset_in_viewport(
            Top,
            Rect {
                top: 4.0,
                left: 10.0,
                width: 40.0,
                height: 24.0,
            },
            Rect {
                top: 0.0,
                left: 0.0,
                width: 200.0,
                height: 400.0,
            },
            None,
            375.0,
            500.0,
        )
        .expect("placement");
        assert!(offset.top >= 0.0);
        assert_ne!(offset.placement, Top);
    }
}
