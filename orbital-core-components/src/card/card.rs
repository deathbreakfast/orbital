use leptos::prelude::*;

use crate::{Flex, FlexGap, Material, MaterialCorners, MaterialElevation, MaterialVariant};
use orbital_macros::component_doc;

use super::styles::card_layout_styles;

/// Groups related content on an Orbital [`Material`] surface.
///
/// # Usage
///
/// Stack parts in document order:
/// 1. [`CardHeader`] — title, description, and optional header action
/// 2. [`CardMedia`] or [`CardPreview`] — hero image or preview region
/// 3. [`CardContent`] — body copy
/// 4. [`CardButtonArea`] — optional whole-card primary action region
/// 5. [`CardFooter`] — independent secondary actions
///
/// Insert [`CardSectionBorder`] between stacked slots when a flush horizontal rule is needed inside the card. Keep spacing on the neighboring slots; the border itself is a zero-padding rule.
///
/// **CardButtonArea vs CardFooter:** Wrap the hero and body in [`CardButtonArea`] when the **entire card** navigates or opens details — one primary click target. Keep footer actions in [`CardFooter`] **outside** the button area so secondary commands (Reply, Share, Cancel) do not share the card-level click handler.
///
/// # Choosing a surface
///
/// - **Outlined (default):** static content such as detail panels, settings, forms and
///   dashboard tiles. A thin border, no shadow, so dense pages stay calm.
/// - **Elevated:** `variant=MaterialVariant::Solid` (resting shadow) or a higher
///   `elevation` for cards that are clickable, draggable, primary or floating above other
///   content. The shadow signals that the card lifts or can be acted on.
/// - **Flat:** a card inside another card renders flat with no border unless it sets
///   `variant` or `elevation`, so a page never stacks a box inside a box or a shadow
///   inside a shadow.
///
/// # Examples
///
/// ## Default (body only)
/// Outlined card with body content. With no header above it, [`CardContent`] is padded on
/// every side.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent};
/// view! {
///     <div data-testid="card-preview" style="max-width: 360px;">
///         <Card>
///             <CardContent>"Card body content"</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Full compound
/// Header, image hero, body, and footer actions in one card.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Button, ButtonAppearance, Card, CardContent, CardFooter, CardHeader,
///     CardHeaderAction, CardHeaderDescription, CardMedia, Subtitle1,
/// };
/// view! {
///     <div data-testid="card-compound-preview" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Team workspace"</Subtitle1>
///                 <CardHeaderDescription slot>
///                     <span>"Shared files and tasks"</span>
///                 </CardHeaderDescription>
///                 <CardHeaderAction slot>
///                     <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
///                 </CardHeaderAction>
///             </CardHeader>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///                 height=140
///             />
///             <CardContent>"Card body copy."</CardContent>
///             <CardFooter>
///                 <Button>"Reply"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Share"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## With header
/// Title header and body content regions.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardHeader, Subtitle1};
/// view! {
///     <div data-testid="card-with-header" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Overview"</Subtitle1>
///             </CardHeader>
///             <CardContent>"Metrics and summary content."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Media card
/// Photo hero with body copy and footer actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Card, CardContent, CardFooter, CardMedia};
/// view! {
///     <div data-testid="card-media-preview" style="max-width: 360px;">
///         <Card>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///                 height=140
///             />
///             <CardContent>"Lizards are a widespread group of squamate reptiles."</CardContent>
///             <CardFooter>
///                 <Button>"Share"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Learn More"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Interactive media card
/// Primary click region wraps media and body; footer actions stay outside.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::{Button, Card, CardButtonArea, CardContent, CardFooter, CardMedia};
/// let clicked = RwSignal::new(false);
/// view! {
///     <div data-testid="card-button-area-preview" style="max-width: 360px;">
///         <Card>
///             <CardButtonArea on_click=Callback::new(move |_| {
///                 clicked.set(true);
///                 #[cfg(target_arch = "wasm32")]
///                 if let Some(window) = web_sys::window() {
///                     let _ = window.alert_with_message("Card clicked!");
///                 }
///             })>
///                 <CardMedia
///                     src="https://picsum.photos/seed/orbital-card/360/140"
///                     alt="Sample card illustration"
///                     height=140
///                 />
///                 <CardContent>
///                     {move || if clicked.get() { "Card clicked!" } else { "Tap the card body to open details." }}
///                 </CardContent>
///             </CardButtonArea>
///             <CardFooter>
///                 <Button on:click=|ev| { ev.stop_propagation(); }>"Share"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Custom hero
/// Arbitrary preview content above the card body.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardPreview};
/// view! {
///     <div data-testid="card-custom-hero-preview" style="max-width: 360px;">
///         <Card>
///             <CardPreview>
///                 <div style="height: 120px; background: var(--orb-color-surface-subtle); display: flex; align-items: center; justify-content: center;">
///                     "Custom hero"
///                 </div>
///             </CardPreview>
///             <CardContent>"Body below custom hero."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Raised elevation
/// Emphasized solid surface using raised elevation.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, MaterialElevation, MaterialVariant};
/// view! {
///     <div data-testid="card-raised" style="max-width: 360px;">
///         <Card variant=MaterialVariant::Solid elevation=MaterialElevation::Raised>
///             <CardContent>"Hero card content"</CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Card inside a card
/// The inner card sets nothing, so it renders flat with no border inside the outlined one.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardHeader, Subtitle1};
/// view! {
///     <div data-testid="card-nested" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Billing revamp"</Subtitle1>
///             </CardHeader>
///             <CardContent>
///                 <Card>
///                     <CardContent>"Nested summary"</CardContent>
///                 </Card>
///             </CardContent>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Solid surface
/// Borderless solid surface with a resting shadow, the look cards had before outlined became
/// the default.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, MaterialVariant};
/// view! {
///     <div data-testid="card-solid" style="max-width: 360px;">
///         <Card variant=MaterialVariant::Solid>
///             <CardContent>"Solid card content"</CardContent>
///         </Card>
///     </div>
/// }
/// ```
/// Context marker: `true` while rendering inside a [`Card`].
#[derive(Clone, Copy)]
struct InsideCard(bool);

