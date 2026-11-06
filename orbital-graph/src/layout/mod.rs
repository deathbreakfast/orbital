//! Layered layout for dependency graphs: node positions, group boxes and
//! right-angled line routes, computed from ids and edges alone.
//!
//! Call [`layout()`] with a [`GraphInput`] and [`LayoutOptions`]. The result is
//! plain geometry in one coordinate space: (0, 0) is the top left of the
//! drawing, x grows right and y grows down. Nothing here draws; a renderer
//! reads [`GraphLayout`] and emits SVG or anything else.
//!
//! # Columns
//!
//! A node's depth is the longest path to it from a source (a node nothing
//! waits on is depth 0, and every other node is one more than its deepest
//! blocker). Columns are the depths in use, numbered left to right with gaps
//! removed: if no node sits at depth 2, depth 3 becomes column 2. Every edge
//! therefore runs left to right.
//!
//! # Lanes, bands and rows
//!
//! Each [`GraphGroup`] gets a lane, in input order, followed by one lane for
//! internal nodes with no group (or a group id the input doesn't list). Lanes
//! stack top to bottom with two empty rows between them, and a lane with no
//! nodes is left out.
//!
//! Inside a lane, external nodes ([`GraphNode::external`]) form a band of
//! their own above the lane's internal nodes, separated by
//! [`LayoutOptions::stub_gap_rows`] empty rows. An external node joins the
//! lane of the first internal node that waits on it, or failing that the
//! first internal node it waits on, or failing that the first lane.
//!
//! Rows are filled one column at a time, left to right. The column's nodes
//! are sorted by the mean row of their already placed blockers in the same
//! band; nodes with none follow in input order. A node takes the row of its
//! first placed blocker when that row is still free in its column, so chains
//! run straight; the others take the lowest free rows in sorted order.
//! [`NodePlacement::row`] counts rows across the whole drawing, so the
//! empty rows between bands are numbered too.
//!
//! Each node is centred vertically on its row, so two nodes on one row line
//! up even when their heights differ.
//!
//! # Group boxes and stubs
//!
//! A [`GroupBox`] wraps its group's internal nodes with
//! [`LayoutOptions::group_padding`] (the top side is taller to leave room for
//! a label). External nodes are marked [`NodePlacement::stub`] and sit in the
//! band above the box, so a stub never overlaps a box. A stub keeps the
//! column of its depth, so an external blocker sits left of what waits on it.
//!
//! # Routing
//!
//! Each [`EdgeRoute`] starts at the middle of its source's right side and ends
//! at the middle of its target's left side, and every segment is horizontal
//! or vertical.
//!
//! Vertical runs sit in the gap right of a column. Each source node gets its
//! own track in that gap (in the order sources are first seen there, cycling
//! through four tracks 8 px apart), so the lines leaving one node share a
//! trunk. An edge to the next column is a single straight segment when the
//! rows match, and right, down or up, right otherwise.
//!
//! An edge that skips columns leaves its source the same way, then drops to
//! the channel halfway between its source's row and the next row towards the
//! target (offset by up to 6 px per source). It crosses the skipped columns
//! in that channel and climbs or drops to the target in the gap left of the
//! target's column. Channels lie between rows, so no line passes behind a
//! node.
//!
//! Lines stay clear of nodes as long as each node is narrower than
//! `column_pitch - 36` and shorter than `row_pitch - 12`; the defaults leave
//! room for [`NodeSize::default`].
//!
//! # Cycles
//!
//! A cycle can't be layered, so the layout breaks it instead of failing. A
//! depth-first walk over nodes in input order (following edges in input
//! order) removes each edge that closes a cycle and lists it in
//! [`GraphLayout::broken_cycles`]. A broken edge gets no route; the rest of the
//! graph lays out normally.
//!
//! # Bad edges
//!
//! Edges naming an id that isn't a node, edges from a node to itself, and
//! repeats of an earlier edge with the same `from`, `to` and `kind` are left
//! out and listed in [`GraphLayout::dropped`] with a [`DropReason`]. A node id
//! that appears twice keeps its first entry.
//!
//! # Truncation
//!
//! Past [`LayoutOptions::max_nodes`] nodes, only the first `max_nodes` (in
//! input order) are laid out and [`GraphLayout::truncated`] is set, so put the
//! nodes that matter most first. Edges touching a removed node are left out
//! without a [`DroppedEdge`] entry.
//!
//! # Determinism
//!
//! The same input always gives the same output, bit for bit. The layout runs
//! on the server and again in the browser during hydration, and the two must
//! agree. Positions come straight from integer columns and rows, with no
//! hashing order, randomness or clock involved.

