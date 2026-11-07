//! View data for [`Graph`](crate::Graph) and the pure rules behind hiding,
//! selection and line truncation.

use std::collections::{HashMap, HashSet};

use crate::{GraphFill, GraphInput};

/// Most text lines a node shows; extra lines are dropped.
pub const MAX_NODE_LINES: usize = 3;

/// Approximate advance of one character, in px, for the 14px semibold first
/// line and the 12px lines below it. Used to truncate before the box edge.
const FIRST_LINE_CHAR_PX: f64 = 7.6;
const OTHER_LINE_CHAR_PX: f64 = 6.6;
/// Left plus right text inset inside a node box.
const TEXT_INSET_PX: f64 = 24.0;

/// What a node looks like, keyed by the id of a [`GraphNode`](crate::GraphNode).
///
/// Kept apart from the layout input so a host can recolour or relabel
/// without a new layout.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphNodeView {
    /// Id of the node this view draws.
    pub id: String,
    /// Background, border and text tokens.
    pub fill: GraphFill,
    /// One to three text lines: the first is the title (14px semibold), the
    /// rest are 12px. Extra lines are dropped; each line is cut with `…` to
    /// fit the box.
    pub lines: Vec<String>,
    /// Accessible name; defaults to the lines joined with ", ".
    pub aria_label: Option<String>,
    /// Legend set the node belongs to, for [`Graph`](crate::Graph)'s
    /// `hidden_sets`.
    pub set: Option<String>,
}

impl GraphNodeView {
    /// The lines to draw in a box `width` px wide: at most three, each cut
    /// with `…` to fit.
    #[must_use]
    pub fn display_lines(&self, width: f64) -> Vec<String> {
        self.lines
            .iter()
            .take(MAX_NODE_LINES)
            .enumerate()
            .map(|(i, line)| {
                let char_px = if i == 0 {
                    FIRST_LINE_CHAR_PX
                } else {
                    OTHER_LINE_CHAR_PX
                };
                truncate_line(line, max_chars(width, char_px))
            })
            .collect()
    }

    /// The accessible name: `aria_label`, or the (uncut) lines joined with
    /// ", ", or the id when there are no lines.
    #[must_use]
    pub fn accessible_name(&self) -> String {
        if let Some(label) = &self.aria_label {
            return label.clone();
        }
        let lines: Vec<&str> = self
            .lines
            .iter()
            .take(MAX_NODE_LINES)
            .map(String::as_str)
            .collect();
        if lines.is_empty() {
            self.id.clone()
        } else {
            lines.join(", ")
        }
    }
}

fn max_chars(width: f64, char_px: f64) -> usize {
    let room = (width - TEXT_INSET_PX).max(0.0) / char_px;
    if room.is_finite() {
        (room.floor() as usize).max(1)
    } else {
        1
    }
}

/// Cuts `line` to `max` characters, ending in `…` when anything was cut.
pub(crate) fn truncate_line(line: &str, max: usize) -> String {
    if line.chars().count() <= max {
        return line.to_string();
    }
    let kept: String = line.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", kept.trim_end())
}

/// Baseline offsets, from the top of a box `height` px tall, for `count`
/// lines: centred as a block, 18px apart after the first line and 16px after
/// that.
pub(crate) fn line_baselines(count: usize, height: f64) -> Vec<f64> {
    let gaps: Vec<f64> = (1..count)
        .map(|i| if i == 1 { 18.0 } else { 16.0 })
        .collect();
    let block: f64 = gaps.iter().sum();
    // Baseline of a single line sits about 5px below the box's middle.
    let mut y = height / 2.0 - block / 2.0 + 5.0;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(y);
        if let Some(gap) = gaps.get(i) {
            y += gap;
        }
    }
    out
}

/// Views by node id; the first view wins when ids repeat.
pub(crate) fn views_by_id(views: &[GraphNodeView]) -> HashMap<String, GraphNodeView> {
    let mut map = HashMap::with_capacity(views.len());
    for view in views {
        map.entry(view.id.clone()).or_insert_with(|| view.clone());
    }
    map
}

