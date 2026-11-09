//! Internal chart root — layout shell, context provider, and SVG viewport.

use std::collections::HashMap;
use std::sync::Arc;

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::Dataset;
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::context::{
    collect_focusable_marks, handle_chart_keyboard, initial_zoom_windows, keyboard_nav_enabled,
    provide_overlay_context, provide_tooltip_context, set_hovered_item, use_chart_state,
    use_hovered_item, ChartContextProvider, ChartInteractionContext, ChartInteractionProvider,
    ChartOverlayContext, ChartPointerLayer, ChartZoomProvider, ZoomDragState,
};
use crate::engine::enabled_zoom_axes;
use crate::shared::axis::{ChartGrid, XAxis, YAxis};
use crate::shared::chart_container::{
    keyboard_listener::use_chart_keyboard_listener, ChartRootOverlayChrome, DrawingArea,
};
use crate::shared::highlight::AxisHighlight;
use crate::shared::layers::{ChartKeyboardFocus, ChartZoomLayer};
use crate::shared::styles::{chart_styles, density_modifier_class};
use crate::shared::{legend_outside_reserve_style, PlotClip};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, AxisPosition, ChartEmbedMode, ChartFeatures,
    ChartFieldBinding, ChartItemId, ChartKind, ChartMotion, ChartOrientation, DomainLimit,
    GridConfig, HighlightScope, LegendConfig, OrbitalChartPalette, OrbitalChartsTheme,
    OverlayMount, PlotInset, ScaleType, SeriesDef, TooltipConfig, TooltipTrigger, ZoomConfig,
    ZoomWindow, CHART_FEATURES_DEFAULT,
};

