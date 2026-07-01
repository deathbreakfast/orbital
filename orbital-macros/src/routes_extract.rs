use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};

/// Expand `#[orbital::routes]` on a route component function.
///
/// Parses the `view! { ... }` body to find `<ParentRoute path=path!("base") ...>`
/// and `<Route path=path!("segment") ...>` elements, then generates a
/// `pub mod paths` alongside the original (unmodified) function.
pub fn expand_routes(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    let tokens: Vec<TokenTree> = input2.clone().into_iter().collect();

    let view_body = find_view_macro_body(&tokens);
    let (parent_path, child_paths) = match view_body {
        Some(body) => extract_routes_from_view(&body),
        None => (None, vec![]),
    };

    let base = parent_path.unwrap_or_default();

    // First pass: collect all items with raw names
    struct ConstItem {
        name: String,
        full_path: String,
    }
    struct FnItem {
        raw_name: String,
        params: Vec<String>,
        format_str: String,
    }

    let mut consts: Vec<ConstItem> = Vec::new();
    let mut fns: Vec<FnItem> = Vec::new();

    for child in &child_paths {
        let full_path = if child.is_empty() {
            format!("/{base}")
        } else {
            format!("/{base}/{child}")
        };

        let params = extract_params(child);

        if params.is_empty() {
            consts.push(ConstItem {
                name: path_to_const_name(child),
                full_path,
            });
        } else {
            let (raw_name, format_str) = build_fn_meta(child, &base);
            fns.push(FnItem {
                raw_name,
                params,
                format_str,
            });
        }
    }

    // Second pass: singularise single-segment function names where safe
    let all_raw: Vec<String> = fns.iter().map(|f| f.raw_name.clone()).collect();
    let const_names: Vec<String> = consts.iter().map(|c| c.name.to_lowercase()).collect();
    let final_fn_names: Vec<String> = fns
        .iter()
        .map(|f| resolve_fn_name(&f.raw_name, &all_raw, &const_names))
        .collect();

    let mut const_items = Vec::new();
    for c in &consts {
        let ident = format_ident!("{}", c.name);
        let fp = &c.full_path;
        const_items.push(quote! {
            pub const #ident: &str = #fp;
        });
    }

    let mut fn_items = Vec::new();
    for (i, f) in fns.iter().enumerate() {
        let fn_ident = safe_ident(&final_fn_names[i]);
        let param_idents: Vec<_> = f.params.iter().map(|p| format_ident!("{}", p)).collect();
        let param_decls: Vec<_> = param_idents.iter().map(|p| quote! { #p: &str }).collect();
        let fmt = &f.format_str;

        fn_items.push(quote! {
            pub fn #fn_ident(#(#param_decls),*) -> String {
                format!(#fmt)
            }
        });
    }

    let paths_mod = quote! {
        pub mod paths {
            #(#const_items)*
            #(#fn_items)*
        }
    };

    let output = quote! {
        #input2
        #paths_mod
    };

    output.into()
}

/// Walk a token stream to find `view ! { ... }` and return the tokens inside
/// the braces.
fn find_view_macro_body(tokens: &[TokenTree]) -> Option<Vec<TokenTree>> {
    for i in 0..tokens.len() {
        if let TokenTree::Ident(ident) = &tokens[i] {
            if ident == "view" {
                // Look for `!` then `{ ... }`
                if i + 2 < tokens.len() {
                    if let TokenTree::Punct(p) = &tokens[i + 1] {
                        if p.as_char() == '!' {
                            if let TokenTree::Group(g) = &tokens[i + 2] {
                                if g.delimiter() == proc_macro2::Delimiter::Brace {
                                    return Some(g.stream().into_iter().collect());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Recurse into groups (the function body is inside braces)
        if let TokenTree::Group(g) = &tokens[i] {
            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
            if let Some(found) = find_view_macro_body(&inner) {
                return Some(found);
            }
        }
    }
    None
}

/// Extract the parent path and child route paths from the view macro body.
fn extract_routes_from_view(tokens: &[TokenTree]) -> (Option<String>, Vec<String>) {
    let mut parent_path: Option<String> = None;
    let mut child_paths: Vec<String> = Vec::new();
    let mut inside_parent_route = false;

    let mut i = 0;
    while i < tokens.len() {
        // Detect `< ParentRoute` or `< Route`
        if let TokenTree::Punct(p) = &tokens[i] {
            if p.as_char() == '<' && i + 1 < tokens.len() {
                if let TokenTree::Ident(ident) = &tokens[i + 1] {
                    let name = ident.to_string();
                    if name == "ParentRoute" {
                        inside_parent_route = true;
                        if let Some(path) = find_path_value(&tokens[i..]) {
                            parent_path = Some(path);
                        }
                    } else if name == "Route" && inside_parent_route {
                        if let Some(path) = find_path_value(&tokens[i..]) {
                            child_paths.push(path);
                        }
                    }
                }
            }
        }
        i += 1;
    }

    (parent_path, child_paths)
}

/// Starting from `<ComponentName`, scan forward to find `path = path ! ( "value" )`
/// and return the string literal value.
fn find_path_value(tokens: &[TokenTree]) -> Option<String> {
    let mut i = 0;
    while i < tokens.len() {
        // Stop at `>` or `/>`
        if let TokenTree::Punct(p) = &tokens[i] {
            if p.as_char() == '>' {
                break;
            }
        }

        // Look for `path = path ! ( "..." )`
        if let TokenTree::Ident(ident) = &tokens[i] {
            if ident == "path" {
                // Expect `=`
                if i + 1 < tokens.len() {
                    if let TokenTree::Punct(eq) = &tokens[i + 1] {
                        if eq.as_char() == '=' {
                            // Expect `path`
                            if i + 2 < tokens.len() {
                                if let TokenTree::Ident(path_macro) = &tokens[i + 2] {
                                    if path_macro == "path" {
                                        // Expect `!`
                                        if i + 3 < tokens.len() {
                                            if let TokenTree::Punct(bang) = &tokens[i + 3] {
                                                if bang.as_char() == '!' {
                                                    // Expect `( "..." )`
                                                    if i + 4 < tokens.len() {
                                                        if let TokenTree::Group(g) = &tokens[i + 4]
                                                        {
                                                            return extract_string_from_group(g);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        i += 1;
    }
    None
}

/// Extract a string literal from a group like `("value")`.
fn extract_string_from_group(group: &proc_macro2::Group) -> Option<String> {
    for tt in group.stream() {
        if let TokenTree::Literal(lit) = tt {
            let s = lit.to_string();
            // Strip surrounding quotes
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                return Some(s[1..s.len() - 1].to_string());
            }
        }
    }
    None
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "yield",
];

/// Create an identifier, using raw identifier syntax (`r#name`) for Rust keywords.
fn safe_ident(name: &str) -> proc_macro2::Ident {
    if RUST_KEYWORDS.contains(&name) {
        format_ident!("r#{}", name)
    } else {
        format_ident!("{}", name)
    }
}

/// Extract `:param` or `*param` segments from a route path string.
fn extract_params(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|seg| seg.starts_with(':') || seg.starts_with('*'))
        .map(|seg| {
            seg.trim_start_matches('*')
                .trim_start_matches(':')
                .to_string()
        })
        .collect()
}

/// Convert a child route path to a SCREAMING_SNAKE_CASE constant name.
///
///  - `""` -> `ROOT`
///  - `"jobs"` -> `JOBS`
///  - `"jobs/new"` -> `JOBS_NEW`
///  - `"high-scores"` -> `HIGH_SCORES`
fn path_to_const_name(path: &str) -> String {
    if path.is_empty() {
        return "ROOT".to_string();
    }
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|seg| {
            let seg = seg.trim_start_matches('*').trim_start_matches(':');
            seg.replace('-', "_").to_uppercase()
        })
        .collect::<Vec<_>>()
        .join("_")
}

/// Build the raw function name (no singularisation) and format string for a
/// parameterised route.
fn build_fn_meta(child: &str, base: &str) -> (String, String) {
    let segments: Vec<&str> = child.split('/').collect();

    let name_parts: Vec<String> = segments
        .iter()
        .filter(|seg| !seg.starts_with(':') && !seg.starts_with('*'))
        .map(|seg| seg.replace('-', "_"))
        .filter(|s| !s.is_empty())
        .collect();

    let raw_name = if name_parts.is_empty() {
        // No static segments (e.g. ":app_name") — derive name from first param.
        let first_param = segments
            .iter()
            .find(|s| s.starts_with(':'))
            .map(|s| &s[1..])
            .unwrap_or("item");
        strip_param_suffix(first_param)
    } else {
        name_parts.join("_")
    };

    let format_segments: Vec<String> = segments
        .iter()
        .map(|seg| {
            if let Some(stripped) = seg.strip_prefix(':') {
                format!("{{{}}}", stripped)
            } else {
                seg.to_string()
            }
        })
        .collect();
    let format_str = format!("/{base}/{}", format_segments.join("/"));

    (raw_name, format_str)
}

/// Strip common suffixes like `_id` and `_name` from a parameter name to
/// produce a shorter function name (e.g. `app_name` → `app`, `job_id` → `job`).
fn strip_param_suffix(param: &str) -> String {
    for suffix in &["_id", "_name", "_slug", "_key"] {
        if let Some(stripped) = param.strip_suffix(suffix) {
            if !stripped.is_empty() {
                return stripped.to_string();
            }
        }
    }
    param.to_string()
}

/// Singularise a name by stripping a trailing 's' when it looks like a simple
/// English plural (e.g. "jobs" → "job"). Returns the original if it doesn't
/// look pluralised.
fn try_singularise(name: &str) -> String {
    if name.ends_with('s') && name.len() > 1 {
        let bytes = name.as_bytes();
        if bytes[bytes.len() - 2] != b's' {
            return name[..name.len() - 1].to_string();
        }
    }
    name.to_string()
}

/// Determine the final function name for a parameterised route, attempting
/// singularisation for single-segment names while avoiding collisions with
/// other functions or constants.
fn resolve_fn_name(raw: &str, all_raw_names: &[String], const_names_lower: &[String]) -> String {
    if !raw.contains('_') {
        let singular = try_singularise(raw);
        if singular != *raw {
            let singular_lower = singular.to_lowercase();
            let collides_const = const_names_lower.contains(&singular_lower);
            let collides_fn = all_raw_names.iter().filter(|n| *n != raw).any(|n| {
                let other_singular = try_singularise(n);
                other_singular == singular || *n == singular
            });
            if !collides_const && !collides_fn {
                return singular;
            }
        }
    }
    raw.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_const_name() {
        assert_eq!(path_to_const_name(""), "ROOT");
        assert_eq!(path_to_const_name("jobs"), "JOBS");
        assert_eq!(path_to_const_name("jobs/new"), "JOBS_NEW");
        assert_eq!(path_to_const_name("high-scores"), "HIGH_SCORES");
        assert_eq!(path_to_const_name("schema"), "SCHEMA");
    }

    #[test]
    fn test_extract_params() {
        assert!(extract_params("jobs").is_empty());
        assert_eq!(extract_params("jobs/:job_id"), vec!["job_id"]);
        assert_eq!(extract_params("schema/:name/id/:eid"), vec!["name", "eid"]);
    }

    #[test]
    fn test_build_fn_meta_single_param() {
        let (name, fmt) = build_fn_meta("jobs/:job_id", "app");
        assert_eq!(name, "jobs");
        assert_eq!(fmt, "/app/jobs/{job_id}");
    }

    #[test]
    fn test_build_fn_meta_multi_param() {
        let (name, fmt) = build_fn_meta("schema/:schema_name/id/:entity_id", "api");
        assert_eq!(name, "schema_id");
        assert_eq!(fmt, "/api/schema/{schema_name}/id/{entity_id}");
    }

    #[test]
    fn test_singularise() {
        assert_eq!(try_singularise("jobs"), "job");
        assert_eq!(try_singularise("runs"), "run");
        assert_eq!(try_singularise("schema"), "schema");
        assert_eq!(try_singularise("class"), "class"); // double-s before
    }

    #[test]
    fn test_resolve_fn_name_no_collision() {
        let all_raw = vec!["jobs".to_string()];
        let consts: Vec<String> = vec![];
        assert_eq!(resolve_fn_name("jobs", &all_raw, &consts), "job");
    }

    #[test]
    fn test_resolve_fn_name_collision_with_other_fn() {
        // "schema" and "schemas" would both singularise to "schema"
        let all_raw = vec!["schema".to_string(), "schemas".to_string()];
        let consts: Vec<String> = vec![];
        assert_eq!(resolve_fn_name("schema", &all_raw, &consts), "schema");
        assert_eq!(resolve_fn_name("schemas", &all_raw, &consts), "schemas");
    }
}
