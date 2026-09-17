//! Static sample data for chart previews.

use leptos::callback::Callback;
use orbital_data::{ChartFieldBinding, DataRecord, DataSchema, DataValue, Dataset};
use std::collections::HashMap;

use crate::{AxisDef, ChartType, DomainLimit, GridConfig, ScaleType, ScatterPoint, SeriesDef};

/// Quarterly revenue categories for demos.
pub fn quarter_categories() -> Vec<String> {
    vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()]
}

/// Inline revenue series aligned with [`quarter_categories`].
pub fn revenue_series() -> SeriesDef {
    SeriesDef {
        id: "revenue".into(),
        label: Some("Revenue".into()),
        data: Some(vec![420_000.0, 510_000.0, 480_000.0, 550_000.0]),
        show_markers: Some(true),
        ..Default::default()
    }
}

/// Inline cost series aligned with [`quarter_categories`].
pub fn cost_series() -> SeriesDef {
    SeriesDef {
        id: "cost".into(),
        label: Some("Cost".into()),
        data: Some(vec![280_000.0, 310_000.0, 295_000.0, 320_000.0]),
        show_markers: Some(true),
        ..Default::default()
    }
}

/// Revenue series with a missing middle value for `connect_nulls` demos.
pub fn sparse_revenue_series() -> SeriesDef {
    SeriesDef {
        id: "revenue".into(),
        label: Some("Revenue".into()),
        data: Some(vec![420_000.0, f64::NAN, 480_000.0, 550_000.0]),
        connect_nulls: Some(true),
        show_markers: Some(true),
        ..Default::default()
    }
}

/// Default band x-axis for quarterly demos.
pub fn quarter_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(quarter_categories()),
        label: Some("Quarter".into()),
        position: crate::AxisPosition::Bottom,
        ..Default::default()
    }
}

/// 24 unique full-timestamp band keys — the dense-axis worst case: one bucket per hour, keyed
/// by full datetime (as `spectra-app` keys real time-series buckets) so labels would collide
/// without [`AxisDef::tick_labels`] rotation/thinning.
pub fn dense_hourly_categories() -> Vec<String> {
    (0..24)
        .map(|h| format!("2026-01-05 {h:02}:00:00"))
        .collect()
}

/// Short `"HH:MM"` display labels for [`dense_hourly_categories`] — same length/order.
pub fn dense_hourly_tick_labels() -> Vec<String> {
    (0..24).map(|h| format!("{h:02}:00")).collect()
}

/// Dense 24-bucket band x-axis pairing a full-timestamp scale key with a short display label —
/// the shape `spectra-app`'s time-series chart produces for a busy schema.
pub fn dense_hourly_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(dense_hourly_categories()),
        tick_labels: Some(dense_hourly_tick_labels()),
        label: Some("Hour".into()),
        position: crate::AxisPosition::Bottom,
        ..Default::default()
    }
}

/// Inline series aligned with [`dense_hourly_categories`] — 24 values, one per hour.
pub fn dense_hourly_series() -> SeriesDef {
    SeriesDef {
        id: "events".into(),
        label: Some("Events".into()),
        data: Some((0..24).map(|h| 40.0 + (h as f64 * 7.0 % 53.0)).collect()),
        ..Default::default()
    }
}

/// Default linear y-axis for revenue demos.
pub fn revenue_y_axis() -> AxisDef {
    AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Linear,
        label: Some("Amount (USD)".into()),
        position: crate::AxisPosition::Left,
        ..Default::default()
    }
}

/// Linear value axis for horizontal bar layouts (categories on y).
pub fn revenue_x_axis_linear() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Linear,
        label: Some("Amount (USD)".into()),
        position: crate::AxisPosition::Bottom,
        ..Default::default()
    }
}

/// Band category axis for horizontal bar layouts.
pub fn quarter_y_axis_band() -> AxisDef {
    AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Band,
        data: Some(quarter_categories()),
        label: Some("Quarter".into()),
        position: crate::AxisPosition::Left,
        ..Default::default()
    }
}

/// Grid with both horizontal and vertical lines enabled.
pub fn full_grid() -> GridConfig {
    GridConfig {
        horizontal: true,
        vertical: true,
    }
}

