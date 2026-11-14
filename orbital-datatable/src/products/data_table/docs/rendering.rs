use leptos::prelude::*;
use orbital_macros::component_doc;

/// Loading and empty overlays, localization, RTL layout, programmatic scroll, and virtualized large datasets.
///
/// Customize overlay slots or pass `loading` for async states; set `locale` and `dir` for translated RTL layouts.
///
/// # When to use
///
/// - Async fetch with loading/empty/no-results states
/// - Localized footer and overlay strings
/// - Ten-thousand-row datasets without DOM blow-up
///
/// # Usage
///
/// 1. Pass custom slot children (`DataTableEmptyView`, etc.) or rely on defaults. For loading,
///    choose `loading_style`: a spinner overlay (default) or skeleton rows under the headers.
/// 2. Set [`DataTableLocale`] and `dir=Direction::Rtl` for RTL.
/// 3. Enable `VIRTUALIZATION` and bound `max_height` for large client datasets.
/// 4. Capture [`DataTableHandle`] for `scroll_to_row` / `scroll_to_column`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Always set `max_height` or `flex` when using virtualization or sticky headers
/// * Provide meaningful `locale.no_rows` and `locale.no_results` strings
///
/// ## Don'ts
///
/// * Do not virtualize server-paged tables — page on the server instead
///
///
/// # Examples
///
///
/// ## Overlays
/// Loading, empty, and no-results overlay states with Spinner defaults.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableRowModel, PagingMode};
/// let empty: RwSignal<Vec<DataTableRowModel>> = RwSignal::new(vec![]);
/// view! {
///     <div data-testid="data-table-overlays-preview">
///         <DataTable
///             paging=PagingMode::None
///             max_height=200.0
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=empty
///         />
///     </div>
/// }
/// ```
///
///
/// ## Skeleton loading rows
/// `loading_style=DataTableLoadingStyle::SkeletonRows(n)` shows placeholder rows under the
/// real column headers while loading. The search box and toolbar stay usable.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use crate::products::data_table::phase9_previews::DataTablePreviewControls;
/// use crate::{DataTable, DataTableColumnDef, DataTableLoadingStyle, DataTableRowModel, PagingMode};
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([
///         ("name".into(), "Ada".into()),
///         ("email".into(), "ada@example.com".into()),
///     ])),
/// ]);
/// let loading = RwSignal::new(true);
/// view! {
///     <div data-testid="data-table-skeleton-preview">
///         <DataTablePreviewControls>
///             <Button
///                 appearance=ButtonAppearance::Subtle
///                 attr:data-testid="skeleton-toggle-loading"
///                 on_click=Callback::new(move |_| loading.update(|l| *l = !*l))
///             >
///                 "Toggle loading"
///             </Button>
///         </DataTablePreviewControls>
///         <DataTable
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("email", "Email"),
///             ]
///             items=items
///             loading=loading
///             loading_style=DataTableLoadingStyle::SkeletonRows(4)
///         />
///     </div>
/// }
/// ```
///
///
/// ## Localization and RTL
/// Custom locale strings and right-to-left layout.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableLocale, DataTableRowModel, PagingMode};
/// use orbital_theme::Direction;
/// let items = RwSignal::new(vec![
///     DataTableRowModel::from_text_cells("1", HashMap::from([("name".into(), "Ada".into())])),
/// ]);
/// let locale = DataTableLocale {
///     footer_rows: "{count} lignes".into(),
///     no_rows: "Aucune ligne".into(),
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="data-table-localization-preview" dir="rtl">
///         <DataTable
///             locale=locale
///             dir=Direction::Rtl
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Nom")]
///             items=items
///         />
///     </div>
/// }
/// ```
///
///
/// ## Programmatic scroll
/// Scroll to a row or column via the imperative handle.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use orbital_core_components::{Button, ButtonAppearance, ButtonGroup};
/// use crate::{DataTable, DataTableColumnDef, DataTableEvents, DataTableHandle, DataTableRowModel, PagingMode};
/// let items = RwSignal::new((0..50).map(|i| {
///     DataTableRowModel::from_text_cells(&i.to_string(), HashMap::from([
///         ("name".into(), format!("Row {i}")),
///         ("email".into(), format!("row{i}@example.com")),
///     ]))
/// }).collect::<Vec<_>>());
/// let handle = RwSignal::new(None::<DataTableHandle>);
/// view! {
///     <div data-testid="data-table-scroll-preview">
///         <div class="orbital-data-table__preview-controls">
///             <ButtonGroup>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="scroll-to-row-25"
///                     on:click=move |_| {
///                         if let Some(h) = handle.get() { h.scroll_to_row.run(("25".into(),)); }
///                     }
///                 >
///                     "Scroll to row 25"
///                 </Button>
///             </ButtonGroup>
///         </div>
///         <DataTable
///             max_height=200.0
///             paging=PagingMode::None
///             columns=vec![
///                 DataTableColumnDef::new("name", "Name"),
///                 DataTableColumnDef::new("email", "Email"),
///             ]
///             items=items
///             data_table_events=DataTableEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
///
/// ## Virtualization
/// Ten thousand rows with virtual viewport rendering.
/// <!-- preview -->
/// ```rust,ignore
/// use std::collections::HashMap;
/// use crate::{DataTable, DataTableColumnDef, DataTableFeatures, DataTableRowModel, PagingMode};
/// let items = RwSignal::new((0..10_000).map(|i| {
///     DataTableRowModel::from_text_cells(&i.to_string(), HashMap::from([("name".into(), format!("Row {i}"))]))
/// }).collect::<Vec<_>>());
/// view! {
///     <div data-testid="data-table-virtualization-preview">
///         <DataTable
///             features=DataTableFeatures::VIRTUALIZATION
///             max_height=400.0
///             paging=PagingMode::None
///             columns=vec![DataTableColumnDef::new("name", "Name")]
///             items=items
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Table",
    group_priority = 80,
    preview_slug = "data-table-rendering",
    preview_label = "Rendering & UX",
    preview_icon = icondata::AiEyeOutlined,
)]
#[component]
pub fn DataTableRenderingDoc() -> impl IntoView {
    view! { () }
}
