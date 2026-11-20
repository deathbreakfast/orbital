//! The `GraphMinimap` component: an overview of the whole graph with a
//! rectangle for the part in view; click or drag on it to move the view.

use leptos::ev::PointerEvent;
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use super::graph::GraphHandle;
use super::styles::GRAPH_STYLES;
use super::viewport::FIT_PADDING;

/// A layout rect `(x, y, width, height)`.
type Rect = (f64, f64, f64, f64);

/// Maps a point on a `mw` × `mh` minimap to layout coordinates. The minimap
/// shows the `lw` × `lh` layout plus [`FIT_PADDING`] on every side, scaled to
/// fit and centred (SVG `xMidYMid meet`).
pub(crate) fn minimap_point(px: f64, py: f64, mw: f64, mh: f64, lw: f64, lh: f64) -> (f64, f64) {
    let bw = lw + 2.0 * FIT_PADDING;
    let bh = lh + 2.0 * FIT_PADDING;
    let k = (mw / bw).min(mh / bh);
    if !k.is_finite() || k <= 0.0 {
        return (lw / 2.0, lh / 2.0);
    }
    let ox = (mw - bw * k) / 2.0;
    let oy = (mh - bh * k) / 2.0;
    ((px - ox) / k - FIT_PADDING, (py - oy) / k - FIT_PADDING)
}

/// Overview of a [`Graph`](crate::Graph): its group boxes (or its nodes when
/// it has no groups) and a rectangle for the part in view. Click or drag on
/// it to move the view there.
///
/// The graph draws one in its bottom-right corner while its `minimap` signal
/// is on (the [`GraphToolbar`](crate::GraphToolbar)'s Minimap button). Place
/// a `GraphMinimap` yourself only to show the overview somewhere else, such
/// as a side panel; it then shows whatever the `minimap` signal says.
///
/// # When to use
///
/// - Large graphs that users zoom into, so they keep their bearings and can
///   jump across the drawing without panning the whole way.
/// - Not for graphs that always fit their container: the minimap hides
///   itself while the whole graph is in view.
///
/// # Usage
///
/// 1. Give the [`Graph`](crate::Graph) a `minimap` signal (off by default)
///    and a [`GraphToolbar`](crate::GraphToolbar) in its `toolbar` slot; the
///    toolbar's Minimap button flips the signal.
/// 2. To place the overview yourself, pass the graph's [`GraphHandle`] as
///    `handle`, or render the minimap inside the graph's `toolbar` slot,
///    where it finds the handle by itself.
///
/// The minimap redraws when the view moves without rerunning the layout. It
/// is hidden from screen readers (`aria-hidden`) and takes no focus: the
/// toolbar's Fit button and the graph's keyboard reach everything it does.
///
/// # Best Practices
///
/// ## Do's
///
/// - Own the `minimap` signal in the host when users expect the choice to
///   stick between visits.
/// - Leave it off by default for small graphs.
///
/// ## Don'ts
///
/// - Don't make the minimap the only way to reach part of the graph; panning,
///   zooming and Fit must still work.
/// - Don't place a second minimap while the graph's own one is on.
///
/// # Examples
///
/// ## Minimap in a zoomed graph
/// The graph starts with its minimap on; zoom in with the toolbar or a
/// double-click and the overview appears in the corner. Click it to move
/// the view.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphToolbar};
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let minimap = RwSignal::new(true);
/// view! {
///     <div data-testid="graph-minimap-preview">
///         <Graph
///             input=input
///             nodes=nodes
///             minimap=minimap
///             height="420px"
///             label="Billing revamp task graph"
///             data_testid="graph-minimap-preview-graph"
///             toolbar=std::sync::Arc::new(|| view! {
///                 <GraphToolbar data_testid="graph-minimap-preview-toolbar" />
///             }.into_any())
///         />
///     </div>
/// }
/// ```
///
/// ## Minimap beside the graph
/// The host keeps the [`GraphHandle`] from `on_handle` and draws the
/// overview above the graph instead of in its corner.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphHandle, GraphMinimap};
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Button, Flex, FlexGap};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let handle = RwSignal::new(None::<GraphHandle>);
/// view! {
///     <div data-testid="graph-minimap-side">
///         <Flex vertical=true gap=FlexGap::Small>
///             <Flex gap=FlexGap::Small>
///                 <GraphMinimap handle=handle data_testid="graph-minimap-side-minimap" />
///                 <span data-testid="graph-minimap-side-zoom">
///                     <Button on_click=Callback::new(move |_| {
///                         if let Some(h) = handle.get_untracked() {
///                             h.zoom_to.run(2.0);
///                         }
///                     })>"Zoom to 200%"</Button>
///                 </span>
///             </Flex>
///             <div style="height: 360px;">
///                 <Graph
///                     input=input
///                     nodes=nodes
///                     on_handle=Callback::new(move |h| handle.set(Some(h)))
///                     label="Billing revamp task graph"
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "graph-minimap",
    preview_label = "Graph Minimap",
    preview_icon = icondata::AiPictureOutlined,
)]
#[component]
pub fn GraphMinimap(
    /// The graph to show. Defaults to the [`GraphHandle`] of the
    /// [`Graph`](crate::Graph) this minimap is drawn inside; pass a signal
    /// filled from `on_handle` to place it elsewhere. Draws nothing until a
    /// handle is known.
    #[prop(optional, into)]
    handle: MaybeProp<GraphHandle>,
    /// Extra CSS class names on the minimap's root `div`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// `data-testid` on the root `div`; the view rectangle gets `{data_testid}-view`.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-graph", GRAPH_STYLES);
    let from_context = use_context::<GraphHandle>();
    move || {
        handle
            .get()
            .or(from_context)
            .map(|handle| minimap_view(handle, class, data_testid))
    }
}

