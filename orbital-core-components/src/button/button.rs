use icondata_core::Icon as IconData;
use leptos::{
    either::{Either, EitherOf3},
    prelude::*,
};
use orbital_base_components::{BaseButton, ComponentRef};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::button_styles;
use super::types::{ButtonAppearance, ButtonRef, ButtonShape, ButtonSize, ButtonType};
use crate::Icon;

fn button_icon_view(icon: IconData) -> impl IntoView {
    view! {
        <span class="orbital-button__icon">
            <Icon icon=icon width="1em" height="1em" />
        </span>
    }
}

/// Runs a single command when activated — form submits, dialog confirmations, toolbar actions, and inline commands.
///
/// Pick [`ButtonAppearance::Primary`] for the one main action on a surface, or [`ButtonAppearance::Danger`] for the confirm step of a destructive action. Wire async work with `loading` and `on_click`. For navigation, use [`Link`](crate::Link) instead.
///
/// # When to use
///
/// - Submitting forms, confirming dialogs, or firing one-off commands - Toolbar and card actions where a single clear primary action is needed - Icon-only affordances when space is tight (pair with `aria-label` on a wrapper)
///
/// # Usage
///
/// 1. Pick an [`ButtonAppearance`] — `Primary` for the main action, `Danger` for destructive confirms, `Secondary` for alternatives. 2. Wire `on_click` with [`Callback`] when the button should run logic (not submit a native form). 3. Set `loading` while async work runs; disable the control when input is invalid or work is in-flight. 4. For E2E hooks, wrap the button in a native element with `data-testid` (see project UI rules).
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `appearance=ButtonAppearance::Primary` for the main action on a surface * Show `loading` during async handlers so users see in-progress state * Disable while the form is invalid or a request is outstanding * Use `icon` for recognizable actions (save, search, add)
///
/// ## Don'ts
///
/// * Do not stack multiple primary buttons in one row * Do not use `data-testid` on the component itself — wrap with a native element * Do not use icon-only buttons without an accessible name on the wrapper
///
/// # Button family
///
/// Orbital ships several command controls. When `Button` is not the right fit:
///
/// - **Single command on click** — `Button` (this component). Use [`Link`](crate::Link) for navigation. - **Primary command plus related alternates** — [`ActionMenuButton`](crate::ActionMenuButton) (Save + Save as / Export). - **Menu of options, no primary segment** — [`MenuButton`](crate::MenuButton), or [`Menu`](crate::Menu) for custom triggers. - **Primary label plus supporting line** — [`CompoundButton`](crate::CompoundButton). - **One action pinned to the viewport** — [`FloatingButton`](crate::FloatingButton). - **Primary float plus fan-out secondaries** — [`FloatingActionsMenu`](crate::FloatingActionsMenu). - **Merge adjacent buttons visually** — [`ButtonGroup`](crate::ButtonGroup) (layout only; style each child explicitly). - **Toolbar on/off pressed state** — [`ToggleButton`](crate::ToggleButton). Prefer [`Switch`](crate::Switch) for immediate settings.
///
/// # Examples
///
/// ## Primary button
/// Default call-to-action on a form or dialog footer.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-preview">
///         <Button appearance=ButtonAppearance::Primary>
///             "Save"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Secondary outline
/// Secondary actions beside the primary—cancel, back, or low-commit choices. Outline styling keeps emphasis below Primary without blending into the surface.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-secondary">
///         <Button appearance=ButtonAppearance::Secondary>
///             "Cancel"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Danger
/// The confirm step of a destructive or irreversible action, such as deleting an account. Keep Cancel beside it as Secondary.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-danger">
///         <Button appearance=ButtonAppearance::Secondary>"Cancel"</Button>
///         <Button appearance=ButtonAppearance::Danger>"Delete account"</Button>
///     </div>
/// }
/// ```
///
/// ## Subtle and transparent
/// Low-emphasis actions that blend into the surface until hovered.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-subtle">
///         <Button appearance=ButtonAppearance::Subtle>"More"</Button>
///         <Button appearance=ButtonAppearance::Transparent>"Dismiss"</Button>
///     </div>
/// }
/// ```
///
/// ## With icon
/// Leading icon reinforces the action (save, add, search) while the text label keeps meaning clear for sighted users and screen readers.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-icon">
///         <Button appearance=ButtonAppearance::Primary icon=icondata::AiSaveOutlined>
///             "Save"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Icon-only
/// Compact affordance when toolbar space is tight. Wrap with `aria-label` in app code—the button has no visible text for assistive technologies.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-icon-only">
///         <Button icon=icondata::AiSearchOutlined appearance=ButtonAppearance::Subtle />
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Small fits toolbars and dense rows; medium is the default; large suits prominent mobile CTAs or hero actions. Set each with `size=ButtonSize::…`.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonSize};
/// view! {
///     <div data-testid="button-sizes">
///         <Button size=ButtonSize::Small>"Small"</Button>
///         <Button size=ButtonSize::Medium>"Medium"</Button>
///         <Button size=ButtonSize::Large>"Large"</Button>
///     </div>
/// }
/// ```
///
/// ## Block (full width)
/// Stretches to the full container width—common for mobile form footers, stacked dialog actions, and narrow layouts.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-block">
///         <Button block=true appearance=ButtonAppearance::Primary>
///             "Continue"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Loading state
/// Spinner replaces the icon slot and blocks clicks while async work runs. Pair with disabled form controls to prevent double submission.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-loading">
///         <Button appearance=ButtonAppearance::Primary loading=true>
///             "Saving…"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Click handler
/// <!-- code-only -->
/// ```rust
/// use leptos::prelude::*;
/// view! {
///     <Button
///         appearance=ButtonAppearance::Primary
///         on_click=|_| {}
///     >
///         "Run action"
///     </Button>
/// }
/// ```
///
/// ## Shapes
/// Rounded is the default for labeled buttons. Circular and square fit icon-only controls in toolbars, floating actions, and compact settings grids.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonShape};
/// view! {
///     <div data-testid="button-shapes">
///         <Button shape=ButtonShape::Rounded>"Rounded"</Button>
///         <Button shape=ButtonShape::Circular icon=icondata::AiPlusOutlined />
///         <Button shape=ButtonShape::Square icon=icondata::AiSettingOutlined />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Unavailable actions stay visibly disabled and ignore clicks. `disabled_focusable` keeps the button in tab order for tooltips or custom disabled messaging.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="button-disabled">
///         <Button appearance=ButtonAppearance::Primary disabled=true>
///             "Unavailable"
///         </Button>
///         <Button appearance=ButtonAppearance::Secondary disabled_focusable=true>
///             "Focusable when disabled"
///         </Button>
///     </div>
/// }
/// ```
///
/// ## Theme: primary uses brand token
/// Wrap in `OrbitalThemeProvider` with a custom brand palette so primary buttons use the theme's brand color token.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_theme::{BrandPalette, OrbitalThemeProvider, Theme, ThemeMode};
///
/// view! {
///     <div data-testid="button-theme-brand">
///         <OrbitalThemeProvider theme=RwSignal::new(Theme::with_brand(
///             ThemeMode::Light,
///             BrandPalette { primary: "#E3008C".to_string() },
///         ))>
///             <Button appearance=ButtonAppearance::Primary>"Brand action"</Button>
///         </OrbitalThemeProvider>
///     </div>
/// }
/// ```
///
/// ## Imperative handle
/// ```rust,ignore
/// // Focus or programmatically click via ButtonRef after mount.
/// use crate::{Button, ButtonRef};
/// use orbital_base_components::ComponentRef;
/// let btn_ref = ComponentRef::<ButtonRef>::default();
/// view! {
///     <Button comp_ref=btn_ref appearance=ButtonAppearance::Primary>"Focus me"</Button>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "button",
    preview_label = "Button",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component(transparent)]
