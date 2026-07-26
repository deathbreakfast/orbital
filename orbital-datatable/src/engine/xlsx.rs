use std::io::{Cursor, Write};

use orbital_data::{DataValue, Dataset};

use crate::engine::format_display;
use crate::types::{DataTableColumnDef, DataTableError};

/// Serialize a dataset to a minimal xlsx file (OpenXML zip).
pub fn serialize_xlsx(
    dataset: &Dataset,
    columns: &[DataTableColumnDef],
) -> Result<Vec<u8>, DataTableError> {
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let mut shared_strings: Vec<String> = Vec::new();
    let mut string_index = |s: &str| -> usize {
        if let Some(idx) = shared_strings.iter().position(|existing| existing == s) {
            idx
        } else {
            shared_strings.push(s.to_string());
            shared_strings.len() - 1
        }
    };

    let mut sheet_rows = String::new();
    sheet_rows.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    sheet_rows.push_str(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>"#);

    // Header row
    sheet_rows.push_str("<row r=\"1\">");
    for (col_idx, col) in columns.iter().enumerate() {
        let cell_ref = format!("{}1", col_letter(col_idx));
        let idx = string_index(&col.header_name);
        sheet_rows.push_str(&format!(r#"<c r="{cell_ref}" t="s"><v>{idx}</v></c>"#));
    }
    sheet_rows.push_str("</row>");

    for (row_idx, record) in dataset.records.iter().enumerate() {
        let r = row_idx + 2;
        sheet_rows.push_str(&format!("<row r=\"{r}\">"));
        for (col_idx, col) in columns.iter().enumerate() {
            let cell_ref = format!("{}{r}", col_letter(col_idx));
            let value = record
                .values
                .get(&col.field)
                .cloned()
                .unwrap_or(DataValue::Null);
            match &value {
                DataValue::Number(n) => {
                    sheet_rows.push_str(&format!(r#"<c r="{cell_ref}"><v>{n}</v></c>"#));
                }
                _ => {
                    let text = format_display(col, &value);
                    let idx = string_index(&text);
                    sheet_rows.push_str(&format!(r#"<c r="{cell_ref}" t="s"><v>{idx}</v></c>"#));
                }
            }
        }
        sheet_rows.push_str("</row>");
    }
    sheet_rows.push_str("</sheetData></worksheet>");

    let mut sst = String::new();
    sst.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    sst.push_str(&format!(
        r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="{}" uniqueCount="{}">"#,
        shared_strings.len(),
        shared_strings.len()
    ));
    for s in &shared_strings {
        sst.push_str(&format!("<si><t>{}</t></si>", xml_escape(s)));
    }
    sst.push_str("</sst>");

    let cursor = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default();

    zip.start_file("[Content_Types].xml", options)
        .map_err(export_err)?;
    zip.write_all(CONTENT_TYPES.as_bytes())
        .map_err(export_err)?;

    zip.start_file("_rels/.rels", options).map_err(export_err)?;
    zip.write_all(RELS.as_bytes()).map_err(export_err)?;

    zip.start_file("xl/workbook.xml", options)
        .map_err(export_err)?;
    zip.write_all(WORKBOOK.as_bytes()).map_err(export_err)?;

    zip.start_file("xl/_rels/workbook.xml.rels", options)
        .map_err(export_err)?;
    zip.write_all(WORKBOOK_RELS.as_bytes())
        .map_err(export_err)?;

    zip.start_file("xl/worksheets/sheet1.xml", options)
        .map_err(export_err)?;
    zip.write_all(sheet_rows.as_bytes()).map_err(export_err)?;

    zip.start_file("xl/sharedStrings.xml", options)
        .map_err(export_err)?;
    zip.write_all(sst.as_bytes()).map_err(export_err)?;

    let cursor = zip.finish().map_err(export_err)?;
    Ok(cursor.into_inner())
}

fn export_err(e: impl ToString) -> DataTableError {
    DataTableError::ExportFailed(e.to_string())
}

fn col_letter(idx: usize) -> String {
    let mut n = idx + 1;
    let mut s = String::new();
    while n > 0 {
        n -= 1;
        s.insert(0, (b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    s
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
<Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
<Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/>
</Types>"#;

const RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"#;

const WORKBOOK: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<sheets><sheet name="Export" sheetId="1" r:id="rId1"/></sheets>
</workbook>"#;

const WORKBOOK_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" Target="sharedStrings.xml"/>
</Relationships>"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::engine::build_export_dataset;
    use crate::types::DataTableRowModel;

    #[test]
    fn xlsx_starts_with_zip_magic() {
        let cols = vec![DataTableColumnDef::new("name", "Name")];
        let rows = vec![DataTableRowModel::from_text_cells(
            "1",
            HashMap::from([("name".into(), "Ada".into())]),
        )];
        let dataset = build_export_dataset(&rows, &cols);
        let bytes = serialize_xlsx(&dataset, &cols).expect("xlsx");
        assert!(bytes.starts_with(b"PK\x03\x04"));
    }
}
