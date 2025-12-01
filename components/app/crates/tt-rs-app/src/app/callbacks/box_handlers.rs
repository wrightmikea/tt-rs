//! Box-related callback handlers.

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use tt_rs_drag::{DragEndEvent, DragStartEvent, DropEvent};
use yew::prelude::*;

use crate::ops::handle_box_drop;
use crate::state::AppState;

pub fn create_box_drag_start(
    d: Rc<RefCell<Option<WidgetId>>>,
    p: Rc<RefCell<Option<usize>>>,
) -> Callback<DragStartEvent> {
    Callback::from(move |e: DragStartEvent| {
        *d.borrow_mut() = Some(e.widget_id);
        *p.borrow_mut() = None;
    })
}

pub fn create_box_drag_end(
    d: Rc<RefCell<Option<WidgetId>>>,
    p: Rc<RefCell<Option<usize>>>,
) -> Callback<DragEndEvent> {
    Callback::from(move |_| {
        *p.borrow_mut() = None;
        *d.borrow_mut() = None;
    })
}

pub fn create_box_drop(
    state: UseStateHandle<AppState>,
    p: Rc<RefCell<Option<usize>>>,
    dirty: UseStateHandle<bool>,
) -> Callback<DropEvent> {
    Callback::from(move |e: DropEvent| {
        let mut s = (*state).clone();
        handle_box_drop(&mut s, &e, p.borrow_mut().take());
        state.set(s);
        // Box operations modify content
        dirty.set(true);
    })
}
