//! [`FunnelChart`] root component.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::funnel_chart_styles;
use crate::engine::{compute_funnel_layout, format_share};
use crate::FunnelStep;

/// Show how many people get through each step of a sequence, and where they stop.
///
/// Each step gets one bar, sized by its count against the first step, with the
/// count and that share printed on the bar. Under every bar after the first, a
/// line says how much of the step before stopped there. The chart is plain HTML
/// that fills its container's width.
///
/// # When to use
///
/// - Sign-up, checkout or onboarding funnels where each step is a subset of the one before.
/// - Query results that count subjects reaching ordered events within a window.
/// - Unordered categories fit [`crate::BarChart`] better; trends over time fit [`crate::LineChart`].
///
/// # Usage
///
/// 1. Build a `Vec<`[`FunnelStep`]`>` in order, first step first, with each step's label and count.
/// 2. Pass it as `steps` (a plain `Vec` or a signal when results load later).
/// 3. Set `aria_label` to name the funnel for screen readers.
/// 4. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// The steps are an ordered list, so screen readers announce the step count and
/// read each label, count, share and drop-off as text. Nothing depends on colour
/// or bar length alone, and the chart has no tab stops.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep step labels short and say what was counted, such as "First checkout within 7 days".
/// * Pair the chart with a table view when people need to export or sort the numbers.
///
/// ## Don'ts
///
/// * Do not use a funnel for steps that are not subsets of each other.
/// * Do not put `data-testid` on the component itself; wrap it in a native element.
///
/// # Examples
///
/// ## Three-step funnel
/// Sign-in, then a first and second checkout. Bars shrink with each step's share of the first, and the drop-off sits under each later bar.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{FunnelChart, FunnelStep};
/// let steps = vec![
///     FunnelStep::new("Signed in", 4120),
///     FunnelStep::new("First checkout within 7 days", 1566),
///     FunnelStep::new("Second checkout within 7 days", 402),
/// ];
/// view! {
///     <div data-testid="funnel-chart-preview">
///         <FunnelChart steps=steps aria_label="Checkout funnel" />
///     </div>
/// }
/// ```
///
/// ## Many steps
/// Five steps with a tiny last step, which reads `<1%`. A bar never gets narrower than its own label, so small shares stay readable.
/// <!-- preview -->
/// ```rust
/// use crate::{FunnelChart, FunnelStep};
/// let steps = vec![
///     FunnelStep::new("Visited pricing", 18_400),
///     FunnelStep::new("Started trial", 6_210),
///     FunnelStep::new("Invited a teammate", 2_380),
///     FunnelStep::new("Created a dashboard", 1_150),
///     FunnelStep::new("Upgraded", 40),
/// ];
/// view! {
///     <div data-testid="funnel-chart-many-steps">
///         <FunnelChart steps=steps aria_label="Trial funnel" />
///     </div>
/// }
/// ```
///
/// ## No one at the first step
/// When the first step counts zero every share is 0% and no drop-off is shown, instead of dividing by zero.
/// <!-- preview -->
/// ```rust
/// use crate::{FunnelChart, FunnelStep};
/// let steps = vec![FunnelStep::new("Signed in", 0), FunnelStep::new("First checkout", 0)];
/// view! {
///     <div data-testid="funnel-chart-empty">
///         <FunnelChart steps=steps aria_label="Funnel with no data" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "funnel-chart",
    preview_label = "Funnel Chart",
    preview_icon = icondata::AiFunnelPlotOutlined,
)]
#[component]
pub fn FunnelChart(
    /// Steps in order, first step first; each bar is sized against the first step's count.
    #[prop(into)]
    steps: Signal<Vec<FunnelStep>>,
    /// Accessible name for the list of steps.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Text shown when there are no steps. Default "No steps to show".
    #[prop(optional, into)]
    empty_label: MaybeProp<String>,
    /// Extra CSS class names merged onto the chart root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-funnel-chart", funnel_chart_styles());

    let layout = Memo::new(move |_| steps.with(|steps| compute_funnel_layout(steps)));
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-funnel-chart {extra}"),
        _ => "orbital-funnel-chart".to_string(),
    };

    view! {
        <div class=root_class>
            <Show
                when=move || layout.with(|l| !l.is_empty())
                fallback=move || {
                    view! {
                        <p class="orbital-funnel-chart__empty">
                            {move || empty_label.get().unwrap_or_else(|| "No steps to show".into())}
                        </p>
                    }
                }
            >
                <ol class="orbital-funnel-chart__steps" aria-label=move || aria_label.get()>
                    {move || {
                        layout
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, step)| {
                                let drop_off = step
                                    .drop_off
                                    .map(|d| {
                                        let share = format_share(d);
                                        if index == 1 {
                                            format!("{share} stopped here")
                                        } else {
                                            format!("{share} of the step before stopped here")
                                        }
                                    });
                                view! {
                                    <li class="orbital-funnel-chart__step" data-step=index>
                                        <span class="orbital-funnel-chart__label">{step.label}</span>
                                        <div class="orbital-funnel-chart__track">
                                            <div
                                                class="orbital-funnel-chart__bar"
                                                style=format!("width: {:.2}%", step.width * 100.0)
                                            >
                                                {format!(
                                                    "{} · {}",
                                                    step.count_label,
                                                    format_share(step.share),
                                                )}
                                            </div>
                                            {drop_off
                                                .map(|text| {
                                                    view! {
                                                        <span class="orbital-funnel-chart__drop-off">
                                                            {text}
                                                        </span>
                                                    }
                                                })}
                                        </div>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ol>
            </Show>
        </div>
    }
}
