//! In-memory analytics dataset + page fetcher (stand-in for a real DB).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_charts::{BarChart, ChartFieldBinding, DataSchema, Dataset, FieldDef};
use orbital_data::{DataRecord, DataType, DataValue};
use orbital_datatable::{
    ColumnType, DataTable, DataTableColumnDef, DataTableSource, PageFetcher, PagingMode,
};
use orbital_paging::{Page, PageRequest};

const TOTAL_ROWS: u32 = 48;
const PAGE_SIZE: u32 = 8;

const REGIONS: &[&str] = &["North", "South", "East", "West"];

fn all_metric_records() -> Vec<DataRecord> {
    (1..=TOTAL_ROWS)
        .map(|i| {
            let region = REGIONS[((i - 1) as usize) % REGIONS.len()];
            let revenue = 1_000.0 + f64::from(i) * 37.5;
            DataRecord::new(
                format!("row-{i}"),
                HashMap::from([
                    ("region".into(), DataValue::Text(region.into())),
                    ("metric".into(), DataValue::Text(format!("M-{i:02}"))),
                    ("revenue".into(), DataValue::Number(revenue)),
                ]),
            )
        })
        .collect()
}

/// Offset/limit page over the demo catalog (no external DB).
pub async fn fetch_metrics_page(request: PageRequest) -> Result<Page<DataRecord>, ServerFnError> {
    let all = all_metric_records();
    let total = all.len() as u64;
    let start = (request.offset as usize).min(all.len());
    let end = (start + request.limit as usize).min(all.len());
    let items = all[start..end].to_vec();
    Ok(Page {
        items,
        has_more: end < all.len(),
        total_count: Some(total),
        next_request_offset: None,
    })
}

pub fn server_source() -> DataTableSource {
    let fetcher: PageFetcher = Arc::new(|request: PageRequest| {
        Box::pin(fetch_metrics_page(request))
            as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    });
    DataTableSource::Server {
        fetcher,
        page_size: PAGE_SIZE,
    }
}

/// Region totals for the summary [`BarChart`] (full catalog, not the current page).
pub fn region_summary_dataset() -> Dataset {
    let mut totals: HashMap<String, f64> = HashMap::new();
    for record in all_metric_records() {
        let region = match record.values.get("region") {
            Some(DataValue::Text(r)) => r.clone(),
            _ => continue,
        };
        let revenue = match record.values.get("revenue") {
            Some(DataValue::Number(n)) => *n,
            _ => 0.0,
        };
        *totals.entry(region).or_insert(0.0) += revenue;
    }

    let schema = DataSchema::new(vec![
        FieldDef::new("region", "Region", DataType::Text),
        FieldDef::new("revenue", "Revenue", DataType::Number),
    ]);
    let records = REGIONS
        .iter()
        .enumerate()
        .map(|(i, region)| {
            let revenue = totals.get(*region).copied().unwrap_or(0.0);
            DataRecord::new(
                format!("region-{i}"),
                HashMap::from([
                    ("region".into(), DataValue::Text((*region).into())),
                    ("revenue".into(), DataValue::Number(revenue)),
                ]),
            )
        })
        .collect();
    Dataset::new(schema, records)
}

pub fn region_summary_binding() -> ChartFieldBinding {
    ChartFieldBinding::new("region", vec!["revenue".into()])
}

/// Analytics page: summary chart + server-paged table.
#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let summary = region_summary_dataset();
    let binding = region_summary_binding();

    view! {
        <div data-testid="server-paged-analytics" style="display: flex; flex-direction: column; gap: 24px;">
            <section data-testid="summary-chart">
                <h2 style="margin: 0 0 12px;">"Revenue by region (full catalog)"</h2>
                <BarChart
                    dataset=summary
                    binding=binding
                    width=560.0
                    height=280.0
                />
            </section>
            <section data-testid="metrics-table">
                <h2 style="margin: 0 0 12px;">"Server-paged metrics"</h2>
                <p style="margin: 0 0 12px; opacity: 0.8;">
                    {format!("{TOTAL_ROWS} rows · page size {PAGE_SIZE} · DataTableSource::Server")}
                </p>
                <DataTable
                    data_source=server_source()
                    paging=PagingMode::Paged
                    columns=vec![
                        DataTableColumnDef::new("region", "Region"),
                        DataTableColumnDef::new("metric", "Metric"),
                        DataTableColumnDef::new("revenue", "Revenue")
                            .with_col_type(ColumnType::Number),
                    ]
                    max_height=360.0
                />
            </section>
        </div>
    }
}
