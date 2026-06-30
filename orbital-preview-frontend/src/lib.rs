#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);

    #[cfg(feature = "hydrate")]
    {
        std::panic::set_hook(Box::new(|info| {
            orbital::hide_boot_loader();
            console_error_panic_hook::hook(info);
        }));

        leptos::mount::hydrate_body(orbital_preview_app::App);
        orbital::hide_boot_loader();
    }

    #[cfg(not(feature = "hydrate"))]
    {
        console_error_panic_hook::set_once();
    }
}
