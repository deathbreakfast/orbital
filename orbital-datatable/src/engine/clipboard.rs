use crate::engine::{export_cell_text, resolve_value};
use crate::types::{
    CellCoord, CellSelection, DataTableColumnDef, DataTableRowModel, NormalizedCellRange,
};

/// Format a cell range as tab-separated values (Excel/Sheets compatible).
pub fn range_to_tsv(
    rows: &[DataTableRowModel],
    columns: &[DataTableColumnDef],
    row_ids: &[String],
    fields: &[String],
    range: NormalizedCellRange,
) -> String {
    let mut lines = Vec::new();
    for row_id in &row_ids[range.row_start..=range.row_end] {
        let Some(row) = rows.iter().find(|r| r.id() == row_id) else {
            continue;
        };
        let cells: Vec<String> = (range.col_start..=range.col_end)
            .filter_map(|col_idx| fields.get(col_idx))
            .filter_map(|field| columns.iter().find(|c| &c.field == field))
            .map(|col| export_cell_text(col, row))
            .collect();
        lines.push(cells.join("\t"));
    }
    lines.join("\n")
}

/// Format selection as TSV using normalized range.
pub fn selection_to_tsv(
    rows: &[DataTableRowModel],
    columns: &[DataTableColumnDef],
    row_ids: &[String],
    fields: &[String],
    selection: &CellSelection,
) -> Option<String> {
    let range = selection.normalized(row_ids, fields)?;
    Some(range_to_tsv(rows, columns, row_ids, fields, range))
}

/// Parse clipboard TSV/grid text into rows of cell strings.
pub fn parse_tsv(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(|cell| cell.to_string()).collect())
        .collect()
}

/// Map a parsed paste grid onto cell coordinates starting at origin.
pub fn paste_grid_coords(
    origin: &CellCoord,
    grid: &[Vec<String>],
    row_ids: &[String],
    fields: &[String],
) -> Vec<(CellCoord, String)> {
    let Some(origin_row) = row_ids.iter().position(|id| id == &origin.row_id) else {
        return Vec::new();
    };
    let Some(origin_col) = fields.iter().position(|f| f == &origin.field) else {
        return Vec::new();
    };

    let mut result = Vec::new();
    for (r_offset, row_cells) in grid.iter().enumerate() {
        let row_idx = origin_row + r_offset;
        if row_idx >= row_ids.len() {
            break;
        }
        for (c_offset, value) in row_cells.iter().enumerate() {
            let col_idx = origin_col + c_offset;
            if col_idx >= fields.len() {
                break;
            }
            result.push((
                CellCoord::new(row_ids[row_idx].clone(), fields[col_idx].clone()),
                value.clone(),
            ));
        }
    }
    result
}

/// Resolve cell display text for copy (single cell).
pub fn cell_text(row: &DataTableRowModel, column: &DataTableColumnDef) -> String {
    export_cell_text(column, row)
}

/// Resolve raw value for paste target.
pub fn cell_value(row: &DataTableRowModel, column: &DataTableColumnDef) -> orbital_data::DataValue {
    resolve_value(column, row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_tsv_splits_rows_and_cols() {
        let grid = parse_tsv("a\tb\nc\td");
        assert_eq!(grid, vec![vec!["a", "b"], vec!["c", "d"]]);
    }

    #[test]
    fn range_to_tsv_formats_block() {
        let cols = vec![
            DataTableColumnDef::new("name", "Name"),
            DataTableColumnDef::new("role", "Role"),
        ];
        let rows = vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([
                    ("name".into(), "Ada".into()),
                    ("role".into(), "Admin".into()),
                ]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([
                    ("name".into(), "Bob".into()),
                    ("role".into(), "Member".into()),
                ]),
            ),
        ];
        let row_ids = vec!["1".into(), "2".into()];
        let fields = vec!["name".into(), "role".into()];
        let range = NormalizedCellRange {
            row_start: 0,
            row_end: 1,
            col_start: 0,
            col_end: 1,
        };
        let tsv = range_to_tsv(&rows, &cols, &row_ids, &fields, range);
        assert_eq!(tsv, "Ada\tAdmin\nBob\tMember");
    }
}
