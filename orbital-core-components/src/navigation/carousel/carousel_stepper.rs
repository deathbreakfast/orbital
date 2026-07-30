use icondata::AiLeftOutlined;
use icondata::AiRightOutlined;
use leptos::prelude::*;
use orbital_base_components::{BaseCarouselStepper, CarouselStateInjection, CarouselStepperLayout};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::Icon;

use super::styles::carousel_styles;

/// `CarouselStepper` adds prev/next controls and slide indicators to a parent [`Carousel`].
/// Place it inside the carousel tree; choose `layout=Bottom` for a row under slides or
/// `layout=Inline` to overlay the viewport. Override dots and arrows with custom `children`
/// when defaults do not match your design.
///
/// # When to use
///
/// - Default stepping UI when you do not need custom carousel chrome - Bottom-aligned controls under the viewport (default layout) - Overlay arrows on the viewport sides and dot indicators along the bottom via `layout=CarouselStepperLayout::Inline`
///
/// # Usage
///
/// 1. Place inside a [`Carousel`] after [`CarouselViewport`]. 2. Choose `layout` for bottom or inline overlay positioning. 3. Override with custom children only when you need fully hand-built controls.
///
/// # Best Practices
///
/// ## Do's
///
/// * Pair with [`CarouselViewport`] and [`CarouselSlide`] children in the same [`Carousel`] root * Use inline layout when the viewport should remain full-bleed with overlay arrows * Rely on the built-in indicators so users can jump directly to a slide
///
/// ## Don'ts
///
/// * Do not mount outside a [`Carousel`] — it requires [`CarouselStateInjection`] * Do not duplicate prev/next buttons elsewhere unless you disable the stepper
///
/// # Examples
///
/// ## Default stepper
/// Bottom layout with previous/next buttons and page indicators.
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// view! {
///     <div data-testid="carousel-stepper-preview">
///         <Carousel default_active_index=0 wrap=true>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <p class="orbital-carousel__slide-eyebrow">"Featured"</p>
///                         <h3 class="orbital-carousel__slide-title">"Workspace overview"</h3>
///                         <p class="orbital-carousel__slide-body">"Step through highlights of the dashboard experience."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--neutral">
///                         <p class="orbital-carousel__slide-eyebrow">"Collaboration"</p>
///                         <h3 class="orbital-carousel__slide-title">"Shared projects"</h3>
///                         <p class="orbital-carousel__slide-body">"Keep teams aligned with a single source of truth."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper />
///         </Carousel>
///     </div>
/// }
/// ```
///
/// ## Inline overlay stepper
/// Arrows overlay the viewport sides; dot indicators sit along the bottom edge—useful for hero banners and image-forward carousels.
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// use orbital_base_components::CarouselStepperLayout;
/// view! {
///     <div data-testid="carousel-stepper-inline">
///         <Carousel default_active_index=0 wrap=true>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--accent">
///                         <p class="orbital-carousel__slide-eyebrow">"Campaign"</p>
///                         <h3 class="orbital-carousel__slide-title">"Summer release"</h3>
///                         <p class="orbital-carousel__slide-body">"Inline steppers keep focus on the slide artwork."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <p class="orbital-carousel__slide-eyebrow">"Update"</p>
///                         <h3 class="orbital-carousel__slide-title">"What's new"</h3>
///                         <p class="orbital-carousel__slide-body">"Navigate without leaving the hero canvas."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper layout=CarouselStepperLayout::Inline />
///         </Carousel>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "carousel-stepper",
    preview_label = "Carousel Stepper",
    preview_icon = icondata::AiCaretLeftOutlined,
)]
#[component]
pub fn CarouselStepper(
    /// Optional CSS class on the stepper element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Layout of controls (`inline` overlays the viewport; `bottom` sits beneath it).
    #[prop(optional, default = CarouselStepperLayout::Bottom)]
    layout: CarouselStepperLayout,
    /// Optional custom stepper button children.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-carousel", carousel_styles());

    let state = CarouselStateInjection::expect_context();
    let slide_count = state.slide_count_signal();
    let active_index = state.active_index_signal();
    let layout = Signal::derive(move || layout);

    view! {
        <BaseCarouselStepper class=class layout=layout>
            {children.map(|render| render())}
            {move || {
                let count = slide_count.get();
                if count <= 0 {
                    return leptos::either::Either::Left(view! { <span></span> });
                }

                leptos::either::Either::Right(view! {
                    <>
                        <button
                            type="button"
                            class="orbital-carousel__stepper-button"
                            aria-label="Previous slide"
                            on:click=move |_| state.prev()
                        >
                            <Icon icon=AiLeftOutlined />
                        </button>
                        <div class="orbital-carousel__indicators" role="tablist">
                            {(0..count).map(|index| {
                                let is_active = move || active_index.get() == index;
                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            if is_active() {
                                                "orbital-carousel__indicator orbital-carousel__indicator--active"
                                            } else {
                                                "orbital-carousel__indicator"
                                            }
                                        }
                                        role="tab"
                                        aria-selected=move || is_active().to_string()
                                        aria-label=format!("Go to slide {}", index + 1)
                                        on:click=move |_| state.go_to(index)
                                    />
                                }
                            }).collect_view()}
                        </div>
                        <button
                            type="button"
                            class="orbital-carousel__stepper-button"
                            aria-label="Next slide"
                            on:click=move |_| state.next()
                        >
                            <Icon icon=AiRightOutlined />
                        </button>
                    </>
                })
            }}
        </BaseCarouselStepper>
    }
}
