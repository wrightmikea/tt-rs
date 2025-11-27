//! Draggable component wrapper.

use crate::Position;
use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use yew::prelude::*;

/// Props for the Draggable component.
#[derive(Properties, PartialEq)]
pub struct DraggableProps {
    pub widget_id: WidgetId,
    pub position: Position,
    pub on_move: Callback<(WidgetId, Position)>,
    pub children: Children,
}

/// State for drag operation.
#[derive(Default)]
struct DragState {
    dragging: bool,
    start_mouse: Position,
    start_pos: Position,
}

/// A wrapper component that makes its children draggable.
#[function_component(Draggable)]
pub fn draggable(props: &DraggableProps) -> Html {
    let drag_state = use_state(DragState::default);

    let style = format!(
        "position: absolute; left: {}px; top: {}px;",
        props.position.x, props.position.y
    );

    let on_mouse_down = {
        let drag_state = drag_state.clone();
        let position = props.position;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            drag_state.set(DragState {
                dragging: true,
                start_mouse: Position::new(e.client_x() as f64, e.client_y() as f64),
                start_pos: position,
            });
        })
    };

    let on_mouse_move = {
        let drag_state = drag_state.clone();
        let on_move = props.on_move.clone();
        let widget_id = props.widget_id;
        Callback::from(move |e: MouseEvent| {
            if drag_state.dragging {
                let dx = e.client_x() as f64 - drag_state.start_mouse.x;
                let dy = e.client_y() as f64 - drag_state.start_mouse.y;
                let new_pos = drag_state.start_pos.offset(dx, dy);
                on_move.emit((widget_id, new_pos));
            }
        })
    };

    let on_mouse_up = {
        let drag_state = drag_state.clone();
        Callback::from(move |_: MouseEvent| {
            if drag_state.dragging {
                drag_state.set(DragState::default());
            }
        })
    };

    // Attach global mouse listeners when dragging
    use_effect_with(drag_state.dragging, move |dragging| {
        if *dragging {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Clone for closure
            let on_move = on_mouse_move.clone();
            let on_up = on_mouse_up.clone();

            let move_closure =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
                    on_move.emit(e);
                }) as Box<dyn FnMut(_)>);

            let up_closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
                on_up.emit(e);
            })
                as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .add_event_listener_with_callback("mouseup", up_closure.as_ref().unchecked_ref())
                .unwrap();

            move_closure.forget();
            up_closure.forget();
        }
        || {}
    });

    html! {
        <div
            class="draggable"
            style={style}
            onmousedown={on_mouse_down}
        >
            { for props.children.iter() }
        </div>
    }
}
