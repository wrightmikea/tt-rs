//! Rendering functions for Nest.

use crate::Nest;
use yew::prelude::*;

/// Renders a Nest as HTML using the tt-nest.svg asset.
pub fn render(nest: &Nest) -> Html {
    let widget_id = nest.id.to_string();
    let is_copy_source = nest.is_copy_source();
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
            <img src="/images/tt-nest.svg" alt="Nest" class="nest-img"/>
            if msg_count > 0 {
                <div class="nest-badge">{msg_count}</div>
            }
        </div>
    }
}
