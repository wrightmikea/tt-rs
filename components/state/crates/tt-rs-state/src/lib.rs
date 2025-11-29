//! State management traits for tt-rs.
//!
//! Substates are composable, each handling a single concern:
//! - PositionStore: Widget positions
//! - BoxContents: Box hole contents
//! - TrainingState: Robot training mode

mod box_contents;
mod position;
mod position_store;
mod training_state;

pub use box_contents::BoxContents;
pub use position::Position;
pub use position_store::PositionStore;
pub use training_state::TrainingState;
