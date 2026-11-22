use icondata_core::Icon as IconData;
use leptos::prelude::*;
use orbital_base_components::{BaseFloatingButton, Handler};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::Icon;

use super::styles::floating_button_styles;
use super::types::FloatingButtonConfig;

/// Elevated floating control for the single most important action on a screen.
///
/// Rounded-square icon-only by default with elevated surface styling. Pin to a viewport corner with [`FloatingButtonConfig::fixed`]. Set `config.variant` to [`FloatingButtonVariant::Circular`] for a full circle, or [`FloatingButtonVariant::Extended`] for icon plus label with the same rounded-square corners.
///
/// # When to use
///
/// - One primary action that should stay visible above page content - Back-to-top and similar floating affordances built on the same base
///
/// # Usage
///
/// 1. Provide an icon or extended label via `children`. 2. Set `aria_label` for icon-only buttons. 3. Use `config.fixed(right, bottom)` when the button should float on the viewport.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use one floating primary action per screen for the most important command * Provide a descriptive `aria_label` when no visible text is shown
///
/// ## Don'ts
///
/// * Do not use floating buttons for secondary or destructive actions without clear labeling
///
/// # Examples
///
/// ## Default
/// Rounded-square icon-only floating button with primary emphasis.
/// <!-- preview -->
/// ```rust
/// use crate::FloatingButton;
/// view! {
///     <div data-testid="floating-button-preview">
///         <FloatingButton aria_label="Add".to_string() icon=icondata::AiPlusOutlined />
///     </div>
/// }
/// ```
///
/// ## Extended
/// Extended variant includes a visible text label beside the icon.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingButton, FloatingButtonConfig, FloatingButtonVariant};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="floating-button-extended">
///         <FloatingButton
///             config=FloatingButtonConfig { variant: FloatingButtonVariant::Extended.into(), ..Default::default() }
///             aria_label="Navigate".to_string()
///             icon=icondata::AiCompassOutlined
///         >
///             "Navigate"
///         </FloatingButton>
///     </div>
/// }
/// ```
///
/// ## Size
/// Compare small, medium, and large icon-only sizes.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingButton, FloatingButtonConfig, FloatingButtonSize, Stack, StackConfig, FlexGap};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="floating-button-size">
///         <Stack config=StackConfig::horizontal(FlexGap::Medium)>
///             <FloatingButton
///                 config=FloatingButtonConfig { size: FloatingButtonSize::Small.into(), ..Default::default() }
///                 aria_label="Add small".to_string()
///                 icon=icondata::AiPlusOutlined
///             />
///             <FloatingButton
///                 config=FloatingButtonConfig { size: FloatingButtonSize::Medium.into(), ..Default::default() }
///                 aria_label="Add medium".to_string()
///                 icon=icondata::AiPlusOutlined
///             />
///             <FloatingButton aria_label="Add large".to_string() icon=icondata::AiPlusOutlined />
///         </Stack>
///     </div>
/// }
/// ```
///
/// ## Secondary color
/// Secondary treatment for less prominent floating actions.
/// <!-- preview -->
/// ```rust
/// use crate::{FloatingButton, FloatingButtonColor, FloatingButtonConfig};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="floating-button-secondary">
///         <FloatingButton
///             config=FloatingButtonConfig { color: FloatingButtonColor::Secondary.into(), ..Default::default() }
///             aria_label="Edit".to_string()
///             icon=icondata::AiEditOutlined
///         />
///     </div>
/// }
/// ```
///
/// ## Click handler
/// `on_click` runs the action; no wrapper element is needed.
/// <!-- preview -->
/// ```rust
/// use crate::FloatingButton;
/// let clicks = RwSignal::new(0u32);
/// view! {
///     <div data-testid="floating-button-click">
///         <FloatingButton
///             aria_label="New project".to_string()
///             icon=icondata::AiPlusOutlined
///             on_click=Callback::new(move |_| clicks.update(|n| *n += 1))
///         />
///         <span data-testid="floating-button-click-count">{move || clicks.get().to_string()}</span>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "floating-button",
    preview_label = "Floating Button",
    preview_icon = icondata::AiPlusCircleOutlined,
)]
#[component]
pub fn FloatingButton(
    /// Color, size, variant, and optional fixed viewport placement.
    #[prop(default = FloatingButtonConfig::default())]
    config: FloatingButtonConfig,
    /// Accessible name for icon-only buttons.
    #[prop(into)]
    aria_label: String,
    /// Optional leading icon from the icondata catalog.
    #[prop(optional, into)]
    icon: MaybeProp<IconData>,
    /// Optional CSS class on the button element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// When true, the button does not respond to clicks.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,
    /// Runs on click (not while disabled).
    #[prop(optional)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Optional visible label for the extended variant.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-floating-button", floating_button_styles());

    let button_class = Signal::derive(move || {
        let mut parts = vec![
            "orbital-floating-button".to_string(),
            format!("orbital-floating-button--{}", config.color.get().as_str()),
            format!("orbital-floating-button--{}", config.size.get().as_str()),
            format!("orbital-floating-button--{}", config.variant.get().as_str()),
        ];
        if config.right.is_some() && config.bottom.is_some() {
            parts.push("orbital-floating-button--fixed".to_string());
        }
        if disabled.get() {
            parts.push("orbital-floating-button--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    });

    let style: MaybeProp<String> = Memo::new(move |_| match (config.right, config.bottom) {
        (Some(right), Some(bottom)) => {
            format!("right: {}px; bottom: {}px", right.get(), bottom.get())
        }
        _ => String::new(),
    })
    .into();

    view! {
        <BaseFloatingButton
            class=button_class
            style=style
            aria_label=aria_label
            disabled=disabled
            nostrip:on_click=on_click.map(|cb| Handler::on(move |ev| cb.run(ev)))
            testid=MaybeProp::from("floating-button".to_string())
        >
            {icon.get().map(|icon| view! {
                <span class="orbital-floating-button__icon">
                    <Icon icon=icon />
                </span>
            })}
            {children.map(|children| children())}
        </BaseFloatingButton>
    }
}
