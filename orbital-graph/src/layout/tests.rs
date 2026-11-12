use super::*;

const EPS: f64 = 1e-9;

fn node(id: &str) -> GraphNode {
    GraphNode {
        id: id.into(),
        ..GraphNode::default()
    }
}

fn in_group(id: &str, group: &str) -> GraphNode {
    GraphNode {
        group: Some(group.into()),
        ..node(id)
    }
}

fn external(id: &str) -> GraphNode {
    GraphNode {
        external: true,
        ..node(id)
    }
}

fn edge(from: &str, to: &str) -> GraphEdge {
    GraphEdge {
        from: from.into(),
        to: to.into(),
        kind: EdgeKind::Depends,
    }
}

fn group(id: &str) -> GraphGroup {
    GraphGroup {
        id: id.into(),
        label: id.to_uppercase(),
    }
}

fn graph(nodes: Vec<GraphNode>, edges: Vec<GraphEdge>) -> GraphInput {
    GraphInput {
        nodes,
        edges,
        groups: Vec::new(),
    }
}

fn place<'a>(layout: &'a GraphLayout, id: &str) -> &'a NodePlacement {
    layout
        .nodes
        .iter()
        .find(|n| n.id == id)
        .unwrap_or_else(|| panic!("{id} is placed"))
}

fn route<'a>(layout: &'a GraphLayout, from: &str, to: &str) -> &'a EdgeRoute {
    layout
        .edges
        .iter()
        .find(|e| e.from == from && e.to == to)
        .unwrap_or_else(|| panic!("{from} → {to} is routed"))
}

fn intersects(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
}

fn centre(layout: &GraphLayout, id: &str) -> f64 {
    let n = place(layout, id);
    n.y + n.height / 2.0
}

fn assert_no_overlaps(out: &GraphLayout) {
    for (i, a) in out.nodes.iter().enumerate() {
        for b in &out.nodes[i + 1..] {
            assert!(
                !intersects((a.x, a.y, a.width, a.height), (b.x, b.y, b.width, b.height)),
                "{} overlaps {}",
                a.id,
                b.id
            );
        }
    }
}

/// Every route is orthogonal and no segment passes through a node other
/// than its own ends.
fn assert_routes_clear(out: &GraphLayout) {
    for r in &out.edges {
        for w in r.points.windows(2) {
            let (a, b) = (w[0], w[1]);
            assert!((a.x - b.x).abs() < EPS || (a.y - b.y).abs() < EPS);
            let (x0, x1, y0, y1) = (a.x.min(b.x), a.x.max(b.x), a.y.min(b.y), a.y.max(b.y));
            for n in out.nodes.iter().filter(|n| n.id != r.from && n.id != r.to) {
                let crosses = x0 < n.x + n.width - EPS
                    && x1 > n.x + EPS
                    && y0 < n.y + n.height - EPS
                    && y1 > n.y + EPS;
                assert!(!crosses, "{} → {} passes behind {}", r.from, r.to, n.id);
            }
        }
    }
}

/// Three layers of eight vertices with seeded random edges between
/// neighbouring layers.
fn seeded_band(seed: u64) -> rows::BandGraph {
    let mut state = seed;
    let mut next = move || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (state >> 33) as usize
    };
    let n = 24;
    let mut preds = vec![Vec::new(); n];
    let mut succs = vec![Vec::new(); n];
    for (v, vp) in preds.iter_mut().enumerate().skip(8) {
        let base = v / 8 * 8 - 8;
        for _ in 0..2 {
            let u = base + next() % 8;
            if !vp.contains(&u) {
                vp.push(u);
                succs[u].push(v);
            }
        }
    }
    rows::BandGraph {
        layer: (0..n).map(|v| v / 8).collect(),
        layer_count: 3,
        preds,
        succs,
        dummy: vec![false; n],
    }
}

#[test]
fn layout_is_deterministic_happy() {
    let input = GraphInput {
        nodes: vec![
            in_group("a", "p"),
            in_group("b", "p"),
            in_group("c", "q"),
            node("d"),
            external("x"),
            in_group("e", "q"),
        ],
        edges: vec![
            edge("a", "b"),
            edge("x", "b"),
            edge("b", "c"),
            edge("a", "e"),
            edge("d", "e"),
            edge("a", "c"),
        ],
        groups: vec![group("p"), group("q")],
    };
    let options = LayoutOptions::default();
    let first = layout(&input, &options);
    let second = layout(&input.clone(), &options);
    assert_eq!(first, second);
    assert_eq!(first.nodes.len(), 6);
}

