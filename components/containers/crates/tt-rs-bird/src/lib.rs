//! tt-rs-bird: Bird widget for message delivery.
//!
//! Birds are the delivery end of ToonTalk's bird/nest messaging system.
//! Each bird has a home nest. When given a message (widget), the bird
//! flies to its nest and delivers the message.
//!
//! # Module Organization
//!
//! - [`Bird`] - struct and constructors
//! - `ops` - accessor methods
//! - `rendering` - HTML rendering

mod bird;
mod ops;
mod rendering;
mod widget_impl;

pub use bird::{Bird, BirdColor, BirdState};
