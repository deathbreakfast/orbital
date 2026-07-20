use crate::types::{CellCoord, CellSelection, NormalizedCellRange};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellMoveDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Start a new selection at the given cell.
pub fn set_anchor(selection: &mut CellSelection, coord: CellCoord) {
    selection.anchor = Some(coord.clone());
    selection.focus = Some(coord);
}

/// Extend or replace selection to the target cell.
pub fn extend_to(selection: &mut CellSelection, coord: CellCoord, extend: bool) {
    if extend {
        if selection.anchor.is_none() {
            selection.anchor = Some(coord.clone());
        }
        selection.focus = Some(coord);
    } else {
        set_anchor(selection, coord);
    }
}

/// Move focus by arrow key; shift extends from anchor.
pub fn move_focus(
    selection: &mut CellSelection,
    direction: CellMoveDirection,
    row_ids: &[String],
    fields: &[String],
    extend: bool,
) {
    let Some(focus) = selection.focus.clone() else {
        if let (Some(row_id), Some(field)) = (row_ids.first(), fields.first()) {
            set_anchor(selection, CellCoord::new(row_id.clone(), field.clone()));
        }
        return;
    };

    let Some(row_idx) = row_ids.iter().position(|id| id == &focus.row_id) else {
        return;
    };
    let Some(col_idx) = fields.iter().position(|f| f == &focus.field) else {
        return;
    };

    let (new_row, new_col) = match direction {
        CellMoveDirection::Up => (row_idx.saturating_sub(1), col_idx),
        CellMoveDirection::Down => ((row_idx + 1).min(row_ids.len().saturating_sub(1)), col_idx),
        CellMoveDirection::Left => (row_idx, col_idx.saturating_sub(1)),
        CellMoveDirection::Right => (row_idx, (col_idx + 1).min(fields.len().saturating_sub(1))),
    };

    let coord = CellCoord::new(row_ids[new_row].clone(), fields[new_col].clone());

    if extend {
        if selection.anchor.is_none() {
            selection.anchor = Some(focus);
        }
        selection.focus = Some(coord);
    } else {
        set_anchor(selection, coord);
    }
}

/// Enumerate all cell coordinates in the current selection.
pub fn coords_in_range(
    selection: &CellSelection,
    row_ids: &[String],
    fields: &[String],
) -> Vec<CellCoord> {
    let Some(range) = selection.normalized(row_ids, fields) else {
        return Vec::new();
    };
    let mut coords = Vec::new();
    for row_id in &row_ids[range.row_start..=range.row_end] {
        for field in &fields[range.col_start..=range.col_end] {
            coords.push(CellCoord::new(row_id.clone(), field.clone()));
        }
    }
    coords
}

/// Resolve the normalized range for export/clipboard.
pub fn normalized_range(
    selection: &CellSelection,
    row_ids: &[String],
    fields: &[String],
) -> Option<NormalizedCellRange> {
    selection.normalized(row_ids, fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> (Vec<String>, Vec<String>) {
        (
            vec!["1".into(), "2".into(), "3".into()],
            vec!["name".into(), "role".into()],
        )
    }

    #[test]
    fn set_anchor_sets_both() {
        let mut sel = CellSelection::default();
        set_anchor(&mut sel, CellCoord::new("1", "name"));
        assert_eq!(sel.anchor.as_ref().unwrap().row_id, "1");
        assert_eq!(sel.focus.as_ref().unwrap().field, "name");
    }

    #[test]
    fn extend_to_keeps_anchor_when_extending() {
        let mut sel = CellSelection::default();
        set_anchor(&mut sel, CellCoord::new("1", "name"));
        extend_to(&mut sel, CellCoord::new("3", "role"), true);
        assert_eq!(sel.anchor.as_ref().unwrap().row_id, "1");
        assert_eq!(sel.focus.as_ref().unwrap().row_id, "3");
    }

    #[test]
    fn coords_in_range_rectangular() {
        let (rows, fields) = grid();
        let mut sel = CellSelection::default();
        set_anchor(&mut sel, CellCoord::new("1", "name"));
        extend_to(&mut sel, CellCoord::new("2", "role"), true);
        let coords = coords_in_range(&sel, &rows, &fields);
        assert_eq!(coords.len(), 4);
    }

    #[test]
    fn move_focus_down() {
        let (rows, fields) = grid();
        let mut sel = CellSelection::default();
        set_anchor(&mut sel, CellCoord::new("1", "name"));
        move_focus(&mut sel, CellMoveDirection::Down, &rows, &fields, false);
        assert_eq!(sel.focus.as_ref().unwrap().row_id, "2");
    }

    #[test]
    fn move_focus_right() {
        let (rows, fields) = grid();
        let mut sel = CellSelection::default();
        set_anchor(&mut sel, CellCoord::new("1", "name"));
        move_focus(&mut sel, CellMoveDirection::Right, &rows, &fields, false);
        assert_eq!(sel.focus.as_ref().unwrap().field, "role");
    }
}
