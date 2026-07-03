# orbital-history

Composable Leptos **History** library — audit timeline presentation (what changed, when, and by whom) for detail pages, side panels, and dialogs.

Consumers own data via Leptos signals or a page fetcher; Orbital owns presentation, formatting defaults, layout, slots, and render extensibility.

## Quick start

```toml
[dependencies]
orbital-history = { git = "https://github.com/unified-field-dev/orbital", default-features = false }
orbital = { git = "https://github.com/unified-field-dev/orbital", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

Use `default-features = false` in production; enable `preview` only for the doc host.

## Preview

Local `http://127.0.0.1:3010/orbital/history-timeline` (with `cargo leptos watch -p orbital-preview`).

## Scope

**In scope:** scrollable audit timeline from a client signal or server page fetcher; default field-diff / created / deleted formatting; custom per-entry / per-`kind` / change-line renderers; vertical and horizontal orientations; empty / loading / error / end-of-list states; relative date-bucket dividers; multi-field change card.

**Out of scope:** backend, database, privacy, or persistence; product-app or non-Orbital platform dependencies; realtime / live-update protocols; rich-text editing; client-side re-sorting (hosts pre-sort newest-first).

## Deferred

- Imperative handle (scroll-to-entry, refresh)
- Dialog shell helper
- Live-update / subscription hook
- Display-timezone-aware date buckets
- Paged mode, client sort, loaded-entry filter

## Docs

Consumer API: component rustdoc and preview catalog. CSS prefix: `orbital-history__*`. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
