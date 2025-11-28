//! Widget identifier type.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

/// Unique identifier for widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    /// Creates a new unique widget ID.
    pub fn new() -> Self {
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Creates a WidgetId from a u64 value.
    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Returns the inner u64 value.
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WidgetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
