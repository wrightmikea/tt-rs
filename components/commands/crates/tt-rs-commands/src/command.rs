//! Command trait definition.

use tt_rs_state::PositionStore;

/// Result of command execution.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    Success,
    NotApplicable,
}

/// Trait for encapsulating operations as objects (Command pattern).
pub trait Command {
    fn execute(&self, positions: &mut dyn PositionStore) -> CommandResult;
    fn description(&self) -> String;
}
