//! Robot execution engine.

mod actions;
mod executor;
mod path_parse;

pub use executor::execute_robot;
