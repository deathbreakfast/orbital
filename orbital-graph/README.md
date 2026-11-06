# orbital-graph

Layered layout for dependency graphs: columns by how deep each node waits, rows that keep chains straight, a box per group, stubs for nodes from outside the group, and right-angled lines that never pass behind a node.

The crate is the layout only. It returns positions and line points and draws nothing.

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

The layout needs only `std`. Use `default-features = false` unless you need the graph components' `ssr` / `hydrate` wiring.

## Key types

- `layout(&GraphInput, &LayoutOptions) -> GraphLayout`
- Input: `GraphInput`, `GraphNode`, `GraphEdge` (`from` is the blocker, `to` waits), `GraphGroup`, `NodeSize`
- Output: `NodePlacement`, `GroupBox`, `EdgeRoute`, `Point`, plus `broken_cycles`, `dropped` and `truncated` for input the layout had to repair

## Deferred (not in current charter)

- `Graph` and `GraphLegend` Leptos components that draw a `GraphLayout` as SVG, with a preview page

## Docs

The rules for columns, rows, stubs, routing, cycles and truncation are in the `layout` module rustdoc. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
