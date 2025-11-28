//! tt-rs-number: Number widget with rational arithmetic.

mod accessors;
mod arithmetic;
mod number;
mod operator;
mod widget_impl;

pub use number::{ErasureLevel, Number};
pub use operator::ArithOperator;
