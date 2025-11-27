//! tt-rs: Cartoon-oriented Talking Programming Application
//!
//! A visual programming environment where users create programs by training
//! robots through demonstration.

mod domain;

use domain::{ArithOperator, Number, Widget};
use yew::prelude::*;

/// Main application component representing the ToonTalk workspace.
#[function_component(App)]
pub fn app() -> Html {
    // Create sample numbers demonstrating different operators
    let add_num = Number::new(5);
    let sub_num = Number::new(3).with_operator(ArithOperator::Subtract);
    let mul_num = Number::new(2).with_operator(ArithOperator::Multiply);
    let div_num = Number::new(4).with_operator(ArithOperator::Divide);

    // Demonstrate arithmetic: 10 + 5 = 15
    let base = Number::new(10);
    let result = add_num.apply_to(&base).unwrap();

    // Log the arithmetic result
    log::info!(
        "Arithmetic demo: {} + {} = {}/{}",
        base.numerator(),
        add_num.numerator(),
        result.numerator(),
        result.denominator()
    );

    // Show the operator accessor is used
    let _ = add_num.operator();

    let numbers = [
        add_num,
        sub_num,
        mul_num,
        div_num,
        Number::rational(1, 2),
        Number::rational(3, 4),
        Number::erased(),
    ];

    html! {
        <div class="workspace">
            <div class="workspace-header">
                {"tt-rs - Visual Programming Environment"}
            </div>
            <div class="workspace-content">
                <div class="widget-container">
                    { for numbers.iter().map(|n| n.render()) }
                </div>
            </div>
        </div>
    }
}

/// WASM entry point - initializes logging and mounts the Yew application.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("tt-rs starting up");
    yew::Renderer::<App>::new().render();
}
