//! Position type for widget placement.

/// A 2D position on the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    /// Creates a new position.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Adds an offset to this position.
    pub fn offset(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
