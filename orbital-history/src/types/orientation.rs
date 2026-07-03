/// Timeline layout orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryOrientation {
    /// Timeline spine / marker. Default — best for panels, drawers, dialogs, and mobile.
    #[default]
    Vertical,
    /// Timestamp rail on the start edge. Opt-in for wide detail cards.
    Horizontal,
}
