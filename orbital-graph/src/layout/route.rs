//! Orthogonal edge routing.

use std::collections::BTreeMap;

use super::rows::Placement;
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

/// One route per edge. An edge runs straight across each column at the
/// position its source, dummies and target give it there, and changes height
/// only in the gutter right of a column.
pub(crate) fn routes(
    nodes: &[&GraphNode],
    edges: &[Edge],
    column: &[u32],
    placement: &Placement,
    grid: &Grid,
) -> Vec<EdgeRoute> {
    let mut gutters: Tracks<u32> = Tracks::default();
    edges
        .iter()
        .zip(&placement.bends)
        .map(|(e, bends)| {
            let (s, t) = (e.from, e.to);
            let (sc, tc) = (column[s], column[t]);
            let (sy, ty) = (placement.centre[s], placement.centre[t]);
            let mut points = vec![Point {
                x: grid.x(sc) + nodes[s].size.width,
                y: grid.centre_at(sy),
            }];
            let mut at = sy;
            // Height in each later column: the dummies, then the target.
            for (c, y) in (sc + 1..).zip(bends.iter().copied().chain([ty])) {
                if y != at {
                    let x = grid.gutter(c - 1, gutters.get(c - 1, s));
                    points.push(Point {
                        x,
                        y: grid.centre_at(at),
                    });
                    points.push(Point {
                        x,
                        y: grid.centre_at(y),
                    });
                    at = y;
                }
            }
            points.push(Point {
                x: grid.x(tc),
                y: grid.centre_at(ty),
            });
            EdgeRoute {
                from: nodes[s].id.clone(),
                to: nodes[t].id.clone(),
                kind: e.kind,
                points,
            }
        })
        .collect()
}
