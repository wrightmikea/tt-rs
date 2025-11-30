//! HTML rendering for DropZone.

use crate::dropzone::DropZone;
use yew::prelude::*;

/// Renders a DropZone as HTML.
pub fn render(dropzone: &DropZone) -> Html {
    let widget_id = dropzone.id.to_string();
    let satisfied = dropzone.is_satisfied();
    let show_error = dropzone.show_error();

    let class = if satisfied {
        "widget dropzone satisfied"
    } else if show_error {
        "widget dropzone error"
    } else {
        "widget dropzone"
    };

    html! {
        <div class={class}
             data-widget-id={widget_id}
             data-widget-type="dropzone">
            if satisfied {
                <div class="dropzone-success">{ "\u{2713}" }</div>
            } else if show_error {
                <div class="dropzone-error">{ "\u{2717}" }</div>
            }
            <div class="dropzone-label">
                { &dropzone.label }
            </div>
            if satisfied {
                // No hint when satisfied
            } else if show_error {
                <div class="dropzone-hint error">{ "Try again!" }</div>
            } else {
                <div class="dropzone-hint">{ "Drop your answer here" }</div>
            }
        </div>
    }
}
