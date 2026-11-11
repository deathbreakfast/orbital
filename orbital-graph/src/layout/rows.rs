//! Lanes, bands and row placement.
//!
//! Each band is placed on its own. A long edge gets a dummy vertex in every
//! column it skips, [`order`](super::order) sorts each column to cut
//! crossings, and [`bk`](super::bk) picks positions. Positions are integer
//! ticks, so the result is the same on every run and every target.

use super::{bk, order, Edge, GraphNode};

/// Ticks per row. Positions inside a band are counted in ticks.
pub(crate) const TICKS_PER_ROW: i64 = 48;
/// Half the extent of a real node, in ticks: two real nodes sit one row apart.
pub(crate) const REAL_HALF: i64 = TICKS_PER_ROW / 2;
/// Half the extent of a dummy, in ticks: two dummies sit an eighth of a row
/// apart (11 px at the default row pitch), and a dummy sits 27 ticks from a
/// real node.
pub(crate) const DUMMY_HALF: i64 = 3;
/// Half-ticks per row, the unit of [`Placement::centre`]. Balancing averages
/// two tick positions, which can land on a half tick.
pub(crate) const HALF_TICKS_PER_ROW: i64 = 2 * TICKS_PER_ROW;

/// Where [`assign`] put every node and long edge.
pub(crate) struct Placement {
    /// Centre of each node, in half-ticks below the centre of row 0.
    pub(crate) centre: Vec<i64>,
    /// Row nearest each node's centre; a node halfway between two rows takes
    /// the upper one.
    pub(crate) row: Vec<u32>,
    /// For each edge, the centre of its dummy in each column it skips, left
    /// to right, in half-ticks. Empty for an edge to the next column.
    pub(crate) bends: Vec<Vec<i64>>,
    /// Crossings left between neighbouring columns, summed over bands.
    pub(crate) crossings: u64,
}

/// One band as a layered graph: the band's nodes first (in input order),
/// then the dummies of its long edges. Every edge joins neighbouring layers.
pub(crate) struct BandGraph {
    /// Layer of each vertex, counted from the band's leftmost column.
    pub(crate) layer: Vec<usize>,
    /// How many layers the band spans.
    pub(crate) layer_count: usize,
    /// Vertices in the previous layer joined to each vertex, in edge order.
    pub(crate) preds: Vec<Vec<usize>>,
    /// Vertices in the next layer joined to each vertex, in edge order.
    pub(crate) succs: Vec<Vec<usize>>,
    /// The vertex is a dummy on a long edge.
    pub(crate) dummy: Vec<bool>,
}

impl BandGraph {
    /// Half a vertex's extent, in ticks.
    pub(crate) fn half(&self, v: usize) -> i64 {
        if self.dummy[v] {
            DUMMY_HALF
        } else {
            REAL_HALF
        }
    }
}

/// Places every node and long edge, band by band from the top of the drawing.
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
) -> Placement {
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

    // Bands top to bottom: members, fewest rows, empty rows after.
    let mut bands: Vec<(Vec<usize>, i64, i64)> = Vec::new();
    for (stubs, own) in lanes {
        if stubs.is_empty() && own.is_empty() {
            continue;
        }
        if !stubs.is_empty() {
            bands.push((stubs, 0, i64::from(stub_gap_rows)));
        }
        bands.push((own, 1, 2));
    }
    let mut band_of = vec![0; n];
    for (b, (members, _, _)) in bands.iter().enumerate() {
        for &m in members {
            band_of[m] = b;
        }
    }

    let mut out = Placement {
        centre: vec![0; n],
        row: vec![0; n],
        bends: vec![Vec::new(); edges.len()],
        crossings: 0,
    };
    let mut row0 = 0_i64;
    for (b, (members, min_rows, gap)) in bands.iter().enumerate() {
        let rows = place_band(b, members, edges, column, &band_of, row0, &mut out);
        row0 += rows.max(*min_rows) + gap;
    }
    out
}

/// Places band `band` with its top at row `row0`; returns how many rows it
/// uses. Long edges leaving the band's nodes get their dummies here, even
/// when the edge ends in another band.
fn place_band(
    band: usize,
    members: &[usize],
    edges: &[Edge],
    column: &[u32],
    band_of: &[usize],
    row0: i64,
    out: &mut Placement,
) -> i64 {
    if members.is_empty() {
        return 0;
    }
    let mut local = vec![usize::MAX; band_of.len()];
    for (k, &m) in members.iter().enumerate() {
        local[m] = k;
    }
    let mut col: Vec<u32> = members.iter().map(|&m| column[m]).collect();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); members.len()];
    let mut succs: Vec<Vec<usize>> = vec![Vec::new(); members.len()];
    // (edge, first dummy) for each long edge leaving this band.
    let mut chains: Vec<(usize, usize)> = Vec::new();
    for (i, e) in edges.iter().enumerate() {
        if band_of[e.from] != band {
            continue;
        }
        let (sc, tc) = (column[e.from], column[e.to]);
        let mut prev = local[e.from];
        if tc > sc + 1 {
            chains.push((i, col.len()));
        }
        for c in sc + 1..tc {
            let d = col.len();
            col.push(c);
            preds.push(vec![prev]);
            succs.push(Vec::new());
            succs[prev].push(d);
            prev = d;
        }
        if band_of[e.to] == band {
            let t = local[e.to];
            succs[prev].push(t);
            preds[t].push(prev);
        }
    }

    let first = col.iter().copied().min().unwrap_or(0);
    let last = col.iter().copied().max().unwrap_or(0);
    let mut dummy = vec![false; col.len()];
    dummy[members.len()..].fill(true);
    let graph = BandGraph {
        layer: col.iter().map(|&c| (c - first) as usize).collect(),
        layer_count: (last - first) as usize + 1,
        preds,
        succs,
        dummy,
    };
    let ordered = order::order(&graph);
    let pos = bk::place(&graph, &ordered.layers);
    out.crossings += ordered.crossings;

    // Shift so the band's top edge is the top of row `row0`.
    let half = |v: usize| 2 * graph.half(v);
    let top = (0..col.len()).map(|v| pos[v] - half(v)).min().unwrap_or(0);
    let bottom = (0..col.len()).map(|v| pos[v] + half(v)).max().unwrap_or(0);
    let shift = row0 * HALF_TICKS_PER_ROW - 2 * REAL_HALF - top;
    for (k, &m) in members.iter().enumerate() {
        let centre = pos[k] + shift;
        out.centre[m] = centre;
        out.row[m] = nearest_row(centre);
    }
    for &(i, d) in &chains {
        let e = edges[i];
        let skipped = (column[e.to] - column[e.from] - 1) as usize;
        out.bends[i] = pos[d..d + skipped].iter().map(|p| p + shift).collect();
    }
    let extent = bottom - top;
    (extent + HALF_TICKS_PER_ROW - 1) / HALF_TICKS_PER_ROW
}

/// Row nearest `centre` (half-ticks); halfway rounds up the page.
fn nearest_row(centre: i64) -> u32 {
    let row = (centre + HALF_TICKS_PER_ROW / 2 - 1).div_euclid(HALF_TICKS_PER_ROW);
    u32::try_from(row).unwrap_or(0)
}
