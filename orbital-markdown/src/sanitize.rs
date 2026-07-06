use std::collections::{HashMap, HashSet};

/// Sanitize HTML with optional image and citation-ref tags.
pub fn sanitize_html(html: &str, allow_images: bool) -> String {
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
            ["href", "target", "id", "class", "data-citation-id", "data-mention-id"]
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

    let cleaned = ammonia::Builder::default()
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
