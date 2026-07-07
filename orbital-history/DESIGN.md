# orbital-history design

Implementation-ready design for the Orbital **history / audit timeline** feature crate. Host applications own data fetch and domain models; Orbital owns presentation, formatting defaults, layout, slots, and render extensibility.

This document is the charter for a multi-PR delivery. Implementers also keep a short [README](README.md) charter (Scope / Deferred) matching other feature crates. Deep API guidance lives here and, once implemented, in rustdoc / `#[component_doc]` catalog pages.

---

## 1. Summary

### Goals

Ship a product-grade history surface comparable in API quality to `orbital-datatable`, `orbital-scheduler`, and `orbital-discussion`:

- Backend-agnostic typed entries (`HistoryEntry`, actor, change payload)
- Dual data sources: client signal and server page fetcher
- Feature flags, slot chrome, renderer fallthrough, locale, tokenized layout
- Public leaf components and preview catalog registration
- Excellent default field-diff formatting and a first-class custom-`kind` path

Apps need to show **what changed, when, and by whom** on a detail page, in a side panel, or in a dialog. Most rows are field diffs; some are lifecycle events (created / deleted); some need fully custom UI keyed by a host-defined `kind` string.

### Non-goals

- Any backend, database, privacy, or persistence design
- Any dependency on product apps or non-Orbital platform crates
- Realtime / live-update protocol design (a future hook may be noted only)
- Full rich-text / markdown editing (history is read-oriented)
- Client-side re-sorting of entries (hosts pre-sort newest-first)

### Success criteria

- A host can render a scrollable audit timeline from either an in-memory signal or a `PageFetcher` without importing app-specific types into Orbital
- Default field-diff, created, and deleted rows look product-ready with locale-overridable templates
- Custom per-entry / per-`kind` / change-line renderers fall through to defaults when they return `None`
- Natural (default) and Compact entry layouts are both fully implemented
- Empty, loading (initial skeleton vs incremental spinner), error, and end-of-list states have defaults and override slots
- Relative date-bucket dividers (not per-day) are available and default-on
- Each delivery PR is independently demoable in the Orbital component preview app when it adds UI

### Pattern alignment

| Pattern | Reference |
| --- | --- |
| `HistorySource::{Client, Server}` + page fetcher + `orbital_paging` | `orbital-datatable` |
| Orientation + render contexts + fallthrough `Option<AnyView>` | `orbital-scheduler` |
| Slots, locale, features, leaf components, `orbital-*__*` CSS, density | `orbital-discussion` |
| Cargo `preview` / `hydrate` / `ssr`, `static_registrations::all()` | all feature crates + `orbital-preview-app` |

---

## 2. Architecture

### Crate layout

```
orbital-history/
  DESIGN.md                 # this document (PR1)
  README.md
  Cargo.toml
  src/
    lib.rs
    types/
      mod.rs
      entry.rs              # HistoryEntry, HistoryActor, HistoryChange, HistoryFieldDiff
      source.rs             # HistorySource, HistoryPageFetcher, HistoryPagingMode
      features.rs           # HistoryFeatures bitflags
      locale.rs             # HistoryLocale + format helpers on locale
      slots.rs              # #[slot] chrome + renderer slots
      renderers.rs          # HistoryRenderers, HistoryRenderContext
      events.rs             # HistoryEvents
      layout.rs             # HistoryLayout
      date_bucket.rs        # HistoryDateBucket, HistoryListItem (PR8)
    format/
      mod.rs                # format_change, truncate_display_value, history_date_bucket
    products/
      mod.rs
      history/
        mod.rs
        styles.rs           # inject_style("orbital-history", …)
        timeline.rs         # HistoryTimeline root
        list.rs             # entry list + infinite scroll wiring
        entry_row.rs        # HistoryEntryRow (vertical / horizontal)
        actor.rs            # HistoryActorLabel
        timestamp.rs        # HistoryTimestamp
        change_line.rs      # HistoryChangeLine
        change_card.rs      # multi-field card (PR9)
        date_divider.rs     # HistoryDateDivider (PR8)
        overlays.rs         # empty / loading / loading-more / error / end defaults
        skeleton.rs         # HistoryTimelineSkeleton, HistoryEntryRowSkeleton (PR5)
        header.rs           # default title header
        docs/               # component_doc catalog pages
    preview/
      mod.rs
      fixtures.rs
      static_registrations.rs
```

Optional thin layers (add when needed, matching discussion/datatable):

- `context/` — provider for locale, features, renderers, events (when leaves need shared context)
- `engine/` — pure list projection (`with_date_dividers`) if `format/` grows too crowded

`lib.rs` pattern: private modules, blanket `pub use` of `types` and `products::history`, `preview` behind `#[cfg(feature = "preview")]`.

### Dependencies

| Crate | Role |
| --- | --- |
| `leptos` | Components, signals |
| `serde`, `chrono` | Wire types (`HistoryEntry` on `Page<T>`) |
| `bitflags` | `HistoryFeatures` |
| `orbital-base-components` | `format_unix` / datetime display helpers |
| `orbital-core-components` (`default-features = false`) | Flex, typography presets, MessageBar, ScrollArea, Link, Spinner, Skeleton, SkeletonItem, Divider |
| `orbital-macros` | `#[component_doc]`, slots |
| `orbital-style` | `inject_style` |
| `orbital-theme` | Density via `use_theme_options` |
| `orbital-paging` (`features = ["leptos"]`) | `Page`, `PageRequest`, `use_paged_infinite_scroll` |
| `inventory` (optional) | Preview registration |

