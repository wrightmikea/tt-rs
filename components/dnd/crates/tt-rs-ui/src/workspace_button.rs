//! Workspace button component.
//!
//! A header button that opens the workspace menu for save/load operations.

use yew::prelude::*;

/// Properties for the WorkspaceButton component.
#[derive(Properties, Clone, PartialEq)]
pub struct WorkspaceButtonProps {
    /// Callback when the button is clicked.
    pub on_click: Callback<()>,
}

/// Header button for workspace management.
#[function_component(WorkspaceButton)]
pub fn workspace_button(props: &WorkspaceButtonProps) -> Html {
    let on_click = {
        let callback = props.on_click.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <button class="workspace-button" onclick={on_click} title="Save/Load Workspace">
            { "Workspace" }
        </button>
    }
}
