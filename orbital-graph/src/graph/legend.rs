//! The `GraphLegend` component: chips that hide or show sets of graph nodes.

use std::collections::HashSet;

use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_core_components::{Flex, FlexAlign, FlexGap, FlexWrap, ToggleButton};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::GRAPH_STYLES;
use crate::GraphFill;

/// One legend chip: a set of nodes that can be hidden.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphLegendItem {
    /// Set id; matches [`GraphNodeView::set`](crate::GraphNodeView::set).
    pub set: String,
    /// Chip label, such as "Blocked".
    pub label: String,
    /// Swatch fill; use the same fill the set's nodes draw with.
    pub fill: GraphFill,
    /// Nodes in the set; shown as "· N hidden" while the chip is off.
    pub count: Option<u32>,
}

/// A static key entry after the chips, such as "─▶ blocks".
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GraphLegendLine {
    /// Drawn sample, such as "─▶" or "┄┄"; hidden from screen readers.
    pub symbol: Option<String>,
    /// What the sample means, such as "blocks".
    pub label: String,
}

impl GraphLegendLine {
    /// An entry with only a label, such as "dashed box: a task in another project".
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            symbol: None,
            label: label.into(),
        }
    }

    /// Adds the drawn sample shown before the label.
    #[must_use]
    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
}

/// Legend for a [`Graph`](crate::Graph): one toggle chip per set of nodes,
/// static key entries for the line styles, and an optional actions slot.
///
/// Turning a chip off adds its set to `hidden_sets`; pass the same signal to
/// the graph and its nodes leave the drawing.
///
/// # When to use
///
/// - Next to a [`Graph`](crate::Graph) whose nodes are coloured by status,
///   milestone, assignee or tag, so readers can see and filter what each
///   colour means.
///
/// # Usage
///
/// 1. Give each [`GraphNodeView`](crate::GraphNodeView) a `set` and build one
///    [`GraphLegendItem`] per set, with the same fill.
/// 2. Create one `RwSignal<HashSet<String>>` and pass it to both the legend
///    and the graph's `hidden_sets`.
/// 3. Add [`GraphLegendLine`]s for line and box styles, and put host actions
///    such as "Customise colours" in `actions`.
///
/// # Best Practices
///
/// ## Do's
///
/// - Set `count` so a hidden chip says how many nodes it hides.
/// - Keep labels short; the chips wrap onto more lines on narrow screens.
///
/// ## Don'ts
///
/// - Don't rely on the swatch alone: the label names the set.
/// - Don't give two items the same `set`; they would toggle together.
///
/// # Examples
///
/// ## Status legend
/// Chips for four statuses with counts, two key entries and a host action at
/// the end. Turning a chip off strikes its label through and shows the count.
/// <!-- preview -->
/// ```rust
/// use std::collections::HashSet;
/// use crate::{GraphLegend, GraphLegendLine};
/// use crate::preview::fixtures::sample_legend_items;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let items = RwSignal::new(sample_legend_items());
/// let hidden_sets = RwSignal::new(HashSet::<String>::new());
/// view! {
///     <div data-testid="graph-legend-preview">
///         <GraphLegend
///             items=items
///             hidden_sets=hidden_sets
///             lines=vec![
///                 GraphLegendLine::new("blocks").with_symbol("─▶"),
///                 GraphLegendLine::new("dashed box: a task in another project"),
///             ]
///             actions=std::sync::Arc::new(|| view! {
///                 <Button appearance=ButtonAppearance::Transparent>"Customise colours"</Button>
///             }.into_any())
///             data_testid="graph-legend-preview-legend"
///         />
///         <span data-testid="graph-legend-preview-hidden">
///             {move || {
///                 let mut sets: Vec<String> = hidden_sets.get().into_iter().collect();
///                 sets.sort();
///                 sets.join(", ")
///             }}
///         </span>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "graph-legend",
    preview_label = "Graph Legend",
    preview_icon = icondata::AiTagsOutlined,
)]
#[component]
pub fn GraphLegend(
    /// One chip per set, in display order.
    #[prop(into)]
    items: Signal<Vec<GraphLegendItem>>,
    /// Sets that are hidden; chips toggle their set in and out. Share it with the graph.
    hidden_sets: RwSignal<HashSet<String>>,
    /// Static key entries shown after the chips.
    #[prop(optional)]
    lines: Vec<GraphLegendLine>,
    /// Slot at the end of the legend for host actions such as "Customise colours".
    #[prop(optional)]
    actions: Option<ChildrenFn>,
    /// Extra CSS class names on the legend's `Flex` row.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// `data-testid` on the root `div`; chips get `{data_testid}-chip-{set}`.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-graph", GRAPH_STYLES);

    let chips = move || {
        let owner_id = data_testid.get();
        items
            .get()
            .into_iter()
            .map(|item| legend_chip(item, hidden_sets, owner_id.clone()))
            .collect_view()
    };
    let key_lines = lines
        .into_iter()
        .map(|line| {
            view! {
                <span class="orbital-graph-legend__line">
                    {line.symbol.map(|s| view! { <span aria-hidden="true">{s} " "</span> })}
                    {line.label}
                </span>
            }
        })
        .collect_view();
    let class = Signal::derive(move || {
        Some(match class.get() {
            Some(extra) if !extra.is_empty() => format!("orbital-graph-legend {extra}"),
            _ => "orbital-graph-legend".to_string(),
        })
    });

    view! {
        <div role="group" aria-label="Graph legend" data-testid=move || data_testid.get()>
            <Flex class=class gap=FlexGap::Small wrap=FlexWrap::Wrap align=FlexAlign::Center>
                {chips}
                {key_lines}
                {actions.map(|actions| actions())}
            </Flex>
        </div>
    }
}

fn legend_chip(
    item: GraphLegendItem,
    hidden_sets: RwSignal<HashSet<String>>,
    owner_id: Option<String>,
) -> impl IntoView {
    let GraphLegendItem {
        set,
        label,
        fill,
        count,
    } = item;
    let shown = RwSignal::new(!hidden_sets.with_untracked(|h| h.contains(&set)));
    {
        let set = set.clone();
        Effect::new(move |_| {
            let next = !hidden_sets.with(|h| h.contains(&set));
            if shown.get_untracked() != next {
                shown.set(next);
            }
        });
    }
    let toggle = {
        let set = set.clone();
        Handler::on(move |now_shown: bool| {
            hidden_sets.update(|h| {
                if now_shown {
                    h.remove(&set);
                } else {
                    h.insert(set.clone());
                }
            });
        })
    };
    let label_class = move || {
        if shown.get() {
            "orbital-graph-legend__label"
        } else {
            "orbital-graph-legend__label orbital-graph-legend__label--hidden"
        }
    };
    let hidden_note = move || {
        (!shown.get())
            .then_some(count)
            .flatten()
            .map(|n| format!(" · {n} hidden"))
    };
    let testid = owner_id.map(|id| format!("{id}-chip-{set}"));

    view! {
        <span class="orbital-graph-legend__item" data-set=set data-testid=testid>
            <ToggleButton pressed=shown on_click=toggle>
                <span
                    class="orbital-graph-legend__swatch"
                    style=fill.style_vars()
                    aria-hidden="true"
                ></span>
                <span class=label_class>{label}</span>
                <span class="orbital-graph-legend__count">{hidden_note}</span>
            </ToggleButton>
        </span>
    }
}
