# orbital-history design — Phase 2

Implementation-ready charter for work **deferred from Phase 1**. Phase 1 ([DESIGN.md](DESIGN.md)) covers PR1–PR9: types, leaf components, `HistoryTimeline`, server paging, renderers, density, locale, date-bucket dividers, and the multi-field change card.

Phase 2 stacks on that public API. Do not start Phase 2 PRs until Phase 1 is mergeable (or land behind the same feature branch with Phase 1 complete first). Implementers keep the short [README](README.md) charter in sync (move items from Deferred into Scope as they ship).

Deep API guidance lives here and, once implemented, in rustdoc / `#[component_doc]` catalog pages.

---

## 1. Summary

### Phase 1 dependency

Phase 2 assumes:

- `HistoryEntry`, `HistorySource::{Client, Server}`, `HistoryTimeline`, slots, renderers, locale, and `HistoryFeatures::DATE_DIVIDERS`
- Loading phases: full skeleton for **initial** load only; footer spinner for **incremental** pagination / refetch
- Date buckets computed from UTC calendar days (`history_date_bucket` / `with_date_dividers`)
- Host pre-sorts newest-first; timeline does not re-sort

### Goals

Ship the four capabilities deferred from Phase 1 “Optional later”:

- **Imperative handle** — programmatic `scroll_to_entry` and `refresh` via `HistoryHandle` (datatable `on_handle` pattern)
- **Dialog shell helper** — thin, fully generic composition of core `Dialog` + `HistoryTimeline`, or a documented host pattern if a component adds no value
- **Live-update surface** — host-facing refresh / prepend guidance so new entries appear without remounting; **no** transport or protocol design
- **Display-timezone date buckets** — relative buckets (Today / Yesterday / …) using host wall-clock timezone, defaulting to UTC for back-compat

### Non-goals

- Any backend, database, privacy, or persistence design
- Realtime transport (WebSocket, SSE, polling loops, auth, conflict resolution, CRDT)
- Classic page-number `Pagination` bar or “go to page N”
- `HistoryFeatures::CLIENT_SORT` / client-side re-sorting
- Full rich-text / markdown editing
- Product-app or non-Orbital platform dependencies
- Redesign of Phase 1 types, slots, or loading-phase rules

### Success criteria

- A host can capture `HistoryHandle` once on mount and call `scroll_to_entry` / `refresh` without reaching into DOM internals
- `scroll_to_entry` is a no-op when the entry id is not in the currently loaded list (no automatic page-until-found)
- Server `refresh` resets accumulated pages and re-fetches; loading chrome follows Phase 1 rules (incremental spinner when entries are already visible)
- Client hosts prepend new entries by mutating their `RwSignal`; Server hosts use `refresh` (and optional live-head merge only if refresh alone is insufficient)
- Date-bucket dividers respect an optional display timezone; omitting it preserves Phase 1 UTC behavior
- Dialog embed is either a thin public helper or a catalog-documented composition — never product-specific chrome
- Each delivery PR is independently demoable in the Orbital component preview app when it adds UI

### Pattern alignment

| Pattern | Reference |
| --- | --- |
| `HistoryHandle` + `HistoryEvents::on_handle` | `DataTableHandle` / `DataTableEvents::on_handle` |
| Scroll-into-view by data attribute | `orbital-discussion` `scroll_reply_into_view` |
| Display timezone for calendar-day math | `OrbitalDateTime` + `DatetimeTimezone` (base-components / date-pickers / scheduler) |
| Dialog composition | `orbital_core_components::{Dialog, DialogSurface, DialogBody, DialogTitle, DialogContent}` |

---

## 2. Architecture deltas

Phase 2 extends the Phase 1 crate layout only where needed:

