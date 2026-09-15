//! [`ResponsiveChartContainer`] — parent-fill chart container.

use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::context::ChartKind;
use crate::engine::default_plot_inset;
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, ChartEmbedMode, ChartItemId, ChartMotion,
    ChartOrientation, GridConfig, HighlightScope, LegendConfig, OrbitalChartPalette,
    OrbitalChartsTheme, OverlayMount, PlotInset, SeriesDef, TooltipConfig,
};
use leptos::callback::Callback;

use super::chart_root::ChartRoot;
use super::resize::use_container_size;

/// Measured host that re-renders when container size changes.
#[component]
fn ResponsiveChartMeasuredHost(
    node_ref: NodeRef<leptos::html::Div>,
    measured: ReadSignal<(f64, f64)>,
    width_override: Option<f64>,
    height_override: Option<f64>,
    dataset: Option<Dataset>,
    binding: Option<ChartFieldBinding>,
    margin: PlotInset,
    skip_animation: Option<bool>,
    motion: Option<ChartMotion>,
    loading: bool,
    series: Option<Vec<SeriesDef>>,
    x_axis: Option<Vec<AxisDef>>,
    y_axis: Option<Vec<AxisDef>>,
    grid: Option<GridConfig>,
    palette: Option<OrbitalChartPalette>,
    orientation: ChartOrientation,
    highlight_scope: Option<HighlightScope>,
    axis_highlight: Option<AxisHighlightConfig>,
    legend: Option<LegendConfig>,
    tooltip: Option<TooltipConfig>,
    charts_theme: Option<OrbitalChartsTheme>,
    highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
    on_legend_click: Option<Callback<(String,), ()>>,
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    embed_mode: ChartEmbedMode,
    overlay_mount: OverlayMount,
    chart_kind: ChartKind,
    prefer_line_x_strict: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    // Not `#[prop(optional)]`: this internal component is always invoked explicitly with
    // `children=children` from `ResponsiveChartContainer` (already an `Option<ChildrenFn>`),
    // and `#[prop(optional)]` would make the generated setter expect the *unwrapped*
    // `ChildrenFn` instead (the macro wraps it in `Some(...)` itself), which is the opposite
    // of what a passthrough of an already-`Option` value needs.
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // Reactive on `measured` (and thus on window/container resize): re-renders `ChartRoot`
    // with fresh dimensions on every ResizeObserver callback. A previous version read
    // `measured.get()` once, outside this closure, at component-setup time — the value was
    // captured but never revisited, so the chart silently stayed pinned to whatever size it
    // measured (or the fallback) on first mount and never grew or shrank afterward.
    view! {
        <div class="orb-chart-responsive-host" node_ref=node_ref>
            {move || {
                let (mw, mh) = measured.get();
                let width = width_override.unwrap_or(mw);
                let height = height_override.unwrap_or(mh);
                // Clone the `Arc` (cheap) rather than let `ChartRoot`'s own `Children`
                // (`FnOnce`) slot move the captured `children` out of this closure's
                // environment — that would make this whole closure only callable once,
                // defeating the resize reactivity this component exists to provide.
                let children = children.clone();
                view! {
                    <ChartRoot
                        class=class.clone()
                        dataset=dataset.clone()
                        binding=binding.clone()
                        width=width
                        height=height
                        margin=margin
                        skip_animation=skip_animation
                        motion=motion.clone()
                        loading=loading
                        series=series.clone()
                        x_axis=x_axis.clone()
                        y_axis=y_axis.clone()
                        grid=grid
                        palette=palette.clone()
                        orientation=orientation
                        highlight_scope=highlight_scope
                        axis_highlight=axis_highlight
                        legend=legend.clone()
                        tooltip=tooltip.clone()
                        charts_theme=charts_theme.clone()
                        highlighted_item=highlighted_item
                        on_highlight_change=on_highlight_change
                        on_item_click=on_item_click
                        on_axis_click=on_axis_click
                        on_legend_click=on_legend_click
                        loading_view=loading_view.clone()
                        empty_view=empty_view.clone()
                        embed_mode=embed_mode
                        overlay_mount=overlay_mount.clone()
                        chart_kind=chart_kind
                        prefer_line_x_strict=prefer_line_x_strict
                    >
                        {children.map(|c| c())}
                    </ChartRoot>
                }
            }}
        </div>
    }
}

