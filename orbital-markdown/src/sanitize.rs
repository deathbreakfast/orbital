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
        "p",
        "strong",
        "em",
        "del",
        "code",
        "pre",
        "a",
        "ul",
        "ol",
        "li",
        "br",
        "sup",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "blockquote",
        "hr",
        "table",
        "thead",
        "tbody",
        "tr",
        "th",
        "td",
        "details",
        "summary",
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
        // Column alignment from `|:--:|`; the filter keeps only `text-align`.
        ("th", ["style"].into_iter().collect()),
        ("td", ["style"].into_iter().collect()),
        ("details", ["open"].into_iter().collect()),
    ]);

    if allow_images {
        tag_attrs.insert(
            "img",
            ["src", "alt", "class", "loading"].into_iter().collect(),
        );
    }

    let image_schemes: Option<Vec<String>> =
        image_url_schemes.map(|schemes| schemes.iter().map(|s| s.to_ascii_lowercase()).collect());
    let mut builder = ammonia::Builder::default();
    builder.attribute_filter(
        move |element, attribute, value| match (element, attribute) {
            ("th" | "td", "style") => cell_alignment(value).map(Into::into),
            ("img", "src") => match (&image_schemes, url_scheme(value)) {
                (Some(schemes), Some(scheme)) if !schemes.contains(&scheme) => None,
                _ => Some(value.into()),
            },
            _ => Some(value.into()),
        },
    );

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

/// The `text-align` pulldown-cmark writes for an aligned table column, or
/// `None` for any other style, which drops the attribute.
fn cell_alignment(style: &str) -> Option<&'static str> {
    match style.trim().trim_end_matches(';').trim() {
        "text-align: left" => Some("text-align: left"),
        "text-align: center" => Some("text-align: center"),
        "text-align: right" => Some("text-align: right"),
        _ => None,
    }
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
