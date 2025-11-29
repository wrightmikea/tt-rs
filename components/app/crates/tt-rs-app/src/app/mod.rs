//! Main application component.

mod callbacks;
mod render;

use tt_rs_core::WidgetId;
use yew::prelude::*;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::new);
    let help_open = use_state(|| false);
    let dragged_box_id = use_mut_ref(|| None::<WidgetId>);
    let pending_new_box = use_mut_ref(|| None::<usize>);

    let cbs = callbacks::create_callbacks(
        state.clone(),
        help_open.clone(),
        dragged_box_id.clone(),
        pending_new_box.clone(),
    );

    callbacks::setup_keydown_listener(cbs.on_keydown.clone());

    let (copy_sources, regular_widgets) = partition_widgets(&state);

    render::render_app(&state, *help_open, &cbs, &copy_sources, &regular_widgets)
}

type WidgetRefs<'a> = Vec<(&'a WidgetId, &'a WidgetItem)>;

fn partition_widgets(state: &AppState) -> (WidgetRefs<'_>, WidgetRefs<'_>) {
    let copy_sources: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            !state.widget_in_box.contains_key(id)
                && matches!(w, WidgetItem::Number(n) if n.is_copy_source())
        })
        .collect();
    let regular: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            !state.widget_in_box.contains_key(id)
                && !matches!(w, WidgetItem::Number(n) if n.is_copy_source())
        })
        .collect();
    (copy_sources, regular)
}
