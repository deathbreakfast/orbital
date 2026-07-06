//! Virtual list window math for long history lists.

/// Default row height estimate for history entry rows (px).
pub const DEFAULT_HISTORY_ROW_HEIGHT_PX: f64 = 72.0;
/// Default overscan rows above/below the visible window.
pub const DEFAULT_HISTORY_ROW_OVERSCAN: usize = 4;
/// Minimum item count before virtualization activates.
pub const HISTORY_VIRTUALIZE_THRESHOLD: usize = 40;

/// Visible index window with spacer padding heights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistoryRowViewport {
    pub start: usize,
    pub end: usize,
    pub padding_top_px: f64,
    pub padding_bottom_px: f64,
}

/// Compute which list item indices to render for a scroll position.
pub fn compute_history_viewport(
    scroll_top: f64,
    viewport_height: f64,
    item_count: usize,
    row_height: f64,
    overscan: usize,
) -> HistoryRowViewport {
    if item_count == 0 || row_height <= 0.0 {
        return HistoryRowViewport {
            start: 0,
            end: 0,
            padding_top_px: 0.0,
            padding_bottom_px: 0.0,
        };
    }

    let first_visible = (scroll_top / row_height).floor() as usize;
    let visible_count = ((viewport_height / row_height).ceil() as usize).max(1);
    let start = first_visible.saturating_sub(overscan);
    let end = (first_visible + visible_count + overscan).min(item_count);

    HistoryRowViewport {
        start,
        end,
        padding_top_px: start as f64 * row_height,
        padding_bottom_px: (item_count.saturating_sub(end)) as f64 * row_height,
    }
}

/// Compute a visible window for variable-height list items.
pub fn compute_variable_viewport(
    scroll_top: f64,
    viewport_height: f64,
    heights: &[f64],
    overscan: usize,
) -> HistoryRowViewport {
    let item_count = heights.len();
    if item_count == 0 {
        return HistoryRowViewport {
            start: 0,
            end: 0,
            padding_top_px: 0.0,
            padding_bottom_px: 0.0,
        };
    }

    let mut cumulative = 0.0;
    let mut start = 0usize;
    while start < item_count && cumulative + heights[start] <= scroll_top {
        cumulative += heights[start];
        start += 1;
    }
    start = start.saturating_sub(overscan);

    let padding_top: f64 = heights.iter().take(start).sum();
    let mut end = start;
    let mut visible = 0.0;
    while end < item_count && visible < viewport_height {
        visible += heights[end];
        end += 1;
    }
    end = (end + overscan).min(item_count);

    let padding_bottom: f64 = heights.iter().skip(end).sum();

    HistoryRowViewport {
        start,
        end,
        padding_top_px: padding_top,
        padding_bottom_px: padding_bottom,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let vp = compute_history_viewport(0.0, 400.0, 0, 72.0, 4);
        assert_eq!(vp.start, 0);
        assert_eq!(vp.end, 0);
    }

    #[test]
    fn variable_window_with_known_heights() {
        let heights = vec![40.0, 120.0, 80.0, 60.0];
        let vp = compute_variable_viewport(50.0, 100.0, &heights, 0);
        assert_eq!(vp.start, 1);
        assert_eq!(vp.padding_top_px, 40.0);
        assert!(vp.end > vp.start);
    }
}
