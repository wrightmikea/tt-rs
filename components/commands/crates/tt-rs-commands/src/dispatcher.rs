//! Command dispatcher for executing commands.

use crate::command::{Command, CommandResult};
use tt_rs_state::PositionStore;

/// Dispatcher for executing commands (Invoker in Command pattern).
pub struct CommandDispatcher;

impl CommandDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, cmd: &dyn Command, pos: &mut dyn PositionStore) -> CommandResult {
        log::info!("Executing: {}", cmd.description());
        cmd.execute(pos)
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
