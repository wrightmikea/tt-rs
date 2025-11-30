//! tt-rs-dropzone: DropZone widget for puzzle verification.
//!
//! DropZones are labeled targets where users drop widgets to verify answers.
//! Used primarily in puzzles to check if the user has created the correct result.
//!
//! # Module Organization
//!
//! - [`DropZone`] - struct and constructors
//! - `ops` - accessor methods
//! - `rendering` - HTML rendering

mod dropzone;
mod ops;
mod rendering;
mod widget_impl;

pub use dropzone::DropZone;
