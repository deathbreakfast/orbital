use leptos::prelude::*;
use orbital_base_components::{BaseMenu, Handler, OverlayAppearance};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::{menu_item_styles, menu_styles};
use super::types::{MenuAppearance, MenuConfig, MenuPosition, MenuTriggerType};
use crate::overlay::{overlay_surface_class, FloatingPanel};
use crate::{MaterialElevation, MenuTrigger};

/// Builds inline width constraints for menu overlay shells.
fn menu_overlay_width_style(min_width: Option<String>, max_width: Option<String>) -> String {
    let min = min_width.filter(|s| !s.trim().is_empty());
    let max = max_width.filter(|s| !s.trim().is_empty());
    if min.is_none() && max.is_none() {
        return String::new();
    }
    let mut parts = vec!["box-sizing: border-box".to_string()];
    if let Some(min) = min {
        parts.push(format!("min-width: {min}"));
    }
    if let Some(max) = max {
        parts.push(format!("max-width: {max}"));
        parts.push(format!("width: {max}"));
    }
    parts.join("; ")
}

/// `Menu` opens a list of actions from any trigger you place in [`MenuTrigger`] — icon buttons,
/// links, or custom controls. Handle choices in `on_select` by matching each [`MenuItem`] `value`.
///
/// Use [`MenuButton`](crate::MenuButton) when a standard text button trigger is enough.
/// Use `Menu` for hover open, custom triggers, placement, or menu surface appearance
/// (`Brand`, `Inverted`). Submenus are not supported yet — nest a future `Menu` pattern when added.
///
/// # When to use
///
/// - Action lists from icon buttons, split controls, or custom triggers
/// - Hover-open menus (`MenuTriggerType::Hover`) or explicit placement
/// - Pickers, detail panels, or form fields — use [`Popover`](crate::Popover) instead
///
/// # Overlay surfaces
///
/// - **Brief non-interactive hint** — [`Tooltip`](crate::Tooltip)
/// - **Floating panel with content or inputs** — [`Popover`](crate::Popover)
/// - **List of actions from a trigger** — `Menu` (this component) or [`MenuButton`](crate::MenuButton)
/// - **Block the page or trap focus** — [`Dialog`](crate::Dialog)
///
/// # Usage
///
/// 1. Put the trigger in [`MenuTrigger`] (often [`crate::Button`] or an icon button).
/// 2. Set `trigger_type`: `Click` (default) or `Hover` for pointer-driven open.
/// 3. Add [`MenuItem`] children with distinct `value` strings.
/// 4. Handle `on_select` by matching the chosen `value`.
///
/// # Examples
///
/// ## Default
/// Click opens an action list. Match each item's `value` in `on_select`.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem};
///
/// view! {
///     <div data-testid="menu-preview">
///         <Menu on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Actions"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("edit")>"Edit"</MenuItem>
///             <MenuItem value=String::from("delete")>"Delete"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Icon trigger with `on_click`
/// Ellipsis or kebab trigger saves toolbar space; optional item icons reinforce command meaning. Pair icon-only triggers with `aria-label` on the button wrapper.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, MenuItem};
///
/// view! {
///     <div data-testid="menu-icons">
///         <Menu on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button
///                     appearance=ButtonAppearance::Subtle
///                     icon=icondata::AiEllipsisOutlined
///                     on_click=Callback::new(|_ev: leptos::ev::MouseEvent| {})
///                 />
///             </MenuTrigger>
///             <MenuItem value=String::from("copy") icon=icondata::AiCopyOutlined>"Copy"</MenuItem>
///             <MenuItem value=String::from("delete")>"Delete"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Hover trigger
/// Opens on pointer enter for low-risk, discoverable command previews—closes after a short leave delay.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, MenuItem};
///
/// view! {
///     <div data-testid="menu-hover">
///         <Menu trigger_type=MenuTriggerType::Hover on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button appearance=ButtonAppearance::Subtle>"Hover me"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("a")>"Item A"</MenuItem>
///             <MenuItem value=String::from("b")>"Item B"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Brand appearance
/// Brand-tinted menu surface matches primary app chrome—use inside branded headers or hero regions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem, MenuAppearance};
///
/// view! {
///     <div data-testid="menu-brand">
///         <Menu appearance=MenuAppearance::Brand on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Brand menu"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("one")>"One"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Inverted appearance
/// Dark inverted surface for menus anchored on dark or image-heavy toolbars.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem, MenuAppearance};
///
/// view! {
///     <div data-testid="menu-inverted">
///         <Menu appearance=MenuAppearance::Inverted on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Inverted"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("x")>"Action"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Position (top)
/// Flips placement above the trigger when space below is tight.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem};
///
/// view! {
///     <div data-testid="menu-position-top">
///         <Menu position=MenuPosition::Top on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Open above"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("a")>"Above"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Disabled item
/// Disabled menu items remain visible but cannot be selected.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem};
///
/// view! {
///     <div data-testid="menu-disabled">
///         <Menu on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Actions"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("ok")>"OK"</MenuItem>
///             <MenuItem value=String::from("blocked") disabled=Signal::from(true)>"Blocked"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
///
/// ## Keyboard navigation
/// Arrow keys move focus between items; Enter selects the focused item.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, MenuItem};
///
/// view! {
///     <div data-testid="menu-keyboard">
///         <Menu on_select=|_value: String| {}>
///             <MenuTrigger slot>
///                 <Button>"Keyboard menu"</Button>
///             </MenuTrigger>
///             <MenuItem value=String::from("edit")>"Edit"</MenuItem>
///             <MenuItem value=String::from("copy")>"Copy"</MenuItem>
///             <MenuItem value=String::from("delete")>"Delete"</MenuItem>
///         </Menu>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "menu",
    preview_label = "Menu",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn Menu<T, V>(
    /// Optional CSS class on the menu surface.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Slot: element or component that opens the menu (typically [`crate::Button`]).
    menu_trigger: MenuTrigger<T>,
    /// How the menu opens: `Click` (default) or `Hover`.
    #[prop(optional)]
    trigger_type: MenuTriggerType,
    /// Placement relative to the trigger (`Bottom` default).
    #[prop(optional)]
    position: MenuPosition,
    /// Called when a [`MenuItem`] is chosen; receives that item's `value`.
    #[prop(into)]
    on_select: Handler<V>,
    /// Visual variant: `Brand` or `Inverted`; omit for default surface.
    #[prop(optional, into)]
    appearance: MaybeProp<MenuAppearance>,
    /// Minimum menu panel width (CSS length, e.g. `"360px"`).
    #[prop(optional, into)]
    min_width: MaybeProp<String>,
    /// Maximum menu panel width (CSS length, e.g. `"400px"`). When set, also sets `width` to this value for stable sizing.
    #[prop(optional, into)]
    max_width: MaybeProp<String>,
    /// Optional `data-testid` on the menu overlay shell.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// [`MenuItem`] children.
    children: Children,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
    V: Clone + Send + Sync + 'static,
{
    inject_style("orbital-menu", menu_styles());
    inject_style("orbital-menu-item", menu_item_styles());

    let config = MenuConfig {
        trigger_type,
        position,
        appearance,
    };

    let appearance = config
        .appearance
        .get()
        .map(|a: MenuAppearance| a.into())
        .unwrap_or(OverlayAppearance::Default);

    let surface_class =
        Signal::derive(move || overlay_surface_class("orbital-menu", appearance, None));

    let overlay_style =
        Signal::derive(move || menu_overlay_width_style(min_width.get(), max_width.get()));
    let panel_fill_style = Signal::derive(move || {
        if min_width.get().is_some() || max_width.get().is_some() {
            Some("width: 100%; box-sizing: border-box".to_string())
        } else {
            None
        }
    });

    view! {
        <BaseMenu
            trigger_type=config.trigger_type.into()
            placement=config.position.into()
            appearance=appearance
            class=class
            style=overlay_style
            data_testid=data_testid
            on_select=on_select
            overlay_trigger=menu_trigger
        >
            <FloatingPanel
                class=surface_class
                style=panel_fill_style
                body_class="orbital-menu-body"
                elevation=MaterialElevation::Floating
            >
                {children()}
            </FloatingPanel>
        </BaseMenu>
    }
}
