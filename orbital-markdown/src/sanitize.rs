use std::collections::{HashMap, HashSet};

/// Sanitize HTML with optional image and citation-ref tags.
///
/// `image_url_schemes`, when set, further restricts `<img src>` to those schemes.
pub fn sanitize_html(
    html: &str,
    allow_images: bool,
    image_url_schemes: Option<&[String]>,
) -> String {
    let mut tags: HashSet<&str> = [
        "p", "strong", "em", "del", "code", "pre", "a", "ul", "ol", "li", "br", "sup",
    ]
    .into_iter()
    .collect();

    if allow_images {
        tags.insert("img");
    }

    let mut tag_attrs: HashMap<&str, HashSet<&str>> = HashMap::from([
        (
            "a",
            [
                "href",
                "target",
                "id",
                "class",
                "data-citation-id",
                "data-mention-id",
            ]
            .into_iter()
            .collect(),
        ),
        ("code", ["class"].into_iter().collect()),
        ("pre", ["class"].into_iter().collect()),
        ("sup", ["class"].into_iter().collect()),
    ]);

    if allow_images {
        tag_attrs.insert(
            "img",
            ["src", "alt", "class", "loading"].into_iter().collect(),
        );
    }

    let mut builder = ammonia::Builder::default();
    if let Some(schemes) = image_url_schemes {
        let schemes: Vec<String> = schemes.iter().map(|s| s.to_ascii_lowercase()).collect();
        builder.attribute_filter(move |element, attribute, value| {
            if element != "img" || attribute != "src" {
                return Some(value.into());
            }
            match url_scheme(value) {
                Some(scheme) if !schemes.contains(&scheme) => None,
                _ => Some(value.into()),
            }
        });
    }

    let cleaned = builder
        .tags(tags)
        .tag_attributes(tag_attrs)
        .url_schemes(
            ["http", "https", "mailto", "mention-ref"]
                .into_iter()
                .collect(),
        )
        .link_rel(Some("noopener noreferrer"))
        .clean(html)
        .to_string();

    cleaned.replace("<pre>", "<pre class=\"orbital-markdown__code-block\">")
}

/// Lowercased scheme of an absolute URL, or `None` for a relative reference.
fn url_scheme(value: &str) -> Option<String> {
    let value = value.trim_start();
    let end = value.find([':', '/', '?', '#'])?;
    let scheme = &value[..end];
    if value.as_bytes()[end] == b':'
        && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        && scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    {
        Some(scheme.to_ascii_lowercase())
    } else {
        None
    }
}
