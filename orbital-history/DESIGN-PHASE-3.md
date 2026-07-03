# orbital-history design — Phase 3

Implementation-ready charter for work listed as **Phase 2 candidates** and related follow-ups. Phase 1 ([DESIGN.md](DESIGN.md)) covers PR1–PR9. Phase 2 ([DESIGN-PHASE-2.md](DESIGN-PHASE-2.md)) covers PR10–PR14: imperative handle, dialog shell, live-update surface, and display-timezone date buckets.

Phase 3 stacks on those public APIs. Do not start Phase 3 PRs until Phase 2 is mergeable (or land behind the same feature branch with Phase 1–2 complete first). Implementers keep the short [README](README.md) charter in sync (move items from Deferred into Scope as they ship).

Deep API guidance lives here and, once implemented, in rustdoc / `#[component_doc]` catalog pages.

---

## 1. Summary

### Phase 1 + 2 dependency

Phase 3 assumes:

- Full Phase 1 timeline: `HistoryEntry`, `HistorySource::{Client, Server}`, slots, renderers, locale, `HistoryFeatures::DATE_DIVIDERS`, loading phases
- Phase 2: `HistoryHandle` (`scroll_to_entry`, `refresh`, `scroll_to_top`), `HistoryEvents::on_handle`, optional `display_timezone` for **bucket** math, optional `live_head` / `HistoryDialog`
- Host pre-sorts newest-first when `CLIENT_SORT` is disabled (default)
- `scroll_to_entry` is a **no-op** when the id is not in the loaded DOM (Phase 2 contract — preserved)

### Goals

Ship the five capabilities deferred from Phase 2 “Candidates”:

- **Timestamp display timezone** — compact / relative timestamps respect `display_timezone` (completes the Phase 2 bucket-only follow-up)
- **Filter / search** — client-side filter of **currently loaded** entries; imperative-first (`filter` prop + `HistoryHandle::set_filter`); no server query protocol
- **Client sort** — opt-in `HistoryFeatures::CLIENT_SORT` for **Client** source only (`NewestFirst` / `OldestFirst`)
- **Paged mode** — `HistoryPagingMode::Paged` with footer page controls and `HistoryHandle::go_to_page` (Server-primary)
- **Load-until-found scroll** — separate `scroll_to_entry_or_load` that pages Server until the id is found, with safety limits

### Non-goals

- Any backend, database, privacy, or persistence design
- Realtime transport (WebSocket, SSE, polling loops, auth, conflict resolution, CRDT)
- Server-driven full-text search or filter protocol (fetcher query params invented by Orbital)
- Re-sorting Server pages inside the crate
- Changing Phase 2 `scroll_to_entry` no-op-on-miss behavior
- Full rich-text / markdown editing
- Product-app or non-Orbital platform dependencies
- Redesign of Phase 1/2 types, slots, or loading-phase rules

### Success criteria

- Visible timestamps follow `display_timezone` when set; `None` preserves UTC display (Phase 2 back-compat)
- A host can filter loaded entries via a controlled `filter` signal or `set_filter` without Orbital owning a search box
- With `CLIENT_SORT` enabled on Client, newest/oldest order is toggleable and date dividers recompute
- Server + `Paged` shows a page-number footer and supports `go_to_page`; infinite scroll remains the default
- `scroll_to_entry_or_load` finds entries on later Server pages or stops cleanly (end of list, error, or `max_pages`)
- Each delivery PR is independently demoable in the Orbital component preview app when it adds UI

### Pattern alignment

| Pattern | Reference |
| --- | --- |
| Page index (0-based) on handle / state | `DataTable` `PaginationState.page` |
| Pagination chrome (1-indexed UI) | `orbital_core_components::Pagination` / `PaginationConfig` |
| Imperative filter / sort on handle | `DataTableHandle` (`set_filter`, `sort_column`) |
| Display timezone for wall-clock formatting | `OrbitalDateTime` + `DatetimeTimezone` (Phase 2 buckets) |
| Scroll-into-view after load | Phase 2 `scroll_to_entry` + discussion scroll helpers |

---