fn minimap_view(
    handle: GraphHandle,
    class: MaybeProp<String>,
    data_testid: MaybeProp<String>,
) -> impl IntoView {
    let canvas = handle.canvas;
    let shapes = Memo::new(move |_| {
        canvas.layout.with(|g| -> Vec<Rect> {
            if g.groups.is_empty() {
                g.nodes
                    .iter()
                    .map(|n| (n.x, n.y, n.width, n.height))
                    .collect()
            } else {
                g.groups
                    .iter()
                    .map(|b| (b.x, b.y, b.width, b.height))
                    .collect()
            }
        })
    });
    let bounds = Memo::new(move |_| canvas.layout.with(|g| (g.width, g.height)));
    let hidden = Memo::new(move |_| {
        let vp = canvas.viewport.get();
        let (cw, ch) = canvas.size.get();
        canvas
            .layout
            .with(|g| g.nodes.is_empty() || vp.fits_in_view(g.width, g.height, cw, ch))
    });
    let view_rect = Memo::new(move |_| {
        let (cw, ch) = canvas.size.get();
        canvas.viewport.get().visible_rect(cw, ch)
    });

    let dragging = StoredValue::new(None::<i32>);
    let navigate = move |ev: &PointerEvent| {
        let Some(el) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let r = el.get_bounding_client_rect();
        let (lw, lh) = bounds.get_untracked();
        let (lx, ly) = minimap_point(
            f64::from(ev.client_x()) - r.left(),
            f64::from(ev.client_y()) - r.top(),
            r.width(),
            r.height(),
            lw,
            lh,
        );
        let (cw, ch) = canvas.size.get_untracked();
        let next = canvas.viewport.get_untracked().centre_on(lx, ly, cw, ch);
        canvas.set_by_user(next);
    };
    let on_pointerdown = move |ev: PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        ev.prevent_default();
        if let Some(target) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }
        dragging.set_value(Some(ev.pointer_id()));
        navigate(&ev);
    };
    let on_pointermove = move |ev: PointerEvent| {
        if dragging.get_value() == Some(ev.pointer_id()) {
            navigate(&ev);
        }
    };
    let end = move |ev: PointerEvent| {
        if dragging.get_value() == Some(ev.pointer_id()) {
            dragging.set_value(None);
        }
    };

    let view_box = move || {
        let (w, h) = bounds.get();
        format!(
            "{} {} {} {}",
            -FIT_PADDING,
            -FIT_PADDING,
            w + 2.0 * FIT_PADDING,
            h + 2.0 * FIT_PADDING
        )
    };
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-graph-minimap {extra}"),
        _ => "orbital-graph-minimap".to_string(),
    };
    let view_testid = move || data_testid.get().map(|id| format!("{id}-view"));

    move || {
        (!hidden.get()).then(|| {
            view! {
                <div
                    class=root_class
                    aria-hidden="true"
                    data-testid=move || data_testid.get()
                    on:pointerdown=on_pointerdown
                    on:pointermove=on_pointermove
                    on:pointerup=end
                    on:pointercancel=end
                >
                    <svg viewBox=view_box preserveAspectRatio="xMidYMid meet" focusable="false">
                        {move || {
                            shapes
                                .get()
                                .into_iter()
                                .map(|(x, y, w, h)| {
                                    view! {
                                        <rect
                                            class="orbital-graph-minimap__shape"
                                            x=x
                                            y=y
                                            width=w
                                            height=h
                                            rx="6"
                                        />
                                    }
                                })
                                .collect_view()
                        }}
                        <rect
                            class="orbital-graph-minimap__view"
                            data-testid=view_testid
                            x=move || view_rect.get().0
                            y=move || view_rect.get().1
                            width=move || view_rect.get().2.max(0.0)
                            height=move || view_rect.get().3.max(0.0)
                            vector-effect="non-scaling-stroke"
                        />
                    </svg>
                </div>
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimap_point_maps_corners_and_centre_happy() {
        // A 1000 × 500 layout padded by 24 is 1048 × 548; in a 180 × 120
        // minimap the width decides the scale and it is centred vertically.
        let (lw, lh) = (1000.0, 500.0);
        let (cx, cy) = minimap_point(90.0, 60.0, 180.0, 120.0, lw, lh);
        assert!((cx - 500.0).abs() < 1e-9);
        assert!((cy - 250.0).abs() < 1e-9);
        let (x0, _) = minimap_point(0.0, 60.0, 180.0, 120.0, lw, lh);
        assert!((x0 + FIT_PADDING).abs() < 1e-9);
        let (x1, _) = minimap_point(180.0, 60.0, 180.0, 120.0, lw, lh);
        assert!((x1 - (lw + FIT_PADDING)).abs() < 1e-9);
    }

    #[test]
    fn minimap_point_unmeasured_minimap_sad() {
        assert_eq!(
            minimap_point(5.0, 5.0, 0.0, 0.0, 200.0, 100.0),
            (100.0, 50.0)
        );
    }
}
