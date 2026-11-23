//! [`HeatmapGrid`] root component.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::heatmap_grid_styles;
use crate::engine::{heatmap_grid_domain, heatmap_grid_level};
use crate::HeatmapGridRow;

/// A table of rows and columns with each value cell shaded by its value.
///
/// Cells take one of five shades of the theme's brand colour, lightest for the
/// smallest value, and always print their label too. A cell with no value yet
/// shows a muted empty-cell label ("not yet" by default) and no shade. Leading
/// columns, such as a cohort's size, sit between the row header and the shaded
/// cells and are never shaded.
///
/// # When to use
///
/// - Retention grids: one row per cohort, one column per later period.
/// - Any small table where the pattern across rows and columns matters as much as each number.
/// - Large or continuous grids with axes and hover fit [`crate::Heatmap`] better.
///
/// # Usage
///
/// 1. Set `row_header` and `columns` for the header row, and `leading_columns` for any unshaded columns.
/// 2. Build one [`HeatmapGridRow`] per row with its label, leading text and a `Vec<Option<`[`HeatmapGridCell`](crate::HeatmapGridCell)`>>`, one entry per column; `None` is a cell with no value yet.
/// 3. Leave `value_min` and `value_max` unset to shade across the data's own range, or set them for a fixed scale such as 0–100.
/// 4. Set `aria_label` to name the table, and `empty_label` to change "not yet".
///
/// The grid is a real table with column and row headers, so screen readers
/// announce each cell with its row and column. Every cell prints its label, so
/// the shade is never the only signal. The grid has no tab stops; it scrolls
/// sideways when it is wider than its container.
///
/// # Best Practices
///
/// ## Do's
///
/// * Format cell labels the way people read them, such as `41%`, before passing them in.
/// * Use a fixed `value_min` and `value_max` when grids are compared side by side.
///
/// ## Don'ts
///
/// * Do not leave a missing value as `0`; pass `None` so it reads "not yet" instead of looking like a real zero.
/// * Do not put `data-testid` on the component itself; wrap it in a native element.
///
/// # Examples
///
/// ## Retention grid
/// Weekly cohorts with their size as a leading column. Later weeks that haven't happened yet show "not yet" and stay unshaded.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{HeatmapGrid, HeatmapGridCell, HeatmapGridRow};
/// let pct = |v: f64| Some(HeatmapGridCell::new(v, format!("{v:.0}%")));
/// let rows = vec![
///     HeatmapGridRow::new("Oct 27", vec![pct(41.0), pct(33.0), pct(29.0)]).with_leading(["214"]),
///     HeatmapGridRow::new("Nov 3", vec![pct(44.0), pct(35.0), None]).with_leading(["238"]),
///     HeatmapGridRow::new("Nov 10", vec![pct(39.0), None, None]).with_leading(["196"]),
/// ];
/// view! {
///     <div data-testid="heatmap-grid-preview">
///         <HeatmapGrid
///             row_header="Cohort week"
///             leading_columns=vec!["Accounts".to_string()]
///             columns=vec!["Week 1".to_string(), "Week 2".to_string(), "Week 3".to_string()]
///             rows=rows
///             aria_label="Weekly retention"
///         />
///     </div>
/// }
/// ```
///
/// ## Fixed scale
/// `value_min` and `value_max` pin the shading to 0–100, so 95% reads darker than 12% no matter what else is in the grid. `empty_label` replaces "not yet".
/// <!-- preview -->
/// ```rust
/// use crate::{HeatmapGrid, HeatmapGridCell, HeatmapGridRow};
/// let pct = |v: f64| Some(HeatmapGridCell::new(v, format!("{v:.0}%")));
/// let rows = vec![
///     HeatmapGridRow::new("eu-west", vec![pct(95.0), pct(71.0), pct(48.0)]),
///     HeatmapGridRow::new("us-east", vec![pct(62.0), pct(30.0), None]),
///     HeatmapGridRow::new("ap-south", vec![pct(12.0), None, pct(5.0)]),
/// ];
/// view! {
///     <div data-testid="heatmap-grid-domain">
///         <HeatmapGrid
///             row_header="Region"
///             columns=vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string()]
///             rows=rows
///             value_min=0.0
///             value_max=100.0
///             empty_label="no data"
///             aria_label="Uptime by region"
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "heatmap-grid",
    preview_label = "Heatmap Grid",
    preview_icon = icondata::AiTableOutlined,
)]
#[component]
pub fn HeatmapGrid(
    /// Header of the row-label column, for example "Cohort week".
    #[prop(into)]
    row_header: String,
    /// Headers of the shaded value columns, in order.
    #[prop(into)]
    columns: Signal<Vec<String>>,
    /// Rows in order; each supplies one cell per value column (missing cells read as empty).
    #[prop(into)]
    rows: Signal<Vec<HeatmapGridRow>>,
    /// Headers of unshaded columns between the row label and the value columns. Default none.
    #[prop(optional, into)]
    leading_columns: Signal<Vec<String>>,
    /// Value shaded lightest; defaults to the smallest cell value.
    #[prop(optional)]
    value_min: Option<f64>,
    /// Value shaded darkest; defaults to the largest cell value.
    #[prop(optional)]
    value_max: Option<f64>,
    /// Text in a cell with no value. Default "not yet".
    #[prop(optional, into)]
    empty_label: MaybeProp<String>,
    /// Accessible name for the table.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Extra CSS class names merged onto the scrolling root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-heatmap-grid", heatmap_grid_styles());

    let domain =
        Memo::new(move |_| rows.with(|rows| heatmap_grid_domain(rows, value_min, value_max)));
    let empty_text = move || empty_label.get().unwrap_or_else(|| "not yet".into());
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-heatmap-grid {extra}"),
        _ => "orbital-heatmap-grid".to_string(),
    };

    view! {
        <div class=root_class>
            <table class="orbital-heatmap-grid__table" aria-label=move || aria_label.get()>
                <thead>
                    <tr>
                        <th scope="col">{row_header}</th>
                        {move || {
                            leading_columns
                                .get()
                                .into_iter()
                                .chain(columns.get())
                                .map(|header| view! { <th scope="col">{header}</th> })
                                .collect_view()
                        }}
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let column_count = columns.with(Vec::len);
                        let leading_count = leading_columns.with(Vec::len);
                        let domain = domain.get();
                        rows.get()
                            .into_iter()
                            .map(|row| {
                                let leading = (0..leading_count)
                                    .map(|i| {
                                        let text = row.leading.get(i).cloned().unwrap_or_default();
                                        view! { <td class="orbital-heatmap-grid__leading">{text}</td> }
                                    })
                                    .collect_view();
                                let cells = (0..column_count)
                                    .map(|i| match row.cells.get(i).cloned().flatten() {
                                        Some(cell) => {
                                            let level = heatmap_grid_level(cell.value, domain);
                                            view! {
                                                <td class="orbital-heatmap-grid__cell" data-level=level>
                                                    {cell.label}
                                                </td>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! {
                                                <td class="orbital-heatmap-grid__cell orbital-heatmap-grid__cell--empty">
                                                    {empty_text()}
                                                </td>
                                            }
                                                .into_any()
                                        }
                                    })
                                    .collect_view();
                                view! {
                                    <tr>
                                        <th scope="row">{row.label}</th>
                                        {leading}
                                        {cells}
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </div>
    }
}
