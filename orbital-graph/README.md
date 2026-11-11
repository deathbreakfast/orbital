# orbital-graph

Layered layout for dependency graphs: columns by how deep each node waits, Brandes–Köpf placement that keeps chains straight and centres a node on what it leads to, a box per group, stubs for nodes from outside the group, and right-angled lines that never pass behind a node.

The layout returns positions and line points and draws nothing. The `Graph` and `GraphLegend` components draw it as SVG.

## Quick start

```toml
[dependencies]
orbital-graph = { version = "0.1", default-features = false }
```

```rust
use orbital_graph::{layout, GraphEdge, GraphInput, GraphNode, LayoutOptions};

let input = GraphInput {
    nodes: vec![
        GraphNode { id: "schema".into(), ..GraphNode::default() },
        GraphNode { id: "api".into(), ..GraphNode::default() },
    ],
    edges: vec![GraphEdge { from: "schema".into(), to: "api".into(), ..GraphEdge::default() }],
    groups: vec![],
};
let graph = layout(&input, &LayoutOptions::default());
assert_eq!(graph.nodes[1].column, 1);
```

Use `default-features = false` unless you want the preview pages; turn on `ssr` / `hydrate` when you render the components.

## Key types

- `layout(&GraphInput, &LayoutOptions) -> GraphLayout`
- Input: `GraphInput`, `GraphNode`, `GraphEdge` (`from` is the blocker, `to` waits), `GraphGroup`, `NodeSize`
- Output: `NodePlacement`, `GroupBox`, `EdgeRoute`, `Point`, plus `broken_cycles`, `dropped` and `truncated` for input the layout had to repair, and `crossings` for the line crossings left after ordering
- Components: `Graph`, `GraphLegend`, with `GraphNodeView`, `GraphFill`, `GraphLegendItem`, `GraphLegendLine` and `GraphHandle`

## Placement

Nodes sit in columns by depth: a node is one column right of its deepest blocker. Within each group's lane, every column is ordered to cut line crossings: a depth-first first order, then twelve barycenter sweeps, keeping the order with the fewest crossings. Brandes–Köpf coordinate assignment then places each node:

```text
          /[2]                       |-[2] - [5]
 [1] -+ - [3]                        |       |-[6]
          L[4]             [1] -+ - [3] - [7]
                                     L[4]
```

A node with an odd number of dependants sits level with the middle one (1 with 3), a node with two sits midway between them (2 between 5 and 6, on a half row), and a chain runs straight (3 and 7). A line that skips columns keeps a slot free in each column it crosses, so it never runs behind a box. The same input always gives the same positions, so server rendering and hydration agree.

## Drawing a graph

`Graph` takes the layout input and the view data separately, so recolouring a node never moves it:

```rust
use std::collections::HashSet;
use leptos::prelude::*;
use orbital_graph::{
    Graph, GraphEdge, GraphFill, GraphInput, GraphLegend, GraphLegendItem, GraphNode,
    GraphNodeView,
};

#[component]
fn TaskGraph() -> impl IntoView {
    let input = RwSignal::new(GraphInput {
        nodes: vec![
            GraphNode { id: "schema".into(), ..GraphNode::default() },
            GraphNode { id: "api".into(), ..GraphNode::default() },
        ],
        edges: vec![GraphEdge { from: "schema".into(), to: "api".into(), ..GraphEdge::default() }],
        groups: vec![],
    });
    let nodes = RwSignal::new(vec![
        GraphNodeView {
            id: "schema".into(),
            fill: GraphFill::Green,
            lines: vec!["Schema migration".into(), "Done".into()],
            set: Some("done".into()),
            ..GraphNodeView::default()
        },
        GraphNodeView {
            id: "api".into(),
            lines: vec!["Invoice API".into(), "To do".into()],
            set: Some("todo".into()),
            ..GraphNodeView::default()
        },
    ]);
    let items = RwSignal::new(vec![GraphLegendItem {
        set: "done".into(),
        label: "Done".into(),
        fill: GraphFill::Green,
        count: Some(1),
    }]);
    let hidden_sets = RwSignal::new(HashSet::new());
    let selected = RwSignal::new(None::<String>);

    view! {
        <GraphLegend items=items hidden_sets=hidden_sets />
        <div style="height: 480px;">
            <Graph
                input=input
                nodes=nodes
                selected=selected
                hidden_sets=hidden_sets
                label="Billing task graph"
            />
        </div>
    }
}
```

- Click a node, or press Enter or Space on it, to select it; the rest dims. Pass `chain=true` to keep the whole chain of predecessors and successors bright.
- Drag the background, or middle-drag anywhere, to pan; scroll to zoom (20% to 220%). Keep the `GraphHandle` from `on_handle` to wire a Fit button.
- The graph is one tab stop: arrow keys move between nodes, Escape clears the selection, `+` and `-` zoom and `0` fits.
- `GraphFill` has six fills (`neutral`, `brand`, `red`, `orange`, `yellow`, `green`), each mapped to theme tokens. Store `wire_value()` if you save colour choices; the strings don't change.

Enable `ssr` on the server build and `hydrate` on the client build, like the other Orbital crates.

## Docs

The rules for columns, placement, stubs, routing, cycles and truncation are in the `layout` module rustdoc. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
