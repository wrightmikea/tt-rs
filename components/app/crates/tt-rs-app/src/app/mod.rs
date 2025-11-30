//! Main application component.

mod callbacks;
mod render;

use tt_rs_core::WidgetId;
use tt_rs_ui::{TooltipLayerProvider, UserLevel, WorkspaceMetadata};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::new);
    let help_open = use_state(|| false);
    let workspace_open = use_state(|| false);
    let user_level = use_state(UserLevel::default);
    let dragged_box_id = use_mut_ref(|| None::<WidgetId>);
    let pending_new_box = use_mut_ref(|| None::<usize>);

    // Set up keydown listener using use_effect_with directly in the component
    // This ensures the hook is properly registered with Yew's hook system
    {
        let dragged = dragged_box_id.clone();
        let pending = pending_new_box.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if dragged.borrow().is_some() {
                    if let Some(c) = e.key().chars().next().filter(|c| c.is_ascii_digit()) {
                        *pending.borrow_mut() = Some(c.to_digit(10).unwrap() as usize);
                        e.prevent_default();
                    }
                }
            }) as Box<dyn FnMut(_)>);
            window
                .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                .unwrap();
            let w = window.clone();
            let c = cb;
            move || {
                let _ =
                    w.remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref());
            }
        });
    }

    let cbs = callbacks::create_callbacks(
        state.clone(),
        help_open.clone(),
        user_level.clone(),
        workspace_open.clone(),
        dragged_box_id.clone(),
        pending_new_box.clone(),
    );

    let planes = partition_into_planes(&state, *user_level);

    // Placeholder workspace list - will be populated from storage in Part 2
    let workspaces: Vec<WorkspaceMetadata> = vec![];

    html! {
        <TooltipLayerProvider>
            { render::render_app(&state, *help_open, *workspace_open, *user_level, &cbs, &planes, &workspaces) }
        </TooltipLayerProvider>
    }
}

type WidgetRefs<'a> = Vec<(&'a WidgetId, &'a WidgetItem)>;

/// Widgets partitioned into z-planes for guaranteed stacking order.
pub struct ZPlanes<'a> {
    /// Plane 0: Copy source stacks (lowest z-index)
    pub copy_sources: WidgetRefs<'a>,
    /// Plane 0.5: Drop zones (below draggable items)
    pub dropzones: WidgetRefs<'a>,
    /// Plane 1: Values (numbers, text)
    pub values: WidgetRefs<'a>,
    /// Plane 2: Agents and comparison (robot, bird, nest, scales)
    pub agents: WidgetRefs<'a>,
    /// Plane 3: Tools (vacuum, wand) - highest z-index for regular widgets
    pub tools: WidgetRefs<'a>,
}

fn partition_into_planes(state: &AppState, level: UserLevel) -> ZPlanes<'_> {
    let is_visible = |w: &WidgetItem| -> bool {
        match level {
            UserLevel::Tt1 => !matches!(w, WidgetItem::Bird(_) | WidgetItem::Nest(_)),
            UserLevel::Tt2 => true,
        }
    };

    let not_in_box = |id: &WidgetId| !state.widget_in_box.contains_key(id);

    let copy_sources: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| not_in_box(id) && is_visible(w) && w.is_copy_source())
        .collect();

    let dropzones: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(w, WidgetItem::DropZone(_))
        })
        .collect();

    let values: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(w, WidgetItem::Number(_) | WidgetItem::Text(_))
        })
        .collect();

    let agents: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(
                    w,
                    WidgetItem::Robot(_)
                        | WidgetItem::Bird(_)
                        | WidgetItem::Nest(_)
                        | WidgetItem::Scales(_)
                )
        })
        .collect();

    let tools: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(w, WidgetItem::Vacuum(_) | WidgetItem::Wand(_))
        })
        .collect();

    ZPlanes {
        copy_sources,
        dropzones,
        values,
        agents,
        tools,
    }
}
