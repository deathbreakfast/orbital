//! Property checks for `orbital_graph::layout` over seeded random graphs.
//!
//! The generator is a small xorshift so the workspace gains no dependency;
//! every seed is fixed, so a failure reproduces exactly.

use std::collections::BTreeMap;
use std::time::Instant;

use orbital_graph::{
    layout, EdgeKind, GraphEdge, GraphGroup, GraphInput, GraphLayout, GraphNode, LayoutOptions,
    NodeSize,
};

const EPS: f64 = 1e-6;

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Uniform in `lo..=hi`.
    fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next() % (hi - lo + 1) as u64) as usize
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}

/// A random DAG: edges only run from a lower index to a higher one.
fn random_dag(seed: u64, nodes: usize, edges: usize, groups: usize) -> GraphInput {
    let mut rng = Rng::new(seed);
    let groups: Vec<GraphGroup> = (0..groups)
        .map(|g| GraphGroup {
            id: format!("g{g}"),
            label: format!("Group {g}"),
        })
        .collect();
    let nodes: Vec<GraphNode> = (0..nodes)
        .map(|i| {
            let external = rng.chance(10);
            let group = if groups.is_empty() || rng.chance(15) {
                None
            } else {
                Some(format!("g{}", rng.range(0, groups.len() - 1)))
            };
            let size = if rng.chance(20) {
                NodeSize {
                    width: rng.range(80, 184) as f64,
                    height: rng.range(24, 58) as f64,
                }
            } else {
                NodeSize::default()
            };
            GraphNode {
                id: format!("n{i}"),
                group,
                external,
                size,
            }
        })
        .collect();
    let edges = (0..edges)
        .filter_map(|_| {
            let a = rng.range(0, nodes.len() - 1);
            let b = rng.range(0, nodes.len() - 1);
            (a != b).then(|| GraphEdge {
                from: nodes[a.min(b)].id.clone(),
                to: nodes[a.max(b)].id.clone(),
                kind: if rng.chance(10) {
                    EdgeKind::Child
                } else {
                    EdgeKind::Depends
                },
            })
        })
        .collect();
    GraphInput {
        nodes,
        edges,
        groups,
    }
}

fn seeded_graphs() -> impl Iterator<Item = (u64, GraphInput)> {
    (1..=200_u64).map(|seed| {
        let mut rng = Rng::new(seed ^ 0xA5A5);
        let nodes = rng.range(10, 300);
        let edges = rng.range(0, nodes * 2);
        let groups = rng.range(0, 3);
        (seed, random_dag(seed, nodes, edges, groups))
    })
}

type Rect = (f64, f64, f64, f64);

fn overlaps(a: Rect, b: Rect) -> bool {
    a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
}

fn rects(out: &GraphLayout) -> Vec<Rect> {
    out.nodes
        .iter()
        .map(|n| (n.x, n.y, n.width, n.height))
        .collect()
}

#[test]
fn props_no_node_overlaps_happy() {
    let options = LayoutOptions::default();
    for (seed, input) in seeded_graphs() {
        let out = layout(&input, &options);
        assert_eq!(out.nodes.len(), input.nodes.len(), "seed {seed}");
        let rects = rects(&out);
        for i in 0..rects.len() {
            for j in i + 1..rects.len() {
                assert!(
                    !overlaps(rects[i], rects[j]),
                    "seed {seed}: {} overlaps {}",
                    out.nodes[i].id,
                    out.nodes[j].id
                );
            }
        }
        for (n, r) in out.nodes.iter().zip(&rects).filter(|(n, _)| n.stub) {
            for b in &out.groups {
                assert!(
                    !overlaps(*r, (b.x, b.y, b.width, b.height)),
                    "seed {seed}: stub {} inside box {}",
                    n.id,
                    b.id
                );
            }
        }
    }
}

/// True when the open segment `a`–`b` passes through the open rect.
fn segment_crosses(a: (f64, f64), b: (f64, f64), r: Rect) -> bool {
    let (x0, x1) = (a.0.min(b.0), a.0.max(b.0));
    let (y0, y1) = (a.1.min(b.1), a.1.max(b.1));
    let inside_x = |x: f64| x > r.0 + EPS && x < r.0 + r.2 - EPS;
    let inside_y = |y: f64| y > r.1 + EPS && y < r.1 + r.3 - EPS;
    if (y1 - y0).abs() < EPS {
        inside_y(y0) && x0 < r.0 + r.2 - EPS && x1 > r.0 + EPS
    } else {
        inside_x(x0) && y0 < r.1 + r.3 - EPS && y1 > r.1 + EPS
    }
}

#[test]
fn props_routes_are_orthogonal_and_clear_happy() {
    let options = LayoutOptions::default();
    for (seed, input) in seeded_graphs() {
        let out = layout(&input, &options);
        let by_id: BTreeMap<&str, usize> = out
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id.as_str(), i))
            .collect();
        let rects = rects(&out);
        for route in &out.edges {
            let s = &out.nodes[by_id[route.from.as_str()]];
            let t = &out.nodes[by_id[route.to.as_str()]];
            let first = route.points[0];
            let last = route.points[route.points.len() - 1];
            assert!((first.x - (s.x + s.width)).abs() < EPS, "seed {seed}");
            assert!(
                (first.y - (s.y + s.height / 2.0)).abs() < EPS,
                "seed {seed}"
            );
            assert!((last.x - t.x).abs() < EPS, "seed {seed}");
            assert!((last.y - (t.y + t.height / 2.0)).abs() < EPS, "seed {seed}");
            for w in route.points.windows(2) {
                let (a, b) = ((w[0].x, w[0].y), (w[1].x, w[1].y));
                assert!(
                    (a.0 - b.0).abs() < EPS || (a.1 - b.1).abs() < EPS,
                    "seed {seed}: {} → {} has a diagonal segment",
                    route.from,
                    route.to
                );
                for (n, r) in out.nodes.iter().zip(&rects) {
                    if n.id == route.from || n.id == route.to {
                        continue;
                    }
                    assert!(
                        !segment_crosses(a, b, *r),
                        "seed {seed}: {} → {} crosses {}",
                        route.from,
                        route.to,
                        n.id
                    );
                }
            }
        }
    }
}

#[test]
fn props_layout_1500_nodes_time_bound_happy() {
    let mut input = random_dag(42, 1_500, 3_400, 3);
    input.edges.truncate(3_000);
    assert_eq!(input.edges.len(), 3_000);
    let start = Instant::now();
    let out = layout(&input, &LayoutOptions::default());
    let elapsed = start.elapsed();
    assert_eq!(out.nodes.len(), 1_500);
    assert!(elapsed.as_secs_f64() < 1.0, "1,500 nodes took {elapsed:?}");
}
