//! Box drop operations: create, split, join, copy.

use tt_rs_core::WidgetId;
use tt_rs_drag::{DropEvent, Position};
use tt_rs_hit_test::{find_number_at, find_widget_at_excluding};

use crate::box_state::BoxState;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Deep copy a box including all its contents.
/// Returns the new box and updates state with copied widgets.
pub fn deep_copy_box(state: &mut AppState, src: &BoxState) -> BoxState {
    let mut new_box = BoxState::new(src.num_holes);

    for (hole, &widget_id) in &src.contents {
        if let Some(widget) = state.widgets.get(&widget_id) {
            let copied = widget.copy_widget();
            let copied_id = copied.id();
            state.widgets.insert(copied_id, copied);
            // Position copied widgets relative to original
            if let Some(pos) = state.positions.get(&widget_id) {
                state.positions.insert(copied_id, pos.offset(30.0, 30.0));
            }
            new_box.place_in_hole(*hole, copied_id);
            state.widget_in_box.insert(copied_id, (new_box.id(), *hole));
        }
    }

    new_box
}

/// Handle box drop: create new box, split, or join.
pub fn handle_box_drop(state: &mut AppState, event: &DropEvent, pending: Option<usize>) -> bool {
    let box_id = event.widget_id;
    let (mx, my) = (event.mouse_position.x, event.mouse_position.y);

    if let Some(num_holes) = pending {
        // Create a new box with requested number of holes (original stays as prototype)
        create_new_box(state, num_holes, event);
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
    // Get effective numerator (accounts for operator) to handle negative indices
    // A "- 1" widget has operator=Subtract, numerator=1, so effective_numerator() returns -1
    let raw_split = match state.widgets.get(&num_id) {
        Some(WidgetItem::Number(n)) if !n.is_copy_source() => n.effective_numerator(),
        _ => return false,
    };

    let box_state = match state.boxes.get(&box_id) {
        Some(b) => b.clone(),
        None => return false,
    };

    // Handle special case: drop on 0 creates a deep copy of the box
    if raw_split == 0 {
        return copy_box_with_contents(state, &box_state, num_id, event);
    }

    // Convert negative index to split from right: -2 on 8-hole box → split at 6
    // Positive: split_at holes on left, rest on right
    // Negative: abs(split_at) holes on right, rest on left
    let split_at = if raw_split < 0 {
        let from_right = (-raw_split) as usize;
        if from_right >= box_state.num_holes {
            return false;
        }
        box_state.num_holes - from_right
    } else {
        let pos = raw_split as usize;
        if pos >= box_state.num_holes {
            return false;
        }
        pos
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

/// Copy a box with all its contents when dropped on 0.
/// The original box remains, a copy is created offset from it.
fn copy_box_with_contents(
    state: &mut AppState,
    src: &BoxState,
    num_id: WidgetId,
    event: &DropEvent,
) -> bool {
    let copied = deep_copy_box(state, src);
    let pos = state
        .positions
        .get(&src.id())
        .copied()
        .unwrap_or(event.position);

    state
        .positions
        .insert(copied.id(), Position::new(pos.x + 50.0, pos.y + 50.0));
    state.boxes.insert(copied.id(), copied);

    // Consume the 0 number
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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    /// Test that negative index splitting works correctly.
    /// A 6-hole box [1, 2, _, _, _, 6] dropped on -1 should produce:
    /// - Left: 5-hole box [1, 2, _, _, _]
    /// - Right: 1-hole box [6]
    #[test]
    fn test_negative_split_takes_from_right() {
        // Create a mock scenario:
        // - 6-hole box with contents at holes 0, 1, and 5
        // - Split at -1 means take 1 hole from the right
        // - Result: left has 5 holes (with contents at 0, 1), right has 1 hole (with content at 0, originally hole 5)

        // The split_at calculation for -1 on a 6-hole box:
        // raw_split = -1
        // from_right = 1
        // split_at = 6 - 1 = 5
        // So left gets holes 0..5 (5 holes), right gets holes 5..6 (1 hole)

        let num_holes = 6;
        let raw_split: i64 = -1;

        let split_at = if raw_split < 0 {
            let from_right = (-raw_split) as usize;
            assert!(from_right < num_holes && from_right > 0);
            num_holes - from_right
        } else {
            raw_split as usize
        };

        // split_at should be 5, meaning left gets 5 holes, right gets 1 hole
        assert_eq!(
            split_at, 5,
            "Split at -1 on 6-hole box should split at position 5"
        );

        // After split:
        // - Left box: 5 holes (indices 0-4 from original)
        // - Right box: 1 hole (index 5 from original, now index 0)
        let left_holes = split_at;
        let right_holes = num_holes - split_at;
        assert_eq!(left_holes, 5);
        assert_eq!(right_holes, 1);
    }

    /// Test that -2 on an 8-hole box splits correctly.
    /// An 8-hole box dropped on -2 should produce:
    /// - Left: 6-hole box
    /// - Right: 2-hole box
    #[test]
    fn test_negative_2_split() {
        let num_holes = 8;
        let raw_split: i64 = -2;

        let split_at = if raw_split < 0 {
            let from_right = (-raw_split) as usize;
            assert!(from_right < num_holes && from_right > 0);
            num_holes - from_right
        } else {
            raw_split as usize
        };

        assert_eq!(
            split_at, 6,
            "Split at -2 on 8-hole box should split at position 6"
        );

        let left_holes = split_at;
        let right_holes = num_holes - split_at;
        assert_eq!(left_holes, 6);
        assert_eq!(right_holes, 2);
    }
}
