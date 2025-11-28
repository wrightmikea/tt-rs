//! tt-rs-core: Core traits and types for ToonTalk widgets.

mod widget_id;
mod widget_trait;

pub use widget_id::WidgetId;
pub use widget_trait::{MatchResult, Widget};
