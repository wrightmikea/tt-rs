//! Bird and Nest operations: message passing.
//!
//! ToonTalk Bird/Nest semantics:
//! - Birds are created paired with a nest (via "hatching")
//! - Drop a widget ON a bird to give it a message
//! - The bird delivers the message to its paired nest
//! - Messages queue on the nest until retrieved by a robot

use tt_rs_core::WidgetId;
use tt_rs_drag::Position;
use tt_rs_hit_test::find_widget_at_excluding;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle when any widget is dropped - check if it landed on a bird.
/// This is called from the general drop handler to check for bird delivery.
pub fn handle_drop_on_bird(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    // Get the widget being dropped
    let dropped = match state.widgets.get(&id) {
        Some(w) if !w.is_copy_source() => w.clone(),
        _ => return false,
    };

    // Check if dropped on a bird
    if let Some((target_id, _is_box)) = find_widget_at_excluding(mx, my, id) {
        if let Some(WidgetItem::Bird(bird)) = state.widgets.get(&target_id) {
            // Can't deliver birds, nests, or tools
            if matches!(
                dropped,
                WidgetItem::Bird(_)
                    | WidgetItem::Nest(_)
                    | WidgetItem::Vacuum(_)
                    | WidgetItem::Wand(_)
            ) {
                return false;
            }

            // Bird must be paired with a nest
            let nest_id = match bird.nest_id() {
                Some(id) => id,
                None => {
                    log::info!("Bird {} has no nest to deliver to", target_id);
                    return false;
                }
            };

            // Remove the dropped widget (bird takes it)
            state.widgets.remove(&id);
            state.positions.remove(&id);

            // Copy the widget for delivery to the nest
            let message = dropped.copy_widget();
            let message_id = message.id();

            // Add message to the nest's queue
            if let Some(WidgetItem::Nest(nest)) = state.widgets.get_mut(&nest_id) {
                nest.receive(message.to_boxed_widget());
            }

            // Place the delivered widget near the nest (visual feedback)
            let nest_pos = state.positions.get(&nest_id).copied().unwrap_or_default();
            state.positions.insert(
                message_id,
                Position::new(nest_pos.x + 40.0, nest_pos.y + 10.0),
            );
            state.widgets.insert(message_id, message);

            // Update bird to flying state (animation would go here)
            if let Some(WidgetItem::Bird(b)) = state.widgets.get_mut(&target_id) {
                b.start_flying();
            }

            log::info!("Bird {} delivered message to nest {}", target_id, nest_id);
            return true;
        }
    }
    false
}

/// Handle bird drop - just positioning, birds don't do anything special when dropped.
/// (In ToonTalk, you drop things ON birds, not the bird on things)
pub fn handle_bird_drop(_state: &mut AppState, _id: WidgetId, _mx: f64, _my: f64) -> bool {
    // Birds don't have special drop behavior - they just get repositioned
    false
}

/// Handle nest drop - just positioning.
/// (Nests receive messages when birds deliver, not via dropping)
pub fn handle_nest_drop(_state: &mut AppState, _id: WidgetId, _mx: f64, _my: f64) -> bool {
    // Nests don't have special drop behavior - they just get repositioned
    false
}