/// Internal chart root layout shell.
#[component]
pub fn ChartRoot(
    #[prop(default = None)] dataset: Option<Dataset>,
    #[prop(default = None)] binding: Option<ChartFieldBinding>,
    #[prop(default = 400.0)] width: f64,
    #[prop(default = 300.0)] height: f64,
    #[prop(default = PlotInset::with_axes())] margin: PlotInset,
    #[prop(default = None)] skip_animation: Option<bool>,
    #[prop(default = None)] motion: Option<ChartMotion>,
    #[prop(default = false)] loading: bool,
    #[prop(default = None)] series: Option<Vec<SeriesDef>>,
    #[prop(default = None)] x_axis: Option<Vec<AxisDef>>,
    #[prop(default = None)] y_axis: Option<Vec<AxisDef>>,
    #[prop(default = None)] grid: Option<GridConfig>,
    #[prop(default = None)] palette: Option<OrbitalChartPalette>,
    #[prop(default = ChartOrientation::Vertical)] orientation: ChartOrientation,
    #[prop(default = ChartKind::Cartesian)] chart_kind: ChartKind,
    #[prop(default = None)] highlight_scope: Option<HighlightScope>,
    #[prop(default = None)] axis_highlight: Option<AxisHighlightConfig>,
    #[prop(default = None)] legend: Option<LegendConfig>,
    #[prop(default = None)] tooltip: Option<TooltipConfig>,
    #[prop(default = None)] charts_theme: Option<OrbitalChartsTheme>,
    #[prop(default = None)] highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    #[prop(default = None)] on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
    #[prop(default = None)] on_item_click: Option<Callback<(ChartItemId,), ()>>,
    #[prop(default = None)] on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    #[prop(default = None)] on_legend_click: Option<Callback<(String,), ()>>,
    #[prop(default = None)] loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    #[prop(default = None)] empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Enable arrow-key navigation between marks (CH-22). Keyboard zoom (CH-24) is deferred.
    #[prop(default = true)]
    keyboard_navigation: bool,
    /// When true, inferred line/area x-axes use [`DomainLimit::Strict`].
    #[prop(default = false)]
    prefer_line_x_strict: bool,
    /// Opt-in capability flags.
    #[prop(default = CHART_FEATURES_DEFAULT)]
    features: ChartFeatures,
    /// Controlled zoom windows per axis.
    #[prop(default = None)]
    zoom: Option<Vec<ZoomWindow>>,
    /// Fired when zoom windows change.
    #[prop(default = None)]
    on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
    /// How the chart is embedded in its host (scroll, dialog, table cell).
    #[prop(default = ChartEmbedMode::Inline)]
    embed_mode: ChartEmbedMode,
    /// Portal mount override for overlay chrome.
    #[prop(default = OverlayMount::ChartLocal)]
    overlay_mount: OverlayMount,
    /// Optional CSS class on the chart root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-charts", chart_styles());

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());
    let charts_theme = charts_theme.unwrap_or_default();
    let theme_style = charts_theme.css_vars();

    let x_axes_ref = x_axis.clone().unwrap_or_default();
    let y_axes_ref = y_axis.clone().unwrap_or_default();
    let enabled_axes: HashMap<String, ZoomConfig> =
        enabled_zoom_axes(features, &x_axes_ref, &y_axes_ref)
            .into_iter()
            .map(|(axis, config)| (axis.id.clone(), config))
            .collect();
    let show_zoom = !enabled_axes.is_empty();

    let initial_windows = initial_zoom_windows(zoom.as_deref(), features, &x_axes_ref, &y_axes_ref);
    let zoom_windows = RwSignal::new(initial_windows);
    let zoom_dragging = RwSignal::new(None::<ZoomDragState>);

    let controlled_zoom = zoom.clone();
    Effect::new(move |_| {
        if let Some(windows) = controlled_zoom.clone() {
            zoom_windows.set(windows);
        }
    });

    let effective_highlight_scope = highlight_scope.or_else(|| {
        tooltip
            .as_ref()
            .filter(|t| matches!(t.trigger, TooltipTrigger::Item))
            .map(|_| HighlightScope::default())
    });

    let state = use_chart_state(
        dataset,
        binding,
        series,
        x_axis,
        y_axis,
        width,
        height,
        margin,
        grid,
        palette,
        skip_animation,
        motion,
        loading,
        orientation,
        chart_kind,
        effective_highlight_scope,
        on_item_click,
        on_axis_click,
        features,
        keyboard_navigation,
        charts_theme.clone(),
        prefer_line_x_strict,
        zoom_windows,
    );

    let tooltip_config = tooltip.clone().unwrap_or_else(|| TooltipConfig {
        trigger: TooltipTrigger::None,
        ..Default::default()
    });
    if !matches!(tooltip_config.trigger, TooltipTrigger::None) {
        provide_tooltip_context(tooltip_config.trigger, tooltip_config.hide_x_header);
    }

    let axis_highlight_config = axis_highlight.unwrap_or_else(|| {
        if matches!(tooltip_config.trigger, TooltipTrigger::Axis) {
            AxisHighlightConfig::bar_default()
        } else {
            AxisHighlightConfig::default()
        }
    });

    let show_pointer_base = chart_kind != ChartKind::Heatmap
        && (matches!(
            tooltip_config.trigger,
            TooltipTrigger::Item | TooltipTrigger::Axis
        ) || axis_highlight_config.x != crate::AxisHighlightMode::None
            || axis_highlight_config.y != crate::AxisHighlightMode::None);

    let legend_config = legend.clone();
    let tooltip_cfg = tooltip.clone();
    let on_zoom_change_cb = on_zoom_change;

    let chart_state = state.get_untracked();
    let area = chart_state.context.drawing_area;
    let plot_transform = format!("translate({}, {})", area.left, area.top);
    let skip_animation_flag = chart_state.context.skip_animation;
    let is_empty = chart_state.context.is_empty;
    let projection_error = chart_state.context.projection_error.clone();
    let show_axes = matches!(
        chart_kind,
        ChartKind::Cartesian | ChartKind::Scatter | ChartKind::Heatmap
    );
    let show_grid = chart_state.context.grid.is_some();

    let left_y_axes: Vec<AxisDef> = chart_state
        .context
        .y_axes
        .iter()
        .filter(|a| a.position == AxisPosition::Left)
        .cloned()
        .collect();
    let right_y_axes: Vec<AxisDef> = chart_state
        .context
        .y_axes
        .iter()
        .filter(|a| a.position == AxisPosition::Right)
        .cloned()
        .collect();

    let legend_outside_style = legend_config
        .as_ref()
        .and_then(legend_outside_reserve_style);

    let zoom_attr = zoom_window_attr(&chart_state.context.zoom_windows);
    let x_domain_attr = Memo::new(move |_| x_domain_attr(&state.get()));
    let x_domain_limit_attr = Memo::new(move |_| x_domain_limit_attr(&state.get()));
    let keyboard_nav = keyboard_nav_enabled(keyboard_navigation, features);
    let aria_label =
        Memo::new(move |_| chart_aria_label(&state.get().context.zoom_windows, keyboard_nav));
    let direction = theme_options.get_untracked().direction.as_str().to_string();

    view! {
        <ChartContextProvider state=chart_state>
            <ChartZoomProvider
                windows=zoom_windows
                dragging=zoom_dragging
                enabled_axes=enabled_axes.clone()
                features=features
                plot_width=area.plot_width
                on_zoom_change=on_zoom_change_cb
            >
                <ChartInteractionProvider
                    highlighted_item=highlighted_item
                    on_highlight_change=on_highlight_change
                >
                    <ChartRootShell
                        width=width
                        height=height
                        theme_style=theme_style
                        density_class=density_class
                        skip_animation_flag=skip_animation_flag
                        area=area
                        plot_transform=plot_transform
                        show_axes=show_axes
                        show_grid=show_grid
                        show_pointer=show_pointer_base
                        show_zoom=show_zoom
                        tooltip_trigger=tooltip_config.trigger
                        axis_highlight_config=axis_highlight_config
                        left_y_axes=left_y_axes
                        right_y_axes=right_y_axes
                        loading=loading
                        is_empty=is_empty
                        projection_error=projection_error
                        loading_view=loading_view
                        empty_view=empty_view
                        legend_config=legend_config
                        tooltip_cfg=tooltip_cfg
                        legend_outside_style=legend_outside_style
                        on_legend_click=on_legend_click
                        root_class=class
                        embed_mode=embed_mode
                        overlay_mount=overlay_mount
                        zoom_attr=zoom_attr
                        x_domain_attr=x_domain_attr
                        x_domain_limit_attr=x_domain_limit_attr
                        keyboard_nav=keyboard_nav
                        aria_label=aria_label
                        direction=direction
                        zoom_dragging=zoom_dragging
                    >
                        {children.map(|c| c())}
                    </ChartRootShell>
                </ChartInteractionProvider>
            </ChartZoomProvider>
        </ChartContextProvider>
    }
}

