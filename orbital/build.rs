use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../orbital-theme/src/baseline");
    println!("cargo:rerun-if-changed=../orbital-theme/src/fonts.rs");
    println!("cargo:rerun-if-changed=../orbital-theme/src/theme.rs");
    println!("cargo:rerun-if-changed=../orbital-theme/src/context.rs");
    println!("cargo:rerun-if-env-changed=LEPTOS_BASE_PATH");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.join("..");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let css = generate_baseline_css();

    let out_file = out_dir.join("orbital-theme-baseline.css");
    fs::write(&out_file, &css).expect("write OUT_DIR baseline css");

    let public_file = workspace_root.join("public/orbital-theme-baseline.css");
    if let Some(parent) = public_file.parent() {
        fs::create_dir_all(parent).expect("create public dir");
    }
    fs::write(&public_file, &css).expect("write public baseline css");

    println!(
        "cargo:rustc-env=ORBITAL_THEME_BASELINE_CSS={}",
        out_file.display()
    );
}

fn generate_baseline_css() -> String {
    let font_prefix = match env::var("LEPTOS_BASE_PATH") {
        Ok(base) if !base.is_empty() => {
            format!("{}/fonts", base.trim_end_matches('/'))
        }
        _ => "/fonts".to_string(),
    };

    orbital_theme::default_first_paint_baseline_css_with_font_prefix(
        orbital_theme::ROOT_THEME_SCOPE_ID,
        &font_prefix,
    )
}
