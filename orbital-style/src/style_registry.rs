use leptos::{
    attr::{any_attribute::AnyAttribute, Attribute},
    context::{Provider, ProviderProps},
    prelude::*,
    tachys::{
        hydration::Cursor,
        renderer::types,
        view::{
            any_view::{AnyViewState, AnyViewWithAttrs},
            Position, PositionState,
        },
    },
};
use std::collections::HashMap;

const STYLE_MARKER: &str = r#"<meta name="orbital-style""#;

/// Collects component-injected styles during SSR and streams them into `<head>`.
#[component]
pub fn StyleRegistry<Chil>(children: TypedChildren<Chil>) -> impl IntoView
where
    Chil: IntoView + 'static,
{
    let context = StyleRegistryContext::default();

    let children = Provider(
        ProviderProps::builder()
            .value(context.clone())
            .children(children)
            .build(),
    )
    .into_any();
    StyleRegistryView { context, children }
}

#[derive(Debug, Clone)]
pub struct StyleRegistryContext {
    styles: ArcStoredValue<HashMap<String, String>>,
}

impl StyleRegistryContext {
    pub fn use_context() -> Option<Self> {
        use_context()
    }

    pub fn push_style(&self, k: String, v: String) {
        // Ignore late injections after the SSR/hydrate owner is gone — panicking
        // here aborts the whole Axum worker (Playwright sees ERR_EMPTY_RESPONSE).
        if let Some(mut styles) = self.styles.try_write_value() {
            styles.insert(k, v);
        }
    }

    /// Upserts a collected stylesheet into `<head>` during client hydration.
    #[cfg(feature = "hydrate")]
    pub fn ensure_style_in_head(&self, id: &str, content: &str) {
        use leptos::prelude::document;

        let Some(head) = document().head() else {
            return;
        };
        let style = match head.query_selector(&format!("style#{id}")) {
            Ok(Some(existing)) => existing,
            Ok(None) => {
                let Ok(style) = document().create_element("style") else {
                    return;
                };
                let _ = style.set_attribute("id", id);

                let orbital_meta = head.query_selector(STYLE_MARKER).ok().flatten();
                if let Some(orbital_meta) = orbital_meta {
                    let _ = head.insert_before(&style, Some(&orbital_meta));
                } else {
                    let _ = head.prepend_with_node_1(&style);
                }
                style
            }
            Err(_) => return,
        };

        style.set_text_content(Some(content));
    }

    fn default() -> Self {
        Self {
            styles: Default::default(),
        }
    }

    fn html_len(&self) -> usize {
        const TEMPLATE_LEN: usize = r#"<style id=""></style>"#.len();
        let mut html_len = 0;
        let Some(styles) = self.styles.try_write_value() else {
            return 0;
        };

        styles.iter().for_each(|(k, v)| {
            html_len += k.len() + v.len() + TEMPLATE_LEN;
        });

        html_len
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_html(self) -> String {
        let Some(mut styles) = self.styles.try_write_value() else {
            return String::new();
        };
        styles
            .drain()
            .map(|(k, v)| format!(r#"<style id="{k}">{v}</style>"#))
            .collect::<String>()
    }
}

struct StyleRegistryView {
    context: StyleRegistryContext,
    children: AnyView,
}

struct StyleRegistryState {
    state: AnyViewState,
}

impl Render for StyleRegistryView {
    type State = StyleRegistryState;

    fn build(self) -> Self::State {
        let state = self.children.build();
        StyleRegistryState { state }
    }

    fn rebuild(self, state: &mut Self::State) {
        self.children.rebuild(&mut state.state);
    }
}

impl AddAnyAttr for StyleRegistryView {
    type Output<SomeNewAttr: Attribute> = AnyViewWithAttrs;

    fn add_any_attr<NewAttr: Attribute>(self, attr: NewAttr) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        self.children.add_any_attr(attr)
    }
}

impl RenderHtml for StyleRegistryView {
    type AsyncOutput = Self;
    type Owned = Self;

    const MIN_LENGTH: usize = 0;

    fn html_len(&self) -> usize {
        self.children.html_len() + self.context.html_len()
    }

    fn dry_resolve(&mut self) {
        self.children.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        self.children
            .to_html_with_buf(buf, position, escape, mark_branches, extra_attrs);

        let Some(head_loc) = buf.find("<head>") else {
            leptos::logging::warn!(
                "StyleRegistry: no <head> tag in SSR buffer; skipping style injection"
            );
            return;
        };
        let marker_loc = buf.find(STYLE_MARKER).unwrap_or(head_loc + 6);

        buf.insert_str(marker_loc, &self.context.to_html());
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut leptos::tachys::ssr::StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        self.children.to_html_async_with_buf::<OUT_OF_ORDER>(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );

        buf.with_buf(|buf| {
            let Some(head_loc) = buf.find("<head>") else {
                leptos::logging::warn!(
                    "StyleRegistry: no <head> tag in SSR buffer; skipping style injection"
                );
                return;
            };
            let marker_loc = buf.find(STYLE_MARKER).unwrap_or(head_loc + 6);
            buf.insert_str(marker_loc, &self.context.to_html());
        });
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let state = self.children.hydrate::<FROM_SERVER>(cursor, position);
        StyleRegistryState { state }
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl Mountable for StyleRegistryState {
    fn unmount(&mut self) {
        self.state.unmount();
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        self.state.mount(parent, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.state.insert_before_this(child)
    }

    fn elements(&self) -> Vec<types::Element> {
        self.state.elements()
    }
}
