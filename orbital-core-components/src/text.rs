//! Orbital typography — [`Text`] and scale presets.
//!
//! Pick a typography preset (`Body1`, `Title2`, `Display`, …) for consistent hierarchy across
//! your app. Drop to [`Text`] when you need explicit [`TextSize`], [`TextWeight`], font family,
//! or decoration props. Sizes and weights come from the active theme — avoid raw CSS font values.
//! For forms, prefer [`FormLabel`], [`FormHint`], and [`SectionTitle`] inside a
//! [`Field`](crate::Field).
//!
//! # Preset roles
//!
//! | Role | Preset | Token | Use for |
//! | --- | --- | --- | --- |
//! | Page / shell title | [`Title3`]–[`Title1`] | `lg`–`2xl` | App bar, dialog title, panel primary heading |
//! | Content section / card title | [`Subtitle1`] | `md` + semibold | Card headers, preview sections, entity display names |
//! | Form / utility group label | [`SectionTitle`] | `2xs` + semibold | Form clusters, TOC rail headings, dense settings |
//! | Body copy | [`Body1`] / [`Body2`] | `sm` / `md` regular | Paragraphs, descriptions |
//! | Metadata | [`Caption1`] / [`Caption2`] | `2xs` (10px) / `xs` (12px) | Timestamps, badges, hints |
//!
//! **Floor rule:** Named section labels (nav groups, card titles, author names, persona primary
//! line) must not render below **Body1** (`--orb-type-size-sm`, 14px).
//!
//! **Caption ramp:** The two caption presets run opposite to Fluent, where `Caption1` is 12px and
//! `Caption2` is 10px. Here [`Caption1`] is the smaller step (`2xs`, 10px) and [`Caption2`] is
//! `xs` (12px). The sizes stay as they are because [`FormLabel`], [`FormHint`] and existing call
//! sites depend on them, and swapping would change downstream apps without a compile error.
//! Use the 10px [`Caption1`] family only for non-essential metadata such as timestamps and
//! badges. Status messages, help text and anything the reader needs should use [`Caption2`] or
//! [`Body1`].
//!
//! **Naming rule:** [`Subtitle1`] and [`Subtitle2`] are identical (S400 / semibold). Prefer
//! [`Subtitle1`] for section and card titles; reserve [`Subtitle2`] for empty-state headlines.
//!
//! # CSS-backed components
//!
//! Some components style titles in CSS rather than presets. Align wrapper tokens with the preset
//! equivalents — when slot content uses a preset (e.g. [`PersonaPrimaryText`](crate::PersonaPrimaryText)),
//! the wrapper CSS should match so styles do not fight.
//!
//! | Class | Token | Preset equivalent |
//! | --- | --- | --- |
//! | `.orbital-dialog-title` | `lg` semibold | [`Title3`] |
//! | `.orbital-data-table__list-card-title` | `md` semibold | [`Subtitle1`] |
//! | `.orbital-persona__primary-text` | `md` semibold | [`Subtitle1`] |
//! | `.orbital-discussion__author-name` | `md` semibold | [`Subtitle1`] |
//! | `.SectionHeader` (navigation) | `md` semibold | [`Subtitle1`] |
//! | `.SectionHeaderBand` (navigation) | `sm` semibold | [`Body1Strong`] |
//! | `.orbital-card-header__description` | `xs` | [`Caption2`] |

