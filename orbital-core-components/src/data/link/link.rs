use leptos::prelude::*;
use orbital_base_components::BaseLink;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::link_styles;

/// Text colour of a [`Link`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkAppearance {
    /// Brand link colour.
    #[default]
    Default,
    /// Primary text colour, underlined on hover.
    Subtle,
}

/// Styled anchor for navigation and inline references.
///
/// With `href`, renders an anchor; without `href`, renders a button (or `span` when `span` is true). Use `inline` when the link sits in body copy. For in-page section scroll-spy rails use [`Anchor`](crate::Anchor) — not `Link`.
///
/// # When to use
///
/// - In-app navigation, external URLs, and lightweight inline actions - Inline references within paragraphs and help text - Disabled links that remain focusable for explanatory tooltips
///
/// # Usage
///
/// 1. Pass `href="#path"` for anchor navigation. 2. Set `inline=true` for underlined links embedded in copy. 3. Set `disabled=true` to dim and block interaction. 4. Omit `href` for button-style links that trigger actions. 5. Set `appearance=LinkAppearance::Subtle` to draw the link in the primary text colour, for names and titles on tinted cards where the brand link colour is too light.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use descriptive link text — avoid "click here" * Set `inline` for links inside paragraphs * Provide `aria-disabled` is handled when `disabled` is set
///
/// ## Don'ts
///
/// * Do not use links for primary actions — prefer [`Button`](crate::Button) * Do not rely on color alone to distinguish links from body text in inline mode
///
/// # Examples
///
/// ## Default link
/// Standard anchor link for in-app or external navigation.
/// <!-- preview -->
/// ```rust
/// use crate::Link;
/// view! {
///     <div data-testid="link-preview">
///         <Link href="#docs">"View documentation"</Link>
///     </div>
/// }
/// ```
///
/// ## Inline link
/// Inline styling keeps links visually integrated with surrounding paragraph text.
/// <!-- preview -->
/// ```rust
/// use crate::Link;
/// view! {
///     <div data-testid="link-inline">
///         <p>
///             "Read the "
///             <Link href="#guide" inline=true>"setup guide"</Link>
///             " before continuing."
///         </p>
///     </div>
/// }
/// ```
///
/// ## Disabled link
/// Disabled links remain visible but cannot be activated.
/// <!-- preview -->
/// ```rust
/// use crate::Link;
/// view! {
///     <div data-testid="link-disabled">
///         <Link href="#locked" disabled=true>"Unavailable section"</Link>
///     </div>
/// }
/// ```
///
/// ## External href
/// Absolute URLs render as standard anchors with the provided destination.
/// <!-- preview -->
/// ```rust
/// use crate::Link;
/// view! {
///     <div data-testid="link-external">
///         <Link href="https://example.com/docs">"External docs"</Link>
///     </div>
/// }
/// ```
///
/// ## Theme link color
/// Link foreground uses brand link tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::Link;
/// view! {
///     <div data-testid="link-theme">
///         <Link href="#theme">"Themed link"</Link>
///     </div>
/// }
/// ```
///
/// ## Subtle link
/// Subtle links use the primary text colour and underline on hover, so they stay readable on grey or tinted surfaces.
/// <!-- preview -->
/// ```rust
/// use crate::{Link, LinkAppearance};
/// view! {
///     <div data-testid="link-subtle" style="padding: 8px; background: var(--orb-color-surface-subtle);">
///         <Link href="#profile" appearance=LinkAppearance::Subtle>"Ada Lovelace"</Link>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "link",
    preview_label = "Link",
    preview_icon = icondata::AiLinkOutlined,
)]
#[component]
pub fn Link(
    /// Optional CSS class on the link element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Destination URL; omit for button-style links.
    #[prop(optional, into)]
    href: Option<String>,
    /// When true, renders a `<span>` instead of anchor or button.
    #[prop(optional)]
    span: bool,
    /// Inline styling for links embedded in body copy.
    #[prop(optional)]
    inline: bool,
    /// Disables interaction and dims styling.
    #[prop(optional)]
    disabled: bool,
    /// Keeps focusable styling when disabled (for tooltips on disabled links).
    #[prop(optional)]
    disabled_focusable: bool,
    /// Text colour: `Default` uses the brand link colour, `Subtle` the primary text colour.
    #[prop(optional)]
    appearance: LinkAppearance,
    /// Link label or inline elements.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-link", link_styles());

    let class = MaybeProp::derive(move || {
        let extra = class.get();
        match appearance {
            LinkAppearance::Default => extra,
            LinkAppearance::Subtle => Some(match extra {
                Some(extra) => format!("orbital-link--subtle {extra}"),
                None => "orbital-link--subtle".to_string(),
            }),
        }
    });

    let href = href.map(Signal::<String>::from);
    let inline = Signal::from(inline);
    let disabled = Signal::from(disabled);
    let disabled_focusable = Signal::from(disabled_focusable);

    view! {
        <BaseLink
            class=class
            span=span
            inline=inline
            disabled=disabled
            disabled_focusable=disabled_focusable
            nostrip:href=href
        >
            {children()}
        </BaseLink>
    }
}
