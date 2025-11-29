//! Position type for widget locations.

/// A 2D position in the workspace.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, dx: f64, dy: f64) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}