mod cycles;
mod route;
mod rows;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

/// Everything [`layout()`] needs: the nodes, the edges between them and the
/// groups to box.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphInput {
    /// Nodes in the order to prefer them: input order breaks ties in row
    /// placement and decides which nodes survive truncation.
    pub nodes: Vec<GraphNode>,
    /// Edges between nodes, by id.
    pub edges: Vec<GraphEdge>,
    /// Groups to draw a box around, in the order to stack them top to bottom.
    pub groups: Vec<GraphGroup>,
}

/// One node of the graph, such as a task.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphNode {
    /// Unique id that edges refer to.
    pub id: String,
    /// Id of the [`GraphGroup`] the node belongs to. An id the input doesn't
    /// list counts as no group. Ignored for external nodes.
    pub group: Option<String>,
    /// The node comes from outside the drawn groups (a task in another
    /// project, say). It is placed above the lane it connects to, outside any
    /// box, and marked [`NodePlacement::stub`].
    pub external: bool,
    /// Size of the node's box.
    pub size: NodeSize,
}

/// Width and height of a node's box, in the layout's units (px for SVG).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeSize {
    /// Box width.
    pub width: f64,
    /// Box height.
    pub height: f64,
}

impl Default for NodeSize {
    /// 184 × 58, the standard task box.
    fn default() -> Self {
        Self {
            width: 184.0,
            height: 58.0,
        }
    }
}

/// A dependency between two nodes: `to` waits on `from`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphEdge {
    /// The node that must finish first (the blocker). Placed left of `to`.
    pub from: String,
    /// The node that waits.
    pub to: String,
    /// What the edge means; layout treats both kinds the same.
    pub kind: EdgeKind,
}

/// What an edge stands for. Layout treats every kind the same; a renderer
/// draws them differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EdgeKind {
    /// `to` depends on `from`.
    #[default]
    Depends,
    /// `from` is a child of `to`.
    Child,
}

/// A group of nodes drawn inside one box, such as a project.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphGroup {
    /// Id that [`GraphNode::group`] refers to.
    pub id: String,
    /// Label to show on the box; copied to [`GroupBox::label`].
    pub label: String,
}

/// Spacing and limits for [`layout()`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutOptions {
    /// Distance between the left edges of neighbouring columns. Default 228.
    pub column_pitch: f64,
    /// Distance between the centres of neighbouring rows. Default 88.
    pub row_pitch: f64,
    /// Space between a group box and the nodes inside it.
    pub group_padding: GroupPadding,
    /// Empty rows between a lane's stubs and its group box. Default 1.
    pub stub_gap_rows: u32,
    /// Most nodes to lay out; the rest are cut and
    /// [`GraphLayout::truncated`] is set. Default 1,500.
    pub max_nodes: usize,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            column_pitch: 228.0,
            row_pitch: 88.0,
            group_padding: GroupPadding::default(),
            stub_gap_rows: 1,
            max_nodes: 1_500,
        }
    }
}

/// Space between a group box and its nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupPadding {
    /// Left, right and bottom padding. Default 14.
    pub side: f64,
    /// Top padding, which holds the group label. Default 22.
    pub top: f64,
}

impl Default for GroupPadding {
    fn default() -> Self {
        Self {
            side: 14.0,
            top: 22.0,
        }
    }
}

/// The computed layout: where every node, box and line goes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphLayout {
    /// One placement per laid-out node, in input order.
    pub nodes: Vec<NodePlacement>,
    /// One box per group with at least one internal node, in input order.
    pub groups: Vec<GroupBox>,
    /// One route per kept edge, in input order.
    pub edges: Vec<EdgeRoute>,
    /// Right edge of the rightmost node, box or line point; 0 when empty.
    pub width: f64,
    /// Bottom edge of the lowest node, box or line point; 0 when empty.
    pub height: f64,
    /// Edges removed to break cycles, as `(from, to)`. They have no route.
    pub broken_cycles: Vec<(String, String)>,
    /// Edges left out because they were invalid.
    pub dropped: Vec<DroppedEdge>,
    /// More than [`LayoutOptions::max_nodes`] nodes came in and the rest
    /// were cut.
    pub truncated: bool,
}

