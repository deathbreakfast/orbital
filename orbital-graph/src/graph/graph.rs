//! The `Graph` component: draws a [`GraphLayout`] as SVG with selection,
//! chain mode, pan, zoom, a background grid, toolbar and minimap overlays,
//! auto-fit and keyboard navigation.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use leptos::ev::{FocusEvent, KeyboardEvent, MouseEvent, PointerEvent, WheelEvent};
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use super::keyboard::{next_focus, NavKey};
use super::minimap::GraphMinimap;
use super::model::{
    chain_closure, filter_hidden, line_baselines, neighbourhood, views_by_id, Highlight,
};
use super::styles::GRAPH_STYLES;
use super::viewport::{fit, Viewport, DBLCLICK_ZOOM, ZOOM_STEP};
use crate::{
    layout, EdgeKind, EdgeRoute, GraphInput, GraphLayout, GraphNodeView, GroupBox, LayoutOptions,
    NodePlacement, Point,
};

/// Lets a host drive the graph's viewport, such as a toolbar's Fit button.
///
/// [`Graph`] hands one over through `on_handle` after it mounts, and puts it
/// in context for a [`GraphToolbar`](crate::GraphToolbar) or
/// [`GraphMinimap`](crate::GraphMinimap) drawn inside it. Only [`Graph`]
/// makes one.
#[derive(Debug, Clone, Copy)]
pub struct GraphHandle {
    /// Scales the layout to fit the container (never above 100%), centres it
    /// and turns auto-fit back on.
    pub fit: Callback<()>,
    /// Zooms to the given scale (clamped to 0.2–2.2) around the container's centre.
    pub zoom_to: Callback<f64>,
    /// Zooms in one step (×1.2) around the container's centre.
    pub zoom_in: Callback<()>,
    /// Zooms out one step (÷1.2) around the container's centre.
    pub zoom_out: Callback<()>,
    /// Current zoom; 1 is 100%.
    pub scale: Signal<f64>,
    /// False once the zoom is at its 2.2 maximum.
    pub can_zoom_in: Signal<bool>,
    /// False once the zoom is at its 0.2 minimum (or a fit went below it).
    pub can_zoom_out: Signal<bool>,
    /// Whether the graph draws its [`GraphMinimap`](crate::GraphMinimap)
    /// overlay; the same signal as the graph's `minimap` prop.
    pub minimap: RwSignal<bool>,
    pub(crate) canvas: GraphCanvas,
}

/// The graph's view state a [`GraphMinimap`](crate::GraphMinimap) reads and moves.
#[derive(Clone, Copy)]
pub(crate) struct GraphCanvas {
    /// The drawn layout, cached; reading it never reruns `layout()`.
    pub(crate) layout: Memo<GraphLayout>,
    pub(crate) viewport: RwSignal<Viewport>,
    /// Container size in px, kept current by a `ResizeObserver`.
    pub(crate) size: RwSignal<(f64, f64)>,
    /// True until the user pans or zooms; while true the graph refits itself.
    pub(crate) auto_fit: StoredValue<bool>,
}

impl std::fmt::Debug for GraphCanvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphCanvas").finish_non_exhaustive()
    }
}

impl GraphCanvas {
    /// Moves the view on the user's behalf, which stops auto-fit.
    pub(crate) fn set_by_user(&self, next: Viewport) {
        self.auto_fit.set_value(false);
        self.viewport.set(next);
    }
}

/// What [`Graph`] draws behind the nodes. It pans and zooms with them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphBackground {
    /// 1px dots on an 18px grid (the default).
    #[default]
    Dots,
    /// Thin lines on an 18px grid.
    Lines,
    /// The plain canvas colour.
    None,
}

impl GraphBackground {
    fn as_str(self) -> &'static str {
        match self {
            GraphBackground::Dots => "dots",
            GraphBackground::Lines => "lines",
            GraphBackground::None => "none",
        }
    }
}

/// Wait after the last viewport change before `on_viewport` fires.
const VIEWPORT_DEBOUNCE: Duration = Duration::from_millis(250);

