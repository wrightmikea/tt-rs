//! Widget trait implementation for Robot.

use crate::robot::{Robot, RobotState};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Robot {
    fn type_name(&self) -> &'static str {
        "robot"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_robot())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() == "robot" {
            // Robots match other robots in the same state
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        let state_class = match self.state {
            RobotState::Idle => "idle",
            RobotState::Training => "training",
            RobotState::Working => "working",
        };

        let state_indicator = match self.state {
            RobotState::Idle => "Idle",
            RobotState::Training => "Training...",
            RobotState::Working => "Working!",
        };

        let action_count = self.actions.len();

        html! {
            <div class={format!("widget robot {}", state_class)} data-widget-id={self.id.to_string()}>
                <img src="images/tt-robot.svg" alt="robot" class="robot-image" />
                <div class="robot-info">
                    <span class="robot-status">{ state_indicator }</span>
                    if action_count > 0 {
                        <span class="robot-actions">{ format!("({} actions)", action_count) }</span>
                    }
                </div>
            </div>
        }
    }

    fn description(&self) -> String {
        let state = match self.state {
            RobotState::Idle => "idle",
            RobotState::Training => "training",
            RobotState::Working => "working",
        };
        format!("Robot ({}, {} actions)", state, self.actions.len())
    }
}