/// Renders `children` as if they were not inside a card, so their cards take the top-level
/// surface again. For frames that happen to be cards themselves, such as the preview
/// catalog's example frame.
#[component]
pub(crate) fn CardNestingBoundary(children: Children) -> impl IntoView {
    provide_context(InsideCard(false));
    children()
}

/// The surface a card gets from its props and placement. Explicit props win; left unset, a
/// top-level card is outlined and flat, a card inside another card is flat with no border
/// (no box inside a box), and a solid card rests on a shadow.
fn resolve_surface(
    variant: Option<MaterialVariant>,
    elevation: Option<MaterialElevation>,
    nested: bool,
) -> (MaterialVariant, MaterialElevation) {
    match (variant, elevation) {
        // Nothing set inside another card: flat, borderless.
        (None, None) if nested => (MaterialVariant::Solid, MaterialElevation::Flat),
        (None, elevation) => (
            MaterialVariant::Outlined,
            elevation.unwrap_or(MaterialElevation::Flat),
        ),
        (Some(variant), elevation) => (
            variant,
            elevation.unwrap_or(match variant {
                MaterialVariant::Solid => MaterialElevation::Resting,
                _ => MaterialElevation::Flat,
            }),
        ),
    }
}

#[component_doc(
    category = "Surfaces",
    preview_slug = "card",
    preview_label = "Card",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component]
pub fn Card(
    /// Optional CSS class merged onto the Material root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Surface material treatment forwarded to [`Material`]. Left unset: outlined (a thin
    /// border), or flat with no border when this card sits inside another [`Card`]. Pass
    /// [`MaterialVariant::Solid`] for an elevated card (clickable, primary or floating).
    #[prop(optional)]
    variant: Option<MaterialVariant>,
    /// Surface elevation forwarded to [`Material`]. Left unset, it follows the variant:
    /// [`MaterialElevation::Flat`] for an outlined or nested card,
    /// [`MaterialElevation::Resting`] for a solid one.
    #[prop(optional)]
    elevation: Option<MaterialElevation>,
    /// Corner treatment forwarded to [`Material`].
    #[prop(default = MaterialCorners::Rounded)]
    corners: MaterialCorners,
    /// Row gap between stacked slot regions. Set [`FlexGap::Size(0)`] for flush sections separated only by [`CardSectionBorder`](crate::CardSectionBorder).
    #[prop(default = FlexGap::Medium)]
    gap: FlexGap,
    /// Compound slot regions — [`CardHeader`], [`CardMedia`], [`CardPreview`], [`CardContent`], [`CardButtonArea`], and [`CardFooter`] in document order.
    children: Children,
) -> impl IntoView {
    let (variant, elevation) = resolve_surface(
        variant,
        elevation,
        use_context::<InsideCard>().is_some_and(|c| c.0),
    );
    provide_context(InsideCard(true));
    let (layout_sheet, layout_classes) = card_layout_styles();
    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card".to_string()
        } else {
            format!("orbital-card {extra}")
        }
    });

    view! {
        <style>{layout_sheet}</style>
        <Material variant=variant elevation=elevation corners=corners class=root_class>
            <Flex vertical=true full_width=true gap=gap class=layout_classes.inner.clone()>
                {children()}
            </Flex>
        </Material>
    }
}

