//! Help button component.
//!
//! A floating button that opens the help panel.

use yew::prelude::*;

/// Properties for the HelpButton component.
#[derive(Properties, Clone, PartialEq)]
pub struct HelpButtonProps {
    /// Callback when the button is clicked.
    pub on_click: Callback<()>,
}

/// Floating help button.
#[function_component(HelpButton)]
pub fn help_button(props: &HelpButtonProps) -> Html {
    let on_click = {
        let callback = props.on_click.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <button class="help-button" onclick={on_click} title="Open Help">
            { "?" }
        </button>
    }
}
