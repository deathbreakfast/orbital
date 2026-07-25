use leptos::prelude::*;
use orbital_base_components::{BaseIcon, IconSize, ThemeColor};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use turf::inline_style_sheet_values;

/// Styled SVG icon from the icondata catalog at `1em` by default, with fill tracking foreground theme tokens via `currentColor`.
///
/// # When to use
///
/// - Decorative or supplementary glyphs beside labels, in buttons, menus, and status rows - Primary visuals in empty states and headers when sized above `1em` - Icon-only controls when paired with an accessible name on a native wrapper
///
/// # Usage
///
/// 1. Import an icon from `icondata` (for example `icondata::AiHomeOutlined`). 2. Render `<Icon icon=... />` beside text or inside [`Button`](crate::Button) via its `icon` prop. 3. Set `width` / `height` when the glyph should not inherit `1em` from surrounding text. 4. Use [`BaseIcon`](orbital_base_components::BaseIcon) only when composing a custom wrapper that should not add `display: inline-block`. 5. For clickable icons, pass `on_click` and put `aria-label` on a wrapping element (see clickable example).
///
/// # Best Practices
///
/// ## Do's
///
/// * Rely on `currentColor` (default fill) so icons track foreground tokens from the theme * Use `Icon` in product UI; reserve `BaseIcon` for custom compound components * Provide `aria-label` on a wrapper for icon-only interactive controls * Prefer [`Button`](crate::Button) with an `icon` prop for interactive glyphs instead of standalone `on_click` on `Icon`
///
/// ## Don'ts
///
/// * Do not use raw inline SVG when `Icon` or `BaseIcon` already forwards icondata props * Do not omit accessible names on icon-only buttons or links * Do not hard-code fill colors unless the design explicitly requires a fixed glyph color
///
/// # Examples
///
/// ## Default beside label
/// `1em` size and `currentColor` inherit text size and foreground from the parent line.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, Icon};
/// view! {
///     <div data-testid="icon-preview">
///         <Flex align=FlexAlign::Center gap=FlexGap::Small>
///             <Icon icon=icondata::AiHomeOutlined />
///             <span>"Home"</span>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Compare default `1em` with explicit pixel sizes for emphasis tiers.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, FlexWrap, Icon};
/// view! {
///     <div data-testid="icon-size-matrix">
///         <Flex gap=FlexGap::Large align=FlexAlign::Center wrap=FlexWrap::Wrap>
///             <div data-testid="icon-size-default">
///                 <Flex align=FlexAlign::Center gap=FlexGap::Small>
///                     <Icon icon=icondata::AiSettingOutlined />
///                     <span>"1em"</span>
///                 </Flex>
///             </div>
///             <div data-testid="icon-size-16">
///                 <Flex align=FlexAlign::Center gap=FlexGap::Small>
///                     <Icon icon=icondata::AiSettingOutlined width="16px" height="16px" />
///                     <span>"16px"</span>
///                 </Flex>
///             </div>
///             <div data-testid="icon-size-24">
///                 <Flex align=FlexAlign::Center gap=FlexGap::Small>
///                     <Icon icon=icondata::AiSettingOutlined width="24px" height="24px" />
///                     <span>"24px"</span>
///                 </Flex>
///             </div>
///             <div data-testid="icon-size-32">
///                 <Flex align=FlexAlign::Center gap=FlexGap::Small>
///                     <Icon icon=icondata::AiSettingOutlined width="32px" height="32px" />
///                     <span>"32px"</span>
///                 </Flex>
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Color inheritance matrix
/// The same glyph adopts each parent's foreground color via `currentColor`.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, Icon};
/// view! {
///     <div data-testid="icon-color-matrix">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <div data-testid="icon-color-brand" style="color: var(--orb-color-brand-fg); display: flex; align-items: center; gap: 8px;">
///                 <Icon icon=icondata::AiBulbOutlined />
///                 <span>"Brand foreground"</span>
///             </div>
///             <div data-testid="icon-color-danger" style="color: var(--orb-color-palette-red-fg); display: flex; align-items: center; gap: 8px;">
///                 <Icon icon=icondata::AiWarningOutlined />
///                 <span>"Danger foreground"</span>
///             </div>
///             <div data-testid="icon-color-neutral" style="color: var(--orb-color-text-secondary); display: flex; align-items: center; gap: 8px;">
///                 <Icon icon=icondata::AiInfoCircleOutlined />
///                 <span>"Neutral secondary"</span>
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Style override
/// Pass inline `style` for one-off opacity or color overrides on the SVG.
/// <!-- preview -->
/// ```rust
/// use crate::Icon;
/// view! {
///     <div data-testid="icon-style-preview">
///         <Icon
///             icon=icondata::AiStarOutlined
///             width="24px"
///             height="24px"
///             style="opacity: 0.6; color: var(--orb-color-text-secondary);"
///         />
///     </div>
/// }
/// ```
///
/// ## Clickable control
/// Pair `on_click` with `aria-label` on a native wrapper for icon-only actions.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::Icon;
/// view! {
///     <div data-testid="icon-clickable" aria-label="Search" role="button" style="cursor: pointer; display: inline-flex;">
///         <Icon icon=icondata::AiSearchOutlined on_click=Callback::new(|_| {}) />
///     </div>
/// }
/// ```
///
/// ## Base vs core
/// `BaseIcon` is headless; `Icon` adds `display: inline-block` for predictable alignment in text rows.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, Icon};
/// use orbital_base_components::BaseIcon;
/// view! {
///     <div data-testid="icon-base-vs-core">
///         <Flex gap=FlexGap::Large align=FlexAlign::End>
///             <div data-testid="icon-base-cell">
///                 <span>"BaseIcon"</span>
///                 <BaseIcon icon=icondata::AiHomeOutlined width="24px" height="24px" />
///             </div>
///             <div data-testid="icon-core-cell">
///                 <span>"Icon"</span>
///                 <Icon icon=icondata::AiHomeOutlined width="24px" height="24px" />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## In button context
/// Prefer the [`Button`](crate::Button) `icon` prop so labeling and hit targets stay consistent.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="icon-in-button">
///         <Button appearance=ButtonAppearance::Primary icon=icondata::AiSaveOutlined>
///             "Save"
///         </Button>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "icon",
    preview_label = "Icon",
    preview_icon = icondata::AiPictureOutlined,
)]
#[component]
pub fn Icon(
    /// Icon glyph from the icondata catalog.
    #[prop(into)]
    icon: icondata_core::Icon,
    /// Typed icon size (overrides `width` / `height` when set).
    #[prop(optional, into)]
    size: MaybeProp<IconSize>,
    /// Theme color applied to the SVG via inline `color` (maps to `currentColor` fill).
    #[prop(optional, into)]
    color: MaybeProp<ThemeColor>,
    /// SVG width. Defaults to `1em` unless `size` is set.
    #[prop(into, optional)]
    width: MaybeProp<String>,
    /// SVG height. Defaults to `1em` unless `size` is set.
    #[prop(into, optional)]
    height: MaybeProp<String>,
    /// Extra classes merged onto the core layout class and `orbital-icon`.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Inline styles forwarded to [`BaseIcon`].
    #[prop(into, optional)]
    style: MaybeProp<String>,
    /// Click handler; pair with `aria-label` on a wrapper for icon-only controls.
    #[prop(optional)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Icon {
            display: inline-block;
        }
    };
    // Head injection — never emit <style> beside the SVG. Inline style siblings
    // inside buttons pollute accessible names and can desync tachys hydration.
    inject_style("orbital-icon", style_sheet);

    let merged_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            class_names.icon.to_string()
        } else {
            format!("{} {extra}", class_names.icon)
        }
    });

    let resolved_width = Signal::derive(move || {
        if let Some(s) = size.get() {
            s.css_value()
        } else {
            width.get().unwrap_or_else(|| "1em".into())
        }
    });

    let resolved_height = Signal::derive(move || {
        if let Some(s) = size.get() {
            s.css_value()
        } else {
            height.get().unwrap_or_else(|| "1em".into())
        }
    });

    let merged_style = Signal::derive(move || {
        let mut parts = Vec::new();
        if let Some(c) = color.get() {
            parts.push(format!("color: {}", c.css_var()));
        }
        if let Some(extra) = style.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("; "))
        }
    });

    let on_click_cb = on_click.map(|cb| {
        leptos::callback::UnsyncCallback::new(move |ev: leptos::ev::MouseEvent| cb.run(ev))
    });

    match on_click_cb {
        Some(on_click_cb) => view! {
            <BaseIcon
                icon=icon
                width=resolved_width
                height=resolved_height
                class=merged_class
                style=merged_style
                on_click=on_click_cb
            />
        }
        .into_any(),
        None => view! {
            <BaseIcon
                icon=icon
                width=resolved_width
                height=resolved_height
                class=merged_class
                style=merged_style
            />
        }
        .into_any(),
    }
}
