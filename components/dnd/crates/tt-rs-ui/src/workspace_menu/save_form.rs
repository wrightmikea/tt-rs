//! Save form for creating/updating workspaces.

use crate::user_level::UserLevel;
use yew::prelude::*;

/// Data from the save form.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SaveFormData {
    /// Workspace name.
    pub name: String,
    /// Workspace description.
    pub description: String,
    /// User level when saved.
    pub user_level: String,
}

/// Properties for the SaveForm component.
#[derive(Properties, Clone, PartialEq)]
pub struct SaveFormProps {
    pub current_level: UserLevel,
    pub on_submit: Callback<SaveFormData>,
    pub on_cancel: Callback<()>,
}

/// Save form component.
#[function_component(SaveForm)]
pub fn save_form(props: &SaveFormProps) -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);

    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_description_input = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            description.set(textarea.value());
        })
    };

    let on_save_click = {
        let name = name.clone();
        let description = description.clone();
        let level = props.current_level.name().to_string();
        let on_submit = props.on_submit.clone();
        Callback::from(move |_| {
            if !name.is_empty() {
                on_submit.emit(SaveFormData {
                    name: (*name).clone(),
                    description: (*description).clone(),
                    user_level: level.clone(),
                });
            }
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    let is_valid = !name.is_empty();

    html! {
        <div class="save-form">
            <div class="form-group">
                <label class="form-label" for="workspace-name">{ "Name:" }</label>
                <input
                    type="text"
                    id="workspace-name"
                    class="form-input"
                    placeholder="My Workspace"
                    value={(*name).clone()}
                    oninput={on_name_input}
                />
            </div>

            <div class="form-group">
                <label class="form-label" for="workspace-description">{ "Description:" }</label>
                <textarea
                    id="workspace-description"
                    class="form-textarea"
                    rows="5"
                    placeholder="Describe your workspace, include instructions for tutorials..."
                    value={(*description).clone()}
                    oninput={on_description_input}
                />
            </div>

            <div class="form-group">
                <label class="form-label">{ "User Level:" }</label>
                <span class="form-value">{ format!("{} (automatically set)", props.current_level.name()) }</span>
            </div>

            <div class="form-actions">
                <button class="form-btn cancel-btn" onclick={on_cancel_click}>
                    { "Cancel" }
                </button>
                <button
                    class="form-btn save-btn"
                    onclick={on_save_click}
                    disabled={!is_valid}
                >
                    { "Save" }
                </button>
            </div>
        </div>
    }
}

/// Render the save form view (wrapper for function component).
pub fn render_save_form(
    current_level: UserLevel,
    on_submit: Callback<SaveFormData>,
    on_cancel: Callback<()>,
) -> Html {
    html! {
        <SaveForm
            current_level={current_level}
            on_submit={on_submit}
            on_cancel={on_cancel}
        />
    }
}
