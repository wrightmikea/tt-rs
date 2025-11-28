//! Copy source component - a static stack that creates draggable copies.

use crate::Position;
use tt_rs_core::WidgetId;
use web_sys::MouseEvent;
use yew::prelude::*;

/// Event emitted when a copy source is clicked to create a new copy.
#[derive(Debug, Clone, Copy)]
pub struct CopySourceClickEvent {
    /// The ID of the copy source widget.
    pub source_id: WidgetId,
    /// Position of the copy source.
    pub position: Position,
    /// Mouse position at click time.
    pub mouse_position: Position,
}

/// Props for the CopySource component.
#[derive(Properties, PartialEq)]
pub struct CopySourceProps {
    /// The widget ID of the copy source.
    pub widget_id: WidgetId,
    /// Position of the copy source.
    pub position: Position,
    /// Callback when the copy source is clicked (to create a copy).
    pub on_click: Callback<CopySourceClickEvent>,
    /// The content to render (the widget visual).
    pub children: Children,
}

/// A copy source component - renders as a stack and creates copies when clicked.
/// The stack stays in place; clicking creates a new draggable widget.
#[function_component(CopySource)]
pub fn copy_source(props: &CopySourceProps) -> Html {
    let on_mouse_down = {
        let on_click = props.on_click.clone();
        let widget_id = props.widget_id;
        let position = props.position;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
            on_click.emit(CopySourceClickEvent {
                source_id: widget_id,
                position,
                mouse_position: mouse_pos,
            });
        })
    };

    let style = format!(
        "position: absolute; left: {}px; top: {}px; cursor: grab;",
        props.position.x, props.position.y
    );

    html! {
        <div
            class="copy-source-stack"
            style={style}
            onmousedown={on_mouse_down}
        >
            { for props.children.iter() }
        </div>
    }
}
