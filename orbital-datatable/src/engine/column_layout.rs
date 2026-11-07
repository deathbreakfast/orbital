use std::collections::HashMap;

use crate::types::{
    ColumnWidth, DataTableColumnDef, DataTableFeatures, PinSide, PinnedColumnsState,
};

/// Pin metadata for a resolved column.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnPinMeta {
    pub side: PinSide,
    pub offset_px: f64,
    pub z_index: u32,
    pub is_last_left: bool,
    pub is_first_right: bool,
}

/// A column ready for rendering with resolved width and pin offsets.
#[derive(Clone, Debug)]
pub struct ResolvedColumn {
    pub def: DataTableColumnDef,
    pub width_px: f64,
    pub pin: Option<ColumnPinMeta>,
}

/// Ordered visible columns with layout metadata.
#[derive(Clone, Debug, Default)]
pub struct ColumnLayout {
    pub columns: Vec<ResolvedColumn>,
}

/// Selection checkbox column width when present.
pub const SELECTION_COLUMN_WIDTH_PX: f64 = 40.0;

/// Table width used for flex distribution until the container has been measured
/// (server render and first paint).
const DEFAULT_TABLE_WIDTH_PX: f64 = 800.0;

pub struct ColumnLayoutInput<'a> {
    pub defs: &'a [DataTableColumnDef],
    pub column_order: &'a [String],
    pub column_visibility: &'a HashMap<String, bool>,
    pub column_widths: &'a HashMap<String, f64>,
    pub pinned_columns: &'a PinnedColumnsState,
    pub features: DataTableFeatures,
    pub table_width_px: Option<f64>,
}

fn is_visible(field: &str, visibility: &HashMap<String, bool>) -> bool {
    !matches!(visibility.get(field), Some(false))
}

fn resolve_pin_side(
    field: &str,
    def: &DataTableColumnDef,
    pinned_columns: &PinnedColumnsState,
) -> Option<PinSide> {
    if pinned_columns.left.iter().any(|f| f == field) {
        return Some(PinSide::Left);
    }
    if pinned_columns.right.iter().any(|f| f == field) {
        return Some(PinSide::Right);
    }
    def.pinned
}

fn base_width(def: &DataTableColumnDef, override_width: Option<f64>) -> f64 {
    if let Some(w) = override_width {
        return w;
    }
    match def.width {
        ColumnWidth::Fixed(w) => w,
        ColumnWidth::Flex(_) => def.min_width.unwrap_or(100.0),
        ColumnWidth::Auto => def.min_width.unwrap_or(50.0),
    }
}

fn resolve_widths(
    defs: &[DataTableColumnDef],
    column_widths: &HashMap<String, f64>,
    table_width_px: f64,
) -> HashMap<String, f64> {
    let mut widths = HashMap::new();
    // Flex columns without a user override: (field, weight).
    let mut flex: Vec<(&DataTableColumnDef, f64)> = Vec::new();
    let mut fixed_total = 0.0;

    for def in defs {
        let override_w = column_widths.get(&def.field).copied();
        match def.width {
            ColumnWidth::Flex(ratio) if override_w.is_none() => {
                flex.push((def, f64::from(ratio)));
            }
            _ => {
                let w = clamp_width(base_width(def, override_w), def.min_width, def.max_width);
                fixed_total += w;
                widths.insert(def.field.clone(), w);
            }
        }
    }

    // Share the remaining width by weight. A column that hits its min or max is
    // frozen at that bound and the rest are re-shared, so the row still fills.
    let mut remaining = (table_width_px - fixed_total).max(0.0);
    while !flex.is_empty() {
        let weight_total: f64 = flex.iter().map(|(_, w)| w).sum();
        if weight_total <= 0.0 {
            for (def, _) in flex.drain(..) {
                widths.insert(
                    def.field.clone(),
                    clamp_width(100.0, def.min_width, def.max_width),
                );
            }
            break;
        }
        let frozen = flex.iter().position(|(def, weight)| {
            let share = remaining * weight / weight_total;
            clamp_width(share, def.min_width, def.max_width) != share
        });
        match frozen {
            Some(i) => {
                let (def, weight) = flex.remove(i);
                let share = remaining * weight / weight_total;
                let w = clamp_width(share, def.min_width, def.max_width);
                remaining = (remaining - w).max(0.0);
                widths.insert(def.field.clone(), w);
            }
            None => {
                for (def, weight) in flex.drain(..) {
                    widths.insert(def.field.clone(), remaining * weight / weight_total);
                }
            }
        }
    }

    widths
}

fn clamp_width(width: f64, min_width: Option<f64>, max_width: Option<f64>) -> f64 {
    let mut w = width.max(0.0);
    if let Some(min) = min_width {
        w = w.max(min);
    }
    if let Some(max) = max_width {
        w = w.min(max);
    }
    w
}

