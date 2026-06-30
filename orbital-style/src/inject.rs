use cfg_if::cfg_if;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
const STYLE_ID_PREFIX: &str = "orbital-style-";
#[cfg(feature = "hydrate")]
const STYLE_MARKER_SELECTOR: &str = r#"meta[name="orbital-style"]"#;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
fn style_element_id(id: &str) -> String {
    format!("{STYLE_ID_PREFIX}{id}")
}

/// Returns true when style text content should be written to the DOM.
#[cfg(feature = "hydrate")]
pub(crate) fn should_update_style_content(current: Option<&str>, next: &str) -> bool {
    current != Some(next)
}

#[cfg(feature = "hydrate")]
fn upsert_style_text_content(style: &leptos::web_sys::Element, content: &str) -> bool {
    if !should_update_style_content(style.text_content().as_deref(), content) {
        return false;
    }
    style.set_text_content(Some(content));
    true
}

#[cfg(feature = "hydrate")]
fn ensure_style_in_head(element_id: &str, content: &str) {
    use leptos::prelude::document;

    let head = document().head().expect("head no exist");
    let style = head
        .query_selector(&format!("style#{element_id}"))
        .expect("query style element error")
        .unwrap_or_else(|| {
            let style = document()
                .create_element("style")
                .expect("create style element error");
            let _ = style.set_attribute("id", element_id);

            let orbital_meta = head
                .query_selector(STYLE_MARKER_SELECTOR)
                .expect("query orbital-style meta element error");

            if let Some(orbital_meta) = orbital_meta {
                let _ = head.insert_before(&style, Some(&orbital_meta));
            } else {
                let _ = head.prepend_with_node_1(&style);
            }

            style
        });

    upsert_style_text_content(&style, content);
}

/// Injects a static stylesheet into `<head>` once (deduplicated by id).
pub fn inject_style(id: &str, content: &'static str) {
    cfg_if! {
        if #[cfg(any(feature = "ssr", feature = "hydrate"))] {
            let element_id = style_element_id(id);
            cfg_if! {
                if #[cfg(feature = "ssr")] {
                    use leptos::view;
                    use leptos_meta::Style;
                    use super::style_registry::StyleRegistryContext;

                    if let Some(context) = StyleRegistryContext::use_context() {
                        context.push_style(element_id, content.to_string());
                        return;
                    }

                    let _ = view! {
                        <Style id=element_id>
                            {content}
                        </Style>
                    };
                } else if #[cfg(feature = "hydrate")] {
                    use super::style_registry::StyleRegistryContext;

                    if let Some(context) = StyleRegistryContext::use_context() {
                        context.push_style(element_id.clone(), content.to_string());
                        context.ensure_style_in_head(&element_id, content);
                        return;
                    }

                    ensure_style_in_head(&element_id, content);
                }
            }
        } else {
            let _ = (id, content);
        }
    }
}

/// Injects a reactive stylesheet into `<head>` (client) or collects it for SSR.
pub fn inject_dynamic_style<T: Fn() -> String + Send + Sync + 'static>(id: String, f: T) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let element_id = style_element_id(&id);
            use leptos::{view, prelude::untrack};
            use leptos_meta::Style;
            use super::style_registry::StyleRegistryContext;

            if let Some(context) = StyleRegistryContext::use_context() {
                context.push_style(element_id, untrack(f));
                return;
            }

            let _ = view! {
                <Style id=element_id>
                    {f()}
                </Style>
            };
        } else if #[cfg(feature = "hydrate")] {
            let element_id = style_element_id(&id);
            use leptos::prelude::document;
            use send_wrapper::SendWrapper;
            use std::ops::Deref;

            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{element_id}"))
                .expect("query style element error")
                .unwrap_or_else(|| {
                    let style = document()
                        .create_element("style")
                        .expect("create style element error");
                    let _ = style.set_attribute("id", &element_id);

                    let orbital_meta = head
                        .query_selector(STYLE_MARKER_SELECTOR)
                        .expect("query orbital-style meta element error");

                    if let Some(orbital_meta) = orbital_meta {
                        let _ = head.insert_before(&style, Some(&orbital_meta));
                    } else {
                        let _ = head.prepend_with_node_1(&style);
                    }

                    style
                });

            let style = SendWrapper::new(style);
            leptos::prelude::Effect::new_isomorphic(move |_| {
                let content = f();
                upsert_style_text_content(style.deref(), &content);
            });
        } else {
            let _ = (id, f);
        }
    }
}
