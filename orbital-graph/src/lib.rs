//! Orbital Graph lays out dependency graphs: which column and row each node
//! goes in, the box around each group, and right-angled lines between nodes.
//!
//! Use it to draw tasks and what blocks them, left to right by how deep each
//! task waits, with a project drawn as a box and tasks from other projects as
//! stubs just outside it. The layout is pure geometry and has no UI of its
//! own; the [`Graph`] component draws the result as SVG and [`GraphLegend`]
//! hides and shows sets of its nodes.
//!
//! # Getting started
//!
//! Build a [`GraphInput`] and call [`layout()`] once. This lays out three tasks
//! in one project plus a blocker from another project, then reads a position
//! and a route:
//!
//! ```
//! use orbital_graph::{
//!     layout, EdgeKind, GraphEdge, GraphGroup, GraphInput, GraphNode, LayoutOptions,
//! };
//!
//! let task = |id: &str, external: bool| GraphNode {
//!     id: id.into(),
//!     group: (!external).then(|| "magnon".to_string()),
//!     external,
//!     ..GraphNode::default()
//! };
//! let waits = |from: &str, to: &str| GraphEdge {
//!     from: from.into(),
//!     to: to.into(),
//!     kind: EdgeKind::Depends,
//! };
//! let input = GraphInput {
//!     nodes: vec![
//!         task("schema", false),
//!         task("api", false),
//!         task("ui", false),
//!         task("auth", true),
//!     ],
//!     edges: vec![waits("schema", "api"), waits("api", "ui"), waits("auth", "api")],
//!     groups: vec![GraphGroup { id: "magnon".into(), label: "Magnon".into() }],
//! };
//!
//! let graph = layout(&input, &LayoutOptions::default());
//!
//! // `ui` waits on `api`, which waits on `schema`: third column.
//! let ui = graph.nodes.iter().find(|n| n.id == "ui").expect("ui is placed");
//! assert_eq!(ui.column, 2);
//!
//! // `auth` is from another project: a stub above the project's box.
//! let auth = graph.nodes.iter().find(|n| n.id == "auth").expect("auth is placed");
//! assert!(auth.stub);
//! assert!(auth.y + auth.height < graph.groups[0].y);
//!
//! // Its line ends at the middle of `api`'s left side.
//! let api = graph.nodes.iter().find(|n| n.id == "api").expect("api is placed");
//! let route = graph.edges.iter().find(|e| e.from == "auth").expect("auth → api is routed");
//! let end = route.points.last().expect("a route has points");
//! assert_eq!((end.x, end.y), (api.x, api.y + api.height / 2.0));
//! ```
//!
//! # Features
//!
//! - **Layered graph layout** — [`layout()`] places nodes in columns by
//!   longest dependency path and orders rows so chains stay straight. See
//!   the [`layout` module](mod@crate::layout) for every rule.
//! - **Group boxes and external stubs** — [`GraphGroup`]s become
//!   [`GroupBox`]es around their nodes, and [`GraphNode::external`] nodes sit
//!   just above the box they connect to, marked [`NodePlacement::stub`].
//! - **Orthogonal routes** — each [`EdgeRoute`] is a right-angled polyline
//!   that never passes behind a node.
//! - **Graph component** — [`Graph`] draws a layout with selection, chain
//!   mode, pan, zoom, Fit and keyboard navigation, and [`GraphLegend`] toggles
//!   sets of nodes. See the [`Graph`] docs for the wiring.
//! - **Bad input reporting** — cycles are broken into
//!   [`GraphLayout::broken_cycles`], invalid edges go to
//!   [`GraphLayout::dropped`], and [`LayoutOptions::max_nodes`] caps the
//!   drawing with [`GraphLayout::truncated`] set.
//!
//! # Feature flags
//!
//! The layout needs only `std`; the components need Leptos and Orbital.
//! Everything builds with `default-features = false`.
//!
//! - `preview` (default) — the catalog pages and their sample graphs.
//! - `ssr` — server rendering for the components.
//! - `hydrate` — client hydration for the components.

#[cfg(feature = "preview")]
pub mod preview;

#[cfg(feature = "preview")]
pub(crate) mod components {
    pub use crate::preview::{ComponentPreviewCard, OrbitalComponentView};
}

mod graph;
pub mod layout;

pub use graph::*;
pub use layout::*;
