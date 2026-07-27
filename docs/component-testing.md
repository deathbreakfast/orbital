# Component testing in Orbital

Orbital uses a three-layer testing model. Each layer validates a different slice of the component pipeline.

## Layer overview

| Layer | Tool | Validates | Typical command |
|-------|------|-----------|-----------------|
| **L1 — Macro / compile** | `#[component_doc]` + `component-preview-e2e` | Doc constants, props extraction, preview codegen | `cargo test -p component-preview-e2e` |
| **L2 — Preview host** | Slim Leptos server on `:3010` | SSR + hydrate render of registry previews | `cargo leptos watch --split` (orbital-preview metadata) |
| **L3 — Browser E2E** | Playwright | Visible variants, interaction smoke, testid hooks | `cargo leptos end-to-end --project orbital-preview` (or `COMPONENT_PREVIEW_E2E=1 npm test` with server already on `:3010`) |

## Authoring previews

Register components with `#[component_doc]` in the owning crate:

```rust
#[component_doc(
    section = "Core",
    category = "Data Display",
    preview_slug = "paginator",
    preview_label = "Paginator",
)]
#[component]
pub fn Paginator(/* ... */) -> impl IntoView { /* ... */ }
```

The macro emits `{NAME}_DOC`, `{NAME}_PROPS`, and (when `preview_slug` is set) a `PreviewRegistration` entry. All doc/preview artifacts are gated behind the crate's `preview` feature; with `preview` off the macro passes the component through unchanged.

Manual previews (`preview = "manual"`) register through a static `PreviewRegistration` in the fixture or preview crate — see `component-preview-e2e/src/fixtures.rs`.

## Component authoring

Documentation lives in **rustdoc comments** on the component function and its parameters. The `#[component_doc]` macro extracts props, sections, and examples into the preview catalog.

### Prop rustdoc

Every parameter shown in the Properties tab needs a `///` line — including exempt props (`class`, `children`, `data_testid`).

| Prop pattern | Guidance |
|--------------|----------|
| `class` | Extra CSS class names merged onto the root element (name the element when helpful: Material root, flex container, etc.). |
| `children` | Context-specific: compound slot regions for layout shells; flex item children for `Flex` / `Stack` / `Space`. |
| Spacing (`padding`, `margin`, `gap`) | Theme-aware: `SpacingInset::all_l()`; fixed px ramp: `SpacingSize::Size200.inset()`; CSS vars like `var(--spacingHorizontalM)` in examples. |
| Signals / callbacks | What state they control and when the parent must own them. |
| Slot props | Which child component fills the slot and what it renders. |

Semantic props should explain what changes visually or behaviorally, valid values, and when to use each — see `Button` in `orbital-core-components/src/button/button.rs`.

### Example descriptions

Under `# Examples`, each live variant uses a `## Title` heading, one or two sentences of prose, then markers and a fence:

```markdown
## Variant title
What the preview shows, which props or features it demonstrates, and when to use this pattern.
<!-- preview -->
```rust
view! { /* ... */ }
```
```

- Describe behavior, not just restate the title.
- Call out non-obvious props exercised in the snippet.
- For layout examples, mention spacing approach (`Flex` `padding` vs wrapper `div` vs CSS vars) — see Layout below.
- `<!-- code-only -->` examples should also include prose when they appear in rustdoc.

#### How-to vs UI demo examples

The **Show code** panel renders the rustdoc fence verbatim. Match the example type to what developers need:

| Example type | Live `<!-- preview -->` fence | Notes |
|--------------|-------------------------------|-------|
| **How-to / integration / feature recipe** | Full wiring: imports, props, callbacks, `data-testid` | Do **not** delegate to `<FooPreview />`, `crate::...::foo_preview()`, or a hidden module — the fence is the developer reference |
| **UI demo / design tool** | Self-mounting shell OK | Theme designer, brand ramp gallery, and similar visual exploration pages |
| **Secondary pattern** | `<!-- code-only -->` | Handle callbacks, out-of-tree consumers, or patterns awkward to live-render — always pair with prose (see `button/button.rs`) |

Module-level helpers (fixtures, axis builders) may stay beside the doc comment, but the fence must show how they are called — not hide the call site behind a wrapper component.

Gold-standard references: `button/button.rs`, `space/space.rs`, `flex/flex.rs`, `orbital-datatable/.../sorting_filtering.rs`, `orbital-charts/.../bar_chart/root.rs`.

Maintainers may run `python3 scripts/maintainer/check_component_docs.py` before opening PRs that touch component docs.

## `data-testid` conventions

| Pattern | Example | Notes |
|---------|---------|-------|
| Default preview wrapper | `{slug}-preview` | On a **native** HTML element (`div`, `span`) |
| Named variant | `{slug}-{variant-kebab}` | e.g. `button-secondary` |
| Not found page | `preview-not-found` | Preview host unknown slug |
| Teleported UI | inside portal content | Dialog: put testids on `DialogContent`, not empty outer wrappers |

