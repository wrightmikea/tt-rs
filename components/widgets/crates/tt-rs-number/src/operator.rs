//! Arithmetic operators for Number widgets.

/// The arithmetic operator displayed on a number widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArithOperator {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl ArithOperator {
    /// Returns the symbol for this operator.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
        }
    }
}