## 2. Architecture deltas

Phase 3 extends the Phase 1–2 crate layout only where needed:

```
orbital-history/
  DESIGN.md
  DESIGN-PHASE-2.md
  DESIGN-PHASE-3.md         # this document
  README.md
  src/
    types/
      handle.rs             # + set_filter, set_sort, go_to_page, scroll_to_entry_or_load
      source.rs             # HistoryPagingMode::Paged
      features.rs           # CLIENT_SORT (FILTER only if default chrome ships)
      filter.rs             # HistoryFilter
      sort.rs               # HistorySort
      locale.rs             # + no_matches (and sort labels if UI ships)
    format/
      mod.rs                # timestamp formatters take DatetimeTimezone
    engine/
      mod.rs                # apply_filter, apply_sort (pure projection)
    products/
      history/
        timeline.rs         # filter / sort / paged / max_scroll_load_pages props
        list.rs             # projected entries
        pagination.rs       # default page footer (PR18)
        scroll.rs           # load-until-found hunt (PR19)
        docs/
    preview/
      fixtures.rs
      static_registrations.rs
```

No networking or search-index module. Filter and sort operate on in-memory entry lists only.

---

## 3. Feature designs

### 3.1 Timestamp display timezone

Phase 2 wires `display_timezone` into **date-bucket** boundaries only. Visible compact / relative timestamps may still format in UTC.

Phase 3 completes the follow-up:

1. Resolve `tz = display_timezone.map(|s| s.get()).unwrap_or(DatetimeTimezone::Utc)`.
2. Pass `tz` into `HistoryTimestamp` and locale helpers:
   - `format_compact_time(&self, at: DateTime<Utc>, tz: DatetimeTimezone) -> String`
   - `format_relative_time(&self, at: DateTime<Utc>, now: DateTime<Utc>, tz: DatetimeTimezone) -> String` (if relative math needs wall-clock days; otherwise keep duration-based relative and only change compact absolute)
3. Prefer `OrbitalDateTime::from_instant(at, tz)` + existing base-components format helpers.
4. Machine-readable `<time datetime>` remains ISO-8601 **UTC** (a11y / interchange unchanged).

Default `display_timezone: None` → UTC for both buckets (Phase 2) and timestamps (Phase 3), preserving back-compat.

Unit tests: same instant formats differently under `Utc` vs a fixed offset (e.g. evening UTC vs prior calendar evening in a negative offset).

---

### 3.2 Filter / search (loaded entries only)

#### Non-goals

Orbital does **not** send filter criteria to the page fetcher, invent query strings, or search unloaded Server pages.

#### Types

```rust
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryFilter {
    /// Case-insensitive substring match against actor display name, kind,
    /// and formatted change summary (via `format_change`).
    pub query: String,
    /// When `Some` and non-empty, entry `kind` must be in the set.
    pub kinds: Option<Vec<String>>,
    /// When `Some` and non-empty, `HistoryActor::User { id }` must be in the set.
    /// `System` actors never match a non-empty `actor_ids` filter.
    pub actor_ids: Option<Vec<String>>,
}

impl HistoryFilter {
    pub fn is_active(&self) -> bool {
        !self.query.trim().is_empty()
            || self.kinds.as_ref().is_some_and(|k| !k.is_empty())
            || self.actor_ids.as_ref().is_some_and(|a| !a.is_empty())
    }
}
```

#### Projection

Pure helper in `engine/`:

```rust
pub fn apply_filter(
    entries: &[HistoryEntry],
    filter: &HistoryFilter,
    locale: &HistoryLocale,
) -> Vec<HistoryEntry>;
```

Apply to **currently loaded** entries:

| Source / mode | Input list |
| --- | --- |
| Client | Full signal vec (after optional client sort) |
| Server + InfiniteScroll | Accumulated pages (and `live_head` if present) |
| Server + Paged | Current page items only |
| Server + None | First-page items |

Pipeline order: **load/source list → sort (if CLIENT_SORT) → filter → date dividers**.

#### API (imperative-first)

