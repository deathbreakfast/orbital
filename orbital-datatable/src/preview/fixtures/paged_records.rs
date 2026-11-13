//! Preview fixtures for DataTable server-source demos.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::{DataRecord, DataValue};
use orbital_paging::{Page, PageCursor, PageRequest};

use crate::engine::{page_from_processed, process_server_rows};
use crate::types::{DataTableColumnDef, DataTableRowModel};

const MOCK_TOTAL: u32 = 30;

fn mock_columns() -> Vec<DataTableColumnDef> {
    vec![
        DataTableColumnDef::new("name", "Name"),
        DataTableColumnDef::new("role", "Role"),
    ]
}

fn all_mock_records() -> Vec<DataTableRowModel> {
    (1..=MOCK_TOTAL)
        .map(|i| {
            let role = match i % 3 {
                0 => "Admin",
                1 => "Member",
                _ => "Guest",
            };
            DataTableRowModel::new(DataRecord::new(
                format!("user-{i}"),
                HashMap::from([
                    ("name".into(), DataValue::Text(format!("User {i}"))),
                    ("role".into(), DataValue::Text(role.into())),
                ]),
            ))
        })
        .collect()
}

/// Simulates a paginated server fetch for preview / E2E demos.
pub async fn mock_paged_users(request: PageRequest) -> Result<Page<DataRecord>, ServerFnError> {
    let columns = mock_columns();
    let rows = all_mock_records();
    let (processed, total) = process_server_rows(rows, &columns, &request);
    Ok(page_from_processed(processed, total, &request))
}

/// Empty server fetcher for infinite-scroll empty-state previews / E2E.
pub async fn mock_empty_paged_users(
    _request: PageRequest,
) -> Result<Page<DataRecord>, ServerFnError> {
    Ok(Page::empty())
}

/// Build a server [`crate::DataTableSource`] using the mock fetcher.
pub fn mock_server_source(page_size: u32) -> crate::DataTableSource {
    let fetcher: crate::PageFetcher = Arc::new(|request: PageRequest| {
        Box::pin(mock_paged_users(request))
            as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    });
    crate::DataTableSource::Server { fetcher, page_size }
}

/// Build an empty server [`crate::DataTableSource`] for empty-state demos.
pub fn mock_empty_server_source(page_size: u32) -> crate::DataTableSource {
    let fetcher: crate::PageFetcher = Arc::new(|request: PageRequest| {
        Box::pin(mock_empty_paged_users(request))
            as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    });
    crate::DataTableSource::Server { fetcher, page_size }
}

/// Scope the cursor preview fingerprints its cursors with.
const CURSOR_PREVIEW_SCOPE: &str = "data-table-cursor-preview";

/// Simulates a server that pages by cursor and never reports a total, the
/// way a privacy-filtered list does when counting would need a full scan.
///
/// A valid cursor continues from the row it names; anything else pages by
/// `offset`. Both paths return the same rows for the same page.
pub async fn mock_cursor_paged_users(
    request: PageRequest,
) -> Result<Page<DataRecord>, ServerFnError> {
    let (processed, _total) = process_server_rows(all_mock_records(), &mock_columns(), &request);
    let start = match PageCursor::from_request(&request, CURSOR_PREVIEW_SCOPE) {
        Some(cursor) => usize::try_from(cursor.position).unwrap_or(usize::MAX),
        None => request.offset as usize,
    };
    let end = start
        .saturating_add(request.limit as usize)
        .min(processed.len());
    let items: Vec<DataRecord> = processed
        .get(start..end)
        .unwrap_or_default()
        .iter()
        .map(|row| row.record.clone())
        .collect();
    let has_more = end < processed.len();
    let next_cursor = has_more.then(|| {
        PageCursor::new(end as u64, request.query_fingerprint(CURSOR_PREVIEW_SCOPE)).encode()
    });
    Ok(Page {
        items,
        has_more,
        ..Page::default()
    }
    .with_next_cursor(next_cursor))
}

/// Build a server source backed by [`mock_cursor_paged_users`].
///
/// `last_request` receives `"cursor <offset>"` or `"offset <offset>"` for each
/// fetch, so the preview (and its E2E spec) can show which path the table took.
pub fn mock_cursor_server_source(
    page_size: u32,
    last_request: RwSignal<String>,
) -> crate::DataTableSource {
    let fetcher: crate::PageFetcher = Arc::new(move |request: PageRequest| {
        let mode = if PageCursor::from_request(&request, CURSOR_PREVIEW_SCOPE).is_some() {
            "cursor"
        } else {
            "offset"
        };
        last_request.set(format!("{mode} {}", request.offset));
        Box::pin(mock_cursor_paged_users(request))
            as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    });
    crate::DataTableSource::Server { fetcher, page_size }
}
