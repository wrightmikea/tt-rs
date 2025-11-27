//! Main application component.

use tt_rs_core::Widget;
use tt_rs_number::{ArithOperator, Number};
use tt_rs_ui::Footer;
use yew::prelude::*;

/// Creates sample numbers for demonstration.
fn demo_numbers() -> [Number; 7] {
    [
        Number::new(5),
        Number::new(3).with_operator(ArithOperator::Subtract),
        Number::new(2).with_operator(ArithOperator::Multiply),
        Number::new(4).with_operator(ArithOperator::Divide),
        Number::rational(1, 2),
        Number::rational(3, 4),
        Number::erased(),
    ]
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    let numbers = demo_numbers();

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
            <Footer />
        </div>
    }
}
