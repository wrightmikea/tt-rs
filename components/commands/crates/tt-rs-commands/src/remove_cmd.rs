//! Remove command for deleting widget positions.

use tt_rs_core::WidgetId;
use tt_rs_state::PositionStore;

use crate::command::{Command, CommandResult};

/// Command to remove a widget's position.
pub struct RemoveCommand {
    widget_id: WidgetId,
}

impl RemoveCommand {
    pub fn new(widget_id: WidgetId) -> Self {
        Self { widget_id }
    }
}

impl Command for RemoveCommand {
    fn execute(&self, positions: &mut dyn PositionStore) -> CommandResult {
        if positions.contains(self.widget_id) {
            positions.remove(self.widget_id);
            CommandResult::Success
        } else {
            CommandResult::NotApplicable
        }
    }

    fn description(&self) -> String {
        format!("Remove {}", self.widget_id)
    }
}