/// Y-axis with currency tick formatting for axis preview.
pub fn formatted_revenue_y_axis() -> AxisDef {
    AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Linear,
        label: Some("Revenue".into()),
        tick_format: Some(Callback::new(|(value,): (f64,)| {
            if value >= 1_000.0 {
                format!("${:.0}k", value / 1_000.0)
            } else {
                format!("${:.0}", value)
            }
        })),
        ..Default::default()
    }
}

/// Dataset fixture mimicking processed table output.
pub fn processed_dataset() -> Dataset {
    let schema = DataSchema::from_text_fields([
        ("quarter".into(), "Quarter".into()),
        ("revenue".into(), "Revenue".into()),
        ("cost".into(), "Cost".into()),
    ]);
    let records = vec![
        DataRecord::new(
            "1",
            HashMap::from([
                ("quarter".into(), DataValue::Category("Q1".into())),
                ("revenue".into(), DataValue::Number(420_000.0)),
                ("cost".into(), DataValue::Number(280_000.0)),
            ]),
        ),
        DataRecord::new(
            "2",
            HashMap::from([
                ("quarter".into(), DataValue::Category("Q2".into())),
                ("revenue".into(), DataValue::Number(510_000.0)),
                ("cost".into(), DataValue::Number(310_000.0)),
            ]),
        ),
        DataRecord::new(
            "3",
            HashMap::from([
                ("quarter".into(), DataValue::Category("Q3".into())),
                ("revenue".into(), DataValue::Number(480_000.0)),
                ("cost".into(), DataValue::Number(295_000.0)),
            ]),
        ),
        DataRecord::new(
            "4",
            HashMap::from([
                ("quarter".into(), DataValue::Category("Q4".into())),
                ("revenue".into(), DataValue::Number(550_000.0)),
                ("cost".into(), DataValue::Number(320_000.0)),
            ]),
        ),
    ];
    Dataset::from_records(schema, records)
}

/// Binding for the processed dataset fixture.
pub fn processed_binding() -> ChartFieldBinding {
    ChartFieldBinding::new("quarter", vec!["revenue".into(), "cost".into()])
}

/// Monthly category labels for area chart demos.
pub fn month_categories() -> Vec<String> {
    vec![
        "Jan".into(),
        "Feb".into(),
        "Mar".into(),
        "Apr".into(),
        "May".into(),
        "Jun".into(),
        "Jul".into(),
    ]
}

/// Stacked area series with shared stack group.
pub fn stacked_area_series() -> Vec<SeriesDef> {
    vec![
        SeriesDef {
            id: "alpha".into(),
            label: Some("Alpha".into()),
            data: Some(vec![4000.0, 3000.0, 2000.0, 2780.0, 1890.0, 2390.0, 3490.0]),
            stack_group: Some("stack".into()),
            area: Some(true),
            ..Default::default()
        },
        SeriesDef {
            id: "beta".into(),
            label: Some("Beta".into()),
            data: Some(vec![2400.0, 1398.0, 9800.0, 3908.0, 4800.0, 3800.0, 4300.0]),
            stack_group: Some("stack".into()),
            area: Some(true),
            ..Default::default()
        },
        SeriesDef {
            id: "gamma".into(),
            label: Some("Gamma".into()),
            data: Some(vec![2400.0, 2210.0, 2290.0, 2000.0, 2181.0, 2500.0, 2100.0]),
            stack_group: Some("stack".into()),
            area: Some(true),
            ..Default::default()
        },
    ]
}

/// Monthly categories for mixed bar + line composition demos.
pub fn mixed_bar_line_categories() -> Vec<String> {
    vec![
        "Jan".into(),
        "Feb".into(),
        "Mar".into(),
        "Apr".into(),
        "May".into(),
        "Jun".into(),
    ]
}

/// Mixed bar + line series for composition preview (CH-17).
pub fn mixed_bar_line_series() -> Vec<SeriesDef> {
    vec![
        SeriesDef {
            id: "revenue".into(),
            label: Some("Revenue".into()),
            chart_type: Some(ChartType::Bar),
            data: Some(vec![
                420_000.0, 510_000.0, 480_000.0, 550_000.0, 520_000.0, 590_000.0,
            ]),
            ..Default::default()
        },
        SeriesDef {
            id: "target".into(),
            label: Some("Target".into()),
            chart_type: Some(ChartType::Line),
            data: Some(vec![
                400_000.0, 500_000.0, 500_000.0, 540_000.0, 530_000.0, 580_000.0,
            ]),
            show_markers: Some(true),
            ..Default::default()
        },
    ]
}

