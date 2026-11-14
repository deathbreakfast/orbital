use leptos::prelude::*;

/// Sizes one child of a [`Flex`](crate::Flex) row or column: how much spare space
/// it takes (`grow`), whether it gives space back (`shrink`), where it starts
/// (`basis`) and how narrow it may get (`min_width`).
///
/// Use it instead of an inline `style="flex: 1 1 0"` wrapper. Leave a prop unset to
/// keep the browser default for it. `grow=1.0 basis="0"` makes siblings share the
/// row equally; `min_width="0"` lets long content (a table, a truncated title)
/// shrink below its natural width.
///
/// ```rust,ignore
/// view! {
///     <Flex gap=FlexGap::Medium>
///         <FlexItem grow=1.0 basis="0" min_width="0">"Main"</FlexItem>
///         <FlexItem shrink=0.0>"Aside"</FlexItem>
///     </Flex>
/// }
/// ```
#[component]
pub fn FlexItem(
    /// `flex-grow`: share of the spare space this item takes.
    #[prop(optional, into)]
    grow: MaybeProp<f32>,
    /// `flex-shrink`: `0.0` keeps the item from shrinking below its basis.
    #[prop(optional, into)]
    shrink: MaybeProp<f32>,
    /// `flex-basis` as a CSS length (`"0"`, `"240px"`, `"30%"`).
    #[prop(optional, into)]
    basis: MaybeProp<String>,
    /// `min-width` as a CSS length; `"0"` lets content shrink below its natural width.
    #[prop(optional, into)]
    min_width: MaybeProp<String>,
    /// Extra CSS class names on the item.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let style =
        Memo::new(move |_| flex_item_style(grow.get(), shrink.get(), basis.get(), min_width.get()));
    view! {
        <div
            class=move || match class.get() {
                Some(extra) if !extra.is_empty() => format!("orbital-flex-item {extra}"),
                _ => "orbital-flex-item".to_string(),
            }
            style=move || style.get()
        >
            {children()}
        </div>
    }
}

fn flex_item_style(
    grow: Option<f32>,
    shrink: Option<f32>,
    basis: Option<String>,
    min_width: Option<String>,
) -> String {
    let mut style = String::new();
    if let Some(grow) = grow {
        style.push_str(&format!("flex-grow:{grow};"));
    }
    if let Some(shrink) = shrink {
        style.push_str(&format!("flex-shrink:{shrink};"));
    }
    if let Some(basis) = basis {
        style.push_str(&format!("flex-basis:{basis};"));
    }
    if let Some(min_width) = min_width {
        style.push_str(&format!("min-width:{min_width};"));
    }
    style
}

#[cfg(test)]
mod tests {
    use super::flex_item_style;

    #[test]
    fn unset_props_leave_the_style_empty() {
        assert_eq!(flex_item_style(None, None, None, None), "");
    }

    #[test]
    fn set_props_map_to_flex_longhands() {
        assert_eq!(
            flex_item_style(Some(1.0), Some(0.0), Some("0".into()), Some("0".into())),
            "flex-grow:1;flex-shrink:0;flex-basis:0;min-width:0;"
        );
    }
}
