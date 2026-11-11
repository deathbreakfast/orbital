//! Crossing reduction inside a band: a depth-first first order, then
//! barycenter sweeps that alternate direction.

use super::rows::BandGraph;

/// Barycenter sweeps to run: left to right, then right to left, and so on.
const SWEEPS: usize = 12;

/// Each layer's vertices top to bottom, and the crossings that order leaves.
pub(crate) struct Ordered {
    pub(crate) layers: Vec<Vec<usize>>,
    pub(crate) crossings: u64,
}

/// Orders each layer of `graph` to cut crossings. Starts from [`initial`],
/// runs [`SWEEPS`] barycenter sweeps and keeps the order with the fewest
/// crossings (the earliest one on a tie).
pub(crate) fn order(graph: &BandGraph) -> Ordered {
    let mut layers = initial(graph);
    let mut pos = positions(graph, &layers);
    let mut best = Ordered {
        crossings: crossings(graph, &layers, &pos),
        layers: layers.clone(),
    };
    for sweep in 0..SWEEPS {
        if best.crossings == 0 {
            break;
        }
        if sweep % 2 == 0 {
            for layer in layers.iter_mut().skip(1) {
                reorder(layer, &graph.preds, &mut pos);
            }
        } else {
            for layer in layers.iter_mut().rev().skip(1) {
                reorder(layer, &graph.succs, &mut pos);
            }
        }
        let count = crossings(graph, &layers, &pos);
        if count < best.crossings {
            best.crossings = count;
            best.layers.clone_from(&layers);
        }
    }
    best
}

/// The first order: a depth-first walk from each vertex with no
/// predecessor, in vertex order (so the band's nodes in input order), that
/// follows edges in input order and appends each vertex to its layer the
/// first time it's reached. Iterative, so a long chain can't overflow the
/// call stack.
pub(crate) fn initial(graph: &BandGraph) -> Vec<Vec<usize>> {
    let n = graph.dummy.len();
    let mut layers = vec![Vec::new(); graph.layer_count];
    let mut seen = vec![false; n];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for root in 0..n {
        // Every vertex has a root ancestor, so only roots start a walk.
        if seen[root] || !graph.preds[root].is_empty() {
            continue;
        }
        seen[root] = true;
        layers[graph.layer[root]].push(root);
        stack.push((root, 0));
        while let Some(&(v, next)) = stack.last() {
            match graph.succs[v].get(next) {
                Some(&w) => {
                    if let Some(top) = stack.last_mut() {
                        top.1 += 1;
                    }
                    if !seen[w] {
                        seen[w] = true;
                        layers[graph.layer[w]].push(w);
                        stack.push((w, 0));
                    }
                }
                None => {
                    stack.pop();
                }
            }
        }
    }
    layers
}

/// Index of each vertex within its layer.
pub(crate) fn positions(graph: &BandGraph, layers: &[Vec<usize>]) -> Vec<usize> {
    let mut pos = vec![0; graph.dummy.len()];
    for layer in layers {
        for (i, &v) in layer.iter().enumerate() {
            pos[v] = i;
        }
    }
    pos
}

/// Sorts `layer` by the mean position of each vertex's `neighbours`.
/// Vertices with no neighbours keep their slots; the rest fill the other
/// slots in barycenter order, ties keeping their current order. Means are
/// compared as exact fractions.
fn reorder(layer: &mut [usize], neighbours: &[Vec<usize>], pos: &mut [usize]) {
    let mut movable: Vec<(usize, u64, u64)> = layer
        .iter()
        .filter(|&&v| !neighbours[v].is_empty())
        .map(|&v| {
            let sum = neighbours[v].iter().map(|&w| pos[w] as u64).sum();
            (v, sum, neighbours[v].len() as u64)
        })
        .collect();
    movable.sort_by(|a, b| (a.1 * b.2).cmp(&(b.1 * a.2)));
    for (slot, (v, _, _)) in layer
        .iter_mut()
        .filter(|v| !neighbours[**v].is_empty())
        .zip(movable)
    {
        *slot = v;
    }
    for (i, &v) in layer.iter().enumerate() {
        pos[v] = i;
    }
}

/// Edge crossings between each pair of neighbouring layers, counted as
/// inversions with a Fenwick tree.
pub(crate) fn crossings(graph: &BandGraph, layers: &[Vec<usize>], pos: &[usize]) -> u64 {
    let mut total = 0;
    let mut targets: Vec<usize> = Vec::new();
    for pair in layers.windows(2) {
        targets.clear();
        for &u in &pair[0] {
            let start = targets.len();
            targets.extend(graph.succs[u].iter().map(|&w| pos[w]));
            targets[start..].sort_unstable();
        }
        // Zero-based Fenwick tree over the next layer's positions.
        let mut tree = vec![0_u64; pair[1].len()];
        for (seen, &t) in targets.iter().enumerate() {
            // Earlier edges that end lower down than `t` cross this one.
            let mut at_or_above = 0;
            let mut i = t + 1;
            while i > 0 {
                at_or_above += tree[i - 1];
                i &= i - 1;
            }
            total += seen as u64 - at_or_above;
            let mut i = t;
            while i < tree.len() {
                tree[i] += 1;
                i |= i + 1;
            }
        }
    }
    total
}
