//! Bird and Nest operations: message passing.

use tt_rs_bird::Bird;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::Position;
use tt_rs_hit_test::find_widget_at_excluding;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle bird drop: pair with nest or deliver message.
pub fn handle_bird_drop(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    let bird = match state.widgets.get(&id) {
        Some(WidgetItem::Bird(b)) if !b.is_copy_source() => b.clone(),
        _ => return false,
    };

    if let Some((target_id, _is_box)) = find_widget_at_excluding(mx, my, id) {
        if handle_bird_on_nest(state, id, &bird, target_id) {
            return true;
        }
        if handle_bird_on_number(state, id, &bird, target_id) {
            return true;
        }
    }
    false
}

/// Handle nest drop: receive message from paired bird.
pub fn handle_nest_drop(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    let is_nest = matches!(
        state.widgets.get(&id),
        Some(WidgetItem::Nest(n)) if !n.is_copy_source()
    );
    if !is_nest {
        return false;
    }

    // Dropping nest on bird pairs them
    if let Some((target_id, _is_box)) = find_widget_at_excluding(mx, my, id) {
        if let Some(WidgetItem::Bird(bird)) = state.widgets.get(&target_id) {
            if bird.nest_id().is_none() && !bird.is_copy_source() {
                pair_bird_and_nest(state, target_id, id);
                return true;
            }
        }
    }
    false
}

/// Pair a bird with a nest when dropped on each other.
fn handle_bird_on_nest(
    state: &mut AppState,
    bird_id: WidgetId,
    bird: &Bird,
    target_id: WidgetId,
) -> bool {
    if bird.nest_id().is_some() {
        return false; // Already paired
    }

    let is_target_nest = matches!(
        state.widgets.get(&target_id),
        Some(WidgetItem::Nest(n)) if !n.is_copy_source()
    );
    if !is_target_nest {
        return false;
    }

    pair_bird_and_nest(state, bird_id, target_id);
    true
}

/// When bird is dropped on a number, deliver it to the paired nest.
fn handle_bird_on_number(
    state: &mut AppState,
    bird_id: WidgetId,
    bird: &Bird,
    target_id: WidgetId,
) -> bool {
    // Bird must be paired to deliver
    let nest_id = match bird.nest_id() {
        Some(id) => id,
        None => return false,
    };

    // Target must be a number
    let number = match state.widgets.get(&target_id) {
        Some(WidgetItem::Number(n)) if !n.is_copy_source() => n.clone(),
        _ => return false,
    };

    // Copy the number to the nest's location
    let nest_pos = state.positions.get(&nest_id).copied().unwrap_or_default();
    let delivered = number.copy_number();
    let delivered_id = delivered.id();

    // Position the delivered number near the nest
    state.positions.insert(
        delivered_id,
        Position::new(nest_pos.x + 40.0, nest_pos.y + 10.0),
    );
    state
        .widgets
        .insert(delivered_id, WidgetItem::Number(delivered));

    // Update bird to flying state
    if let Some(WidgetItem::Bird(b)) = state.widgets.get_mut(&bird_id) {
        b.start_flying();
    }

    log::info!("Bird {} delivered message to nest {}", bird_id, nest_id);
    true
}

/// Create the pairing between bird and nest.
fn pair_bird_and_nest(state: &mut AppState, bird_id: WidgetId, nest_id: WidgetId) {
    // Update bird to know its nest
    if let Some(WidgetItem::Bird(b)) = state.widgets.get_mut(&bird_id) {
        b.pair_with_nest(nest_id);
    }

    // Position bird near nest to show pairing
    if let Some(nest_pos) = state.positions.get(&nest_id).copied() {
        state
            .positions
            .insert(bird_id, Position::new(nest_pos.x + 50.0, nest_pos.y));
    }

    log::info!("Bird {} paired with Nest {}", bird_id, nest_id);
}
