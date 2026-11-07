//! The `Graph` component: draws a [`GraphLayout`] as SVG with selection,
//! chain mode, pan, zoom and keyboard navigation.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

use leptos::ev::{FocusEvent, KeyboardEvent, PointerEvent, WheelEvent};
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use super::keyboard::{next_focus, NavKey};
use super::model::{
    chain_closure, filter_hidden, line_baselines, neighbourhood, views_by_id, Highlight,
};
use super::styles::GRAPH_STYLES;
use super::viewport::{fit, Viewport, ZOOM_STEP};
use crate::{
    layout, EdgeKind, EdgeRoute, GraphInput, GraphLayout, GraphNodeView, GroupBox, LayoutOptions,
    NodePlacement, Point,
};

/// Lets a host drive the graph's viewport, such as a toolbar's Fit button.
///
/// [`Graph`] hands one over through `on_handle` after it mounts.
#[derive(Debug, Clone, Copy)]
pub struct GraphHandle {
    /// Scales the layout to fit the container (never above 100%) and centres it.
    pub fit: Callback<()>,
    /// Zooms to the given scale (clamped to 0.2–2.2) around the container's centre.
    pub zoom_to: Callback<f64>,
}

/// A pointer drag of less than this many px counts as a click.
const CLICK_SLOP_PX: f64 = 4.0;

/// Per-page counter for marker id prefixes; read only in the browser, after mount.
static INSTANCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy)]
struct Drag {
    pointer_id: i32,
    start_x: f64,
    start_y: f64,
    origin: Viewport,
    moved: bool,
}

/// Shared reactive state each node reads.
#[derive(Clone, Copy)]
struct NodeCtx {
    selected: RwSignal<Option<String>>,
    highlight: Memo<Option<Highlight>>,
    tab_stop: Memo<Option<String>>,
    focused: RwSignal<Option<String>>,
    select: Callback<Option<String>>,
    reveal: Callback<String>,
}

