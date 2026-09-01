use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

/// Page-level max-width and horizontal centering inside the shell.
///
/// The container does not add its own padding — [`LayoutMain`](orbital_core_components::LayoutMain) or the host layout's content scroll area should provide responsive padding. Default max-width is 1200px. Common widths: `"720px"` for focused reading, `"900px"` for forms and articles, `"1200px"` for default page content.
///
/// # Examples
///
/// ## Default (1200px)
/// Standard page width centered in the shell — the default for catalog and app content.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="container-preview" style="width: 100%; border: 1px dashed var(--orb-color-border-default);">
///         <Container>
///             <div style="padding: var(--orb-space-inline-md); background: var(--orb-color-surface-subtle);">
///                 "Centered content at 1200px max-width"
///             </div>
///         </Container>
///     </div>
/// }
/// ```
///
/// ## Narrow form width
/// Tighter `max_width` for focused forms or reading columns without changing shell padding.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="container-narrow" style="width: 100%; min-width: 720px; border: 1px dashed var(--orb-color-border-default);">
///         <Container max_width="720px">
///             <div style="padding: var(--orb-space-inline-md); background: var(--orb-color-surface-subtle);">
///                 "Focused content at 720px"
///             </div>
///         </Container>
///     </div>
/// }
/// ```
///
/// ## Custom max width
/// Override `max_width` for marketing hero sections or wide data tables.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="container-wide" style="width: 100%; border: 1px dashed var(--orb-color-border-default);">
///         <Container max_width="900px">
///             <div style="padding: var(--orb-space-inline-md); background: var(--orb-color-surface-subtle);">
///                 "Form or article width at 900px"
///             </div>
///         </Container>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "container",
    preview_label = "Container",
    preview_icon = icondata::AiColumnWidthOutlined,
)]
#[component]
pub fn Container(
    /// Maximum width of the content area. Defaults to `"1200px"`. Common values: `"900px"` for form pages, `"720px"` for focused content.
    #[prop(optional, into, default = "1200px".into())]
    max_width: String,
    /// Minimum width of the content area (CSS length, e.g. `"360px"`).
    #[prop(optional, into)]
    min_width: MaybeProp<String>,
    /// Optional CSS class to merge onto the container element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional `data-testid` attribute for testing.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// Child content.
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Container {
            margin-inline: auto;
            box-sizing: border-box;
        }
    };

    let container_style = Signal::derive(move || {
        let mut parts = vec![format!("width: min({}, 100%)", max_width.clone())];
        if let Some(min) = min_width.get().filter(|s| !s.trim().is_empty()) {
            parts.push(format!("min-width: {min}"));
        }
        parts.push("box-sizing: border-box".to_string());
        parts.join("; ")
    });

    let container_class = {
        let base = class_names.container.to_string();
        match class.get() {
            Some(extra) if !extra.trim().is_empty() => format!("{} {}", base, extra),
            _ => base,
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div
            class=container_class
            style=container_style
            data-testid=move || data_testid.get()
        >
            {children()}
        </div>
    }
}
