//! Confirmation dialog component.
//!
//! A modal dialog that asks the user to confirm or cancel an action.

use yew::prelude::*;

/// Properties for the ConfirmDialog component.
#[derive(Properties, Clone, PartialEq)]
pub struct ConfirmDialogProps {
    /// The message to display in the dialog.
    pub message: AttrValue,
    /// Callback when user confirms.
    pub on_confirm: Callback<()>,
    /// Callback when user cancels.
    pub on_cancel: Callback<()>,
    /// Optional title for the dialog (default: "Confirm").
    #[prop_or("Confirm".into())]
    pub title: AttrValue,
    /// Label for the confirm button (default: "OK").
    #[prop_or("OK".into())]
    pub confirm_label: AttrValue,
    /// Label for the cancel button (default: "Cancel").
    #[prop_or("Cancel".into())]
    pub cancel_label: AttrValue,
}

/// A modal confirmation dialog.
#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let on_confirm = {
        let cb = props.on_confirm.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let on_cancel = {
        let cb = props.on_cancel.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    // Also cancel on backdrop click
    let on_backdrop_click = {
        let cb = props.on_cancel.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    // Stop propagation on dialog click to prevent backdrop handling
    let on_dialog_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class="confirm-dialog-backdrop" onclick={on_backdrop_click}>
            <div class="confirm-dialog" onclick={on_dialog_click}>
                <div class="confirm-dialog-header">
                    <span class="confirm-dialog-title">{ &props.title }</span>
                </div>
                <div class="confirm-dialog-body">
                    <p class="confirm-dialog-message">{ &props.message }</p>
                </div>
                <div class="confirm-dialog-footer">
                    <button
                        class="confirm-dialog-btn cancel-btn"
                        onclick={on_cancel}
                    >
                        { &props.cancel_label }
                    </button>
                    <button
                        class="confirm-dialog-btn confirm-btn"
                        onclick={on_confirm}
                    >
                        { &props.confirm_label }
                    </button>
                </div>
            </div>
        </div>
    }
}
