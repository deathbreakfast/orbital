//! Chart interaction state (hover / highlight / legend visibility).

use std::collections::HashSet;

use leptos::callback::Callback;
use leptos::prelude::*;

use crate::engine::BarGeometry;
use crate::ChartItemId;

/// Interaction context for item hover, highlight, legend, and pointer tracking.
#[derive(Clone, Copy)]
pub struct ChartInteractionContext {
    /// Currently hovered chart item.
    pub hovered_item: RwSignal<Option<ChartItemId>>,
    /// Highlighted item (controlled or derived from hover).
    pub highlighted_item: RwSignal<Option<ChartItemId>>,
    /// Item focused via keyboard navigation or native `:focus`, distinct from mouse hover so
    /// [`crate::shared::layers::ChartKeyboardFocus`]'s ring only shows for keyboard users —
    /// mouse interaction clears it, mirroring `:focus-visible` semantics.
    pub keyboard_focus_item: RwSignal<Option<ChartItemId>>,
    /// Series ids hidden via legend toggle.
    pub hidden_series: RwSignal<HashSet<String>>,
    /// Pointer position in plot coordinates.
    pub pointer_plot: RwSignal<Option<(f64, f64)>>,
    /// Category index under the pointer for axis tooltips and band highlight.
    pub axis_data_index: RwSignal<Option<usize>>,
    /// Latest bar geometries for pointer hit testing.
    pub plot_bars: RwSignal<Vec<BarGeometry>>,
    /// Line marker positions for pointer hit testing.
    pub plot_line_markers: RwSignal<Vec<(f64, f64, String, usize)>>,
    /// Optional callback when highlight changes.
    pub on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
}

impl ChartInteractionContext {
    /// Create a new interaction context with default signals.
    pub fn new(on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>) -> Self {
        Self {
            hovered_item: RwSignal::new(None),
            highlighted_item: RwSignal::new(None),
            keyboard_focus_item: RwSignal::new(None),
            hidden_series: RwSignal::new(HashSet::new()),
            pointer_plot: RwSignal::new(None),
            axis_data_index: RwSignal::new(None),
            plot_bars: RwSignal::new(Vec::new()),
            plot_line_markers: RwSignal::new(Vec::new()),
            on_highlight_change,
        }
    }
}

impl Default for ChartInteractionContext {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Access the hovered item signal.
pub fn use_hovered_item() -> RwSignal<Option<ChartItemId>> {
    expect_context::<ChartInteractionContext>().hovered_item
}

/// Set the hovered item (or clear with `None`) from a mouse/pointer interaction. Clears
/// [`ChartInteractionContext::keyboard_focus_item`] — moving the pointer means keyboard focus is
/// no longer the active input, so its ring should disappear (see
/// [`set_keyboard_focus_item`] for the counterpart that sets it).
pub fn set_hovered_item(item: Option<ChartItemId>) {
    let ctx = expect_context::<ChartInteractionContext>();
    ctx.keyboard_focus_item.set(None);
    ctx.hovered_item.set(item);
    sync_highlight_from_hover(ctx);
}

/// Access the keyboard-focus item signal (drives [`crate::shared::layers::ChartKeyboardFocus`]).
pub fn use_keyboard_focus_item() -> RwSignal<Option<ChartItemId>> {
    expect_context::<ChartInteractionContext>().keyboard_focus_item
}

/// Set the item focused via keyboard navigation or native `:focus` (not mouse hover). Also
/// updates hover/highlight so tooltips and fade behavior stay in sync with the focused mark.
pub fn set_keyboard_focus_item(item: Option<ChartItemId>) {
    let ctx = expect_context::<ChartInteractionContext>();
    ctx.keyboard_focus_item.set(item.clone());
    ctx.hovered_item.set(item);
    sync_highlight_from_hover(ctx);
}

/// Access the highlighted item signal.
pub fn use_highlighted_item() -> RwSignal<Option<ChartItemId>> {
    expect_context::<ChartInteractionContext>().highlighted_item
}

/// Set highlighted item and notify listeners.
pub fn set_highlighted_item(item: Option<ChartItemId>) {
    let ctx = expect_context::<ChartInteractionContext>();
    if let Some(cb) = ctx.on_highlight_change.as_ref() {
        cb.run((item.clone(),));
    }
    ctx.highlighted_item.set(item);
}

/// Access hidden series ids.
pub fn use_hidden_series() -> RwSignal<HashSet<String>> {
    expect_context::<ChartInteractionContext>().hidden_series
}

/// Toggle visibility for a series id in the legend.
pub fn toggle_series_visibility(series_id: &str) {
    use_hidden_series().update(|set| {
        if set.contains(series_id) {
            set.remove(series_id);
        } else {
            set.insert(series_id.to_string());
        }
    });
}

/// Returns true when the series is visible (not hidden by legend).
pub fn is_series_visible(series_id: &str) -> bool {
    !use_hidden_series().with(|set| set.contains(series_id))
}

/// Pointer position in plot coordinates.
pub fn use_pointer_plot() -> ReadSignal<Option<(f64, f64)>> {
    expect_context::<ChartInteractionContext>()
        .pointer_plot
        .read_only()
}

/// Category index under pointer for axis interactions.
pub fn use_axis_data_index() -> ReadSignal<Option<usize>> {
    expect_context::<ChartInteractionContext>()
        .axis_data_index
        .read_only()
}

fn sync_highlight_from_hover(ctx: ChartInteractionContext) {
    let item = ctx.hovered_item.get();
    if let Some(cb) = ctx.on_highlight_change.as_ref() {
        cb.run((item.clone(),));
    }
    ctx.highlighted_item.set(item);
}

/// Provide interaction context to descendants.
#[component]
pub fn ChartInteractionProvider(
    /// Optional external highlight signal for controlled mode.
    #[prop(default = None)]
    highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    /// Fired when highlight changes.
    #[prop(default = None)]
    on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
    children: Children,
) -> impl IntoView {
    let mut ctx = ChartInteractionContext::new(on_highlight_change);
    if let Some(external) = highlighted_item {
        ctx.highlighted_item = external;
    }
    provide_context(ctx);
    children()
}
