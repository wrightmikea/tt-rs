//! Rendering functions for Number.

use crate::Number;
use yew::prelude::*;

/// Renders a Number as HTML.
pub fn render(n: &Number) -> Html {
    let value_str = n.display_value();
    let widget_id = n.id.to_string();
    let is_tool = n.is_tool();
    let is_copy_source = n.is_copy_source();

    let class = css_class(is_tool, is_copy_source);

    // Show operator for tools OR for copy sources (so +1 and +5 show the +)
    if is_tool || is_copy_source {
        render_tool(n, class, &widget_id, &value_str, is_copy_source)
    } else {
        render_plain(class, &widget_id, &value_str, is_copy_source)
    }
}

fn css_class(is_tool: bool, is_copy_source: bool) -> &'static str {
    match (is_copy_source, is_tool) {
        (true, true) => "widget number tool copy-source",
        (true, false) => "widget number copy-source",
        (false, true) => "widget number tool",
        (false, false) => "widget number",
    }
}

fn render_tool(n: &Number, class: &'static str, id: &str, val: &str, is_copy: bool) -> Html {
    let op = n.operator().symbol();
    html! {
        <div class={class} data-widget-id={id.to_string()} data-copy-source={is_copy.to_string()}>
            <div class="tool-content">
                <span class="tool-operator">{op}</span>
                <span class="tool-value">{val.to_string()}</span>
            </div>
        </div>
    }
}

fn render_plain(class: &'static str, id: &str, val: &str, is_copy: bool) -> Html {
    html! {
        <div class={class} data-widget-id={id.to_string()} data-copy-source={is_copy.to_string()}>
            <div class="number-value">{val.to_string()}</div>
        </div>
    }
}
