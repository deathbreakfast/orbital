use icondata_core::Icon;
use leptos::prelude::*;
use orbital_base_components::{ButtonAppearance, ButtonShape, ButtonSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use orbital_base_components::Handler;

use crate::{Button, ButtonGroup, Menu, MenuTrigger};

use super::styles::action_menu_button_styles;
use super::types::ActionMenuItems;

/// Pairs a primary command with a chevron menu for related alternates — for example, Save alongside Save as and Export.
///
/// Wire the main action with `on_click` and menu picks with `on_select`. Do not repeat the primary label in menu items. When menu labels are abbreviated, ensure surrounding context gives screen-reader users enough detail.
///
/// # When to use
///
/// - One dominant action plus a small set of related alternates in a menu - Save / Save as / Export, Send / Schedule send, and similar command clusters
///
/// # Usage
///
/// 1. Put primary label text in `children` and menu items in the `action_menu_items` slot. 2. Wire `on_click` for the primary segment and `on_select` for menu item values. Name the chevron with `menu_label` (default "More options"), e.g. "More save options". 3. Prefer [`MenuButton`](crate::MenuButton) when every choice lives in the menu with no separate primary click.
///
/// # Examples
///
/// ## Default action menu button
/// Primary save action with a chevron menu for alternates.
/// <!-- preview -->
/// ```rust
/// use crate::{ActionMenuButton, ActionMenuItems, ButtonAppearance, MenuItem};
/// view! {
///     <div data-testid="action-menu-button-preview">
///         <ActionMenuButton appearance=ButtonAppearance::Primary>
///             "Save"
///             <ActionMenuItems slot>
///                 <MenuItem value="save-as".to_string()>"Save as"</MenuItem>
///                 <MenuItem value="export".to_string()>"Export"</MenuItem>
///             </ActionMenuItems>
///         </ActionMenuButton>
///     </div>
/// }
/// ```
///
/// ## With icon
/// Primary segment includes a leading icon; `menu_label` names the chevron.
/// <!-- preview -->
/// ```rust
/// use crate::{ActionMenuButton, ActionMenuItems, ButtonAppearance, MenuItem};
/// use icondata::AiSaveOutlined;
/// view! {
///     <div data-testid="action-menu-button-icon">
///         <ActionMenuButton
///             appearance=ButtonAppearance::Primary
///             icon=AiSaveOutlined
///             menu_label="More save options"
///         >
///             "Save"
///             <ActionMenuItems slot>
///                 <MenuItem value="draft".to_string()>"Save draft"</MenuItem>
///             </ActionMenuItems>
///         </ActionMenuButton>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "action-menu-button",
    preview_label = "Action Menu Button",
    preview_icon = icondata::AiColumnWidthOutlined,
)]
#[component]
pub fn ActionMenuButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: Signal<ButtonAppearance>,
    #[prop(optional, into)] shape: Signal<ButtonShape>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] icon: MaybeProp<Icon>,
    #[prop(optional, into)] menu_icon: MaybeProp<Icon>,
    #[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    /// Accessible name for the chevron menu button. Defaults to "More options".
    #[prop(optional, into)]
    menu_label: MaybeProp<String>,
    /// Optional menu items rendered in the dropdown.
    action_menu_items: Option<ActionMenuItems>,
    /// Primary label text.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-action-menu-button", action_menu_button_styles());

    let menu_icon = Memo::new(move |_| menu_icon.get().unwrap_or(icondata::AiCaretDownOutlined));
    let on_select_handler = Handler::on(move |value: String| {
        if let Some(cb) = on_select {
            cb.run(value);
        }
    });

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-action-menu-button".to_string()];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            <ButtonGroup>
                <Button
                    class="orbital-action-menu-button__primary".to_string()
                    appearance=appearance
                    shape=shape
                    size=size
                    disabled=disabled
                    icon=icon
                    nostrip:on_click=on_click
                >
                    {children()}
                </Button>
                <Menu on_select=on_select_handler>
                    <MenuTrigger slot>
                        <Button
                            class="orbital-action-menu-button__menu".to_string()
                            attr:aria-label=move || {
                                menu_label.get().unwrap_or_else(|| "More options".to_string())
                            }
                            attr:aria-haspopup="menu"
                            appearance=appearance
                            shape=shape
                            size=size
                            disabled=disabled
                            icon=Signal::derive(move || menu_icon.get())
                        />
                    </MenuTrigger>
                    {action_menu_items.map(|slot| (slot.children)())}
                </Menu>
            </ButtonGroup>
        </div>
    }
}
