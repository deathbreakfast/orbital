use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_macros::component_doc;

use crate::{Button, ButtonAppearance};

/// Stays visually pressed until clicked again — bind `pressed` to a `RwSignal<bool>` for toolbar modes like bold or italic.
///
/// The control sets `aria-pressed` and swaps appearance between subtle and secondary. Label text should communicate both the action and current state (for example, "Bold, on"). For settings that apply immediately, prefer [`Switch`](crate::Switch); for grouped exclusive choices, compose multiple toggles manually until a group wrapper ships.
///
/// # When to use
///
/// - Rich text toolbar actions (bold, italic, underline) - View mode switches that are not instant settings - Binary states where pressed appearance should persist
///
/// # Usage
///
/// 1. Bind `pressed` to a boolean signal for two-way toggle state. 2. Optionally pass `on_click` to react when the pressed state changes. 3. Put label text in children.
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind `pressed` with a signal for two-way sync * Use short, action-oriented labels in toolbars
///
/// ## Don'ts
///
/// * Do not use for settings that apply immediately — prefer [`Switch`](crate::Switch) * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default toggle
/// Starts unpressed; each click toggles the pressed state and updates `aria-pressed`.
/// <!-- preview -->
/// ```rust
/// use crate::ToggleButton;
/// let pressed = RwSignal::new(false);
/// view! {
///     <div data-testid="toggle-button-preview">
///         <ToggleButton pressed=pressed>"Bold"</ToggleButton>
///     </div>
/// }
/// ```
///
/// ## Starts pressed
/// Initial pressed state for formatting actions that are already active in the editor.
/// <!-- preview -->
/// ```rust
/// use crate::ToggleButton;
/// let pressed = RwSignal::new(true);
/// view! {
///     <div data-testid="toggle-button-on">
///         <ToggleButton pressed=pressed>"Italic"</ToggleButton>
///     </div>
/// }
/// ```
///
/// ## Click toggles state
/// Two-way binding keeps the pressed signal in sync as users click the control on and off.
/// <!-- preview -->
/// ```rust
/// use crate::ToggleButton;
/// let pressed = RwSignal::new(false);
/// view! {
///     <div data-testid="toggle-button-click">
///         <ToggleButton pressed=pressed>"Underline"</ToggleButton>
///     </div>
/// }
/// ```
///
/// ## Fires on_click handler
/// Optional callback runs after each toggle with the new pressed value for app-side reactions.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::Handler;
/// use crate::ToggleButton;
/// let pressed = RwSignal::new(false);
/// let callback_seen = RwSignal::new(false);
/// view! {
///     <div data-testid="toggle-button-handler">
///         <ToggleButton
///             pressed=pressed
///             on_click=Handler::on(move |_next_pressed: bool| callback_seen.set(true))
///         >
///             "Track click"
///         </ToggleButton>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// `disabled` blocks toggling while the action is unavailable; the pressed state stays as it was.
/// <!-- preview -->
/// ```rust
/// use crate::ToggleButton;
/// let pressed = RwSignal::new(true);
/// view! {
///     <div data-testid="toggle-button-disabled">
///         <ToggleButton pressed=pressed disabled=true>"Ready to start"</ToggleButton>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "toggle-button",
    preview_label = "Toggle Button",
    preview_icon = icondata::AiBoldOutlined,
)]
#[component]
pub fn ToggleButton(
    /// Two-way pressed state for the toggle.
    #[prop(optional, into)]
    pressed: RwSignal<bool>,
    /// Callback invoked after toggle with the new pressed value.
    #[prop(optional, into)]
    on_click: Option<Handler<bool>>,
    /// When true, the toggle is disabled and keeps its pressed state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Optional button label; omit for icon-only toggles (pair with `aria-label` on a wrapper).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let appearance = Signal::derive(move || {
        if pressed.get() {
            ButtonAppearance::Secondary
        } else {
            ButtonAppearance::Subtle
        }
    });

    let handle_click = Callback::new(move |_ev: leptos::ev::MouseEvent| {
        pressed.update(|state| *state = !*state);
        if let Some(handler) = on_click.as_ref() {
            handler.run(pressed.get_untracked());
        }
    });

    view! {
        <Button
            appearance=appearance
            aria_pressed=Signal::derive(move || if pressed.get() { "true".to_string() } else { "false".to_string() })
            on_click=handle_click
            disabled=disabled
        >
            {children.map(|c| c())}
        </Button>
    }
}