/// Band x-axis for mixed composition demos.
pub fn mixed_bar_line_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(mixed_bar_line_categories()),
        label: Some("Month".into()),
        ..Default::default()
    }
}

/// Stacked bar series with diverging-friendly signed values.
pub fn stacked_bar_series() -> Vec<SeriesDef> {
    vec![
        SeriesDef {
            id: "product".into(),
            label: Some("Product".into()),
            chart_type: Some(ChartType::Bar),
            data: Some(vec![40.0, 35.0, 50.0, 45.0, 55.0, 48.0, 52.0]),
            stack_group: Some("total".into()),
            ..Default::default()
        },
        SeriesDef {
            id: "services".into(),
            label: Some("Services".into()),
            chart_type: Some(ChartType::Bar),
            data: Some(vec![25.0, 30.0, 20.0, 28.0, 22.0, 26.0, 24.0]),
            stack_group: Some("total".into()),
            ..Default::default()
        },
        SeriesDef {
            id: "adjustments".into(),
            label: Some("Adjustments".into()),
            chart_type: Some(ChartType::Bar),
            data: Some(vec![5.0, -8.0, 3.0, -6.0, 4.0, -2.0, 1.0]),
            stack_group: Some("total".into()),
            ..Default::default()
        },
    ]
}

/// Band x-axis for stacked bar demos.
pub fn stacked_bar_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(month_categories()),
        label: Some("Month".into()),
        ..Default::default()
    }
}

/// Pie slice inline series for market share demo.
pub fn market_share_pie_slices() -> SeriesDef {
    SeriesDef {
        id: "share".into(),
        label: Some("Market share".into()),
        data: Some(vec![35.0, 28.0, 22.0, 15.0]),
        ..Default::default()
    }
}

/// Category labels for pie demos.
pub fn market_share_x_axis() -> AxisDef {
    AxisDef {
        id: "labels".into(),
        scale_type: ScaleType::Band,
        data: Some(vec![
            "North".into(),
            "South".into(),
            "East".into(),
            "West".into(),
        ]),
        ..Default::default()
    }
}

/// Dataset fixture for pie chart binding.
pub fn market_share_dataset() -> Dataset {
    let schema = DataSchema::from_text_fields([
        ("segment".into(), "Segment".into()),
        ("share".into(), "Share".into()),
    ]);
    let records = vec![
        DataRecord::new(
            "1",
            HashMap::from([
                ("segment".into(), DataValue::Category("North".into())),
                ("share".into(), DataValue::Number(35.0)),
            ]),
        ),
        DataRecord::new(
            "2",
            HashMap::from([
                ("segment".into(), DataValue::Category("South".into())),
                ("share".into(), DataValue::Number(28.0)),
            ]),
        ),
        DataRecord::new(
            "3",
            HashMap::from([
                ("segment".into(), DataValue::Category("East".into())),
                ("share".into(), DataValue::Number(22.0)),
            ]),
        ),
        DataRecord::new(
            "4",
            HashMap::from([
                ("segment".into(), DataValue::Category("West".into())),
                ("share".into(), DataValue::Number(15.0)),
            ]),
        ),
    ];
    Dataset::from_records(schema, records)
}

/// Binding for market share pie dataset.
pub fn market_share_pie_binding() -> ChartFieldBinding {
    ChartFieldBinding {
        label_field: Some("segment".into()),
        y_fields: vec!["share".into()],
        ..Default::default()
    }
}

/// Single-series scatter points for preview.
pub fn correlation_scatter_series() -> SeriesDef {
    SeriesDef {
        id: "observations".into(),
        label: Some("Observations".into()),
        scatter_data: Some(vec![
            ScatterPoint {
                x: 120.0,
                y: 80.0,
                id: "1".into(),
                z: None,
            },
            ScatterPoint {
                x: 150.0,
                y: 110.0,
                id: "2".into(),
                z: None,
            },
            ScatterPoint {
                x: 180.0,
                y: 95.0,
                id: "3".into(),
                z: None,
            },
            ScatterPoint {
                x: 210.0,
                y: 140.0,
                id: "4".into(),
                z: None,
            },
            ScatterPoint {
                x: 240.0,
                y: 120.0,
                id: "5".into(),
                z: None,
            },
            ScatterPoint {
                x: 270.0,
                y: 165.0,
                id: "6".into(),
                z: None,
            },
        ]),
        ..Default::default()
    }
}

