//! Domain layer - core ToonTalk widget abstractions.
//!
//! This module defines the fundamental traits and types for ToonTalk widgets.
//! Widgets are the visual programming objects that users manipulate.

mod number;

pub use number::{ArithOperator, Number};

use yew::Html;

/// Unique identifier for widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    /// Creates a new unique widget ID.
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of pattern matching between widgets.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// Pattern matches the widget.
    Match,
    /// Pattern does not match the widget.
    NoMatch,
    /// Pattern matches with bindings (for erased/wildcard patterns).
    MatchWithBindings(Vec<(String, Box<dyn Widget>)>),
}

/// The fundamental trait for all ToonTalk widgets.
///
/// Widgets are the visual programming objects in ToonTalk. They can be:
/// - Numbers (for arithmetic)
/// - Boxes (containers)
/// - Robots (programs)
/// - Birds/Nests (message passing)
/// - Scales (comparison)
/// - And more...
pub trait Widget: std::fmt::Debug {
    /// Returns the type name of this widget (e.g., "number", "box", "robot").
    fn type_name(&self) -> &'static str;

    /// Returns the unique identifier for this widget.
    fn id(&self) -> WidgetId;

    /// Creates a deep copy of this widget.
    fn copy(&self) -> Box<dyn Widget>;

    /// Checks if this widget matches another widget (for robot pattern matching).
    ///
    /// In ToonTalk, robots use pattern matching to decide when to run.
    /// A widget can be "erased" to various levels to create patterns.
    fn matches(&self, other: &dyn Widget) -> MatchResult;

    /// Renders this widget as HTML/SVG for display.
    fn render(&self) -> Html;

    /// Returns a text description of this widget (for accessibility/debugging).
    fn description(&self) -> String;
}

impl Clone for Box<dyn Widget> {
    fn clone(&self) -> Self {
        self.copy()
    }
}

impl PartialEq for Box<dyn Widget> {
    fn eq(&self, other: &Self) -> bool {
        // Two widgets are equal if they match exactly
        matches!(self.matches(other.as_ref()), MatchResult::Match)
    }
}
