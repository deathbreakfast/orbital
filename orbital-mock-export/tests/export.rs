//! Runs the built `orbital-mock-export` binary end to end and checks what it writes.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// Exports once into a fresh directory under Cargo's test temp dir and returns it.
fn exported() -> &'static Path {
    static OUT: OnceLock<PathBuf> = OnceLock::new();
    OUT.get_or_init(|| {
        let out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("orbital-mock-export");
        if out.exists() {
            std::fs::remove_dir_all(&out).expect("clear previous export");
        }
        let status = Command::new(env!("CARGO_BIN_EXE_orbital-mock-export"))
            .arg(&out)
            .status()
            .expect("run orbital-mock-export");
        assert!(status.success(), "orbital-mock-export exited with {status}");
        out
    })
}

fn read(name: &str) -> String {
    let path = exported().join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn export_writes_css_tokens_and_fonts_happy() {
    let css = read("orbital.css");
    for class in [
        ".orbital-button",
        ".orbital-dialog-surface",
        ".orbital-text",
    ] {
        assert!(css.contains(class), "orbital.css is missing {class}");
    }

    let tokens = read("tokens.css");
    assert!(tokens.contains(":root, .orbital-theme-provider, [data-theme=\"light\"] {"));
    assert!(tokens.contains("@media (prefers-color-scheme: dark)"));
    assert!(tokens.contains("[data-theme=\"dark\"] {"));
    assert!(
        tokens.contains("--orb-"),
        "tokens.css has no --orb-* variables"
    );

    let fonts = exported().join("fonts");
    let entries = std::fs::read_dir(&fonts)
        .unwrap_or_else(|err| panic!("read {}: {err}", fonts.display()))
        .count();
    assert!(entries > 0, "fonts/ is empty");
}

#[test]
fn export_has_no_theme_or_duplicate_blocks_sad() {
    let css = read("orbital.css");
    assert!(
        !css.contains("orbital-style-orbital-theme-"),
        "theme variable blocks belong in tokens.css, not orbital.css"
    );
    assert!(!css.contains("orbital-style-orbital-font-faces"));

    // Each block sits under a `/* <id> */` comment; id'd blocks must appear once.
    let mut seen = HashSet::new();
    for id in css
        .lines()
        .filter_map(|line| line.strip_prefix("/* orbital-style-")?.strip_suffix(" */"))
    {
        assert!(
            seen.insert(id),
            "orbital.css repeats block orbital-style-{id}"
        );
    }
    assert!(!seen.is_empty(), "orbital.css has no id'd component blocks");
}
