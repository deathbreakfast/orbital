# Orbital examples

Runnable **adopter** teaching hosts — not the maintainer catalog under `orbital-preview-*` / `component-preview-e2e`. Start with the SSR+hydrate shell; then auth gates; then server-paged analytics.

Each card: when to use · command · success · look next.

**Prerequisites:** nightly (`rust-toolchain.toml`) · [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) ≥ 0.3.6.

Workspace `.cargo/config.toml` pins `LEPTOS_OUTPUT_NAME=orbital-preview` for the catalog. Override per example when watching:

```bash
export LEPTOS_OUTPUT_NAME=<example-name>
```

---

## Canonical path

### 1. `minimal-ssr-hydrate` — SSR + hydrate shell (first)

**Teaches:** `orbital_shell` / boot overlay, `OrbitalTemplate`, and `hide_boot_loader` after WASM hydrate.

```bash
export LEPTOS_OUTPUT_NAME=minimal-ssr-hydrate
cargo leptos watch --split --project minimal-ssr-hydrate
# open http://127.0.0.1:3030/
```

**Open first:** [`minimal-ssr-hydrate/src/app.rs`](minimal-ssr-hydrate/src/app.rs)

**Success:** boot overlay dismisses; **Increment** bumps the counter without a full reload.

**Next step:** [`authenticated-dashboard`](#2-authenticated-dashboard--auth--theme).

---

### 2. `authenticated-dashboard` — auth + theme

**Teaches:** `provide_auth_context` + `RequireAuthenticated`, plus theme mode / density prefs in `localStorage`.

```bash
export LEPTOS_OUTPUT_NAME=authenticated-dashboard
cargo leptos watch --split --project authenticated-dashboard
# open http://127.0.0.1:3031/
```

**Open first:** [`authenticated-dashboard/src/app.rs`](authenticated-dashboard/src/app.rs)

**Success:** `/dashboard` gates anonymous users; after **Sign in** the protected card renders; Dark/Compact survive refresh.

**Next step:** [`server-paged-analytics`](#3-server-paged-analytics--table--chart).

---

### 3. `server-paged-analytics` — table + chart

**Teaches:** `DataTableSource::Server` with `PageRequest`/`Page`, plus a summary `BarChart` on an `orbital_data::Dataset`.

```bash
export LEPTOS_OUTPUT_NAME=server-paged-analytics
cargo leptos watch --split --project server-paged-analytics
# open http://127.0.0.1:3032/
```

**Open first:** [`server-paged-analytics/src/data.rs`](server-paged-analytics/src/data.rs)

**Success:** 8 rows per page across 48; pagination changes rows; four region bars on the chart.

**Next step:** Component preview for widget APIs, or a feature-crate composition host when published.

---

## Quick reference

| Example | Port | Command |
|---------|------|---------|
| `minimal-ssr-hydrate` | 3030 | `cargo leptos watch --split --project minimal-ssr-hydrate` |
| `authenticated-dashboard` | 3031 | `cargo leptos watch --split --project authenticated-dashboard` |
| `server-paged-analytics` | 3032 | `cargo leptos watch --split --project server-paged-analytics` |

Compile-check (no WASM / no browser):

```bash
cargo check -p minimal-ssr-hydrate --features ssr
cargo check -p authenticated-dashboard --features ssr
cargo check -p server-paged-analytics --features ssr
```

**Not demos:** `orbital-preview-*`, `component-preview-e2e`, and `end2end/` are maintainer catalog / Playwright infra — copy APIs from them if useful, but do not treat them as adopter quickstarts.
