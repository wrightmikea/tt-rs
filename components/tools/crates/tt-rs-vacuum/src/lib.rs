//! tt-rs-vacuum: Vacuum tool for erasing values.
//!
//! The vacuum is a tool that erases values from widgets to create patterns.
//! Drop the vacuum on a number to turn it into an "erased" number that
//! matches any number value.

mod vacuum;
mod widget_impl;

pub use vacuum::Vacuum;
