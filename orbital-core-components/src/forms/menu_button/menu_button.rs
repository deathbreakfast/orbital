use icondata_core::Icon as IconData;
use leptos::either::Either;
use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::button::{Button, ButtonAppearance};
#[cfg(feature = "preview")]
use crate::navigation::MenuItem;
use crate::navigation::{Menu, MenuTrigger};
use crate::Icon;

use super::styles::menu_button_styles;

/// Fast path to an actions menu — a labeled trigger opens a list of [`MenuItem`] options and reports the chosen `value` through `on_select`.
///
/// Default label is `"Actions"`; override with `label`. Includes a trailing chevron on the trigger. Set `icon` for an icon-only trigger such as a ⋯ overflow button; it shows no text or chevron and takes its accessible name from `aria_label`. No separate primary action — use [`ActionMenuButton`](crate::ActionMenuButton) for Save + menu patterns. For custom triggers, placement, or hover open, compose [`Menu`](crate::Menu) directly.
///
/// # When to use
///
/// - Quick action menus where a standard text button trigger is enough - Export, share, or overflow menus with a fixed set of choices
///
/// # Usage
///
/// 1. Provide `on_select` to handle the chosen item `value`. 2. Override `label` when `"Actions"` is too generic, or set `icon` and `aria_label` for an icon-only trigger. 3. Prefer [`ActionMenuButton`](crate::ActionMenuButton) when one menu item should run immediately on the main segment click.
///
/// # Examples
///
/// ## Default
/// Button with the default `"Actions"` label and trailing chevron opens a list of options on click.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="menu-button-preview">
///         <MenuButton on_select=|_v: String| {}>
///             <MenuItem value=String::from("edit")>"Edit"</MenuItem>
///             <MenuItem value=String::from("share")>"Share"</MenuItem>
///         </MenuButton>
///     </div>
/// }
/// ```
///
/// ## Primary appearance
/// Trigger styled as a primary button. Use when the menu represents the main action on a surface, such as "Save as" or export choices.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="menu-button-primary">
///         <MenuButton
///             appearance=ButtonAppearance::Primary
///             on_select=|_v: String| {}
///         >
///             <MenuItem value=String::from("save")>"Save as"</MenuItem>
///         </MenuButton>
///     </div>
/// }
/// ```
///
/// ## Custom label
/// Override the default `"Actions"` label via the `label` prop.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="menu-button-label">
///         <MenuButton label="Options" on_select=|_v: String| {}>
///             <MenuItem value=String::from("edit")>"Edit"</MenuItem>
///         </MenuButton>
///     </div>
/// }
/// ```
///
/// ## Icon-only trigger
/// An overflow ⋯ button for card or row actions. `aria_label` names the button for screen readers, since it shows no text.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="menu-button-icon">
///         <MenuButton
///             icon=icondata::AiMoreOutlined
///             aria_label="More actions"
///             appearance=ButtonAppearance::Subtle
///             on_select=|_v: String| {}
///         >
///             <MenuItem value=String::from("rename")>"Rename"</MenuItem>
///             <MenuItem value=String::from("archive")>"Archive"</MenuItem>
///         </MenuButton>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "menu-button",
    preview_label = "Menu Button",
    preview_icon = icondata::AiCaretDownOutlined,
)]
#[component]
pub fn MenuButton(
    /// Button visual appearance.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// Called when a menu item is chosen; receives that item's `value`.
    #[prop(into)]
    on_select: Handler<String>,
    /// Trigger button label when no custom trigger is provided.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Icon for an icon-only trigger. Replaces the label text and chevron.
    #[prop(optional, into)]
    icon: Option<IconData>,
    /// Accessible name for the trigger. An icon-only trigger falls back to
    /// `label`, then `"More actions"`.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// [`MenuItem`] children.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-menu-button", menu_button_styles());

    view! {
        <Menu on_select=on_select>
            <MenuTrigger slot>
                {match icon {
                    Some(icon) => Either::Left(view! {
                        <Button
                            appearance=appearance
                            icon=icon
                            class="orbital-menu-button__trigger"
                            attr:aria-label=move || {
                                aria_label
                                    .get()
                                    .or_else(|| label.get())
                                    .unwrap_or_else(|| "More actions".to_string())
                            }
                        />
                    }),
                    None => Either::Right(view! {
                        <Button
                            appearance=appearance
                            class="orbital-menu-button__trigger"
                            attr:aria-label=move || aria_label.get()
                        >
                            {move || label.get().unwrap_or_else(|| "Actions".to_string())}
                            <span class="orbital-menu-button__chevron">
                                <Icon icon=icondata::AiCaretDownOutlined width="1em" height="1em" />
                            </span>
                        </Button>
                    }),
                }}
            </MenuTrigger>
            {children()}
        </Menu>
    }
}