```
orbital-history/
  DESIGN.md
  DESIGN-PHASE-2.md         # this document
  README.md
  src/
    types/
      handle.rs             # HistoryHandle (PR10)
      events.rs             # + on_handle (PR10)
      source.rs             # unchanged contract; refresh wires Server path (PR11)
    format/
      mod.rs                # timezone-aware bucket helpers (PR12)
    products/
      history/
        timeline.rs         # handle wiring, display_timezone prop, optional live_head
        list.rs             # data-history-entry-id on rows; scroll target
        dialog.rs           # HistoryDialog (PR13, only if kept)
        scroll.rs           # hydrate scroll-into-view helper (PR10)
        docs/               # new / updated catalog pages
    preview/
      fixtures.rs           # handle, timezone boundary, live-head fixtures
      static_registrations.rs
```

No separate protocol or networking module. Live-update is handle + host signals only.

---

## 3. Feature designs

### 3.1 Imperative handle

Align with `DataTableHandle`: a cloneable struct of `Callback`s delivered once on mount via `HistoryEvents::on_handle`.

```rust
use leptos::prelude::*;

/// Imperative handle for programmatic HistoryTimeline actions.
#[derive(Clone)]
pub struct HistoryHandle {
    /// Scroll the timeline scroll region so the entry with the given `id` is visible.
    /// No-op when the id is not present in the currently rendered list.
    pub scroll_to_entry: Callback<(String,), ()>,
    /// Re-fetch server pages (Server source). Client source: no-op; host mutates the signal.
    pub refresh: Callback<(), ()>,
    /// Scroll the list to the top (newest entries). Always available.
    pub scroll_to_top: Callback<(), ()>,
}
```

#### Delivery

```rust
#[derive(Clone, Default)]
pub struct HistoryEvents {
    pub on_actor_click: Option<Callback<HistoryActor, ()>>,
    pub on_entry_click: Option<Callback<HistoryEntry, ()>>,
    pub on_load_error: Option<Callback<ServerFnError, ()>>,
    /// Receives imperative [`HistoryHandle`] callbacks once on mount.
    pub on_handle: Option<Callback<HistoryHandle, ()>>,
}
```

`HistoryTimeline` builds the handle from internal scroll / paging state and calls `events.notify_handle(handle)` once after mount (same timing as datatable).

#### `scroll_to_entry`

1. Each rendered entry row sets `data-history-entry-id="{entry.id}"` on the row root (or a stable inner element).
2. On hydrate, `scroll_to_entry` queries `[data-history-entry-id="…"]` inside the timeline scroll container and calls `scroll_into_view` (discussion pattern: schedule after paint via double `request_animation_frame` when the row may have just mounted).
3. If no element matches, **no-op**. Do **not** page-until-found, do not error, do not toast.
4. SSR / non-hydrate: callbacks are present but scroll helpers are no-ops.

Rationale for no-op on miss: automatic load-until-found couples the handle to paging offsets, error recovery, and unbounded fetch loops. Hosts that need a deep link into older history should fetch that page themselves (or pass a Client list that already contains the entry), then call `scroll_to_entry`.

#### `scroll_to_top`

Sets the scroll container’s `scrollTop` to `0` (newest-first list). Useful after `refresh` or when the host prepends live entries and wants the user to see the head.

#### `refresh`

| Source | Behavior |
| --- | --- |
| `HistorySource::Server` | Reset accumulated pages and re-invoke the page fetcher from the first page (same as a full list reload). Preserve Phase 1 loading rules: if entries were already visible / `ever_loaded`, show **incremental** footer spinner — **not** the full timeline skeleton. If the list was empty and never loaded, initial skeleton is allowed. |
| `HistorySource::Client` | **No-op.** Host owns the signal; document that hosts should mutate `RwSignal<Vec<HistoryEntry>>` (replace or prepend) and optionally call `scroll_to_top`. |

Expose refresh internals only through the handle (and any paging-hook reset the timeline already owns). Do not add a public “go to page N” API.

#### Usage sketch

