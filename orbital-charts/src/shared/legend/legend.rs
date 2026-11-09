//! Series legend with color swatches and visibility toggles.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_core_components::{
    Checkbox, CheckboxSize, List, ListItem, Material, MaterialElevation,
};

use crate::context::{toggle_series_visibility, use_chart_context, use_hidden_series};
use crate::engine::{resolve_series_color, resolve_series_label};
use crate::{LabelLocation, LegendConfig, LegendDirection};

use super::position::{legend_padding_style, legend_position_class};

/// Chart legend listing series with color swatches and optional visibility toggles.
#[component]
pub fn Legend(
    /// Legend layout and behavior configuration.
    #[prop(default = LegendConfig::default())]
    config: LegendConfig,
    /// Fired when a legend entry is clicked.
    #[prop(default = None)]
    on_legend_click: Option<Callback<(String,), ()>>,
) -> impl IntoView {
    if config.hidden {
        return ().into_view().into_any();
    }

    let ctx = use_chart_context();
    let hidden = use_hidden_series();
    let direction_class = match config.direction {
        LegendDirection::Row => "orb-legend--row",
        LegendDirection::Column => "orb-legend--column",
    };
    let position_class = legend_position_class(&config);
    let edge_class = match config.position.horizontal {
        crate::LegendHorizontalAlign::Right => "orb-legend--outside-right",
        crate::LegendHorizontalAlign::Left => "orb-legend--outside-left",
        _ => "",
    };
    let mark_size = config.item_mark_size;
    let disable_toggle = config.disable_series_toggle;
    let inset_style = legend_padding_style(&config);

    view! {
        <div
            class=format!("orb-legend {direction_class} {position_class} {edge_class}")
            style=inset_style
        >
            <Material elevation=MaterialElevation::Raised class="orb-legend-surface">
                <List>
                    {move || {
                        let _ = hidden.get();
                        ctx.series
                            .iter()
                            .enumerate()
                            .map(|(idx, series)| {
                                let series_id = series.id.clone();
                                let label = resolve_series_label(series, LabelLocation::Legend);
                                let color = resolve_series_color(idx, series, &ctx.palette);
                                let on_click = on_legend_click;
                                let sid_toggle = series_id.clone();

                                view! {
                                    <ListItem selected=Signal::from(false)>
                                        <div class="orb-legend-item">
                                            {(!disable_toggle).then(|| {
                                                let sid = sid_toggle.clone();
                                                let checked =
                                                    RwSignal::new(!hidden.with_untracked(|set| set.contains(&sid)));
                                                view! {
                                                    <Checkbox
                                                        checked=checked
                                                        label=String::new()
                                                        size=Signal::from(CheckboxSize::Medium)
                                                        on_change=Callback::new(move |value| {
                                                            let visible = !hidden.with_untracked(|set| set.contains(&sid));
                                                            if value != visible {
                                                                toggle_series_visibility(&sid);
                                                            }
                                                            if let Some(cb) = on_click.as_ref() {
                                                                cb.run((sid.clone(),));
                                                            }
                                                        })
                                                    />
                                                }
                                            })}
                                            <span
                                                class="orb-legend-mark"
                                                style=format!(
                                                    "display:inline-block;width:{mark_size}px;height:{mark_size}px;background:{color};border-radius:2px;"
                                                )
                                            />
                                            <span class="orb-legend-label">{label}</span>
                                        </div>
                                    </ListItem>
                                }
                            })
                            .collect_view()
                    }}
                </List>
            </Material>
        </div>
    }
    .into_any()
}