use leptos::{prelude::*, tachys::view::any_view::IntoAny};
use orbital_base_components::ThemeColor;
use orbital_style::inject_style;
use turf::inline_style_sheet_values;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextTag {
    B,
    Code,
    Div,
    Em,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    I,
    Label,
    P,
    Pre,
    #[default]
    Span,
    Strong,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    Center,
    End,
    Justify,
    #[default]
    Start,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextFont {
    #[default]
    Base,
    Monospace,
    Numeric,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextWeight {
    Bold,
    Medium,
    #[default]
    Regular,
    Semibold,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextSize {
    S100,
    S200,
    #[default]
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
    S1000,
}

/// Low-level typography control when presets do not match your need.
///
/// Prefer named presets ([`Body1`], [`Title2`], [`Display`], …) for hierarchy — each locks `size` and `weight` to a step on Orbital's typography scale. Use `Text` when you need a custom token combination, a semantic [`TextTag`], or decorations without a matching preset.
///
/// There is no bundled `variant` prop — pick a preset or set [`TextSize`] and [`TextWeight`] explicitly. Use [`tag`] to choose the rendered HTML element; use [`truncate`] for single-line ellipsis inside a bounded container.
///
/// # When to use
///
/// - Explicit size/weight/font combinations outside the preset matrix - Semantic HTML via [`TextTag`] (`H1`, `P`, `Code`, …) with token styling - One-off decorations (`italic`, `underline`, `strikethrough`) on any scale step - For field labels, hints, and settings headings, use [`FormLabel`], [`FormHint`], or [`SectionTitle`] instead
///
/// # Examples
///
/// ## Preset vs low-level
///
/// ```
/// use leptos::prelude::*;
/// use orbital_core_components::{Body1, Text, TextSize, TextWeight};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     view! {
///         <Body1>"Default body copy from the typography scale."</Body1>
///         <Text size=TextSize::S400 weight=TextWeight::Semibold>"Custom emphasis step."</Text>
///     }
/// }
/// ```
#[component]
pub fn Text(
    /// HTML element to render — default `Span`. Use `H1`–`H6`, `P`, `Label`, or `Code` for document structure.
    #[prop(optional)]
    tag: TextTag,
    /// Horizontal alignment inside a block container: `Start`, `Center`, `End`, or `Justify`.
    #[prop(optional)]
    align: TextAlign,
    /// Render as a block-level box (`display: block`) instead of inline.
    #[prop(optional)]
    block: bool,
    /// Font family preset: `Base`, `Numeric` (tabular figures), or `Monospace`.
    #[prop(optional)]
    font: TextFont,
    /// Italic emphasis.
    #[prop(optional)]
    italic: bool,
    /// Typography scale step from theme tokens (`S100` smallest through `S1000` largest).
    #[prop(optional)]
    size: TextSize,
    /// Line-through decoration for deleted or superseded copy.
    #[prop(optional)]
    strikethrough: bool,
    /// Single-line ellipsis overflow — pair with `block=true` inside a bounded-width container.
    #[prop(optional)]
    truncate: bool,
    /// Underline decoration — use for inline links or emphasized phrases.
    #[prop(optional)]
    underline: bool,
    /// Font weight preset: `Regular`, `Medium`, `Semibold`, or `Bold`.
    #[prop(optional)]
    weight: TextWeight,
    /// Allow line breaks at whitespace. Set `false` to keep copy on one line (may overflow).
    ///
    /// Note: use `#[prop(default = true)]`, not `#[prop(optional = true)]`. In Leptos, `optional` only means the prop may be omitted; omitted `bool` still defaults to `false`.
    #[prop(default = true)]
    wrap: bool,
    /// Foreground color from the active theme palette.
    #[prop(optional, into)]
    color: MaybeProp<ThemeColor>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    /// Test id hook (maps to `data-testid`).
    #[prop(optional, into)]
    test_id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Root {
            display: inline;
            font-family: inherit;
            margin: 0;
            color: var(--orb-color-text-primary);
        }

        .Block {
            display: block;
        }

        .AlignStart {
            text-align: start;
        }
        .AlignCenter {
            text-align: center;
        }
        .AlignEnd {
            text-align: end;
        }
        .AlignJustify {
            text-align: justify;
        }

        .FontBase {
            font-family: var(--orb-type-family-sans);
        }
        .FontNumeric {
            font-family: var(--orb-type-family-numeric);
            font-variant-numeric: tabular-nums;
        }
        .FontMonospace {
            font-family: var(--orb-type-family-mono);
        }

        .Italic {
            font-style: italic;
        }

        .Underline {
            text-decoration-line: underline;
        }

        .Strikethrough {
            text-decoration-line: line-through;
        }

        .Underline.Strikethrough {
            text-decoration-line: underline line-through;
        }

        .Wrap {
            white-space: normal;
            overflow-wrap: break-word;
        }

        .NoWrap {
            white-space: nowrap;
        }

        .Truncate {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            max-width: 100%;
            display: inline-block;
        }

        .Block.Truncate {
            display: block;
        }

        // Typography scale tracks theme tokens on the active OrbitalThemeProvider scope.
        .Size100 { font-size: var(--orb-type-size-2xs); line-height: var(--orb-type-line-sm); }
        .Size200 { font-size: var(--orb-type-size-xs); line-height: var(--orb-type-line-sm); }
        .Size300 { font-size: var(--orb-type-size-sm); line-height: var(--orb-type-line-md); }
        .Size400 { font-size: var(--orb-type-size-md); line-height: var(--orb-type-line-lg); }
        .Size500 { font-size: var(--orb-type-size-lg); line-height: var(--orb-type-line-xl); }
        .Size600 { font-size: var(--orb-type-size-xl); line-height: 1.333; }
        .Size700 { font-size: var(--orb-type-size-2xl); line-height: 1.286; }
        .Size800 { font-size: var(--orb-type-size-3xl); line-height: 1.25; }
        .Size900 { font-size: var(--orb-type-size-4xl); line-height: 1.3; }
        .Size1000 { font-size: var(--orb-type-size-5xl); line-height: 1.35; }

        .WeightRegular { font-weight: var(--orb-type-weight-regular); }
        .WeightMedium { font-weight: 500; }
        .WeightSemibold { font-weight: var(--orb-type-weight-semibold); }
        .WeightBold { font-weight: var(--orb-type-weight-bold); }
    };
    inject_style("orbital-text", style_sheet);

    let class = Signal::derive(move || {
        let mut classes: Vec<String> = vec![class_names.root.to_string()];

        if block {
            classes.push(class_names.block.to_string());
        }

        classes.push(
            match align {
                TextAlign::Start => class_names.align_start,
                TextAlign::Center => class_names.align_center,
                TextAlign::End => class_names.align_end,
                TextAlign::Justify => class_names.align_justify,
            }
            .to_string(),
        );

        classes.push(
            match font {
                TextFont::Base => class_names.font_base,
                TextFont::Numeric => class_names.font_numeric,
                TextFont::Monospace => class_names.font_monospace,
            }
            .to_string(),
        );

        if italic {
            classes.push(class_names.italic.to_string());
        }
        if underline {
            classes.push(class_names.underline.to_string());
        }
        if strikethrough {
            classes.push(class_names.strikethrough.to_string());
        }

        classes.push(
            match size {
                TextSize::S100 => class_names.size_100,
                TextSize::S200 => class_names.size_200,
                TextSize::S300 => class_names.size_300,
                TextSize::S400 => class_names.size_400,
                TextSize::S500 => class_names.size_500,
                TextSize::S600 => class_names.size_600,
                TextSize::S700 => class_names.size_700,
                TextSize::S800 => class_names.size_800,
                TextSize::S900 => class_names.size_900,
                TextSize::S1000 => class_names.size_1000,
            }
            .to_string(),
        );

        classes.push(
            match weight {
                TextWeight::Regular => class_names.weight_regular,
                TextWeight::Medium => class_names.weight_medium,
                TextWeight::Semibold => class_names.weight_semibold,
                TextWeight::Bold => class_names.weight_bold,
            }
            .to_string(),
        );

        if truncate {
            classes.push(class_names.truncate.to_string());
        }

        classes.push(if wrap {
            class_names.wrap.to_string()
        } else {
            class_names.no_wrap.to_string()
        });

        if let Some(extra) = class.get() {
            if !extra.trim().is_empty() {
                classes.push(extra);
            }
        }

        classes.join(" ")
    });

    let merged_style = move || {
        let mut parts = Vec::new();
        if let Some(c) = color.get() {
            parts.push(format!("color: {}", c.css_var()));
        }
        if let Some(extra) = style.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("; "))
        }
    };

    let rendered = match tag {
        TextTag::B => {
            view! { <b class=class style=merged_style data-testid=test_id>{children()}</b> }.into_any()
        }
        TextTag::Code => {
            view! { <code class=class style=merged_style data-testid=test_id>{children()}</code> }
                .into_any()
        }
        TextTag::Div => {
            view! { <div class=class style=merged_style data-testid=test_id>{children()}</div> }.into_any()
        }
        TextTag::Em => {
            view! { <em class=class style=merged_style data-testid=test_id>{children()}</em> }.into_any()
        }
        TextTag::H1 => {
            view! { <h1 class=class style=merged_style data-testid=test_id>{children()}</h1> }.into_any()
        }
        TextTag::H2 => {
            view! { <h2 class=class style=merged_style data-testid=test_id>{children()}</h2> }.into_any()
        }
        TextTag::H3 => {
            view! { <h3 class=class style=merged_style data-testid=test_id>{children()}</h3> }.into_any()
        }
        TextTag::H4 => {
            view! { <h4 class=class style=merged_style data-testid=test_id>{children()}</h4> }.into_any()
        }
        TextTag::H5 => {
            view! { <h5 class=class style=merged_style data-testid=test_id>{children()}</h5> }.into_any()
        }
        TextTag::H6 => {
            view! { <h6 class=class style=merged_style data-testid=test_id>{children()}</h6> }.into_any()
        }
        TextTag::I => {
            view! { <i class=class style=merged_style data-testid=test_id>{children()}</i> }.into_any()
        }
        TextTag::Label => {
            view! { <label class=class style=merged_style data-testid=test_id>{children()}</label> }
                .into_any()
        }
        TextTag::P => {
            view! { <p class=class style=merged_style data-testid=test_id>{children()}</p> }.into_any()
        }
        TextTag::Pre => {
            view! { <pre class=class style=merged_style data-testid=test_id>{children()}</pre> }.into_any()
        }
        TextTag::Span => {
            view! { <span class=class style=merged_style data-testid=test_id>{children()}</span> }
                .into_any()
        }
        TextTag::Strong => {
            view! { <strong class=class style=merged_style data-testid=test_id>{children()}</strong> }
                .into_any()
        }
    };

    view! {
        {rendered}
    }
}