```rust,ignore
let handle = RwSignal::new(None::<HistoryHandle>);
view! {
    <Button on:click=move |_| {
        if let Some(h) = handle.get() {
            h.scroll_to_entry.run(("entry-42".into(),));
        }
    }>
        "Jump to entry"
    </Button>
    <HistoryTimeline
        data_source=source
        events=HistoryEvents {
            on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
            ..Default::default()
        }
    />
}
```

---

### 3.2 Dialog shell helper

Phase 1 documents dialog embed as `max_height` + internal `ScrollArea` only (“No dialog shell helper in v1”).

Phase 2 ships a helper **only if it stays fully generic**: open state owned by the host, title from locale (or an optional override), default `max_height`, no product routes or domain types.

#### Proposed API

```rust,ignore
#[component]
pub fn HistoryDialog(
    /// Host-owned open binding (same pattern as core `Dialog`).
    open: OpenBind,
    data_source: HistorySource,
    #[prop(optional)]
    title: Option<String>,
    /// Default `"360px"` — matches Phase 1 dialog guidance.
    #[prop(optional, default = "360px".to_string())]
    max_height: String,
    #[prop(optional, default = HistoryOrientation::Vertical)]
    orientation: HistoryOrientation,
    #[prop(optional, default = HistoryFeatures::default_enabled())]
    features: HistoryFeatures,
    #[prop(optional)]
    locale: Option<HistoryLocale>,
    #[prop(optional)]
    events: HistoryEvents,
    #[prop(optional)]
    renderers: Option<HistoryRenderers>,
    #[prop(optional)]
    display_timezone: Option<Signal<DatetimeTimezone>>,
    // pass-through slots as needed for timeline chrome
) -> impl IntoView
```

Composition (conceptual):

```text
Dialog (open)
  DialogSurface
    DialogBody
      DialogTitle  ← title.unwrap_or(locale.title)
      DialogContent
        HistoryTimeline (max_height, data_source, …)
```

Dismiss / backdrop / focus trap remain core `Dialog` behavior. `HistoryDialog` does not invent a trigger; the host opens via `open`.

#### Ship-or-document gate (PR13)

If implementation review finds the helper is only a few lines of composition with no shared defaults worth owning:

1. **Do not** ship `HistoryDialog`
2. Document the host pattern in rustdoc and the `history-embed` catalog page (Phase 1 already lists this slug)
3. Keep `max_height` guidance as the supported embed API

Prefer shipping the helper when it centralizes default `max_height`, title resolution, and a stable `data-testid` / class root (`orbital-history-dialog`) used by previews and e2e.

---

### 3.3 Live-update surface

#### Non-goals (reiterated)

No WebSocket, SSE, polling, auth, conflict resolution, or CRDT. Orbital does not subscribe to anything.

#### Minimal API (preferred)

| Host setup | How new entries appear |
| --- | --- |
| **Client** | Host prepends (or replaces) items on `RwSignal<Vec<HistoryEntry>>`. Timeline already reacts. Optional `scroll_to_top` via handle. |
| **Server** | Host calls `HistoryHandle::refresh` after its own subscription / poll / mutation succeeds. Timeline re-fetches page 0 and rebuilds the list. |

Document Client prepend in rustdoc and a catalog page. Do **not** add `HistoryHandle::prepend` — the signal is the source of truth.

#### Optional live-head merge (PR14, only if needed)

If product hosts need **Server** infinite scroll **and** must show brand-new events at the head **without** discarding already-loaded older pages, add an optional prop:

```rust
/// Newest entries supplied by the host (e.g. from a live channel).
/// Merged above the paged Server list; host keeps newest-first and de-dupes by `id`.
#[prop(optional)]
live_head: Option<Signal<Vec<HistoryEntry>>>,
```

Merge rules:

1. Render `live_head` entries first (newest-first), then accumulated Server pages.
2. De-dupe by `HistoryEntry.id`: if an id appears in both, prefer `live_head` and omit the paged duplicate.
3. Date dividers run on the **merged** list.
4. `scroll_to_entry` searches the merged DOM.
5. `refresh` still resets Server pages; it does **not** clear `live_head` (host owns that signal).

