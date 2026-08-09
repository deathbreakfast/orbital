use leptos::prelude::*;
use orbital_base_components::{
    use_breakpoint_down, AppBarDensity, AppBarInset, BaseLayout, BaseLayoutBody, LayoutPosition,
    OpenBind,
};
use orbital_macros::component_doc;
use orbital_theme::Breakpoint;

use super::context::LayoutSidebarOpen;
use super::main::LayoutMainShell;
use super::overlay::LayoutOverlayScroll;
use super::sidebar::LayoutSidebarShell;
use super::sidebar_presentation::{SidebarPresentation, DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT};
use super::slots::{LayoutHeader, LayoutMain, LayoutSidebar};
use super::styles::{layout_styles, SIDEBAR_DRAWER_OVERLAY_CSS};
use crate::navigation::{DrawerPosition, DrawerSize, OverlayDrawer};
use crate::ScrollArea;

/// Application shell with optional overlay header and side navigation.
///
/// Compose with [`LayoutHeader`], [`LayoutSidebar`], and [`LayoutMain`] slots. Pair `overlay_header=true` with a Fixed or Sticky [`AppBar`](crate::AppBar) in the header slot.
///
/// # When to use
///
/// - Full-page application shells with header, optional side nav, and scrollable main content - Pinned sticky frost headers where page content scrolls beneath the bar - Opaque fixed headers where content and scrollbar start below the bar (inner scroll) - Inline header layouts without overlay chrome
///
/// # Usage
///
/// 1. Set `overlay_header=true` when the header uses Sticky or Fixed [`AppBar`]. 2. Use Sticky [`AppBar`] with frost material for the default pinned window-scroll shell. 3. Set `main_inset_scroll=true` for opaque fixed bars with inner main scroll below the bar. 4. Match `header_inset` density to the [`AppBar`] density tier. 5. Place navigation in [`LayoutSidebar`] and page content in [`LayoutMain`]. 6. [`LayoutSidebar`] stays pinned below the bar; only the page scrolls in pinned mode.
///
/// # Scroll modes
///
/// | Mode | Props | When |
/// |------|-------|------|
/// | Inline header | default | Header scrolls with page content |
/// | Overlay pinned | `overlay_header=true`, sticky frost [`AppBar`] | Content scrolls beneath the bar |
/// | Overlay inset | `overlay_header=true`, `main_inset_scroll=true`, opaque fixed [`AppBar`] | Main scrolls below the bar inside the shell |
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`MaterialElevation::Flat`] on shell chrome — borders separate regions, not shadows * Flat shell chrome in overlay layouts shares the layout canvas surface (`Background3`), not card fills * Use Sticky [`AppBar`] with Frost or Shell [`AppBarMaterial`] for pinned shells * Pair [`Navigation`] with `Solid` + `Flat` in [`LayoutSidebar`] for co-planar side nav * Use `main_inset_scroll=true` with Solid opaque fixed bars in bounded containers * Keep `header_inset` in sync with [`AppBar`] density * Put long scrolling content in [`LayoutMain`], not the layout root * Reserve `Resting` / `Raised` elevation for in-content cards and callouts, not shell regions
///
/// ## Don'ts
///
/// * Do not use Fixed AppBar with pinned overlay — use Sticky so the header row stays in document flow * Do not add manual top padding in main when the header sits above the body row * Do not use elevated Material tiers on AppBar or Navigation — use Flat with auto border separators
///
/// # Examples
///
/// ## Inline header
/// Header and main in normal document flow—the default stacked shell.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Layout, LayoutHeader, LayoutMain, Title3};
/// view! {
///     <div data-testid="layout-preview" style="height: 200px; border: 1px solid var(--orb-color-border-subtle);">
///         <Layout>
///             <LayoutHeader slot>
///                 <DemoBox data_testid="layout-header-demo"><Title3>"Workspace"</Title3></DemoBox>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <DemoBox fill=true data_testid="layout-main-demo">"Main content"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## With sidebar
/// Side navigation beside the primary content column.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Layout, LayoutMain, LayoutSidebar};
/// view! {
///     <div data-testid="layout-with-sidebar" style="height: 200px; border: 1px solid var(--orb-color-border-subtle);">
///         <Layout>
///             <LayoutSidebar slot>
///                 <DemoBox fill=true data_testid="layout-sidebar-demo">"Nav"</DemoBox>
///             </LayoutSidebar>
///             <LayoutMain slot>
///                 <DemoBox fill=true data_testid="layout-main-demo">"Main column"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Overlay pinned header
/// Sticky frost header—content starts below the bar and scrolls beneath it via window scroll.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, Layout, LayoutHeader, LayoutMain,
///     MaterialCorners, MaterialElevation, MaterialVariant, Title3,
/// };
/// view! {
///     <div data-testid="layout-overlay-header" style="height: 400px; border: 1px solid var(--orb-color-border-subtle); overflow: auto;">
///         <Layout overlay_header=true page_scrollport=false>
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Sticky>
///                     <AppBarMaterial variant=MaterialVariant::Frost elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot><Title3>"Overlay shell"</Title3></AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <DemoBox height="1200px" data_testid="layout-scroll-content">
///                     <p>"First line starts below the bar."</p>
///                     <p>"Scroll to see content pass under the frosted header."</p>
///                 </DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Overlay inset header
/// Opaque fixed header—content and scrollbar start below the bar with no scroll-under.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, Layout, LayoutHeader, LayoutMain,
///     MaterialCorners, MaterialElevation, MaterialVariant, Title3,
/// };
/// view! {
///     <div data-testid="layout-inset-header" style="height: 240px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true main_inset_scroll=true>
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Fixed>
///                     <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Raised corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot><Title3>"Inset shell"</Title3></AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <DemoBox height="400px" data_testid="layout-inset-content">
///                     <p>"First line starts below the bar."</p>
///                     <p>"Content stays below the opaque header when scrolling."</p>
///                 </DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## App shell
/// Header, sidebar, and main—the catalog-style compound shell.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing, Button,
///     ButtonAppearance, DemoBox, Layout, LayoutHeader, LayoutMain, LayoutSidebar, MaterialCorners,
///     MaterialElevation, MaterialVariant, Navigation, NavigationBody, NavigationConfig, NavigationItem,
///     Title3,
/// };
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="layout-app-shell" style="height: 260px; border: 1px solid var(--orb-color-border-subtle); overflow: auto;">
///         <Layout overlay_header=true page_scrollport=false>
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Sticky>
///                     <AppBarMaterial variant=MaterialVariant::Frost elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot><Title3>"Orbital Components"</Title3></AppBarLeading>
///                     <AppBarTrailing slot>
///                         <Button appearance=ButtonAppearance::Transparent icon=icondata::AiBulbOutlined />
///                     </AppBarTrailing>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutSidebar slot>
///                 <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///                     <NavigationBody slot>
///                         <NavigationItem config="card" icon=icondata::AiAppstoreOutlined>"Card"</NavigationItem>
///                     </NavigationBody>
///                 </Navigation>
///             </LayoutSidebar>
///             <LayoutMain slot>
///                 <DemoBox fill=true data_testid="layout-main-demo">"Preview outlet"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Sidebar toggle
/// Coordinated sidebar open state via [`LayoutSidebarToggle`].
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, Layout, LayoutHeader, LayoutMain,
///     LayoutSidebar, LayoutSidebarToggle, MaterialCorners, MaterialElevation, MaterialVariant,
///     Navigation, NavigationBody, NavigationConfig, NavigationItem, Title3,
/// };
/// let sidebar_open = RwSignal::new(true);
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="layout-sidebar-toggle" style="height: 240px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true sidebar_open=sidebar_open>
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Sticky>
///                     <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot>
///                         <LayoutSidebarToggle />
///                         <Title3>"Shell"</Title3>
///                     </AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutSidebar slot>
///                 <Navigation config=NavigationConfig::new().with_open(Signal::derive(move || sidebar_open.get())).with_selected_value(selected).with_open_categories(open)>
///                     <NavigationBody slot>
///                         <NavigationItem config="home" icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
///                     </NavigationBody>
///                 </Navigation>
///             </LayoutSidebar>
///             <LayoutMain slot>
///                 <DemoBox fill=true data_testid="layout-main-demo">"Main content"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Sidebar closed
/// Initial closed rail with expand affordance in the header.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, Layout, LayoutHeader, LayoutMain,
///     LayoutSidebar, LayoutSidebarToggle, MaterialCorners, MaterialElevation, MaterialVariant,
///     Navigation, NavigationBody, NavigationConfig, NavigationItem, Title3,
/// };
/// let sidebar_open = RwSignal::new(false);
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="layout-sidebar-closed" style="height: 240px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true sidebar_open=sidebar_open>
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Sticky>
///                     <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot>
///                         <LayoutSidebarToggle />
///                         <Title3>"Collapsed rail"</Title3>
///                     </AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutSidebar slot>
///                 <Navigation config=NavigationConfig::new().with_open(Signal::derive(move || sidebar_open.get())).with_selected_value(selected).with_open_categories(open)>
///                     <NavigationBody slot>
///                         <NavigationItem config="home" icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
///                     </NavigationBody>
///                 </Navigation>
///             </LayoutSidebar>
///             <LayoutMain slot>
///                 <DemoBox fill=true data_testid="layout-main-demo">"Main content"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Responsive Auto sidebar
/// Below the Md breakpoint the sidebar becomes an overlay drawer; wide viewports keep the inline column.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, Layout, LayoutHeader, LayoutMain,
///     LayoutSidebar, LayoutSidebarToggle, MaterialCorners, MaterialElevation, MaterialVariant,
///     SidebarPresentation, Title3,
/// };
/// let sidebar_open = RwSignal::new(false);
/// view! {
///     <div data-testid="layout-responsive-auto" style="height: 240px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout
///             overlay_header=true
///             sidebar_open=sidebar_open
///             sidebar_presentation=SidebarPresentation::Auto
///             page_scrollport=false
///         >
///             <LayoutHeader slot>
///                 <AppBar position=AppBarPosition::Sticky>
///                     <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot>
///                         <LayoutSidebarToggle />
///                         <Title3>"Responsive shell"</Title3>
///                     </AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutSidebar slot>
///                 <DemoBox fill=true data_testid="layout-responsive-nav">"Nav"</DemoBox>
///             </LayoutSidebar>
///             <LayoutMain slot>
///                 <DemoBox fill=true>"Main"</DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Shell",
    preview_slug = "layout",
    preview_label = "Layout",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn Layout(
    /// Extra CSS class names merged onto the shell root grid.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional `data-testid` for E2E hooks on the layout root.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// When true, the header overlays main content and provides [`AppBarInset`] context.
    #[prop(default = false)]
    overlay_header: bool,
    /// When true with `overlay_header`, main content scrolls inside an inset below the header.
    #[prop(default = false)]
    main_inset_scroll: bool,
    /// When true (default), pinned overlay shells scroll via a full-viewport [`ScrollArea`] so Chrome applies themed scrollbar chrome. Set false for bounded preview hosts.
    #[prop(default = true)]
    page_scrollport: bool,
    /// Header height token used for overlay inset padding — match the [`AppBar`] density.
    #[prop(default = AppBarDensity::Standard)]
    header_inset: AppBarDensity,
    /// Parent-owned sidebar open state; defaults to an internal open signal when omitted.
    #[prop(optional, into)]
    sidebar_open: Option<RwSignal<bool>>,
    /// How the sidebar is presented — inline column, overlay drawer, or auto by breakpoint.
    /// Defaults to [`SidebarPresentation::Inline`] for backward compatibility.
    #[prop(optional)]
    sidebar_presentation: SidebarPresentation,
    /// Breakpoint used when `sidebar_presentation` is [`SidebarPresentation::Auto`]
    /// (viewports below this width use an overlay drawer). Default: [`Breakpoint::Md`].
    #[prop(optional)]
    sidebar_overlay_breakpoint: Option<Breakpoint>,
    /// Header slot — typically an [`AppBar`] with leading and trailing regions.
    #[prop(optional)]
    layout_header: Option<LayoutHeader>,
    /// Sidebar slot — typically a [`Navigation`] rail beside main content.
    #[prop(optional)]
    layout_sidebar: Option<LayoutSidebar>,
    /// Main content slot — page body rendered in the scrollable region.
    #[prop(optional)]
    layout_main: Option<LayoutMain>,
) -> impl IntoView {
    if overlay_header {
        provide_context(AppBarInset {
            height_px: header_inset.height_px(),
        });
        provide_context(LayoutOverlayScroll { main_inset_scroll });
    }

    let sidebar_open = sidebar_open.unwrap_or_else(|| RwSignal::new(true));
    LayoutSidebarOpen::provide(sidebar_open);
    let sidebar_open_signal = Signal::derive(move || sidebar_open.get());

    let overlay_bp = sidebar_overlay_breakpoint.unwrap_or(DEFAULT_SIDEBAR_OVERLAY_BREAKPOINT);
    let narrow = use_breakpoint_down(overlay_bp);
    let use_overlay_sidebar = Signal::derive(move || match sidebar_presentation {
        SidebarPresentation::Inline => false,
        SidebarPresentation::Overlay => true,
        SidebarPresentation::Auto => narrow.get(),
    });

    // Close the sidebar when crossing into/out of overlay mode so surfaces do not stack.
    {
        let prev = StoredValue::new(use_overlay_sidebar.get_untracked());
        Effect::new(move |_| {
            let now = use_overlay_sidebar.get();
            if now != prev.get_value() {
                prev.set_value(now);
                sidebar_open.set(false);
            }
        });
    }

    let has_sidebar = layout_sidebar.is_some();
    let inset_px = header_inset.height_px();
    let root_style = Signal::derive(move || {
        if overlay_header {
            format!("--orbital-layout-header-inset: {inset_px}px;")
        } else {
            String::new()
        }
    });

    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        let mut parts = Vec::new();
        if has_sidebar {
            parts.push("orbital-layout--has-sidebar".to_string());
            if use_overlay_sidebar.get() {
                parts.push("orbital-layout--sidebar-overlay".to_string());
                parts.push("orbital-layout--sidebar-closed".to_string());
            } else if !sidebar_open_signal.get() {
                parts.push("orbital-layout--sidebar-closed".to_string());
            }
        }
        if overlay_header && main_inset_scroll {
            parts.push("orbital-layout--inset-header".to_string());
        }
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });

    let style_sheet = layout_styles();
    let use_page_scrollport = overlay_header && !main_inset_scroll && page_scrollport;
    let page_scroll_style = format!(
        "display: block; width: 100%; height: 100%; box-sizing: border-box; \
         --orbital-layout-header-inset: {inset_px}px; scroll-padding-top: {inset_px}px;"
    );

    let sidebar_children = layout_sidebar.map(|slot| slot.children);
    let drawer_open: OpenBind = sidebar_open.into();
    let sidebar_for_inline = sidebar_children.clone();
    let sidebar_for_drawer = sidebar_children;

    let shell = view! {
        <BaseLayout
            class=root_class
            style=root_style
            data_testid=data_testid
            overlay_header=overlay_header
            position=LayoutPosition::Static
        >
            {layout_header.map(|slot| (slot.children)())}
            <BaseLayoutBody>
                {
                    move || {
                        let Some(children) = sidebar_for_inline.clone() else {
                            return ().into_any();
                        };
                        if use_overlay_sidebar.get() {
                            ().into_any()
                        } else {
                            view! { <LayoutSidebarShell>{children()}</LayoutSidebarShell> }.into_any()
                        }
                    }
                }
                {layout_main.map(|slot| view! { <LayoutMainShell>{(slot.children)()}</LayoutMainShell> })}
            </BaseLayoutBody>
        </BaseLayout>
        {
            move || {
                let Some(children) = sidebar_for_drawer.clone() else {
                    return ().into_any();
                };
                if !use_overlay_sidebar.get() {
                    return ().into_any();
                }
                view! {
                    <OverlayDrawer
                        open=drawer_open
                        position=DrawerPosition::Left
                        size=DrawerSize::Navigation
                        close_on_esc=true
                        class="orbital-layout-sidebar-drawer"
                    >
                        <div data-testid="layout-sidebar-drawer">
                            {children()}
                        </div>
                    </OverlayDrawer>
                }
                .into_any()
            }
        }
    };

    view! {
        <style>{style_sheet}</style>
        <style>{SIDEBAR_DRAWER_OVERLAY_CSS}</style>
        {if use_page_scrollport {
            view! {
                <ScrollArea
                    class="orbital-layout__page-scroll"
                    style=page_scroll_style
                >
                    {shell}
                </ScrollArea>
            }
            .into_any()
        } else {
            shell.into_any()
        }}
    }
}