macro_rules! text_preset {
    ($name:ident, size: $size:expr, weight: $weight:expr) => {
        #[component]
        pub fn $name(
            #[prop(optional)] tag: TextTag,
            #[prop(optional)] align: TextAlign,
            #[prop(optional)] block: bool,
            #[prop(default = true)] wrap: bool,
            #[prop(optional)] italic: bool,
            #[prop(optional)] underline: bool,
            #[prop(optional)] strikethrough: bool,
            #[prop(optional)] truncate: bool,
            #[prop(optional, into)] color: MaybeProp<ThemeColor>,
            #[prop(optional, into)] class: MaybeProp<String>,
            #[prop(optional, into)] style: MaybeProp<String>,
            #[prop(optional, into)] test_id: MaybeProp<String>,
            children: Children,
        ) -> impl IntoView {
            view! {
                <Text
                    tag=tag
                    align=align
                    block=block
                    wrap=wrap
                    italic=italic
                    underline=underline
                    strikethrough=strikethrough
                    truncate=truncate
                    color=color
                    class=class
                    style=style
                    test_id=test_id
                    size=$size
                    weight=$weight
                >
                    {children()}
                </Text>
            }
        }
    };
}

// Typography presets — each locks `size` + `weight` to a scale step.
// Pass `tag`, decorations, `color`, and other layout props through; `size`, `weight`, and `font`
// are fixed per preset.
text_preset!(Caption2, size: TextSize::S200, weight: TextWeight::Regular);
text_preset!(Caption2Strong, size: TextSize::S200, weight: TextWeight::Semibold);
text_preset!(Caption1, size: TextSize::S100, weight: TextWeight::Regular);
text_preset!(Caption1Strong, size: TextSize::S100, weight: TextWeight::Semibold);
text_preset!(Caption1Stronger, size: TextSize::S100, weight: TextWeight::Bold);

