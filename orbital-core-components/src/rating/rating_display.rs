use leptos::prelude::*;
use orbital_base_components::{RatingColor, RatingSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::injection::RatingInjection;
use super::rating_item::RatingItem;
use super::styles::rating_styles;

/// Non-interactive star rating for reviews and aggregates.
///
/// Orbital RatingDisplay control. `value` is shown as filled stars plus numeric text. Items render with `interactive=false` so no radio inputs appear.
///
/// # Examples
///
/// ## Default
/// Read-only four-star display with value label — use in reviews and list metadata.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="rating-display-preview">
///         <RatingDisplay value=4.0 />
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Size tokens scale icons for dense tables versus spacious detail headers.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, RatingDisplay, RatingSize};
/// view! {
///     <div data-testid="rating-display-sizes">
///         <Flex gap=FlexGap::Large align=FlexAlign::Center>
///             <RatingDisplay value=3.5 size=RatingSize::Small />
///             <RatingDisplay value=3.5 size=RatingSize::Medium />
///             <RatingDisplay value=3.5 size=RatingSize::Large />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Colors
/// Brand, marigold, and neutral presets for marketing versus subdued contexts.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, RatingColor, RatingDisplay};
/// view! {
///     <div data-testid="rating-display-colors">
///         <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Start>
///             <RatingDisplay value=4.0 color=RatingColor::Brand />
///             <RatingDisplay value=4.0 color=RatingColor::Marigold />
///             <RatingDisplay value=4.0 color=RatingColor::Neutral />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Half value
/// Fractional scores render half-filled stars when the value includes a `.5` step.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="rating-display-half">
///         <RatingDisplay value=3.5 />
///     </div>
/// }
/// ```
///
/// ## Custom max
/// Increase `max` when displaying scores on a scale larger than five stars.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="rating-display-max">
///         <RatingDisplay value=8.0 max=10 />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "rating-display",
    preview_label = "Rating Display",
    preview_icon = icondata::AiStarOutlined,
)]
#[component]
pub fn RatingDisplay(
    /// Optional CSS class on the display root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Displayed rating value.
    #[prop(optional, into)]
    value: Signal<f32>,
    /// Number of star slots (minimum 2).
    #[prop(default = 5.into(), into)]
    max: Signal<u8>,
    /// Icon size preset.
    #[prop(default = RatingSize::Medium.into(), into)]
    size: Signal<RatingSize>,
    /// Color preset.
    #[prop(optional, into)]
    color: Signal<RatingColor>,
) -> impl IntoView {
    inject_style("orbital-rating", rating_styles());

    let bound_value = RwSignal::new(Some(value.get_untracked()));
    Effect::new(move |_| {
        bound_value.set(Some(value.get()));
    });

    let class = MaybeProp::derive(move || {
        let mut parts = vec![
            "orbital-rating-display".to_string(),
            format!("orbital-rating-display--{}", size.get().as_str()),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <div role="img" class=move || class.get().unwrap_or_default()>
            <leptos::context::Provider value=RatingInjection {
                value: bound_value.into(),
                hovered_value: RwSignal::new(None::<f32>),
                name: Signal::from(String::new()),
                step: 0.5.into(),
                size,
                color,
                interactive: false,
            }>
                {move || {
                    let mut max = max.get();
                    if max < 2 {
                        max = 2;
                    }
                    (0..max)
                        .map(|i| view! { <RatingItem value=i + 1 /> })
                        .collect_view()
                }}
            </leptos::context::Provider>
            <span aria-hidden="true" class="orbital-rating-display__value-text">
                {move || value.get()}
            </span>
        </div>
    }
}