/// Server-render tests: `cargo test -p orbital-core-components --features ssr --lib card_ssr`.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::CardContent;
    use orbital_theme::OrbitalThemeProvider;

    fn render(view: fn() -> AnyView) -> String {
        // The workspace test build also enables leptos csr, so Effects need an executor.
        let _ = any_spawner::Executor::init_futures_executor();
        Owner::new()
            .with(|| view! { <OrbitalThemeProvider>{view()}</OrbitalThemeProvider> }.to_html())
    }

    #[test]
    fn card_ssr_default_is_outlined_and_flat() {
        let html = render(|| view! { <Card><CardContent>"Body"</CardContent></Card> }.into_any());
        assert!(
            html.contains("data-material-variant=\"outlined\""),
            "{html}"
        );
        assert!(html.contains("data-material-elevation=\"flat\""), "{html}");
    }

    #[test]
    fn card_ssr_solid_variant_keeps_a_resting_shadow() {
        let html = render(|| {
            view! { <Card variant=MaterialVariant::Solid><CardContent>"Body"</CardContent></Card> }
                .into_any()
        });
        assert!(html.contains("data-material-variant=\"solid\""), "{html}");
        assert!(
            html.contains("data-material-elevation=\"resting\""),
            "{html}"
        );
    }

    #[test]
    fn card_ssr_nested_card_is_flat_without_a_border() {
        let html = render(|| {
            view! { <Card><CardContent><Card><CardContent>"Inner"</CardContent></Card></CardContent></Card> }
                .into_any()
        });
        let outer = html
            .find("data-material-variant=\"outlined\"")
            .expect("outer card");
        let inner = html[outer + 1..]
            .find("data-material-variant=")
            .map(|i| &html[outer + 1 + i..])
            .expect("inner card");
        assert!(
            inner.starts_with("data-material-variant=\"solid\""),
            "{html}"
        );
        assert!(inner.contains("data-material-elevation=\"flat\""), "{html}");
    }

    #[test]
    fn card_ssr_explicit_elevation_wins_over_the_variant_default() {
        let html = render(|| {
            view! { <Card elevation=MaterialElevation::Raised><CardContent>"Body"</CardContent></Card> }.into_any()
        });
        assert!(
            html.contains("data-material-variant=\"outlined\""),
            "{html}"
        );
        assert!(
            html.contains("data-material-elevation=\"raised\""),
            "{html}"
        );
    }
}

#[cfg(test)]
mod surface_tests {
    use super::*;
    use MaterialElevation::{Flat, Raised, Resting};
    use MaterialVariant::{Outlined, Solid};

    #[test]
    fn top_level_card_defaults_to_outlined_and_flat() {
        assert_eq!(resolve_surface(None, None, false), (Outlined, Flat));
    }

    #[test]
    fn nested_card_with_nothing_set_is_flat_without_a_border() {
        assert_eq!(resolve_surface(None, None, true), (Solid, Flat));
    }

    #[test]
    fn explicit_props_keep_their_meaning_when_nested() {
        assert_eq!(resolve_surface(Some(Solid), None, true), (Solid, Resting));
        assert_eq!(
            resolve_surface(Some(Outlined), None, true),
            (Outlined, Flat)
        );
        assert_eq!(
            resolve_surface(None, Some(Raised), true),
            (Outlined, Raised)
        );
    }

    #[test]
    fn solid_card_rests_on_a_shadow() {
        assert_eq!(resolve_surface(Some(Solid), None, false), (Solid, Resting));
    }
}
