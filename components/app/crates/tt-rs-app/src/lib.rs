//! tt-rs-app: Main WASM application entry point.

mod app;
mod box_state;
mod demo;
mod demo_ops;
mod demo_runner;
mod ops;
mod robot_exec;
pub mod routing;
mod state;
mod widget_item;
pub mod workspace;

use app::App;

/// WASM entry point.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("tt-rs starting up");
    yew::Renderer::<App>::new().render();
}
