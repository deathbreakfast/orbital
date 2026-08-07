use syn::{Expr, Lit, Meta, Path};

/// Parsed `#[component_doc(...)]` preview and documentation attributes.
///
/// ## Sections
///
/// | Section | Default priority | Use for |
/// |---------|------------------|---------|
/// | *(none)* | 0 | Introduction (`nav_item = true`) |
/// | Getting Started | 1 | Theme, Typography |
/// | Core Components | 2 | Native Orbital components |
/// | Motion | 3 | Motion tokens, presence, choreography (collapsed by default) |
/// | Foundation | 4 | Foundation catalog components |
/// | Charts | 5 | Chart catalog |
/// | Data Table | 6 | Data table catalog |
/// | Tree | 7 | Tree view |
/// | Scheduling | 8 | Scheduler catalog |
/// | Discussion | 9 | Discussion catalog |
/// | History | 10 | History timeline catalog |
///
/// Section is inferred from `file!()` path when not set explicitly. Typography category
/// maps to Getting Started.
///
/// ## Categories under Core Components
///
/// | Category | Default priority | Default collapsed |
/// |----------|------------------|-------------------|
/// | Layout | 10 | yes |
/// | Surfaces | 20 | yes |
/// | Inputs | 30 | yes |
/// | Feedback | 40 | yes |
/// | Data Display | 50 | yes |
/// | Navigation | 60 | yes |
/// | Calendar & Time | 70 | yes |
/// | Shell | 80 | yes |
/// | Patterns | 90 | yes |
/// | Motion | 100 | yes |
/// | Integrations | 110 | yes |
///
/// Nested group folders (Card, Buttons, Pickers, Chart Types, …) also start collapsed.
/// Preview hosts open only ancestors of the current slug — no section defaults open.
///
/// Nested `group` values (Card, Buttons, Chart Types, …) are inferred from `preview_slug`
/// via [`category_defaults`](crate::component_doc::category_defaults) when omitted.
///
/// Override per component with `section_priority`, `category_priority`, `group_priority`,
/// or `category_default_collapsed` only when the defaults table is wrong.
#[derive(Debug, Default, Clone)]
pub struct ComponentDocAttrs {
    pub section: Option<String>,
    pub section_priority: Option<u16>,
    pub category: Option<String>,
    pub category_priority: Option<u16>,
    pub category_default_collapsed: Option<bool>,
    pub group: Option<String>,
    pub group_priority: Option<u16>,
    pub nav_item: bool,
    pub preview_slug: Option<String>,
    pub preview_label: Option<String>,
    pub preview_icon: Option<Path>,
    pub preview_import: Option<Path>,
    pub props_import: Option<Path>,
    pub preview_manual: bool,
}

impl ComponentDocAttrs {
    pub fn parse(metas: &syn::punctuated::Punctuated<Meta, syn::Token![,]>) -> syn::Result<Self> {
        let mut attrs = Self::default();
        for meta in metas {
            match meta {
                Meta::NameValue(nv) if nv.path.is_ident("section") => {
                    attrs.section = Some(lit_to_string(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("section_priority") => {
                    attrs.section_priority = Some(lit_to_u16(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("category") => {
                    attrs.category = Some(lit_to_string(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("category_priority") => {
                    attrs.category_priority = Some(lit_to_u16(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("category_default_collapsed") => {
                    attrs.category_default_collapsed = Some(lit_to_bool(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("group") => {
                    attrs.group = Some(lit_to_string(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("group_priority") => {
                    attrs.group_priority = Some(lit_to_u16(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("nav_item") => {
                    attrs.nav_item = lit_to_bool(&nv.value)?;
                }
                Meta::Path(path) if path.is_ident("nav_item") => {
                    attrs.nav_item = true;
                }
                Meta::NameValue(nv) if nv.path.is_ident("preview_slug") => {
                    attrs.preview_slug = Some(lit_to_string(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("preview_label") => {
                    attrs.preview_label = Some(lit_to_string(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("preview") => {
                    let mode = lit_to_string(&nv.value)?;
                    attrs.preview_manual = mode == "manual";
                }
                Meta::NameValue(nv) if nv.path.is_ident("preview_icon") => {
                    attrs.preview_icon = Some(expr_to_path(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("preview_import") => {
                    attrs.preview_import = Some(expr_to_path(&nv.value)?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("props_import") => {
                    attrs.props_import = Some(expr_to_path(&nv.value)?);
                }
                Meta::Path(path) if path.is_ident("preview") => {
                    attrs.preview_manual = false;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "unknown `#[component_doc]` attribute",
                    ));
                }
            }
        }
        Ok(attrs)
    }

    pub fn is_preview_enabled(&self) -> bool {
        !self.preview_manual && self.preview_slug.is_some()
    }
}

fn lit_to_string(expr: &Expr) -> syn::Result<String> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(s) => Ok(s.value()),
            other => Err(syn::Error::new_spanned(other, "expected string literal")),
        },
        other => Err(syn::Error::new_spanned(other, "expected string literal")),
    }
}

fn lit_to_u16(expr: &Expr) -> syn::Result<u16> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(int) => int.base10_parse(),
            other => Err(syn::Error::new_spanned(other, "expected integer literal")),
        },
        other => Err(syn::Error::new_spanned(other, "expected integer literal")),
    }
}

fn lit_to_bool(expr: &Expr) -> syn::Result<bool> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Bool(value) => Ok(value.value()),
            other => Err(syn::Error::new_spanned(other, "expected bool literal")),
        },
        other => Err(syn::Error::new_spanned(other, "expected bool literal")),
    }
}

fn expr_to_path(expr: &Expr) -> syn::Result<Path> {
    match expr {
        Expr::Path(expr_path) => Ok(expr_path.path.clone()),
        other => Err(syn::Error::new_spanned(other, "expected type path")),
    }
}