/// Dependency graph drawn as SVG: rounded boxes in the layered layout from
/// [`layout()`](crate::layout()), group boxes, dashed stubs for nodes from
/// elsewhere and right-angled lines between them.
///
/// Click a node, or focus it and press Enter or Space, to select it; the rest
/// dims. Drag the background to pan, scroll to zoom, and press `0` to fit.
///
/// # When to use
///
/// - Tasks and what blocks them, laid out left to right by how long each waits.
/// - Any small-to-medium dependency graph (up to about 1,500 nodes) that needs
///   an accessible name per node and server rendering.
///
/// # Usage
///
/// 1. Build the shape as a [`GraphInput`] and pass it as `input`.
/// 2. Pass one [`GraphNodeView`] per node id as `nodes`: its fill, one to three
///    text lines and, for legend hiding, its `set`.
/// 3. Give the graph an accessible `label` and a height through its container.
/// 4. Own `selected` when the host shows details for the selected node; pass
///    `chain` and `hidden_sets` for the toolbar and [`GraphLegend`](crate::GraphLegend).
/// 5. Keep the [`GraphHandle`] from `on_handle` to wire a Fit button.
///
/// The layout reruns only when `input` or the hidden sets change, so
/// recolouring or relabelling through `nodes` keeps every box in place.
///
/// Keyboard: the graph is one tab stop. Arrow Left and Right move to the
/// nearest node in the previous or next column, Up and Down within the
/// column, Enter or Space selects, Escape clears, `+` and `-` zoom and `0`
/// fits.
///
/// # Best Practices
///
/// ## Do's
///
/// - Put status in a node's lines or `aria_label` as well as its fill; colour
///   is never the only signal.
/// - Give the container a height (`height: 480px`, or a flex child that
///   grows); the graph fills it with a 320px minimum.
/// - Keep the first line short: it is cut with `…` at about 21 characters.
///
/// ## Don'ts
///
/// - Don't rebuild `input` when only colours or labels change; update `nodes`.
/// - Don't put hex colours in a host's colour rules; choose a [`GraphFill`](crate::GraphFill).
///
/// # Examples
///
/// ## Project with external tasks
/// Nine tasks in a project box, two tasks from other projects as dashed stubs
/// with orange lines, and one dependency cycle drawn as a red dashed line.
/// <!-- preview -->
/// ```rust
/// use crate::Graph;
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// view! {
///     <div data-testid="graph-project" style="height: 460px;">
///         <Graph input=input nodes=nodes label="Billing revamp task graph" />
///     </div>
/// }
/// ```
///
/// ## Selection and chain
/// Selecting a task keeps its direct blockers and dependants bright. With the
/// chain switch on, every task before and after it on the chain stays bright
/// instead. The host owns `selected` and shows it below the graph.
/// <!-- preview -->
/// ```rust
/// use crate::Graph;
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Flex, FlexGap, Switch};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let selected = RwSignal::new(None::<String>);
/// let chain = RwSignal::new(false);
/// view! {
///     <div data-testid="graph-selection">
///         <Flex vertical=true gap=FlexGap::Small>
///             <div data-testid="graph-selection-chain">
///                 <Switch bind=chain label="Only this task's chain" />
///             </div>
///             <div style="height: 460px;">
///                 <Graph
///                     input=input
///                     nodes=nodes
///                     selected=selected
///                     chain=chain
///                     label="Billing revamp task graph"
///                 />
///             </div>
///             <span data-testid="graph-selection-selected">
///                 {move || selected.get().unwrap_or_else(|| "none".to_string())}
///             </span>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Legend hides a set
/// A [`GraphLegend`](crate::GraphLegend) and the graph share one `hidden_sets`
/// signal. Turning a chip off removes its tasks and their lines, and the
/// layout closes the gap.
/// <!-- preview -->
/// ```rust
/// use std::collections::HashSet;
/// use crate::{Graph, GraphLegend, GraphLegendLine};
/// use crate::preview::fixtures::{
///     sample_legend_items, sample_project_input, sample_project_views,
/// };
/// use orbital_core_components::{Flex, FlexGap};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let items = RwSignal::new(sample_legend_items());
/// let hidden_sets = RwSignal::new(HashSet::<String>::new());
/// view! {
///     <div data-testid="graph-legend-toggle">
///         <Flex vertical=true gap=FlexGap::Small>
///             <GraphLegend
///                 items=items
///                 hidden_sets=hidden_sets
///                 lines=vec![GraphLegendLine::new("blocks").with_symbol("─▶")]
///                 data_testid="graph-legend-toggle-legend"
///             />
///             <div style="height: 460px;">
///                 <Graph
///                     input=input
///                     nodes=nodes
///                     hidden_sets=hidden_sets
///                     label="Billing revamp task graph"
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Parents and children
/// No group box: child tasks point at their parent with dashed lines ending
/// in a bar, next to a solid "waits on" line.
/// <!-- preview -->
/// ```rust
/// use crate::Graph;
/// use crate::preview::fixtures::{sample_parent_input, sample_parent_views};
/// let input = RwSignal::new(sample_parent_input());
/// let nodes = RwSignal::new(sample_parent_views());
/// view! {
///     <div data-testid="graph-parents" style="height: 360px;">
///         <Graph input=input nodes=nodes label="Billing revamp parent tasks" />
///     </div>
/// }
/// ```
///
/// ## Fit and zoom
/// `on_handle` hands the host a [`GraphHandle`]; these buttons call its `fit`
/// and `zoom_to`. The graph also fits itself once after it mounts.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphHandle};
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Button, Flex, FlexGap};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let handle = RwSignal::new(None::<GraphHandle>);
/// view! {
///     <div data-testid="graph-fit">
///         <Flex vertical=true gap=FlexGap::Small>
///             <Flex gap=FlexGap::Small>
///                 <span data-testid="graph-fit-button">
///                     <Button on_click=Callback::new(move |_| {
///                         if let Some(h) = handle.get_untracked() {
///                             h.fit.run(());
///                         }
///                     })>"Fit"</Button>
///                 </span>
///                 <span data-testid="graph-fit-zoom-in">
///                     <Button on_click=Callback::new(move |_| {
///                         if let Some(h) = handle.get_untracked() {
///                             h.zoom_to.run(2.0);
///                         }
///                     })>"Zoom to 200%"</Button>
///                 </span>
///             </Flex>
///             <div style="height: 420px;">
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
///
/// ## Theme token
/// One node per [`GraphFill`](crate::GraphFill). Fills map to theme tokens
/// only, so switching the catalog to dark mode recolours every box and line.
/// <!-- preview -->
/// ```rust
/// use crate::Graph;
/// use crate::preview::fixtures::{sample_fill_input, sample_fill_views};
/// let input = RwSignal::new(sample_fill_input());
/// let nodes = RwSignal::new(sample_fill_views());
/// view! {
///     <div data-testid="graph-theme" style="height: 320px;">
///         <Graph input=input nodes=nodes label="Graph fills" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "graph",
    preview_label = "Graph",
    preview_icon = icondata::AiApartmentOutlined,
)]
#[component]
pub fn Graph(
    /// Shape of the graph: nodes, edges and groups. The layout is computed from it.
    #[prop(into)]
    input: Signal<GraphInput>,
    /// Spacing and the node cap for the layout; defaults to the standard task grid.
    #[prop(optional)]
    options: LayoutOptions,
    /// Fill, text lines, accessible name and legend set per node id. Nodes without a view show their id.
    #[prop(into)]
    nodes: Signal<Vec<GraphNodeView>>,
    /// Selected node id (controlled). Clicks, Enter / Space and background clicks write it.
    #[prop(optional, into)]
    selected: RwSignal<Option<String>>,
    /// Fires with the new selection on click, Enter / Space, Escape and background click (`None`).
    #[prop(optional)]
    on_select: Option<Callback<Option<String>>>,
    /// When true, a selection keeps its whole chain of predecessors and successors bright, not just direct neighbours.
    #[prop(optional, into)]
    chain: Signal<bool>,
    /// Legend sets to hide: their nodes and lines are left out and the layout closes the gap.
    #[prop(optional, into)]
    hidden_sets: Signal<HashSet<String>>,
    /// Accessible name of the graph, such as "Billing revamp task graph".
    #[prop(into)]
    label: String,
    /// Extra CSS class names on the root `div`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// `data-testid` on the root `div`.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// Receives a [`GraphHandle`] after mount, for a host's Fit and zoom controls.
    #[prop(optional)]
    on_handle: Option<Callback<GraphHandle>>,
) -> impl IntoView {
    inject_style("orbital-graph", GRAPH_STYLES);

    let filtered = Memo::new(move |_| {
        input.with(|input| {
            nodes.with(|views| hidden_sets.with(|hidden| filter_hidden(input, views, hidden)))
        })
    });
    let graph = Memo::new(move |_| filtered.with(|input| layout(input, &options)));
    let views = Memo::new(move |_| nodes.with(|views| views_by_id(views)));
    let highlight = Memo::new(move |_| {
        let id = selected.get()?;
        filtered.with(|input| {
            input.nodes.iter().any(|n| n.id == id).then(|| {
                if chain.get() {
                    chain_closure(input, &id)
                } else {
                    neighbourhood(input, &id)
                }
            })
        })
    });

    let viewport = RwSignal::new(Viewport::default());
    let focused = RwSignal::new(None::<String>);
    let panning = RwSignal::new(false);
    let drag = StoredValue::new(None::<Drag>);
    let prefix = RwSignal::new(None::<String>);
    let container = NodeRef::<leptos::html::Div>::new();

    // Marker ids are made after mount so server and client markup agree.
    Effect::new(move |_| {
        let n = INSTANCE.fetch_add(1, Ordering::Relaxed);
        prefix.set(Some(format!("orbital-graph-{n}")));
    });

    let container_size = move || {
        container
            .get_untracked()
            .map(|el| (f64::from(el.client_width()), f64::from(el.client_height())))
            .unwrap_or((0.0, 0.0))
    };
    let do_fit = move || {
        let (cw, ch) = container_size();
        let next = graph.with_untracked(|g| fit(g.width, g.height, cw, ch));
        viewport.set(next);
    };
    let zoom_centre = move |scale: f64| {
        let (cw, ch) = container_size();
        viewport.update(|vp| *vp = vp.zoom_at(cw / 2.0, ch / 2.0, scale));
    };

    let select = Callback::new(move |id: Option<String>| {
        if selected.get_untracked() != id {
            selected.set(id.clone());
        }
        if let Some(on_select) = on_select {
            on_select.run(id);
        }
    });

    let reveal = Callback::new(move |id: String| {
        let rect = graph.with_untracked(|g| {
            g.nodes
                .iter()
                .find(|n| n.id == id)
                .map(|n| (n.x, n.y, n.width, n.height))
        });
        if let Some(rect) = rect {
            let (cw, ch) = container_size();
            viewport.update(|vp| *vp = vp.reveal(rect, cw, ch));
        }
    });

    let focus_node = move |id: String| {
        focused.set(Some(id.clone()));
        let Some(root) = container.get_untracked() else {
            return;
        };
        if let Ok(Some(el)) = root.query_selector(&node_selector(&id)) {
            if let Ok(el) = el.dyn_into::<web_sys::SvgElement>() {
                let _ = el.focus();
            }
        }
        reveal.run(id);
    };

    // Fit once the first non-empty layout is on screen.
    let fitted = StoredValue::new(false);
    Effect::new(move |_| {
        let empty = graph.with(|g| g.nodes.is_empty());
        if !empty && !fitted.get_value() {
            fitted.set_value(true);
            do_fit();
        }
    });

    if let Some(on_handle) = on_handle {
        let handle = GraphHandle {
            fit: Callback::new(move |()| do_fit()),
            zoom_to: Callback::new(zoom_centre),
        };
        Effect::new(move |_| on_handle.run(handle));
    }

    // A selected node whose set gets hidden clears the selection.
    Effect::new(move |_| {
        let Some(id) = selected.get() else {
            return;
        };
        let in_input = input.with(|i| i.nodes.iter().any(|n| n.id == id));
        let shown = filtered.with(|i| i.nodes.iter().any(|n| n.id == id));
        if in_input && !shown {
            select.run(None);
        }
    });

    let tab_stop = Memo::new(move |_| {
        graph.with(|g| {
            let placed = |id: &String| g.nodes.iter().any(|n| &n.id == id);
            focused
                .get()
                .filter(placed)
                .or_else(|| selected.get().filter(placed))
                .or_else(|| g.nodes.first().map(|n| n.id.clone()))
        })
    });

    let ctx = NodeCtx {
        selected,
        highlight,
        tab_stop,
        focused,
        select,
        reveal,
    };

    let on_wheel = move |ev: WheelEvent| {
        ev.prevent_default();
        let Some(root) = container.get_untracked() else {
            return;
        };
        let rect = root.get_bounding_client_rect();
        let px = f64::from(ev.client_x()) - rect.left();
        let py = f64::from(ev.client_y()) - rect.top();
        let factor = if ev.delta_y() < 0.0 {
            ZOOM_STEP
        } else {
            1.0 / ZOOM_STEP
        };
        viewport.update(|vp| *vp = vp.zoom_at(px, py, vp.scale * factor));
    };

    let on_pointerdown = move |ev: PointerEvent| {
        if ev.button() != 0 {
            return;
        }
        if let Some(target) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }
        drag.set_value(Some(Drag {
            pointer_id: ev.pointer_id(),
            start_x: f64::from(ev.client_x()),
            start_y: f64::from(ev.client_y()),
            origin: viewport.get_untracked(),
            moved: false,
        }));
    };

    let on_pointermove = move |ev: PointerEvent| {
        let Some(mut d) = drag.get_value() else {
            return;
        };
        if d.pointer_id != ev.pointer_id() {
            return;
        }
        let dx = f64::from(ev.client_x()) - d.start_x;
        let dy = f64::from(ev.client_y()) - d.start_y;
        if !d.moved && dx.hypot(dy) >= CLICK_SLOP_PX {
            d.moved = true;
            panning.set(true);
        }
        if d.moved {
            viewport.set(d.origin.pan_by(dx, dy));
        }
        drag.set_value(Some(d));
    };

    let end_drag = move |ev: &PointerEvent| -> Option<Drag> {
        let d = drag
            .get_value()
            .filter(|d| d.pointer_id == ev.pointer_id())?;
        drag.set_value(None);
        panning.set(false);
        if let Some(target) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = target.release_pointer_capture(ev.pointer_id());
        }
        Some(d)
    };

    let on_pointerup = move |ev: PointerEvent| {
        if let Some(d) = end_drag(&ev) {
            if !d.moved {
                select.run(None);
            }
        }
    };
    let on_pointercancel = move |ev: PointerEvent| {
        let _ = end_drag(&ev);
    };

    let on_keydown = move |ev: KeyboardEvent| {
        let key = ev.key();
        let current = focused.get_untracked();
        if let Some(dir) = NavKey::from_key(&key) {
            if let Some(current) = current {
                ev.prevent_default();
                let next = graph.with_untracked(|g| next_focus(&g.nodes, &current, dir));
                focus_node(next);
            }
            return;
        }
        match key.as_str() {
            "Enter" | " " => {
                if let Some(current) = current {
                    ev.prevent_default();
                    select.run(Some(current));
                }
            }
            "Escape" => {
                if selected.get_untracked().is_some() {
                    ev.prevent_default();
                    select.run(None);
                }
            }
            "+" | "=" => {
                ev.prevent_default();
                zoom_centre(viewport.get_untracked().scale * ZOOM_STEP);
            }
            "-" | "_" => {
                ev.prevent_default();
                zoom_centre(viewport.get_untracked().scale / ZOOM_STEP);
            }
            "0" => {
                ev.prevent_default();
                do_fit();
            }
            _ => {}
        }
    };

    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-graph {extra}"),
        _ => "orbital-graph".to_string(),
    };
    let svg_class = move || {
        if panning.get() {
            "orbital-graph__svg orbital-graph__svg--panning"
        } else {
            "orbital-graph__svg"
        }
    };

    view! {
        <div class=root_class data-testid=move || data_testid.get() node_ref=container>
            <svg
                class=svg_class
                role="group"
                aria-label=label
                on:wheel=on_wheel
                on:pointerdown=on_pointerdown
                on:pointermove=on_pointermove
                on:pointerup=on_pointerup
                on:pointercancel=on_pointercancel
                on:keydown=on_keydown
            >
                <defs>{move || prefix.get().map(markers)}</defs>
                <g
                    class="orbital-graph__viewport"
                    data-scale=move || viewport.get().scale.to_string()
                    style=move || format!("transform: {};", viewport.get().transform())
                >
                    {move || graph.with(|g| g.groups.iter().map(group_view).collect_view())}
                    {move || graph.with(|g| edges_view(g, highlight, prefix))}
                    {move || {
                        let views = views.get();
                        graph.with(|g| {
                            g.nodes
                                .iter()
                                .map(|p| node_view(p, views.get(&p.id), ctx))
                                .collect_view()
                        })
                    }}
                </g>
            </svg>
        </div>
    }
}