Do **not** implement PR14 until PR11 `refresh` is proven insufficient for a real host. Prefer refresh-only for most apps.

#### Events

No new network events. Optional analytics-only callback is out of scope unless a host requests it during implementation; default is none.

---

### 3.4 Display-timezone-aware date buckets

Phase 1 uses UTC calendar days derived from `DateTime<Utc>` for `HistoryDateBucket` (Today / Yesterday / Last7Days / Last30Days / Older).

Phase 2 adds an optional display timezone so “Today” matches the user’s wall clock.

#### Prop

On `HistoryTimeline` (and `HistoryDialog` if shipped):

```rust
/// Wall-clock timezone for date-bucket boundaries.
/// When `None`, buckets use UTC (Phase 1 behavior).
#[prop(optional)]
display_timezone: Option<Signal<DatetimeTimezone>>,
```

`DatetimeTimezone` comes from `orbital_base_components` (`Local`, `Utc`, `FixedOffset`).

#### Format helpers

Extend pure helpers without breaking Phase 1 call sites:

```rust
pub fn history_date_bucket(
    changed_at: DateTime<Utc>,
    now: DateTime<Utc>,
) -> HistoryDateBucket;

pub fn history_date_bucket_in_tz(
    changed_at: DateTime<Utc>,
    now: DateTime<Utc>,
    tz: DatetimeTimezone,
) -> HistoryDateBucket;

pub fn with_date_dividers(
    entries: &[HistoryEntry],
    now: DateTime<Utc>,
) -> Vec<HistoryListItem>;

pub fn with_date_dividers_in_tz(
    entries: &[HistoryEntry],
    now: DateTime<Utc>,
    tz: DatetimeTimezone,
) -> Vec<HistoryListItem>;
```

UTC helpers remain the default implementation path (`history_date_bucket` ≡ `history_date_bucket_in_tz(..., DatetimeTimezone::Utc)` or shared internal).

Bucket **rules** are unchanged from Phase 1; only the calendar-day projection changes:

1. Map `changed_at` and `now` to `OrbitalDateTime::from_instant(..., tz)`
2. Compare calendar days via existing `start_of_day` / same-day helpers
3. Apply Today / Yesterday / Last7Days / Last30Days / Older thresholds in that timezone

`HistoryEntry.changed_at` stays `DateTime<Utc>` on the wire (paging serde unchanged).

#### Tests

Unit-test timezone boundary cases, for example:

- Instant that is “evening UTC” but still “today” in `America/Los_Angeles`-equivalent fixed offset (use `FixedOffset` in tests for determinism)
- Instant that is “today” UTC but “yesterday” in a positive offset
- Divider transitions on a mixed list spanning bucket boundaries in Local vs Utc

---

## 4. API surface (Phase 2 delta)

### Types

| Item | Module | PR |
| --- | --- | --- |
| `HistoryHandle` | `types/handle.rs` | PR10 |
| `HistoryEvents::on_handle` | `types/events.rs` | PR10 |

### `HistoryTimeline` props added

| Prop | Type | Default | PR |
| --- | --- | --- | --- |
| `display_timezone` | `Option<Signal<DatetimeTimezone>>` | `None` (UTC buckets) | PR12 |
| `live_head` | `Option<Signal<Vec<HistoryEntry>>>` | `None` | PR14 (optional) |

### Components

| Component | Role | PR |
| --- | --- | --- |
| `HistoryDialog` | Generic dialog shell around `HistoryTimeline` | PR13 (or document-only) |

### Format

| Function | PR |
| --- | --- |
| `history_date_bucket_in_tz` | PR12 |
| `with_date_dividers_in_tz` | PR12 |

### DOM / CSS

- Row attribute: `data-history-entry-id`
- Optional dialog root: `orbital-history-dialog` / `[data-orbital-history-dialog]` if `HistoryDialog` ships
- No new density tokens required

