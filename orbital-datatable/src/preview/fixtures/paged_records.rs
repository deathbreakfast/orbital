//! Preview fixtures for DataTable server-source demos.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::{DataRecord, DataValue};
use orbital_paging::{Page, PageRequest};

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
    Ok(Page {
        items: vec![],
        has_more: false,
        total_count: Some(0),
        next_request_offset: None,
    })
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
