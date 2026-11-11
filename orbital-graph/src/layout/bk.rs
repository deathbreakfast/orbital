//! Brandes–Köpf coordinate assignment.
//!
//! Follows Brandes & Köpf, "Fast and Simple Horizontal Coordinate Assignment"
//! (2001), with the published erratum, in the paper's four steps:
//!
//! 1. Mark type-1 conflicts: a segment between a real vertex and anything
//!    that crosses an inner segment (dummy to dummy), so long edges win and
//!    stay straight.
//! 2. Build four vertical alignments: each vertex joins a median neighbour
//!    in the previous layer (predecessors walking left to right, successors
//!    walking right to left), scanning each layer from the top or from the
//!    bottom. Aligned vertices form a block that shares one position.
//! 3. Compact each alignment: blocks are packed against each other along a
//!    block graph, then pulled back toward their lower neighbours.
//! 4. Shift the four results to the narrowest one and give each vertex the
//!    mean of its two middle positions.
//!
//! The paper lays layers out top to bottom with positions running left to
//! right; here layers are columns and positions run down the page. All
//! arithmetic is on integer ticks.

use std::collections::BTreeSet;

use super::rows::BandGraph;

/// Positions for every vertex of `graph`, in half-ticks, given the order of
/// each layer. Neighbours in a layer are always at least the sum of their
/// half extents apart.
pub(crate) fn place(graph: &BandGraph, layers: &[Vec<usize>]) -> Vec<i64> {
    let n = graph.dummy.len();
    let conflicts = type1_conflicts(graph, layers);

    // Upper-left, upper-right, lower-left, lower-right in the paper's terms.
    let mut xss: Vec<Vec<i64>> = Vec::with_capacity(4);
    for successors in [false, true] {
        for from_bottom in [false, true] {
            let mut layering = layers.to_vec();
            if successors {
                layering.reverse();
            }
            if from_bottom {
                for layer in &mut layering {
                    layer.reverse();
                }
            }
            let neighbours = if successors {
                &graph.succs
            } else {
                &graph.preds
            };
            let root = vertical_alignment(n, &layering, &conflicts, neighbours);
            let mut xs = horizontal_compaction(graph, &layering, &root);
            if from_bottom {
                for x in &mut xs {
                    *x = -*x;
                }
            }
            xss.push(xs);
        }
    }

    align_to_narrowest(graph, &mut xss);
    let mut out: Vec<i64> = (0..n)
        .map(|v| {
            let mut four = [xss[0][v], xss[1][v], xss[2][v], xss[3][v]];
            four.sort_unstable();
            four[1] + four[2]
        })
        .collect();

    // Balancing can, rarely, bring two neighbours closer than their
    // separation; push the lower one down when it does.
    for layer in layers {
        for pair in layer.windows(2) {
            let min = out[pair[0]] + 2 * (graph.half(pair[0]) + graph.half(pair[1]));
            if out[pair[1]] < min {
                out[pair[1]] = min;
            }
        }
    }
    out
}

/// Segments that cross an inner segment and aren't inner themselves, as
/// `(lower id, higher id)` pairs.
fn type1_conflicts(graph: &BandGraph, layers: &[Vec<usize>]) -> BTreeSet<(usize, usize)> {
    let pos = super::order::positions(graph, layers);
    let mut conflicts = BTreeSet::new();
    for pair in layers.windows(2) {
        let (prev, layer) = (&pair[0], &pair[1]);
        let mut k0 = 0;
        let mut scan = 0;
        for (i, &v) in layer.iter().enumerate() {
            let inner = if graph.dummy[v] {
                graph.preds[v].iter().copied().find(|&u| graph.dummy[u])
            } else {
                None
            };
            let k1 = inner.map_or(prev.len(), |u| pos[u]);
            if inner.is_none() && i + 1 != layer.len() {
                continue;
            }
            for &s in &layer[scan..=i] {
                for &u in &graph.preds[s] {
                    if (pos[u] < k0 || k1 < pos[u]) && !(graph.dummy[u] && graph.dummy[s]) {
                        conflicts.insert(key(u, s));
                    }
                }
            }
            scan = i + 1;
            k0 = k1;
        }
    }
    conflicts
}

