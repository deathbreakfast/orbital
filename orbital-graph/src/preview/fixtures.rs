//! Sample graphs for the Graph and Graph Legend preview pages.

use crate::{
    EdgeKind, GraphEdge, GraphFill, GraphGroup, GraphInput, GraphLegendItem, GraphNode,
    GraphNodeView,
};

fn task(id: &str, group: Option<&str>) -> GraphNode {
    GraphNode {
        id: id.into(),
        group: group.map(str::to_string),
        external: group.is_none(),
        ..GraphNode::default()
    }
}

fn plain(id: &str) -> GraphNode {
    GraphNode {
        id: id.into(),
        ..GraphNode::default()
    }
}

fn waits(from: &str, to: &str) -> GraphEdge {
    GraphEdge {
        from: from.into(),
        to: to.into(),
        kind: EdgeKind::Depends,
    }
}

fn child_of(child: &str, parent: &str) -> GraphEdge {
    GraphEdge {
        from: child.into(),
        to: parent.into(),
        kind: EdgeKind::Child,
    }
}

fn node_view(id: &str, title: &str, footer: &str, fill: GraphFill, set: &str) -> GraphNodeView {
    GraphNodeView {
        id: id.into(),
        fill,
        lines: vec![title.into(), footer.into()],
        aria_label: None,
        set: Some(set.into()),
    }
}

/// Billing revamp: nine tasks in one project, two tasks from other projects
/// and one dependency cycle (`retry` and `receipts` wait on each other).
pub fn sample_project_input() -> GraphInput {
    let p = Some("billing");
    GraphInput {
        nodes: vec![
            task("schema", p),
            task("vault", p),
            task("invoice-api", p),
            task("retry", p),
            task("billing-ui", p),
            task("receipts", p),
            task("load-test", p),
            task("docs", p),
            task("launch", p),
            task("auth-tokens", None),
            task("ledger-export", None),
        ],
        edges: vec![
            waits("schema", "vault"),
            waits("schema", "invoice-api"),
            waits("invoice-api", "retry"),
            waits("vault", "billing-ui"),
            waits("invoice-api", "billing-ui"),
            waits("retry", "receipts"),
            waits("receipts", "retry"),
            waits("billing-ui", "load-test"),
            waits("receipts", "load-test"),
            waits("billing-ui", "docs"),
            waits("load-test", "launch"),
            waits("docs", "launch"),
            waits("auth-tokens", "vault"),
            waits("ledger-export", "receipts"),
        ],
        groups: vec![GraphGroup {
            id: "billing".into(),
            label: "Billing revamp".into(),
        }],
    }
}

/// Views for [`sample_project_input`], filled and grouped by status.
pub fn sample_project_views() -> Vec<GraphNodeView> {
    use GraphFill::{Brand, Green, Neutral, Red};
    vec![
        node_view(
            "schema",
            "Schema migration",
            "Data model · Ana · Done",
            Green,
            "done",
        ),
        node_view(
            "vault",
            "Vault cutover",
            "Vault cutover · Lena · Blocked",
            Red,
            "blocked",
        ),
        node_view(
            "invoice-api",
            "Invoice API",
            "API · Sam · In progress",
            Brand,
            "doing",
        ),
        node_view(
            "retry",
            "Retry worker",
            "API · Sam · To do",
            Neutral,
            "todo",
        ),
        node_view(
            "billing-ui",
            "Billing settings page",
            "UI · Kai · In progress",
            Brand,
            "doing",
        ),
        node_view(
            "receipts",
            "Email receipts",
            "API · Lena · To do",
            Neutral,
            "todo",
        ),
        node_view(
            "load-test",
            "Load test the checkout",
            "Launch · Ana · To do",
            Neutral,
            "todo",
        ),
        node_view(
            "docs",
            "Help centre docs",
            "Launch · Kai · To do",
            Neutral,
            "todo",
        ),
        node_view("launch", "Launch", "Launch · Sam · To do", Neutral, "todo"),
        node_view(
            "auth-tokens",
            "Auth tokens",
            "Identity · In progress",
            Neutral,
            "external",
        ),
        node_view(
            "ledger-export",
            "Ledger export",
            "Finance · Done",
            Neutral,
            "external",
        ),
    ]
}

/// Legend items for [`sample_project_views`], with counts.
pub fn sample_legend_items() -> Vec<GraphLegendItem> {
    let item = |set: &str, label: &str, fill, count| GraphLegendItem {
        set: set.into(),
        label: label.into(),
        fill,
        count: Some(count),
    };
    vec![
        item("done", "Done", GraphFill::Green, 1),
        item("doing", "In progress", GraphFill::Brand, 2),
        item("blocked", "Blocked", GraphFill::Red, 1),
        item("todo", "To do", GraphFill::Neutral, 5),
        item("external", "Other projects", GraphFill::Neutral, 2),
    ]
}

/// An epic with child tasks (dashed child → parent lines) and one blocker.
pub fn sample_parent_input() -> GraphInput {
    GraphInput {
        nodes: ["epic", "design", "build", "rollout", "contract"]
            .into_iter()
            .map(plain)
            .collect(),
        edges: vec![
            child_of("design", "epic"),
            child_of("build", "epic"),
            child_of("rollout", "epic"),
            waits("contract", "build"),
        ],
        groups: vec![],
    }
}

/// Views for [`sample_parent_input`].
pub fn sample_parent_views() -> Vec<GraphNodeView> {
    use GraphFill::{Brand, Green, Neutral, Yellow};
    vec![
        node_view("epic", "Billing revamp", "Epic · 3 children", Brand, "epic"),
        node_view("design", "Design review", "Child · Done", Green, "done"),
        node_view(
            "build",
            "Build checkout",
            "Child · In review",
            Yellow,
            "review",
        ),
        node_view(
            "rollout",
            "Staged rollout",
            "Child · To do",
            Neutral,
            "todo",
        ),
        node_view(
            "contract",
            "Vendor contract",
            "Blocks Build checkout",
            Neutral,
            "todo",
        ),
    ]
}

/// One node per [`GraphFill`], in two short chains.
pub fn sample_fill_input() -> GraphInput {
    GraphInput {
        nodes: GraphFill::ALL
            .iter()
            .map(|fill| plain(fill.wire_value()))
            .collect(),
        edges: vec![
            waits("neutral", "brand"),
            waits("brand", "red"),
            waits("orange", "yellow"),
            waits("yellow", "green"),
        ],
        groups: vec![],
    }
}

/// Views for [`sample_fill_input`], labelled with each fill's wire value.
pub fn sample_fill_views() -> Vec<GraphNodeView> {
    GraphFill::ALL
        .iter()
        .map(|&fill| GraphNodeView {
            id: fill.wire_value().into(),
            fill,
            lines: vec![fill.wire_value().into(), "GraphFill".into()],
            aria_label: None,
            set: None,
        })
        .collect()
}
