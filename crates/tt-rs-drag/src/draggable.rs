//! Draggable component wrapper.

use crate::Position;
use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlElement, MouseEvent};
use yew::prelude::*;

/// Helper to add/remove class on body element for global drag state.
fn set_body_dragging(active: bool) {
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(body) = document.body()
    {
        let body_el: HtmlElement = body;
        if active {
            let _ = body_el.class_list().add_1("dragging-active");
        } else {
            let _ = body_el.class_list().remove_1("dragging-active");
        }
    }
}

/// Information about a drag start event.
#[derive(Debug, Clone, Copy)]
pub struct DragStartEvent {
    /// The ID of the widget that started dragging.
    pub widget_id: WidgetId,
    /// The starting position of the widget.
    pub position: Position,
}

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
    /// Optional callback for when drag starts.
    #[prop_or_default]
    pub on_drag_start: Option<Callback<DragStartEvent>>,
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

    // Yew state to track if we're currently dragging (for rendering)
    let is_dragging = use_state(|| false);

    // Track current position for rendering
    let current_pos = props.position;

    // Build style - include pointer-events: none when dragging
    let style = if *is_dragging {
        format!(
            "position: absolute; left: {}px; top: {}px; pointer-events: none; opacity: 0.8; z-index: 100;",
            current_pos.x, current_pos.y
        )
    } else {
        format!(
            "position: absolute; left: {}px; top: {}px;",
            current_pos.x, current_pos.y
        )
    };

    // Store closures using use_mut_ref instead of use_state
    // This prevents Yew from dropping them during re-renders
    let closures: Rc<RefCell<Option<DragClosures>>> = use_mut_ref(|| None);

    let on_mouse_down = {
        let drag_state = drag_state.clone();
        let closures = closures.clone();
        let on_move = props.on_move.clone();
        let on_drag_start = props.on_drag_start.clone();
        let on_drop = props.on_drop.clone();
        let widget_id = props.widget_id;
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // Emit drag start event
            if let Some(ref on_start) = on_drag_start {
                on_start.emit(DragStartEvent {
                    widget_id,
                    position: current_pos,
                });
            }

            // Set dragging state for CSS
            is_dragging.set(true);
            set_body_dragging(true);

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
            let is_dragging_up = is_dragging.clone();
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

                // Clear dragging state for CSS
                is_dragging_up.set(false);
                set_body_dragging(false);

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

    let class = if *is_dragging {
        "draggable dragging"
    } else {
        "draggable"
    };

    html! {
        <div
            class={class}
            style={style}
            onmousedown={on_mouse_down}
        >
            { for props.children.iter() }
        </div>
    }
}
