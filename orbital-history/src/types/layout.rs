/// Timeline entry layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryLayout {
    /// Timeline spine with stacked timestamp, actor, and change (default).
    #[default]
    Natural,
    /// Dense inline sentence: actor + change + time on one line.
    Compact,
}