```rust
// HistoryTimeline prop
#[prop(optional)]
filter: Option<Signal<HistoryFilter>>,

// HistoryHandle
pub set_filter: Callback<(HistoryFilter,), ()>,
```

Resolution:

1. If `filter: Some(signal)` is provided, that signal is the source of truth (controlled).
2. Else the timeline owns an internal `RwSignal<HistoryFilter>` updated by `set_filter`.
3. `set_filter` on a controlled timeline updates only if the host also writes the signal — prefer documenting controlled mode as host-owned: `set_filter` writes through when the prop is a `RwSignal` exposed as `Signal`, or is a no-op write that hosts should mirror. Simplest implementable rule: **controlled `filter` prop wins for reads; `set_filter` updates internal state only when `filter` prop is `None`**. Hosts using controlled mode call `filter_signal.set(...)` themselves (handle method still useful for uncontrolled embeds).

No default filter chrome in Phase 3 unless a later PR adds `HistoryFeatures::FILTER` (default **off**). Hosts own the search box / chips and bind `filter`.

#### Empty state

When the source list is non-empty but the filtered list is empty, show empty chrome with `locale.no_matches` (new string; English default e.g. `"No matching history"`). Do **not** use `locale.empty` (that means no history at all).

```rust
// HistoryLocale addition
pub no_matches: String,
```

---

### 3.3 Client sort (`CLIENT_SORT`)

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistorySort {
    #[default]
    NewestFirst,
    OldestFirst,
}

// HistoryFeatures
const CLIENT_SORT = 1 << 3; // default_enabled does NOT include this bit
```

#### Behavior

| Source | `CLIENT_SORT` enabled | Behavior |
| --- | --- | --- |
| Client | yes | Project entries with `apply_sort`; optional UI or handle `set_sort` |
| Client | no (default) | Host order unchanged (newest-first contract) |
| Server | any | **No re-sort.** Fetcher must return the desired order. `set_sort` is a no-op (or updates internal state only for future use — prefer **no-op** + rustdoc). |

```rust
pub fn apply_sort(entries: &[HistoryEntry], sort: HistorySort) -> Vec<HistoryEntry>;
```

- `NewestFirst`: `changed_at` descending, stable by `id` on ties
- `OldestFirst`: `changed_at` ascending, stable by `id` on ties

Date dividers run on the sorted list. When sort is `OldestFirst`, bucket order in the list is reversed relative to the default newest-first audit log (Today may appear at the bottom) — that is expected.

#### API

```rust
#[prop(optional, default = HistorySort::NewestFirst)]
sort: HistorySort, // or Option<Signal<HistorySort>> if controlled sort is needed