text_preset!(Body1, size: TextSize::S300, weight: TextWeight::Regular);
text_preset!(Body1Strong, size: TextSize::S300, weight: TextWeight::Semibold);
text_preset!(Body1Stronger, size: TextSize::S300, weight: TextWeight::Bold);
text_preset!(Body2, size: TextSize::S400, weight: TextWeight::Regular);

/// Empty-state headlines and ramp specimens — S400 / semibold. Same size as [`Subtitle1`].
text_preset!(Subtitle2, size: TextSize::S400, weight: TextWeight::Semibold);
text_preset!(Subtitle2Stronger, size: TextSize::S400, weight: TextWeight::Bold);
/// Section and card titles — S400 / semibold. Prefer for card headers, preview sections,
/// and in-page doc sections.
text_preset!(Subtitle1, size: TextSize::S400, weight: TextWeight::Semibold);

text_preset!(Title3, size: TextSize::S500, weight: TextWeight::Semibold);
text_preset!(Title2, size: TextSize::S600, weight: TextWeight::Semibold);
text_preset!(Title1, size: TextSize::S700, weight: TextWeight::Semibold);
text_preset!(LargeTitle, size: TextSize::S800, weight: TextWeight::Semibold);
text_preset!(Display, size: TextSize::S1000, weight: TextWeight::Semibold);

