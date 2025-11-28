//! Widget trait implementation for Number.

use crate::number::{ErasureLevel, Number};
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Number {
    fn type_name(&self) -> &'static str {
        "number"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(Number {
            id: WidgetId::new(),
            numerator: self.numerator,
            denominator: self.denominator,
            operator: self.operator,
            erasure: self.erasure,
            // Copies are NOT copy sources - only the original stack produces copies
            is_copy_source: false,
        })
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() != "number" {
            return MatchResult::NoMatch;
        }
        let matches =
            self.erasure == ErasureLevel::Value || other.description() == self.description();
        if matches {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        let value_str = format_value(self);
        let widget_id = self.id.to_string();
        let is_tool = self.is_tool();

        // Build CSS class
        let class = if self.is_copy_source {
            if is_tool {
                "widget number tool copy-source"
            } else {
                "widget number copy-source"
            }
        } else if is_tool {
            "widget number tool"
        } else {
            "widget number"
        };

        if is_tool {
            // Tools show operator and value (e.g., "+1", "×2")
            let op = self.operator.symbol();
            html! {
                <div class={class} data-widget-id={widget_id} data-copy-source={self.is_copy_source.to_string()}>
                    <div class="tool-content">
                        <span class="tool-operator">{op}</span>
                        <span class="tool-value">{value_str}</span>
                    </div>
                </div>
            }
        } else {
            // Plain numbers show just the value
            html! {
                <div class={class} data-widget-id={widget_id} data-copy-source={self.is_copy_source.to_string()}>
                    <div class="number-value">{value_str}</div>
                </div>
            }
        }
    }

    fn description(&self) -> String {
        if self.erasure == ErasureLevel::Value {
            "erased number".to_string()
        } else if self.is_integer() {
            format!("number {}", self.numerator)
        } else {
            format!("number {}/{}", self.numerator, self.denominator)
        }
    }
}

fn format_value(n: &Number) -> String {
    if n.erasure == ErasureLevel::Value {
        "?".to_string()
    } else if n.is_integer() {
        n.numerator.to_string()
    } else {
        format!("{}/{}", n.numerator, n.denominator)
    }
}
