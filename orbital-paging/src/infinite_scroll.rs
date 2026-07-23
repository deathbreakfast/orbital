//! Client-side infinite scroll hook for [`Page<T>`](crate::Page) pagination.
//!
//! Provides [`use_paged_infinite_scroll`], a reusable Leptos hook that
//! encapsulates the 4-signal + `use_infinite_scroll` callback pattern
//! established by the notification bell and leaderboard components.
//!
//! This module is only available when the **`leptos`** feature is enabled.

use crate::Page;
use crate::PageRequest;
use leptos::html::Div;
use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;

/// Reactive state returned by [`use_paged_infinite_scroll`].
///
/// All fields are public `RwSignal`s that can be passed directly to child
/// components as props.
pub struct UsePagedInfiniteScroll<T: Send + Sync + 'static> {
    /// Accumulated items across all loaded pages.
    pub items: RwSignal<Vec<T>>,
    /// Whether more pages are available beyond what has been loaded.
    pub has_more: RwSignal<bool>,
    /// Whether a page fetch is currently in flight.
    pub loading: RwSignal<bool>,
    /// Whether the initial load has completed at least once.
    pub ever_loaded: RwSignal<bool>,
    /// Total item count, populated from the first page's
    /// [`total_count`](crate::Page::total_count) field.
    pub total_count: RwSignal<Option<u64>>,
    /// Offset value the hook will pass as the next request's `offset`.
    ///
    /// Updated after each successful page using [`Page::next_request_offset`] when present,
    /// otherwise `prior_offset + returned_items.len()`.
    pub next_request_offset: RwSignal<u32>,
    /// The scroll sentinel element reference passed to the hook.
    pub scroll_el: NodeRef<Div>,
}

impl<T: Send + Sync + 'static> UsePagedInfiniteScroll<T> {
    /// Clear accumulated items and reset pagination state.
    ///
    /// After calling `reset()`, the infinite-scroll hook will typically
    /// re-fire because the container will be empty (below the distance
    /// threshold), triggering a fresh first-page load.  If your use case
    /// requires an immediate guaranteed reload (e.g. after a WebSocket
    /// event), call `reset()` and then spawn the fetch manually.
    pub fn reset(&self) {
        self.items.set(Vec::new());
        self.has_more.set(true);
        self.ever_loaded.set(false);
        self.loading.set(false);
        self.total_count.set(None);
        self.next_request_offset.set(0);
    }
}

/// Adapt a legacy `(offset, limit)` fetch closure to [`PageRequest`].
pub fn page_fetch_from_tuple<F, Fut>(
    f: F,
) -> impl Fn(PageRequest) -> Fut + Clone + Send + Sync + 'static
where
    F: Fn(u32, u32) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future + Send + 'static,
{
    move |request: PageRequest| f(request.offset, request.limit)
}