### Usage sketches

#### Handle + refresh (Server)

```rust,ignore
let handle = RwSignal::new(None::<HistoryHandle>);
// host subscription callback:
// if let Some(h) = handle.get_untracked() { h.refresh.run(()); }
view! {
    <HistoryTimeline
        data_source=HistorySource::Server { fetcher, page_size: 20 }
        events=HistoryEvents {
            on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
            ..Default::default()
        }
    />
}
```

#### Client live prepend

```rust,ignore
let entries = RwSignal::new(vec![/* existing */]);
// on live event:
entries.update(|v| {
    v.insert(0, new_entry);
});
```

#### Display timezone

```rust,ignore
view! {
    <HistoryTimeline
        data_source=source
        display_timezone=Signal::from(DatetimeTimezone::Local)
    />
}
```

#### Dialog (if shipped)

```rust,ignore
let open = RwSignal::new(false);
view! {
    <Button on:click=move |_| open.set(true)>"History"</Button>
    <HistoryDialog open=open.into() data_source=source />
}
```

---

## 5. Layout, a11y, and loading (delta only)

### Scroll targets

- Entry rows remain list items; `data-history-entry-id` is an attribute only (no extra interactive control).
- Programmatic scroll must not steal focus unless the host also moves focus; default is scroll-only (match discussion / datatable).

### Refresh loading

Unchanged Phase 1 rules:

| Situation | UI |
| --- | --- |
| Refresh while entries visible | Incremental footer (`HistoryLoadingMoreView` / spinner) |
| Refresh / first load with empty list and never loaded | Initial skeleton |
| Refresh error | `HistoryErrorView` / `on_load_error` |

**Never** replace a populated list with the full skeleton on refresh.

### Live head

If PR14 ships, live-head rows use the same entry chrome and a11y as paged rows. No separate “live” badge in Orbital defaults (hosts may use custom `kind` renderers).

### Dialog

If `HistoryDialog` ships: title is a dialog heading; timeline scroll region stays inside `DialogContent` with `max_height` so focus trap and scroll do not fight the page.

### Timezone

Visible timestamps may continue to use existing locale compact / relative formatters. Phase 2 only changes **bucket boundaries**, not necessarily absolute timestamp display. If compact time should also follow `display_timezone`, prefer reusing base-components format helpers in the same PR only when trivial; otherwise leave timestamp display as Phase 1 and document follow-up.

---

## 6. Preview & docs plan

### Catalog pages (add / extend)

| Slug | Focus | PR |
| --- | --- | --- |
| `history-handle` | `on_handle`, scroll-to-entry, scroll-to-top | PR10 |
| `history-refresh` | Server refresh + incremental loading | PR11 |
| `history-timezone-buckets` | UTC vs Local / FixedOffset dividers | PR12 |
| `history-embed` | Dialog `max_height` / `HistoryDialog` vs flex card | PR13 |
| `history-live-update` | Client prepend + Server refresh (and live_head if PR14) | PR11 minimal; PR14 if merge ships |

### Fixtures

- Client list with stable ids for scroll targets (including off-screen entries)
- Mock Server fetcher that can reset on refresh and optionally delay for loading-more
- Entries whose UTC day differs from a fixed-offset “local” day
- Optional live_head overlapping paged ids for de-dupe demo

### Registration

Same Phase 1 pattern: `#[component_doc]`, `preview/static_registrations.rs` → `all()`, preview-app collect loop.

---

## 7. Phased delivery

Every phase lists scope, public API added, previews/tests, and explicit out-of-scope. Prefer small, mergeable PRs. Stack on `feat/history` (or equivalent) after Phase 1, as directed by maintainers.

### PR10 — `HistoryHandle` + scroll