/// CSS selector for a node's `<g>`, with the id quoted safely.
fn node_selector(id: &str) -> String {
    let escaped = id.replace('\\', "\\\\").replace('"', "\\\"");
    format!("[data-node-id=\"{escaped}\"]")
}

fn markers(prefix: String) -> impl IntoView {
    let arrow = |name: &str, class: &'static str| {
        view! {
            <marker
                id=format!("{prefix}-{name}")
                viewBox="0 0 10 10"
                refX="9"
                refY="5"
                markerWidth="5"
                markerHeight="5"
                orient="auto-start-reverse"
            >
                <path class=class d="M0 0 L10 5 L0 10 z" />
            </marker>
        }
    };
    let tee = |name: &str, class: &'static str| {
        view! {
            <marker
                id=format!("{prefix}-{name}")
                viewBox="0 0 10 10"
                refX="9"
                refY="5"
                markerWidth="5"
                markerHeight="5"
                orient="auto"
            >
                <path class=class d="M7 0 H10 V10 H7 z" />
            </marker>
        }
    };
    view! {
        {arrow("arrow", "orbital-graph__marker")}
        {arrow("arrow-stub", "orbital-graph__marker orbital-graph__marker--stub")}
        {tee("tee", "orbital-graph__marker")}
        {tee("tee-stub", "orbital-graph__marker orbital-graph__marker--stub")}
    }
}

