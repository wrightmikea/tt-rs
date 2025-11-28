//! Robot struct and operations.

use tt_rs_core::WidgetId;

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

/// A robot widget that can be trained to automate tasks.
///
/// Robots watch user demonstrations and learn patterns.
/// When given matching input, they replay the recorded actions.
#[derive(Debug, Clone)]
pub struct Robot {
    pub(crate) id: WidgetId,
    /// Current state of the robot.
    pub(crate) state: RobotState,
    /// The pattern this robot expects (optional - starts empty).
    pub(crate) pattern: Option<WidgetId>,
    /// Recorded actions to perform.
    pub(crate) actions: Vec<Action>,
    /// Next robot in chain (optional).
    pub(crate) next_robot: Option<WidgetId>,
}

impl Robot {
    /// Creates a new idle robot.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            state: RobotState::Idle,
            pattern: None,
            actions: Vec::new(),
            next_robot: None,
        }
    }

    /// Returns the robot's state.
    pub fn state(&self) -> RobotState {
        self.state
    }

    /// Starts training mode - robot begins recording actions.
    pub fn start_training(&mut self) {
        self.state = RobotState::Training;
        self.actions.clear();
    }

    /// Stops training mode - robot saves recorded actions.
    pub fn stop_training(&mut self) {
        self.state = RobotState::Idle;
    }

    /// Records an action during training.
    pub fn record_action(&mut self, action: Action) {
        if self.state == RobotState::Training {
            self.actions.push(action);
        }
    }

    /// Returns the recorded actions.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Sets the pattern this robot expects.
    pub fn set_pattern(&mut self, pattern_id: WidgetId) {
        self.pattern = Some(pattern_id);
    }

    /// Returns the pattern this robot expects.
    pub fn pattern(&self) -> Option<WidgetId> {
        self.pattern
    }

    /// Starts working mode - robot executes actions.
    pub fn start_working(&mut self) {
        if !self.actions.is_empty() {
            self.state = RobotState::Working;
        }
    }

    /// Stops working mode.
    pub fn stop_working(&mut self) {
        self.state = RobotState::Idle;
    }

    /// Creates a copy of this robot with a new ID.
    pub fn copy_robot(&self) -> Robot {
        Robot {
            id: WidgetId::new(),
            state: RobotState::Idle, // Copies start idle
            pattern: self.pattern,
            actions: self.actions.clone(),
            next_robot: self.next_robot,
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}
