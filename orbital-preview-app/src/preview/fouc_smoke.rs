//! Minimal FOUC smoke page — single primary button, default light theme.

use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance};
use orbital_theme::OrbitalThemeProvider;

/// Bare page for Playwright first-paint / WASM-delay FOUC regression tests.
#[component]
pub fn FoucSmokePage() -> impl IntoView {
    view! {
        <OrbitalThemeProvider>
            <main data-testid="fouc-smoke-root" style="min-height: 100vh; padding: 16px; box-sizing: border-box;">
                <div data-testid="fouc-smoke-button">
                    <Button appearance=ButtonAppearance::Primary>"FOUC smoke"</Button>
                </div>
            </main>
        </OrbitalThemeProvider>
    }
}
