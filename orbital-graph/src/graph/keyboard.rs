//! Arrow-key navigation between placed nodes. Pure, so it is tested without
//! a browser.

use crate::NodePlacement;

/// Direction of an arrow key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavKey {
    Left,
    Right,
    Up,
    Down,
}

impl NavKey {
    pub(crate) fn from_key(key: &str) -> Option<Self> {
        match key {
            "ArrowLeft" => Some(NavKey::Left),
            "ArrowRight" => Some(NavKey::Right),
            "ArrowUp" => Some(NavKey::Up),
            "ArrowDown" => Some(NavKey::Down),
            _ => None,
        }
    }
}

fn centre_y(node: &NodePlacement) -> f64 {
    node.y + node.height / 2.0
}

/// The node an arrow key moves to from `current`.
///
/// Left and Right go to the nearest column before or after that has nodes,
/// picking the node whose middle is closest vertically. Up and Down go to the
/// closest node above or below in the same column. At an edge, or for an
/// unknown `current`, the focus stays on `current`.
pub(crate) fn next_focus(nodes: &[NodePlacement], current: &str, key: NavKey) -> String {
    let Some(here) = nodes.iter().find(|n| n.id == current) else {
        return current.to_string();
    };
    let y = centre_y(here);
    let closest = |candidates: Vec<&NodePlacement>| -> Option<String> {
        candidates
            .into_iter()
            .min_by(|a, b| {
                (centre_y(a) - y)
                    .abs()
                    .total_cmp(&(centre_y(b) - y).abs())
                    .then(a.y.total_cmp(&b.y))
            })
            .map(|n| n.id.clone())
    };
    let next = match key {
        NavKey::Left | NavKey::Right => {
            let columns = nodes.iter().map(|n| n.column);
            let column = if key == NavKey::Right {
                columns.filter(|&c| c > here.column).min()
            } else {
                columns.filter(|&c| c < here.column).max()
            };
            column.and_then(|column| closest(nodes.iter().filter(|n| n.column == column).collect()))
        }
        NavKey::Up | NavKey::Down => closest(
            nodes
                .iter()
                .filter(|n| n.column == here.column && n.id != here.id)
                .filter(|n| {
                    if key == NavKey::Down {
                        centre_y(n) > y
                    } else {
                        centre_y(n) < y
                    }
                })
                .collect(),
        ),
    };
    next.unwrap_or_else(|| current.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, column: u32, y: f64) -> NodePlacement {
        NodePlacement {
            id: id.into(),
            x: f64::from(column) * 228.0,
            y,
            width: 184.0,
            height: 58.0,
            column,
            row: 0,
            stub: false,
        }
    }

    fn sample() -> Vec<NodePlacement> {
        vec![
            node("a", 0, 0.0),
            node("b", 0, 88.0),
            node("c", 1, 10.0),
            node("d", 1, 180.0),
            node("e", 3, 90.0),
        ]
    }

    #[test]
    fn keyboard_arrow_right_picks_nearest_next_column_happy() {
        let nodes = sample();
        assert_eq!(next_focus(&nodes, "a", NavKey::Right), "c");
        assert_eq!(next_focus(&nodes, "b", NavKey::Right), "c");
        // Column 2 is empty, so Right from column 1 lands in column 3.
        assert_eq!(next_focus(&nodes, "d", NavKey::Right), "e");
        assert_eq!(next_focus(&nodes, "e", NavKey::Left), "c");
        assert_eq!(next_focus(&nodes, "a", NavKey::Down), "b");
        assert_eq!(next_focus(&nodes, "d", NavKey::Up), "c");
    }

    #[test]
    fn keyboard_arrow_at_edge_stays_sad() {
        let nodes = sample();
        assert_eq!(next_focus(&nodes, "a", NavKey::Left), "a");
        assert_eq!(next_focus(&nodes, "e", NavKey::Right), "e");
        assert_eq!(next_focus(&nodes, "a", NavKey::Up), "a");
        assert_eq!(next_focus(&nodes, "missing", NavKey::Down), "missing");
        assert_eq!(next_focus(&[], "a", NavKey::Right), "a");
    }
}
