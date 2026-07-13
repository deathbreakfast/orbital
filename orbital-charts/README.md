# orbital-charts

Leptos chart library — bar, line, area, pie, scatter, heatmap, gauge, sparkline, zoom/pan, tooltips, and keyboard navigation.

## Quick start

```toml
[dependencies]
orbital-charts = { version = "0.1", default-features = false }
orbital-ui = { version = "0.1", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

Use `default-features = false` in production; enable `preview` only for the doc host.

## Key types

- `BarChart`, `LineChart`, `PieChart`, `ScatterChart`, `HeatmapChart`, `GaugeChart`
- `ChartContainer`, `ChartFeatures`, `ChartEmbedMode`, `ChartOverlayLayer`
- Shared data: [`orbital-data`](../orbital-data/) `Dataset` (projected via `engine::project_chart_data`)

## Preview

[Bar chart preview](https://unified-field-dev.github.io/orbital/bar-chart) · local `http://127.0.0.1:3010/orbital/bar-chart` (with `cargo leptos watch -p orbital-preview`)

## Deferred (not in current charter)

Advanced chart types: `RadarChart`, `FunnelChart`, `SankeyChart`, `CandlestickChart`, `RadialBarChart`, `RadialLineChart`, `TreemapChart`, `GanttChart`, `PyramidChart`, `RangeBarChart`, polar axes.

## Docs

Consumer API: component rustdoc and preview catalog. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