/// `input` without the nodes whose view's `set` is in `hidden`, and without
/// every edge that touches one of them.
#[must_use]
pub(crate) fn filter_hidden(
    input: &GraphInput,
    views: &[GraphNodeView],
    hidden: &HashSet<String>,
) -> GraphInput {
    if hidden.is_empty() {
        return input.clone();
    }
    let removed: HashSet<&str> = views
        .iter()
        .filter(|v| v.set.as_ref().is_some_and(|s| hidden.contains(s)))
        .map(|v| v.id.as_str())
        .collect();
    if removed.is_empty() {
        return input.clone();
    }
    GraphInput {
        nodes: input
            .nodes
            .iter()
            .filter(|n| !removed.contains(n.id.as_str()))
            .cloned()
            .collect(),
        edges: input
            .edges
            .iter()
            .filter(|e| !removed.contains(e.from.as_str()) && !removed.contains(e.to.as_str()))
            .cloned()
            .collect(),
        groups: input.groups.clone(),
    }
}

/// The nodes and edges a selection keeps undimmed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Highlight {
    pub(crate) nodes: HashSet<String>,
    pub(crate) edges: HashSet<(String, String)>,
}

impl Highlight {
    pub(crate) fn has_edge(&self, from: &str, to: &str) -> bool {
        self.edges.contains(&(from.to_string(), to.to_string()))
    }
}

/// The selected node, its direct predecessors and successors, and the edges
/// that touch the selected node.
pub(crate) fn neighbourhood(input: &GraphInput, selected: &str) -> Highlight {
    let mut out = Highlight::default();
    out.nodes.insert(selected.to_string());
    for edge in &input.edges {
        if edge.from == selected || edge.to == selected {
            out.nodes.insert(edge.from.clone());
            out.nodes.insert(edge.to.clone());
            out.edges.insert((edge.from.clone(), edge.to.clone()));
        }
    }
    out
}

