//! tt-rs-number: Number widget with rational arithmetic.
//!
//! # Module Organization
//!
//! - [`Number`] - struct and constructors
//! - `ops` - accessor methods
//! - `builders` - builder methods and operations
//! - `rendering` - HTML rendering

mod builders;
mod number;
mod operator;
mod ops;
mod rendering;
mod widget_impl;

pub use number::{ErasureLevel, Number};
pub use operator::ArithOperator;
