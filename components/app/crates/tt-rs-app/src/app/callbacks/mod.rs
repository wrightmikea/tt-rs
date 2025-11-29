//! Callback handler creation.

mod box_handlers;
mod keydown;
mod widget_handlers;

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySourceClickEvent, DragEndEvent, DragStartEvent, DropEvent, Position};
use yew::prelude::*;

use crate::state::AppState;

pub use keydown::setup_keydown_listener;

pub struct Callbacks {
    pub on_help_open: Callback<()>,
    pub on_help_close: Callback<()>,
    pub on_box_drag_start: Callback<DragStartEvent>,
    pub on_box_drag_end: Callback<DragEndEvent>,
    pub on_box_drop: Callback<DropEvent>,
    pub on_keydown: Callback<web_sys::KeyboardEvent>,
    pub on_copy_source_click: Callback<CopySourceClickEvent>,
    pub on_move: Callback<(WidgetId, Position)>,
    pub on_drop: Callback<DropEvent>,
}

pub fn create_callbacks(
    state: UseStateHandle<AppState>,
    help_open: UseStateHandle<bool>,
    dragged_box_id: Rc<RefCell<Option<WidgetId>>>,
    pending_new_box: Rc<RefCell<Option<usize>>>,
) -> Callbacks {
    Callbacks {
        on_help_open: {
            let h = help_open.clone();
            Callback::from(move |_| h.set(true))
        },
        on_help_close: {
            let h = help_open;
            Callback::from(move |_| h.set(false))
        },
        on_box_drag_start: box_handlers::create_box_drag_start(
            dragged_box_id.clone(),
            pending_new_box.clone(),
        ),
        on_box_drag_end: box_handlers::create_box_drag_end(
            dragged_box_id.clone(),
            pending_new_box.clone(),
        ),
        on_box_drop: box_handlers::create_box_drop(state.clone(), pending_new_box.clone()),
        on_keydown: keydown::create_keydown(dragged_box_id, pending_new_box),
        on_copy_source_click: widget_handlers::create_copy_source(state.clone()),
        on_move: widget_handlers::create_move(state.clone()),
        on_drop: widget_handlers::create_widget_drop(state),
    }
}
