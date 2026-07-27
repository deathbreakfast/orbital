# minimal-ssr-hydrate

**Teaches:** Smallest Orbital product host — `orbital_shell` / boot overlay, `OrbitalTemplate`, and `hide_boot_loader` after WASM hydrate.

**Topology:** Embedded (one Axum + Leptos process). No external services.

## Prerequisites

```bash
# cargo install cargo-leptos   # once, ≥ 0.3.6
# nightly via rust-toolchain.toml
```

Workspace `.cargo/config.toml` pins `LEPTOS_OUTPUT_NAME=orbital-preview` for the catalog host. Override when watching this example:

```bash
export LEPTOS_OUTPUT_NAME=minimal-ssr-hydrate
```

## Run

```bash
cargo leptos watch --split --project minimal-ssr-hydrate
```

Open <http://127.0.0.1:3030/> — boot overlay should dismiss; **Increment** bumps the counter without a full reload.

Compile-check (no WASM):

```bash
cargo check -p minimal-ssr-hydrate --features ssr
```

**Open first:** [`src/app.rs`](src/app.rs) → [`src/lib.rs`](src/lib.rs) (`hydrate` + `hide_boot_loader`) → [`src/main.rs`](src/main.rs)

**Success:** page loads; overlay gone after hydrate; Increment updates `Clicks: N`.

**Next step:** [`../authenticated-dashboard`](../authenticated-dashboard/) for auth context, route guards, and theme/density.