Do **not** depend on product apps, Valence, or non-Orbital platform crates.

### Cargo features

Mirror discussion / datatable:

```toml
[features]
default = ["preview"]
preview = ["dep:inventory", "orbital-core-components/preview"]
hydrate = [
  "leptos/hydrate",
  "orbital-core-components/hydrate",
  "orbital-theme/hydrate",
  "orbital-paging/hydrate",
  "preview",
  "dep:inventory",
  # wasm deps as needed
]
ssr = [
  "leptos/ssr",
  "orbital-core-components/ssr",
  "orbital-theme/ssr",
  "orbital-paging/ssr",
  "preview",
  "dep:inventory",
]
```

Production consumers use `default-features = false` and enable only `hydrate` or `ssr` as needed. Enable `preview` only on the doc host.

### Workspace touchpoints (PR1)

1. Root `Cargo.toml` — add `"orbital-history"` to `members`
2. `orbital-primitives` — path dep, `pub use orbital_history::*`, hydrate/ssr/preview feature wiring
3. `orbital-preview-app` — path dep, `registry.rs` collect loop for `orbital_history::preview::static_registrations::all()`, feature flags
4. Root `README.md` — feature-crate list / table entry
5. `CONTRIBUTING.md` / CI verify list — `cargo check -p orbital-history --no-default-features`

---

## 3. Data model