// HistoryHandle
pub set_sort: Callback<(HistorySort,), ()>,
```

Prefer a controlled optional `sort: Option<Signal<HistorySort>>` mirroring filter, with internal state when `None` and `CLIENT_SORT` enabled. When `CLIENT_SORT` is disabled, ignore sort prop/handle.

Default chrome (sort toggle) is **optional**. Imperative-first: no built-in toggle required in PR17; a minimal preview control is enough. If a default control ships, gate it behind `CLIENT_SORT` and keep it minimal (e.g. menu or segmented control using locale labels).

```rust
// HistoryLocale additions (if UI ships)
pub sort_newest: String,
pub sort_oldest: String,
```

---

### 3.4 Paged mode

Phase 1 `HistoryPagingMode`:

```rust
pub enum HistoryPagingMode {
    InfiniteScroll, // default for Server
    None,
}
```

Phase 3 adds:

```rust
pub enum HistoryPagingMode {
    /// Client: render all. Server: infinite scroll (default for Server).
    #[default]
    InfiniteScroll,
    /// Client: render all (Paged is ignored). Server: single first-page fetch only.
    None,
    /// Server: one page at a time with footer page controls.
    /// Client: render all (same as None) — page bar is Server-primary.
    Paged,
}
```

#### Server + `Paged`

1. Fetch exactly one page for the current page index (0-based), using `page_size` from `HistorySource::Server`.
2. Render that page’s items (after filter projection).
3. Show footer chrome via `orbital_core_components::Pagination` (1-indexed UI).
4. Derive `page_count` from `Page::total_count` when present: `ceil(total_count / page_size)`. If `total_count` is missing, fall back to prev/next only (`has_more` / page > 0) or hide numbered buttons — document the preferred path as **total_count required for full page numbers**.
5. Page change: fetch the new page; use **incremental** loading chrome when a page was already shown (not full skeleton). Initial load of page 0 with empty list may use the skeleton.

#### Client + `Paged`

**Ignore** `Paged` — render the full client list (same as `None` / `InfiniteScroll` for Client). Rationale: in-memory lists are already complete; a page bar adds little and complicates filter/sort. Hosts that need windowing can slice the signal themselves.

#### Handle

```rust
/// Jump to a 0-based page index (Server + Paged only). No-op otherwise.
pub go_to_page: Callback<(usize,), ()>,
```

0-based index matches `DataTable` `PaginationState.page`. Convert to 1-based when binding `PaginationConfig::page`.

Out-of-range page indices clamp to `[0, page_count)` or no-op — prefer **clamp**.

#### Slots

Optional `HistoryPaginationView` slot to replace the default footer. Default uses core `Pagination`.

#### `refresh` interaction

Server `refresh` (Phase 2) resets to page 0 and re-fetches.

---

### 3.5 Load-until-found (`scroll_to_entry_or_load`)

Phase 2 `scroll_to_entry` stays a **no-op** when the id is missing. Phase 3 adds a **separate** method so that contract does not change.

```rust
/// Scroll to entry id. On Server + InfiniteScroll, load additional pages until
/// found, exhausted, errored, or `max_scroll_load_pages` is hit. Then scroll
/// or no-op.
pub scroll_to_entry_or_load: Callback<(String,), ()>,
```

#### Behavior

| Source / mode | Behavior |
| --- | --- |
| Client | Identical to `scroll_to_entry` (full list already loaded) |
| Server + None / Paged | Search current loaded page only (same as `scroll_to_entry`). Do **not** auto-advance pages in Paged mode (host should `go_to_page` if they know the page). |
| Server + InfiniteScroll | If id in DOM → scroll. Else while `has_more` and pages loaded this hunt `< max_scroll_load_pages`, fetch next page, then check again. Stop on found (scroll), `!has_more`, fetch error (`on_load_error`), or max pages (no-op). |

#### Safety

```rust
/// Max additional pages to fetch during `scroll_to_entry_or_load` (default 20).
#[prop(optional, default = 20)]
max_scroll_load_pages: u32,
```

- Cancel an in-flight hunt on component unmount or when a newer `scroll_to_entry_or_load` call starts (latest wins).
- Show incremental loading chrome during the hunt.
- Never use the full timeline skeleton while entries are already visible.

#### Why not Paged auto-advance

Walking page indices without knowing which page contains the id requires either total scans or server search — out of scope. Hosts with deep links into paged history should resolve the page server-side or use InfiniteScroll + `scroll_to_entry_or_load`.

---

## 4. API surface (Phase 3 delta)

### Types

| Item | Module | PR |
| --- | --- | --- |
| `HistoryFilter` | `types/filter.rs` | PR16 |
| `HistorySort` | `types/sort.rs` | PR17 |
| `HistoryPagingMode::Paged` | `types/source.rs` | PR18 |
| `HistoryFeatures::CLIENT_SORT` | `types/features.rs` | PR17 |

### `HistoryTimeline` props added

| Prop | Type | Default | PR |
| --- | --- | --- | --- |
| `filter` | `Option<Signal<HistoryFilter>>` | `None` (internal) | PR16 |
| `sort` | `Option<Signal<HistorySort>>` or `HistorySort` | `NewestFirst` | PR17 |
| `max_scroll_load_pages` | `u32` | `20` | PR19 |

`display_timezone` already exists in Phase 2; PR15 only consumes it for timestamps.

### `HistoryHandle` methods added

| Method | PR |
| --- | --- |
| `set_filter` | PR16 |
| `set_sort` | PR17 |
| `go_to_page` | PR18 |
| `scroll_to_entry_or_load` | PR19 |

### Locale

| Field | PR |
| --- | --- |
| `no_matches` | PR16 |
| `sort_newest` / `sort_oldest` | PR17 (if UI ships) |

### Format / engine

| Item | PR |
| --- | --- |
| Timestamp formatters take `DatetimeTimezone` | PR15 |
| `apply_filter` | PR16 |
| `apply_sort` | PR17 |

### Components / chrome

| Item | PR |
| --- | --- |
| Default pagination footer (`Pagination`) | PR18 |
| Optional `HistoryPaginationView` slot | PR18 |

### Usage sketches

#### Timestamp timezone

```rust,ignore
view! {
    <HistoryTimeline
        data_source=source
        display_timezone=Signal::from(DatetimeTimezone::Local)
    />
}
```

#### Controlled filter

```rust,ignore
let filter = RwSignal::new(HistoryFilter {
    query: String::new(),
    ..Default::default()
});
view! {
    <input
        prop:value=move || filter.get().query
        on:input=move |ev| {
            filter.update(|f| f.query = event_target_value(&ev));
        }
    />
    <HistoryTimeline data_source=source filter=filter.read_only() />
}
```

#### Client sort

```rust,ignore
view! {
    <HistoryTimeline
        data_source=HistorySource::Client(entries)
        features=HistoryFeatures::default_enabled() | HistoryFeatures::CLIENT_SORT
    />
}
// via handle:
// h.set_sort.run((HistorySort::OldestFirst,));
```

#### Paged Server

```rust,ignore
view! {
    <HistoryTimeline
        data_source=HistorySource::Server { fetcher, page_size: 20 }
        paging=HistoryPagingMode::Paged
    />
}
// h.go_to_page.run((2usize,)); // 0-based → third page
```

#### Load-until-found

```rust,ignore
if let Some(h) = handle.get() {
    h.scroll_to_entry_or_load.run(("entry-42".into(),));
}
```

---

## 5. Layout, a11y, and loading (delta only)

### Timestamps

- Visible text uses display timezone; `datetime` attribute stays UTC ISO-8601.
- No extra controls.

### Filter

- Host-owned inputs; timeline does not steal focus.
- `no_matches` empty region uses the same empty-slot structure as `empty`, with distinct copy.
- Filtering does not change loading phase selection.

### Sort

- If a default sort control ships, it is a single activatable control with an accessible name from locale strings.
- Re-sort does not reset scroll unless the host calls `scroll_to_top`.

### Pagination footer

- Place below the scroll region (or sticky footer inside the product root), class `orbital-history__pagination`.
- Wire `Pagination` with `aria` defaults from core components.
- Page changes announce busy state via existing incremental loading (`aria-busy` on the list region when appropriate).

### Load-until-found

- Incremental footer spinner while hunting.
- Do not move focus to the entry unless the host does so separately (scroll-only, Phase 2 rule).

### Pipeline vs dividers

When `DATE_DIVIDERS` is on, dividers reflect the **projected** list (sorted + filtered). Empty buckets are never shown.

---

## 6. Preview & docs plan

### Catalog pages (add / extend)

| Slug | Focus | PR |
| --- | --- | --- |
| `history-timezone-display` | Compact timestamps under Local vs Utc | PR15 |
| `history-filter` | Controlled filter + `no_matches` | PR16 |
| `history-sort` | `CLIENT_SORT` newest/oldest | PR17 |
| `history-paged` | Server `Paged` + `go_to_page` | PR18 |
| `history-scroll-load` | `scroll_to_entry_or_load` across pages | PR19 |

Extend `history-timezone-buckets` (Phase 2) only if PR15 shares the page; prefer a dedicated display page for clarity.

### Fixtures

- Entries with `changed_at` near TZ day boundaries (timestamp strings differ by zone)
- Mixed kinds / actors for filter demos
- Long Client list for sort
- Mock Server fetcher with `total_count` and stable ids across pages for paged + scroll-load demos
- Entry id only on a late infinite-scroll page for load-until-found

### Registration

Same Phase 1 pattern: `#[component_doc]`, `preview/static_registrations.rs` → `all()`, preview-app collect loop.