/// Where one node goes.
#[derive(Debug, Clone, PartialEq)]
pub struct NodePlacement {
    /// The node's id.
    pub id: String,
    /// Left edge.
    pub x: f64,
    /// Top edge.
    pub y: f64,
    /// Box width, from [`GraphNode::size`].
    pub width: f64,
    /// Box height, from [`GraphNode::size`].
    pub height: f64,
    /// Column, counted from 0 at the left.
    pub column: u32,
    /// Row, counted from 0 at the top of the whole drawing.
    pub row: u32,
    /// The node is external and should be drawn as a stub.
    pub stub: bool,
}

/// A box around one group's internal nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupBox {
    /// The group's id.
    pub id: String,
    /// The group's label.
    pub label: String,
    /// Left edge.
    pub x: f64,
    /// Top edge.
    pub y: f64,
    /// Box width.
    pub width: f64,
    /// Box height.
    pub height: f64,
}

/// The line for one edge.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRoute {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// The edge's kind.
    pub kind: EdgeKind,
    /// Polyline from the middle of the source's right side to the middle of
    /// the target's left side. Every segment is horizontal or vertical.
    pub points: Vec<Point>,
}

/// A point in layout coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// Horizontal position.
    pub x: f64,
    /// Vertical position.
    pub y: f64,
}

/// An edge left out of the layout, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct DroppedEdge {
    /// The edge's `from` id.
    pub from: String,
    /// The edge's `to` id.
    pub to: String,
    /// Why it was left out.
    pub reason: DropReason,
}

/// Why an edge was left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DropReason {
    /// `from` or `to` names no node in the input.
    UnknownNode,
    /// `from` and `to` are the same node.
    SelfLoop,
    /// An earlier edge has the same `from`, `to` and `kind`.
    Duplicate,
}

/// An edge between node indexes, after cleaning.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Edge {
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) kind: EdgeKind,
}

/// Grid geometry shared by placement and routing. Every coordinate is
/// computed from an integer column or row, so no error accumulates.
pub(crate) struct Grid {
    pub(crate) side: f64,
    pub(crate) column_pitch: f64,
    pub(crate) row_pitch: f64,
    /// Centre line of row 0.
    pub(crate) centre0: f64,
    /// Widest node in each column.
    pub(crate) column_width: Vec<f64>,
}

impl Grid {
    pub(crate) fn x(&self, column: u32) -> f64 {
        self.side + f64::from(column) * self.column_pitch
    }

    pub(crate) fn centre(&self, row: u32) -> f64 {
        self.centre0 + f64::from(row) * self.row_pitch
    }

    /// The line halfway between rows: half row `2r + 1` lies between rows `r`
    /// and `r + 1`, and `-1` lies above row 0.
    pub(crate) fn half_row(&self, half: i64) -> f64 {
        self.centre0 + half as f64 * self.row_pitch / 2.0
    }

    /// x of vertical track `track` in the gap right of `column`.
    pub(crate) fn gutter(&self, column: u32, track: usize) -> f64 {
        let width = self
            .column_width
            .get(column as usize)
            .copied()
            .unwrap_or(0.0);
        self.x(column) + width + 12.0 + (track % 4) as f64 * 8.0
    }
}

