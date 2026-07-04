use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

use super::HistoryEntry;

/// Async fetcher for server-driven history pages.
pub type HistoryPageFetcher = Arc<
    dyn Fn(
            PageRequest,
        ) -> Pin<Box<dyn Future<Output = Result<Page<HistoryEntry>, ServerFnError>> + Send>>
        + Send
        + Sync,
>;

/// Data source: in-memory list or server page fetcher.
#[derive(Clone)]
pub enum HistorySource {
    /// In-memory list; host owns updates (newest-first).
    Client(RwSignal<Vec<HistoryEntry>>),
    /// Server-driven pages via `orbital_paging`.
    Server {
        fetcher: HistoryPageFetcher,
        page_size: u32,
    },
}

/// How Server sources load pages. Client always renders the full signal list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryPagingMode {
    /// Client: render all. Server: infinite scroll (default for Server).
    #[default]
    InfiniteScroll,
    /// Client: render all. Server: single first-page fetch only.
    None,
    /// Server: one page at a time with footer page controls.
    /// Client: render all (same as [`None`]).
    Paged,
}

impl HistorySource {
    pub fn client_rw(items: RwSignal<Vec<HistoryEntry>>) -> Self {
        Self::Client(items)
    }

    pub fn is_server(&self) -> bool {
        matches!(self, Self::Server { .. })
    }

    pub fn server_page_size(&self) -> Option<u32> {
        match self {
            Self::Server { page_size, .. } => Some(*page_size),
            Self::Client(_) => None,
        }
    }

    pub fn server_fetcher(&self) -> Option<HistoryPageFetcher> {
        match self {
            Self::Server { fetcher, .. } => Some(fetcher.clone()),
            Self::Client(_) => None,
        }
    }

    pub fn client_items(&self) -> Option<RwSignal<Vec<HistoryEntry>>> {
        match self {
            Self::Client(items) => Some(*items),
            Self::Server { .. } => None,
        }
    }
}
