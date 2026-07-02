//! Heatmap cell plot layer.

use leptos::prelude::*;

use crate::context::{use_chart_context, use_heatmap_plot_context, use_x_scale, use_y_scale};
use crate::engine::{
    compute_heatmap_cell_layouts, HEATMAP_CANVAS_THRESHOLD, HEATMAP_CELL_GAP_RATIO,
};
use crate::shared::marks::HeatmapCellMark;
use crate::{AxisClickData, ChartScale};

/// Renders heatmap cells as SVG rects (canvas for dense grids on hydrate).
#[component]
pub fn HeatmapPlot(
    /// Gap ratio between cells.
    #[prop(default = HEATMAP_CELL_GAP_RATIO)]
    gap_ratio: f64,
) -> impl IntoView {
    let ctx = use_chart_context();
    let highlight_scope = ctx.highlight_scope;
    let on_axis = ctx.on_axis_click;
    let heatmap = use_heatmap_plot_context();
    let x_scale = use_x_scale("x");
    let y_scale = use_y_scale("y");

    let cells = heatmap.cells.clone();
    let color_scale = heatmap.color_scale.clone();
    let value_min = heatmap.value_min;
    let value_max = heatmap.value_max;
    let use_canvas = cells.len() > HEATMAP_CANVAS_THRESHOLD;
    let plot_height = ctx.drawing_area.plot_height;

    view! {
        {move || {
            let (x_band, y_band) = match (&x_scale, &y_scale) {
                (ChartScale::Band(x), ChartScale::Band(y)) => (x.clone(), y.clone()),
                _ => return ().into_any(),
            };

            let layouts = compute_heatmap_cell_layouts(
                &cells,
                &x_band,
                &y_band,
                &color_scale,
                value_min,
                value_max,
                gap_ratio,
            );

            if use_canvas {
                #[cfg(feature = "hydrate")]
                {
                    return view! {
                        <HeatmapCanvasLayer layouts=layouts plot_width=ctx.drawing_area.plot_width plot_height=ctx.drawing_area.plot_height />
                    }
                    .into_any();
                }
                #[cfg(not(feature = "hydrate"))]
                {
                    // SSR fallback to SVG
                }
            }

            let x_categories = ctx
                .x_axes
                .first()
                .and_then(|a| a.data.clone())
                .unwrap_or_default();
            let axis_id = ctx.x_axes.first().map(|a| a.id.clone()).unwrap_or_else(|| "x".into());
            let click_bands = on_axis.as_ref().map(|cb| {
                let bw = x_band.bandwidth();
                x_categories
                    .iter()
                    .enumerate()
                    .map(|(index, _)| {
                        let x_center = x_band.scale_by_index(index).unwrap_or(0.0);
                        let cb = *cb;
                        let axis_id = axis_id.clone();
                        view! {
                            <rect
                                class="orb-axis-click-band"
                                x=x_center - bw / 2.0
                                y=plot_height
                                width=bw
                                height=8.0
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                    cb.run((AxisClickData {
                                        axis_id: axis_id.clone(),
                                        value: index as f64,
                                        series_values: vec![],
                                    },));
                                }
                            />
                        }
                    })
                    .collect_view()
            });

            view! {
                {layouts
                    .into_iter()
                    .map(|layout| view! {
                        <HeatmapCellMark layout=layout highlight_scope=highlight_scope />
                    })
                    .collect_view()}
                {click_bands}
            }
            .into_any()
        }}
    }
}

#[cfg(feature = "hydrate")]
#[component]
fn HeatmapCanvasLayer(
    layouts: Vec<crate::HeatmapCellLayout>,
    plot_width: f64,
    plot_height: f64,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            use wasm_bindgen::JsCast;
            use web_sys::Element;

            let w = plot_width.max(1.0) as u32;
            let h = plot_height.max(1.0) as u32;
            let el: &Element = canvas.as_ref();
            let _ = el.set_attribute("width", &w.to_string());
            let _ = el.set_attribute("height", &h.to_string());
            if let Ok(Some(ctx)) = canvas.get_context("2d") {
                if let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                    ctx.clear_rect(0.0, 0.0, w as f64, h as f64);
                    for cell in &layouts {
                        ctx.set_fill_style_str(&cell.fill);
                        ctx.fill_rect(cell.x, cell.y, cell.width, cell.height);
                    }
                }
            }
        }
    });

    view! {
        <foreignObject x="0" y="0" width=plot_width height=plot_height>
            <canvas
                node_ref=canvas_ref
                class="orb-heatmap-canvas"
                style="width: 100%; height: 100%;"
            />
        </foreignObject>
    }
}
