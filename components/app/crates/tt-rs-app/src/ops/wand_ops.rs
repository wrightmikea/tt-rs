//! Wand tool operations: copy widgets.

use tt_rs_core::WidgetId;
use tt_rs_drag::{DropEvent, Position};
use tt_rs_hit_test::find_widget_at_excluding;
use tt_rs_robot::Action;

use super::robot_ops::copy_widget;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle wand drop: copy target widget or box.
pub fn handle_wand_drop(
    state: &mut AppState,
    id: WidgetId,
    mx: f64,
    my: f64,
    event: &DropEvent,
) -> bool {
    if !state.widgets.get(&id).map(|w| w.is_wand()).unwrap_or(false) {
        return false;
    }

    if let Some((target_id, is_box)) = find_widget_at_excluding(mx, my, id) {
        if is_box {
            copy_box(state, target_id);
        } else {
            copy_target(state, target_id);
        }
    }

    state.positions.insert(id, event.position);
    true
}

fn copy_box(state: &mut AppState, target_id: WidgetId) {
    if let Some(b) = state.boxes.get(&target_id) {
        let copied = b.copy_box();
        let pos = state.positions.get(&target_id).copied().unwrap_or_default();
        state
            .positions
            .insert(copied.id(), Position::new(pos.x + 30.0, pos.y + 30.0));
        state.boxes.insert(copied.id(), copied);
    }
}

fn copy_target(state: &mut AppState, target_id: WidgetId) {
    let widget = match state.widgets.get(&target_id) {
        Some(w) if !matches!(w, WidgetItem::Number(n) if n.is_copy_source()) => w,
        _ => return,
    };

    let copied = copy_widget(widget);
    state.record_action(Action::Copy {
        path: format!("widget:{}", target_id),
    });
    let pos = state.positions.get(&target_id).copied().unwrap_or_default();
    state
        .positions
        .insert(copied.id(), Position::new(pos.x + 30.0, pos.y + 30.0));
    state.widgets.insert(copied.id(), copied);
}
