use leptos::prelude::*;
use leptos_router::components::Outlet;
use orbital::components::{
    AppBar, AppBarDensity, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing,
    BackToTop, Container, Layout, LayoutHeader, LayoutMain, LayoutSidebar, LayoutSidebarToggle,
    MaterialCorners, MaterialElevation, MaterialVariant, SidebarPresentation, Title3,
};
use orbital::{init_auth_resource, provide_auth_context, AuthSession};
use orbital_theme::{Density, OrbitalThemeProvider, Theme, ThemeMode, ThemeOverrides};

use super::{PreviewCatalogNav, PreviewCatalogSearch, PreviewThemeToggle, PreviewToolbarLinks};

/// Full-page catalog shell: AppBar, sidebar nav, and centered content outlet.
#[component]
pub fn PreviewCatalogShell() -> impl IntoView {
    let auth = provide_auth_context(AuthSession::default());
    let _auth_resource = init_auth_resource(auth);
    let theme = RwSignal::new(Theme::custom(
        ThemeMode::Dark,
        ThemeOverrides {
            density: Some(Density::Compact),
            ..Default::default()
        },
    ));

    view! {
        <OrbitalThemeProvider theme=theme>
            <Layout
                overlay_header=true
                header_inset=AppBarDensity::Compact
                sidebar_presentation=SidebarPresentation::Auto
                data_testid="preview-catalog-shell"
            >
                <LayoutHeader slot>
                    <AppBar position=AppBarPosition::Sticky density=AppBarDensity::Compact>
                        <AppBarMaterial
                            variant=MaterialVariant::Frost
                            elevation=MaterialElevation::Flat
                            corners=MaterialCorners::Square
                            slot
                        />
                        <AppBarLeading slot>
                            <LayoutSidebarToggle />
                            <Title3>"Orbital Components"</Title3>
                        </AppBarLeading>
                        <PreviewCatalogSearch />
                        <AppBarTrailing slot>
                            <PreviewToolbarLinks />
                            <PreviewThemeToggle />
                        </AppBarTrailing>
                    </AppBar>
                </LayoutHeader>
                <LayoutSidebar slot>
                    <PreviewCatalogNav />
                </LayoutSidebar>
                <LayoutMain slot>
                    <Container max_width="1200px".to_string()>
                        <Outlet />
                        <BackToTop
                            right=Signal::from(24)
                            bottom=Signal::from(24)
                            testid="back-to-top-button"
                        />
                    </Container>
                </LayoutMain>
            </Layout>
        </OrbitalThemeProvider>
    }
}
