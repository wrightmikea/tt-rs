//! tt-rs-app: Main WASM application entry point.

mod app;

use app::App;

/// WASM entry point.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("tt-rs starting up");
    yew::Renderer::<App>::new().render();
}
