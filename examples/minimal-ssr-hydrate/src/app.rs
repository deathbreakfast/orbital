//! App shell and home page for the minimal SSR+hydrate host.

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use orbital::components::{
    Button, ButtonAppearance, Card, CardContent, CardFooter, CardHeader, CardHeaderDescription,
    Subtitle1, Title3,
};
use orbital::{
    orbital_shell_with_meta, OrbitalDocumentMeta, OrbitalTemplate,
};

/// SSR document shell — [`orbital_shell_with_meta`] injects first-paint CSS + boot overlay.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    orbital_shell_with_meta(
        options,
        OrbitalDocumentMeta {
            title: "minimal-ssr-hydrate",
            favicon_href: "/favicon.ico",
            apple_touch_icon_href: None,
        },
        || view! { <App/> },
    )
}

/// Root app: theme provider + a single route proving hydrate.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <OrbitalTemplate>
            <Router>
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </Router>
        </OrbitalTemplate>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let clicks = RwSignal::new(0u32);

    view! {
        <main
            data-testid="minimal-ssr-hydrate-home"
            style="padding: 24px; max-width: 420px; margin: 0 auto;"
        >
            <Card>
                <CardHeader>
                    <Title3>"Orbital SSR + hydrate"</Title3>
                    <CardHeaderDescription slot>
                        "Boot overlay dismisses after WASM hydrate; click proves client reactivity."
                    </CardHeaderDescription>
                </CardHeader>
                <CardContent>
                    <div data-testid="click-count">
                        <Subtitle1>
                            {move || format!("Clicks: {}", clicks.get())}
                        </Subtitle1>
                    </div>
                </CardContent>
                <CardFooter>
                    <div data-testid="increment">
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=Callback::new(move |_| clicks.update(|n| *n += 1))
                        >
                            "Increment"
                        </Button>
                    </div>
                </CardFooter>
            </Card>
        </main>
    }
}
