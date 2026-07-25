# Contributing to Orbital

Orbital is an early-release (v0.1.x) Leptos component library. Small, focused
changes are easiest to review and merge.

## Prerequisites

- **Nightly Rust** — see [`rust-toolchain.toml`](rust-toolchain.toml)
- **cargo-leptos** ≥ 0.3.6 — `cargo install cargo-leptos --locked --version 0.3.6`
- **wasm-bindgen-cli** — match the workspace pin in [`Cargo.toml`](Cargo.toml) (currently `0.2.126`)
- **Node 24+** and Playwright for browser E2E — see [README — Development](README.md#development)

## Verify locally

Run the same checks as CI before opening a PR:

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

### Full preview E2E (optional, slower)

```bash
cd end2end && npm ci && npx playwright install --with-deps chromium
cargo leptos end-to-end --project orbital-preview
```

Or with the preview server already on `:3010`: `cd end2end && npm test`.

Component testing layers (L1/L2/L3) are documented in [docs/component-testing.md](docs/component-testing.md).

## Release / tagging (maintainers)

Do not push a `v*` tag until the local release gate passes:

```bash
./scripts/maintainer/preview_release_gate.sh
```

Tag deploys run full Playwright E2E and publish the preview catalog to GitHub Pages via [`.github/workflows/preview-pages.yml`](.github/workflows/preview-pages.yml).

## Publishing to crates.io (maintainers)

Library crates are published to crates.io under names such as `orbital-ui` (public crate;
Rust imports remain `use orbital::...`), `orbital-macros`, `orbital-core-components`,
and the other `orbital-*` packages. Preview/e2e workspace members set `publish = false`.

Publish bottom-up (leaves first) after `cargo publish -p <crate> --dry-run` succeeds.
See the project publish plan for order and docs.rs feature settings.

Consumers should depend on registry versions (for example
`orbital-ui = { version = "0.1", default-features = false, features = ["hydrate"] }`),
not workspace `[patch.crates-io]` entries — this repo no longer ships vendor patches
that crates.io users would receive.

## Publishing preview

One-time setup: **Settings → Pages → Build and deployment** → Source: **GitHub Actions**.

After `preview_release_gate.sh` passes on `main`, push a version tag:

```bash
git tag v0.1.1
git push origin v0.1.1
```

The preview workflow publishes `target/site-preview/` to [Component preview](https://unified-field-dev.github.io/orbital/).

## Pull request expectations

- Keep diffs focused; one logical change per PR when possible
- Run the verify commands above (or explain what you could not run)
- Update docs and preview examples when behavior or public API changes
- Do not commit secrets, credentials, or large binaries

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

See [SECURITY.md](SECURITY.md). Do not open public issues for vulnerabilities.
