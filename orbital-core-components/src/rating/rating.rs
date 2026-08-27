use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::HtmlInputElement;
use leptos::{ev, prelude::*};
use orbital_base_components::{
    new_field_id, FieldInjection, OptionBind, RatingColor, RatingRule, RatingRuleTrigger,
    RatingSize, Rule,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::injection::RatingInjection;
use super::rating_item::RatingItem;
use super::styles::rating_styles;

/// Collects star feedback through an accessible radiogroup.
///
/// Bind `value` with [`OptionBind`](orbital_base_components::OptionBind), set `max` and `step` (`0.5` for half stars), and pair with [`Field`](crate::Field) plus `rules` for validation. For read-only summaries use [`RatingDisplay`](crate::RatingDisplay).
///
/// # Examples
///
/// ## Default rating
/// Five-star interactive rating with brand color — bind `value` to persisted state.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(Some(3.0f32));
/// view! {
///     <div data-testid="rating-preview">
///         <Rating value=value />
///     </div>
/// }
/// ```
///
/// ## Half steps
/// `step=0.5` enables half-star hover and selection for finer granularity.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(Some(2.5f32));
/// view! {
///     <div data-testid="rating-half">
///         <Rating value=value step=0.5 />
///     </div>
/// }
/// ```
///
/// ## Colors
/// Brand, marigold, and neutral presets for marketing versus subdued contexts.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, Rating, RatingColor};
/// let value = RwSignal::new(Some(4.0f32));
/// view! {
///     <div data-testid="rating-colors">
///         <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Start>
///             <Rating value=value color=RatingColor::Brand />
///             <Rating value=value color=RatingColor::Marigold />
///             <Rating value=value color=RatingColor::Neutral />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Size tokens scale icons for dense tables versus spacious detail headers.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexAlign, FlexGap, Rating, RatingSize};
/// let value = RwSignal::new(Some(3.0f32));
/// view! {
///     <div data-testid="rating-sizes">
///         <Flex gap=FlexGap::Large align=FlexAlign::Center>
///             <Rating value=value size=RatingSize::Small />
///             <Rating value=value size=RatingSize::Medium />
///             <Rating value=value size=RatingSize::Large />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Required validation inside Field
/// Field shows the label and required indicator; `RatingRule::required` drives validation text below the control.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Rating, RatingRule};
/// let value = RwSignal::new(None::<f32>);
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="rating-validation">
///         <Field label="Quality" name="quality" required=true>
///             <Rating
///                 value=value
///                 rules=vec![RatingRule::required(required)]
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Custom max
/// Increase `max` when more than five levels are needed (minimum 2).
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(Some(7.0f32));
/// view! {
///     <div data-testid="rating-max">
///         <Rating value=value max=10 />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "rating",
    preview_label = "Rating",
    preview_icon = icondata::AiStarOutlined,
)]
#[component]
pub fn Rating(
    /// Optional CSS class on the radiogroup root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional element id (pairs with [`Field`](crate::Field) validation).
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Validation rules run on change.
    #[prop(optional, into)]
    rules: Vec<RatingRule>,
    /// Radio group `name`; auto-generated when omitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Current rating value (`None` when unset).
    #[prop(optional, into)]
    value: OptionBind<f32>,
    /// Number of stars (minimum 2).
    #[prop(default = 5.into(), into)]
    max: Signal<u8>,
    /// Step size: `1.0` for whole stars, `0.5` for half stars.
    #[prop(default = 1.0.into(), into)]
    step: Signal<f32>,
    /// Icon size preset.
    #[prop(default = RatingSize::ExtraLarge.into(), into)]
    size: Signal<RatingSize>,
    /// Color preset (`Brand`, `Marigold`, `Neutral`).
    #[prop(optional, into)]
    color: Signal<RatingColor>,
) -> impl IntoView {
    inject_style("orbital-rating", rating_styles());

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    let fallback_name = StoredValue::new(new_field_id());
    let resolved_name =
        Signal::derive(move || name.get().unwrap_or_else(|| fallback_name.get_value()));
    let hovered_value = RwSignal::new(None::<f32>);

    let on_change = move |ev: ev::Event| {
        let Some(target) = ev.target() else {
            return;
        };
        let Ok(el) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        if el.type_() != "radio" || el.name() != resolved_name.get() {
            return;
        }
        if let Ok(new_value) = el.value().parse::<f32>() {
            value.with_value(|value| value.set(Some(new_value)));
            validate.run(Some(RatingRuleTrigger::Change));
        }
    };

    let on_mouseover = move |ev: ev::MouseEvent| {
        let Some(target) = ev.target() else {
            return;
        };
        let Ok(el) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        if el.type_() != "radio" || el.name() != resolved_name.get() {
            return;
        }
        if let Ok(new_value) = el.value().parse::<f32>() {
            hovered_value.set(Some(new_value));
        }
    };

    let on_mouseleave = move |_| {
        hovered_value.set(None);
    };

    let class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-rating".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <div
            role="radiogroup"
            class=move || class.get().unwrap_or_default()
            id=id
            on:change=on_change
            on:mouseover=on_mouseover
            on:mouseleave=on_mouseleave
        >
            <leptos::context::Provider value=RatingInjection {
                value: value.get_value(),
                hovered_value,
                name: resolved_name,
                step,
                size,
                color,
                interactive: true,
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
        </div>
    }
}
