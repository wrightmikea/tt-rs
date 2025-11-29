//! tt-rs-wand: Magic wand tool for copying widgets.
//!
//! The wand is a tool that creates copies of widgets. Drop the wand on any
//! widget to create a duplicate with a new ID.

mod wand;
mod widget_impl;

pub use wand::Wand;
