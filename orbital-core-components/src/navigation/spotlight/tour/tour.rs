use leptos::prelude::*;
use orbital_base_components::{
    build_anchor_arrow, AnchoredSurface, OpenBind, OverlayAppearance, OverlayDismiss, Placement,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::navigation::popover::popover_styles;
use crate::overlay::{overlay_surface_class, FloatingPanel};
use crate::{Backdrop, PopoverAppearance, PopoverSize, SpotlightBackdrop};

use super::super::arrow::spotlight_arrow;
use super::super::backdrop_map::spotlight_backdrop_config;
use super::super::styles::spotlight_styles;
use super::injection::{SpotlightTourInjection, SpotlightTourState};

/// `SpotlightTour` walks users through several UI targets in order. Add [`SpotlightTourStep`]
/// children with optional `anchor_id` values matching stable DOM `id` attributes (omit to
/// center in the viewport) and optional per-step anatomy slots; bind `open` to start the tour.
/// The built-in footer provides Back, Next, and Finish. Wire `on_finish` to record completion
/// in your app state. Prefer [`SpotlightPopover`](super::super::popover::SpotlightPopover) when
/// the user opens help from a button; prefer this component when the app drives a multi-step
/// walkthrough. Skip and dismiss are not first-class props yet.
///
/// # Spotlight coaching
///
/// - **Single programmatic step** — [`SpotlightTip`](super::super::tip::SpotlightTip)
/// - **Multi-step walkthrough** — `SpotlightTour` (this component)
///
/// # Examples
///
/// ## Multi-step tour
/// Declarative steps with built-in next/previous navigation and a moving anchor.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, SpotlightBody, SpotlightHeader, SpotlightTour, SpotlightTourStep,
/// };
/// use leptos::prelude::*;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="spotlight-tour">
///         <Button on:click=move |_| open.set(true)>"Start tour"</Button>
///         <div id="spotlight-tour-target-1" data-testid="spotlight-tour-target-1" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "Toolbar"
///         </div>
///         <div id="spotlight-tour-target-2" data-testid="spotlight-tour-target-2" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "Sidebar"
///         </div>
///         <SpotlightTour open=open>
///             <SpotlightTourStep anchor_id="spotlight-tour-target-1">
///                 <SpotlightHeader slot>"Step 1"</SpotlightHeader>
///                 <SpotlightBody slot>"Filter events from the toolbar."</SpotlightBody>
///             </SpotlightTourStep>
///             <SpotlightTourStep anchor_id="spotlight-tour-target-2">
///                 <SpotlightHeader slot>"Step 2"</SpotlightHeader>
///                 <SpotlightBody slot>"Navigate between sections."</SpotlightBody>
///             </SpotlightTourStep>
///         </SpotlightTour>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Spotlight",
    preview_slug = "spotlight-tour",
    preview_label = "Spotlight Tour",
    preview_icon = icondata::AiFlagOutlined,
)]
#[component]
pub fn SpotlightTour(
    #[prop(into)] open: OpenBind,
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: MaybeProp<PopoverAppearance>,
    #[prop(optional, into)] size: Signal<PopoverSize>,
    #[prop(optional)] on_finish: Option<Callback<()>>,
    #[prop(default = SpotlightBackdrop::tour_default())] backdrop: SpotlightBackdrop,
) -> impl IntoView {
    inject_style("orbital-spotlight", spotlight_styles());
    inject_style("orbital-popover", popover_styles());

    let open_rw = match open {
        OpenBind::Signal(signal) => signal,
        OpenBind::ReadWrite(read, write) => {
            let rw = RwSignal::new(read.get_untracked());
            Effect::new(move |_| {
                let value = read.get();
                if rw.get_untracked() != value {
                    rw.set(value);
                }
            });
            Effect::new(move |_| {
                write.set(rw.get());
            });
            rw
        }
    };

    let tour_state = SpotlightTourState::new(open_rw, on_finish);
    provide_context(SpotlightTourInjection(tour_state));

    provide_context(OverlayDismiss {
        close: Callback::new(move |_| tour_state.dismiss()),
    });

    let show: Signal<bool> = tour_state.open.read_only().into();
    let anchor_id = Signal::derive(move || tour_state.anchor_for_active());
    let placement = Signal::derive(move || Placement::from(tour_state.placement_for_active()));

    let overlay_appearance = appearance
        .get()
        .map(|a: PopoverAppearance| a.into())
        .unwrap_or(OverlayAppearance::Default);
    let panel_size = StoredValue::new(size.get_untracked());
    let surface_class = Signal::derive(move || {
        let mut parts = vec![overlay_surface_class(
            "orbital-popover-surface",
            overlay_appearance,
            Some(panel_size.get_value().as_str()),
        )];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let arrow_ref = NodeRef::<leptos::html::Div>::new();
    let anchor_arrow = build_anchor_arrow(arrow_ref);
    let backdrop_layer =
        spotlight_backdrop_config(OpenBind::Signal(tour_state.open), backdrop, anchor_id);

    view! {
        {backdrop_layer.map(|(config, on_click)| match on_click {
            Some(handler) => view! {
                <Backdrop
                    class="orbital-spotlight-portal__backdrop".to_string()
                    config=config
                    on_click=handler
                />
            }.into_any(),
            None => view! {
                <Backdrop
                    class="orbital-spotlight-portal__backdrop".to_string()
                    config=config
                />
            }.into_any(),
        })}
        <AnchoredSurface
            show=show
            anchor_id=anchor_id
            placement=placement
            arrow=anchor_arrow
        >
            <div class="orbital-popover-shell orbital-spotlight">
                {spotlight_arrow(arrow_ref)}
                <FloatingPanel
                    class=surface_class
                    body_class="orbital-popover-body"
                >
                    <div aria-live="polite" class="orbital-spotlight">
                        {children()}
                    </div>
                </FloatingPanel>
            </div>
        </AnchoredSurface>
    }
}
