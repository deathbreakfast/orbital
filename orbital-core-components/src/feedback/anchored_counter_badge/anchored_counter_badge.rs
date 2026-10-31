use leptos::{ev, leptos_dom::helpers::WindowListenerHandle, prelude::*};
use orbital_base_components::overlay::dom_events::{
    add_event_listener_capture, get_scroll_parent_node, EventListenerHandle,
};
use orbital_base_components::{BadgeAppearance, BadgeColor, BadgeSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::{closure::Closure, JsCast};

use super::position::{badge_point, BadgeCorner, Rect};
use super::styles::anchored_counter_badge_styles;
use crate::overlay::ThemedPortal;
use crate::CounterBadge;

/// Pins a clickable count to the corner of a page element the caller doesn't render, found by its `id`.
///
/// The badge stays on that corner as the page or any scroll container around the element
/// scrolls, as the window resizes, and as the element itself changes size. It hides while the
/// corner is scrolled out of view or the element is missing. The button is at least 24×24px
/// and needs an `aria_label` that says what the count is.
///
/// Use [`CounterBadge`](crate::CounterBadge) with children instead when you render the host
/// yourself, and [`SpotlightTip`](crate::SpotlightTip) for a coaching panel beside an element.
///
/// The element must be in the DOM when the badge mounts or when `anchor_id` changes.
///
/// # Examples
///
/// ## On page elements
/// Badges on three elements inside a scroll container. Scroll the container, resize the chart
/// or the window, and the badges follow; click one to open that element's discussion.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{AnchoredCounterBadge, Button};
/// use leptos::prelude::*;
/// let clicked = RwSignal::new(String::from("none"));
/// let wide = RwSignal::new(false);
/// let target = "padding: 12px; border: 1px solid var(--orb-color-border-subtle); border-radius: var(--orb-radius-md);";
/// view! {
///     <div data-testid="anchored-counter-badge-preview">
///         <Button on:click=move |_| wide.update(|w| *w = !*w)>"Resize chart"</Button>
///         <div
///             data-testid="anchored-counter-badge-scroller"
///             style="height: 240px; overflow: auto; margin-top: 12px; padding: 16px; display: flex; flex-direction: column; gap: 24px; border: 1px solid var(--orb-color-border-subtle);"
///         >
///             <div
///                 id="acb-results-chart"
///                 data-testid="acb-results-chart"
///                 style=move || format!("{target} flex: none; height: 96px; width: {};", if wide.get() { "85%" } else { "55%" })
///             >
///                 "Results chart"
///             </div>
///             <div id="acb-ballot" data-testid="acb-ballot" style=format!("{target} flex: none; width: 55%;")>"Ballot"</div>
///             <div id="acb-comments-hub" data-testid="acb-comments-hub" style=format!("{target} flex: none; width: 55%;")>"Comments hub"</div>
///             <div style="flex: none; height: 320px;"></div>
///         </div>
///         <p data-testid="anchored-counter-badge-clicked">"Opened: " {move || clicked.get()}</p>
///         <AnchoredCounterBadge
///             anchor_id="acb-results-chart"
///             count=3
///             aria_label="3 comments on Results chart"
///             on_click=move |_| clicked.set("Results chart".into())
///         />
///         <AnchoredCounterBadge
///             anchor_id="acb-ballot"
///             count=42
///             aria_label="42 comments on Ballot"
///             on_click=move |_| clicked.set("Ballot".into())
///         />
///         <AnchoredCounterBadge
///             anchor_id="acb-comments-hub"
///             count=1200
///             overflow_count=999
///             aria_label="1200 comments on Comments hub"
///             on_click=move |_| clicked.set("Comments hub".into())
///         />
///     </div>
/// }
/// ```
///
/// ## Corners
/// `corner` picks which corner of the element the badge sits on.
/// <!-- preview -->
/// ```rust
/// use crate::{AnchoredCounterBadge, BadgeCorner};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="anchored-counter-badge-corners" style="padding: 24px;">
///         <div
///             id="acb-corners-target"
///             data-testid="acb-corners-target"
///             style="height: 96px; padding: 12px; border: 1px solid var(--orb-color-border-subtle); border-radius: var(--orb-radius-md);"
///         >
///             "Option grid"
///         </div>
///         <AnchoredCounterBadge anchor_id="acb-corners-target" count=1 corner=BadgeCorner::TopStart aria_label="Top start" on_click=move |_| {} />
///         <AnchoredCounterBadge anchor_id="acb-corners-target" count=2 corner=BadgeCorner::TopEnd aria_label="Top end" on_click=move |_| {} />
///         <AnchoredCounterBadge anchor_id="acb-corners-target" count=3 corner=BadgeCorner::BottomStart aria_label="Bottom start" on_click=move |_| {} />
///         <AnchoredCounterBadge anchor_id="acb-corners-target" count=4 corner=BadgeCorner::BottomEnd aria_label="Bottom end" on_click=move |_| {} />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "anchored-counter-badge",
    preview_label = "Anchored Counter Badge",
    preview_icon = icondata::AiNumberOutlined,
)]
#[component]
pub fn AnchoredCounterBadge(
    /// `id` of the page element to sit on. Empty, `None`, or an id with no element renders nothing.
    #[prop(into)]
    anchor_id: MaybeProp<String>,
    /// Count value to display.
    #[prop(into)]
    count: Signal<u32>,
    /// Maximum count before showing overflow (e.g. `999+`). Defaults to 99.
    #[prop(optional)]
    overflow_count: Option<u32>,
    /// Corner of the element the badge is centred on.
    #[prop(optional, into)]
    corner: Signal<BadgeCorner>,
    /// Accessible name for the button, e.g. "3 comments on Results chart".
    #[prop(into)]
    aria_label: Signal<String>,
    /// Called when the badge is clicked or activated from the keyboard.
    #[prop(into)]
    on_click: Callback<()>,
    /// Visual style preset.
    #[prop(optional, into)]
    appearance: Signal<BadgeAppearance>,
    /// Size preset.
    #[prop(optional, into)]
    size: Signal<BadgeSize>,
    /// Semantic color preset.
    #[prop(optional, into)]
    color: Signal<BadgeColor>,
) -> impl IntoView {
    inject_style(
        "orbital-anchored-counter-badge",
        anchored_counter_badge_styles(),
    );

    let overflow = overflow_count.unwrap_or(99);
    let point = RwSignal::new(None::<(f64, f64)>);
    let tracking = StoredValue::new_local(None::<Tracking>);
    let scroll_handle = StoredValue::new(None::<EventListenerHandle>);
    let resize_handle = StoredValue::new(None::<WindowListenerHandle>);

    let sync = move || {
        let next = tracking
            .try_with_value(|t| {
                let t = t.as_ref()?;
                let clip = t
                    .scroll_parents
                    .iter()
                    .fold(viewport_rect(), |clip, el| clip.intersect(element_rect(el)));
                badge_point(element_rect(&t.anchor), corner.get_untracked(), clip)
            })
            .flatten();
        if point.try_get_untracked() != Some(next) {
            point.try_set(next);
        }
    };

    let stop = move || {
        if let Some(handle) = scroll_handle.try_update_value(Option::take).flatten() {
            handle.remove();
        }
        if let Some(handle) = resize_handle.try_update_value(Option::take).flatten() {
            handle.remove();
        }
        if let Some(t) = tracking.try_update_value(Option::take).flatten() {
            if let Some(observer) = t.observer {
                observer.disconnect();
            }
        }
    };

    // Effects only run in the browser, so SSR renders nothing and attaches no listeners.
    Effect::new(move |_| {
        let id = anchor_id.get();
        corner.track();
        stop();

        let Some(anchor) = id
            .filter(|id| !id.is_empty())
            .and_then(|id| document().get_element_by_id(&id))
        else {
            point.set(None);
            return;
        };

        // Fires once on observe, then whenever the element's box changes size.
        let on_resize = Closure::<dyn FnMut()>::new(sync);
        let observer = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()).ok();
        if let Some(observer) = &observer {
            observer.observe(&anchor);
        }
        tracking.set_value(Some(Tracking {
            scroll_parents: scroll_parents(&anchor),
            anchor,
            observer,
            _on_resize: on_resize,
        }));

        // Scroll events don't bubble, but a capture listener on the document sees every
        // scroll container's events, including ones added after the badge mounted.
        scroll_handle.set_value(Some(add_event_listener_capture(
            document(),
            ev::scroll,
            move |_| sync(),
        )));
        resize_handle.set_value(Some(window_event_listener(ev::resize, move |_| sync())));
        sync();
    });

    on_cleanup(stop);

    let style = move || {
        point
            .get()
            .map(|(x, y)| format!("left: {x}px; top: {y}px;"))
            .unwrap_or_default()
    };

    view! {
        <ThemedPortal>
            <button
                type="button"
                class="orbital-anchored-counter-badge"
                style=style
                hidden=move || point.get().is_none()
                aria-label=move || aria_label.get()
                on:click=move |_| on_click.run(())
            >
                {move || {
                    view! {
                        <CounterBadge
                            count=count.get()
                            overflow_count=overflow
                            appearance=appearance
                            size=size
                            color=color
                        />
                    }
                }}
            </button>
        </ThemedPortal>
    }
}

/// DOM state for the current anchor, replaced whenever `anchor_id` or `corner` changes.
struct Tracking {
    anchor: web_sys::Element,
    /// Scroll containers around the anchor, nearest first; their boxes clip the badge.
    scroll_parents: Vec<web_sys::Element>,
    observer: Option<web_sys::ResizeObserver>,
    /// Keeps the ResizeObserver callback alive until the observer is disconnected.
    _on_resize: Closure<dyn FnMut()>,
}

fn element_rect(el: &web_sys::Element) -> Rect {
    let r = el.get_bounding_client_rect();
    Rect::new(r.left(), r.top(), r.right(), r.bottom())
}

fn viewport_rect() -> Rect {
    let window = window();
    let width = window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let height = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    Rect::new(0.0, 0.0, width, height)
}

fn scroll_parents(anchor: &web_sys::Element) -> Vec<web_sys::Element> {
    let mut parents = Vec::new();
    let mut cursor = get_scroll_parent_node(anchor);
    while let Some(node) = cursor {
        cursor = get_scroll_parent_node(&node);
        if let Ok(el) = node.dyn_into::<web_sys::Element>() {
            parents.push(el);
        }
    }
    parents
}
