//! Draggable component wrapper.

use crate::Position;
use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;
use yew::prelude::*;

/// Information about a drop event.
#[derive(Debug, Clone, Copy)]
pub struct DropEvent {
    /// The ID of the widget that was dropped.
    pub widget_id: WidgetId,
    /// The final position of the widget.
    pub position: Position,
    /// The mouse position where the drop occurred.
    pub mouse_position: Position,
}

/// Props for the Draggable component.
#[derive(Properties, PartialEq)]
pub struct DraggableProps {
    pub widget_id: WidgetId,
    pub position: Position,
    pub on_move: Callback<(WidgetId, Position)>,
    /// Optional callback for when the drag ends (drop occurs).
    #[prop_or_default]
    pub on_drop: Option<Callback<DropEvent>>,
    pub children: Children,
}

/// State for drag operation stored in RefCell for interior mutability.
#[derive(Default, Clone)]
struct DragState {
    dragging: bool,
    start_mouse: Position,
    start_pos: Position,
}

/// Type alias for the pair of closures used for drag event listeners.
type DragClosures = (
    Closure<dyn FnMut(MouseEvent)>,
    Closure<dyn FnMut(MouseEvent)>,
);

/// A wrapper component that makes its children draggable.
#[function_component(Draggable)]
pub fn draggable(props: &DraggableProps) -> Html {
    // Use Rc<RefCell<>> for shared mutable state across closures
    let drag_state = use_mut_ref(DragState::default);

    // Track current position for rendering
    let current_pos = props.position;

    let style = format!(
        "position: absolute; left: {}px; top: {}px;",
        current_pos.x, current_pos.y
    );

    // Store closures using use_mut_ref instead of use_state
    // This prevents Yew from dropping them during re-renders
    let closures: Rc<RefCell<Option<DragClosures>>> = use_mut_ref(|| None);

    let on_mouse_down = {
        let drag_state = drag_state.clone();
        let closures = closures.clone();
        let on_move = props.on_move.clone();
        let on_drop = props.on_drop.clone();
        let widget_id = props.widget_id;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // Set drag state
            {
                let mut state = drag_state.borrow_mut();
                state.dragging = true;
                state.start_mouse = Position::new(e.client_x() as f64, e.client_y() as f64);
                state.start_pos = current_pos;
            }

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Create move handler
            let drag_state_move = drag_state.clone();
            let on_move_clone = on_move.clone();
            let move_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
                let state = drag_state_move.borrow();
                if state.dragging {
                    let dx = e.client_x() as f64 - state.start_mouse.x;
                    let dy = e.client_y() as f64 - state.start_mouse.y;
                    let new_pos = state.start_pos.offset(dx, dy);
                    on_move_clone.emit((widget_id, new_pos));
                }
            }) as Box<dyn FnMut(_)>);

            // Create up handler
            let drag_state_up = drag_state.clone();
            let closures_up = closures.clone();
            let document_up = document.clone();
            let on_drop_clone = on_drop.clone();
            let up_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
                // Get current position before stopping drag
                let final_pos = {
                    let state = drag_state_up.borrow();
                    let dx = e.client_x() as f64 - state.start_mouse.x;
                    let dy = e.client_y() as f64 - state.start_mouse.y;
                    state.start_pos.offset(dx, dy)
                };
                let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);

                // Stop dragging
                {
                    let mut state = drag_state_up.borrow_mut();
                    state.dragging = false;
                }

                // Remove event listeners FIRST
                {
                    let closures_ref = closures_up.borrow();
                    if let Some((ref move_cl, ref up_cl)) = *closures_ref {
                        let _ = document_up.remove_event_listener_with_callback(
                            "mousemove",
                            move_cl.as_ref().unchecked_ref(),
                        );
                        let _ = document_up.remove_event_listener_with_callback(
                            "mouseup",
                            up_cl.as_ref().unchecked_ref(),
                        );
                    }
                }

                // Emit drop event AFTER removing listeners
                if let Some(ref on_drop_cb) = on_drop_clone {
                    on_drop_cb.emit(DropEvent {
                        widget_id,
                        position: final_pos,
                        mouse_position: mouse_pos,
                    });
                }

                // Clear closures LAST (after all other operations)
                *closures_up.borrow_mut() = None;
            }) as Box<dyn FnMut(_)>);

            // Add event listeners
            document
                .add_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .add_event_listener_with_callback("mouseup", up_closure.as_ref().unchecked_ref())
                .unwrap();

            // Store closures to keep them alive
            *closures.borrow_mut() = Some((move_closure, up_closure));
        })
    };

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
