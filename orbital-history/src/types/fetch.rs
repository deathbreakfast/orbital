use orbital_paging::PageRequest;

use super::{HistoryFilter, HistorySort};

/// Parameters passed to a server [`HistoryPageFetcher`](super::HistoryPageFetcher).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryFetchParams {
    pub page: PageRequest,
    pub filter: HistoryFilter,
    pub sort: HistorySort,
}

impl HistoryFetchParams {
    pub fn new(page: PageRequest, filter: HistoryFilter, sort: HistorySort) -> Self {
        Self {
            page,
            filter,
            sort,
        }
    }
}
