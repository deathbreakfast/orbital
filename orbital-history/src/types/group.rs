/// Group consecutive timeline entries for collapse UI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryGroupBy {
    #[default]
    None,
    Actor,
    Kind,
}