/// A pointer drag of less than this many px counts as a click.
const CLICK_SLOP_PX: f64 = 4.0;

/// `MouseEvent.button` value of the middle button.
const MIDDLE_BUTTON: i16 = 1;

/// What a pointer press on the graph starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragKind {
    /// Left button: pans, and a release under the click slop clears the selection.
    Background,
    /// Middle button: always pans and never selects.
    Middle,
}

/// Maps a pressed button to the drag it starts; other buttons start none.
fn drag_kind(button: i16) -> Option<DragKind> {
    match button {
        0 => Some(DragKind::Background),
        MIDDLE_BUTTON => Some(DragKind::Middle),
        _ => None,
    }
}

/// A node keeps a press to itself (so it never starts a pan) unless it's the
/// middle button, which pans from anywhere.
fn node_swallows_pointerdown(button: i16) -> bool {
    button != MIDDLE_BUTTON
}

/// Per-page counter for marker id prefixes; read only in the browser, after mount.
static INSTANCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy)]
struct Drag {
    pointer_id: i32,
    kind: DragKind,
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
    open: Option<Callback<String>>,
}

/// Dependency graph drawn as SVG: rounded boxes in the layered layout from
/// [`layout()`](crate::layout()), group boxes, dashed stubs for nodes from
/// elsewhere and right-angled lines between them.
///
/// Click a node, or focus it and press Enter or Space, to select it; the rest
/// dims. Drag the background, or middle-drag anywhere, to pan; scroll to zoom;
/// double-click the background to zoom in ×2 at the pointer (Ctrl or ⌘ +
/// double-click zooms out); press `0` to fit. A dot grid behind the nodes
/// moves and scales with them. The graph refits itself when its nodes or its
/// container size change, until the user pans or zooms.
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
/// 3. Give the graph an accessible `label` and a height: the `height` prop, or a
///    container with a height.
/// 4. Own `selected` when the host shows details for the selected node; pass
///    `chain` and `hidden_sets` for the toolbar and [`GraphLegend`](crate::GraphLegend).
/// 5. Put a [`GraphToolbar`](crate::GraphToolbar) in the `toolbar` slot for
///    zoom, Fit and Minimap buttons plus the host's own actions, or keep the
///    [`GraphHandle`] from `on_handle` to wire controls elsewhere.
/// 6. Set `on_node_open` when a node opens something (a task page or peek);
///    set `on_viewport` to save the view.
///
/// The layout reruns only when `input` or the hidden sets change, so
/// recolouring or relabelling through `nodes` keeps every box in place.
/// Zooming, panning and selecting change signals only; nothing remounts.
///
/// Auto-fit: the graph fits itself after mount and again whenever the drawn
/// set (`input` or `hidden_sets`) or the container size changes. A pan or
/// zoom by the user turns that off so the view stays where they put it; Fit
/// (the button, `0` or [`GraphHandle::fit`]) turns it back on. Zoom changes
/// ease over 150ms, and jump straight there under `prefers-reduced-motion`.
///
/// Keyboard: the graph is one tab stop. Arrow Left and Right move to the
/// nearest node in the previous or next column, Up and Down within the
/// column, Enter or Space selects, Shift+Enter opens the node (when
/// `on_node_open` is set), Escape clears, `+` and `-` zoom and `0` fits. The
/// toolbar's buttons are named for screen readers; the minimap is hidden
/// from them, since Fit and the keyboard reach everything it does.
///
/// # Best Practices
///
/// ## Do's
///
/// - Put status in a node's lines or `aria_label` as well as its fill; colour
///   is never the only signal.
/// - Set `height="480px"`, or give the container a height (or make it a flex
///   child that grows); without either the graph fills its container with a
///   320px minimum.
/// - Keep the first line short: it is cut with `…` at about 21 characters.
/// - Put filters and view options in the toolbar's `leading` / `trailing`
///   slots so every canvas control sits in one row.
///
/// ## Don'ts
///
/// - Don't rebuild `input` when only colours or labels change; update `nodes`.
/// - Don't put hex colours in a host's colour rules; choose a [`GraphFill`](crate::GraphFill).
/// - Don't call `fit` after every data refresh; auto-fit already refits until
///   the user takes over, and a forced fit throws away their view.
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
///     <div data-testid="graph-project">
///         <Graph input=input nodes=nodes height="460px" label="Billing revamp task graph" />
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
/// ## Canvas with toolbar and minimap
/// A [`GraphToolbar`](crate::GraphToolbar) in the `toolbar` slot holds Zoom
/// out, Zoom in, Fit and Minimap, with a host action in its `leading` slot.
/// Double-click the background to zoom in; zoom in and turn the minimap on
/// to move around from the corner.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphToolbar};
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Button, ButtonAppearance};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let minimap = RwSignal::new(false);
/// let clicks = RwSignal::new(0_u32);
/// view! {
///     <div data-testid="graph-canvas">
///         <Graph
///             input=input
///             nodes=nodes
///             minimap=minimap
///             height="460px"
///             label="Billing revamp task graph"
///             data_testid="graph-canvas-graph"
///             toolbar=std::sync::Arc::new(move || view! {
///                 <GraphToolbar
///                     data_testid="graph-canvas-toolbar"
///                     leading=std::sync::Arc::new(move || view! {
///                         <span data-testid="graph-canvas-action">
///                             <Button
///                                 appearance=ButtonAppearance::Subtle
///                                 on_click=Callback::new(move |_| clicks.update(|n| *n += 1))
///                             >
///                                 "Hide done"
///                             </Button>
///                         </span>
///                     }.into_any())
///                 />
///             }.into_any())
///         />
///         <span data-testid="graph-canvas-clicks">{move || clicks.get().to_string()}</span>
///     </div>
/// }
/// ```
///
/// ## Background
/// `background` picks the grid behind the nodes: dots (the default), lines
/// or none.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphBackground};
/// use crate::preview::fixtures::{sample_parent_input, sample_parent_views};
/// let input = RwSignal::new(sample_parent_input());
/// let nodes = RwSignal::new(sample_parent_views());
/// view! {
///     <div data-testid="graph-background" style="height: 300px;">
///         <Graph
///             input=input
///             nodes=nodes
///             background=GraphBackground::Lines
///             label="Billing revamp parent tasks on a line grid"
///         />
///     </div>
/// }
/// ```
///
/// ## Open a node
/// With `on_node_open` set, double-clicking a node (or Shift+Enter on a
/// focused one) opens it instead of zooming. `on_viewport` reports the view
/// a moment after it stops moving, for hosts that save it.
/// <!-- preview -->
/// ```rust
/// use crate::Graph;
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Flex, FlexGap};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let opened = RwSignal::new(None::<String>);
/// let scale = RwSignal::new(None::<f64>);
/// view! {
///     <div data-testid="graph-open">
///         <Flex vertical=true gap=FlexGap::Small>
///             <div style="height: 420px;">
///                 <Graph
///                     input=input
///                     nodes=nodes
///                     on_node_open=Callback::new(move |id| opened.set(Some(id)))
///                     on_viewport=Callback::new(move |vp: crate::viewport::Viewport| {
///                         scale.set(Some(vp.scale))
///                     })
///                     label="Billing revamp task graph"
///                 />
///             </div>
///             <span data-testid="graph-open-opened">
///                 {move || opened.get().unwrap_or_else(|| "none".to_string())}
///             </span>
///             <span data-testid="graph-open-scale">
///                 {move || scale.get().map(|s| format!("{s:.2}")).unwrap_or_default()}
///             </span>
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
    /// Height as a CSS length (`"480px"`, `"60vh"`). Without it the graph fills its
    /// container's height, with a 320px minimum.
    #[prop(optional, into)]
    height: MaybeProp<String>,
    /// Receives a [`GraphHandle`] after mount, for a host's Fit and zoom controls.
    #[prop(optional)]
    on_handle: Option<Callback<GraphHandle>>,
    /// Grid behind the nodes, panning and zooming with them. Defaults to
    /// [`GraphBackground::Dots`]: 1px dots 18px apart in `--orb-graph-grid`.
    #[prop(optional)]
    background: GraphBackground,
    /// Slot overlaid along the top of the canvas, usually a
    /// [`GraphToolbar`](crate::GraphToolbar) (which finds this graph's
    /// [`GraphHandle`] by itself). Empty by default.
    #[prop(optional)]
    toolbar: Option<ChildrenFn>,
    /// Whether the [`GraphMinimap`](crate::GraphMinimap) overlay shows in the
    /// bottom-right corner (off by default). The toolbar's Minimap button
    /// toggles it; own it to remember the choice. It stays hidden while the
    /// whole graph is in view.
    #[prop(optional, into)]
    minimap: RwSignal<bool>,
    /// Fires with a node's id on double-click, or Shift+Enter on the focused
    /// node. When unset, double-clicking a node zooms in like the background.
    #[prop(optional)]
    on_node_open: Option<Callback<String>>,
    /// Fires with the new [`Viewport`] 250ms after the view stops changing,
    /// for hosts that save and restore it.
    #[prop(optional)]
    on_viewport: Option<Callback<Viewport>>,
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

    let size = RwSignal::new((0.0_f64, 0.0_f64));
    let auto_fit = StoredValue::new(true);
    let canvas = GraphCanvas {
        layout: graph,
        viewport,
        size,
        auto_fit,
    };

    let container_size = move || {
        container
            .get_untracked()
            .map(|el| (f64::from(el.client_width()), f64::from(el.client_height())))
            .unwrap_or((0.0, 0.0))
    };
    let do_fit = move || {
        auto_fit.set_value(true);
        let (cw, ch) = container_size();
        let next = graph.with_untracked(|g| fit(g.width, g.height, cw, ch));
        viewport.set(next);
    };
    let zoom_centre = move |scale: f64| {
        let (cw, ch) = container_size();
        let next = viewport.get_untracked().zoom_at(cw / 2.0, ch / 2.0, scale);
        canvas.set_by_user(next);
    };

    // Track the container size so auto-fit and the minimap follow resizes.
    Effect::new(move |_| {
        let Some(el) = container.get() else {
            return;
        };
        observe_size(el, size);
    });

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
            let current = viewport.get_untracked();
            let next = current.reveal(rect, cw, ch);
            if next != current {
                canvas.set_by_user(next);
            }
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

    // Auto-fit: refit when the drawn layout or the container size changes,
    // until the user pans or zooms.
    Effect::new(move |_| {
        let empty = graph.with(|g| g.nodes.is_empty());
        size.track();
        if !empty && auto_fit.get_value() {
            do_fit();
        }
    });

    let scale = Memo::new(move |_| viewport.get().scale);
    let can_zoom_in = Memo::new(move |_| viewport.get().can_zoom_in());
    let can_zoom_out = Memo::new(move |_| viewport.get().can_zoom_out());
    let handle = GraphHandle {
        fit: Callback::new(move |()| do_fit()),
        zoom_to: Callback::new(zoom_centre),
        zoom_in: Callback::new(move |()| zoom_centre(viewport.get_untracked().scale * ZOOM_STEP)),
        zoom_out: Callback::new(move |()| zoom_centre(viewport.get_untracked().scale / ZOOM_STEP)),
        scale: scale.into(),
        can_zoom_in: can_zoom_in.into(),
        can_zoom_out: can_zoom_out.into(),
        minimap,
        canvas,
    };
    provide_context(handle);
    if let Some(on_handle) = on_handle {
        Effect::new(move |_| on_handle.run(handle));
    }

    // Report the view once it settles.
    if let Some(on_viewport) = on_viewport {
        let generation = StoredValue::new(0_u64);
        Effect::new(move |prev: Option<Viewport>| {
            let vp = viewport.get();
            if prev.is_some_and(|prev| prev != vp) {
                let n = generation.get_value() + 1;
                generation.set_value(n);
                set_timeout(
                    move || {
                        if generation.try_get_value() == Some(n) {
                            on_viewport.run(vp);
                        }
                    },
                    VIEWPORT_DEBOUNCE,
                );
            }
            vp
        });
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
        open: on_node_open,
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
        let vp = viewport.get_untracked();
        canvas.set_by_user(vp.zoom_at(px, py, vp.scale * factor));
    };

    // Double-click zooms in ×2 at the pointer; Ctrl or ⌘ zooms out. A node
    // with `on_node_open` set stops the event before it gets here.
    let on_dblclick = move |ev: MouseEvent| {
        ev.prevent_default();
        let Some(root) = container.get_untracked() else {
            return;
        };
        let rect = root.get_bounding_client_rect();
        let px = f64::from(ev.client_x()) - rect.left();
        let py = f64::from(ev.client_y()) - rect.top();
        let vp = viewport.get_untracked();
        let scale = if ev.ctrl_key() || ev.meta_key() {
            vp.scale / DBLCLICK_ZOOM
        } else {
            vp.scale * DBLCLICK_ZOOM
        };
        canvas.set_by_user(vp.zoom_at(px, py, scale));
    };

    let on_pointerdown = move |ev: PointerEvent| {
        let Some(kind) = drag_kind(ev.button()) else {
            return;
        };
        if kind == DragKind::Middle {
            ev.prevent_default();
        }
        if let Some(target) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }
        drag.set_value(Some(Drag {
            pointer_id: ev.pointer_id(),
            kind,
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
            auto_fit.set_value(false);
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
            if d.kind == DragKind::Background && !d.moved {
                select.run(None);
            }
        }
    };
    let on_pointercancel = move |ev: PointerEvent| {
        let _ = end_drag(&ev);
    };

    // Stops the browser's middle-click autoscroll and focus change.
    let on_mousedown = move |ev: MouseEvent| {
        if ev.button() == MIDDLE_BUTTON {
            ev.prevent_default();
        }
    };
    // Stops a middle drag from activating links hosted inside nodes.
    let on_auxclick = move |ev: MouseEvent| {
        if ev.button() == MIDDLE_BUTTON {
            ev.prevent_default();
        }
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
            "Enter" if ev.shift_key() && on_node_open.is_some() => {
                if let (Some(current), Some(open)) = (current, on_node_open) {
                    ev.prevent_default();
                    open.run(current);
                }
            }
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

    let root_class = move || {
        let mut out = "orbital-graph".to_string();
        if panning.get() {
            out.push_str(" orbital-graph--panning");
        }
        if let Some(extra) = class.get().filter(|c| !c.is_empty()) {
            out.push(' ');
            out.push_str(&extra);
        }
        out
    };
    let grid = (background != GraphBackground::None).then(|| {
        let grid_class = format!(
            "orbital-graph__grid orbital-graph__grid--{}",
            background.as_str()
        );
        let grid_style = move || {
            let vp = viewport.get();
            let pitch = round_px(vp.grid_pitch());
            format!(
                "background-size: {pitch}px {pitch}px; background-position: {}px {}px;",
                round_px(vp.x),
                round_px(vp.y)
            )
        };
        view! { <div class=grid_class style=grid_style aria-hidden="true"></div> }
    });
    let toolbar = toolbar.map(|toolbar| {
        view! { <div class="orbital-graph__toolbar">{toolbar()}</div> }
    });
    let minimap_overlay = move || {
        minimap.get().then(|| {
            let testid = data_testid.get().map(|id| format!("{id}-minimap"));
            view! {
                <div class="orbital-graph__minimap-slot">
                    <GraphMinimap handle=handle data_testid=testid />
                </div>
            }
        })
    };
    let svg_class = move || {
        if panning.get() {
            "orbital-graph__svg orbital-graph__svg--panning"
        } else {
            "orbital-graph__svg"
        }
    };

    view! {
        <div
            class=root_class
            data-testid=move || data_testid.get()
            style=move || height.get().map(|h| format!("height:{h};min-height:{h};"))
            data-background=background.as_str()
            node_ref=container
        >
            {grid}
            <svg
                class=svg_class
                role="group"
                aria-label=label
                on:wheel=on_wheel
                on:dblclick=on_dblclick
                on:pointerdown=on_pointerdown
                on:mousedown=on_mousedown
                on:auxclick=on_auxclick
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
            {toolbar}
            {minimap_overlay}
        </div>
    }
}

/// Rounds a px value to 0.001 for stable inline styles.
fn round_px(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

/// Writes the element's size into `size` now and on every resize, until the
/// owning effect is cleaned up.
fn observe_size(el: web_sys::HtmlDivElement, size: RwSignal<(f64, f64)>) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;

    let measure = |el: &web_sys::HtmlDivElement| {
        (f64::from(el.client_width()), f64::from(el.client_height()))
    };
    size.set(measure(&el));
    let target = el.clone();
    let on_resize = Closure::<dyn FnMut()>::new(move || {
        let next = measure(&target);
        if size.try_get_untracked().is_some_and(|now| now != next) {
            size.set(next);
        }
    });
    let Ok(observer) = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()) else {
        return;
    };
    observer.observe(&el);
    let held = SendWrapper::new((observer, on_resize));
    on_cleanup(move || {
        let (observer, on_resize) = held.take();
        observer.disconnect();
        drop(on_resize);
    });
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
    let on_dblclick = {
        let id = id.clone();
        move |ev: MouseEvent| {
            if let Some(open) = ctx.open {
                ev.stop_propagation();
                ev.prevent_default();
                open.run(id.clone());
            }
        }
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
            on:pointerdown=|ev: PointerEvent| {
                if node_swallows_pointerdown(ev.button()) {
                    ev.stop_propagation();
                }
            }
            on:click=on_click
            on:dblclick=on_dblclick
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
        // The workspace test build also enables leptos csr (component-preview-e2e),
        // so Effects spawn and need an executor even while rendering on the server.
        let _ = any_spawner::Executor::init_futures_executor();
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
        // Dot grid by default; no toolbar or minimap unless asked for.
        assert!(html.contains("data-background=\"dots\""), "{html}");
        assert!(html.contains("orbital-graph__grid orbital-graph__grid--dots"));
        assert!(html.contains("background-size: 18px 18px;"), "{html}");
        assert!(!html.contains("orbital-graph__toolbar"));
        assert!(!html.contains("orbital-graph-minimap"));
    }

    #[test]
    fn graph_ssr_toolbar_slot_and_no_grid_happy() {
        let _ = any_spawner::Executor::init_futures_executor();
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <Graph
                    input=Signal::stored(sample_project_input())
                    nodes=Signal::stored(sample_project_views())
                    background=GraphBackground::None
                    label="Billing revamp task graph"
                    toolbar=std::sync::Arc::new(|| view! {
                        <crate::GraphToolbar data_testid="tb" />
                    }.into_any())
                />
            }
            .to_html()
        });
        assert!(html.contains("data-background=\"none\""));
        assert!(!html.contains("orbital-graph__grid"));
        assert!(html.contains("orbital-graph__toolbar"), "{html}");
        for name in ["Zoom out", "Zoom in", "Fit to view"] {
            assert!(
                html.contains(&format!("aria-label=\"{name}\"")),
                "{name}: {html}"
            );
        }
        assert!(html.contains("aria-label=\"Graph controls\""));
        assert!(html.contains("data-testid=\"tb-minimap\""));
        // Still exactly one node tab stop with the toolbar present.
        assert_eq!(html.matches("tabindex=\"0\"").count(), 1);
    }

    #[test]
    fn drag_kind_maps_buttons_happy() {
        assert_eq!(drag_kind(0), Some(DragKind::Background));
        assert_eq!(drag_kind(1), Some(DragKind::Middle));
    }

    #[test]
    fn drag_kind_ignores_other_buttons_sad() {
        for button in [-1, 2, 3, 4] {
            assert_eq!(drag_kind(button), None);
        }
    }

    #[test]
    fn node_only_lets_middle_press_bubble_happy() {
        assert!(node_swallows_pointerdown(0));
        assert!(node_swallows_pointerdown(2));
        assert!(!node_swallows_pointerdown(1));
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