Orbital components do **not** forward arbitrary attributes — wrap controls in a native element when you need a test hook.

### Layout: `Flex` vs wrapper `div`

Native `Flex` has no `style` prop. Split responsibilities:

| Concern | Use |
|---------|-----|
| Direction, gap, align, justify, wrap | `<Flex>` props |
| Padding, margin (theme-aware) | `padding` / `margin` with [`SpacingInset`] + [`SpacingHorizontal`] / [`SpacingVertical`] |
| Padding (design-language px ramp) | `padding=SpacingSize::Size200.inset()` |
| Border, background, fixed height on a frame | Native wrapper `div` **without** `display: flex` |
| `height: 100%`, full width, wrap on the flex container | `fill`, `full_width`, `wrap=FlexWrap::Wrap` |
| Flex item sizing (`flex: 1`, `flex-shrink: 0`) | Native child `div` inside `Flex` |

```rust
// Preview frame: chrome on div, layout on Flex
view! {
    <div style="height: 420px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
        <Flex fill=true>
            <div style="flex: 1; overflow: auto;">{canvas()}</div>
            <div style="width: 320px; overflow: auto;">{sidebar()}</div>
        </Flex>
    </div>
}

// Theme-aware card body padding on Flex (responds to density)
view! {
    <Flex vertical=true gap=FlexGap::Medium padding=SpacingInset::all_l()>
        {children()}
    </Flex>
}

// Design-language px ramp (fixed, non-theme)
view! {
    <Flex vertical=true padding=SpacingSize::Size200.inset()>
        {children()}
    </Flex>
}
```

Avoid `<div style="display: flex; ...">` when `Flex` props cover the layout.

### Preview catalog shell (`:3010`)

All `/orbital/*` routes render inside `PreviewCatalogShell` (`data-testid="preview-catalog-shell"`):

- **Density** — compact end-to-end: `Density::Compact` theme tokens, `AppBarDensity::Compact` (48px) with matching `Layout` `header_inset`, and `NavigationDensity::Compact` sidebar rows
- **AppBar** (`data-testid="app-bar"`) fixed overlay with Shell/Flat `AppBarMaterial` — main content scrolls beneath the chrome bar; `data-testid="preview-theme-toggle"` (dark mode switch)
- **Sidebar** `Navigation` with registry categories (`data-testid="preview-catalog-nav"`)
- **Main** content in `Container` + `<Outlet />`

`waitForPreviewShell` in [`helpers.ts`](../end2end/tests/helpers.ts) waits for the catalog shell before asserting preview content.

### Preview example frames

Each documented example uses **one** elevated `Card` frame (`OrbitalPreviewCardBody` / `PrimitivePreviewCardBody`):

| Region | Structure | Notes |
|--------|-----------|-------|
| Section header | `<section>` + `Subtitle1` + optional description | No elevation |
| Demo area | `Card` → `CardContent` | `max-width: 700px`, `--orb-color-surface-card` |
| Toolbar | `CardContent` | Show/Hide code button, right-aligned |
| Code panel | `CardPreview` (full-bleed) → `CardContent` → `Code` | `--orb-color-surface-muted` to card edges; text padded only |

Use Orbital theme CSS variables for all preview chrome (`--orb-color-surface-*`, `--orb-color-border-*`, spacing tokens). No hardcoded hex colors in preview stylesheets.

Place `data-testid` on **demo wrapper elements** inside the preview area, not on the frame `Card`.

## Playwright helpers

Shared helpers live in [`end2end/tests/_preview.ts`](../end2end/tests/_preview.ts):

- `openComponentPreview(page, slug)` — navigates to `/orbital/{slug}`, waits for `preview-catalog-shell`, then `{slug}-preview`
- `expectPreviewVariants(page, testIds)` — asserts variant wrappers are visible

Specs gate on `COMPONENT_PREVIEW_E2E=1` (set automatically by `npm test` / `cargo leptos end-to-end --project orbital-preview`).

## Layer 3c — Doc panel E2E

The unified preview shell (`OrbitalComponentView`) exposes stable hooks for macro-generated doc content:

| Test id | Element |
|---------|---------|
| `preview-page-title` | Component catalog title (`Title1`) |
| `preview-doc-panel` | Doc section (title + tabs + content) |
| `preview-doc-content` | Active tab panel body |
| `preview-example-nav` | In-page aside anchor rail listing live examples |

Shared helpers in [`end2end/tests/_preview.ts`](../end2end/tests/_preview.ts):

- `expectPreviewPageTitle(page, label)` — assert page title text
- `expectPreviewDocTabs(page)` — Description / Best Practices / Properties visible
- `clickPreviewDocTab(page, name)` — switch tabs via `role="tab"`
- `expectPreviewDocContent(page, { contains?, notContains?, noPreBlocks? })` — assert panel text; `noPreBlocks` catches fenced-code leaks rendered as `<pre>`