/// Create a paginated infinite-scroll hook.
///
/// Encapsulates the common pattern of:
/// - 5 reactive signals (`items`, `has_more`, `ever_loaded`, `loading`, `next_request_offset`)
/// - First-page [`Resource`] pre-loading (both SSR and hydrate — required for
///   hydration contract; see docs/src/02-counter-app-walkthrough/08-realtime-values.md)
/// - [`use_infinite_scroll_with_options`](leptos_use::use_infinite_scroll_with_options)
///   for additional pages (hydrate only)
/// - Offset for the next page: [`Page::next_request_offset`] when the server sets
///   it (DB-row pagination with post-filtering); otherwise `previous_offset + page.len()`
///
/// # Arguments
///
/// * `scroll_el` — [`NodeRef<Div>`] attached to the scrollable container
///   element.  The caller creates this and also passes it to the view's
///   `node_ref`.
/// * `page_size` — number of items per page (passed as `limit` to `fetch`).
/// * `refresh` — bump this signal to reset accumulated items and reload the first page.
/// * `fetch` — async function `PageRequest -> Result<Page<T>, ServerFnError>`.
///   Use [`page_fetch_from_tuple`] to adapt legacy `(offset, limit)` fetchers.
pub fn use_paged_infinite_scroll<T, Fut, F>(
    scroll_el: NodeRef<Div>,
    page_size: u32,
    refresh: Signal<u32>,
    fetch: F,
) -> UsePagedInfiniteScroll<T>
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
    Fut: Future<Output = Result<Page<T>, ServerFnError>> + Send + 'static,
    F: Fn(PageRequest) -> Fut + Send + Sync + Clone + 'static,
{
    let items = RwSignal::new(Vec::<T>::new());
    let has_more = RwSignal::new(false);
    let ever_loaded = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let total_count = RwSignal::new(None::<u64>);
    let next_request_offset = RwSignal::new(0u32);

    let skip_initial_refresh_reset = StoredValue::new(true);
    Effect::new(move || {
        let _ = refresh.get();
        if skip_initial_refresh_reset.get_value() {
            skip_initial_refresh_reset.set_value(false);
            return;
        }
        // Keep prior rows visible while the Resource refetch completes (no empty flash).
        next_request_offset.set(0);
    });

    let fetch_first = fetch.clone();
    let first_page = Resource::new(
        move || refresh.get(),
        move |_| {
            let fetch = fetch_first.clone();
            async move { fetch(PageRequest::new(0, page_size)).await }
        },
    );

    Effect::new(move || {
        if let Some(Ok(page)) = first_page.get() {
            // Coordinator dedupe returns an empty stub (`stale_page` with has_more=true).
            // Never apply it: on a non-empty list it would wipe rows; on an empty list it
            // would flip has_more back to true and re-arm the scroll loader → empty flash.
            let looks_like_stale_stub = page.items.is_empty()
                && page.has_more
                && page.total_count.is_none();
            if looks_like_stale_stub {
                return;
            }
            has_more.set(page.has_more);
            ever_loaded.set(true);
            if let Some(count) = page.total_count {
                total_count.set(Some(count));
            }
            items.set(page.items.clone());
            next_request_offset.set(page.next_request_offset.unwrap_or(page.items.len() as u32));
        }
    });

    #[cfg(not(feature = "hydrate"))]
    Effect::new(move || {
        loading.set(first_page.get().is_none());
    });

    #[cfg(feature = "hydrate")]
    {
        use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};

        let hook_loading =
            use_infinite_scroll_with_options(
                scroll_el,
                move |_| {
                    let fetch = fetch.clone();
                    async move {
                        if !has_more.get_untracked() {
                            return;
                        }
                        // First page is owned by the Resource; never load-more at offset 0
                        // (refresh resets offset to 0 while keeping rows — extending would
                        // duplicate or apply stale stubs and flash).
                        let request_offset = next_request_offset.get_untracked();
                        if request_offset == 0 {
                            return;
                        }
                        match fetch(PageRequest::new(request_offset, page_size)).await {
                            Ok(page) => {
                                let looks_like_stale_stub = page.items.is_empty()
                                    && page.has_more
                                    && page.total_count.is_none();
                                if looks_like_stale_stub {
                                    return;
                                }
                                has_more.set(page.has_more);
                                ever_loaded.set(true);
                                if let Some(count) = page.total_count {
                                    total_count.set(Some(count));
                                }
                                items.update(|existing| existing.extend(page.items.clone()));
                                next_request_offset.set(page.next_request_offset.unwrap_or(
                                    request_offset.saturating_add(page.items.len() as u32),
                                ));
                            }
                            Err(_) => {
                                ever_loaded.set(true);
                                has_more.set(false);
                            }
                        }
                    }
                },
                UseInfiniteScrollOptions::default().distance(10.0),
            );

        Effect::new(move || {
            // Ignore load-more loading pulses when there is nothing more to fetch.
            // Empty tables keep the sentinel in view, so leptos_use would otherwise
            // toggle loading every frame and flash the DataTable empty overlay.
            let load_more_busy = has_more.get() && hook_loading.get();
            loading.set(first_page.get().is_none() || load_more_busy);
        });
    }

    UsePagedInfiniteScroll {
        items,
        has_more,
        loading,
        ever_loaded,
        total_count,
        next_request_offset,
        scroll_el,
    }
}
