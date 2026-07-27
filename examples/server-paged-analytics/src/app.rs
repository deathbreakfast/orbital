//! App shell for server-paged analytics.

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use orbital::components::Title3;
use orbital::{orbital_shell_with_meta, OrbitalDocumentMeta, OrbitalTemplate};

use crate::data::AnalyticsPage;

/// SSR document shell.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    orbital_shell_with_meta(
        options,
        OrbitalDocumentMeta {
            title: "server-paged-analytics",
            favicon_href: "/favicon.ico",
            apple_touch_icon_href: None,
        },
        || view! { <App/> },
    )
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <OrbitalTemplate>
            <Router>
                <header style="padding: 16px 24px;">
                    <Title3>"server-paged-analytics"</Title3>
                </header>
                <main style="padding: 0 24px 24px; max-width: 960px; margin: 0 auto;">
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=StaticSegment("") view=AnalyticsPage/>
                    </Routes>
                </main>
            </Router>
        </OrbitalTemplate>
    }
}
