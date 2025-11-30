//! Rendering functions for Nest.

use crate::Nest;
use yew::prelude::*;

/// Renders a Nest as HTML using the tt-nest.svg asset.
/// If the nest has messages, the top message is rendered overlaid on the nest.
pub fn render(nest: &Nest) -> Html {
    let widget_id = nest.id.to_string();
    let is_copy_source = nest.is_copy_source();
    let msg_count = nest.message_count();

    let class = if is_copy_source {
        "widget nest copy-source"
    } else {
        "widget nest"
    };

    // Render top message if present
    let top_message_html = if let Some(top_msg) = nest.peek_top() {
        html! {
            <div class="nest-contents">
                { top_msg.render() }
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class={class}
             data-widget-id={widget_id}
             data-copy-source={is_copy_source.to_string()}>
            <img src="images/tt-nest.svg" alt="Nest" class="nest-img"/>
            { top_message_html }
            if msg_count > 1 {
                <div class="nest-badge">{format!("+{}", msg_count - 1)}</div>
            }
        </div>
    }
}