#[test]
fn layout_columns_follow_longest_path_happy() {
    let out = layout(
        &graph(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("a", "c"), edge("b", "c")],
        ),
        &LayoutOptions::default(),
    );
    assert_eq!(place(&out, "a").column, 0);
    assert_eq!(place(&out, "b").column, 1);
    assert_eq!(place(&out, "c").column, 2);
}

#[test]
fn layout_first_predecessor_row_kept_happy() {
    let out = layout(
        &graph(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("b", "c")],
        ),
        &LayoutOptions::default(),
    );
    let row = place(&out, "a").row;
    assert!(out.nodes.iter().all(|n| n.row == row));
    assert_eq!(out.edges.len(), 2);
    for r in &out.edges {
        assert_eq!(r.points.len(), 2);
        assert!((r.points[0].y - r.points[1].y).abs() < EPS);
    }
}

#[test]
fn layout_barycenter_orders_rows_happy() {
    let out = layout(
        &graph(
            vec![node("a1"), node("b1"), node("a2"), node("b2"), node("m")],
            vec![
                edge("a1", "a2"),
                edge("b1", "b2"),
                edge("a2", "m"),
                edge("b2", "m"),
            ],
        ),
        &LayoutOptions::default(),
    );
    assert_eq!(place(&out, "a2").row, place(&out, "a1").row);
    assert_eq!(place(&out, "b2").row, place(&out, "b1").row);
    assert_ne!(place(&out, "a1").row, place(&out, "b1").row);
    // m waits on both chains, so it sits halfway between them.
    let mid = (centre(&out, "a2") + centre(&out, "b2")) / 2.0;
    assert!((centre(&out, "m") - mid).abs() < EPS);
    assert_eq!(out.crossings, 0);
}

/// 1 leads to 2, 3 and 4.
#[test]
fn layout_parent_level_with_middle_child_happy() {
    let out = layout(
        &graph(
            vec![node("1"), node("2"), node("3"), node("4")],
            vec![edge("1", "2"), edge("1", "3"), edge("1", "4")],
        ),
        &LayoutOptions::default(),
    );
    assert!((centre(&out, "1") - centre(&out, "3")).abs() < EPS);
    assert!(centre(&out, "2") < centre(&out, "3"));
    assert!(centre(&out, "3") < centre(&out, "4"));
    assert_no_overlaps(&out);
}

/// 1 leads to 2, 3 and 4; 2 leads to 5 and 6; 3 leads to 7.
#[test]
fn layout_parent_centred_on_children_happy() {
    let options = LayoutOptions::default();
    let out = layout(
        &graph(
            ["1", "2", "3", "4", "5", "6", "7"].map(node).to_vec(),
            vec![
                edge("1", "2"),
                edge("1", "3"),
                edge("1", "4"),
                edge("2", "5"),
                edge("2", "6"),
                edge("3", "7"),
            ],
        ),
        &options,
    );
    let mid = (centre(&out, "5") + centre(&out, "6")) / 2.0;
    assert!((centre(&out, "2") - mid).abs() < EPS);
    assert!((centre(&out, "3") - centre(&out, "7")).abs() < EPS);
    assert!((centre(&out, "1") - centre(&out, "3")).abs() < EPS);
    // 2 sits on a half row, between the rows of 5 and 6.
    assert!((centre(&out, "6") - centre(&out, "5") - options.row_pitch).abs() < EPS);
    assert_eq!(place(&out, "2").row, place(&out, "5").row);
    assert_no_overlaps(&out);
    assert_eq!(out.crossings, 0);
}

#[test]
fn layout_crossings_counted_sad() {
    // Every order of two blockers and two dependants that both wait on
    // both crosses once.
    let out = layout(
        &graph(
            vec![node("a"), node("b"), node("c"), node("d")],
            vec![
                edge("a", "c"),
                edge("a", "d"),
                edge("b", "c"),
                edge("b", "d"),
            ],
        ),
        &LayoutOptions::default(),
    );
    assert_eq!(out.crossings, 1);
}

#[test]
fn layout_external_stub_outside_group_happy() {
    let input = GraphInput {
        nodes: vec![in_group("t1", "p"), in_group("t2", "p"), external("x")],
        edges: vec![edge("t1", "t2"), edge("x", "t1")],
        groups: vec![group("p")],
    };
    let out = layout(&input, &LayoutOptions::default());
    let x = place(&out, "x");
    let t1 = place(&out, "t1");
    let b = &out.groups[0];
    assert!(x.stub);
    assert!(!t1.stub);
    assert_eq!(x.column, 0);
    assert_eq!(t1.column, 1);
    assert!(x.y + x.height < b.y);
    assert!(!intersects(
        (x.x, x.y, x.width, x.height),
        (b.x, b.y, b.width, b.height)
    ));
}

