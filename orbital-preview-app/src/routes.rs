use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;
use orbital::{OrbitalBootLoaderHeadAssets, OrbitalBootOverlay, OrbitalFirstPaintHeadAssets};
use orbital_style::StyleRegistry;
use orbital_theme::ThemeMode;

use crate::preview::{
    DebugBarePreviewPage, FoucSmokePage, PreviewCatalogShell, PreviewIndexPage, PreviewSlugPage,
};
use crate::site_base::{preview_asset_path, preview_site_base};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router base=preview_site_base()>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <Route path=path!("debug/fouc-smoke") view=FoucSmokePage />
                <Route path=path!("debug/*slug") view=DebugBarePreviewPage />
                <ParentRoute path=path!("") view=PreviewCatalogShell>
                    <Route path=path!("") view=PreviewIndexPage />
                    <Route path=path!("/*slug") view=PreviewSlugPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// SSR document shell for the preview server fallback handler.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    let hydration_root = preview_site_base().to_string();
    let stylesheet_href = preview_asset_path("pkg/orbital-preview.css");

    view! {
        <StyleRegistry>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <OrbitalFirstPaintHeadAssets base_path=preview_site_base().to_string() />
                    <OrbitalBootLoaderHeadAssets />
                    <meta name="orbital-style"/>
                    <Title text="Orbital Preview"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options root=hydration_root />
                    <Stylesheet id="orbital-preview" href=stylesheet_href />
                    <MetaTags/>
                </head>
                <body style="margin: 0;">
                    <App />
                    <OrbitalBootOverlay theme_mode=ThemeMode::Dark />
                </body>
            </html>
        </StyleRegistry>
    }
}
