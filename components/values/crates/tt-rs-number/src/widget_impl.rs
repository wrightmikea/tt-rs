//! Widget trait implementation for Number.

use crate::number::{ErasureLevel, Number};
use crate::rendering;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::Html;

impl Widget for Number {
    fn type_name(&self) -> &'static str {
        "number"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_number())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "number" {
            return MatchResult::NoMatch;
        }
        if self.erasure == ErasureLevel::Value || other.description() == self.description() {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        rendering::render(self)
    }

    fn description(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "erased number".to_string()
        } else {
            // Include operator prefix to preserve full widget state
            let op_prefix = self.operator.symbol();
            if self.is_integer() {
                format!("number {}{}", op_prefix, self.numerator)
            } else {
                format!(
                    "number {}{}/{}",
                    op_prefix, self.numerator, self.denominator
                )
            }
        }
    }
}