/// The selected node with every node it waits on (directly or not) and every
/// node that waits on it, plus the edges inside each side.
pub(crate) fn chain_closure(input: &GraphInput, selected: &str) -> Highlight {
    let reach = |forward: bool| -> HashSet<String> {
        let mut seen = HashSet::from([selected.to_string()]);
        let mut stack = vec![selected.to_string()];
        while let Some(id) = stack.pop() {
            for edge in &input.edges {
                let (here, next) = if forward {
                    (&edge.from, &edge.to)
                } else {
                    (&edge.to, &edge.from)
                };
                if *here == id && seen.insert(next.clone()) {
                    stack.push(next.clone());
                }
            }
        }
        seen
    };
    let ancestors = reach(false);
    let descendants = reach(true);
    let mut out = Highlight::default();
    for edge in &input.edges {
        let inside = |side: &HashSet<String>| side.contains(&edge.from) && side.contains(&edge.to);
        if inside(&ancestors) || inside(&descendants) {
            out.edges.insert((edge.from.clone(), edge.to.clone()));
        }
    }
    out.nodes = ancestors;
    out.nodes.extend(descendants);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EdgeKind, GraphEdge, GraphNode};

    fn edge(from: &str, to: &str) -> GraphEdge {
        GraphEdge {
            from: from.into(),
            to: to.into(),
            kind: EdgeKind::Depends,
        }
    }

    /// a → b → c → d, plus x → c and b → y.
    fn chain_input() -> GraphInput {
        GraphInput {
            nodes: ["a", "b", "c", "d", "x", "y"]
                .into_iter()
                .map(|id| GraphNode {
                    id: id.into(),
                    ..GraphNode::default()
                })
                .collect(),
            edges: vec![
                edge("a", "b"),
                edge("b", "c"),
                edge("c", "d"),
                edge("x", "c"),
                edge("b", "y"),
            ],
            groups: vec![],
        }
    }

    fn ids(set: &HashSet<String>) -> Vec<&str> {
        let mut v: Vec<&str> = set.iter().map(String::as_str).collect();
        v.sort_unstable();
        v
    }

    #[test]
    fn chain_closure_includes_all_predecessors_and_successors_happy() {
        let h = chain_closure(&chain_input(), "c");
        assert_eq!(ids(&h.nodes), ["a", "b", "c", "d", "x"]);
        assert!(h.has_edge("a", "b"));
        assert!(h.has_edge("b", "c"));
        assert!(h.has_edge("x", "c"));
        assert!(h.has_edge("c", "d"));
        // `y` waits on `b` but is not on `c`'s chain.
        assert!(!h.has_edge("b", "y"));
    }

    #[test]
    fn neighbourhood_keeps_direct_edges_only_happy() {
        let h = neighbourhood(&chain_input(), "c");
        assert_eq!(ids(&h.nodes), ["b", "c", "d", "x"]);
        assert_eq!(h.edges.len(), 3);
        assert!(!h.has_edge("a", "b"));
    }

    #[test]
    fn chain_closure_survives_a_cycle_sad() {
        let mut input = chain_input();
        input.edges.push(edge("d", "a"));
        let h = chain_closure(&input, "c");
        assert!(h.nodes.contains("y"));
        assert_eq!(h.nodes.len(), 6);
    }

    #[test]
    fn node_lines_capped_at_three_sad() {
        let view = GraphNodeView {
            id: "n".into(),
            lines: vec![
                "MAG-12".into(),
                "Short title".into(),
                "Footer".into(),
                "Dropped".into(),
            ],
            ..GraphNodeView::default()
        };
        let lines = view.display_lines(184.0);
        assert_eq!(lines, ["MAG-12", "Short title", "Footer"]);
        assert_eq!(view.accessible_name(), "MAG-12, Short title, Footer");
    }

    #[test]
    fn node_long_line_truncated_with_ellipsis_sad() {
        let view = GraphNodeView {
            id: "n".into(),
            lines: vec!["Rotate the billing vault keys before cutover".into()],
            ..GraphNodeView::default()
        };
        let line = &view.display_lines(184.0)[0];
        assert!(line.ends_with('…'), "{line}");
        assert!(line.chars().count() <= 21, "{line}");
        // The accessible name keeps the whole line.
        assert_eq!(
            view.accessible_name(),
            "Rotate the billing vault keys before cutover"
        );
    }

    #[test]
    fn node_without_lines_names_itself_by_id_sad() {
        let view = GraphNodeView {
            id: "orphan".into(),
            ..GraphNodeView::default()
        };
        assert!(view.display_lines(184.0).is_empty());
        assert_eq!(view.accessible_name(), "orphan");
        assert_eq!(max_chars(0.0, 7.6), 1);
    }

    #[test]
    fn line_baselines_centre_the_block_happy() {
        assert_eq!(line_baselines(1, 58.0), [34.0]);
        assert_eq!(line_baselines(2, 58.0), [25.0, 43.0]);
        assert_eq!(line_baselines(3, 58.0), [17.0, 35.0, 51.0]);
    }

    #[test]
    fn hidden_set_removes_nodes_and_edges_happy() {
        let views = vec![
            GraphNodeView {
                id: "b".into(),
                set: Some("done".into()),
                ..GraphNodeView::default()
            },
            GraphNodeView {
                id: "c".into(),
                set: Some("open".into()),
                ..GraphNodeView::default()
            },
        ];
        let hidden = HashSet::from(["done".to_string()]);
        let out = filter_hidden(&chain_input(), &views, &hidden);
        assert!(out.nodes.iter().all(|n| n.id != "b"));
        assert_eq!(out.nodes.len(), 5);
        assert!(out.edges.iter().all(|e| e.from != "b" && e.to != "b"));
        assert_eq!(out.edges.len(), 2);

        // Nothing hidden: the input comes back unchanged.
        assert_eq!(
            filter_hidden(&chain_input(), &views, &HashSet::new()),
            chain_input()
        );
    }
}