#[test]
fn layout_group_box_wraps_members_happy() {
    let input = GraphInput {
        nodes: vec![
            in_group("a", "p"),
            in_group("b", "p"),
            in_group("c", "p"),
            in_group("d", "q"),
            node("e"),
        ],
        edges: vec![edge("a", "b"), edge("a", "c"), edge("c", "d")],
        groups: vec![group("p"), group("q")],
    };
    let options = LayoutOptions::default();
    let out = layout(&input, &options);
    assert_eq!(out.groups.len(), 2);
    let pad = options.group_padding;
    for (members, b) in [
        (&["a", "b", "c"][..], &out.groups[0]),
        (&["d"][..], &out.groups[1]),
    ] {
        for id in members {
            let n = place(&out, id);
            assert!(n.x - b.x >= pad.side - EPS);
            assert!(n.y - b.y >= pad.top - EPS);
            assert!(b.x + b.width - (n.x + n.width) >= pad.side - EPS);
            assert!(b.y + b.height - (n.y + n.height) >= pad.side - EPS);
        }
    }
    let e = place(&out, "e");
    for b in &out.groups {
        assert!(!intersects(
            (e.x, e.y, e.width, e.height),
            (b.x, b.y, b.width, b.height)
        ));
    }
}

#[test]
fn layout_long_edge_reserves_rows_happy() {
    let options = LayoutOptions::default();
    // a → d skips columns 1 and 2, which also hold b, c, p and q.
    let out = layout(
        &graph(
            ["a", "b", "c", "d", "p", "q"].map(node).to_vec(),
            vec![
                edge("a", "b"),
                edge("b", "c"),
                edge("c", "d"),
                edge("a", "p"),
                edge("p", "q"),
                edge("a", "d"),
            ],
        ),
        &options,
    );
    assert_eq!(place(&out, "d").column, 3);
    let skip = route(&out, "a", "d");
    for column in [1_u32, 2] {
        let left = options.group_padding.side + f64::from(column) * options.column_pitch;
        let crossing = skip
            .points
            .windows(2)
            .find(|w| {
                (w[0].y - w[1].y).abs() < EPS
                    && w[0].x.min(w[1].x) < left
                    && w[0].x.max(w[1].x) > left + NodeSize::default().width
            })
            .unwrap_or_else(|| panic!("a → d crosses column {column} in one run"));
        // The run's height is free in that column: no node within half a
        // node of it.
        for n in out.nodes.iter().filter(|n| n.column == column) {
            let gap = (crossing[0].y - (n.y + n.height / 2.0)).abs();
            assert!(gap > n.height / 2.0, "a → d passes behind {}", n.id);
        }
    }
    assert_routes_clear(&out);
}

#[test]
fn layout_stub_long_edge_stays_in_stub_band_happy() {
    let input = GraphInput {
        nodes: vec![
            in_group("t1", "p"),
            in_group("t2", "p"),
            in_group("t3", "p"),
            external("x"),
        ],
        edges: vec![edge("t1", "t2"), edge("t2", "t3"), edge("x", "t3")],
        groups: vec![group("p")],
    };
    let out = layout(&input, &LayoutOptions::default());
    let x = place(&out, "x");
    let b = &out.groups[0];
    assert_eq!((x.column, place(&out, "t3").column), (0, 2));
    assert!(x.y + x.height < b.y);
    // The skipped column is crossed above the box.
    let skip = route(&out, "x", "t3");
    assert!(skip.points[1].y < b.y);
    assert_routes_clear(&out);
}

#[test]
fn layout_band_with_empty_column_sad() {
    // Group p holds columns 0 and 2; its middle column is empty.
    let input = GraphInput {
        nodes: vec![in_group("a", "p"), in_group("b", "q"), in_group("c", "p")],
        edges: vec![edge("a", "b"), edge("b", "c")],
        groups: vec![group("p"), group("q")],
    };
    let out = layout(&input, &LayoutOptions::default());
    assert_eq!(place(&out, "c").column, 2);
    assert_eq!(place(&out, "a").row, place(&out, "c").row);
    assert!(place(&out, "b").row > place(&out, "a").row);
    assert_no_overlaps(&out);
    assert_routes_clear(&out);
}

#[test]
fn order_sweeps_cut_crossings_happy() {
    let graph = seeded_band(7);
    let first = order::initial(&graph);
    let before = order::crossings(&graph, &first, &order::positions(&graph, &first));
    let ordered = order::order(&graph);
    assert_eq!(
        ordered.crossings,
        order::crossings(
            &graph,
            &ordered.layers,
            &order::positions(&graph, &ordered.layers)
        )
    );
    assert_eq!((before, ordered.crossings), (51, 35));
}

