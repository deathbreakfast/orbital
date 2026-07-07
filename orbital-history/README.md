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

**In scope:** scrollable audit timeline from a client signal or server page fetcher; default field-diff / created / deleted formatting; custom per-entry / per-`kind` / change-line renderers; natural and compact entry layouts; empty / loading / error / end-of-list / no-matches states; relative date-bucket dividers and compact timestamps with optional display timezone; multi-field change card; imperative handle (`scroll_to_entry`, `scroll_to_entry_or_load`, `scroll_to_top`, Server `refresh`, `prepend_live`, `set_filter`, `set_sort`, `go_to_page`, `export_state`, `restore_state`, `set_read_watermark`, `mark_all_read`, `expand_group`, `collapse_group`, `expand_all_groups`); `HistoryDialog` shell with timeline prop passthrough; Client prepend / Server `live_head` merge / `prepend_live` live-update patterns with `HistoryLiveScrollPolicy`; loaded-entry filter with optional kind/actor chips; Client sort (`CLIENT_SORT`); Server paged mode with page footer and `HistoryPaginationView` slot; optional built-in filter/sort chrome (`FILTER_CHROME`, `SORT_CHROME`); server-side filter/sort fetch protocol (`SERVER_FILTER`, `SERVER_SORT` via `HistoryFetchParams`); Client paged windowing; paged `scroll_to_entry_or_load`; virtualized long lists with measured viewport (`VIRTUALIZE`) and optional variable row heights (`VARIABLE_ROW_HEIGHT`); read-only markdown change bodies (`MARKDOWN_BODIES`, `HistoryChange::Markdown` with optional `citations`, `mentions`, `attachments`); markdown citation refs (`MARKDOWN_CITATIONS`, `on_citation_click`); markdown mention refs (`MARKDOWN_MENTIONS`, Persona hover popover, `on_mention_click`); markdown image attachments (`MARKDOWN_IMAGES`); configurable group collapse by actor or kind (`GROUP_COLLAPSE`, `group_by` prop); field diff highlighting (`DIFF_HIGHLIGHT`); unread markers via `read_watermark` and `UNREAD_HIGHLIGHT`; timeline state export (`HistorySerializedState` including read watermark).

**Out of scope:** backend, database, privacy, or persistence; product-app or non-Orbital platform dependencies; WS/SSE/poll transport inside Orbital (host-owned transport pushes via `live_head`, `prepend_live`, and `refresh` hooks — see `history-live-transport` catalog doc); rich-text entry editing; CSV/JSON export.

## Docs

Consumer API: component rustdoc and preview catalog. CSS prefix: `orbital-history__*`. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