fn zoom_window_attr(windows: &[ZoomWindow]) -> String {
    windows
        .iter()
        .map(|w| format!("{}:{}-{}", w.axis_id, w.start, w.end))
        .collect::<Vec<_>>()
        .join(";")
}

fn chart_aria_label(windows: &[ZoomWindow], keyboard_nav: bool) -> String {
    let mut parts = Vec::new();
    if windows.is_empty() || windows.iter().all(|w| w.start <= 0.0 && w.end >= 100.0) {
        parts.push("Chart plot area".to_string());
    } else {
        let ranges: Vec<String> = windows
            .iter()
            .filter(|w| w.start > 0.0 || w.end < 100.0)
            .map(|w| {
                format!(
                    "{} axis {:.0}–{:.0} percent visible",
                    w.axis_id, w.start, w.end
                )
            })
            .collect();
        if ranges.is_empty() {
            parts.push("Chart plot area".to_string());
        } else {
            parts.push(format!("Chart plot area. Zoomed: {}.", ranges.join(", ")));
        }
    }
    if keyboard_nav {
        parts.push(
            "Arrow keys move between marks. Home and End jump to first and last marks.".into(),
        );
    }
    parts.join(" ")
}

fn x_domain_attr(state: &crate::context::ChartState) -> String {
    let linear_x = state
        .context
        .x_axes
        .iter()
        .find(|a| a.scale_type == ScaleType::Linear);
    if let Some(axis) = linear_x {
        if let Some((min, max)) = state.axis_domains.get(&axis.id) {
            return format!("{min},{max}");
        }
    }
    String::new()
}

fn x_domain_limit_attr(state: &crate::context::ChartState) -> String {
    state
        .context
        .x_axes
        .first()
        .and_then(|a| a.domain_limit)
        .map(|limit| match limit {
            DomainLimit::Strict => "strict",
            DomainLimit::Nice => "nice",
        })
        .unwrap_or("")
        .to_string()
}

