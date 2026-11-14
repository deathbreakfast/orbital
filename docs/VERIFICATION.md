# Verifying Orbital

How to run what CI runs, and how to run it quickly. The commands come from
[`.github/workflows/ci.yml`](../.github/workflows/ci.yml); when the two disagree, CI wins
and this file needs an update.

## What CI runs

Every job sets `RUSTFLAGS="-D warnings"`, `CARGO_PROFILE_DEV_DEBUG=0` and
`CARGO_PROFILE_TEST_DEBUG=0`, and builds with the nightly toolchain from
[`rust-toolchain.toml`](../rust-toolchain.toml) plus the `wasm32-unknown-unknown` target.

| Job | Command |
|---|---|
| fmt | `cargo fmt --all -- --check` |
| build | `cargo test -p component-preview-e2e`, then `cargo build --workspace`, then `cargo build -p <crate> --no-default-features` for `orbital-core-components`, `orbital-datatable`, `orbital-charts`, `orbital-date-pickers`, `orbital-scheduler` and `orbital-tree` |
| test | `cargo test --workspace --lib --tests --exclude orbital-preview-frontend --exclude orbital-preview-server` |
| clippy | `RUSTFLAGS="" cargo clippy --workspace --all-targets -- -D warnings` |
| audit | `cargo audit` |
| deny | `cargo deny check` |
| leptos-lints | `RUSTFLAGS="-D warnings -Zcrate-attr=feature(stdarch_x86_avx512)" cargo dylint --all` (toolchain `nightly-2025-05-14` with `rustc-dev` and `llvm-tools-preview`, `cargo-dylint` 4.1.0) |
| e2e | the preview catalog's Playwright suite against a `--release` build (see below) |

## Running it quickly

The jobs don't depend on each other, so run them at the same time, each with its own
target directory so no job waits on another's build lock:

```bash
CARGO_TARGET_DIR=target-build  cargo build --workspace &
CARGO_TARGET_DIR=target-test   cargo test --workspace --lib --tests \
  --exclude orbital-preview-frontend --exclude orbital-preview-server &
CARGO_TARGET_DIR=target-clippy RUSTFLAGS="" cargo clippy --workspace --all-targets -- -D warnings &
CARGO_TARGET_DIR=target-lints  cargo dylint --all &
cargo fmt --all -- --check; cargo audit; cargo deny check
wait
```

Each extra target directory costs disk (roughly 5 to 15 GB each) and a full rebuild, so
keep one per job, not one per crate. On a machine with little memory, run fewer at once.

### Browser tests in lanes

`end2end/run-lanes.sh` builds the catalog once, then splits the suite across several
preview servers and Playwright workers ("lanes"), one server and one browser per lane:

```bash
cd end2end
./run-lanes.sh                         # release build, npm ci, 14 lanes, merged summary
./run-lanes.sh --no-build --no-install # rerun the lanes on the same build
LANES=8 ./run-lanes.sh --no-build      # fewer lanes on a smaller machine
./run-lanes.sh --serial --no-build     # one server, every test in order
```

The settings live in `end2end/lanes.conf`; `end2end/lanes/list-<n>.txt` hold the test
lists, balanced by measured test time. A test that no list names is dealt into a lane in
turn, so a new spec still runs. The run writes `lanes-out/<stamp>/summary.md` and exits
non-zero on any failed, missing or duplicated test.

To rebalance the lists after the suite changes a lot, run the lanes once, merge the lanes'
`results.json` files into one Playwright JSON report, and partition it:

```bash
python3 -I lanes.py partition --results merged.json --n 14 --out lanes
```

`cargo leptos end-to-end --release --project orbital-preview` (what CI runs) still works,
but it runs the whole suite on one server.

### One change at a time

While fixing something, rebuild and run only the specs that cover it. The debug settings
in `end2end/lanes-debug.conf` rebuild a one-file change in about 30 s:

```bash
cd end2end
LANES_CONF=lanes-debug.conf ./run-lanes.sh tests/components/card.spec.ts   # build, one spec file
LANES_CONF=lanes-debug.conf ./run-lanes.sh --no-build -g "headerless"      # titles that match
cargo test -p orbital-core-components --features ssr --lib card            # SSR unit tests
```

Run the full lanes and the CI jobs once, at the end.

## Measured times

On a 16 vCPU, 30 GB Linux host with a warm compiler cache, November 2026:

| Step | Time |
|---|---|
| Every Rust job above at the same time | 15 min, set by the build job (clippy 4.5, leptos-lints 6, test 9) |
| Release catalog build | 5 min |
| Full browser suite (1,225 tests), 14 lanes, release build | 2.2 min |
| Same suite, 10 lanes, release build | 2.8 min |
| Same suite, 10 lanes, debug build | 5.6 min |
| Debug rebuild after a one-file change, then one spec file | under 1 min |

Placeholder photos in the catalog come from external hosts. `playwright.config.ts`
resolves those hosts to localhost, so a slow photo host can't stall the tests that wait
for the network to go idle.
