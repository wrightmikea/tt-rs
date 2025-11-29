//! Box drop operations: create, split, join.

use tt_rs_core::WidgetId;
use tt_rs_drag::{DropEvent, Position};
use tt_rs_hit_test::{find_number_at, find_widget_at_excluding};

use crate::box_state::BoxState;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle box drop: create new box, split, or join.
pub fn handle_box_drop(state: &mut AppState, event: &DropEvent, pending: Option<usize>) -> bool {
    let box_id = event.widget_id;
    let (mx, my) = (event.mouse_position.x, event.mouse_position.y);

    if let Some(num_holes) = pending {
        create_new_box(state, num_holes, event);
        state.positions.insert(box_id, event.position);
        return true;
    }

    if let Some(target_id) = find_number_at(mx, my) {
        if try_split_box(state, box_id, target_id, event) {
            return true;
        }
    }

    if let Some((target_id, _)) = find_widget_at_excluding(mx, my, box_id) {
        if try_join_boxes(state, box_id, target_id, event) {
            return true;
        }
    }

    state.positions.insert(box_id, event.position);
    false
}

fn create_new_box(state: &mut AppState, num_holes: usize, event: &DropEvent) {
    let new_box = BoxState::new(num_holes);
    let id = new_box.id();
    state
        .positions
        .insert(id, event.position.offset(50.0, 50.0));
    state.boxes.insert(id, new_box);
    log::info!("Created box {} with {} holes", id, num_holes);
}

fn try_split_box(
    state: &mut AppState,
    box_id: WidgetId,
    num_id: WidgetId,
    event: &DropEvent,
) -> bool {
    let split_at = match state.widgets.get(&num_id) {
        Some(WidgetItem::Number(n)) if !n.is_copy_source() => n.numerator() as usize,
        _ => return false,
    };

    let box_state = match state.boxes.get(&box_id) {
        Some(b) if split_at >= 1 && split_at < b.num_holes => b.clone(),
        _ => return false,
    };

    let (left, right) = split_contents(&box_state, split_at, state);
    let pos = state
        .positions
        .get(&box_id)
        .copied()
        .unwrap_or(event.position);

    state.positions.insert(left.id(), pos);
    state
        .positions
        .insert(right.id(), Position::new(pos.x + 100.0, pos.y));
    state.boxes.insert(left.id(), left);
    state.boxes.insert(right.id(), right);

    state.boxes.remove(&box_id);
    state.positions.remove(&box_id);
    state.widgets.remove(&num_id);
    state.positions.remove(&num_id);
    true
}

fn split_contents(src: &BoxState, at: usize, state: &mut AppState) -> (BoxState, BoxState) {
    let mut left = BoxState::new(at);
    let mut right = BoxState::new(src.num_holes - at);

    for i in 0..at {
        if let Some(wid) = src.contents.get(&i).copied() {
            left.place_in_hole(i, wid);
            state.widget_in_box.insert(wid, (left.id(), i));
        }
    }
    for i in at..src.num_holes {
        if let Some(wid) = src.contents.get(&i).copied() {
            right.place_in_hole(i - at, wid);
            state.widget_in_box.insert(wid, (right.id(), i - at));
        }
    }
    (left, right)
}

fn try_join_boxes(
    state: &mut AppState,
    src_id: WidgetId,
    tgt_id: WidgetId,
    event: &DropEvent,
) -> bool {
    let src = match state.boxes.get(&src_id) {
        Some(b) => b.clone(),
        None => return false,
    };
    let tgt = match state.boxes.get(&tgt_id) {
        Some(b) => b.clone(),
        None => return false,
    };

    let joined = join_contents(&src, &tgt, state);
    let pos = state
        .positions
        .get(&tgt_id)
        .copied()
        .unwrap_or(event.position);

    state.positions.insert(joined.id(), pos);
    state.boxes.insert(joined.id(), joined);
    state.boxes.remove(&src_id);
    state.boxes.remove(&tgt_id);
    state.positions.remove(&src_id);
    state.positions.remove(&tgt_id);
    true
}

fn join_contents(src: &BoxState, tgt: &BoxState, state: &mut AppState) -> BoxState {
    let mut joined = BoxState::new(tgt.num_holes + src.num_holes);

    for i in 0..tgt.num_holes {
        if let Some(wid) = tgt.contents.get(&i).copied() {
            joined.place_in_hole(i, wid);
            state.widget_in_box.insert(wid, (joined.id(), i));
        }
    }
    for i in 0..src.num_holes {
        if let Some(wid) = src.contents.get(&i).copied() {
            let hole = tgt.num_holes + i;
            joined.place_in_hole(hole, wid);
            state.widget_in_box.insert(wid, (joined.id(), hole));
        }
    }
    joined
}
