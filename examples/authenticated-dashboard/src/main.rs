//! Boot Axum + Leptos for the authenticated-dashboard host.

use axum::Router;
use authenticated_dashboard::{shell, App};
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("logger");
    any_spawner::Executor::init_futures_executor().expect("futures executor");

    let conf = get_configuration(None).expect("leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let site_root = std::path::PathBuf::from(leptos_options.site_root.as_ref());
    let pkg_dir = site_root.join(leptos_options.site_pkg_dir.as_ref());
    let fonts_dir = site_root.join("fonts");

    let leptos_options_for_routes = leptos_options.clone();
    let leptos_options_state = leptos_options.clone();

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new(pkg_dir))
        .nest_service("/fonts", ServeDir::new(fonts_dir))
        .leptos_routes(&leptos_options, routes, move || {
            shell(leptos_options_for_routes.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options_state);

    log::info!("authenticated-dashboard listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind authenticated-dashboard");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve authenticated-dashboard");
}
