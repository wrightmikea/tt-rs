//! Drop zone operations for puzzle verification.

use tt_rs_core::WidgetId;
use tt_rs_hit_test::find_dropzone_at;

use crate::state::AppState;
use crate::widget_item::WidgetItem;
use crate::workspace::{BoxPatternData, WidgetData};

/// Handle dropping a widget on a drop zone.
/// Returns true if the drop was handled (regardless of match result).
pub fn handle_dropzone_drop(state: &mut AppState, dropped_id: WidgetId, mx: f64, my: f64) -> bool {
    // Find if there's a dropzone at the drop location
    let dropzone_id = match find_dropzone_at(mx, my) {
        Some(id) => id,
        None => return false,
    };

    // Get the expected pattern for this dropzone
    let expected = match state.dropzone_patterns.get(&dropzone_id) {
        Some(pattern) => pattern.clone(),
        None => {
            log::warn!("DropZone {} has no expected pattern", dropzone_id);
            return false;
        }
    };

    // Check if the dropped widget matches the expected pattern
    let matches = check_widget_matches(state, dropped_id, &expected);

    if matches {
        log::info!(
            "Puzzle solved! Widget {} matches expected pattern",
            dropped_id
        );

        // Mark the dropzone as satisfied
        if let Some(WidgetItem::DropZone(dz)) = state.widgets.get_mut(&dropzone_id) {
            dz.set_satisfied(true);

            // Get success message before the borrow ends
            if let Some(msg) = dz.on_success_message() {
                log::info!("Success: {}", msg);
            }
        }

        // Remove the dropped widget from the workspace (it was consumed)
        state.widgets.remove(&dropped_id);
        state.positions.remove(&dropped_id);

        // If it was a box, remove it too
        if state.boxes.contains_key(&dropped_id) {
            // Remove contents from widget_in_box tracking
            if let Some(box_state) = state.boxes.get(&dropped_id) {
                for widget_id in box_state.contents.values() {
                    state.widgets.remove(widget_id);
                    state.widget_in_box.remove(widget_id);
                }
            }
            state.boxes.remove(&dropped_id);
        }
    } else {
        log::info!(
            "Wrong answer: Widget {} does not match expected pattern",
            dropped_id
        );

        // Mark the dropzone as showing error
        if let Some(WidgetItem::DropZone(dz)) = state.widgets.get_mut(&dropzone_id) {
            dz.set_show_error(true);
        }

        // Widget bounces back (position stays at original location)
    }

    true
}

/// Check if a widget matches an expected pattern.
fn check_widget_matches(state: &AppState, widget_id: WidgetId, expected: &WidgetData) -> bool {
    match expected {
        WidgetData::Number(expected_num) => {
            // Check if widget is a number with matching value
            if let Some(WidgetItem::Number(n)) = state.widgets.get(&widget_id) {
                n.numerator() == expected_num.numerator
                    && n.denominator() == expected_num.denominator
            } else {
                false
            }
        }
        WidgetData::Box(expected_box) => {
            // Check if it's a box with matching contents
            check_box_matches(state, widget_id, expected_box)
        }
        // Other widget types can be added as needed
        _ => false,
    }
}

/// Check if a box matches an expected box pattern.
fn check_box_matches(state: &AppState, box_id: WidgetId, expected: &BoxPatternData) -> bool {
    let box_state = match state.boxes.get(&box_id) {
        Some(b) => b,
        None => return false,
    };

    // Check number of holes matches
    if box_state.num_holes != expected.num_holes {
        return false;
    }

    // Check each expected content
    for hole_content in &expected.contents {
        let hole = hole_content.hole;

        // Get widget in this hole
        let widget_id = match box_state.widget_in_hole(hole) {
            Some(id) => id,
            None => return false, // Expected content but hole is empty
        };

        // Check if the widget matches the expected content
        if !check_widget_matches(state, widget_id, &hole_content.widget) {
            return false;
        }
    }

    true
}
