//! tt-rs-robot: Robot widget for trainable automation.
//!
//! Robots are the core programming construct in ToonTalk. They:
//! - Watch and learn from user demonstrations
//! - Execute recorded action sequences
//! - Match patterns in their "thought bubble" (input box)
//! - Can chain to other robots for complex behavior

mod robot;
mod widget_impl;

pub use robot::{Action, Robot, RobotState};