#[component]
fn ChartRootShell(
    width: f64,
    height: f64,
    theme_style: String,
    density_class: Memo<String>,
    skip_animation_flag: bool,
    area: DrawingArea,
    plot_transform: String,
    show_axes: bool,
    show_grid: bool,
    show_pointer: bool,
    show_zoom: bool,
    tooltip_trigger: TooltipTrigger,
    axis_highlight_config: AxisHighlightConfig,
    left_y_axes: Vec<AxisDef>,
    right_y_axes: Vec<AxisDef>,
    loading: bool,
    is_empty: bool,
    projection_error: Option<String>,
    loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    legend_config: Option<LegendConfig>,
    tooltip_cfg: Option<TooltipConfig>,
    legend_outside_style: Option<String>,
    on_legend_click: Option<Callback<(String,), ()>>,
    embed_mode: ChartEmbedMode,
    overlay_mount: OverlayMount,
    zoom_attr: String,
    x_domain_attr: Memo<String>,
    x_domain_limit_attr: Memo<String>,
    keyboard_nav: bool,
    aria_label: Memo<String>,
    direction: String,
    zoom_dragging: RwSignal<Option<ZoomDragState>>,
    #[prop(optional, into)] root_class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let hovered = use_hovered_item();
    let interaction = expect_context::<ChartInteractionContext>();
    let hovered_attr = Signal::derive(move || {
        hovered
            .get()
            .map(|item| format!("{}:{}", item.series_id, item.data_index))
            .unwrap_or_default()
    });
    let marker_count_attr = Signal::derive(move || {
        if !keyboard_nav {
            return String::new();
        }
        interaction.plot_line_markers.get();
        interaction.plot_bars.get();
        collect_focusable_marks(&interaction).len().to_string()
    });
    let root_tabindex = if keyboard_nav { 0 } else { -1 };
    let root_role = if keyboard_nav { "application" } else { "" };

    let show_pointer_layer = move || show_pointer && zoom_dragging.get().is_none();

    let root_ref = NodeRef::<leptos::html::Div>::new();
    let layer_ref = NodeRef::<leptos::html::Div>::new();
    use_chart_keyboard_listener(root_ref, keyboard_nav, interaction, area);
    provide_overlay_context(ChartOverlayContext {
        layer_ref,
        root_ref,
        embed_mode,
        overlay_mount: overlay_mount.clone(),
    });
    view! {
        <div
            class=move || {
                let mut classes = vec!["orb-chart-root".to_string()];
                if let Some(extra) = root_class.get() {
                    if !extra.is_empty() {
                        classes.push(extra);
                    }
                }
                let d = density_class.get();
                if !d.is_empty() {
                    classes.push(d);
                }
                classes.join(" ")
            }
            node_ref=root_ref
            data-orbital-chart=""
            data-orbital-chart-embed=embed_mode.data_attr()
            data-orbital-chart-skip-animation=if skip_animation_flag { "true" } else { "false" }
            data-orbital-chart-hovered=hovered_attr
            data-orbital-chart-marker-count=marker_count_attr
            data-orbital-chart-zoom-window=zoom_attr
            data-orbital-chart-x-domain=move || x_domain_attr.get()
            data-orbital-chart-x-domain-limit=move || x_domain_limit_attr.get()
            dir=direction
            tabindex=root_tabindex
            role=root_role
            aria-label=move || aria_label.get()
            on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                let key = ev.key();
                if keyboard_nav && key != "Escape" {
                    let interaction = expect_context::<ChartInteractionContext>();
                    let active = hovered.get_untracked();
                    if handle_chart_keyboard(&ev, interaction, area, active) {
                        return;
                    }
                }
                if key == "Escape" {
                    set_hovered_item(None);
                    if let Some(ctx) = leptos::prelude::use_context::<ChartInteractionContext>() {
                        ctx.pointer_plot.set(None);
                        ctx.axis_data_index.set(None);
                    }
                }
            }
            style={
                let mut style = format!("width: {width}px; height: {height}px; {theme_style}");
                if let Some(outside) = legend_outside_style {
                    style.push_str(&outside);
                }
                style
            }
        >
            <svg
                class="orb-chart-svg"
                width=area.width
                height=area.height
                viewBox=format!("0 0 {} {}", area.width, area.height)
                role="img"
                aria-label=move || aria_label.get()
            >
                {show_axes.then(|| view! {
                    {left_y_axes.into_iter().map(|axis| {
                        view! { <YAxis axis_id=axis.id.clone() position=AxisPosition::Left /> }
                    }).collect_view()}
                })}
                <g class="orb-plot-content" transform=plot_transform>
                    {show_grid.then(|| view! { <ChartGrid /> })}
                    <PlotClip id="orb-plot-clip".to_string()>
                        {children()}
                        {keyboard_nav.then(|| view! { <ChartKeyboardFocus /> })}
                    </PlotClip>
                    {show_zoom.then(|| view! { <ChartZoomLayer /> })}
                    {move || show_pointer_layer().then(|| view! {
                        <ChartPointerLayer trigger=tooltip_trigger />
                    })}
                    {(axis_highlight_config.x != crate::AxisHighlightMode::None
                        || axis_highlight_config.y != crate::AxisHighlightMode::None)
                        .then(|| view! { <AxisHighlight config=axis_highlight_config /> })}
                </g>
                {show_axes.then(|| view! {
                    {right_y_axes.into_iter().map(|axis| {
                        view! { <YAxis axis_id=axis.id.clone() position=AxisPosition::Right /> }
                    }).collect_view()}
                })}
                {show_axes.then(|| view! { <XAxis /> })}
            </svg>
            <ChartRootOverlayChrome
                layer_ref=layer_ref
                loading=loading
                is_empty=is_empty
                projection_error=projection_error
                loading_view=loading_view
                empty_view=empty_view
                legend_config=legend_config
                tooltip_cfg=tooltip_cfg
                on_legend_click=on_legend_click
            />
        </div>
    }
}