pub fn Button(
    /// Extra CSS class names merged onto the root `<button>` element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Visual emphasis: primary, secondary, subtle, or transparent.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// Border shape: rounded (default), circular, or square.
    #[prop(optional, into)]
    shape: Signal<ButtonShape>,
    /// Control size.
    #[prop(optional, into)]
    size: Signal<ButtonSize>,
    /// Native button `type` attribute (`submit`, `reset`, or `button`).
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// When true, the button stretches to the full width of its container.
    #[prop(optional, into)]
    block: Signal<bool>,
    /// Leading icon from the icondata catalog; omit `children` for icon-only buttons.
    #[prop(optional, into)]
    icon: MaybeProp<IconData>,
    /// When true, the button does not respond to clicks or submit actions.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// When true, the button stays focusable while disabled (for tooltips or custom UX).
    #[prop(optional, into)]
    disabled_focusable: Signal<bool>,
    /// When true, shows a spinner and blocks click handling until cleared.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Optional `aria-pressed` for toggle buttons.
    #[prop(optional, into)]
    aria_pressed: MaybeProp<String>,
    /// Handler invoked on click when not disabled or loading.
    #[prop(optional)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Button label text; optional when `icon` alone is sufficient.
    #[prop(optional)]
    children: Option<Children>,
    /// Imperative handle for `focus` and `click` on the underlying DOM button.
    #[prop(optional)]
    comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView {
    inject_style("orbital-button", button_styles());
    let none_children = children.is_none();
    let only_icon = Memo::new(move |_| icon.with(|i| i.is_some()) && none_children);
    let btn_disabled = Memo::new(move |_| disabled.get());

    let appearance_class =
        Signal::derive(move || format!("orbital-button--{}", appearance.get().as_str()));
    let shape_class = Signal::derive(move || format!("orbital-button--{}", shape.get().as_str()));
    let size_class = Signal::derive(move || format!("orbital-button--{}", size.get().as_str()));

    let modifier_class = Signal::derive(move || {
        let mut parts = vec!["orbital-button".to_string()];
        if btn_disabled.get() {
            parts.push("orbital-button--disabled".to_string());
        }
        if block.get() {
            parts.push("orbital-button--block".to_string());
        }
        if only_icon.get() {
            parts.push("orbital-button--only-icon".to_string());
        }
        if icon.with(|i| i.is_some()) {
            parts.push("orbital-button--with-icon".to_string());
        }
        if loading.get() {
            parts.push("orbital-button--loading".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let leading = move || {
        if loading.get() {
            EitherOf3::A(view! {
                <span class="orbital-button__icon">
                    <span class="orbital-button__spinner"></span>
                </span>
            })
        } else if let Some(icon) = icon.get() {
            EitherOf3::B(button_icon_view(icon))
        } else {
            EitherOf3::C(())
        }
    };

    view! {
        <BaseButton
            class=modifier_class
            appearance=appearance_class
            shape=shape_class
            size=size_class
            button_type=button_type
            block=block
            disabled=disabled
            disabled_focusable=disabled_focusable
            loading=loading
            aria_pressed=aria_pressed
            nostrip:on_click=on_click
            comp_ref=comp_ref
        >
            {leading}
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </BaseButton>
    }
}