---

## 7. Phased delivery

Every phase lists scope, public API added, previews/tests, and explicit out-of-scope. Prefer small, mergeable PRs. Stack on `feat/history` (or equivalent) after Phase 2, as directed by maintainers.

### PR15 — Timestamp display timezone

| | |
| --- | --- |
| **Scope** | Wire Phase 2 `display_timezone` into compact / relative timestamp formatting; UTC when `None` |
| **Public API** | Format helper signatures that accept `DatetimeTimezone` (or internal-only if locale methods gain an overload) |
| **Previews / tests** | `history-timezone-display`; unit tests for offset boundaries |
| **Out of scope** | Filter, sort, paged mode, load-until-found |

### PR16 — Filter projection

| | |
| --- | --- |
| **Scope** | `HistoryFilter`, `apply_filter`, `filter` prop, `set_filter`, `locale.no_matches` |
| **Public API** | Filter type, prop, handle method, locale field |
| **Previews / tests** | `history-filter` with host-owned input |
| **Out of scope** | Default filter chrome, server-side search, sort, paged |

### PR17 — Client sort

| | |
| --- | --- |
| **Scope** | `HistorySort`, `HistoryFeatures::CLIENT_SORT`, `apply_sort`, `set_sort`, divider recompute |
| **Public API** | Sort type, feature bit, handle method, optional sort prop |
| **Previews / tests** | `history-sort` |
| **Out of scope** | Server re-sort, paged mode, load-until-found |

