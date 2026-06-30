# Orbital

Leptos UI component library and design system facade — shell, themed primitives, auth hooks, and preview catalog registration.

## Quick start

```toml
[dependencies]
orbital = { git = "https://github.com/unified-field-dev/orbital", default-features = false, features = ["hydrate"] }
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
        </OrbitalTemplate>
    }
}
```

## Documentation

| Resource | Description |
|----------|-------------|
| [Introduction](https://unified-field-dev.github.io/orbital/) | Design principles, tokens, typography, motion |
| [Theme](https://unified-field-dev.github.io/orbital/theme) | Mode, density, CSS variables |
| [Component preview](https://unified-field-dev.github.io/orbital/) | Full interactive catalog |

Local catalog: `cargo leptos watch -p orbital-preview` → `http://127.0.0.1:3010/orbital/{slug}`

## Features

| Feature | Purpose |
|---------|---------|
| `default-features = false` | Production builds without preview catalog deps |
| `hydrate` | Client hydration + theme/style/shell |
| `ssr` | Server rendering with Axum integration |
| `preview` | `#[component_doc]` catalog metadata (doc host only) |

Use `default-features = false` and enable only the features you need. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).

## Design language

Orbital defines spacing tokens, typography presets, elevation ramps, and surface materials authors compose through shared components. Read the live [Introduction](https://unified-field-dev.github.io/orbital/) for principles and token chapters.

## Composition

- **Shell** — [`OrbitalTemplate`] wraps apps with theme and layout primitives.
- **Boot loader** — [`orbital_shell`] injects a WASM-free loading overlay until hydration completes (see below).
- **Auth** — [`AuthContext`] + [`init_auth_resource`] for session state; the preview catalog uses a stub, while integrators wire a real session server function.
- **Preview catalog** — `#[component_doc]` emits [`PreviewRegistration`] entries; run `cargo leptos watch -p orbital-preview` and browse `/orbital/{slug}` on `:3010`.

Capability flags on feature crates (for example `DataTableFeatures`, `ChartFeatures`) express optional behavior — not license-tier suffixes on type names.

## Boot loader

While `/pkg/*.wasm` downloads and `hydrate()` runs, users see a static bootstrap overlay instead of an unstyled, non-interactive page.

### Using [`orbital_shell`]

[`orbital_shell`] already wires:

- [`OrbitalBootLoaderHeadAssets`] — inline critical CSS and a script error listener in `<head>`
- [`OrbitalBootOverlay`] — `#orbital-boot-overlay` in `<body>` **after** your app root (see [Boot loader](https://unified-field-dev.github.io/orbital/boot-loader) in the preview catalog)

Your WASM entrypoint **must** call [`hide_boot_loader`] immediately after hydration:

```rust,ignore
#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
    orbital::hide_boot_loader();
}
```

### Custom document shells

If you do not use [`orbital_shell`], add the same pieces manually:

1. `<OrbitalBootLoaderHeadAssets />` in `<head>` after [`OrbitalFirstPaintHeadAssets`], **before** `<HydrationScripts>`
2. `<OrbitalBootOverlay />` in `<body>` **after** `{app_fn()}` (app root must remain the first body child for hydration)
3. Call [`hide_boot_loader`] after `hydrate_body` in your `hydrate()` export

See [`orbital-preview-app/src/routes.rs`](../orbital-preview-app/src/routes.rs) and [`orbital-preview-frontend/src/lib.rs`](../orbital-preview-frontend/src/lib.rs) for the in-repo reference implementation. The [Boot loader](https://unified-field-dev.github.io/orbital/boot-loader) Getting Started page in the preview catalog walks through wiring and shows static loading/error demos.

### Load failures

Leptos `HydrationScripts` does not catch failed JS/WASM loads. [`OrbitalBootLoaderHeadAssets`] registers inline `error` and `unhandledrejection` listeners that set `html[data-orbital-boot-state="error"]` and reveal a static error panel: dialog layout primitives (`DialogBody`, `DialogTitle`, `DialogContent`) and a [`MessageBar`] inside a fixed dialog surface — not the hydrated [`Dialog`] component (portal/focus trap).

Rust panics **after** WASM is running are logged via `console_error_panic_hook` only unless you add custom panic handling. Set a panic hook that calls [`hide_boot_loader`] first (see `orbital-preview-frontend/src/lib.rs`) so startup failures do not leave the overlay trapping the page.

Do **not** use hydrated components such as `LoadingBar` or `ProgressBar` for this phase — they require WASM.

See the [repository README](../README.md) for workspace layout, testing, and development.
