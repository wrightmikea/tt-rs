//! Robot execution coordinator.

use tt_rs_core::WidgetId;
use tt_rs_robot::Action;

use super::actions;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Execute all actions recorded by a robot.
pub fn execute_robot(state: &mut AppState, robot_id: WidgetId) {
    let actions = get_actions(state, robot_id);
    if actions.is_empty() {
        return;
    }

    set_working(state, robot_id, true);
    for action in &actions {
        execute_action(state, action);
    }
    set_working(state, robot_id, false);
}

fn get_actions(state: &AppState, id: WidgetId) -> Vec<Action> {
    state
        .widgets
        .get(&id)
        .and_then(|w| match w {
            WidgetItem::Robot(r) => Some(r.actions().to_vec()),
            _ => None,
        })
        .unwrap_or_default()
}

fn set_working(state: &mut AppState, id: WidgetId, working: bool) {
    if let Some(WidgetItem::Robot(r)) = state.widgets.get_mut(&id) {
        if working {
            r.start_working();
        } else {
            r.stop_working();
        }
    }
}

fn execute_action(state: &mut AppState, action: &Action) {
    match action {
        Action::ApplyArithmetic {
            operator,
            numerator,
            denominator,
            target_path,
        } => actions::execute_arithmetic(state, *operator, *numerator, *denominator, target_path),
        Action::Drop { path } => actions::execute_drop(path),
        Action::Copy { path } => actions::execute_copy(state, path),
        Action::Remove { path } => actions::execute_remove(state, path),
        Action::PickUp { path } => log::info!("PickUp at {} (not implemented)", path),
    }
}
