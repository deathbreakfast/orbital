# Orbital

[![CI](https://github.com/unified-field-dev/orbital/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/orbital/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/orbital-ui.svg)](https://crates.io/crates/orbital-ui)
[![docs.rs](https://docs.rs/orbital-ui/badge.svg)](https://docs.rs/orbital-ui)
[![Preview catalog](https://img.shields.io/badge/docs-preview_catalog-blue)](https://unified-field-dev.github.io/orbital/)

**Orbital** is a Leptos component library and design system for building focused, accessible product interfaces in Rust. It ships themed UI primitives, shared typed data models, and higher-level feature crates (tables, charts, scheduling, and more) that compose on the same tokens, motion vocabulary, and accessibility patterns.

**Status:** v0.1.1 early release · [MIT](LICENSE) · [GitHub](https://github.com/unified-field-dev/orbital)

**Requires:** nightly Rust ([`rust-toolchain.toml`](rust-toolchain.toml)) · [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) **≥ 0.3.6** · Node 24+ for Playwright

## Quick start

Add the public crate (`orbital-ui` on crates.io; Rust imports stay `orbital::`) and wrap your app in [`OrbitalTemplate`](orbital/README.md):

```toml
[dependencies]
orbital-ui = { version = "0.1", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

```rust,ignore
use leptos::prelude::*;
use orbital::{OrbitalTemplate, components::{
    Button, ButtonAppearance, Card, CardContent, CardFooter, CardHeader,
    CardHeaderAction, CardHeaderDescription, CardMedia, Field, Flex,
    FlexGap, Input, InputAppearance, Subtitle1, Tooltip, TooltipPosition,
}};

#[component]
fn App() -> impl IntoView {
    let display_name = RwSignal::new(String::new());
    let work_email = RwSignal::new(String::new());

    view! {
        <OrbitalTemplate>
            <main style="padding: 24px; max-width: 420px; margin: 0 auto;">
                <Card>
                    <CardHeader>
                        <Subtitle1>"Create your workspace"</Subtitle1>
                        <CardHeaderDescription slot>
                            "Set up a team space in a few steps."
                        </CardHeaderDescription>
                        <CardHeaderAction slot>
                            <Tooltip content="More options" position=TooltipPosition::Bottom>
                                <Button
                                    appearance=ButtonAppearance::Transparent
                                    icon=icondata::AiMoreOutlined
                                    aria-label="More options"
                                />
                            </Tooltip>
                        </CardHeaderAction>
                    </CardHeader>
                    <CardMedia
                        src="https://picsum.photos/seed/orbital-workspace/420/160"
                        alt="Workspace illustration"
                        height=160
                    />
                    <CardContent>
                        <Flex vertical=true gap=FlexGap::Medium full_width=true>
                            <Field label="Display name" name="display_name">
                                <Input
                                    bind=display_name
                                    appearance=InputAppearance::with_placeholder("Acme Team")
                                />
                            </Field>
                            <Field label="Work email" name="email">
                                <Input
                                    bind=work_email
                                    appearance=InputAppearance::email("you@company.com")
                                />
                            </Field>
                        </Flex>
                    </CardContent>
                    <CardFooter>
                        <Button appearance=ButtonAppearance::Secondary>"Cancel"</Button>
                        <Button>"Create workspace"</Button>
                    </CardFooter>
                </Card>
            </main>
        </OrbitalTemplate>
    }
}
```

This demonstrates [`OrbitalTemplate`](orbital/README.md) shell · compound `Card` (header, media, body, footer) · `Field` + `Input` · `Tooltip` on an icon action · primary/secondary CTAs in `CardFooter`. Live docs for these components are in the [component preview](#component-preview) below.

For SSR + hydrate feature flags, auth wiring, and `default-features = false` in production, see [`orbital/README.md`](orbital/README.md) and [orbital-macros/README.md — consumer feature flags](orbital-macros/README.md#consumer-feature-flags).

## Examples (adopter hosts)

Runnable teaching hosts under [`examples/`](examples/) — **not** the preview catalog. Full ladder: [`examples/README.md`](examples/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`minimal-ssr-hydrate`](examples/minimal-ssr-hydrate/) | First SSR+hydrate shell (`orbital_shell`, boot overlay, `hide_boot_loader`) | `LEPTOS_OUTPUT_NAME=minimal-ssr-hydrate cargo leptos watch --split --project minimal-ssr-hydrate` → :3030 | Overlay dismisses; Increment bumps counter | `authenticated-dashboard` |
| [`authenticated-dashboard`](examples/authenticated-dashboard/) | Auth context + `RequireAuthenticated` (+ theme/density prefs) | `LEPTOS_OUTPUT_NAME=authenticated-dashboard cargo leptos watch --split --project authenticated-dashboard` → :3031 | `/dashboard` gates anonymous; Sign in unlocks card | `server-paged-analytics` |
| [`server-paged-analytics`](examples/server-paged-analytics/) | Server-paged `DataTable` + summary `BarChart` | `LEPTOS_OUTPUT_NAME=server-paged-analytics cargo leptos watch --split --project server-paged-analytics` → :3032 | 8 rows/page of 48; chart shows four regions | Preview catalog for widget APIs |

Compile-check without WASM: `cargo check -p minimal-ssr-hydrate --features ssr` (same pattern for the other two).

`orbital-preview-*` and `component-preview-e2e` remain maintainer catalog / E2E infra — not copy-paste product demos.

## How Orbital compares

Orbital targets **product-grade Leptos apps** — a cohesive design system plus first-party feature crates for data tables, charts, scheduling, discussion, and related application UI. Other frameworks ship broader primitive suites, but none combine this product-heavy feature set in one integrated Rust-native package set today. Public-surface counts below are approximate and not apples-to-apples (catalog pages vs. docs.rs modules vs. gallery entries).

### Web UI frameworks (baseline)

These are the **rendering layers** component libraries sit on. Orbital is **Leptos-only** today.

| Framework | Reactivity | Typical web performance | SSR / full-stack | Notes |
|-----------|------------|------------------------|------------------|-------|
| [Leptos](https://leptos.dev/) | Fine-grained signals (no VDOM) | Strong for interactive UIs — updates track data changes, not whole trees | First-class (`cargo-leptos`, server functions) | Orbital’s foundation |
| [Sycamore](https://sycamore.dev/) | Fine-grained signals (no VDOM) | Strong; same reactive model as Leptos | SSR + streaming supported | Smaller component-library ecosystem |
| [Yew](https://yew.rs/) | Virtual DOM | Mature ecosystem; diffing overhead on large trees | Supported | Longest-running Rust WASM UI stack |
| [Dioxus](https://dioxuslabs.com/) | Virtual DOM | Solid; VDOM diff cost vs signals | SSR + desktop/mobile targets | “Write once, run everywhere” focus |

Component libraries inherit their framework’s runtime model. Orbital does not change Leptos’s signal performance — it adds typed components, tokens, and heavier widgets on top.

### Component library comparison

| Framework | Library | ~Public surface | Design system | Data table | Charts / viz | Scheduling | Live docs |
|-----------|---------|----------------|---------------|------------|--------------|------------|-----------|
| Leptos | **[Orbital](https://github.com/unified-field-dev/orbital)** (this repo) | ~220 catalog pages | Tokens, motion, materials, density | Sort, filter, edit, paging, grouping, pivot, export | Bar, line, area, pie, scatter, heatmap, gauge, sparkline | Calendar + timeline, drag-resize, resource lanes, event editing | [Component preview](https://unified-field-dev.github.io/orbital/) |
| Leptos | [Radix-Leptos](https://github.com/cloud-shuttle/radix-leptos) | 57+ | Unstyled accessible primitives | — | — | — | [docs.rs](https://docs.rs/radix-leptos) |
| Leptos | [Thaw UI](https://github.com/thaw-ui/thaw) | ~50+ (conservative) | Fluent-inspired | `Table` | — | — | [thawui.vercel.app](https://thawui.vercel.app) |
| Leptos | [leptos-shadcn-ui](https://github.com/cloud-shuttle/leptos-shadcn-ui) | 25+ | shadcn / Tailwind | Basic `Table` | — | — | [docs.rs](https://docs.rs/leptos-shadcn-ui) |
| Yew | [PatternFly Yew](https://github.com/patternfly-yew/patternfly-yew) | ~64 modules | PatternFly | `table` module | — | — | [docs.rs](https://docs.rs/patternfly-yew) |
| Yew | [yew-bootstrap](https://github.com/isosphere/yew-bootstrap) | ~38 | Bootstrap | Basic table | — | — | [docs.rs](https://docs.rs/yew-bootstrap) |
| Yew | [material-yew](https://material-yew.rm.rs/) | ~23 | Material Web Components | — | — | — | [material-yew.rm.rs](https://material-yew.rm.rs/) |
| Dioxus | [Dioxus Components](https://github.com/DioxusLabs/components) | ~44 gallery entries | shadcn-style on unstyled primitives | — | — | — | [dioxuslabs.github.io/dioxus-components](https://dioxuslabs.github.io/dioxus-components/) |
| Dioxus | [dioxus-daisyui](https://crates.io/crates/dioxus-daisyui) | DaisyUI set | Tailwind + DaisyUI | Basic table | — | — | [docs.rs](https://docs.rs/dioxus-daisyui) |

Specialized widgets omitted from this table: [`yew-datatable`](https://docs.rs/yew-datatable-core) and [`table-rs`](https://crates.io/crates/table-rs) are table-focused libraries, not general component suites.

## Documentation

- **Foundation components** — buttons, forms, navigation, overlays, tables, and layout primitives for SSR and hydration.
- **Feature crates** — data tables, charts, date/time pickers, schedulers, trees, discussion, and history for product-heavy screens ([datatable](orbital-datatable/README.md), [charts](orbital-charts/README.md), [date-pickers](orbital-date-pickers/README.md), [scheduler](orbital-scheduler/README.md), [discussion](orbital-discussion/README.md), [history](orbital-history/README.md)).
- **Design language** — spacing, typography, elevation, materials, brand tone, and motion via CSS variables and theme hooks ([Introduction](https://unified-field-dev.github.io/orbital/), [Theme](https://unified-field-dev.github.io/orbital/theme), [Boot loader](https://unified-field-dev.github.io/orbital/boot-loader)).
- **Shared types** — UI-agnostic `Dataset`, paging, and datetime crates so tables, charts, and schedulers share one wire format ([details below](#shared-types)).
- **Live preview** — ~220 interactive catalog pages with examples and Playwright coverage.

### Component preview

The **[live catalogue](https://unified-field-dev.github.io/orbital/)** is the published reference for browsing ~220 component pages — jump to [Data Table](https://unified-field-dev.github.io/orbital/data-table), [Charts](https://unified-field-dev.github.io/orbital/bar-chart), [Date pickers](https://unified-field-dev.github.io/orbital/date-picker), [Scheduler](https://unified-field-dev.github.io/orbital/scheduler-calendar), [Discussion](https://unified-field-dev.github.io/orbital/discussion), or [Tree view](https://unified-field-dev.github.io/orbital/tree-view). For local iteration, `cargo leptos watch --split -p orbital-preview` serves the same routes at `http://127.0.0.1:3010/orbital/{slug}` — that preview host is what Playwright drives in CI; open it when you are editing a component or writing a spec, and use the GitHub Pages catalogue when you want the stable, shareable doc surface without running cargo-leptos.

Catalog pages are generated by the `#[component_doc]` macro in [`orbital-macros`](orbital-macros/README.md) — rustdoc, props metadata, and live examples stay in sync with the source.

```rust,ignore
use orbital_macros::component_doc;

#[component_doc(
    category = "Actions",
    preview_slug = "button",
    preview_label = "Button",
)]
#[component]
pub fn Button(/* ... */) -> impl IntoView { /* ... */ }
```

Each entry becomes a page at `/orbital/{preview_slug}` (for example [Button](https://unified-field-dev.github.io/orbital/button)). Mark examples with `<!-- preview -->` in `# Examples` rustdoc; enable the crate `preview` feature only on the doc host. See [`orbital-macros/README.md`](orbital-macros/README.md) and [component testing](docs/component-testing.md).

Local: `cargo leptos watch --split -p orbital-preview` → `http://127.0.0.1:3010/orbital/{slug}`

### Repository docs

[`orbital/README.md`](orbital/README.md) · [`orbital-macros/README.md`](orbital-macros/README.md) · [component testing](docs/component-testing.md) · `cargo doc -p orbital-ui --open`

## Design language

Components share a single visual and interaction vocabulary:

| Layer | Crate | What it defines |
|-------|-------|-----------------|
| Tokens | `orbital-shell`, `orbital-style` | Corner radius, elevation, materials, brand tone, stroke, glow |
| Theme | `orbital-theme` | Light/dark mode, density (`Compact` / `Default` / `Spacious`), typography and spacing overrides |
| Motion | `orbital-motion` | Duration curves, presence transitions, reduced-motion respect |
| Components | `orbital-core-components` | Themed controls that consume the tokens above |

- **[Introduction (live)](https://unified-field-dev.github.io/orbital/)** — principles, color, elevation, typography, motion
- **Crate READMEs:** [datatable](orbital-datatable/README.md) · [charts](orbital-charts/README.md) · [date-pickers](orbital-date-pickers/README.md) · [scheduler](orbital-scheduler/README.md) · [discussion](orbital-discussion/README.md) · [history](orbital-history/README.md)

Authors wrap apps in `OrbitalThemeProvider` and compose shell chrome via [`OrbitalTemplate`](orbital/README.md). Capability is expressed through feature flags on each crate (for example `DataTableFeatures`, `ChartFeatures`) — not license-tier suffixes on type names.

## Shared types

These crates have **no UI dependency** — use them in server functions, adapters, and tests without pulling in Leptos views.

| Crate | Key types | Used by |
|-------|-----------|---------|
| [`orbital-data`](orbital-data/) | `Dataset`, `DataRecord`, `DataValue`, `DataSchema`, `FieldDef` | Data tables and charts share one chart-friendly dataset model |
| [`orbital-paging`](orbital-paging/) | `Page<T>`, `PageRequest`, `SortDirection` | Server fetch, infinite scroll, table paging |
| [`orbital-base-components`](orbital-base-components/) (datetime) | `OrbitalDateTime`, `DatetimeTimezone`, `DatetimeFormat` | Date pickers and scheduler events |
| [`orbital-markdown`](orbital-markdown/) | `OrbitalMarkdownOptions`, citation refs | Discussion and rich text rendering |

## Component surface

Import the public crate [`orbital`](orbital/) for shell, auth hooks, and `orbital::primitives`, or depend on individual workspace crates with `default-features = false` in production (enable `preview` only for the doc host).

### Foundation

Buttons, inputs, dialogs, drawers, menus, tabs, cards, lists, avatars, pagination, and layout (`Flex`, `Grid`, `Stack`, `ScrollArea`). Browse the [component preview](https://unified-field-dev.github.io/orbital/) or run `cargo leptos watch --split -p orbital-preview` locally.

### Feature crates

| Product | Crate | Highlights |
|---------|-------|------------|
| Data Table | `orbital-datatable` | Sorting, filtering, editing, paging, grouping, pivot, export, chart binding |
| Charts | `orbital-charts` | Bar, line, area, pie, scatter, heatmap, gauge, sparkline, zoom/pan, tooltips |
| Date Pickers | `orbital-date-pickers` | Calendar, fields, range pickers, timezone-aware `OrbitalDateTime` |
| Scheduler | `orbital-scheduler` | Calendar and timeline views, drag-resize, resource lanes, event editing |
| Tree | `orbital-tree` | Hierarchical navigation and selection |
| Discussion | `orbital-discussion` | Threaded replies, composer, markdown, attachments |
| History | `orbital-history` | Audit timeline, field diffs, custom kind renderers |

## Development

**Toolchain:** nightly Rust (see `rust-toolchain.toml`), [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) **≥ 0.3.6**, Node 24+ for Playwright.

**Fonts:** Orbital ships self-hosted [League of Moveable Type](https://www.theleagueofmoveabletype.com/) faces (League Spartan, League Mono, Orbitron) under [OFL 1.1](https://openfontlicense.org/). Files live in [`public/fonts/`](public/fonts/) and are copied into the preview site by cargo-leptos — they are not generated by the build. Ensure the `.woff2` files are present locally before running font-related E2E tests.

`wasm-bindgen` is pinned to an exact patch version in the workspace `Cargo.toml` (currently `=0.2.126`) so it matches the `wasm-bindgen-cli` version used during `cargo leptos build --split`. If you see a schema version mismatch error, align `cargo-leptos`, the workspace pin, and your global `wasm-bindgen-cli` to the same patch version.

Component work uses a three-layer test model — see [docs/component-testing.md](docs/component-testing.md) for conventions, helpers, and `data-testid` rules.

### L1 — Macro / compile

```bash
cargo test -p component-preview-e2e
```

### L2 — Preview host

```bash
cargo leptos watch --split -p orbital-preview
# http://127.0.0.1:3010/orbital/button
```

### L3 — Browser E2E

Playwright specs live in `end2end/tests/components/` (259 spec files + smoke).

**One-shot:**

```bash
cd end2end && npm ci && npx playwright install --with-deps chromium
cargo leptos end-to-end --project orbital-preview
```

**With the server already running:**

```bash
cd end2end && npm test
```

### Release gate (before tagging)

```bash
./scripts/maintainer/preview_release_gate.sh
```

Runs a workspace build (deny warnings) then full release-mode E2E (~1–2 hours). PR CI already runs the Playwright suite; this is a maintainer convenience before tagging. Tag deploy publishes via [`.github/workflows/preview-pages.yml`](.github/workflows/preview-pages.yml). Maintainers: see [CONTRIBUTING.md — Publishing preview](CONTRIBUTING.md#publishing-preview).

## Verify

Same commands as [`.github/workflows/ci.yml`](.github/workflows/ci.yml):

```bash
cargo fmt --all -- --check
cargo test -p component-preview-e2e
cargo check --workspace
cargo check -p orbital-core-components --no-default-features
cargo check -p orbital-datatable --no-default-features
cargo check -p orbital-charts --no-default-features
cargo check -p orbital-date-pickers --no-default-features
cargo check -p orbital-scheduler --no-default-features
cargo check -p orbital-tree --no-default-features
cargo check -p orbital-history --no-default-features
cargo clippy --workspace --all-targets -- -D warnings
```

## License and support

Orbital is **open source and free** under [MIT](LICENSE).

If Orbital helps your project, consider sponsoring development:

- [GitHub Sponsors](https://github.com/sponsors/deathbreakfast)
- [Patreon](https://www.patreon.com/cw/u4202798)

## Contributing

| Doc | Purpose |
|-----|---------|
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Prerequisites, verify commands, PR expectations |
| [`SECURITY.md`](SECURITY.md) | Report vulnerabilities privately |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Community standards |