### Core types

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    /// Host-defined kind string. Suggested defaults for built-in changes:
    /// `"field_diff"`, `"created"`, `"deleted"`, `"custom"`.
    pub kind: String,
    pub changed_at: DateTime<Utc>,
    pub actor: HistoryActor,
    pub change: HistoryChange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryActor {
    System,
    User {
        id: String,
        display_name: String,
        /// Host-provided route or URL. Orbital never invents app routes.
        href: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryFieldDiff {
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryChange {
    Created,
    Deleted { label: String },
    FieldDiff {
        field: String,
        old_value: String,
        new_value: String,
    },
    /// Multi-field card variant (data + formatters in PR2; default UI in PR9).
    FieldDiffs { fields: Vec<HistoryFieldDiff> },
    /// Plain summary when no structured change applies; often paired with a custom renderer.
    Custom { summary: String },
}
```

### Time type choice

`changed_at` is **`chrono::DateTime<Utc>`**, not `OrbitalDateTime`.

Rationale: `orbital_paging::use_paged_infinite_scroll` requires `Serialize + DeserializeOwned` on page items. `OrbitalDateTime` has no serde today. Timestamp display uses `orbital_base_components::format_unix` (and locale templates) — the same display stack as date-pickers. Hosts that hold `OrbitalDateTime` map with `.instant()` when building `HistoryEntry`.

### Sort contract

The timeline **does not re-sort**. Newest-first is the host contract:

- Client: host keeps `RwSignal<Vec<HistoryEntry>>` newest-first
- Server: each page and the accumulated list are newest-first; offset paging advances into older entries

Optional future: `HistoryFeatures::CLIENT_SORT` — out of scope for v1.

### Serialization / SSR

`HistoryEntry` and nested enums derive `Serialize` / `Deserialize` so they can cross Leptos server-function boundaries as `Page<HistoryEntry>`. Hosts may also use a private DTO on the wire and map to `HistoryEntry` inside the `HistoryPageFetcher` closure; Orbital does not hydrate domain models beyond these types.

### Data source

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

pub type HistoryPageFetcher = Arc<
    dyn Fn(
            PageRequest,
        ) -> Pin<Box<dyn Future<Output = Result<Page<HistoryEntry>, ServerFnError>> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub enum HistorySource {
    /// In-memory list; host owns updates.
    Client(RwSignal<Vec<HistoryEntry>>),
    /// Server-driven pages via `orbital_paging`.
    Server {
        fetcher: HistoryPageFetcher,
        page_size: u32,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryPagingMode {
    /// Client: render all. Server: infinite scroll (default for Server).
    #[default]
    InfiniteScroll,
    /// Client: render all. Server: single first-page fetch only.
    None,
}
```

Helpers (datatable-style): `HistorySource::client_rw`, `is_server`, `server_page_size`, `server_fetcher`, `client_items`.

### Pagination API (summary)

Paging is **infinite-scroll oriented**, not a page-number footer like `DataTable`’s `PagingMode::Paged`.

| Piece | Role |
| --- | --- |
| `HistorySource::Server { fetcher, page_size }` | Server-driven list |
| `HistoryPageFetcher` | `PageRequest -> Result<Page<HistoryEntry>, ServerFnError>` |
| `orbital_paging::{Page, PageRequest, use_paged_infinite_scroll}` | Shared wire types + scroll hook |
| `HistoryPagingMode::InfiniteScroll` | Load more near bottom (default for Server) |
| `HistoryPagingMode::None` | Client: all rows; Server: first page only |
| `HistoryEvents::on_load_error` | Fetch failures |
| `HistoryEndView` / `locale.end_of_list` | End-of-list chrome |

**Not in v1:** classic `Pagination` bar, client-side re-sort, or a public imperative “go to page N” handle (optional-later only). Hosts implement the fetcher (typically a server function) and return `Page { items, has_more, total_count, next_request_offset }`. Newest-first is the host contract (see sort rules above).

### Loading phases

Loading chrome depends on whether any entries are already visible. **Never** show the full timeline skeleton during pagination or refetch when data is already on screen.

| Phase | When | Default UI |
| --- | --- | --- |
| **Initial** | A load is in flight and no entries are shown yet (`!ever_loaded` for Server, or host `loading` while the list is empty) | **Full timeline skeleton** — placeholder rows with `Skeleton` / `SkeletonItem` for timestamp, actor, and change line, matching `orientation` |
| **Incremental** | Entries are already visible and more data is loading (scroll page or host refetch) | Footer / end-area **Spinner** (or slim loading row) — **not** the full skeleton |
| **Error** | Fetch failed | `HistoryErrorView` / `MessageBar` |

Resolution of the loading flag:

1. If `loading: Option<Signal<bool>>` is `Some(signal)`: treat as loading when `signal.get()` is true.
2. Else if `HistorySource::Server`: derive from `use_paged_infinite_scroll` (`loading` + `ever_loaded`).
3. Else (`Client`, no prop): never show loading chrome.

Phase selection:

- **Initial:** loading && entries are empty && (Server: `!ever_loaded`, or host-controlled with empty list)
- **Incremental:** loading && (entries are non-empty or Server `ever_loaded`)

See [§4 API surface](#4-api-surface) for props, slots, and leaf skeleton components (PR5).

### Date buckets (PR8)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HistoryDateBucket {
    Today,
    Yesterday,
    Last7Days,
    Last30Days,
    Older,
}

pub enum HistoryListItem {
    Divider(HistoryDateBucket),
    Entry(HistoryEntry),
}
```

Bucket rules and list projection are specified in [§6 Formatting rules](#6-formatting-rules) and [§5 Layout](#5-layout--visual-design).

---

## 4. API surface

### Layout

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HistoryLayout {
    /// Timeline spine with stacked timestamp, actor, and change (default).
    #[default]
    Natural,
    /// Dense inline sentence: actor + change + time on one line.
    Compact,
}
```

### Features

```rust
bitflags::bitflags! {
    /// Opt-in / opt-out capabilities (runtime; not Cargo features).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct HistoryFeatures: u32 {
        /// Render user actors as links when `href` is present.
        const ACTOR_LINKS = 1 << 0;
        /// Insert relative date-bucket dividers (Today / Yesterday / Last 7 days / …).
        const DATE_DIVIDERS = 1 << 1;
        /// Make the entry row activatable and fire `HistoryEvents::on_entry_click`.
        const ENTRY_CLICK = 1 << 2;
    }
}

impl HistoryFeatures {
    pub fn default_enabled() -> Self {
        Self::ACTOR_LINKS | Self::DATE_DIVIDERS
    }
}

impl Default for HistoryFeatures {
    fn default() -> Self {
        Self::default_enabled()
    }
}
```

Hosts disable dividers or links with `features.remove(...)` or an explicit bitset.

### Locale

All user-visible strings and format templates live on `HistoryLocale`. Template placeholders: `{field}`, `{old}`, `{new}`, `{label}`, `{n}`.

```rust
pub struct HistoryLocale {
    pub title: String,
    pub system_actor: String,
    pub empty: String,
    /// Accessible label for the initial-load skeleton region.
    pub loading: String,
    /// Footer label while loading additional pages (incremental).
    pub loading_more: String,
    pub error: String,
    pub end_of_list: String,
    pub created_template: String,       // e.g. "created"
    pub deleted_template: String,       // e.g. "deleted \"{label}\""
    pub field_diff_template: String,    // e.g. "changed {field} from \"{old}\" to \"{new}\""
    pub field_diffs_header_template: String, // e.g. "changed {n} fields"
    pub actor_link_aria_template: String,    // e.g. "View profile for {name}"
    pub date_bucket_today: String,
    pub date_bucket_yesterday: String,
    pub date_bucket_last_7_days: String,
    pub date_bucket_last_30_days: String,
    pub date_bucket_older: String,
    pub relative_time: HistoryRelativeTimeLocale,
    // compact absolute time preferences as needed (12h vs 24h)
}

pub struct HistoryRelativeTimeLocale {
    pub just_now: String,
    pub minutes_ago: String, // "{n}m ago"
    pub hours_ago: String,
    pub days_ago: String,
    pub weeks_ago: String,
    pub months_ago: String,
    pub years_ago: String,
}
```

Presets: `HistoryLocale::english()`, `HistoryLocale::french()` (PR8). Helpers: `resolve_history_locale`, `locale_signal`.

Methods:

- `format_created(&self) -> String`
- `format_deleted(&self, label: &str) -> String`
- `format_field_diff(&self, field, old, new) -> String`
- `format_field_diffs_header(&self, n: usize) -> String`
- `format_relative_time(&self, at: DateTime<Utc>, now: DateTime<Utc>) -> String`
- `format_compact_time(&self, at: DateTime<Utc>) -> String` — absolute compact via `format_unix` (`Time12` / `Time24`)
- `date_bucket_label(&self, bucket: HistoryDateBucket) -> &str`

### Events

Interaction-only callbacks (no network inside the crate):

```rust
#[derive(Clone, Default)]
pub struct HistoryEvents {
    pub on_actor_click: Option<Callback<HistoryActor, ()>>,
    pub on_entry_click: Option<Callback<HistoryEntry, ()>>,
    pub on_load_error: Option<Callback<ServerFnError, ()>>,
    /// Receives imperative [`HistoryHandle`] callbacks once on mount.
    pub on_handle: Option<Callback<HistoryHandle, ()>>,
    /// Fired when a markdown citation ref anchor is activated.
    pub on_citation_click: Option<Callback<String, ()>>,
    /// Fired when a markdown mention ref anchor is activated (hover shows Persona card).
    pub on_mention_click: Option<Callback<String, ()>>,
}
```

Actor links use host-provided `href` via `orbital_core_components::Link` when `ACTOR_LINKS` is enabled. `on_actor_click` is an optional extra (e.g. analytics) and does not replace navigation when `href` is set.

### Renderers

Scheduler-style fallthrough (`None` → default) plus discussion-style slot merge:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct HistoryRenderContext {
    pub entry: HistoryEntry,
    pub layout: HistoryLayout,
    pub locale: HistoryLocale,
}

pub type HistoryEntryView =
    Arc<dyn Fn(HistoryRenderContext) -> Option<AnyView> + Send + Sync>;
pub type HistoryChangeView =
    Arc<dyn Fn(HistoryRenderContext) -> Option<AnyView> + Send + Sync>;

#[derive(Clone, Default)]
pub struct HistoryRenderers {
    /// Full row override. When it returns `Some`, change_view is not consulted.
    pub entry_view: Option<HistoryEntryView>,
    /// Change-line only; actor and timestamp chrome stay default.
    pub change_view: Option<HistoryChangeView>,
    /// Per-kind full-row overrides; consulted when `entry_view` returns `None`.
    pub kind_views: HashMap<String, HistoryEntryView>,
}
```

**Resolution order** for each entry:

1. `entry_view(ctx)` → if `Some(view)`, use it
2. Else `kind_views.get(&entry.kind)` → if present and returns `Some(view)`, use it
3. Else default `HistoryEntryRow` with:
   - `change_view(ctx)` if `Some(view)`
   - else default `HistoryChangeLine` / `HistoryChangeCard` (PR9 for `FieldDiffs`)

Hosts may load extra data by `entry.id` / `entry.kind` themselves. Orbital does not hydrate domain models.

Prefer Leptos slots over a deprecated `renderers` prop; if both exist, **slots win** (discussion `merge_with_slots` pattern).

### Slots

Structural chrome (`ChildrenFn`):

| Slot | Default when omitted |
| --- | --- |
| `HistoryHeader` | Title from `locale.title` (`Subtitle1`) |
| `HistoryEmptyView` | `MessageBar` + `locale.empty` |
| `HistoryLoadingView` | **Initial** load only: `HistoryTimelineSkeleton` (not a bare spinner) |
| `HistoryLoadingMoreView` | **Incremental** load only: small `Spinner` + `locale.loading_more` |
| `HistoryErrorView` | `MessageBar` (error intent) + `locale.error` |
| `HistoryEndView` | Caption / subtle text with `locale.end_of_list` |

`HistoryLoadingView` never applies during pagination or refetch when entries are already visible. `HistoryLoadingMoreView` never replaces the list with a full skeleton.

Renderer slots (`render: Arc<…>`):

| Slot | Maps to |
| --- | --- |
| `HistoryEntrySlot` | `entry_view` |
| `HistoryChangeSlot` | `change_view` |

Kind-specific renderers are registered via `HistoryRenderers.kind_views` (prop or builder), not individual slots, to avoid unbounded slot props.

Internal aggregator: `HistorySlots::from_slot_props(...)`, `HistoryRenderers::from_slots` / `merge_with_slots`.

### Root: `HistoryTimeline`

```rust
#[component]
pub fn HistoryTimeline(
    data_source: HistorySource,
    #[prop(optional, default = HistoryLayout::Natural)]
    layout: HistoryLayout,
    #[prop(optional, default = HistoryFeatures::default_enabled())]
    features: HistoryFeatures,
    #[prop(optional)]
    locale: Option<HistoryLocale>,
    /// e.g. Some("320px"). None = flex-fill (`min-height: 0`) in parent.
    #[prop(optional)]
    max_height: Option<String>,
    #[prop(optional, default = HistoryPagingMode::InfiniteScroll)]
    paging: HistoryPagingMode,
    /// Host override for loading. When `None`, Server derives from
    /// `use_paged_infinite_scroll` (`loading` + `ever_loaded`). Client is
    /// non-loading unless this is set.
    #[prop(optional)]
    loading: Option<Signal<bool>>,
    /// Placeholder rows in the initial skeleton (default 5).
    #[prop(optional, default = 5)]
    skeleton_row_count: u32,
    #[prop(optional)]
    events: HistoryEvents,
    #[prop(optional)]
    renderers: Option<HistoryRenderers>,
    #[prop(optional, into)]
    class: MaybeProp<String>,
    // slot props: history_header, history_empty_view, history_loading_view,
    // history_loading_more_view, history_error_view, history_end_view,
    // history_entry_slot, history_change_slot
) -> impl IntoView
```

Root responsibilities:

- `inject_style("orbital-history", history_styles())`
- Apply density modifier from `use_theme_options()`
- Resolve locale, slots, renderers
- Provide context for leaves
- Resolve loading phase (initial skeleton vs incremental footer) per [§3 Loading phases](#loading-phases)
- Render header + scroll region + list / overlays
- Wire `HistorySource::Server` to `use_paged_infinite_scroll` when `paging == InfiniteScroll`

### Public leaf components

| Component | Role |
| --- | --- |
| `HistoryEntryRow` | One entry; vertical or horizontal layout |
| `HistoryActorLabel` | System label or user name / link |
| `HistoryTimestamp` | Compact time (`Caption1`) |
| `HistoryChangeLine` | Single-line formatted change |
| `HistoryChangeCard` | Multi-field card (PR9) |
| `HistoryDateDivider` | Bucket section header (PR8) |
| `HistoryTimelineSkeleton` | Initial-load shell: `skeleton_row_count` fake rows (PR5) |
| `HistoryEntryRowSkeleton` | One placeholder row (timestamp / actor / change bars) |
| `HistoryDefaultHeader` | Default title chrome |
| Default overlays | Empty / initial loading / loading-more / error / end |

Skeleton leaves use `orbital_core_components::{Skeleton, SkeletonItem}` and mirror the active `orientation` (spine + bars for Vertical; time rail + bars for Horizontal). Classes: `orbital-history__skeleton`, `orbital-history__skeleton-row`, etc.

### Format module (pure, no I/O)

```rust
pub const DEFAULT_TRUNCATE_LEN: usize = 80;

pub fn format_change(change: &HistoryChange, locale: &HistoryLocale) -> String;
pub fn truncate_display_value(value: &str, max_len: usize) -> String;
pub fn history_date_bucket(changed_at: DateTime<Utc>, now: DateTime<Utc>) -> HistoryDateBucket;
pub fn with_date_dividers(entries: &[HistoryEntry], now: DateTime<Utc>) -> Vec<HistoryListItem>;
```

Unit-test templates, truncation, quotes, and bucket boundaries in PR2 / PR8.

### Usage sketches

#### 1. Client signal (preview / small lists)

```rust,ignore
let entries = RwSignal::new(vec![/* HistoryEntry… */]);
view! {
    <HistoryTimeline data_source=HistorySource::Client(entries) />
}
```

#### 2. Server infinite scroll

```rust,ignore
let fetcher: HistoryPageFetcher = Arc::new(|req| {
    Box::pin(async move { fetch_history_page(req).await })
});
view! {
    <HistoryTimeline
        data_source=HistorySource::Server { fetcher, page_size: 20 }
        paging=HistoryPagingMode::InfiniteScroll
    />
}
```

#### 3. Default field-diff only

Host maps domain events to `HistoryChange::FieldDiff { … }` / `Created` / `Deleted`. No custom renderers required.

#### 4. Custom `kind` renderer

```rust,ignore
let mut kind_views = HashMap::new();
kind_views.insert(
    "comment".into(),
    Arc::new(|ctx: HistoryRenderContext| {
        Some(view! { <CommentHistoryRow entry=ctx.entry /> }.into_any())
    }) as HistoryEntryView,
);
let renderers = HistoryRenderers {
    kind_views,
    ..Default::default()
};
view! {
    <HistoryTimeline data_source=source renderers=renderers />
}
```

Return `None` from a kind renderer to fall through to the default row for that entry.

#### 5. Layout modes

```rust,ignore
// Default: natural timeline spine
<HistoryTimeline data_source=source />

// Dense inline sentences
<HistoryTimeline data_source=source layout=HistoryLayout::Compact />
```

#### 6. Dialog vs detail card

```rust,ignore
// Dialog body
<HistoryTimeline data_source=source max_height="360px".to_string() />

// Detail card / tab panel (flex parent with min-height: 0)
<div class="host-card-body">
    <HistoryTimeline data_source=source />
</div>
```

#### 7. Host-controlled initial loading (client source)

```rust,ignore
let entries = RwSignal::new(Vec::<HistoryEntry>::new());
let loading = RwSignal::new(true);
// host fetches, then: entries.set(data); loading.set(false);
view! {
    <HistoryTimeline
        data_source=HistorySource::Client(entries)
        loading=loading.read_only()
        skeleton_row_count=6
    />
}
```

Server source does not need `loading` unless the host wants to override the hook-derived state. Pagination always uses the incremental footer loader, never the full skeleton.

---

## 5. Layout & visual design

### Natural (default)

```
  •  3:42 PM
  |  Jordan Lee
  |  changed name from "A" to "B"
```

- Spine marker + vertical connector using `var(--orb-color-border-subtle)`
- Timestamp above actor: `Caption1`, neutral foreground
- Actor: `Body1Strong`; change line: `Body1`
- Best default for drawers, side panels, dialogs, and mobile

### Compact

```
Jordan Lee changed name from "A" to "B" at 3:42 PM
```

- Single inline sentence: actor + change + localized `" at "` prefix + timestamp
- No spine column; tighter row padding
- Opt-in for dense feeds and narrow panels

### Initial-load skeleton

Same chrome as real rows, with `SkeletonItem` bars instead of text. Default `skeleton_row_count` is **5**.

**Natural (default):**

```
  •  [====]     ← timestamp bar
  |  [========] ← actor bar
  |  [==============] ← change bar
  •  [====]
  |  [========]
  |  [==============]
  …
```

**Compact:**

```
[==============================================]  ← full-width sentence bar
[==============================================]
…
```

Incremental loading does **not** use this shell. It renders a compact footer row (`orbital-history__loading-more`) with `Spinner` below the existing entries.

### Composition primitives

| Need | Primitive |
| --- | --- |
| Row structure | Flex / gap tokens |
| Section title | `Subtitle1` (section/card titles; not `Subtitle2`) |
| Actor / change | `Body1` / `Body1Strong` |
| Timestamp | `Caption1` |
| Empty / error | `MessageBar` |
| Initial loading | `Skeleton` / `SkeletonItem` via `HistoryTimelineSkeleton` |
| Incremental loading | `Spinner` (footer) |
| Scroll | `ScrollArea` + paging infinite-scroll sentinel |
| Actor link | `Link` when `ACTOR_LINKS` and `href` present |
| Date divider | `Divider` + label (discussion pattern) |
| Avatar | Optional in custom / rich rows only — **not** default field-diff |

No long-term reliance on one-off inline styles; tokens and `orbital-history__*` classes are the product surface.

### CSS

- Inject once at product root: `inject_style("orbital-history", history_styles())`
- Block: `orbital-history` on `[data-orbital-history]`
- Elements: `orbital-history__*` (e.g. `__scroll`, `__list`, `__entry`, `__time-rail`, `__spine`, `__actor`, `__change`, `__date-divider`)
- Density: `orbital-history--density-compact` / `orbital-history--density-spacious` from `use_theme_options().density` (default density = no modifier class)
- Orientation modifiers: `orbital-history__entry--vertical` / `--horizontal`
- Skeleton: `orbital-history__skeleton`, `orbital-history__skeleton-row`
- Incremental loader: `orbital-history__loading-more`
- CSS variables on the root for rail width, spine size, row padding (overridden by density)

### Date dividers (visual)

When `DATE_DIVIDERS` is enabled, insert `HistoryDateDivider` on **bucket transitions** only (newest-first). Empty buckets are never shown. At most five section headers appear in a long list (Today, Yesterday, Last 7 days, Last 30 days, Older). See [§6](#6-formatting-rules).

### Host embeds

| Host | Guidance |
| --- | --- |
| Detail-page card | Flex-fill parent; consider `Compact` when space is tight |
| Tab panel | Flex-fill (`min-height: 0` on ancestors) |
| Dialog body | `max_height` (e.g. `"360px"`) + internal `ScrollArea` |
| Drawer | Default `Natural` |

No dialog shell helper in v1.

---

## 6. Formatting rules

### Default templates (English)

| Change | Template |
| --- | --- |
| `Created` | `created` |
| `Deleted { label }` | `deleted "{label}"` |
| `FieldDiff` | `changed {field} from "{old}" to "{new}"` |
| `FieldDiffs` | Header: `changed {n} fields`; each line uses the field-diff template |
| `Custom { summary }` | `summary` as-is (after truncation policy) |

All templates are locale-overridable.

### Truncation

- Default max length: **80** characters (`DEFAULT_TRUNCATE_LEN`)
- Apply to `old_value`, `new_value`, `label`, and `Custom.summary` before quoting / template substitution
- Ellipsis: Unicode `…` (U+2026)
- Truncate the display string, not mid-escape sequence (plain text only in v1)

### Quotes and escaping

- Wrap substituted values in ASCII double quotes: `"…"`
- Do **not** HTML-escape in the format helpers; values render as text nodes (safe by default)
- Hosts must not pass untrusted HTML into custom renderers without sanitizing

### Multi-field (`FieldDiffs`)

- **PR2:** data model + `format_change` produces a readable multi-line or joined summary for tests / accessibility text
- **PR3–PR8:** default row may render `FieldDiffs` as a single summary line via `field_diffs_header_template` or joined field-diff lines
- **PR9:** `HistoryChangeCard` — card-style list of field rows inside one entry

### Date buckets

Calendar-day math uses UTC dates derived from `DateTime<Utc>` for v1 (document display-timezone follow-up if hosts need wall-clock buckets). Relative to `now`:

| Bucket | Rule |
| --- | --- |
| `Today` | Same calendar day as `now` |
| `Yesterday` | Previous calendar day |
| `Last7Days` | 2–7 days ago (exclusive of Today / Yesterday) |
| `Last30Days` | 8–30 days ago |
| `Older` | 31+ days ago |

`with_date_dividers`:

1. Iterate entries in list order (newest-first)
2. Compute bucket per entry
3. When bucket differs from previous entry’s bucket, emit `HistoryListItem::Divider(bucket)` then `Entry`
4. First entry always gets a divider for its bucket so the section is labeled

Disable with `features.remove(HistoryFeatures::DATE_DIVIDERS)`.

---

## 7. Extensibility

### Fallthrough rules

| Hook | Behavior |
| --- | --- |
| `entry_view` returns `Some` | Full custom row; stop |
| `entry_view` returns `None` or absent | Continue |
| `kind_views[kind]` returns `Some` | Full custom row for that kind; stop |
| `kind_views[kind]` returns `None` or absent | Continue |
| `change_view` returns `Some` | Custom change region inside default chrome |
| `change_view` returns `None` or absent | `HistoryChangeLine` / `HistoryChangeCard` |

Structural slots (header, empty, initial loading, loading-more, error, end) **replace** defaults entirely when present (discussion toolbar/empty pattern).

### Host patterns

1. Map domain audit DTOs → `HistoryEntry` in the host (or inside the page fetcher)
2. Use structured `HistoryChange` variants for default formatting
3. Set `kind` to a stable host string for custom rows (`"comment"`, `"attachment"`, `"permission_grant"`)
4. Register `kind_views` or use `HistoryEntrySlot` for one-off full-row UI
5. Load extra data by `entry.id` in the host renderer; do not expect Orbital to fetch

### What Orbital does not do

- Invent routes for actors (`href` is host-provided)
- Persist or subscribe to live updates
- Interpret host domain models beyond `HistoryEntry`

---

## 8. Accessibility

- Entry list uses list semantics (`<ul>` / `<li>` or equivalent `role="list"` / `role="listitem"`)
- Date dividers use `role="separator"` and an accessible name from the bucket label
- Actor links expose an accessible name via `actor_link_aria_template` (include display name)
- System actor is plain text (`locale.system_actor`), not a link
- When `ENTRY_CLICK` is enabled, the row is a single activatable control with a clear accessible name (actor + change summary); avoid nested interactive controls without care
- Do **not** rely on color alone to distinguish change vs delete — wording comes from locale templates
- Timestamps: visible compact text; optional `datetime` attribute on a `<time>` element with ISO-8601 UTC for machines
- During **initial** load, the list region sets `aria-busy="true"` and an accessible name from `locale.loading`; skeleton rows are presentational placeholders (no interactive controls)
- During **incremental** load, the footer spinner uses `locale.loading_more` as its accessible label; existing entries remain in the accessibility tree
- Error regions use `MessageBar` defaults consistent with other Orbital products

---

## 9. Preview & docs plan

### Registration

- `#[component_doc(category = "History", preview_slug = "…", preview_label = "…", preview_icon = …)]` on `HistoryTimeline` and topic doc stubs under `products/history/docs/`
- `preview/static_registrations.rs` → `all()` lists every `*_PREVIEW_REGISTRATION`
- `orbital-preview-app` merges `orbital_history::preview::static_registrations::all()` (same transmute pattern as discussion/datatable)

### Fixtures (`preview/fixtures.rs`)

- Small newest-first field-diff list (client)
- Empty list
- Mixed actors (user + system)
- Long values (truncation)
- Multi-kind list including a `comment` (or similar) for custom renderer demos
- Spanning buckets: Today, Yesterday, Last 7 days, Last 30 days, Older (PR8)
- Mock `HistoryPageFetcher` that pages fixture data (PR5)
- Forced initial skeleton (`loading=true`, empty list) and delayed pages for loading-more (PR5)

### Catalog pages (suggested)

| Slug | Focus |
| --- | --- |
| `history-timeline` | Root product, default vertical |
| `history-layout` | Natural vs Compact |
| `history-data-source` | Client vs Server / infinite scroll |
| `history-loading` | Initial skeleton vs incremental footer spinner (PR5; polish in PR8 if needed) |
| `history-slots` | Empty / loading / loading-more / error / end / header |
| `history-renderers` | Kind + change fallthrough |
| `history-localization` | English / French |
| `history-date-dividers` | Bucket dividers |
| `history-multi-diff` | FieldDiffs card (PR9) |
| `history-embed` | Dialog `max_height` vs flex card |

Each PR that adds UI registers at least one preview so the phase is demoable in `cargo leptos watch -p orbital-preview`.

---

## 10. Phased delivery

Every phase lists scope, public API added, previews/tests, and explicit out-of-scope. Prefer small, mergeable PRs. Stack on `feat/history` (or equivalent) as directed by maintainers.

### PR1 — Crate skeleton

| | |
| --- | --- |
| **Scope** | Workspace member, feature flags (`ssr` / `hydrate` / `preview`), empty `lib.rs`, README stub, this `DESIGN.md`, preview-app / primitives / root README / CONTRIBUTING wiring stubs |
| **Public API** | Crate exists; no UI types yet (or module placeholders only) |
| **Previews / tests** | `cargo check -p orbital-history --no-default-features` |
| **Out of scope** | Types, components, styles |

### PR2 — Types + formatters

| | |
| --- | --- |
| **Scope** | `HistoryEntry`, actor/change enums, source/paging types, features, locale struct (English defaults), `format_change`, `truncate_display_value` |
| **Public API** | All data-model and format helpers above (except date-bucket UI) |
| **Previews / tests** | Unit tests for templates, truncation, quotes; `FieldDiffs` formatting |
| **Out of scope** | Components, CSS |

### PR3 — Leaf components

| | |
| --- | --- |
| **Scope** | `HistoryTimestamp`, `HistoryActorLabel`, `HistoryChangeLine`, `HistoryEntryRow` for **both** Vertical and Horizontal; tokenized styles |
| **Public API** | Leaf components + `history_styles` injection from a thin demo shell if needed |
| **Previews / tests** | Leaf preview cards for both orientations |
| **Out of scope** | List, timeline root, server paging |

### PR4 — `HistoryTimeline` (client source)

| | |
| --- | --- |
| **Scope** | Root component, header, empty state, scroll region, client `HistorySource`, `orientation` prop (**default Vertical**; both orientations fully wired) |
| **Public API** | `HistoryTimeline` with client source |
| **Previews / tests** | Client list, empty, header, orientation toggle |
| **Out of scope** | Server paging, loading phases / skeleton, custom renderers, date dividers |

### PR5 — Server source, infinite scroll, loading phases

| | |
| --- | --- |
| **Scope** | `HistorySource::Server`, `use_paged_infinite_scroll`, `HistoryPagingMode`, `on_load_error`; **loading phases**: optional host `loading` signal, `skeleton_row_count`, initial `HistoryTimelineSkeleton` / `HistoryEntryRowSkeleton`, `HistoryLoadingView` (initial only), `HistoryLoadingMoreView` + footer spinner (incremental only), end / error slots and defaults, `locale.loading_more` |
| **Public API** | Server source path; loading props; skeleton leaves; loading / loading-more / end / error slots |
| **Previews / tests** | Mock fetcher infinite-scroll preview; initial skeleton; scroll-triggered loading-more |
| **Out of scope** | Custom renderers, density polish, full catalog polish for loading (optional `history-loading` page may land minimal here and be polished in PR8) |

**Do not** use the full skeleton during pagination or refetch when entries are already visible.

### PR6 — Renderers + slots

| | |
| --- | --- |
| **Scope** | `HistoryRenderers`, entry / kind / change fallthrough, renderer slots, document host patterns in rustdoc |
| **Public API** | Full renderer and slot surface |
| **Previews / tests** | Custom `kind` preview |
| **Out of scope** | Density, locale French preset |

### PR7 — Density + responsive notes

| | |
| --- | --- |
| **Scope** | Density modifier classes, narrow-panel guidance in docs |
| **Public API** | Density CSS variables / modifiers (no new components required) |
| **Previews / tests** | Density previews (`ThemeDensityStepper` if used elsewhere) |
| **Out of scope** | Locale French, date dividers |

### PR8 — Locale, date-bucket dividers, a11y, catalog polish

| | |
| --- | --- |
| **Scope** | `HistoryLocale::french`, `HistoryDateBucket`, `history_date_bucket`, `with_date_dividers`, `HistoryDateDivider`, `DATE_DIVIDERS` (default on), events polish, a11y pass, catalog polish (including dedicated `history-loading` page if PR5 shipped a minimal one) |
| **Public API** | Date-bucket types, divider component, full locale presets |
| **Previews / tests** | Localization + bucket-divider fixtures (Today / Last 7 days / Older); loading catalog polish |
| **Out of scope** | Multi-field card UI |

### PR9 — Multi-field diff card

| | |
| --- | --- |
| **Scope** | `HistoryChangeCard` default UI for `HistoryChange::FieldDiffs` |
| **Public API** | `HistoryChangeCard` |
| **Previews / tests** | Multi-diff preview |
| **Out of scope** | Realtime |

### Optional later

Phases 2–7 are implemented in the crate. Remaining candidates:

- Host-side transport helper crate (outside Orbital; timeline hooks are documented)
- Rich-text editing (history remains read-oriented)
- CSV/JSON export of visible entries (datatable-style)
- Discussion `@` mention parity in composer (history is read-only; host inserts markdown)

### Phase 7 (implemented)

| PR | Feature |
| --- | --- |
| PR43 | Live-update host integration docs (`live_transport` catalog; hooks only, no in-crate WS/SSE) |
| PR44 | `@[Name](id)` mentions via `orbital-markdown`, `HistoryMention`, Persona hover popover, `MARKDOWN_MENTIONS`, `on_mention_click` |
| PR45 | `HistoryAttachment` on Markdown change, `MARKDOWN_IMAGES`, attachment image dedup/render parity |
| PR46 | Configurable `group_by` actor \| kind, `GROUP_COLLAPSE`, group header UI, handle expand/collapse |
| PR47 | Playwright E2E (mentions, images, grouping) + README/DESIGN sync |

### Phase 6 (implemented)

| PR | Feature |
| --- | --- |
| PR36 | `HistoryLiveScrollPolicy` on `live_head` / `prepend_live` |
| PR37 | `read_watermark`, `UNREAD_HIGHLIGHT`, handle + serialized state |
| PR38 | `HistoryCitation` on Markdown change, history citation anchors, `on_citation_click` |
| PR39 | Variable-height virtualization (`VARIABLE_ROW_HEIGHT`) |
| PR40 | `HistoryDialog` passthrough for Phase 5/6 timeline props |
| PR41 | Playwright E2E expansion (`history.spec.ts`) |

### Phase 5 (implemented)

| PR | Feature |
| --- | --- |
| PR28 | `live_head` merge + `HistoryHandle::prepend_live` |
| PR29 | Filter chrome kind/actor chips (`filter_kinds`, `filter_actors`) |
| PR30 | `HistoryPaginationView` slot |
| PR31 | Virtualization polish (ResizeObserver viewport, `virtual_row_height`) |
| PR32 | `HistorySerializedState` + `export_state` / `restore_state` |
| PR33 | `DIFF_HIGHLIGHT` field diff styling |
| PR34 | Playwright E2E (`history.spec.ts`) |

### Phase 4 (implemented)

| PR | Feature |
| --- | --- |
| PR20 | `FILTER_CHROME` — built-in search input |
| PR21 | `SORT_CHROME` — built-in newest/oldest toggle |
| PR22 | `SERVER_FILTER` + `HistoryFetchParams` |
| PR23 | `SERVER_SORT` on fetcher |
| PR24 | Client `Paged` in-memory windowing |
| PR25 | `scroll_to_entry_or_load` across Server `Paged` pages |
| PR26 | `VIRTUALIZE` long lists |
| PR27 | `MARKDOWN_BODIES` + `HistoryChange::Markdown` |

---

## 11. Open questions

None remaining for implementers.

### Resolved decisions

- **Layout:** Both ship (PR3 leaves + PR4 timeline). **`HistoryLayout::Natural` is the default**; `Compact` is opt-in for dense inline rows.
- **Date dividers:** Ship in PR8 behind `HistoryFeatures::DATE_DIVIDERS` (**default on**). Use **relative buckets** (Today → Yesterday → Last 7 days → Last 30 days → Older), not per-calendar-day dividers, so a long audit log shows at most a handful of section headers.
- **Time type:** `DateTime<Utc>` on `HistoryEntry` for paging serde; format with base-components helpers.
- **Sort:** Host pre-sorts newest-first; timeline does not re-sort in v1.
- **Pagination:** Infinite-scroll via `HistorySource::Server` + `HistoryPageFetcher` + `HistoryPagingMode`; no page-number bar in v1.
- **Loading phases:** Full timeline skeleton for **initial** load only; footer spinner for **incremental** pagination / refetch. Optional host `loading` prop; Server derives from the paging hook when omitted.
- **Quality rubric:** Planning-only; not part of this document.

When uncertain during implementation, choose the option that keeps the crate extractable and host-agnostic.
