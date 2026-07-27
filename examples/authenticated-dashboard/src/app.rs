//! Dashboard shell with auth gates and theme/density controls.

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use orbital::components::{
    AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, Card, CardContent, CardHeader,
    CardHeaderDescription, Subtitle1, Title3,
};
use orbital::routes::RequireAuthenticated;
use orbital::{
    init_auth_resource, orbital_shell_with_meta, provide_auth_context, AuthSession,
    AuthenticatedUser, OrbitalDocumentMeta,
};
use orbital_primitives::Switch;
use orbital_theme::{Density, OrbitalThemeProvider, Theme, ThemeMode};
#[cfg(feature = "hydrate")]
use orbital_theme::{set_density, set_theme_mode};

use crate::theme_prefs::{load_prefs, persist_theme_effect, DensityWire, ThemeModeWire, ThemePrefs};

/// SSR document shell.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    orbital_shell_with_meta(
        options,
        OrbitalDocumentMeta {
            title: "authenticated-dashboard",
            favicon_href: "/favicon.ico",
            apple_touch_icon_href: None,
        },
        || view! { <App/> },
    )
}

/// Root: auth context + theme provider + routes.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let auth = provide_auth_context(AuthSession::default());
    let _auth_resource = init_auth_resource(auth.clone());

    let prefs = load_prefs();
    let theme = RwSignal::new(prefs.to_theme());
    persist_theme_effect(theme);

    view! {
        <OrbitalThemeProvider theme=theme>
            <Router>
                <DashboardChrome auth=auth theme=theme />
                <main style="padding: 24px; max-width: 720px; margin: 0 auto;">
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("dashboard") view=ProtectedDashboard/>
                    </Routes>
                </main>
            </Router>
        </OrbitalThemeProvider>
    }
}

#[component]
fn DashboardChrome(auth: orbital::AuthContext, theme: RwSignal<Theme>) -> impl IntoView {
    let session = auth.session();

    view! {
        <AppBar>
            <AppBarLeading slot>
                <Title3>"authenticated-dashboard"</Title3>
            </AppBarLeading>
            <AppBarTrailing slot>
                <ThemeDensityControls theme=theme />
                <AuthControls auth=auth session=session />
            </AppBarTrailing>
        </AppBar>
    }
}

#[component]
fn ThemeDensityControls(theme: RwSignal<Theme>) -> impl IntoView {
    let dark = RwSignal::new(theme.get_untracked().mode == ThemeMode::Dark);
    let compact = RwSignal::new(theme.get_untracked().options.density == Density::Compact);

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        dark.set(theme.with(|t| t.mode == ThemeMode::Dark));
        compact.set(theme.with(|t| t.options.density == Density::Compact));
    });

    #[cfg(feature = "hydrate")]
    Effect::new(move |prev: Option<(bool, bool)>| {
        let is_dark = dark.get();
        let is_compact = compact.get();
        if prev.is_some() && prev != Some((is_dark, is_compact)) {
            set_theme_mode(
                theme,
                if is_dark {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                },
            );
            set_density(
                theme,
                if is_compact {
                    Density::Compact
                } else {
                    Density::Default
                },
            );
        }
        (is_dark, is_compact)
    });

    view! {
        <div style="display: flex; gap: 12px; align-items: center;">
            <div data-testid="theme-dark-toggle">
                <Switch bind=dark label="Dark" />
            </div>
            <div data-testid="theme-compact-toggle">
                <Switch bind=compact label="Compact" />
            </div>
        </div>
    }
}

#[component]
fn AuthControls(
    auth: orbital::AuthContext,
    session: RwSignal<AuthSession>,
) -> impl IntoView {
    view! {
        {move || match session.get() {
            AuthSession::Anonymous(_) => view! {
                <div data-testid="sign-in">
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=Callback::new({
                            let auth = auth.clone();
                            move |_| {
                                auth.session().set(AuthSession::Authenticated(AuthenticatedUser {
                                    user_id: "demo-user".into(),
                                    email: Some("demo@example.com".into()),
                                    display_name: Some("Demo User".into()),
                                    avatar_url: None,
                                    roles: vec!["member".into()],
                                    email_verified: true,
                                }));
                                auth.trigger_refresh();
                            }
                        })
                    >
                        "Sign in"
                    </Button>
                </div>
            }.into_any(),
            AuthSession::Authenticated(user) => {
                let label = user
                    .display_name
                    .clone()
                    .unwrap_or_else(|| user.user_id.clone());
                view! {
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <span data-testid="signed-in-as">{format!("Signed in as {label}")}</span>
                        <div data-testid="sign-out">
                            <Button
                                appearance=ButtonAppearance::Secondary
                                on_click=Callback::new({
                                    let auth = auth.clone();
                                    move |_| {
                                        auth.session().set(AuthSession::default());
                                        auth.trigger_refresh();
                                    }
                                })
                            >
                                "Sign out"
                            </Button>
                        </div>
                    </div>
                }
                .into_any()
            }
        }}
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div data-testid="home-card">
            <Card>
                <CardHeader>
                    <Title3>"Public home"</Title3>
                    <CardHeaderDescription slot>
                        "Open /dashboard while signed out to see RequireAuthenticated. Sign in, then revisit."
                    </CardHeaderDescription>
                </CardHeader>
                <CardContent>
                    <a href="/dashboard" data-testid="goto-dashboard">"Go to protected dashboard →"</a>
                </CardContent>
            </Card>
        </div>
    }
}

#[component]
fn ProtectedDashboard() -> impl IntoView {
    view! {
        <RequireAuthenticated>
            <div data-testid="dashboard-card">
                <Card>
                    <CardHeader>
                        <Title3>"Protected dashboard"</Title3>
                        <CardHeaderDescription slot>
                            "Visible only when AuthSession::Authenticated. Theme/density prefs persist in localStorage."
                        </CardHeaderDescription>
                    </CardHeader>
                    <CardContent>
                        <Subtitle1>"Welcome — auth gate passed."</Subtitle1>
                        <PrefsSummary />
                    </CardContent>
                </Card>
            </div>
        </RequireAuthenticated>
    }
}

#[component]
fn PrefsSummary() -> impl IntoView {
    let theme = Theme::use_rw_theme();
    view! {
        <p data-testid="prefs-summary">
            {move || {
                let prefs = ThemePrefs::from_theme(&theme.get());
                let mode = match prefs.mode {
                    ThemeModeWire::Light => "light",
                    ThemeModeWire::Dark => "dark",
                };
                let density = match prefs.density {
                    DensityWire::Compact => "compact",
                    DensityWire::Default => "default",
                    DensityWire::Spacious => "spacious",
                };
                format!("Theme: {mode} · Density: {density}")
            }}
        </p>
    }
}
