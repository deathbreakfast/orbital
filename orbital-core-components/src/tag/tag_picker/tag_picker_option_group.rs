use leptos::prelude::*;
use orbital_base_components::new_field_id;

/// Labeled group of [`TagPickerOption`](super::tag_picker_option::TagPickerOption) children.
///
/// The label names the group for screen readers, and the optional caption describes it.
#[component]
pub fn TagPickerOptionGroup(
    /// Optional CSS class merged onto the group root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Label of the group.
    #[prop(into)]
    label: String,
    /// Muted line under the label, such as where the group's options came from.
    /// Defaults to none.
    #[prop(optional, into)]
    caption: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let group_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-tag-picker-option-group".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });
    let label_id = new_field_id();
    let labelled_by = label_id.clone();
    let caption_id = new_field_id();
    let described_by = {
        let caption_id = caption_id.clone();
        move || caption.with(Option::is_some).then(|| caption_id.clone())
    };

    view! {
        <div
            role="group"
            class=group_class
            aria-labelledby=labelled_by
            aria-describedby=described_by
        >
            <span id=label_id class="orbital-tag-picker-option-group__label">
                {label}
            </span>
            {move || {
                caption.get().map(|caption| {
                    view! {
                        <span id=caption_id.clone() class="orbital-tag-picker-option-group__caption">
                            {caption}
                        </span>
                    }
                })
            }}
            {children()}
        </div>
    }
}

/// Server-render tests: `cargo test -p orbital-core-components --features ssr --lib option_group_ssr`.
#[cfg(all(test, feature = "ssr"))]
mod option_group_ssr_tests {
    use super::*;

    fn attr<'a>(html: &'a str, name: &str) -> Option<&'a str> {
        let start = html.find(&format!("{name}=\""))? + name.len() + 2;
        let end = html[start..].find('"')? + start;
        Some(&html[start..end])
    }

    #[test]
    fn option_group_ssr_is_named_by_its_label_and_described_by_its_caption() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <TagPickerOptionGroup label="Acme directory" caption="From Google Workspace">
                    <span>"Paula Mendes"</span>
                </TagPickerOptionGroup>
            }
            .to_html()
        });
        let labelled_by = attr(&html, "aria-labelledby").expect("aria-labelledby");
        let described_by = attr(&html, "aria-describedby").expect("aria-describedby");
        assert!(html.contains(&format!("id=\"{labelled_by}\"")), "{html}");
        assert!(html.contains(&format!("id=\"{described_by}\"")), "{html}");
        assert!(html.contains("Acme directory"), "{html}");
        assert!(html.contains("From Google Workspace"), "{html}");
    }

    #[test]
    fn option_group_ssr_without_caption_has_no_description() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <TagPickerOptionGroup label="Recent">
                    <span>"Theo Brandt"</span>
                </TagPickerOptionGroup>
            }
            .to_html()
        });
        assert!(attr(&html, "aria-labelledby").is_some(), "{html}");
        assert!(attr(&html, "aria-describedby").is_none(), "{html}");
        assert!(
            !html.contains("orbital-tag-picker-option-group__caption"),
            "{html}"
        );
    }
}
