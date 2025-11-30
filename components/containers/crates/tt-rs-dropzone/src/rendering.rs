//! HTML rendering for DropZone.

use crate::dropzone::DropZone;
use yew::prelude::*;

/// Renders a DropZone as HTML.
pub fn render(dropzone: &DropZone) -> Html {
    let widget_id = dropzone.id.to_string();
    let satisfied = dropzone.is_satisfied();

    let class = if satisfied {
        "widget dropzone satisfied"
    } else {
        "widget dropzone"
    };

    html! {
        <div class={class}
             data-widget-id={widget_id}
             data-widget-type="dropzone">
            if satisfied {
                <div class="dropzone-success">{ "\u{2713}" }</div>
            }
            <div class="dropzone-label">
                { &dropzone.label }
            </div>
            if !satisfied {
                <div class="dropzone-hint">{ "Drop your answer here" }</div>
            }
        </div>
    }
}
