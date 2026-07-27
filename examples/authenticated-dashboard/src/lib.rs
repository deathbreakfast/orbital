//! Auth context + route guards + theme/density persistence host.

mod app;
mod theme_prefs;

pub use app::{shell, App};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);

    std::panic::set_hook(Box::new(|info| {
        orbital::hide_boot_loader();
        console_error_panic_hook::hook(info);
    }));

    leptos::mount::hydrate_body(App);
    orbital::hide_boot_loader();
}
