use leptos::prelude::*;
use orbital_macros::component_doc;

/// Unified data source for client signals and server fetchers with paged or infinite-scroll pagination.
///
/// Use [`DataTableSource::Server`] with a [`PageFetcher`] and set `paging` to [`PagingMode::Paged`] or [`PagingMode::InfiniteScroll`].
///
/// # When to use
///
/// - Server-driven datasets too large for client memory
/// - Offset/limit APIs shared with [`Paginator`](orbital::Paginator)
/// - Scroll-to-load-more UX
///
/// # Usage
///
/// 1. Build `data_source=DataTableSource::server(fetcher, page_size)`.
/// 2. Set `paging` to match your API contract.
/// 3. Round-trip sort/filter/quick-search via the same fetcher (server mode).
/// 4. Optional: return [`Page::next_cursor`](orbital_paging::Page::next_cursor) to let the
///    table continue from the previous page instead of re-reading from `offset` (see
///    *Cursor paging* below).
///
/// # Cursor paging
///
/// Every request carries `offset` and `limit`. A fetcher that can continue cheaply from
/// where a page stopped (a Valence resume position, a keyset) also returns
/// [`Page::next_cursor`](orbital_paging::Page::next_cursor), built with
/// [`PageCursor`](orbital_paging::PageCursor). The table sends it back as
/// [`PageRequest::cursor`](orbital_paging::PageRequest::cursor):
///
/// | Action | What the fetcher receives |
/// |--------|---------------------------|
/// | Next page | The current page's `next_cursor`, plus the new `offset` |
/// | Previous page | The cursor recorded when that page was first reached; none for page 1 |
/// | Jump to a page never reached by Next | `offset` only |
/// | Sort, filter, quick search, page size or refresh change | `offset` only, from page 1 |
/// | Infinite scroll | The last loaded page's `next_cursor` |
///
/// The fetcher decodes the cursor with
/// [`PageCursor::from_request`](orbital_paging::PageCursor::from_request); when that returns
/// `None` it pages by `offset`. Cursor pages may leave `total_count` as `None`. The footer
/// then shows the rows seen so far ("21–30 of more than 30") and keeps Next enabled while
/// `has_more` is true. The `orbital-paging` crate docs have a full server example.
///
/// # Best Practices
///
/// ## Do's
///
/// * Standardize fetch signatures on [`PageRequest`](orbital_paging::PageRequest)
/// * Use `PagingMode::None` for small static datasets
///
/// ## Don'ts
///
/// * Do not use infinite scroll when exact page counts matter for compliance UIs
/// * Do not enable [`DataTableFeatures::ROW_GROUPING`], [`DataTableFeatures::PIVOTING`], or
///   [`DataTableFeatures::TREE_DATA`] on [`DataTableSource::Server`] expecting the client pipeline
///   to transform fetched pages — implement those semantics in the server fetcher instead
///
/// # Data source reference
///
/// | Type / prop | Description |
/// |-------------|-------------|
/// | [`DataTableSource::Client`] | In-memory rows; engine sorts/filters/paginates locally |
/// | [`DataTableSource::Server`] | `fetcher` + `page_size` for server-driven paging |
/// | [`ServerFetchPolicy`] | Stale-response suppression and optional dedupe on server fetches |
/// | [`PagingMode::Paged`] | Footer page controls (default) |
/// | [`PagingMode::InfiniteScroll`] | Load more rows near scroll bottom |
/// | [`PagingMode::None`] | No pagination chrome |
/// | [`Page::next_cursor`](orbital_paging::Page::next_cursor) | Optional continuation token; the table sends it back on Next and Previous |
///
///
/// # Examples
///
///
/// ## Server paged source
/// Footer pagination drives a mock server fetcher returning distinct rows per page.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableSource, PagingMode};
/// use crate::preview::fixtures::paged_records::mock_server_source;
/// view! {
///     <div data-testid="data-table-server-preview">
///         <DataTable
///             data_source=mock_server_source(5)
///             paging=PagingMode::Paged
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///         />
///     </div>
/// }
/// ```
///
///
/// ## Cursor paged source
/// A fetcher that continues from `next_cursor` and never reports a total. Next and
/// Previous send a cursor; a filter change starts again at page 1 by offset.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableSource, PagingMode};
/// use crate::preview::fixtures::paged_records::mock_cursor_server_source;
/// let last_request = RwSignal::new(String::new());
/// view! {
///     <div data-testid="data-table-cursor-preview">
///         <DataTable
///             data_source=mock_cursor_server_source(5, last_request)
///             paging=PagingMode::Paged
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("role", "Role"),
///             ]
///         />
///         <p>"Last request: " <span data-testid="data-table-cursor-last-request">{move || last_request.get()}</span></p>
///     </div>
/// }
/// ```
///
///
/// ## Infinite scroll
/// Server source with scroll-to-load-more pagination.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableSource, PagingMode};
/// use crate::preview::fixtures::paged_records::mock_server_source;
/// view! {
///     <div data-testid="data-table-infinite-scroll-preview">
///         <DataTable
///             data_source=mock_server_source(8)
///             paging=PagingMode::InfiniteScroll
///             max_height=320.0
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")]
///         />
///     </div>
/// }
/// ```
///
///
/// ## Infinite scroll empty
/// Server infinite scroll with an empty dataset shows overlay empty slot only.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{DataTable, DataTableColumnDef, DataTableEmptyView, DataTableSource, PagingMode};
/// use crate::preview::fixtures::paged_records::mock_empty_server_source;
/// view! {
///     <div data-testid="data-table-infinite-empty-preview">
///         <DataTable
///             data_source=mock_empty_server_source(8)
///             paging=PagingMode::InfiniteScroll
///             max_height=320.0
///             columns=vec![DataTableColumnDef::new("name", "Name"), DataTableColumnDef::new("role", "Role")]
///         >
///             <DataTableEmptyView slot>"No items yet."</DataTableEmptyView>
///         </DataTable>
///     </div>
/// }
/// ```
///
///
/// ## Full pagination footer
/// Locale range label, page size selector, and Pagination control.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{
///     DataTable, DataTableColumnDef, DataTableInitialState, DataTableRowModel, PagingMode,
///     PaginationState,
/// };
/// use orbital_core_components::{Flex, FlexGap, ThemeDensityStepper};
/// let items = RwSignal::new((1..=25).map(|i| {
///     DataTableRowModel::from_text_cells(
///         &i.to_string(),
///         HashMap::from([("name".into(), format!("Row {i}"))]),
///     )
/// }).collect::<Vec<_>>());
/// view! {
///     <div data-testid="data-table-pagination-preview">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <ThemeDensityStepper />
///             <DataTable
///                 paging=PagingMode::Paged
///                 page_size_options=Some(vec![5, 10, 25])
///                 initial_state=DataTableInitialState {
///                     pagination: Some(PaginationState { page: 0, page_size: 5 }),
///                     ..Default::default()
///                 }
///                 columns=vec![DataTableColumnDef::new("name", "Name")]
///                 items=items
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 60,
    preview_slug = "data-table-data-source",
    preview_label = "Data Source & Pagination",
    preview_icon = icondata::AiCloudServerOutlined,
)]
#[component]
pub fn DataTableDataSourceDoc() -> impl IntoView {
    view! { () }
}