/// Dual y-axis scatter series (CH-11).
pub fn dual_axis_scatter_series() -> Vec<SeriesDef> {
    vec![
        SeriesDef {
            id: "mass".into(),
            label: Some("Mass".into()),
            y_axis_id: Some("leftAxis".into()),
            scatter_data: Some(vec![
                ScatterPoint {
                    x: 160.0,
                    y: 58.0,
                    id: "1".into(),
                    z: None,
                },
                ScatterPoint {
                    x: 170.0,
                    y: 62.0,
                    id: "2".into(),
                    z: None,
                },
                ScatterPoint {
                    x: 180.0,
                    y: 70.0,
                    id: "3".into(),
                    z: None,
                },
            ]),
            ..Default::default()
        },
        SeriesDef {
            id: "strength".into(),
            label: Some("Strength".into()),
            y_axis_id: Some("rightAxis".into()),
            scatter_data: Some(vec![
                ScatterPoint {
                    x: 160.0,
                    y: 420.0,
                    id: "1".into(),
                    z: None,
                },
                ScatterPoint {
                    x: 170.0,
                    y: 480.0,
                    id: "2".into(),
                    z: None,
                },
                ScatterPoint {
                    x: 180.0,
                    y: 510.0,
                    id: "3".into(),
                    z: None,
                },
            ]),
            ..Default::default()
        },
    ]
}

/// Scatter dataset fixture.
pub fn scatter_dataset() -> Dataset {
    let schema = DataSchema::from_text_fields([
        ("height".into(), "Height (cm)".into()),
        ("weight".into(), "Weight (kg)".into()),
        ("subject".into(), "Subject".into()),
    ]);
    let records = vec![
        DataRecord::new(
            "1",
            HashMap::from([
                ("height".into(), DataValue::Number(160.0)),
                ("weight".into(), DataValue::Number(58.0)),
                ("subject".into(), DataValue::Text("A".into())),
            ]),
        ),
        DataRecord::new(
            "2",
            HashMap::from([
                ("height".into(), DataValue::Number(170.0)),
                ("weight".into(), DataValue::Number(62.0)),
                ("subject".into(), DataValue::Text("B".into())),
            ]),
        ),
    ];
    Dataset::from_records(schema, records)
}

/// Scatter field binding.
pub fn scatter_binding() -> ChartFieldBinding {
    ChartFieldBinding {
        x_field: Some("height".into()),
        y_fields: vec!["weight".into()],
        id_field: Some("subject".into()),
        ..Default::default()
    }
}

/// Default scatter x-axis with floor at zero.
pub fn scatter_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Linear,
        label: Some("Height (cm)".into()),
        min: Some(0.0),
        position: crate::AxisPosition::Bottom,
        ..Default::default()
    }
}

/// Default single y-axis for correlation scatter.
pub fn scatter_y_axis() -> Vec<AxisDef> {
    vec![AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Linear,
        label: Some("Weight (kg)".into()),
        min: Some(0.0),
        position: crate::AxisPosition::Left,
        ..Default::default()
    }]
}

/// Left and right y-axes for biaxial scatter (CH-11).
pub fn scatter_biaxial_y_axes() -> Vec<AxisDef> {
    vec![
        AxisDef {
            id: "leftAxis".into(),
            scale_type: ScaleType::Linear,
            label: Some("Mass (kg)".into()),
            min: Some(0.0),
            position: crate::AxisPosition::Left,
            ..Default::default()
        },
        AxisDef {
            id: "rightAxis".into(),
            scale_type: ScaleType::Linear,
            label: Some("Strength (N)".into()),
            min: Some(0.0),
            position: crate::AxisPosition::Right,
            ..Default::default()
        },
    ]
}

/// All scatter y-axis definitions (single + biaxial); prefer [`scatter_y_axis`] or [`scatter_biaxial_y_axes`].
pub fn scatter_y_axes() -> Vec<AxisDef> {
    scatter_y_axis()
}

/// Weekly throughput sample for sparkline previews.
pub fn sparkline_sample_data() -> Vec<f64> {
    vec![3.0, 5.0, 2.0, 8.0, 6.0, 9.0, 7.0, 11.0]
}

/// Heatmap x categories (hours).
pub fn heatmap_x_categories() -> Vec<String> {
    vec![
        "08:00".into(),
        "10:00".into(),
        "12:00".into(),
        "14:00".into(),
        "16:00".into(),
    ]
}