/// Chart container that fills its parent via resize observer.
///
/// # When to use
///
/// - Dashboard grids or flex layouts where the chart should adapt to available space.
/// - Cards without a fixed pixel width — the SVG viewport tracks the parent box.
///
/// # Usage
///
/// 1. Wrap in a sized parent (`width: 100%`, explicit height, or flex grow).
/// 2. Pass the same series, axis, and binding props as [`ChartContainer`].
/// 3. Optionally set explicit `width` / `height` to override measured dimensions.
///
/// # Best Practices
///
/// ## Do's
///
/// * Give the parent a definite height — width alone is not enough for layout stability.
/// * Debounce expensive re-projection in app code if the parent resizes frequently.
///
/// ## Don'ts
///
/// * Do not nest responsive containers without a bounded parent — measurement loops are possible.
///
/// # Examples
///
/// ## Responsive container
/// Fills a 100% × 280px parent. Resize the browser to confirm the SVG viewport tracks the host box.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ResponsiveChartContainer;
/// view! {
///     <div style="width: 100%; height: 280px;" data-testid="chart-container-responsive-host">
///         <ResponsiveChartContainer />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "chart-container-responsive",
    preview_label = "Responsive Chart Container",
    preview_icon = icondata::AiExpandOutlined,
)]
#[component]
pub fn ResponsiveChartContainer(
    /// Series definitions (explicit or derived from dataset + binding).
    #[prop(default = None)]
    series: Option<Vec<SeriesDef>>,
    /// Tabular data — primary consumption mode.
    #[prop(default = None)]
    dataset: Option<Dataset>,
    /// Shorthand field binding when using dataset.
    #[prop(default = None)]
    binding: Option<ChartFieldBinding>,
    /// X-axis definitions.
    #[prop(default = None)]
    x_axis: Option<Vec<AxisDef>>,
    /// Y-axis definitions.
    #[prop(default = None)]
    y_axis: Option<Vec<AxisDef>>,
    /// Optional explicit width override (pixels).
    #[prop(default = None)]
    width: Option<f64>,
    /// Optional explicit height override (pixels).
    #[prop(default = None)]
    height: Option<f64>,
    /// Space between SVG border and plot. User docs: "plot inset".
    #[prop(default = None)]
    margin: Option<PlotInset>,
    /// Background grid configuration.
    #[prop(default = None)]
    grid: Option<GridConfig>,
    /// Whether the chart is in a loading state.
    #[prop(default = None)]
    loading: Option<bool>,
    /// When true, skip all enter/update animations.
    #[prop(default = None)]
    skip_animation: Option<bool>,
    /// Chart-level animation configuration.
    #[prop(default = None)]
    motion: Option<ChartMotion>,
    /// Color palette override.
    #[prop(default = None)]
    palette: Option<OrbitalChartPalette>,
    /// Chart orientation for cartesian plots (e.g. `BarChart` horizontal bars).
    #[prop(default = ChartOrientation::Vertical)]
    orientation: ChartOrientation,
    /// Highlight and fade scope.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
    /// Axis highlight configuration.
    #[prop(default = None)]
    axis_highlight: Option<AxisHighlightConfig>,
    /// Legend configuration.
    #[prop(default = None)]
    legend: Option<LegendConfig>,
    /// Tooltip configuration.
    #[prop(default = None)]
    tooltip: Option<TooltipConfig>,
    /// Chart theme extension.
    #[prop(default = None)]
    charts_theme: Option<OrbitalChartsTheme>,
    /// Controlled highlighted item.
    #[prop(default = None)]
    highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    /// Fired when highlight changes.
    #[prop(default = None)]
    on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
    /// Fired when a legend entry is clicked.
    #[prop(default = None)]
    on_legend_click: Option<Callback<(String,), ()>>,
    /// Fired when a mark is clicked.
    #[prop(default = None)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Fired when an axis band is clicked.
    #[prop(default = None)]
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Custom loading overlay slot.
    #[prop(default = None)]
    loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Custom empty overlay slot.
    #[prop(default = None)]
    empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// How the chart is embedded in its host (scroll, dialog, table cell).
    #[prop(default = ChartEmbedMode::Inline)]
    embed_mode: ChartEmbedMode,
    /// Portal mount override for overlay chrome.
    #[prop(default = OverlayMount::ChartLocal)]
    overlay_mount: OverlayMount,
    /// Chart geometry family.
    #[prop(default = ChartKind::Cartesian)]
    chart_kind: ChartKind,
    /// When true, inferred line/area x-axes use [`crate::DomainLimit::Strict`]. Chart roots
    /// that set this in their fixed-size branch (e.g. `LineChart`, `AreaChart`) must pass the
    /// same value here so responsive mode doesn't silently change domain behavior.
    #[prop(default = false)]
    prefer_line_x_strict: bool,
    /// Composition children (plot, axis, legend layers). `ChildrenFn` (not the usual
    /// one-shot `Children`) because the inner chart re-renders on every container resize.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let host_ref = NodeRef::<leptos::html::Div>::new();
    let fallback_w = width.unwrap_or(400.0);
    let fallback_h = height.unwrap_or(300.0);
    let measured = use_container_size(host_ref, (fallback_w, fallback_h));
    let theme_options = use_theme_options();
    let inset = margin.unwrap_or_else(|| default_plot_inset(theme_options.get_untracked().density));
    let is_loading = loading.unwrap_or(false);
    let root_class = class;

    view! {
        <ResponsiveChartMeasuredHost
            node_ref=host_ref
            measured=measured
            width_override=width
            height_override=height
            class=root_class
            dataset=dataset
                binding=binding
                margin=inset
                skip_animation=skip_animation
                motion=motion
                loading=is_loading
                series=series
                x_axis=x_axis
                y_axis=y_axis
                grid=grid
                palette=palette
                orientation=orientation
                highlight_scope=highlight_scope
                axis_highlight=axis_highlight
                legend=legend
                tooltip=tooltip
                charts_theme=charts_theme
                highlighted_item=highlighted_item
                on_highlight_change=on_highlight_change
                on_item_click=on_item_click
                on_axis_click=on_axis_click
                on_legend_click=on_legend_click
                loading_view=loading_view
                empty_view=empty_view
                embed_mode=embed_mode
                overlay_mount=overlay_mount
                chart_kind=chart_kind
                prefer_line_x_strict=prefer_line_x_strict
                children=children
            />
    }
}
