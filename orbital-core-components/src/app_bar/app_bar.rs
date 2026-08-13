use leptos::prelude::*;
use orbital_base_components::{AppBarDensity, AppBarPosition, BaseAppBar};
use orbital_macros::component_doc;

use crate::{
    material::material_flat_outline_modifier, material::material_modifier_classes,
    material::MaterialOutlineEdge, Material,
};
use orbital_base_components::AppBarInset;

use super::slots::{default_app_bar_material, AppBarLeading, AppBarMaterial, AppBarTrailing};
use super::styles::app_bar_styles;

/// Application shell header composed from [`Material`] and region slots.
///
/// The top application header surfaces screen context, wayfinding, and actions tied to the current view. Content is host-composed via [`AppBarLeading`], default `children` (center), and [`AppBarTrailing`]. Configure surface treatment through the [`AppBarMaterial`] slot (`variant`, `elevation`, `corners`).
///
/// # When to use
///
/// - Application shell chrome above primary content - Screen titles, breadcrumbs, and workspace branding - Inline search, filters, or section navigation in the center band - Account, theme, and utility controls in the trailing region
///
/// # Usage
///
/// 1. Configure surface via [`AppBarMaterial`] slot (`variant`, `elevation`, `corners`). 2. Place branding and wayfinding in [`AppBarLeading`]. 3. Put search, filters, or inline nav in default `children` (center region). 4. Place account and utility controls in [`AppBarTrailing`]. 5. Adjust `position` and `density` for scroll behavior and vertical budget. 6. Pair Sticky bars with [`Layout`](crate::Layout) `overlay_header=true` for pinned frost shells; use `main_inset_scroll=true` with Fixed opaque bars for inset inner scroll. 7. For mobile tuck-away chrome, wrap Sticky AppBar in [`HideOnScroll`](crate::HideOnScroll).
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer `Flat` elevation for shell chrome — a bottom border separates the bar from content * Prefer Frost or Shell + `Flat` for pinned scroll-under shells * Prefer `Shell` + `Flat` for catalog and workspace shells when opaque chrome is needed * Put search fields and inline nav in default children (center) * Use `ButtonAppearance::Transparent` for icon utilities in trailing * Wrap E2E hooks in a native `div` with `data-testid`
///
/// ## Don'ts
///
/// * Do not use `Resting` or `Raised` on shell AppBar — reserve elevated tiers for in-content surfaces * Do not duplicate Material shadow CSS on AppBar — elevation belongs on [`Material`] * Do not bake Button variants into AppBar — compose controls in slots * Do not use icon-only controls without an accessible name on a wrapper
///
/// # Examples
///
/// ## Standard shell
/// Workspace title in leading and a sign-in action in trailing—the default application header.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, Title3};
/// view! {
///     <div data-testid="app-bar-preview">
///         <AppBar>
///             <AppBarLeading slot>
///                 <Title3>"Orbital Components"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Primary>"Sign in"</Button>
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Navigation affordance
/// Menu control beside the workspace title; sign-out action in trailing.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, Title3};
/// view! {
///     <div data-testid="app-bar-with-menu">
///         <AppBar>
///             <AppBarLeading slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMenuOutlined />
///                 <Title3>"Workspace"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Secondary>"Sign out"</Button>
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Inline wayfinding
/// Product mark in leading, section links in center, account control in trailing.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, Flex, FlexAlign, FlexGap, Title3};
/// view! {
///     <div data-testid="app-bar-with-nav">
///         <AppBar>
///             <AppBarLeading slot>
///                 <Title3>"Orbital"</Title3>
///             </AppBarLeading>
///             <Flex gap=FlexGap::Large align=FlexAlign::Center>
///                 <span>"Components"</span>
///                 <span>"Patterns"</span>
///                 <span>"Tokens"</span>
///             </Flex>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiUserOutlined />
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Centered search
/// Title in leading, scoped search field in center, actions in trailing.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, SearchBox, SearchBoxAppearance, Title3};
/// let search = RwSignal::new(String::new());
/// view! {
///     <div data-testid="app-bar-with-search">
///         <AppBar>
///             <AppBarLeading slot>
///                 <Title3>"Catalog"</Title3>
///             </AppBarLeading>
///             <SearchBox bind=search appearance=SearchBoxAppearance::with_placeholder("Search components…") />
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiSettingOutlined />
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Search-first
/// Minimal leading label; the search field owns the center band.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarTrailing, Button, ButtonAppearance, SearchBox, SearchBoxAppearance};
/// let search = RwSignal::new(String::new());
/// view! {
///     <div data-testid="app-bar-search-first">
///         <AppBar>
///             <AppBarLeading slot>
///                 <span>"Find"</span>
///             </AppBarLeading>
///             <SearchBox bind=search appearance=SearchBoxAppearance::with_placeholder("Search workspace…") />
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Primary>"Go"</Button>
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Compact density
/// 48px compact bar for secondary tool rows.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarDensity, AppBarLeading, AppBarTrailing, Button, ButtonAppearance};
/// view! {
///     <div data-testid="app-bar-compact">
///         <AppBar density=AppBarDensity::Compact>
///             <AppBarLeading slot>
///                 <span>"Tools"</span>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiFilterOutlined />
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Expanded density
/// 96px expanded band for hero title or branding in leading.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarDensity, AppBarLeading};
/// view! {
///     <div data-testid="app-bar-expanded">
///         <AppBar density=AppBarDensity::Expanded>
///             <AppBarLeading slot>
///                 <span style="font-size: 28px; font-weight: 600;">"Design System"</span>
///             </AppBarLeading>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Shell surface
/// Shell material with flat elevation—catalog-style frosted shell chrome.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarMaterial, AppBarTrailing, Button, ButtonAppearance, MaterialCorners, MaterialElevation, MaterialVariant, Title3};
/// view! {
///     <div data-testid="app-bar-shell-flat">
///         <AppBar>
///             <AppBarMaterial variant=MaterialVariant::Shell elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///             <AppBarLeading slot>
///                 <Title3>"Orbital Components"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiBulbOutlined />
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Raised over content
/// Raised elevation separates the header from scrolling body content below.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarMaterial, AppBarTrailing, Button, ButtonAppearance, MaterialElevation, MaterialVariant, Title3};
/// view! {
///     <div data-testid="app-bar-raised">
///         <AppBar>
///             <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Raised slot />
///             <AppBarLeading slot>
///                 <Title3>"Dashboard"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiBellOutlined />
///             </AppBarTrailing>
///         </AppBar>
///     </div>
/// }
/// ```
///
/// ## Sticky while scrolling
/// Bar pins to the top of its scroll container while content moves beneath.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarPosition, AppBarTrailing, Button, ButtonAppearance, Title3};
/// view! {
///     <div data-testid="app-bar-sticky" style="height: 200px; overflow: auto; border: 1px solid var(--orb-color-border-subtle);">
///         <AppBar position=AppBarPosition::Sticky>
///             <AppBarLeading slot>
///                 <Title3>"Sticky header"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
///             </AppBarTrailing>
///         </AppBar>
///         <div style="height: 400px; padding: 16px;">"Scroll to see the bar stay pinned."</div>
///     </div>
/// }
/// ```
///
/// ## Fixed with scroll body
/// Fixed bar with scrollable content below—the host must offset body content.
/// <!-- preview -->
/// ```rust
/// use crate::{AppBar, AppBarLeading, AppBarPosition, AppBarTrailing, Button, ButtonAppearance, Title3};
/// view! {
///     <div data-testid="app-bar-fixed" style="position: relative; height: 200px; overflow: hidden; border: 1px solid var(--orb-color-border-subtle);">
///         <AppBar position=AppBarPosition::Fixed>
///             <AppBarLeading slot>
///                 <Title3>"Fixed header"</Title3>
///             </AppBarLeading>
///             <AppBarTrailing slot>
///                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiCloseOutlined />
///             </AppBarTrailing>
///         </AppBar>
///         <div style="padding-top: 56px; height: 100%; overflow: auto; box-sizing: border-box;">
///             <div style="height: 400px; padding: 16px;">"Content scrolls beneath the fixed bar."</div>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Shell",
    preview_slug = "app-bar",
    preview_label = "App Bar",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn AppBar(
    /// Extra CSS class names merged onto the app bar root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Positioning mode — `Fixed` and `Sticky` provide [`AppBarInset`] context for [`Layout`].
    #[prop(default = AppBarPosition::Static)]
    position: AppBarPosition,
    /// Bar height and row spacing — `Standard` (56px) or `Compact` (48px).
    #[prop(default = AppBarDensity::Standard)]
    density: AppBarDensity,
    /// Optional [`Material`] treatment override for the bar surface.
    #[prop(optional)]
    app_bar_material: Option<AppBarMaterial>,
    /// Leading slot — menu button, back affordance, or title cluster on the start edge.
    #[prop(optional)]
    app_bar_leading: Option<AppBarLeading>,
    /// Trailing slot — search, actions, or account controls on the end edge.
    #[prop(optional)]
    app_bar_trailing: Option<AppBarTrailing>,
    /// Center title or custom bar content between leading and trailing slots.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (variant, elevation, corners) = app_bar_material
        .map(|m| (m.variant, m.elevation, m.corners))
        .unwrap_or_else(default_app_bar_material);

    if matches!(position, AppBarPosition::Fixed | AppBarPosition::Sticky) {
        provide_context(AppBarInset {
            height_px: density.height_px(),
        });
    }

    let style_sheet = app_bar_styles();
    let material_modifiers = material_modifier_classes(variant, elevation, corners);
    let outline = material_flat_outline_modifier(variant, elevation, MaterialOutlineEdge::Bottom)
        .map(|class| format!(" {class}"))
        .unwrap_or_default();
    let material_class =
        format!("orbital-app-bar__material orbital-material {material_modifiers}{outline}");

    view! {
        <style>{style_sheet}</style>
        <BaseAppBar
            class=class
            position=position
            density=density
        >
            <Material class=material_class variant=variant elevation=elevation corners=corners>
                <div class="orbital-app-bar__row">
                    {app_bar_leading.map(|slot| {
                        view! {
                            <div class="orbital-app-bar__leading">
                                {(slot.children)()}
                            </div>
                        }
                    })}
                    <div class="orbital-app-bar__center">
                        {children.map(|c| c())}
                    </div>
                    {app_bar_trailing.map(|slot| {
                        view! {
                            <div class="orbital-app-bar__trailing">
                                {(slot.children)()}
                            </div>
                        }
                    })}
                </div>
            </Material>
        </BaseAppBar>
    }
}