fn key(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

/// Aligns each vertex with a median neighbour in the previous layer of
/// `layering`, skipping conflicted segments and any that would cross an
/// alignment already made in the layer (which also stops two vertices
/// taking the same neighbour). Returns each vertex's block root
/// (its first vertex in `layering` order).
fn vertical_alignment(
    n: usize,
    layering: &[Vec<usize>],
    conflicts: &BTreeSet<(usize, usize)>,
    neighbours: &[Vec<usize>],
) -> Vec<usize> {
    let mut root: Vec<usize> = (0..n).collect();
    let mut aligned = vec![false; n];
    let mut pos = vec![0; n];
    for layer in layering {
        for (i, &v) in layer.iter().enumerate() {
            pos[v] = i;
        }
    }
    let mut ws: Vec<usize> = Vec::new();
    for layer in layering {
        let mut last: Option<usize> = None;
        for &v in layer {
            ws.clear();
            ws.extend(&neighbours[v]);
            if ws.is_empty() {
                continue;
            }
            ws.sort_by_key(|&w| pos[w]);
            let len = ws.len();
            for &w in &ws[(len - 1) / 2..=len / 2] {
                if !aligned[v] && last.is_none_or(|p| p < pos[w]) && !conflicts.contains(&key(v, w))
                {
                    root[v] = root[w];
                    aligned[v] = true;
                    last = Some(pos[w]);
                }
            }
        }
    }
    root
}

/// Packs the blocks of one alignment: each block sits as high as the blocks
/// above it allow, then moves down toward the blocks below it as far as
/// they allow. Returns every vertex's position in ticks.
fn horizontal_compaction(graph: &BandGraph, layering: &[Vec<usize>], root: &[usize]) -> Vec<i64> {
    let n = root.len();
    let mut below: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    let mut in_degree = vec![0_usize; n];
    for layer in layering {
        for pair in layer.windows(2) {
            let (u, v) = (pair[0], pair[1]);
            below[root[u]].push((root[v], graph.half(u) + graph.half(v)));
            in_degree[root[v]] += 1;
        }
    }

    // Topological order of the blocks, smallest root first among the ready.
    let mut ready: Vec<usize> = (0..n)
        .rev()
        .filter(|&v| root[v] == v && in_degree[v] == 0)
        .collect();
    let mut topo = Vec::with_capacity(n);
    while let Some(b) = ready.pop() {
        topo.push(b);
        for &(c, _) in &below[b] {
            in_degree[c] -= 1;
            if in_degree[c] == 0 {
                ready.push(c);
            }
        }
    }

    let mut xs = vec![0_i64; n];
    for &b in &topo {
        for &(c, sep) in &below[b] {
            xs[c] = xs[c].max(xs[b] + sep);
        }
    }
    for &b in topo.iter().rev() {
        if let Some(limit) = below[b].iter().map(|&(c, sep)| xs[c] - sep).min() {
            xs[b] = xs[b].max(limit);
        }
    }
    (0..n).map(|v| xs[root[v]]).collect()
}

/// Shifts each alignment onto the narrowest one: the two that scanned from
/// the top share its top edge, the two from the bottom its bottom edge.
fn align_to_narrowest(graph: &BandGraph, xss: &mut [Vec<i64>]) {
    let bounds = |xs: &[i64]| {
        let lo = xs.iter().copied().min().unwrap_or(0);
        let hi = xs.iter().copied().max().unwrap_or(0);
        (lo, hi)
    };
    let width = |xs: &[i64]| {
        let top = xs.iter().enumerate().map(|(v, x)| x - graph.half(v)).min();
        let bottom = xs.iter().enumerate().map(|(v, x)| x + graph.half(v)).max();
        bottom.unwrap_or(0) - top.unwrap_or(0)
    };
    let narrowest = (0..xss.len())
        .min_by_key(|&k| (width(&xss[k]), k))
        .unwrap_or(0);
    let (lo, hi) = bounds(&xss[narrowest]);
    for (k, xs) in xss.iter_mut().enumerate() {
        if k == narrowest {
            continue;
        }
        let (min, max) = bounds(xs);
        let delta = if k % 2 == 0 { lo - min } else { hi - max };
        for x in xs.iter_mut() {
            *x += delta;
        }
    }
}
