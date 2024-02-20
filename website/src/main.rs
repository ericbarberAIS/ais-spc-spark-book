mod components;
mod models;
mod pages;
mod router;
mod services;
// mod content;

use crate::components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
