//! Scales operations: place numbers on pans.

use tt_rs_core::WidgetId;
use tt_rs_hit_test::{find_scales_pan_at, ScalesPan};

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle dropping number on scales pan.
pub fn handle_scales_drop(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    let (scales_id, pan) = match find_scales_pan_at(mx, my) {
        Some(s) => s,
        None => return false,
    };

    let value = match state.widgets.get(&id) {
        Some(WidgetItem::Number(n)) if !n.is_copy_source() => n.numerator(),
        _ => return false,
    };

    if let Some(WidgetItem::Scales(scales)) = state.widgets.get_mut(&scales_id) {
        match pan {
            ScalesPan::Left => scales.set_left(value),
            ScalesPan::Right => scales.set_right(value),
        }
        state.widgets.remove(&id);
        state.positions.remove(&id);
        return true;
    }
    false
}
