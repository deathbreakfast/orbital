//! Writes default first-paint baseline CSS to stdout (used by `orbital/build.rs`).

use orbital_theme::{default_first_paint_baseline_css, ROOT_THEME_SCOPE_ID};

fn main() {
    print!("{}", default_first_paint_baseline_css(ROOT_THEME_SCOPE_ID));
}
