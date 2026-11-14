use leptos::prelude::*;
use orbital_macros::component_doc;

use super::styles::card_content_styles;

/// Body content region inside a card.
///
/// Use between header or media slots and the footer for primary copy. When it opens the card
/// (no [`CardHeader`](crate::CardHeader), media or preview above it) it is padded on every
/// side; below another slot it drops its top padding and the card's gap spaces the two.
/// Override padding via Turf `class` setting `--orbital-card-content-padding`.
///
/// # Examples
///
/// ## Default body
/// Body copy in a card with no header: padded on every side.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent};
/// view! {
///     <div data-testid="card-content-preview">
///         <div data-testid="card-content-default" style="max-width: 360px;">
///             <Card>
///                 <CardContent>"Card body copy."</CardContent>
///             </Card>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Stacked below media
/// Body region below a media hero and above footer actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardFooter, CardMedia, Button};
/// view! {
///     <div data-testid="card-content-stacked" style="max-width: 360px;">
///         <Card>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///             />
///             <CardContent>"Body below the hero image."</CardContent>
///             <CardFooter>
///                 <Button>"Action"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-content",
    preview_label = "Card Content",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardContent(
    /// Extra CSS class names merged onto the body region. Override padding via Turf class setting `--orbital-card-content-padding`.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Primary card body copy and nested content between header or media slots and the footer.
    children: Children,
) -> impl IntoView {
    let style_sheet = card_content_styles();
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card-content".to_string()
        } else {
            format!("orbital-card-content {extra}")
        }
    });

    view! {
        <style>{style_sheet}</style>
        <div class=root_class>
            {children()}
        </div>
    }
}