| | |
| --- | --- |
| **Scope** | `HistoryHandle` with `scroll_to_entry` and `scroll_to_top`; `HistoryEvents::on_handle`; `data-history-entry-id` on rows; hydrate scroll helper |
| **Public API** | `HistoryHandle`, `on_handle` |
| **Previews / tests** | `history-handle` preview; unit/DOM tests for miss → no-op where practical |
| **Out of scope** | `refresh`, timezone, dialog, live_head |

`refresh` may exist on the struct as a no-op stub in PR10 only if it simplifies typing; prefer adding the real Server implementation in PR11 and documenting Client no-op there.

### PR11 — Server `refresh`

| | |
| --- | --- |
| **Scope** | `HistoryHandle::refresh` for `HistorySource::Server` (reset pages, re-fetch); Client no-op + rustdoc; loading-phase correctness on refresh |
| **Public API** | Full `refresh` behavior |
| **Previews / tests** | `history-refresh`; minimal `history-live-update` (Client prepend + Server refresh buttons) |
| **Out of scope** | `live_head` merge, timezone, dialog |

### PR12 — Display-timezone date buckets

| | |
| --- | --- |
| **Scope** | `display_timezone` prop; `history_date_bucket_in_tz` / `with_date_dividers_in_tz`; wire dividers when `DATE_DIVIDERS` enabled |
| **Public API** | Prop + `*_in_tz` helpers |
| **Previews / tests** | `history-timezone-buckets`; unit tests for TZ boundaries |
| **Out of scope** | Dialog, live_head, changing wire type of `changed_at` |

### PR13 — Dialog shell or embed docs

| | |
| --- | --- |
| **Scope** | Ship `HistoryDialog` **or** document host composition only; polish `history-embed` catalog page |
| **Public API** | `HistoryDialog` if shipped; otherwise docs-only |
| **Previews / tests** | Embed preview (dialog open/close) |
| **Out of scope** | Product-specific actions, live_head |

Apply the ship-or-document gate in §3.2.

### PR14 — Live-head merge (optional)

| | |
| --- | --- |
| **Scope** | `live_head` prop; merge + de-dupe by id; dividers on merged list |
| **Public API** | `live_head` |
| **Previews / tests** | `history-live-update` extended |
| **Out of scope** | Transport, auto-refresh timers inside Orbital |

**Skip PR14** if PR11 refresh satisfies hosts.

---

## 8. Candidates (not in Phase 2)

Phase 3 charter: [DESIGN-PHASE-3.md](DESIGN-PHASE-3.md).

Documented so implementers do not silently expand scope:

- Classic page-number pagination bar / “go to page N”
- `HistoryFeatures::CLIENT_SORT`
- Load-until-found for `scroll_to_entry`
- Timestamp display forced through `display_timezone` (bucket-only unless trivial)
- Imperative filter / search on the timeline

---

## 9. Open questions

None remaining for implementers.

### Resolved decisions

- **Handle delivery:** `HistoryEvents::on_handle` once on mount (datatable pattern), not a render prop or context-only API.
- **Scroll miss:** **No-op** when `entry.id` is not in the loaded DOM; no page-until-found.
- **Client `refresh`:** **No-op**; host mutates the client signal.
- **Server `refresh` loading:** Incremental footer when entries already visible; never full skeleton on populated list.
- **Live-update minimalism:** Client prepend + Server `refresh` first; `live_head` only in PR14 if required.
- **No `HistoryHandle::prepend`:** Signal remains source of truth for Client.
- **Dialog:** Ship only if fully generic and worth a stable root; otherwise document embed pattern.
- **Timezone default:** `display_timezone: None` → UTC buckets (Phase 1 back-compat).
- **Wire time type:** `changed_at` remains `DateTime<Utc>`; timezone affects bucket projection only.
- **Out of Phase 2:** page-number bar, `CLIENT_SORT`, transport protocols.

When uncertain during implementation, choose the option that keeps the crate extractable and host-agnostic, and prefer the smaller API (refresh over live-head, docs over a thin dialog wrapper when equivalent).
