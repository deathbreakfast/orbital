//! Paginator component for offset/limit paginated tables.
//!
//! Bridges `orbital_paging`'s `offset`/`limit`/`total_count` convention to the
//! 1-indexed `page`/`page_count` model used by [`Pagination`], so every
//! paginated table in the platform gets consistent controls out of the box.

use leptos::prelude::*;
use orbital_base_components::SignalModel;
use orbital_core_components::{Pagination, PaginationConfig};
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

/// Table footer control for offset/limit queries.
///
/// Bind `offset` and `limit` to your fetch `Resource`, set `total_count` from `Page::total_count`, and page clicks update `offset` automatically. When `total_count` is unknown, navigation collapses to a single page. Renders core [`Pagination`] for the page-button UX.
///
/// # When to use
///
/// - Table and list views with explicit page controls and a known (or eventually known) total
/// - Platform paging convention: `offset` + `limit` rather than page index in your query layer
///
/// Prefer [`crate::components::OrbitalInfiniteScroll`] when load-more-on-scroll fits better — social feeds, activity logs, and long catalogs where scrolling is the primary navigation. Use [`crate::components::EmptyState`] when the first page returns no items.
///
/// # Usage
///
/// 1. Create an `offset` signal (starts at `0`). 2. Pass `limit` matching your query page size. 3. Set `total_count` from the first page response (`Page::total_count`). 4. Bind the table `Resource` to `offset` so page clicks trigger a refetch.
///
/// Renders native [`Pagination`] from `orbital-core-components`.
///
/// Minimal wiring (docs snippet):
///
/// ```rust
/// let offset = RwSignal::new(0u32);
/// let rows = Resource::new(move || offset.get(), |o| fetch_page(o, 25));
/// ```
///
/// # Best Practices
///
/// ## Do's
///
/// * Reset `offset` to `0` when filters or search change. * Keep **limit** stable for a given view. * When `total_count` is unknown, pass `None`.
///
/// ## Don'ts
///
/// * Do not fetch on every render — bind the table `Resource` to `offset`. * Do not use paginator UX when infinite scroll fits better ([`crate::components::OrbitalInfiniteScroll`]).
///
/// # Examples
///
/// ## Default pagination
/// Table footer with 25 rows per page and a known total.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// let offset = RwSignal::new(0u32);
/// let total_count = RwSignal::new(Some(250u64));
///
/// view! {
///     <div data-testid="paginator-preview" style="display: flex; justify-content: center; width: 100%;">
///         <Paginator offset=offset limit=25 total_count=total_count />
///     </div>
/// }
/// ```
///
/// ## Small dataset (3 pages)
/// Compact footer with ten rows per page and thirty total items, yielding three page buttons. Use to verify pagination layout and behavior on short lists without many controls.
/// <!-- preview -->
/// ```rust
/// let offset = RwSignal::new(0u32);
/// let total = RwSignal::new(Some(30u64));
///
/// view! {
///     <div data-testid="paginator-small">
///         <Paginator offset=offset limit=10 total_count=total />
///     </div>
/// }
/// ```
///
/// ## Unknown total
/// Single-page fallback when the server has not returned a total yet.
/// <!-- preview -->
/// ```rust
/// let offset = RwSignal::new(0u32);
/// let total = Signal::derive(|| None::<u64>);
///
/// view! {
///     <div data-testid="paginator-unknown">
///         <Paginator offset=offset limit=25 total_count=total />
///     </div>
/// }
/// ```
///
/// ## Click page 2 updates offset
/// Paginator bound to an offset signal with a live readout of the current offset value. Use to confirm page clicks write back to the parent offset for Resource refetches.
/// <!-- preview -->
/// ```rust
/// let offset = RwSignal::new(0u32);
/// let total = RwSignal::new(Some(50u64));
///
/// view! {
///     <div data-testid="paginator-offset">
///         <Paginator offset=offset limit=25 total_count=total />
///         <span data-testid="paginator-offset-value">{move || offset.get()}</span>
///     </div>
/// }
/// ```
///
/// ## Theme surfaces
/// Active page button resolves theme tokens.
/// <!-- preview -->
/// ```rust
/// let offset = RwSignal::new(25u32);
/// let total = RwSignal::new(Some(100u64));
///
/// view! {
///     <div data-testid="paginator-theme">
///         <Paginator offset=offset limit=25 total_count=total />
///     </div>
/// }
/// ```
///
/// ## Wiring with a Resource
/// Docs-only — not rendered live.
/// ```rust,ignore
/// let offset = RwSignal::new(0u32);
/// let rows = Resource::new(move || (offset.get(),), |(o,)| fetch_page(o, 25));
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "paginator",
    preview_label = "Paginator"
)]
#[component]
pub fn Paginator(
    /// Current 0-based item offset. The component writes this signal when the user clicks a page button.
    offset: RwSignal<u32>,
    /// Number of items per page (passed as `limit` to the fetch function).
    limit: u32,
    /// Total number of matching items. Typically provided on the first page response via `Page::total_count`. When `None`, page count defaults to 1 (disabling last-page navigation).
    #[prop(into)]
    total_count: Signal<Option<u64>>,
) -> impl IntoView {
    let page_count = Memo::new(move |_| {
        total_count
            .get()
            .map(|total| {
                let pages = (total as f64 / limit as f64).ceil() as usize;
                pages.max(1)
            })
            .unwrap_or(1)
    });

    let page = RwSignal::new((offset.get_untracked() / limit) as usize + 1);

    // Bidirectional offset/page sync runs on the client only; SSR renders a static snapshot.
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let current_offset = offset.get();
            let derived_page = (current_offset / limit) as usize + 1;
            if page.get_untracked() != derived_page {
                page.set(derived_page);
            }
        });

        Effect::new(move |_| {
            let current_page = page.get();
            let derived_offset = (current_page.saturating_sub(1) as u32) * limit;
            if offset.get_untracked() != derived_offset {
                offset.set(derived_offset);
            }
        });
    }

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .PaginatorWrapper {
            display: flex;
            justify-content: center;
            padding: var(--orb-space-block-sm) 0;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.paginator_wrapper data-testid="orbital-paginator">
            <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count.into()) />
        </div>
    }
}