#[test]
fn order_keeps_crossing_free_order_sad() {
    // A chain has nothing to reorder; the first order stands.
    let graph = rows::BandGraph {
        layer: vec![0, 1, 2],
        layer_count: 3,
        preds: vec![vec![], vec![0], vec![1]],
        succs: vec![vec![1], vec![2], vec![]],
        dummy: vec![false; 3],
    };
    let ordered = order::order(&graph);
    assert_eq!(ordered.crossings, 0);
    assert_eq!(ordered.layers, vec![vec![0], vec![1], vec![2]]);
}

#[test]
fn layout_cycle_broken_and_reported_sad() {
    let out = layout(
        &graph(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("b", "c"), edge("c", "a")],
        ),
        &LayoutOptions::default(),
    );
    assert_eq!(out.nodes.len(), 3);
    assert_eq!(out.broken_cycles, vec![("c".to_string(), "a".to_string())]);
    assert_eq!(out.edges.len(), 2);
    assert!(!out.edges.iter().any(|e| e.from == "c" && e.to == "a"));
}

#[test]
fn layout_bad_edges_dropped_sad() {
    let nodes = vec![node("a"), node("b")];
    let options = LayoutOptions::default();
    let clean = layout(&graph(nodes.clone(), vec![edge("a", "b")]), &options);
    let out = layout(
        &graph(
            nodes,
            vec![
                edge("a", "b"),
                edge("a", "zz"),
                edge("b", "b"),
                edge("a", "b"),
            ],
        ),
        &options,
    );
    let reasons: Vec<(&str, &str, DropReason)> = out
        .dropped
        .iter()
        .map(|d| (d.from.as_str(), d.to.as_str(), d.reason))
        .collect();
    assert_eq!(
        reasons,
        vec![
            ("a", "zz", DropReason::UnknownNode),
            ("b", "b", DropReason::SelfLoop),
            ("a", "b", DropReason::Duplicate),
        ]
    );
    assert_eq!(out.nodes, clean.nodes);
    assert_eq!(out.edges, clean.edges);
    assert_eq!(out.groups, clean.groups);
    assert!(out.broken_cycles.is_empty());
}

#[test]
fn layout_truncates_above_max_nodes_sad() {
    let ids: Vec<String> = (0..1_600).map(|i| format!("n{i}")).collect();
    let input = graph(
        ids.iter().map(|id| node(id)).collect(),
        ids.windows(2).map(|w| edge(&w[0], &w[1])).collect(),
    );
    let out = layout(&input, &LayoutOptions::default());
    assert_eq!(out.nodes.len(), 1_500);
    assert!(out.truncated);
    assert!(out.dropped.is_empty());
    let kept: std::collections::BTreeSet<&str> = ids[..1_500].iter().map(String::as_str).collect();
    assert!(out
        .edges
        .iter()
        .all(|e| kept.contains(e.from.as_str()) && kept.contains(e.to.as_str())));
    assert_eq!(out.edges.len(), 1_499);
}

#[test]
fn layout_empty_input_happy() {
    let out = layout(&GraphInput::default(), &LayoutOptions::default());
    assert_eq!(out, GraphLayout::default());
    assert_eq!((out.width, out.height), (0.0, 0.0));
}

#[test]
fn layout_readme_example_happy() {
    let task = |id: &str, external: bool| GraphNode {
        id: id.into(),
        group: (!external).then(|| "magnon".to_string()),
        external,
        ..GraphNode::default()
    };
    let waits = |from: &str, to: &str| GraphEdge {
        from: from.into(),
        to: to.into(),
        kind: EdgeKind::Depends,
    };
    let input = GraphInput {
        nodes: vec![
            task("schema", false),
            task("api", false),
            task("ui", false),
            task("auth", true),
        ],
        edges: vec![
            waits("schema", "api"),
            waits("api", "ui"),
            waits("auth", "api"),
        ],
        groups: vec![GraphGroup {
            id: "magnon".into(),
            label: "Magnon".into(),
        }],
    };

    let graph = layout(&input, &LayoutOptions::default());

    let ui = graph
        .nodes
        .iter()
        .find(|n| n.id == "ui")
        .expect("ui is placed");
    assert_eq!(ui.column, 2);

    let auth = graph
        .nodes
        .iter()
        .find(|n| n.id == "auth")
        .expect("auth is placed");
    assert!(auth.stub);
    assert!(auth.y + auth.height < graph.groups[0].y);

    let api = graph
        .nodes
        .iter()
        .find(|n| n.id == "api")
        .expect("api is placed");
    let route = graph
        .edges
        .iter()
        .find(|e| e.from == "auth")
        .expect("auth → api is routed");
    let end = route.points.last().expect("a route has points");
    assert_eq!((end.x, end.y), (api.x, api.y + api.height / 2.0));
}
