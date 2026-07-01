//! Getting Started preview page for the bootstrap loading overlay.

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::components::{
    Body1, Flex, FlexGap, Material, MaterialElevation, MaterialVariant, Subtitle1, TextTag,
    Title1,
};
use crate::shell::{OrbitalBootErrorContent, OrbitalBootLoadingPanel};
use orbital_theme::ThemeMode;

const CODE_BLOCK_STYLE: &str = "display: block; margin: 0; padding: 12px; border-radius: 8px; background: var(--orb-color-surface-subtle, #f5f5f5); color: var(--orb-color-text-primary, #232425); font-family: var(--orb-type-family-monospace, ui-monospace, monospace); font-size: 14px; line-height: 18px; white-space: pre-wrap; overflow-x: auto;";
const FRAME_STYLE: &str = "padding: 20px;";
const ERROR_DEMO_STYLE: &str = "width: min(100%, 32rem); margin-inline: auto;";

const SHELL_SNIPPET: &str = r#"<OrbitalFirstPaintHeadAssets />
<OrbitalBootLoaderHeadAssets />
<HydrationScripts options />

<body>
    {app_fn()}
    <OrbitalBootOverlay theme_mode=ThemeMode::Dark />
</body>"#;

const HYDRATE_SNIPPET: &str = r#"std::panic::set_hook(Box::new(|info| {
    orbital::hide_boot_loader();
    console_error_panic_hook::hook(info);
}));

leptos::mount::hydrate_body(App);
orbital::hide_boot_loader();"#;

#[component]
fn BootLoaderLoadingDemo() -> impl IntoView {
    view! {
        <div data-testid="boot-loader-loading-demo">
            <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
                <div style=FRAME_STYLE>
                    <OrbitalBootLoadingPanel
                        title="Loading application…".to_string()
                        theme_mode=ThemeMode::Dark
                    />
                </div>
            </Material>
        </div>
    }
}

#[component]
fn BootLoaderErrorDemo() -> impl IntoView {
    view! {
        <div data-testid="boot-loader-error-demo" style=ERROR_DEMO_STYLE>
            <OrbitalBootErrorContent theme_mode=ThemeMode::Dark />
        </div>
    }
}

/// Getting Started guide for the WASM-free bootstrap loading overlay.
///
/// Covers document-shell wiring, the hydrate entrypoint, and static loading/error UI.
///
/// # Examples
///
/// ## Guide page
/// Full Getting Started walkthrough with wiring snippets and static loading/error demos.
/// <!-- preview -->
/// ```rust
/// view! { <BootLoaderGuidePreview /> }
/// ```
#[component_doc(
    section = "Getting Started",
    nav_item = true,
    preview_slug = "boot-loader",
    preview_label = "Boot loader",
)]
#[component]
pub fn BootLoaderGuidePreview() -> impl IntoView {
    view! {
        <>
            <Title1 tag=TextTag::H1 block=true test_id="preview-page-title">
                "Boot loader"
            </Title1>
            <Body1 block=true>
                "While the WASM bundle downloads and Leptos hydrates, show a static full-viewport overlay instead of an empty or half-styled page. The boot loader is SSR-safe: inline CSS, plain HTML, and Leptos components that do not require hydration."
            </Body1>

            <Subtitle1 block=true>"Wiring checklist"</Subtitle1>
            <Flex vertical=true gap=FlexGap::Small>
                <Body1 block=true>
                    "1. Add "
                    <code>"<OrbitalBootLoaderHeadAssets />"</code>
                    " in "
                    <code>"<head>"</code>
                    " after "
                    <code>"<OrbitalFirstPaintHeadAssets />"</code>
                    ", before "
                    <code>"<HydrationScripts>"</code>
                    "."
                </Body1>
                <Body1 block=true>
                    "2. Add "
                    <code>"<OrbitalBootOverlay />"</code>
                    " in "
                    <code>"<body>"</code>
                    " after your app root. The hydrated app must stay the first body child."
                </Body1>
                <Body1 block=true>
                    "3. Call "
                    <code>"orbital::hide_boot_loader()"</code>
                    " immediately after "
                    <code>"leptos::mount::hydrate_body(...)"</code>
                    " in every "
                    <code>"hydrate()"</code>
                    " export."
                </Body1>
            </Flex>

            <Subtitle1 block=true>"Document shell"</Subtitle1>
            <Body1 block=true>
                <code>"orbital_shell"</code>
                " wires the head assets and overlay automatically. Custom shells match "
                <code>"orbital-preview-app/src/routes.rs"</code>
                ":"
            </Body1>
            <pre style=CODE_BLOCK_STYLE>{SHELL_SNIPPET}</pre>

            <Subtitle1 block=true>"Hydrate entrypoint"</Subtitle1>
            <Body1 block=true>
                "Set a panic hook that calls "
                <code>"hide_boot_loader()"</code>
                " so startup failures do not leave the overlay trapping the page. See "
                <code>"orbital-preview-frontend/src/lib.rs"</code>
                ":"
            </Body1>
            <pre style=CODE_BLOCK_STYLE>{HYDRATE_SNIPPET}</pre>

            <Subtitle1 block=true>"Loading state"</Subtitle1>
            <Body1 block=true>
                "A modal dialog with an overall progress bar, elapsed time, per-step durations, and boot step checklist renders inside "
                <code>"#orbital-boot-overlay"</code>
                " until hydration completes. Completed steps show frozen durations on the right; the active step shows live elapsed time; pending steps show "
                <code>"—"</code>
                ". On success, the backdrop fades out and the modal card fades with a slight scale (motion-aligned exit); "
                <code>"prefers-reduced-motion"</code>
                " skips the animation."
            </Body1>
            <BootLoaderLoadingDemo />

            <Subtitle1 block=true>"Error state"</Subtitle1>
            <Body1 block=true>
                "If JS or WASM fails to load, inline listeners set "
                <code>"html[data-orbital-boot-state=\"error\"]"</code>
                " and reveal a static dialog surface with "
                <code>"MessageBar"</code>
                " — not the hydrated "
                <code>"Dialog"</code>
                " component (portal/focus trap)."
            </Body1>
            <BootLoaderErrorDemo />

            <Subtitle1 block=true>"Constraints"</Subtitle1>
            <Flex vertical=true gap=FlexGap::Small>
                <Body1 block=true>
                    "Keep the boot loader WASM-free — no "
                    <code>"LoadingBar"</code>
                    ", reactive "
                    <code>"ProgressBar"</code>
                    ", or full "
                    <code>"Dialog"</code>
                    ". Progress bar styling uses shared CSS tokens only; width is updated by inline JS."
                </Body1>
                <Body1 block=true>
                    "Place "
                    <code>"<OrbitalBootOverlay />"</code>
                    " after the app root, not before it, to avoid hydration DOM mismatches."
                </Body1>
                <Body1 block=true>
                    "Post-hydration Rust panics are logged only unless you add custom handling; call "
                    <code>"hide_boot_loader()"</code>
                    " from your panic hook when WASM did start."
                </Body1>
            </Flex>
        </>
    }
}
