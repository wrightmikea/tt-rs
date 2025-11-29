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
    /// Optional callback when the copy source is moved (for repositioning stacks).
    #[prop_or_default]
    pub on_move: Option<Callback<(WidgetId, Position)>>,
    /// The content to render (the widget visual).
    pub children: Children,
}

/// A copy source component - renders as a stack and creates copies when clicked.
/// The stack can be dragged to reposition it; clicking (without drag) creates a copy.
#[function_component(CopySource)]
pub fn copy_source(props: &CopySourceProps) -> Html {
    let is_dragging = use_state(|| false);
    let drag_start = use_state(|| Position::new(0.0, 0.0));
    let start_pos = use_state(|| Position::new(0.0, 0.0));
    let current_pos = use_state(|| props.position);

    // Update position when props change
    {
        let current_pos = current_pos.clone();
        let prop_pos = props.position;
        use_effect_with(prop_pos, move |pos| {
            current_pos.set(*pos);
            || ()
        });
    }

    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        let drag_start = drag_start.clone();
        let start_pos = start_pos.clone();
        let current_pos = current_pos.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_dragging.set(true);
            let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
            drag_start.set(mouse_pos);
            start_pos.set(*current_pos);
        })
    };

    let on_mouse_move = {
        let is_dragging = is_dragging.clone();
        let drag_start = drag_start.clone();
        let start_pos = start_pos.clone();
        let current_pos = current_pos.clone();

        Callback::from(move |e: MouseEvent| {
            if *is_dragging {
                let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
                let delta_x = mouse_pos.x - drag_start.x;
                let delta_y = mouse_pos.y - drag_start.y;
                let new_pos = Position::new(start_pos.x + delta_x, start_pos.y + delta_y);
                current_pos.set(new_pos);
            }
        })
    };

    let on_mouse_up = {
        let is_dragging = is_dragging.clone();
        let drag_start = drag_start.clone();
        let current_pos = current_pos.clone();
        let on_click = props.on_click.clone();
        let on_move = props.on_move.clone();
        let widget_id = props.widget_id;
        let prop_position = props.position;

        Callback::from(move |e: MouseEvent| {
            if *is_dragging {
                is_dragging.set(false);
                let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
                let delta_x = (mouse_pos.x - drag_start.x).abs();
                let delta_y = (mouse_pos.y - drag_start.y).abs();

                // If moved less than 5 pixels, treat as click (create copy)
                if delta_x < 5.0 && delta_y < 5.0 {
                    on_click.emit(CopySourceClickEvent {
                        source_id: widget_id,
                        position: prop_position,
                        mouse_position: mouse_pos,
                    });
                } else if let Some(ref cb) = on_move {
                    // Moved enough - update position
                    cb.emit((widget_id, *current_pos));
                }
            }
        })
    };

    // z-index handled by CSS (.copy-source-stack has z-index: 1, lower than widgets)
    let style = format!(
        "position: absolute; left: {}px; top: {}px; cursor: grab;",
        current_pos.x, current_pos.y
    );

    html! {
        <div
            class="copy-source-stack"
            style={style}
            onmousedown={on_mouse_down}
            onmousemove={on_mouse_move}
            onmouseup={on_mouse_up.clone()}
            onmouseleave={on_mouse_up}
        >
            { for props.children.iter() }
        </div>
    }
}
