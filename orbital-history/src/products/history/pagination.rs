use leptos::prelude::*;
use orbital_base_components::SignalModel;
use orbital_core_components::{Pagination, PaginationConfig};

/// Default page-number footer for Server + [`HistoryPagingMode::Paged`](crate::HistoryPagingMode::Paged).
///
/// `page` is **1-based** (matches core [`Pagination`]).
#[component]
pub fn HistoryDefaultPagination(
    /// 1-based page index.
    page: RwSignal<usize>,
    /// Total page count (at least 1).
    page_count: Signal<usize>,
) -> impl IntoView {
    let config = PaginationConfig::new(
        SignalModel::from(page),
        Signal::derive(move || page_count.get().max(1)),
    );

    view! {
        <div class="orbital-history__pagination" data-testid="history-pagination">
            <Pagination config=config />
        </div>
    }
}
