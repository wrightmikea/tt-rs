//! Robot types and enums.

/// The state a robot can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RobotState {
    /// Robot is idle, waiting for input that matches its pattern.
    #[default]
    Idle,
    /// Robot is being trained - recording user actions.
    Training,
    /// Robot is executing its recorded actions.
    Working,
}

/// An action that a robot can perform.
#[derive(Debug, Clone)]
pub enum Action {
    /// Pick up a widget from a location.
    PickUp {
        /// Path to the widget (e.g., "hole:0" for first hole).
        path: String,
    },
    /// Drop the held widget at a location.
    Drop {
        /// Path to the target location.
        path: String,
    },
    /// Copy a widget (using magic wand).
    Copy {
        /// Path to the widget to copy.
        path: String,
    },
    /// Remove/erase a widget (using vacuum).
    Remove {
        /// Path to the widget to remove.
        path: String,
    },
    /// Apply arithmetic operation (drop number on number).
    ApplyArithmetic {
        /// The operator to apply (+, -, *, /)
        operator: char,
        /// The operand value (numerator for rational)
        numerator: i64,
        /// The operand value (denominator for rational)
        denominator: i64,
        /// Path to the target number.
        target_path: String,
    },
}