### PR18 — Paged mode

| | |
| --- | --- |
| **Scope** | `HistoryPagingMode::Paged`, pagination footer (`Pagination`), `go_to_page`, loading rules on page change |
| **Public API** | `Paged` variant, handle method, optional pagination slot |
| **Previews / tests** | `history-paged` with `total_count` |
| **Out of scope** | Client page windowing, load-until-found, filter chrome |

### PR19 — Load-until-found

| | |
| --- | --- |
| **Scope** | `scroll_to_entry_or_load`, `max_scroll_load_pages`, cancel/latest-wins, incremental loading during hunt |
| **Public API** | Handle method + prop |
| **Previews / tests** | `history-scroll-load` |
| **Out of scope** | Auto page-walk in `Paged` mode, changing `scroll_to_entry` no-op contract |

---

## 8. Candidates (not in Phase 3)

Documented so implementers do not silently expand scope:

- Server-side search / filter protocol (pass filter into `HistoryPageFetcher`)
- Server-side sort hints in the fetcher
- Client in-memory page windowing for `Paged`
- Default filter / sort chrome beyond minimal preview controls
- Load-until-found across `Paged` pages
- Realtime transport (WS/SSE)
- Rich-text / markdown in change bodies
- Virtualized long lists

---

## 9. Open questions

None remaining for implementers.

### Resolved decisions

- **Timestamp TZ:** `display_timezone: None` → UTC display (matches Phase 2 bucket default).
- **Filter scope:** Loaded entries only; no server query protocol.
- **Filter UI:** Imperative-first (controlled prop + `set_filter`); no required default chrome.
- **Filter empty copy:** `locale.no_matches`, distinct from `locale.empty`.
- **Sort:** `CLIENT_SORT` default **off**; Client only; Server `set_sort` is a no-op.
- **Pipeline order:** source list → sort → filter → date dividers.
- **Paged:** Server-primary; Client ignores `Paged` and renders all.
- **Page index:** `go_to_page` is **0-based** (datatable); Pagination UI is **1-based** (core `Pagination`).
- **Load-until-found:** Separate `scroll_to_entry_or_load`; Phase 2 `scroll_to_entry` unchanged.
- **Hunt limits:** Default `max_scroll_load_pages = 20`; latest-wins cancellation; InfiniteScroll only.
- **Out of Phase 3:** server search/sort protocols, transport, rich-text, Client page windows.

When uncertain during implementation, choose the option that keeps the crate extractable and host-agnostic, and prefer the smaller API (imperative filter over built-in chrome, no-op Server sort over half-implemented fetcher hints).