/// All column definitions in display order (includes hidden columns).
pub fn ordered_column_defs(
    defs: &[DataTableColumnDef],
    column_order: &[String],
) -> Vec<DataTableColumnDef> {
    let def_map: HashMap<&str, &DataTableColumnDef> =
        defs.iter().map(|d| (d.field.as_str(), d)).collect();

    let order = if column_order.is_empty() {
        defs.iter().map(|d| d.field.clone()).collect()
    } else {
        let mut ordered: Vec<String> = column_order
            .iter()
            .filter(|f| def_map.contains_key(f.as_str()))
            .cloned()
            .collect();
        for def in defs {
            if !ordered.iter().any(|f| f == &def.field) {
                ordered.push(def.field.clone());
            }
        }
        ordered
    };

    order
        .iter()
        .filter_map(|field| def_map.get(field.as_str()).copied())
        .cloned()
        .collect()
}

/// Build ordered visible columns with resolved widths and pin offsets.
pub fn build_column_layout(input: &ColumnLayoutInput<'_>) -> ColumnLayout {
    let visible_order: Vec<DataTableColumnDef> =
        ordered_column_defs(input.defs, input.column_order)
            .into_iter()
            .filter(|def| is_visible(&def.field, input.column_visibility))
            .collect();

    let table_width = input.table_width_px.unwrap_or(DEFAULT_TABLE_WIDTH_PX);
    let width_map = resolve_widths(&visible_order, input.column_widths, table_width);

    let mut left: Vec<DataTableColumnDef> = Vec::new();
    let mut center: Vec<DataTableColumnDef> = Vec::new();
    let mut right: Vec<DataTableColumnDef> = Vec::new();

    for def in visible_order {
        match resolve_pin_side(&def.field, &def, input.pinned_columns) {
            Some(PinSide::Left) if input.features.contains(DataTableFeatures::COLUMN_PINNING) => {
                left.push(def);
            }
            Some(PinSide::Right) if input.features.contains(DataTableFeatures::COLUMN_PINNING) => {
                right.push(def);
            }
            _ => center.push(def),
        }
    }

    let ordered: Vec<DataTableColumnDef> = left
        .iter()
        .chain(center.iter())
        .chain(right.iter())
        .cloned()
        .collect();

    let left_count = left.len();
    let right_count = right.len();

    let mut offset = 0.0;
    let mut resolved: Vec<ResolvedColumn> = Vec::new();

    for (index, def) in ordered.iter().enumerate() {
        let width_px = *width_map.get(&def.field).unwrap_or(&100.0);
        let pin_side = resolve_pin_side(&def.field, def, input.pinned_columns);

        let pin = if input.features.contains(DataTableFeatures::COLUMN_PINNING) {
            pin_side.map(|side| {
                let meta = match side {
                    PinSide::Left => {
                        let z = 2
                            + (left_count.saturating_sub(1)).saturating_sub(
                                left.iter().position(|d| d.field == def.field).unwrap_or(0),
                            ) as u32;
                        let is_last_left =
                            left.last().map(|d| d.field == def.field).unwrap_or(false);
                        let current_offset = offset;
                        offset += width_px;
                        ColumnPinMeta {
                            side,
                            offset_px: current_offset,
                            z_index: z,
                            is_last_left,
                            is_first_right: false,
                        }
                    }
                    PinSide::Right => ColumnPinMeta {
                        side,
                        offset_px: 0.0,
                        z_index: 2,
                        is_last_left: false,
                        is_first_right: false,
                    },
                };
                meta
            })
        } else {
            None
        };

        resolved.push(ResolvedColumn {
            def: def.clone(),
            width_px,
            pin,
        });

        let _ = index;
    }

    // Compute right offsets (walk from right edge inward)
    if input.features.contains(DataTableFeatures::COLUMN_PINNING) && right_count > 0 {
        let mut right_offset = 0.0;
        for def in right.iter().rev() {
            let width_px = *width_map.get(&def.field).unwrap_or(&100.0);
            if let Some(col) = resolved.iter_mut().find(|c| c.def.field == def.field) {
                if let Some(ref mut pin) = col.pin {
                    pin.offset_px = right_offset;
                    right_offset += width_px;
                }
            }
        }
        if let Some(first_right_field) = right.first().map(|d| d.field.clone()) {
            for col in resolved.iter_mut() {
                if let Some(ref mut pin) = col.pin {
                    if pin.side == PinSide::Right && col.def.field == first_right_field {
                        pin.is_first_right = true;
                    }
                }
            }
        }
    }

    ColumnLayout { columns: resolved }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DataTableColumnDef;

    #[test]
    fn hides_invisible_columns() {
        let defs = vec![
            DataTableColumnDef::new("a", "A"),
            DataTableColumnDef::new("b", "B"),
        ];
        let visibility = HashMap::from([("b".into(), false)]);
        let layout = build_column_layout(&ColumnLayoutInput {
            defs: &defs,
            column_order: &[],
            column_visibility: &visibility,
            column_widths: &HashMap::new(),
            pinned_columns: &PinnedColumnsState::default(),
            features: DataTableFeatures::empty(),
            table_width_px: None,
        });
        assert_eq!(layout.columns.len(), 1);
        assert_eq!(layout.columns[0].def.field, "a");
    }

    #[test]
    fn pins_left_columns_with_offsets() {
        let defs = vec![
            DataTableColumnDef::new("a", "A").with_pinned(PinSide::Left),
            DataTableColumnDef::new("b", "B"),
        ];
        let layout = build_column_layout(&ColumnLayoutInput {
            defs: &defs,
            column_order: &[],
            column_visibility: &HashMap::new(),
            column_widths: &HashMap::new(),
            pinned_columns: &PinnedColumnsState::default(),
            features: DataTableFeatures::COLUMN_PINNING,
            table_width_px: None,
        });
        assert_eq!(layout.columns.len(), 2);
        assert!(layout.columns[0].pin.is_some());
        assert_eq!(layout.columns[0].pin.unwrap().offset_px, 0.0);
        assert!(layout.columns[1].pin.is_none());
    }

    fn flex_def(field: &str, weight: f32) -> DataTableColumnDef {
        DataTableColumnDef::new(field, field).with_width(ColumnWidth::Flex(weight))
    }

    fn total(widths: &HashMap<String, f64>) -> f64 {
        widths.values().sum()
    }

    #[test]
    fn default_columns_fill_measured_width() {
        for table_width in [390.0, 1024.0, 1440.0] {
            let defs = vec![
                DataTableColumnDef::new("a", "A"),
                DataTableColumnDef::new("b", "B"),
                DataTableColumnDef::new("c", "C"),
            ];
            let widths = resolve_widths(&defs, &HashMap::new(), table_width);
            assert!((total(&widths) - table_width).abs() < 1e-6, "{table_width}");
        }
    }

    #[test]
    fn flex_weights_split_proportionally() {
        let defs = vec![flex_def("a", 1.0), flex_def("b", 3.0)];
        let widths = resolve_widths(&defs, &HashMap::new(), 400.0);
        assert_eq!(widths["a"], 100.0);
        assert_eq!(widths["b"], 300.0);
    }

    #[test]
    fn dragged_column_keeps_width_and_others_refill() {
        let defs = vec![flex_def("a", 1.0), flex_def("b", 1.0), flex_def("c", 1.0)];
        let dragged = HashMap::from([("a".to_string(), 150.0)]);
        let before = resolve_widths(&defs, &dragged, 900.0);
        let after = resolve_widths(&defs, &dragged, 600.0);
        assert_eq!(before["a"], 150.0);
        assert_eq!(after["a"], 150.0);
        assert_eq!(before["b"], 375.0);
        assert_eq!(after["b"], 225.0);
        assert!((total(&after) - 600.0).abs() < 1e-6);
    }

    #[test]
    fn fixed_columns_are_subtracted_before_flex_share() {
        let defs = vec![
            DataTableColumnDef::new("a", "A").with_width(ColumnWidth::Fixed(200.0)),
            flex_def("b", 1.0),
            flex_def("c", 1.0),
        ];
        let widths = resolve_widths(&defs, &HashMap::new(), 600.0);
        assert_eq!(widths["a"], 200.0);
        assert_eq!(widths["b"], 200.0);
        assert_eq!(widths["c"], 200.0);
    }

    #[test]
    fn clamped_flex_column_leaves_the_rest_to_its_siblings() {
        let mut capped = flex_def("a", 1.0);
        capped.max_width = Some(100.0);
        let defs = vec![capped, flex_def("b", 1.0)];
        let widths = resolve_widths(&defs, &HashMap::new(), 600.0);
        assert_eq!(widths["a"], 100.0);
        assert_eq!(widths["b"], 500.0);

        let mut floored = flex_def("a", 1.0);
        floored.min_width = Some(400.0);
        let defs = vec![floored, flex_def("b", 1.0)];
        let widths = resolve_widths(&defs, &HashMap::new(), 600.0);
        assert_eq!(widths["a"], 400.0);
        assert_eq!(widths["b"], 200.0);
    }

    #[test]
    fn narrow_container_never_goes_below_min_width() {
        let mut a = flex_def("a", 1.0);
        a.min_width = Some(120.0);
        let widths = resolve_widths(&[a], &HashMap::new(), 50.0);
        assert_eq!(widths["a"], 120.0);
    }

    #[test]
    fn build_layout_uses_measured_width_and_falls_back() {
        let defs = vec![flex_def("a", 1.0), flex_def("b", 1.0)];
        let layout_for = |table_width_px| {
            build_column_layout(&ColumnLayoutInput {
                defs: &defs,
                column_order: &[],
                column_visibility: &HashMap::new(),
                column_widths: &HashMap::new(),
                pinned_columns: &PinnedColumnsState::default(),
                features: DataTableFeatures::empty(),
                table_width_px,
            })
        };
        assert_eq!(layout_for(Some(1000.0)).columns[0].width_px, 500.0);
        assert_eq!(
            layout_for(None).columns[0].width_px,
            DEFAULT_TABLE_WIDTH_PX / 2.0
        );
    }
}
