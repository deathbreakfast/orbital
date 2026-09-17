//! Keyboard navigation between chart marks (CH-22).

use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

use crate::context::ChartInteractionContext;
use crate::DrawingArea;
use crate::{ChartFeatures, ChartItemId};

/// A focusable mark with plot-space center coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct FocusableMark {
    /// Item identity for highlight and tooltip.
    pub item: ChartItemId,
    /// Center x in plot coordinates.
    pub plot_x: f64,
    /// Center y in plot coordinates.
    pub plot_y: f64,
}

/// Whether keyboard navigation is active for this chart.
pub fn keyboard_nav_enabled(keyboard_navigation: bool, features: ChartFeatures) -> bool {
    keyboard_navigation && features.contains(ChartFeatures::KEYBOARD_NAV)
}

/// Collect ordered focus targets from the latest plot geometry.
pub fn collect_focusable_marks(interaction: &ChartInteractionContext) -> Vec<FocusableMark> {
    let mut marks = Vec::new();

    for (x, y, series_id, data_index) in interaction.plot_line_markers.get_untracked() {
        marks.push(FocusableMark {
            item: ChartItemId {
                series_id,
                data_index,
            },
            plot_x: x,
            plot_y: y,
        });
    }

    for bar in interaction.plot_bars.get_untracked() {
        marks.push(FocusableMark {
            item: ChartItemId {
                series_id: bar.series_id,
                data_index: bar.data_index,
            },
            plot_x: bar.x + bar.width / 2.0,
            plot_y: bar.y + bar.height / 2.0,
        });
    }

    marks.sort_by(|a, b| {
        a.item
            .data_index
            .cmp(&b.item.data_index)
            .then_with(|| a.item.series_id.cmp(&b.item.series_id))
            .then_with(|| {
                a.plot_x
                    .partial_cmp(&b.plot_x)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
    marks
}

fn set_hover_item(interaction: &ChartInteractionContext, item: Option<ChartItemId>) {
    if let Some(cb) = interaction.on_highlight_change.as_ref() {
        cb.run((item.clone(),));
    }
    interaction.keyboard_focus_item.set(item.clone());
    interaction.hovered_item.set(item.clone());
    interaction.highlighted_item.set(item);
}

fn focus_mark(interaction: &ChartInteractionContext, area: &DrawingArea, mark: &FocusableMark) {
    interaction
        .pointer_plot
        .set(Some((area.left + mark.plot_x, area.top + mark.plot_y)));
    set_hover_item(interaction, Some(mark.item.clone()));
}

fn current_index(marks: &[FocusableMark], active: Option<&ChartItemId>) -> Option<usize> {
    let item = active?;
    marks.iter().position(|m| &m.item == item)
}

/// Handle arrow / home / end keyboard navigation from a key string.
pub fn apply_chart_keyboard_key(
    key: &str,
    interaction: ChartInteractionContext,
    area: DrawingArea,
    active_item: Option<ChartItemId>,
) -> bool {
    let marks = collect_focusable_marks(&interaction);
    if marks.is_empty() {
        return false;
    }

    match key {
        "ArrowLeft" | "ArrowRight" => {
            let idx = current_index(&marks, active_item.as_ref());
            let next = match (key, idx) {
                ("ArrowRight", None) => 0,
                ("ArrowLeft", None) => marks.len().saturating_sub(1),
                ("ArrowRight", Some(i)) => (i + 1).min(marks.len() - 1),
                ("ArrowLeft", Some(i)) => i.saturating_sub(1),
                _ => return true,
            };
            focus_mark(&interaction, &area, &marks[next]);
            true
        }
        "Home" => {
            focus_mark(&interaction, &area, &marks[0]);
            true
        }
        "End" => {
            focus_mark(&interaction, &area, marks.last().expect("non-empty"));
            true
        }
        _ => false,
    }
}

/// Handle arrow / home / end keyboard navigation on chart marks.
pub fn handle_chart_keyboard(
    ev: &KeyboardEvent,
    interaction: ChartInteractionContext,
    area: DrawingArea,
    active_item: Option<ChartItemId>,
) -> bool {
    let key = ev.key();
    if apply_chart_keyboard_key(key.as_str(), interaction, area, active_item) {
        ev.prevent_default();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Owner;

    fn with_owner<F: FnOnce()>(f: F) {
        Owner::new().with(f);
    }

    #[test]
    fn collect_focusable_marks_sorts_by_index_then_series() {
        with_owner(|| {
            let interaction = ChartInteractionContext::new(None);
            interaction.plot_line_markers.set(vec![
                (30.0, 10.0, "b".into(), 1),
                (10.0, 20.0, "a".into(), 0),
                (20.0, 15.0, "b".into(), 0),
            ]);
            let marks = collect_focusable_marks(&interaction);
            assert_eq!(marks.len(), 3);
            assert_eq!(marks[0].item.data_index, 0);
            assert_eq!(marks[0].item.series_id, "a");
            assert_eq!(marks[1].item.series_id, "b");
            assert_eq!(marks[1].item.data_index, 0);
            assert_eq!(marks[2].item.data_index, 1);
        });
    }

    #[test]
    fn apply_chart_keyboard_key_moves_to_first_mark() {
        with_owner(|| {
            let interaction = ChartInteractionContext::new(None);
            interaction.plot_line_markers.set(vec![
                (10.0, 20.0, "revenue".into(), 0),
                (30.0, 10.0, "revenue".into(), 1),
            ]);
            let area = DrawingArea {
                left: 40.0,
                top: 20.0,
                width: 200.0,
                height: 100.0,
                plot_width: 160.0,
                plot_height: 80.0,
            };
            assert!(apply_chart_keyboard_key(
                "ArrowRight",
                interaction,
                area,
                None,
            ));
            assert_eq!(
                interaction.hovered_item.get_untracked(),
                Some(ChartItemId {
                    series_id: "revenue".into(),
                    data_index: 0,
                })
            );
        });
    }
}