// Form-specific convenience components (Orbital-only).
/// Accessible field label styled as `Caption2` with a native `<label>` element.
///
/// Prefer inside a [`Field`](crate::Field) over hand-rolling `Caption2` with [`TextTag::Label`].
#[component]
pub fn FormLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] test_id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Caption2 tag=TextTag::Label block=true class=class style=style test_id=test_id>
            {children()}
        </Caption2>
    }
}

/// Supplementary hint below a form control — format guidance, validation context, or privacy notes.
///
/// Renders muted `Caption2` copy. Pair with [`FormLabel`] inside a [`Field`](crate::Field).
#[component]
pub fn FormHint(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] test_id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .FormHint {
            color: var(--orb-color-text-tertiary);
            margin-top: 4px;
        }
    };
    inject_style("orbital-form-hint", style_sheet);
    // Combine base class with optional user class (evaluated at init)
    let base = class_names.form_hint.to_string();
    let class_val = match class.get() {
        None => base.clone(),
        Some(ref s) if s.trim().is_empty() => base.clone(),
        Some(ref s) => format!("{} {}", base, s),
    };
    view! {
        <Caption2 block=true class=class_val style=style test_id=test_id>
            {children()}
        </Caption2>
    }
}

/// Grouped settings heading — a section label within a form or panel, not a page-level heading.
///
/// Renders `Caption1Strong`. Use for clusters like "Account settings" or "Notification preferences".
#[component]
pub fn SectionTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] test_id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .SectionTitle {
            color: var(--orb-color-text-primary);
        }
    };
    inject_style("orbital-section-title", style_sheet);
    let base = class_names.section_title.to_string();
    let class_val = match class.get() {
        None => base.clone(),
        Some(ref s) if s.trim().is_empty() => base.clone(),
        Some(ref s) => format!("{} {}", base, s),
    };
    view! {
        <Caption1Strong block=true class=class_val style=style test_id=test_id>
            {children()}
        </Caption1Strong>
    }
}
