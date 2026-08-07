# Orbital Macros

Proc macros for Orbital design-system documentation and routing helpers.

## Macros (this crate)

### `#[component_doc]`

Extracts component documentation and optional preview registration metadata.

```rust
use orbital_macros::component_doc;

#[component_doc(
    category = "Layout",
    preview_slug = "flex",
    preview_label = "Flex",
)]
#[component]
pub fn Flex(/* ... */) -> impl IntoView { /* ... */ }

#[component_doc(
    category = "Surfaces",
    group = "Card",
    preview_slug = "card-header",
    preview_label = "Card Header",
)]
#[component]
pub fn CardHeader(/* ... */) -> impl IntoView { /* ... */ }

#[component_doc(
    section = "Getting Started",
    nav_item = true,
    preview_slug = "theme",
    preview_label = "Theme",
)]
#[component]
pub fn ThemePreview(/* ... */) -> impl IntoView { /* ... */ }
```

Nav metadata fields: `section`, `section_priority`, `category`, `category_priority`,
`category_default_collapsed`, `group`, `group_priority`, `nav_item`. Defaults for
section/category/group ordering live in `category_defaults.rs`.

#### Consumer feature flags

Component crates expose a `preview` feature. When it is **disabled** (the default on
`orbital-core-components`):

- `#[component_doc]` passes the component through without emitting doc/preview tokens.
- Preview-only deps (`inventory`, `pulldown-cmark`) are not required.

When `preview` is **enabled**, the macro emits doc constants, props metadata, preview
wrapper components, and `PreviewRegistration` entries for the catalog.

**Production app (components only):**

```toml
orbital-core-components = { path = "../orbital-core-components", default-features = false }
# or via the public crate:
orbital-ui = { path = "../orbital", package = "orbital-ui", default-features = false, features = ["hydrate"] }
```

**Preview app / maintainer workflows:**

```toml
orbital-ui = { path = "../orbital", package = "orbital-ui", features = ["preview", "hydrate"] }
# or per crate:
orbital-core-components = { path = "../orbital-core-components", features = ["preview"] }
```

Note: `hydrate` and `ssr` on several Orbital crates currently enable `preview` as well.
Use `default-features = false` and enable only the features you need if you want hydrate
without the preview catalog.

### `preview_registrations!`

Exports a crate-local `all() -> &'static [&'static PreviewRegistration]` table for host
catalogs. Bring `PreviewRegistration` into scope, then list the `*_PREVIEW_REGISTRATION`
statics from `#[component_doc]`:

```rust
use orbital_macros::preview_registrations;
use crate::preview::PreviewRegistration;
use crate::status_chip::STATUSCHIP_PREVIEW_REGISTRATION;

preview_registrations! {
    &STATUSCHIP_PREVIEW_REGISTRATION,
}
```

Behind `feature = "preview"` the macro emits the full table; with preview off it emits an
empty `all()`. Product hosts merge with `orbital_primitives::preview::PreviewCatalog` —
see [Component testing — External product catalogs](../docs/component-testing.md#external-product-catalogs).
Keep product registrations in the product crate; do not open PRs against Orbital leaf
`static_registrations` only to surface an external widget.

### `orbital_routes!`

Include multiple route components inside `<Routes>`.

### `#[orbital_routes_extract]`

Generate a typed `paths` module from a route component's `view!` tree.
