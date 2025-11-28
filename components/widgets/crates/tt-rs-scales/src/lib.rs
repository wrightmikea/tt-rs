//! tt-rs-scales: Scales widget for comparing values.
//!
//! The scales widget compares two values and tips toward the heavier one.
//! Drop widgets on the left or right pan to compare them.

mod scales;
mod widget_impl;

pub use scales::{CompareResult, Scales};
