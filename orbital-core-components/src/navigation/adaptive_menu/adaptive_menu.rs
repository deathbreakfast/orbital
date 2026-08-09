//! [`AdaptiveMenu`] — responsive popover ↔ overlay drawer.

use leptos::prelude::*;
use orbital_base_components::{use_breakpoint_down, OpenBind};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use orbital_theme::Breakpoint;

use super::styles::adaptive_menu_styles;
use super::types::{AdaptiveMenuTrigger, DEFAULT_ADAPTIVE_MENU_BREAKPOINT};
use crate::navigation::{
    DrawerPosition, DrawerSize, OverlayDrawer, Popover, PopoverPosition, PopoverTrigger,
    PopoverTriggerType,
};
use crate::{
    Button, ButtonAppearance, Flex, FlexJustify, SpacingHorizontal, SpacingInset, SpacingVertical,
};

/// Opens rich content from a trigger as a popover on wide viewports and an
/// [`OverlayDrawer`](crate::OverlayDrawer) below a breakpoint.
///
/// Use for AppBar mega-menus and dense navigation that cannot fit a clipped popover on phones.
///
/// # Examples
///
/// ## Default
/// <!-- preview -->
/// ```rust
/// use crate::{AdaptiveMenu, AdaptiveMenuTrigger, Button, ButtonAppearance};
/// view! {
///     <div data-testid="adaptive-menu-preview">
///         <AdaptiveMenu>
///             <AdaptiveMenuTrigger slot>
///                 <Button appearance=ButtonAppearance::Subtle>"Platform"</Button>
///             </AdaptiveMenuTrigger>
///             <div data-testid="adaptive-menu-panel">
///                 <p>"Nav links and sections"</p>
///             </div>
///         </AdaptiveMenu>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "adaptive-menu",
    preview_label = "Adaptive Menu",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn AdaptiveMenu(
    /// Desktop popover placement (default [`PopoverPosition::Bottom`]).
    #[prop(optional)]
    position: Option<PopoverPosition>,
    /// Drawer edge on narrow viewports (default [`DrawerPosition::Right`]).
    #[prop(optional)]
    drawer_position: Option<DrawerPosition>,
    /// Below this breakpoint the menu becomes a drawer (default [`Breakpoint::Md`]).
    #[prop(optional)]
    breakpoint: Option<Breakpoint>,
    /// Optional accessible name for the mobile drawer.
    #[prop(optional, into)]
    drawer_aria_label: MaybeProp<String>,
    /// Trigger control.
    adaptive_menu_trigger: AdaptiveMenuTrigger,
    /// Panel / drawer body.
    children: ChildrenFn,
) -> impl IntoView {
    inject_style("orbital-adaptive-menu", adaptive_menu_styles());
    let position = position.unwrap_or(PopoverPosition::Bottom);
    let drawer_position = drawer_position.unwrap_or(DrawerPosition::Right);
    let breakpoint = breakpoint.unwrap_or(DEFAULT_ADAPTIVE_MENU_BREAKPOINT);
    let narrow = use_breakpoint_down(breakpoint);
    // Media queries are `false` during SSR; wait until hydrate before choosing
    // popover vs drawer so SSR/client do not leave both surfaces mounted.
    let hydrated = RwSignal::new(false);
    Effect::new(move |_| {
        hydrated.set(true);
    });
    let open = RwSignal::new(false);
    let open_bind: OpenBind = open.into();

    {
        let prev = StoredValue::new(narrow.get_untracked());
        Effect::new(move |_| {
            let now = narrow.get();
            if now != prev.get_value() {
                prev.set_value(now);
                open.set(false);
            }
        });
    }

    let trigger = adaptive_menu_trigger.children;
    let body = children;

    view! {
        <div data-testid="adaptive-menu" class="orbital-adaptive-menu">
            {
                let trigger = trigger.clone();
                let body = body.clone();
                move || {
                    if !hydrated.get() {
                        let trigger = trigger.clone();
                        return view! {
                            <div data-testid="adaptive-menu-trigger">
                                {trigger()}
                            </div>
                        }
                        .into_any();
                    }
                    if narrow.get() {
                        let trigger = trigger.clone();
                        let body = body.clone();
                        let aria = drawer_aria_label.get()
                            .unwrap_or_else(|| "Menu".to_string());
                        view! {
                            <div
                                data-testid="adaptive-menu-trigger"
                                on:click=move |_| open.update(|v| *v = !*v)
                            >
                                {trigger()}
                            </div>
                            <OverlayDrawer
                                open=open_bind
                                position=drawer_position
                                size=DrawerSize::Navigation
                                close_on_esc=true
                                class="orbital-adaptive-menu-drawer-host"
                            >
                                <div
                                    data-testid="adaptive-menu-drawer"
                                    role="dialog"
                                    aria-label=aria
                                    class="orbital-adaptive-menu-drawer"
                                >
                                    <Flex
                                        class="orbital-adaptive-menu-drawer__header"
                                        justify=FlexJustify::FlexEnd
                                        padding=SpacingInset::symmetric(
                                            SpacingHorizontal::S,
                                            SpacingVertical::S,
                                        )
                                    >
                                        <Button
                                            appearance=ButtonAppearance::Subtle
                                            icon=icondata::AiCloseOutlined
                                            on_click=Callback::new(move |_: leptos::ev::MouseEvent| {
                                                open.set(false);
                                            })
                                        />
                                    </Flex>
                                    <div data-testid="adaptive-menu-body" class="orbital-adaptive-menu-drawer__body">
                                        {body()}
                                    </div>
                                </div>
                            </OverlayDrawer>
                        }
                        .into_any()
                    } else {
                        let trigger = trigger.clone();
                        let body = body.clone();
                        view! {
                            <Popover
                                trigger_type=PopoverTriggerType::Click
                                position=position
                            >
                                <PopoverTrigger slot>
                                    <div data-testid="adaptive-menu-trigger">
                                        {trigger()}
                                    </div>
                                </PopoverTrigger>
                                <div data-testid="adaptive-menu-popover" class="orbital-adaptive-menu-popover">
                                    <div data-testid="adaptive-menu-body">
                                        {body()}
                                    </div>
                                </div>
                            </Popover>
                        }
                        .into_any()
                    }
                }
            }
        </div>
    }
}
