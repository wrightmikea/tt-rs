//! Widget-related callback handlers.

use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{CopySourceClickEvent, DropEvent, Position};
use yew::prelude::*;

use crate::ops::*;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

pub fn create_copy_source(state: UseStateHandle<AppState>) -> Callback<CopySourceClickEvent> {
    Callback::from(move |e: CopySourceClickEvent| {
        let mut s = (*state).clone();
        if let Some(WidgetItem::Number(n)) = s.widgets.get(&e.source_id) {
            let copy = n.copy_number();
            s.positions.insert(copy.id(), e.position);
            s.widgets.insert(copy.id(), WidgetItem::Number(copy));
            state.set(s);
        }
    })
}

pub fn create_move(state: UseStateHandle<AppState>) -> Callback<(WidgetId, Position)> {
    Callback::from(move |(id, pos): (WidgetId, Position)| {
        let mut s = (*state).clone();
        s.positions.insert(id, pos);
        state.set(s);
    })
}

pub fn create_widget_drop(state: UseStateHandle<AppState>) -> Callback<DropEvent> {
    Callback::from(move |e: DropEvent| {
        let mut s = (*state).clone();
        let id = e.widget_id;
        let (mx, my) = (e.mouse_position.x, e.mouse_position.y);

        if handle_robot_click(&mut s, id, &e)
            || handle_vacuum_drop(&mut s, id, mx, my, &e)
            || handle_wand_drop(&mut s, id, mx, my, &e)
            || handle_scales_drop(&mut s, id, mx, my)
            || handle_number_on_number(&mut s, id, mx, my)
            || handle_box_hole_drop(&mut s, id, mx, my, &e)
        {
            state.set(s);
        } else {
            s.positions.insert(id, e.position);
            state.set(s);
        }
    })
}
