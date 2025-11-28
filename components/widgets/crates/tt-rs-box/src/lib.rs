//! tt-rs-box: Box widget with holes for containing other widgets.
//!
//! In ToonTalk, a Box is a container with numbered holes that can hold
//! other widgets. Boxes are fundamental for organizing and structuring
//! data in visual programs.
//!
//! # Module Organization
//!
//! - [`ToonBox`] - struct and constructors
//! - [`Hole`] - hole struct and operations
//! - `ops` - accessor and query methods
//! - `mutators` - mutation methods

mod hole;
mod mutators;
mod ops;
mod rendering;
#[cfg(test)]
mod tests;
mod toon_box;
mod widget_impl;

pub use hole::Hole;
pub use toon_box::ToonBox;
