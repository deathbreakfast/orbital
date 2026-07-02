use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../orbital-theme/src/baseline");
    println!("cargo:rerun-if-changed=../orbital-theme/src/fonts.rs");
    println!("cargo:rerun-if-changed=../orbital-theme/src/theme.rs");
    println!("cargo:rerun-if-changed=../orbital-theme/src/context.rs");
    println!("cargo:rerun-if-env-changed=LEPTOS_BASE_PATH");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.join("..");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let css = generate_baseline_css(&workspace_root);

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

fn generate_baseline_css(workspace_root: &Path) -> String {
    let bin = workspace_root.join("target/debug/write_baseline_css");
    if !bin.exists() {
        let status = Command::new(env!("CARGO"))
            .current_dir(workspace_root)
            .env("CARGO_BUILD_JOBS", "1")
            .args([
                "build",
                "-p",
                "orbital-theme",
                "--bin",
                "write_baseline_css",
                "-j",
                "1",
            ])
            .status()
            .expect("build write_baseline_css");
        if !status.success() {
            panic!("failed to build write_baseline_css");
        }
    }

    let output = Command::new(&bin)
        .env(
            "LEPTOS_BASE_PATH",
            env::var("LEPTOS_BASE_PATH").unwrap_or_default(),
        )
        .output()
        .expect("run write_baseline_css");

    if !output.status.success() {
        panic!(
            "write_baseline_css failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).expect("baseline css utf8")
}
