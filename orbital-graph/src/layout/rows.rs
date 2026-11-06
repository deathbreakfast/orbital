//! Lanes, bands and row placement.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use super::{Edge, GraphNode};

/// Returns each node's row, counted across the whole drawing.
///
/// `group_of` holds the group index of each internal node; `group_count`
/// lanes come first, then the lane for ungrouped internal nodes.
pub(crate) fn assign(
    nodes: &[&GraphNode],
    edges: &[Edge],
    column: &[u32],
    group_of: &[Option<usize>],
    group_count: usize,
    stub_gap_rows: u32,
) -> Vec<u32> {
    let n = nodes.len();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut succs: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges {
        preds[e.to].push(e.from);
        succs[e.from].push(e.to);
    }

    let ungrouped = group_count;
    let internal_lane = |i: usize| group_of[i].unwrap_or(ungrouped);
    let lane: Vec<usize> = (0..n)
        .map(|i| {
            if !nodes[i].external {
                return internal_lane(i);
            }
            succs[i]
                .iter()
                .chain(&preds[i])
                .find(|&&o| !nodes[o].external)
                .map_or(0, |&o| internal_lane(o))
        })
        .collect();

    let mut lanes: Vec<(Vec<usize>, Vec<usize>)> = vec![(Vec::new(), Vec::new()); group_count + 1];
    for i in 0..n {
        let (stubs, own) = &mut lanes[lane[i]];
        if nodes[i].external {
            stubs.push(i);
        } else {
            own.push(i);
        }
    }

    let mut bands = Bands {
        preds: &preds,
        column,
        band: vec![usize::MAX; n],
        row: vec![0; n],
        placed: vec![false; n],
        next_band: 0,
    };
    let mut row0 = 0_u32;
    for (stubs, own) in &lanes {
        if stubs.is_empty() && own.is_empty() {
            continue;
        }
        if !stubs.is_empty() {
            row0 += bands.place(stubs, row0) + stub_gap_rows;
        }
        row0 += bands.place(own, row0).max(1) + 2;
    }
    bands.row
}

struct Bands<'a> {
    preds: &'a [Vec<usize>],
    column: &'a [u32],
    /// Band each node belongs to, `usize::MAX` before it is assigned one.
    band: Vec<usize>,
    /// Global row of each placed node.
    row: Vec<u32>,
    placed: Vec<bool>,
    next_band: usize,
}

impl Bands<'_> {
    /// Places `members` (in input order) in rows from `row0`; returns how many
    /// rows the band uses.
    fn place(&mut self, members: &[usize], row0: u32) -> u32 {
        let band = self.next_band;
        self.next_band += 1;
        for &m in members {
            self.band[m] = band;
        }
        let mut columns: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for &m in members {
            columns.entry(self.column[m]).or_default().push(m);
        }

        let mut rows_used = 0_u32;
        for members in columns.values() {
            // Local rows of each member's placed blockers in this band, in edge order.
            let placed_preds: Vec<Vec<u32>> = members
                .iter()
                .map(|&m| {
                    self.preds[m]
                        .iter()
                        .filter(|&&p| self.band[p] == band && self.placed[p])
                        .map(|&p| self.row[p] - row0)
                        .collect()
                })
                .collect();
            let mut order: Vec<(usize, Option<f64>)> = placed_preds
                .iter()
                .enumerate()
                .map(|(k, rows)| {
                    let mean = (!rows.is_empty()).then(|| {
                        rows.iter().map(|&r| f64::from(r)).sum::<f64>() / rows.len() as f64
                    });
                    (k, mean)
                })
                .collect();
            // Stable, so ties and nodes without placed blockers keep input order.
            order.sort_by(|a, b| match (a.1, b.1) {
                (Some(x), Some(y)) => x.total_cmp(&y),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            });

            let mut taken: Vec<bool> = Vec::new();
            let mut local = vec![0_u32; members.len()];
            let mut later = Vec::new();
            for &(k, _) in &order {
                match placed_preds[k].first() {
                    Some(&want) if !is_taken(&taken, want) => {
                        take(&mut taken, want);
                        local[k] = want;
                    }
                    _ => later.push(k),
                }
            }
            let mut free = 0_u32;
            for k in later {
                while is_taken(&taken, free) {
                    free += 1;
                }
                take(&mut taken, free);
                local[k] = free;
            }
            for (k, &m) in members.iter().enumerate() {
                self.row[m] = row0 + local[k];
                self.placed[m] = true;
                rows_used = rows_used.max(local[k] + 1);
            }
        }
        rows_used
    }
}

fn is_taken(taken: &[bool], row: u32) -> bool {
    taken.get(row as usize).copied().unwrap_or(false)
}

fn take(taken: &mut Vec<bool>, row: u32) {
    let row = row as usize;
    if taken.len() <= row {
        taken.resize(row + 1, false);
    }
    taken[row] = true;
}
