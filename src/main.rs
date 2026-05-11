use dioxus::prelude::*;

mod app;
mod components;
mod error;
mod models;
mod pages;
mod services;
mod utils;

fn main() {
    dioxus::launch(app::App);
}
