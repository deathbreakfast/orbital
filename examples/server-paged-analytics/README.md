# server-paged-analytics

**Teaches:** `DataTableSource::Server` + `PageRequest`/`Page` paging alongside a summary [`BarChart`](../../orbital-charts) bound to an `orbital_data::Dataset`.

**Topology:** Embedded (in-memory catalog standing in for a DB). Replace [`fetch_metrics_page`](src/data.rs) with a real server function / SQL query in production.

## Prerequisites

```bash
# cargo install cargo-leptos   # once
export LEPTOS_OUTPUT_NAME=server-paged-analytics
```

## Run

```bash
cargo leptos watch --split --project server-paged-analytics
```

Open <http://127.0.0.1:3032/> — region bar chart above; paginate the metrics table below.

Compile-check:

```bash
cargo check -p server-paged-analytics --features ssr
```

**Open first:** [`src/data.rs`](src/data.rs) (`server_source`, `fetch_metrics_page`, `region_summary_dataset`)

**Success:** table shows 8 rows per page across 48 total; Next/Prev changes rows; chart shows four region bars.

**Next step:** Feature-crate composition (scheduler + discussion + history) when that host lands, or browse the [component preview](https://unified-field-dev.github.io/orbital/) for widget APIs.
