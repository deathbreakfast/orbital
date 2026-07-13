mod constants;
mod preview;
mod registry;

#[cfg(test)]
mod expand_smoke;

use quote::quote;
use syn::ItemFn;

use super::attrs::ComponentDocAttrs;
use super::doc_raw;
use super::props;
use super::sections;

use syn::spanned::Spanned;

/// True when expanding `#[component_doc]` for the `orbital-motion` package.
///
/// Prefer `CARGO_PKG_NAME` (set for the consumer during expansion). Fall back to
/// path segments so packaged verify dirs like `orbital-motion-0.1.1/` still match.
fn is_orbital_motion_consumer(source_path: &str) -> bool {
    if std::env::var_os("CARGO_PKG_NAME").is_some_and(|n| n == "orbital-motion") {
        return true;
    }
    source_path.split('/').any(|seg| {
        seg == "orbital-motion" || seg.starts_with("orbital-motion-")
    })
}

pub fn expand(attrs: &ComponentDocAttrs, input_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut attrs = attrs.clone();
    let mut input_fn = input_fn.clone();
    doc_raw::sanitize_doc_attrs_for_doctest(&mut input_fn.attrs);

    let fn_name = &input_fn.sig.ident;
    let source_path = super::category_defaults::caller_source_path(input_fn.span());
    // Prefer CARGO_PKG_NAME: `cargo publish` verifies from
    // `target/package/orbital-motion-<version>/`, which does not contain `orbital-motion/`.
    // Keep a path-segment fallback for older toolchains / odd layouts.
    if attrs.props_import.is_none() && is_orbital_motion_consumer(&source_path) {
        attrs.props_import = Some(syn::parse_quote!(crate::preview::ComponentPropDoc));
    }
    let doc_comments = doc_raw::extract_doc_comments(&input_fn.attrs);
    let props = props::extract_props(&input_fn.sig.inputs);
    let model = sections::parse_doc_string(&doc_comments);

    let base = constants::emit_base(&attrs, fn_name, &doc_comments, &props, &model);
    let preview = if attrs.is_preview_enabled() {
        preview::emit_preview(&attrs, fn_name, &model, &doc_comments, &source_path)
    } else {
        quote! {}
    };

    let expanded = quote! {
        #input_fn
        #base
        #preview
    };

    expanded
}

#[cfg(test)]
mod consumer_detect_tests {
    use super::is_orbital_motion_consumer;

    #[test]
    fn detects_workspace_and_packaged_motion_paths() {
        assert!(is_orbital_motion_consumer(
            "/home/dev/orbital/orbital-motion/src/group.rs"
        ));
        assert!(is_orbital_motion_consumer(
            "/home/dev/orbital/target/package/orbital-motion-0.1.1/src/group.rs"
        ));
        assert!(!is_orbital_motion_consumer(
            "/home/dev/orbital/orbital-charts/src/lib.rs"
        ));
        assert!(!is_orbital_motion_consumer(
            "/home/dev/orbital/orbital-motionless/src/lib.rs"
        ));
    }
}
