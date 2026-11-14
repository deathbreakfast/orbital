use leptos::prelude::*;
use orbital_base_components::BaseFlex;
use orbital_macros::component_doc;

pub use orbital_base_components::{FlexAlign, FlexGap, FlexJustify, FlexWrap, SpacingInset};

/// Flexbox layout container for arranging children in a row or column.
///
/// The canonical one-dimensional layout primitive. [`Stack`](crate::Stack) and [`Space`](crate::Space) are convenience wrappers with opinionated defaults.
///
/// Orbital-only props beyond basic flex direction and gap: **`wrap`**, **`fill`** (height 100%), **`full_width`**, and token-based **`padding`** / **`margin`** via [`SpacingInset`]. Size individual children with [`FlexItem`](crate::FlexItem) (grow, shrink, basis, min-width).
///
/// # When to use
///
/// - Full control over direction, wrap, inline, fill, and inset padding - Toolbars and form rows that need alignment along both axes - Inline clusters beside text or other inline content (`inline=true`) - When [`Stack`](crate::Stack) (even-gap column) or [`Space`](crate::Space) (space-between) defaults do not fit
///
/// # Usage
///
/// 1. Choose direction: default row, or `vertical=true` for a column stack. 2. Set `gap` to a [`FlexGap`] preset or custom size — avoid margin hacks. 3. Tune `align` (cross-axis) and `justify` (main-axis) for centering or distribution. 4. Set `inline=true` when the flex container should sit in flowing text or inline UI.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `gap` instead of margin hacks between items * Set `vertical` for stacked form fields or list actions * Pair with `align` / `justify` for centering and distribution * Use `FlexGap::Size` or `FlexGap::WH` when presets do not match your spacing rhythm * Put borders, fixed heights, and backgrounds on a native wrapper `div`; use `Flex` for direction, gap, and alignment * Use `wrap`, `fill`, and `full_width` props instead of inline flex CSS on `Flex` * Use [`SpacingInset`] with [`SpacingHorizontal`] / [`SpacingVertical`] for theme-aware padding and margin * Prefer [`Stack`](crate::Stack) for simple even-gap vertical sections — `Stack` defaults to column + full-width * Prefer [`Space`](crate::Space) for opposite-edge distribution — `Space` defaults to space-between + full-width
///
/// ## Don'ts
///
/// * Do not use Flex for two-dimensional page grids — prefer [`Grid`](crate::Grid) * Avoid nesting many Flex containers when a single grid suffices * Do not reach for Flex when [`Stack`](crate::Stack) or [`Space`](crate::Space) defaults already match your layout
///
/// # Layout primitives
///
/// When `Flex` is not the right fit:
///
/// - **Even gap between every sibling on one axis** — [`Stack`](crate::Stack). - **Opposite edges / space-between on one axis** — [`Space`](crate::Space). - **Full flex control** (wrap, inline, fill, inset padding) — `Flex` (this component). - **Fixed column count with span/offset per cell** — [`Grid`](crate::Grid) + [`GridItem`](crate::GridItem). - **Fluid card tiles that reflow by viewport width** — [AutoGrid](/auto-grid) in the orbital crate. - **Single node with padding/surface tokens, no sibling gaps** — [`Box`](crate::Box). - **Page max-width centering inside the shell** — [Container](/container) in the orbital crate. - **Doc-style content + sticky aside rail** — [`ContentWithAside`](crate::ContentWithAside). - **Application shell** (header, sidebar, main) — [`Layout`](crate::Layout).
///
/// **Spacing vocabulary:** [`FlexGap`] presets for `Flex`, `Stack`, and `Space`; pixel gaps on [`Grid`](crate::Grid); [`SpacingSize`](/auto-grid) on AutoGrid. Pick the token type each component expects.
///
/// # Examples
///
/// ## Default
/// Horizontal flex row with medium gap between items—the baseline for toolbars, button groups, and inline control rows.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::DemoBox;
/// view! {
///     <div data-testid="flex-preview">
///         <Flex gap=FlexGap::Medium>
///             <DemoBox data_testid="flex-item-a">"Item A"</DemoBox>
///             <DemoBox data_testid="flex-item-b">"Item B"</DemoBox>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Vertical stack
/// Column direction stacks children for form fields, settings sections, and vertically listed actions.
/// <!-- preview -->
/// ```rust
/// use crate::DemoBox;
/// view! {
///     <div data-testid="flex-vertical">
///         <Flex vertical=true gap=FlexGap::Small>
///             <DemoBox data_testid="flex-stack-1">"First"</DemoBox>
///             <DemoBox data_testid="flex-stack-2">"Second"</DemoBox>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Inline flex
/// `inline-flex` keeps the container in the text flow so compact clusters sit beside surrounding copy without breaking the line.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="flex-inline">
///         <span>"Before "</span>
///         <Flex inline=true gap=FlexGap::Small>
///             <span data-testid="inline-a" style="padding: var(--orb-space-inline-sm); border: 1px dashed var(--orb-color-border-default); border-radius: var(--orb-radius-md);">"A"</span>
///             <span data-testid="inline-b" style="padding: var(--orb-space-inline-sm); border: 1px dashed var(--orb-color-border-default); border-radius: var(--orb-radius-md);">"B"</span>
///         </Flex>
///         <span>" after"</span>
///     </div>
/// }
/// ```
///
/// ## Gap matrix
/// Compare Small, Medium, and Large presets plus custom `Size` and `WH` values so spacing stays consistent without margin hacks.
/// <!-- preview -->
/// ```rust
/// use crate::DemoBox;
/// view! {
///     <div data-testid="flex-gap-matrix">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <Flex gap=FlexGap::Small>
///                 <DemoBox data_testid="gap-small-a">"Small"</DemoBox>
///                 <DemoBox data_testid="gap-small-b">"Small"</DemoBox>
///             </Flex>
///             <Flex gap=FlexGap::Medium>
///                 <DemoBox data_testid="gap-medium-a">"Medium"</DemoBox>
///                 <DemoBox data_testid="gap-medium-b">"Medium"</DemoBox>
///             </Flex>
///             <Flex gap=FlexGap::Large>
///                 <DemoBox data_testid="gap-large-a">"Large"</DemoBox>
///                 <DemoBox data_testid="gap-large-b">"Large"</DemoBox>
///             </Flex>
///             <Flex gap=FlexGap::Size(20)>
///                 <DemoBox data_testid="gap-size-a">"Size(20)"</DemoBox>
///                 <DemoBox data_testid="gap-size-b">"Size(20)"</DemoBox>
///             </Flex>
///             <Flex gap=FlexGap::WH(8, 24)>
///                 <DemoBox data_testid="gap-wh-a">"WH(8,24)"</DemoBox>
///                 <DemoBox data_testid="gap-wh-b">"WH(8,24)"</DemoBox>
///             </Flex>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Centered content
/// Center on both axes when a single block should sit in the middle of a fixed-height region (empty states, compact panels).
/// <!-- preview -->
/// ```rust
/// use crate::DemoBox;
/// view! {
///     <div data-testid="flex-centered" style="width: 100%; max-width: 560px; height: 160px; border: 1px dashed var(--orb-color-border-default); border-radius: var(--orb-radius-md);">
///         <Flex
///             fill=true
///             full_width=true
///             justify=FlexJustify::Center
///             align=FlexAlign::Center
///         >
///             <DemoBox data_testid="flex-center-label">"Centered"</DemoBox>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Align (cross-axis)
/// Start, Center, and End alignment along the cross axis when row height exceeds item height.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="flex-align" style="height: 120px;">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <Flex align=FlexAlign::Start gap=FlexGap::Small>
///                 <DemoBox height="24px">"Start"</DemoBox>
///                 <DemoBox height="48px">"Start"</DemoBox>
///             </Flex>
///             <Flex align=FlexAlign::Center gap=FlexGap::Small>
///                 <DemoBox height="24px">"Center"</DemoBox>
///                 <DemoBox height="48px">"Center"</DemoBox>
///             </Flex>
///             <Flex align=FlexAlign::End gap=FlexGap::Small>
///                 <DemoBox height="24px">"End"</DemoBox>
///                 <DemoBox height="48px">"End"</DemoBox>
///             </Flex>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Sizing children with FlexItem
/// Wrap a child in [`FlexItem`](crate::FlexItem) to set its grow, shrink, basis and
/// min-width. Two `grow=1.0 basis="0"` items split the row equally; a `shrink=0.0`
/// item keeps its width.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Flex, FlexGap, FlexItem};
/// view! {
///     <div data-testid="flex-item-sizing" style="width: 100%; max-width: 560px;">
///         <Flex gap=FlexGap::Small full_width=true>
///             <FlexItem grow=1.0 basis="0" min_width="0">
///                 <DemoBox data_testid="flex-grow-a">"Grow"</DemoBox>
///             </FlexItem>
///             <FlexItem grow=1.0 basis="0" min_width="0">
///                 <DemoBox data_testid="flex-grow-b">"Grow, longer label"</DemoBox>
///             </FlexItem>
///             <FlexItem shrink=0.0>
///                 <DemoBox data_testid="flex-fixed">"Fixed"</DemoBox>
///             </FlexItem>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Justify (main-axis)
/// Start, Center, End, and SpaceBetween distribute items along the main axis—SpaceBetween is common for footer action bars.
/// <!-- preview -->
/// ```rust
/// use crate::{DemoBox, Flex, FlexGap, FlexJustify};
/// view! {
///     <div data-testid="flex-justify">
///         <Flex vertical=true gap=FlexGap::Medium>
///             <Flex justify=FlexJustify::Start gap=FlexGap::Small full_width=true>
///                 <DemoBox>"Start"</DemoBox>
///                 <DemoBox>"Start"</DemoBox>
///             </Flex>
///             <Flex justify=FlexJustify::Center gap=FlexGap::Small full_width=true>
///                 <DemoBox>"Center"</DemoBox>
///                 <DemoBox>"Center"</DemoBox>
///             </Flex>
///             <Flex justify=FlexJustify::End gap=FlexGap::Small full_width=true>
///                 <DemoBox>"End"</DemoBox>
///                 <DemoBox>"End"</DemoBox>
///             </Flex>
///             <Flex justify=FlexJustify::SpaceBetween gap=FlexGap::Small full_width=true>
///                 <DemoBox>"Between"</DemoBox>
///                 <DemoBox>"Between"</DemoBox>
///             </Flex>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "flex",
    preview_label = "Flex",
    preview_icon = icondata::AiColumnWidthOutlined,
)]
#[component]
pub fn Flex(
    /// Optional CSS class names merged onto the flex container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Spacing between flex items. Presets: `Small`, `Medium` (default), `Large`; or custom via `Size(px)` / `WH(row_px, col_px)`.
    #[prop(optional)]
    gap: FlexGap,
    /// When `true`, lays out children in a column (`flex-direction: column`).
    #[prop(optional)]
    vertical: bool,
    /// When `true`, uses `display: inline-flex` so the container flows inline.
    #[prop(optional, into)]
    inline: Signal<bool>,
    /// Cross-axis alignment (`align-items`).
    #[prop(optional, into)]
    align: MaybeProp<FlexAlign>,
    /// Main-axis distribution (`justify-content`).
    #[prop(optional, into)]
    justify: MaybeProp<FlexJustify>,
    /// Whether flex items wrap onto multiple lines.
    #[prop(optional, default = FlexWrap::NoWrap)]
    wrap: FlexWrap,
    /// When `true`, the container fills the height of its parent (`height: 100%`).
    #[prop(optional, default = false)]
    fill: bool,
    /// When `true`, the container spans the full width of its parent.
    #[prop(optional, default = false)]
    full_width: bool,
    /// Theme-aware padding using Orbital spacing tokens.
    #[prop(optional, into)]
    padding: MaybeProp<SpacingInset>,
    /// Theme-aware margin using Orbital spacing tokens.
    #[prop(optional, into)]
    margin: MaybeProp<SpacingInset>,
    /// Flex item children.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseFlex
            class=class
            gap=gap
            vertical=vertical
            inline=inline
            align=align
            justify=justify
            wrap=wrap
            fill=fill
            full_width=full_width
            padding=padding
            margin=margin
        >
            {children()}
        </BaseFlex>
    }
}