fn group_view(group: &GroupBox) -> impl IntoView {
    view! {
        <g class="orbital-graph__group" data-group-id=group.id.clone()>
            <rect x=group.x y=group.y width=group.width height=group.height rx="10" />
            <text x=group.x + 12.0 y=group.y + 16.0>
                {group.label.clone()}
            </text>
        </g>
    }
}

fn path_d(points: &[Point]) -> String {
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        let cmd = if i == 0 { 'M' } else { 'L' };
        if !d.is_empty() {
            d.push(' ');
        }
        d.push_str(&format!("{cmd}{} {}", p.x, p.y));
    }
    d
}

fn edges_view(
    graph: &GraphLayout,
    highlight: Memo<Option<Highlight>>,
    prefix: RwSignal<Option<String>>,
) -> impl IntoView {
    let stubs: HashSet<&str> = graph
        .nodes
        .iter()
        .filter(|n| n.stub)
        .map(|n| n.id.as_str())
        .collect();
    let placed: HashMap<&str, &NodePlacement> =
        graph.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let dim_class = move |base: String, from: String, to: String| {
        move || {
            let dimmed = highlight.with(|h| h.as_ref().is_some_and(|h| !h.has_edge(&from, &to)));
            if dimmed {
                format!("{base} orbital-graph--dimmed")
            } else {
                base.clone()
            }
        }
    };

    let routed = graph
        .edges
        .iter()
        .map(|route: &EdgeRoute| {
            let stub = stubs.contains(route.from.as_str()) || stubs.contains(route.to.as_str());
            let mut base = "orbital-graph__edge".to_string();
            if route.kind == EdgeKind::Child {
                base.push_str(" orbital-graph__edge--child");
            }
            if stub {
                base.push_str(" orbital-graph__edge--stub");
            }
            let marker = match (route.kind, stub) {
                (EdgeKind::Depends, false) => "arrow",
                (EdgeKind::Depends, true) => "arrow-stub",
                (EdgeKind::Child, false) => "tee",
                (EdgeKind::Child, true) => "tee-stub",
            };
            let marker_end = move || prefix.get().map(|p| format!("url(#{p}-{marker})"));
            view! {
                <path
                    class=dim_class(base, route.from.clone(), route.to.clone())
                    data-edge-from=route.from.clone()
                    data-edge-to=route.to.clone()
                    d=path_d(&route.points)
                    marker-end=marker_end
                />
            }
        })
        .collect_view();

    // Edges the layout removed to break a cycle get a straight dashed line,
    // so nothing disappears silently.
    let broken = graph
        .broken_cycles
        .iter()
        .filter_map(|(from, to)| {
            let a = placed.get(from.as_str())?;
            let b = placed.get(to.as_str())?;
            let points = [
                Point {
                    x: a.x + a.width,
                    y: a.y + a.height / 2.0,
                },
                Point {
                    x: b.x,
                    y: b.y + b.height / 2.0,
                },
            ];
            let base = "orbital-graph__edge orbital-graph__edge--broken".to_string();
            Some(view! {
                <path
                    class=dim_class(base, from.clone(), to.clone())
                    data-edge-from=from.clone()
                    data-edge-to=to.clone()
                    data-broken-cycle="true"
                    d=path_d(&points)
                />
            })
        })
        .collect_view();

    view! {
        {routed}
        {broken}
    }
}