/// Heatmap y categories (regions).
pub fn heatmap_y_categories() -> Vec<String> {
    vec![
        "North".into(),
        "Central".into(),
        "South".into(),
        "East".into(),
    ]
}

/// Sample heatmap cell tuples for previews.
pub fn heatmap_cells() -> Vec<crate::HeatmapCell> {
    use crate::HeatmapCell;
    vec![
        HeatmapCell {
            x: 0,
            y: 0,
            value: 12.0,
        },
        HeatmapCell {
            x: 1,
            y: 0,
            value: 28.0,
        },
        HeatmapCell {
            x: 2,
            y: 0,
            value: 45.0,
        },
        HeatmapCell {
            x: 3,
            y: 0,
            value: 33.0,
        },
        HeatmapCell {
            x: 4,
            y: 0,
            value: 18.0,
        },
        HeatmapCell {
            x: 0,
            y: 1,
            value: 22.0,
        },
        HeatmapCell {
            x: 1,
            y: 1,
            value: 55.0,
        },
        HeatmapCell {
            x: 2,
            y: 1,
            value: 61.0,
        },
        HeatmapCell {
            x: 3,
            y: 1,
            value: 48.0,
        },
        HeatmapCell {
            x: 4,
            y: 1,
            value: 30.0,
        },
        HeatmapCell {
            x: 0,
            y: 2,
            value: 8.0,
        },
        HeatmapCell {
            x: 1,
            y: 2,
            value: 19.0,
        },
        HeatmapCell {
            x: 2,
            y: 2,
            value: 35.0,
        },
        HeatmapCell {
            x: 3,
            y: 2,
            value: 42.0,
        },
        HeatmapCell {
            x: 4,
            y: 2,
            value: 25.0,
        },
        HeatmapCell {
            x: 0,
            y: 3,
            value: 15.0,
        },
        HeatmapCell {
            x: 1,
            y: 3,
            value: 38.0,
        },
        HeatmapCell {
            x: 2,
            y: 3,
            value: 52.0,
        },
        HeatmapCell {
            x: 3,
            y: 3,
            value: 67.0,
        },
        HeatmapCell {
            x: 4,
            y: 3,
            value: 44.0,
        },
    ]
}

/// Monthly categories for zoom/pan demos (24 points).
pub fn monthly_categories() -> Vec<String> {
    (1..=24).map(|m| format!("M{m:02}")).collect()
}

/// Revenue series aligned with [`monthly_categories`].
pub fn monthly_revenue_series() -> SeriesDef {
    SeriesDef {
        id: "revenue".into(),
        label: Some("Revenue".into()),
        data: Some(vec![
            120.0, 135.0, 128.0, 142.0, 155.0, 160.0, 172.0, 168.0, 175.0, 180.0, 190.0, 205.0,
            198.0, 210.0, 215.0, 220.0, 228.0, 235.0, 240.0, 245.0, 250.0, 255.0, 260.0, 268.0,
        ]),
        ..Default::default()
    }
}

/// Band x-axis for monthly zoom demos.
pub fn monthly_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(monthly_categories()),
        label: Some("Month".into()),
        ..Default::default()
    }
}

/// Y-axis for monthly revenue demos.
pub fn monthly_y_axis() -> AxisDef {
    AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Linear,
        label: Some("Revenue (k)".into()),
        ..Default::default()
    }
}

/// Single-series values for x-axis domain limit comparison (horizontal line).
pub fn domain_demo_series() -> SeriesDef {
    SeriesDef {
        id: "value".into(),
        label: Some("Value".into()),
        data: Some(vec![2.0, 50.0, 92.0, 45.0]),
        chart_type: Some(ChartType::Line),
        show_markers: Some(true),
        ..Default::default()
    }
}

/// Band y-axis for horizontal domain demos.
pub fn domain_demo_y_axis() -> AxisDef {
    AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Band,
        data: Some(vec!["A".into(), "B".into(), "C".into(), "D".into()]),
        label: Some("Category".into()),
        ..Default::default()
    }
}

/// Linear x-axis with nice domain padding (override for domain comparison).
pub fn domain_demo_nice_x_axis() -> AxisDef {
    AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Linear,
        label: Some("Value".into()),
        domain_limit: Some(DomainLimit::Nice),
        ..Default::default()
    }
}
