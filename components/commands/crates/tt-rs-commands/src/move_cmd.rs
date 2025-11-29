//! Move command for repositioning widgets.

use tt_rs_core::WidgetId;
use tt_rs_state::{Position, PositionStore};

use crate::command::{Command, CommandResult};

/// Command to move a widget to a new position.
pub struct MoveCommand {
    widget_id: WidgetId,
    new_position: Position,
}

impl MoveCommand {
    pub fn new(widget_id: WidgetId, new_position: Position) -> Self {
        Self {
            widget_id,
            new_position,
        }
    }
}

impl Command for MoveCommand {
    fn execute(&self, positions: &mut dyn PositionStore) -> CommandResult {
        positions.set(self.widget_id, self.new_position);
        CommandResult::Success
    }

    fn description(&self) -> String {
        format!("Move {}", self.widget_id)
    }
}
