//! tt-rs-nest: Nest widget for message receiving.
//!
//! Nests are the receiving end of ToonTalk's bird/nest messaging system.
//! Birds deliver messages to their paired nest, where they queue until retrieved.
//!
//! # Module Organization
//!
//! - [`Nest`] - struct and constructors
//! - `ops` - accessor methods
//! - `mutators` - message receive/take operations
//! - `rendering` - HTML rendering

mod mutators;
mod nest;
mod ops;
mod rendering;
mod widget_impl;

pub use nest::{Nest, NestColor};
