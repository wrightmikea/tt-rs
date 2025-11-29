//! Command pattern implementation for tt-rs operations.
//!
//! Commands encapsulate operations as objects, enabling:
//! - Undo/redo functionality
//! - Robot action recording
//! - Loose coupling between UI and business logic

mod command;
mod dispatcher;
mod move_cmd;
mod remove_cmd;

pub use command::Command;
pub use dispatcher::CommandDispatcher;
pub use move_cmd::MoveCommand;
pub use remove_cmd::RemoveCommand;