/// Lays out `input`: columns by dependency depth, rows that keep chains
/// straight, a box per group, stubs for external nodes and orthogonal routes.
///
/// Never panics and never fails: cycles are broken and bad edges dropped,
/// with both reported on the result. See the [module docs](self) for the
/// rules.
#[must_use]
pub fn layout(input: &GraphInput, options: &LayoutOptions) -> GraphLayout {
    let mut out = GraphLayout::default();

    // Nodes: first entry per id wins, then cut to `max_nodes`.
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    let mut nodes: Vec<&GraphNode> = Vec::new();
    for node in &input.nodes {
        if !seen_ids.insert(node.id.as_str()) {
            continue;
        }
        if nodes.len() >= options.max_nodes {
            out.truncated = true;
            continue;
        }
        index.insert(node.id.as_str(), nodes.len());
        nodes.push(node);
    }
    if nodes.is_empty() {
        out.dropped = drop_all(input, &seen_ids);
        return out;
    }

    // Edges: drop unknown ids, self loops and repeats.
    let mut seen_edges: BTreeSet<(usize, usize, EdgeKind)> = BTreeSet::new();
    let mut edges: Vec<Edge> = Vec::new();
    for edge in &input.edges {
        let reason = match (index.get(edge.from.as_str()), index.get(edge.to.as_str())) {
            (Some(&from), Some(&to)) => {
                if from == to {
                    Some(DropReason::SelfLoop)
                } else if !seen_edges.insert((from, to, edge.kind)) {
                    Some(DropReason::Duplicate)
                } else {
                    edges.push(Edge {
                        from,
                        to,
                        kind: edge.kind,
                    });
                    None
                }
            }
            // Touches a node cut by truncation: leave it out silently.
            _ if seen_ids.contains(edge.from.as_str()) && seen_ids.contains(edge.to.as_str()) => {
                None
            }
            _ => Some(DropReason::UnknownNode),
        };
        if let Some(reason) = reason {
            out.dropped.push(DroppedEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                reason,
            });
        }
    }

    // Cycles, then depths over what's left.
    let back = cycles::back_edges(nodes.len(), &edges);
    let mut kept = Vec::with_capacity(edges.len());
    for (edge, is_back) in edges.iter().zip(&back) {
        if *is_back {
            out.broken_cycles
                .push((nodes[edge.from].id.clone(), nodes[edge.to].id.clone()));
        } else {
            kept.push(*edge);
        }
    }
    let depth = cycles::depths(nodes.len(), &kept);
    let mut used = depth.clone();
    used.sort_unstable();
    used.dedup();
    let column: Vec<u32> = depth
        .iter()
        .map(|d| used.partition_point(|u| u < d) as u32)
        .collect();

    // Group of each internal node, as an index into `input.groups`.
    let mut group_index: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, group) in input.groups.iter().enumerate() {
        group_index.entry(group.id.as_str()).or_insert(i);
    }
    let group_of: Vec<Option<usize>> = nodes
        .iter()
        .map(|n| {
            if n.external {
                None
            } else {
                n.group.as_deref().and_then(|g| group_index.get(g).copied())
            }
        })
        .collect();

    let row = rows::assign(
        &nodes,
        &kept,
        &column,
        &group_of,
        input.groups.len(),
        options.stub_gap_rows,
    );

    let max_height = nodes.iter().map(|n| n.size.height).fold(0.0, f64::max);
    let mut column_width = vec![0.0_f64; used.len()];
    for (n, &c) in nodes.iter().zip(&column) {
        let w = &mut column_width[c as usize];
        *w = w.max(n.size.width);
    }
    let pad = options.group_padding;
    let grid = Grid {
        side: pad.side,
        column_pitch: options.column_pitch,
        row_pitch: options.row_pitch,
        centre0: pad.top + max_height / 2.0,
        column_width,
    };

    out.nodes = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| NodePlacement {
            id: n.id.clone(),
            x: grid.x(column[i]),
            y: grid.centre(row[i]) - n.size.height / 2.0,
            width: n.size.width,
            height: n.size.height,
            column: column[i],
            row: row[i],
            stub: n.external,
        })
        .collect();

    for (g, group) in input.groups.iter().enumerate() {
        if group_index.get(group.id.as_str()) != Some(&g) {
            continue;
        }
        let mut bounds: Option<(f64, f64, f64, f64)> = None;
        for (p, _) in out
            .nodes
            .iter()
            .zip(&group_of)
            .filter(|(_, o)| **o == Some(g))
        {
            let (x0, y0, x1, y1) = (p.x, p.y, p.x + p.width, p.y + p.height);
            bounds = Some(match bounds {
                None => (x0, y0, x1, y1),
                Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
            });
        }
        if let Some((x0, y0, x1, y1)) = bounds {
            out.groups.push(GroupBox {
                id: group.id.clone(),
                label: group.label.clone(),
                x: x0 - pad.side,
                y: y0 - pad.top,
                width: x1 - x0 + 2.0 * pad.side,
                height: y1 - y0 + pad.top + pad.side,
            });
        }
    }

    out.edges = route::routes(&nodes, &kept, &column, &row, &grid);

    for p in &out.nodes {
        out.width = out.width.max(p.x + p.width);
        out.height = out.height.max(p.y + p.height);
    }
    for b in &out.groups {
        out.width = out.width.max(b.x + b.width);
        out.height = out.height.max(b.y + b.height);
    }
    for point in out.edges.iter().flat_map(|e| &e.points) {
        out.width = out.width.max(point.x);
        out.height = out.height.max(point.y);
    }
    out
}

/// With no nodes laid out, every edge is either unknown or cut.
fn drop_all(input: &GraphInput, seen_ids: &BTreeSet<&str>) -> Vec<DroppedEdge> {
    input
        .edges
        .iter()
        .filter(|e| !(seen_ids.contains(e.from.as_str()) && seen_ids.contains(e.to.as_str())))
        .map(|e| DroppedEdge {
            from: e.from.clone(),
            to: e.to.clone(),
            reason: DropReason::UnknownNode,
        })
        .collect()
}
