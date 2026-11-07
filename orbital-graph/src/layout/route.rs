//! Orthogonal edge routing.

use std::collections::BTreeMap;

use super::{Edge, EdgeRoute, GraphNode, Grid, Point};

/// Hands out tracks: the first owner seen under a key gets track 0, the next
/// track 1, and so on.
#[derive(Default)]
struct Tracks<K> {
    assigned: BTreeMap<(K, usize), usize>,
    count: BTreeMap<K, usize>,
}

impl<K: Ord + Copy> Tracks<K> {
    fn get(&mut self, key: K, owner: usize) -> usize {
        let count = self.count.entry(key).or_insert(0);
        *self.assigned.entry((key, owner)).or_insert_with(|| {
            *count += 1;
            *count - 1
        })
    }
}

pub(crate) fn routes(
    nodes: &[&GraphNode],
    edges: &[Edge],
    column: &[u32],
    row: &[u32],
    grid: &Grid,
) -> Vec<EdgeRoute> {
    let mut gutters: Tracks<u32> = Tracks::default();
    let mut channels: Tracks<i64> = Tracks::default();
    edges
        .iter()
        .map(|e| {
            let (s, t) = (e.from, e.to);
            let (sc, tc) = (column[s], column[t]);
            let (sy, ty) = (grid.centre(row[s]), grid.centre(row[t]));
            let start = Point {
                x: grid.x(sc) + nodes[s].size.width,
                y: sy,
            };
            let end = Point {
                x: grid.x(tc),
                y: ty,
            };
            let x1 = grid.gutter(sc, gutters.get(sc, s));
            let points = if tc == sc + 1 {
                if row[s] == row[t] {
                    vec![start, end]
                } else {
                    vec![start, Point { x: x1, y: sy }, Point { x: x1, y: ty }, end]
                }
            } else {
                // Skip edge: cross the skipped columns in the channel between
                // the source's row and its neighbour towards the target.
                let r = i64::from(row[s]);
                let half = if row[t] >= row[s] {
                    2 * r + 1
                } else {
                    2 * r - 1
                };
                let track = channels.get(half, s);
                let cy = grid.half_row(half) + ((track % 3) as f64 - 1.0) * 6.0;
                let x2 = grid.gutter(tc - 1, gutters.get(tc - 1, s));
                vec![
                    start,
                    Point { x: x1, y: sy },
                    Point { x: x1, y: cy },
                    Point { x: x2, y: cy },
                    Point { x: x2, y: ty },
                    end,
                ]
            };
            EdgeRoute {
                from: nodes[s].id.clone(),
                to: nodes[t].id.clone(),
                kind: e.kind,
                points,
            }
        })
        .collect()
}
