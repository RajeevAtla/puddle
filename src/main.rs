mod app;
mod browser;
mod components;
mod error;
mod models;
mod pages;
mod services;
mod utils;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    dioxus::launch(app::App);
}
