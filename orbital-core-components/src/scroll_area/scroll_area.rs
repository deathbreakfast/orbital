//! Theme-aware scrollport using native CSS scrollbars.

use leptos::html::Div;
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use turf::inline_style_sheet_values;

use super::styles::scroll_area_styles;

/// Scrollable region with Orbital-themed native scrollbars.
///
/// Styled **native scrolling** — not virtualization. Uses `overflow: auto` and CSS `scrollbar-*` / `::-webkit-scrollbar` rules so thumbs track design tokens (light/dark). Use in shell [`Layout`](crate::Layout) columns, [`Navigation`](crate::Navigation) bodies, and custom panels.
///
/// # When to use
///
/// - Fixed-height regions with overflowing content (sidebar nav, main columns, panels) - Any scrollport that should match Orbital theme rather than default OS chrome colors - Horizontal overflow when `horizontal=true` (wide toolbars, carousels)
///
/// # Usage
///
/// 1. Set `style` height (or width for horizontal) on `ScrollArea` so the scrollport has bounds. 2. Put overflowing content inside the default slot. 3. Set `show_scrollbar=false` to hide scrollbar chrome while keeping wheel/touch scroll. 4. Adjust `size` for scrollbar thumb/track thickness in pixels (default `8`) — not viewport size.
///
/// # Best Practices
///
/// ## Do's
///
/// * Give the scrollport explicit `height: 100%` or `flex: 1; min-height: 0` in flex layouts * Use `ScrollArea` in [`Layout`](crate::Layout) and [`Navigation`](crate::Navigation) for consistent scrollbar styling
///
/// ## Don'ts
///
/// * Do not nest `ScrollArea` inside another scrollport without clearing overflow on the parent
///
/// # Examples
///
/// ## Default vertical scroll
/// Vertical scrollport with themed thumbs on a bounded demo frame.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::ScrollArea;
/// view! {
///     <div data-testid="scroll-area-default" style="width: 100%; max-width: 560px;">
///         <ScrollArea style="display: block; width: 100%; height: 280px; box-sizing: border-box; border: 1px solid var(--orb-color-border-default); padding: 16px; background: var(--orb-color-surface-subtle);">
///             <div style="display: flex; flex-direction: column; gap: 12px; min-height: 520px;">
///                 <p>"Content is taller than the 280px scrollport."</p>
///                 <p>"Scroll with wheel, trackpad, or drag the themed scrollbar thumb."</p>
///                 {(0..12)
///                     .map(|i| view! { <p>{format!("Line {i} — extra content to ensure vertical overflow.")}</p> })
///                     .collect_view()}
///             </div>
///         </ScrollArea>
///     </div>
/// }
/// ```
///
/// ## Scrollbar visibility
/// Side-by-side: themed thumbs (default) versus hidden chrome with wheel scroll only.
/// <!-- preview -->
/// ```rust
/// use crate::{Body1Strong, Flex, FlexGap, ScrollArea};
/// const FRAME: &str = "display: block; width: 100%; height: 240px; box-sizing: border-box; border: 1px solid var(--orb-color-border-default); padding: 12px; background: var(--orb-color-surface-subtle);";
/// view! {
///     <div style="width: 100%; max-width: 560px;">
///     <Flex vertical=true gap=FlexGap::Large>
///         <div data-testid="scroll-area-visible" style="width: 100%;">
///             <Body1Strong block=true style="margin: 0 0 8px;">"Thumbs visible"</Body1Strong>
///             <ScrollArea style=FRAME>
///                 <div style="min-height: 480px; display: flex; flex-direction: column; gap: 8px;">
///                     {(0..10)
///                         .map(|i| view! { <p style="margin: 0;">{format!("Visible scrollbar line {i}")}</p> })
///                         .collect_view()}
///                 </div>
///             </ScrollArea>
///         </div>
///         <div data-testid="scroll-area-hidden" style="width: 100%;">
///             <Body1Strong block=true style="margin: 0 0 8px;">"Thumbs hidden"</Body1Strong>
///             <ScrollArea show_scrollbar=false style=FRAME>
///                 <div style="min-height: 480px; display: flex; flex-direction: column; gap: 8px;">
///                     {(0..10)
///                         .map(|i| view! { <p style="margin: 0;">{format!("Hidden scrollbar line {i}")}</p> })
///                         .collect_view()}
///                 </div>
///             </ScrollArea>
///         </div>
///     </Flex>
///     </div>
/// }
/// ```
///
/// ## Horizontal scroll
/// Bounded width with wide inner content; horizontal thumb uses the same theme tokens.
/// <!-- preview -->
/// ```rust
/// use crate::ScrollArea;
/// view! {
///     <div data-testid="scroll-area-horizontal" style="width: 100%; max-width: 560px;">
///         <ScrollArea
///             horizontal=true
///             style="display: block; width: 100%; height: 200px; box-sizing: border-box; border: 1px solid var(--orb-color-border-default); padding: 12px; background: var(--orb-color-surface-subtle);"
///         >
///             <div style="width: 900px; padding: 8px; white-space: nowrap;">
///                 "Wide content that scrolls horizontally — drag or shift+wheel to move along the row."
///             </div>
///         </ScrollArea>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "scroll-area",
    preview_label = "Scroll Area",
    preview_icon = icondata::AiSwapOutlined,
)]
#[component]
pub fn ScrollArea(
    /// Primary scroll axis when true (horizontal); default is vertical.
    #[prop(optional, default = false)]
    horizontal: bool,
    /// Show themed scrollbar thumbs (default true).
    #[prop(optional, default = true)]
    show_scrollbar: bool,
    /// Scrollbar thumb/track thickness in pixels (default 8).
    #[prop(optional, default = 8)]
    size: u8,
    /// Extra CSS class names merged onto the scrollport element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Inline CSS declarations for bounded preview frames (height, max-height, etc.).
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Optional reference to the scrollport root element (for scroll listeners / infinite scroll).
    #[prop(optional, default = NodeRef::<Div>::new())]
    node_ref: NodeRef<Div>,
    /// Optional `data-testid` for the scrollport root (not forwarded to the injected `<style>` tag).
    #[prop(optional, into)]
    scroll_testid: MaybeProp<String>,
    /// Optional `data-column-order` for consumers that track column layout on the scrollport.
    #[prop(optional, into)]
    scroll_data_column_order: MaybeProp<String>,
    /// Scrollable content inside the themed scrollbar chrome.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-scroll-area", scroll_area_styles());

    // Stable `orbital-scroll-area*` names so static HTML mocks can reuse these rules.
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-scroll-area--vertical {
            overflow: auto;
        }

        .orbital-scroll-area--horizontal {
            overflow-x: auto;
            overflow-y: hidden;
        }

        .orbital-scroll-area__content--sized {
            min-height: 0;
            min-width: 0;
        }

        .orbital-scroll-area__content--horizontal {
            min-width: max-content;
            width: max-content;
        }
    };

    let root_class = move || {
        let mut parts = vec!["orbital-scroll-area".to_string()];
        if horizontal {
            parts.push("orbital-scroll-area--horizontal".to_string());
        } else {
            parts.push("orbital-scroll-area--vertical".to_string());
        }
        if !show_scrollbar {
            parts.push("orbital-scroll-area--hide-chrome".to_string());
        }
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    };

    let root_style = move || {
        let size_var = format!("--orbital-scrollbar-size: {size}px;");
        match style.get() {
            Some(user) if !user.trim().is_empty() => format!("{size_var} {user}"),
            _ => size_var,
        }
    };

    let content_class = move || {
        if horizontal {
            "orbital-scroll-area__content orbital-scroll-area__content--sized orbital-scroll-area__content--horizontal"
                .to_string()
        } else {
            "orbital-scroll-area__content orbital-scroll-area__content--sized".to_string()
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div
            class=root_class
            style=root_style
            node_ref=node_ref
            data-testid=scroll_testid
            data-column-order=scroll_data_column_order
        >
            <div class=content_class>
                {children()}
            </div>
        </div>
    }
}
