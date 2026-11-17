mod parts;
mod styles;
pub(crate) use styles::table_styles;
mod table;
mod types;

pub use parts::{TableBody, TableCell, TableCellLayout, TableHeader, TableHeaderCell, TableRow};
pub use table::Table;
pub use types::{TableCellLayoutConfig, TableHeaderCellConfig};

#[cfg(feature = "preview")]
pub use table::{TABLE_DESCRIPTION, TABLE_DOC, TABLE_PREVIEW_REGISTRATION, TABLE_PROPS};
