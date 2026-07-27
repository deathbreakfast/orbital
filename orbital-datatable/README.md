# orbital-datatable

Leptos **DataTable** — sortable, filterable, editable grids with paging, grouping, pivot, export, and optional chart binding.

## Quick start

```toml
[dependencies]
orbital-datatable = { version = "0.1", default-features = false }
orbital-ui = { version = "0.1", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

Use `default-features = false` in production; enable `preview` only for the doc host.

## Key types

- `DataTable`, `DataTableColumnDef`, `DataTableRowModel`, `DataTableFeatures`
- `DataTableSource`, `DataTableHandle`, `ChartBinding`
- Shared data: [`orbital-data`](../orbital-data/) `Dataset`, `DataRecord`, `DataValue`

## Preview

[Data Table preview](https://unified-field-dev.github.io/orbital/data-table) · local `http://127.0.0.1:3010/orbital/data-table` (with `cargo leptos watch --split -p orbital-preview`)

## Deferred (not in current charter)

- **Chart Panel UI** — toolbar config drawer for chart type and field mapping (charts integration wiring is shipped).

## Docs

Consumer API: component rustdoc and preview catalog (`#[component_doc]`). See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