Reference slug matrix in [`preview-shell.spec.ts`](../end2end/tests/components/preview-shell.spec.ts):

| Slug | Validates |
|------|-----------|
| `divider` | Minimal summary-only description |
| `flex` | Full sections; Usage fences excluded from Description |
| `button` | Best Practices + Properties tabs; show-code toggle |
| `layout` | Usage numbered steps without fences |
| `fixture-doc-panel` | Controlled macro fixture (`component-preview-e2e`) |

Extend this matrix when adding new `# Usage` doc patterns — do not duplicate doc-panel checks in every component spec.

Macro regression fixtures live in [`orbital-macros/tests/fixtures/doc_strings/`](../orbital-macros/tests/fixtures/doc_strings/) (`usage_with_fence.txt`, `examples_markers.txt`) and [`component-preview-e2e`](../component-preview-e2e/tests/expansion.rs) (`FIXTUREDOCPANEL_*` constants).

## Layer 3b — Prop, style, and theme verification

Beyond visibility smoke tests, migrated components assert **computed styles**, **ARIA/state attributes**, **click behavior**, and **theme token propagation**.

Shared helpers live in [`end2end/tests/_assertions.ts`](../end2end/tests/_assertions.ts):

- `getCssVariable(page, selector, varName)` — read a CSS custom property from a scoped element
- `expectComputedStyle(page, testId, styles, { childSelector? })` — assert `getComputedStyle` values on a preview wrapper
- `expectThemeVarChange(page, { toggleTestId, scopeSelector, cssVar, before?, after? })` — toggle a theme control and assert the variable changed

Example (Flex prop verification):

```typescript
import { expectComputedStyle } from "../_assertions";

test("vertical prop sets flex-direction column", async ({ page }) => {
  await openComponentPreview(page, "flex");
  const container = page.getByTestId("flex-vertical").locator(".orbital-flex");
  await expect(container).toHaveCSS("flex-direction", "column");
});
```

Example (theme token propagation):

```typescript
import { getCssVariable } from "../_assertions";

test("provider exposes spacing tokens", async ({ page }) => {
  await openComponentPreview(page, "theme");
  const spacing = await getCssVariable(page, ".orbital-theme-provider", "--spacingHorizontalM");
  expect(spacing).toBeTruthy();
});
```

Batch 1 matrices: theme T-01..T-07 in [`theme.spec.ts`](../end2end/tests/components/theme.spec.ts); Flex gap/align in [`flex.spec.ts`](../end2end/tests/components/flex.spec.ts); Button disabled/loading/brand in [`button.spec.ts`](../end2end/tests/components/button.spec.ts); Material variant/elevation in [`material.spec.ts`](../end2end/tests/components/material.spec.ts).

Example (Material elevation verification):

```typescript
test("raised shadow differs from resting", async ({ page }) => {
  await openComponentPreview(page, "material");
  const resting = page.getByTestId("material-preview").locator(".orbital-material").first();
  const raised = page.getByTestId("material-raised-preview").locator(".orbital-material").first();
  const restingShadow = await resting.evaluate((el) => getComputedStyle(el).boxShadow);
  const raisedShadow = await raised.evaluate((el) => getComputedStyle(el).boxShadow);
  expect(raisedShadow).not.toEqual(restingShadow);
});
```

## Spec template

```typescript
test.describe("{slug} preview", () => {
  test.skip(!process.env.COMPONENT_PREVIEW_E2E, "preview server on :3010");

  test("renders default", async ({ page }) => {
    await openComponentPreview(page, "{slug}");
  });
});
```

## App shell smoke — boot loader

Bootstrap overlay behavior is covered outside the per-component preview matrix in [`boot-loader.spec.ts`](../end2end/tests/smoke/boot-loader.spec.ts). These tests hit the preview app shell (not `/orbital/{slug}`) and use Playwright route helpers from [`end2end/tests/lib/wasm-delay.ts`](../end2end/tests/lib/wasm-delay.ts):

| Test | What it validates |
|------|-------------------|
| B-01 | SSR contract — `#orbital-boot-overlay`, head CSS, overlay after app root |
| B-02 / B-03 | `delayWasm(8000)` — spinner visible during slow download, dismisses after hydrate |
| B-04 | Fast path — overlay removed after hydration |
| B-05 | `blockWasm` — error panel (`alertdialog`, MessageBar copy) |

Getting Started catalog wiring for the boot-loader doc page is in [`catalog-shell.spec.ts`](../end2end/tests/components/catalog-shell.spec.ts) (nav link under **Getting Started**, alongside Theme). Open the live guide with `openComponentPreview(page, "boot-loader")`.

## Out of scope

- Auto-generated Playwright from macro output
- Visual regression (Percy/Chromatic)
- Auth-gated preview tests (out of scope for this repo; the `:3010` preview catalog runs without auth)
