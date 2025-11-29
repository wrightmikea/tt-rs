//! Rendering functions for Bird.

use crate::Bird;
use yew::prelude::*;

/// Renders a Bird as HTML using the tt-bird.svg asset.
pub fn render(bird: &Bird) -> Html {
    let widget_id = bird.id.to_string();
    let is_copy_source = bird.is_copy_source();

    let class = if is_copy_source {
        "widget bird copy-source"
    } else {
        "widget bird"
    };

    html! {
        <div class={class}
             data-widget-id={widget_id}
             data-copy-source={is_copy_source.to_string()}>
            <img src="images/tt-bird.svg" alt="Bird" class="bird-img"/>
        </div>
    }
}
