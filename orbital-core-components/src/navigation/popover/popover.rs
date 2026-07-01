use leptos::prelude::*;
use orbital_base_components::{BasePopover, OverlayAppearance, PopoverEvents as BasePopoverEvents};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::panel::PopoverPanel;
use super::styles::popover_styles;
use super::types::{
    PopoverAppearance, PopoverConfig, PopoverLifecycle, PopoverPosition, PopoverSize,
    PopoverTriggerType,
};
use crate::overlay::overlay_surface_class;
use crate::PopoverTrigger;

/// `Popover` anchors a floating panel to a trigger — detail cards, compact pickers, or
/// on-demand content. Put the anchor in [`PopoverTrigger`]; use `trigger_type=Click` when
/// the panel has inputs or must stay open. Tune `size`, `position`, and `appearance`; hook
/// `lifecycle` to load data on first open.
///
/// # When to use
///
/// - Detail cards, compact pickers, or rich content on demand
/// - Panels that should not block the whole page — unlike [`Dialog`](crate::Dialog)
/// - Brief hover hints — use [`Tooltip`](crate::Tooltip) instead
///
/// # Overlay surfaces
///
/// - **Brief non-interactive hint** — [`Tooltip`](crate::Tooltip)
/// - **Floating panel with content or inputs** — `Popover` (this component)
/// - **List of actions from a trigger** — [`Menu`](crate::Menu) or [`MenuButton`](crate::MenuButton)
/// - **Block the page or trap focus** — [`Dialog`](crate::Dialog)
///
/// # Usage
///
/// 1. Put the anchor in [`PopoverTrigger`] (often [`crate::Button`]).
/// 2. Set `trigger_type`: `Hover` (default) or `Click` when the panel has inputs or must stay open.
/// 3. Optional: `appearance`, `size`, `position`, `lifecycle` for lazy-load on first open.
/// 4. Render panel body as remaining children.
///
/// ```rust
/// use crate::Button;
/// view! {
///     <Popover trigger_type=PopoverTriggerType::Click>
///         <PopoverTrigger slot>
///             <Button>"Details"</Button>
///         </PopoverTrigger>
///         <p>"Panel content"</p>
///     </Popover>
/// }
/// ```
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `Click` when the panel contains inputs or must stay open * Use `on_open` / `on_close` to lazy-load panel data * Match `size` to content density (`Small` for hints, `Large` for forms)
///
/// ## Don'ts
///
/// * Do not rely on a popover as the only place for irreversible actions — use [`Dialog`](crate::Dialog) * Do not nest popovers deeply — use a dialog for multi-step flows
///
/// # Examples
///
/// ## Click popover
/// Explicit click opens a floating panel—required when the panel contains inputs, links, or must stay open while the user interacts.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-preview">
///         <Popover trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Open popover"</Button>
///             </PopoverTrigger>
///             <div data-testid="popover-content">"Popover body content"</div>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Hover popover (default trigger)
/// PopoverTrigger defaults to hover for lightweight supplementary content shown on pointer enter. Use for read-only detail cards, not interactive forms.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// view! {
///     <div data-testid="popover-hover">
///         <Popover>
///             <PopoverTrigger slot>
///                 <Button appearance=ButtonAppearance::Subtle>"Hover"</Button>
///             </PopoverTrigger>
///             <span data-testid="popover-hover-body">"Details on hover"</span>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Brand appearance
/// Brand-colored popover surface aligned with primary theme tokens for contextual panels in branded chrome.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-brand">
///         <Popover appearance=PopoverAppearance::Brand trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Brand popover"</Button>
///             </PopoverTrigger>
///             <div data-testid="popover-brand-body">"Branded surface"</div>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Inverted appearance
/// Dark inverted panel for anchors on dark surfaces or high-contrast overlays.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-inverted">
///         <Popover appearance=PopoverAppearance::Inverted trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Inverted"</Button>
///             </PopoverTrigger>
///             <div data-testid="popover-inverted-body">"Dark surface"</div>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Small size
/// Compact width for short hints or single-line pickers.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-small">
///         <Popover size=PopoverSize::Small trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Small"</Button>
///             </PopoverTrigger>
///             <span>"Compact panel"</span>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Large size
/// Roomier panel for mini-forms or short lists without upgrading to a modal dialog.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-large">
///         <Popover size=PopoverSize::Large trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Large"</Button>
///             </PopoverTrigger>
///             <div style="padding: 8px;">"Roomier panel"</div>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Position (left)
/// Opens beside the trigger when vertical space is constrained.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// view! {
///     <div data-testid="popover-left">
///         <Popover position=PopoverPosition::Left trigger_type=PopoverTriggerType::Click>
///             <PopoverTrigger slot>
///                 <Button>"Left"</Button>
///             </PopoverTrigger>
///             <span>"Opens to the left"</span>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Lifecycle hooks
/// Use `on_open` and `on_close` to lazy-load panel data or record analytics when visibility changes.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::Handler;
/// use crate::{Button, PopoverLifecycle, PopoverTriggerType};
///
/// let opened = RwSignal::new(false);
/// view! {
///     <div data-testid="popover-lifecycle">
///         <Popover
///             trigger_type=PopoverTriggerType::Click
///             lifecycle=PopoverLifecycle {
///                 on_open: Some(Handler::new(move || opened.set(true))),
///                 on_close: Some(Handler::new(move || opened.set(false))),
///             }
///         >
///             <PopoverTrigger slot>
///                 <Button>"Lifecycle"</Button>
///             </PopoverTrigger>
///             <span data-opened=move || opened.get()>"Panel body"</span>
///         </Popover>
///     </div>
/// }
/// ```
///
/// ## Inner scroll area
/// Popover trigger inside a bounded [`ScrollArea`](crate::ScrollArea). Scrolling the scrollport after the panel opens should keep the teleported surface aligned with the trigger.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, PopoverTriggerType, ScrollArea};
///
/// const FRAME: &str = "display: block; width: 100%; height: 280px; box-sizing: border-box; border: 1px solid var(--orb-color-border-default); padding: 12px; background: var(--orb-color-surface-subtle);";
/// view! {
///     <div data-testid="popover-scroll-area" style="width: 100%; max-width: 560px;">
///         <ScrollArea scroll_testid="popover-scroll-area-scrollport" style=FRAME>
///             <div style="display: flex; flex-direction: column; gap: 12px; min-height: 720px;">
///                 {(0..10)
///                     .map(|i| view! { <p style="margin: 0;">{format!("Filler line {i}")}</p> })
///                     .collect_view()}
///                 <Popover trigger_type=PopoverTriggerType::Click>
///                     <PopoverTrigger slot>
///                         <Button>"Open in scroll area"</Button>
///                     </PopoverTrigger>
///                     <div data-testid="popover-scroll-area-content">"Anchored panel"</div>
///                 </Popover>
///                 {(0..10)
///                     .map(|i| view! { <p style="margin: 0;">{format!("Trailing line {i}")}</p> })
///                     .collect_view()}
///             </div>
///         </ScrollArea>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "popover",
    preview_label = "Popover",
    preview_icon = icondata::AiExpandAltOutlined,
)]
#[component]
pub fn Popover<T>(
    /// Optional CSS class on the popover surface.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// How the popover opens: `Hover` (default) or `Click`.
    #[prop(optional)]
    trigger_type: PopoverTriggerType,
    /// Slot: anchor element or component (typically [`crate::Button`]).
    popover_trigger: PopoverTrigger<T>,
    /// Placement relative to the trigger (`Top` default).
    #[prop(optional)]
    position: PopoverPosition,
    /// Visual variant: `Brand` or `Inverted`; omit for default surface.
    #[prop(optional, into)]
    appearance: MaybeProp<PopoverAppearance>,
    /// Panel width preset: `Small`, `Medium` (default), or `Large`.
    #[prop(optional, into)]
    size: Signal<PopoverSize>,
    /// Open/close lifecycle hooks.
    #[prop(optional)]
    lifecycle: PopoverLifecycle,
    /// Portal mount target. When omitted, the panel portals to `document.body`.
    #[prop(default = None)]
    mount: Option<NodeRef<leptos::html::Div>>,
    /// Panel body (any view).
    children: Children,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    inject_style("orbital-popover", popover_styles());

    let config = PopoverConfig {
        trigger_type,
        position,
        appearance,
        size,
    };

    let base_events = BasePopoverEvents {
        on_open: lifecycle
            .on_open
            .map(|cb| Callback::new(move |_| cb.run(()))),
        on_close: lifecycle
            .on_close
            .map(|cb| Callback::new(move |_| cb.run(()))),
    };

    let appearance = config
        .appearance
        .get()
        .map(|a: PopoverAppearance| a.into())
        .unwrap_or(OverlayAppearance::Default);

    let size = StoredValue::new(config.size.get_untracked());
    let surface_class = Signal::derive(move || {
        overlay_surface_class(
            "orbital-popover-surface",
            appearance,
            Some(size.get_value().as_str()),
        )
    });

    view! {
        <BasePopover
            trigger_type=config.trigger_type.into()
            placement=config.position.into()
            appearance=appearance
            size=config.size.get_untracked().into()
            events=base_events
            class=class
            mount=mount
            overlay_trigger=popover_trigger
        >
            <PopoverPanel class=surface_class>
                {children()}
            </PopoverPanel>
        </BasePopover>
    }
}
