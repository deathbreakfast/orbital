/// Overlay state for loading, empty, and no-results views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayState {
    None,
    Loading,
    Empty,
    NoResults,
}

/// What the table shows while it loads rows.
///
/// A [`DataTableLoadingView`](crate::DataTableLoadingView) slot, when given, replaces either.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataTableLoadingStyle {
    /// A spinner overlay over the table body (the default).
    #[default]
    Spinner,
    /// This many placeholder rows under the real column headers. The header,
    /// search row and toolbar stay usable; only row data is replaced.
    SkeletonRows(usize),
}

impl DataTableLoadingStyle {
    /// Placeholder row count for [`DataTableLoadingStyle::SkeletonRows`].
    pub fn skeleton_rows(self) -> Option<usize> {
        match self {
            Self::Spinner => None,
            Self::SkeletonRows(n) => Some(n),
        }
    }
}

/// Skeleton row count to render now: only while loading, and only when the
/// style asks for skeleton rows and no custom loading view replaces them.
pub fn skeleton_rows_while_loading(
    overlay: OverlayState,
    style: DataTableLoadingStyle,
    has_loading_view: bool,
) -> Option<usize> {
    if overlay != OverlayState::Loading || has_loading_view {
        return None;
    }
    style.skeleton_rows()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_rows_only_while_loading() {
        let style = DataTableLoadingStyle::SkeletonRows(4);
        assert_eq!(
            skeleton_rows_while_loading(OverlayState::Loading, style, false),
            Some(4)
        );
        assert_eq!(
            skeleton_rows_while_loading(OverlayState::None, style, false),
            None
        );
        assert_eq!(
            skeleton_rows_while_loading(OverlayState::Empty, style, false),
            None
        );
    }

    #[test]
    fn spinner_style_and_custom_view_skip_skeleton_rows() {
        assert_eq!(
            skeleton_rows_while_loading(
                OverlayState::Loading,
                DataTableLoadingStyle::Spinner,
                false
            ),
            None
        );
        assert_eq!(
            skeleton_rows_while_loading(
                OverlayState::Loading,
                DataTableLoadingStyle::SkeletonRows(4),
                true
            ),
            None
        );
    }
}
