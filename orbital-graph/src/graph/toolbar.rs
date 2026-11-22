//! The `GraphToolbar` component: zoom, Fit and Minimap controls for a
//! `Graph`, with slots for the host's own buttons and filters.

use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexGap, FlexWrap, ToggleButton,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::graph::GraphHandle;
use super::styles::GRAPH_STYLES;

/// Canvas controls for a [`Graph`](crate::Graph): Zoom out, Zoom in, Fit and
/// a Minimap toggle, with `leading` and `trailing` slots for the host's own
/// buttons and filters.
///
/// Put it in the graph's `toolbar` slot and it overlays the top of the
/// canvas and finds the graph's [`GraphHandle`] by itself. It also works
/// outside the graph with a handle from `on_handle`.
///
/// # When to use
///
/// - Any [`Graph`](crate::Graph) users zoom and pan, so the controls are
///   visible and not only on the wheel and keyboard.
/// - Hosts with view options for the graph, such as Magnon's Hide done,
///   colour and milestone filters: put them in `leading` or `trailing` so
///   all canvas controls sit in one row.
///
/// # Usage
///
/// 1. Pass `toolbar` to the [`Graph`](crate::Graph) with a `GraphToolbar`
///    inside it; to place the toolbar elsewhere, pass the handle from the
///    graph's `on_handle` as `handle`.
/// 2. Give the graph a `minimap` signal if the host should own whether the
///    minimap shows; otherwise the toolbar's Minimap button toggles the
///    graph's own signal.
/// 3. Add host buttons and filters to `leading` (before the zoom controls) or
///    `trailing` (after them); a [`GraphLegend`](crate::GraphLegend) with
///    `collapsible` fits in `trailing`.
///
/// Every control is a real button. Zoom out and Zoom in have `aria-label`s
/// and are disabled at the zoom limits (0.2 and 2.2); Fit is named "Fit to
/// view"; Minimap is a pressed / not-pressed toggle. The toolbar is a
/// labelled group, and each button is its own tab stop, after the graph's
/// single one.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep slot content to buttons, toggles and compact filters; long text
///   wraps the toolbar onto more lines on a phone.
/// - Give icon-only slot buttons an `aria-label`, like the built-in ones.
///
/// ## Don'ts
///
/// - Don't hide the zoom controls to save space; they are the only visible
///   zoom for users without a wheel.
/// - Don't put a second Fit button in a slot; the built-in one also turns
///   auto-fit back on.
///
/// # Examples
///
/// ## Toolbar in the graph
/// The toolbar overlays the canvas, with a host button in `leading` and a
/// filter toggle in `trailing`. Zoom in until the button disables, or turn
/// the minimap on after zooming.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphToolbar};
/// use crate::preview::fixtures::{sample_project_input, sample_project_views};
/// use orbital_core_components::{Button, ButtonAppearance, ToggleButton};
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let clicks = RwSignal::new(0_u32);
/// let milestones = RwSignal::new(false);
/// view! {
///     <div data-testid="graph-toolbar-preview">
///         <Graph
///             input=input
///             nodes=nodes
///             height="440px"
///             label="Billing revamp task graph"
///             data_testid="graph-toolbar-preview-graph"
///             toolbar=std::sync::Arc::new(move || view! {
///                 <GraphToolbar
///                     data_testid="graph-toolbar-preview-toolbar"
///                     leading=std::sync::Arc::new(move || view! {
///                         <span data-testid="graph-toolbar-preview-action">
///                             <Button
///                                 appearance=ButtonAppearance::Subtle
///                                 on_click=Callback::new(move |_| clicks.update(|n| *n += 1))
///                             >
///                                 "Hide done"
///                             </Button>
///                         </span>
///                     }.into_any())
///                     trailing=std::sync::Arc::new(move || view! {
///                         <ToggleButton pressed=milestones>"Milestones"</ToggleButton>
///                     }.into_any())
///                 />
///             }.into_any())
///         />
///         <span data-testid="graph-toolbar-preview-clicks">
///             {move || clicks.get().to_string()}
///         </span>
///     </div>
/// }
/// ```
///
/// ## Toolbar with a legend
/// A collapsible [`GraphLegend`](crate::GraphLegend) in the `trailing` slot;
/// hiding a set refits the graph.
/// <!-- preview -->
/// ```rust
/// use std::collections::HashSet;
/// use crate::{Graph, GraphLegend, GraphToolbar};
/// use crate::preview::fixtures::{
///     sample_legend_items, sample_project_input, sample_project_views,
/// };
/// let input = RwSignal::new(sample_project_input());
/// let nodes = RwSignal::new(sample_project_views());
/// let items = RwSignal::new(sample_legend_items());
/// let hidden_sets = RwSignal::new(HashSet::<String>::new());
/// view! {
///     <div data-testid="graph-toolbar-legend">
///         <Graph
///             input=input
///             nodes=nodes
///             hidden_sets=hidden_sets
///             height="440px"
///             label="Billing revamp task graph"
///             data_testid="graph-toolbar-legend-graph"
///             toolbar=std::sync::Arc::new(move || view! {
///                 <GraphToolbar
///                     data_testid="graph-toolbar-legend-toolbar"
///                     trailing=std::sync::Arc::new(move || view! {
///                         <GraphLegend
///                             items=items
///                             hidden_sets=hidden_sets
///                             collapsible=true
///                             data_testid="graph-toolbar-legend-legend"
///                         />
///                     }.into_any())
///                 />
///             }.into_any())
///         />
///     </div>
/// }
/// ```
///
/// ## Toolbar outside the graph
/// The host keeps the [`GraphHandle`] from `on_handle` and passes it to a
/// toolbar above the graph.
/// <!-- preview -->
/// ```rust
/// use crate::{Graph, GraphHandle, GraphToolbar};
/// use crate::preview::fixtures::{sample_parent_input, sample_parent_views};
/// use orbital_core_components::{Flex, FlexGap};
/// let input = RwSignal::new(sample_parent_input());
/// let nodes = RwSignal::new(sample_parent_views());
/// let handle = RwSignal::new(None::<GraphHandle>);
/// view! {
///     <div data-testid="graph-toolbar-outside">
///         <Flex vertical=true gap=FlexGap::Small>
///             <GraphToolbar
///                 handle=handle
///                 show_minimap_toggle=false
///                 data_testid="graph-toolbar-outside-toolbar"
///             />
///             <div style="height: 320px;">
///                 <Graph
///                     input=input
///                     nodes=nodes
///                     on_handle=Callback::new(move |h| handle.set(Some(h)))
///                     label="Billing revamp parent tasks"
///                 />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "graph-toolbar",
    preview_label = "Graph Toolbar",
    preview_icon = icondata::AiZoomInOutlined,
)]
#[component]
pub fn GraphToolbar(
    /// The graph to control. Defaults to the [`GraphHandle`] of the
    /// [`Graph`](crate::Graph) whose `toolbar` slot holds this toolbar; pass
    /// a signal filled from `on_handle` to place it elsewhere. Without a
    /// handle only the slots render.
    #[prop(optional, into)]
    handle: MaybeProp<GraphHandle>,
    /// Slot before the zoom controls for host buttons and filters, such as
    /// "Hide done". Empty by default.
    #[prop(optional)]
    leading: Option<ChildrenFn>,
    /// Slot after the built-in controls for host buttons, filters or a
    /// collapsible [`GraphLegend`](crate::GraphLegend). Empty by default.
    #[prop(optional)]
    trailing: Option<ChildrenFn>,
    /// Shows the Minimap toggle (default `true`); turn it off for graphs too
    /// small to need one.
    #[prop(default = true)]
    show_minimap_toggle: bool,
    /// Accessible name of the toolbar's group (default "Graph controls").
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Extra CSS class names on the toolbar's root `div`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// `data-testid` on the root `div`; the built-in buttons get
    /// `{data_testid}-zoom-out`, `-zoom-in`, `-fit` and `-minimap`.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-graph", GRAPH_STYLES);
    let from_context = use_context::<GraphHandle>();
    let testid =
        move |suffix: &'static str| move || data_testid.get().map(|id| format!("{id}-{suffix}"));

    let controls = move || {
        handle.get().or(from_context).map(|handle| {
            let zoom_out = Callback::new(move |_| handle.zoom_out.run(()));
            let zoom_in = Callback::new(move |_| handle.zoom_in.run(()));
            let fit = Callback::new(move |_| handle.fit.run(()));
            let minimap_toggle = show_minimap_toggle.then(|| {
                view! {
                    <span class="orbital-graph-toolbar__minimap" data-testid=testid("minimap")>
                        <ToggleButton pressed=handle.minimap>"Minimap"</ToggleButton>
                    </span>
                }
            });
            view! {
                <Flex gap=FlexGap::Size(2) align=FlexAlign::Center>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        icon=icondata::AiZoomOutOutlined
                        disabled=Signal::derive(move || !handle.can_zoom_out.get())
                        on_click=zoom_out
                        attr:aria-label="Zoom out"
                        attr:title="Zoom out"
                        attr:data-testid=testid("zoom-out")
                    />
                    <Button
                        appearance=ButtonAppearance::Subtle
                        icon=icondata::AiZoomInOutlined
                        disabled=Signal::derive(move || !handle.can_zoom_in.get())
                        on_click=zoom_in
                        attr:aria-label="Zoom in"
                        attr:title="Zoom in"
                        attr:data-testid=testid("zoom-in")
                    />
                    <Button
                        appearance=ButtonAppearance::Subtle
                        icon=icondata::AiExpandOutlined
                        on_click=fit
                        attr:aria-label="Fit to view"
                        attr:title="Fit to view"
                        attr:data-testid=testid("fit")
                    />
                    {minimap_toggle}
                </Flex>
            }
        })
    };
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-graph-toolbar {extra}"),
        _ => "orbital-graph-toolbar".to_string(),
    };
    let aria_label = move || label.get().unwrap_or_else(|| "Graph controls".to_string());

    view! {
        <div
            class=root_class
            role="group"
            aria-label=aria_label
            data-testid=move || data_testid.get()
        >
            <Flex gap=FlexGap::Small wrap=FlexWrap::Wrap align=FlexAlign::Center>
                {leading.map(|leading| leading())}
                {controls}
                {trailing.map(|trailing| trailing())}
            </Flex>
        </div>
    }
}