fn node_view(
    placement: &NodePlacement,
    view: Option<&GraphNodeView>,
    ctx: NodeCtx,
) -> impl IntoView {
    let id = placement.id.clone();
    let fallback;
    let view = match view {
        Some(view) => view,
        None => {
            fallback = GraphNodeView {
                id: id.clone(),
                lines: vec![id.clone()],
                ..GraphNodeView::default()
            };
            &fallback
        }
    };
    let lines = view.display_lines(placement.width);
    let baselines = line_baselines(lines.len(), placement.height);
    let base = if placement.stub {
        "orbital-graph__node orbital-graph__stub"
    } else {
        "orbital-graph__node"
    };

    let is_selected = {
        let id = id.clone();
        move || ctx.selected.with(|s| s.as_deref() == Some(id.as_str()))
    };
    let class = {
        let id = id.clone();
        let is_selected = is_selected.clone();
        move || {
            let mut class = base.to_string();
            if is_selected() {
                class.push_str(" orbital-graph__node--selected");
            }
            let dimmed = ctx
                .highlight
                .with(|h| h.as_ref().is_some_and(|h| !h.nodes.contains(&id)));
            if dimmed {
                class.push_str(" orbital-graph--dimmed");
            }
            class
        }
    };
    let tabindex = {
        let id = id.clone();
        move || {
            if ctx.tab_stop.with(|t| t.as_deref() == Some(id.as_str())) {
                "0"
            } else {
                "-1"
            }
        }
    };
    let pressed = move || if is_selected() { "true" } else { "false" };
    let on_click = {
        let id = id.clone();
        move |_| ctx.select.run(Some(id.clone()))
    };
    let on_focus = {
        let id = id.clone();
        move |_: FocusEvent| {
            ctx.focused.set(Some(id.clone()));
            ctx.reveal.run(id.clone());
        }
    };
    let (w, h) = (placement.width, placement.height);
    let text = lines
        .into_iter()
        .zip(baselines)
        .enumerate()
        .map(|(i, (line, y))| {
            view! {
                <text class=(i == 0).then_some("orbital-graph__title") x="12" y=y>
                    {line}
                </text>
            }
        })
        .collect_view();

    view! {
        <g
            class=class
            role="button"
            tabindex=tabindex
            aria-label=view.accessible_name()
            aria-pressed=pressed
            data-node-id=id
            transform=format!("translate({} {})", placement.x, placement.y)
            style=view.fill.style_vars()
            on:pointerdown=|ev: PointerEvent| ev.stop_propagation()
            on:click=on_click
            on:focus=on_focus
        >
            <rect class="orbital-graph__focus" x="-4" y="-4" width=w + 8.0 height=h + 8.0 rx="11" />
            <rect width=w height=h rx="8" />
            {text}
        </g>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::preview::fixtures::{sample_project_input, sample_project_views};

    fn render(hidden: HashSet<String>) -> (String, GraphLayout) {
        let owner = Owner::new();
        owner.with(|| {
            let input = sample_project_input();
            let views = sample_project_views();
            let expected = layout(
                &filter_hidden(&input, &views, &hidden),
                &LayoutOptions::default(),
            );
            let html = view! {
                <Graph
                    input=Signal::stored(input)
                    nodes=Signal::stored(views)
                    hidden_sets=Signal::stored(hidden)
                    label="Billing revamp task graph"
                />
            }
            .to_html();
            (html, expected)
        })
    }

    #[test]
    fn graph_ssr_renders_nodes_and_edges_happy() {
        let (html, layout) = render(HashSet::new());
        assert_eq!(html.matches("data-node-id=").count(), layout.nodes.len());
        assert_eq!(layout.nodes.len(), 11);
        assert_eq!(
            html.matches("<path data-edge-from=").count(),
            layout.edges.len() + layout.broken_cycles.len(),
            "{html}"
        );
        assert_eq!(layout.broken_cycles.len(), 1);
        assert!(html.contains("data-broken-cycle=\"true\""));
        assert!(html.contains("Billing revamp</text>"), "{html}");
        assert!(html.contains("orbital-graph__node orbital-graph__stub"));
        assert!(html.contains("orbital-graph__edge--stub"));
        assert!(html.contains("aria-label=\"Billing revamp task graph\""));
        assert!(html.contains("role=\"group\""));
        assert!(!html.contains("NaN"), "{html}");
        // One tab stop: exactly one node is reachable with Tab.
        assert_eq!(html.matches("tabindex=\"0\"").count(), 1);
        // The server renders the identity viewport; the client fits after mount.
        assert!(html.contains("transform: translate(0px, 0px) scale(1);"));
    }

    #[test]
    fn graph_ssr_hidden_set_drops_its_nodes_happy() {
        let (html, layout) = render(HashSet::from(["external".to_string()]));
        assert_eq!(html.matches("data-node-id=").count(), 9);
        assert_eq!(layout.nodes.len(), 9);
        assert!(!html.contains("orbital-graph__stub"));
        assert!(!html.contains("orbital-graph__edge--stub"));
    }
}
