//! Rendering functions for Nest.

use crate::Nest;
use yew::prelude::*;

/// Renders a Nest as HTML (egg-shaped visual).
pub fn render(nest: &Nest) -> Html {
    let widget_id = nest.id.to_string();
    let is_copy_source = nest.is_copy_source();
    let color = nest.color().css_color();
    let msg_count = nest.message_count();

    let class = if is_copy_source {
        "widget nest copy-source"
    } else {
        "widget nest"
    };

    html! {
        <div class={class}
             data-widget-id={widget_id}
             data-copy-source={is_copy_source.to_string()}>
            <svg viewBox="0 0 60 70" class="nest-svg">
                // Nest base (twigs)
                <ellipse cx="30" cy="55" rx="25" ry="12"
                         fill="#8B4513" stroke="#5D2E0C" stroke-width="2"/>
                // Egg shape
                <ellipse cx="30" cy="35" rx="18" ry="22"
                         fill={color} stroke="#333" stroke-width="1.5"/>
                // Highlight on egg
                <ellipse cx="24" cy="28" rx="5" ry="8"
                         fill="white" opacity="0.3"/>
            </svg>
            if msg_count > 0 {
                <div class="nest-badge">{msg_count}</div>
            }
        </div>
    }
}
