//! Main application component.

mod callbacks;
mod render;

use tt_rs_core::WidgetId;
use tt_rs_ui::{ConfirmDialog, TooltipLayerProvider, UserLevel, WorkspaceMetadata};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::routing::{current_route, Route};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Pending action that requires user confirmation.
#[derive(Clone, PartialEq)]
pub enum PendingAction {
    /// Reset the workspace (reload current puzzle/tutorial or sandbox default)
    Reset,
    /// Change to a different user level
    LevelChange(UserLevel),
}

/// Load puzzle/tutorial based on route.
fn load_route(route: &Route) -> Option<AppState> {
    match route {
        Route::Puzzle(id) => {
            // Try multiple variations of the puzzle ID
            let variations = [
                id.clone(),                                   // exact: fill-a-box
                format!("puzzle-{id}"),                       // with prefix: puzzle-fill-a-box
                id.replace("-a-", "-"),                       // normalized: fill-box
                format!("puzzle-{}", id.replace("-a-", "-")), // puzzle-fill-box
            ];

            for puzzle_id in &variations {
                if let Some(workspace) = crate::workspace::load_bundled_puzzle(puzzle_id) {
                    log::info!("Loaded puzzle from URL: {}", puzzle_id);
                    return Some(crate::workspace::from_workspace(&workspace));
                }
            }

            log::warn!("Puzzle not found: {} (tried: {:?})", id, variations);
            None
        }
        Route::Tutorial(id) => {
            // Try multiple variations of the tutorial ID
            let variations = [
                format!("tutorial-{id}"), // with prefix: tutorial-fill-box
                id.clone(),               // exact: fill-box
                format!("tutorial-{}", id.replace("-a-", "-")), // normalized
            ];

            for tutorial_id in &variations {
                if let Some(workspace) = crate::workspace::load_bundled_puzzle(tutorial_id) {
                    log::info!("Loaded tutorial from URL: {}", tutorial_id);
                    return Some(crate::workspace::from_workspace(&workspace));
                }
            }

            log::warn!("Tutorial not found: {} (tried: {:?})", id, variations);
            None
        }
        Route::Sandbox => None,
    }
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| {
        // Check URL on initial load
        let route = current_route();
        load_route(&route).unwrap_or_default()
    });
    let help_open = use_state(|| false);
    let workspace_open = use_state(|| false);
    let user_level = use_state(UserLevel::default);
    let dragged_box_id = use_mut_ref(|| None::<WidgetId>);
    let pending_new_box = use_mut_ref(|| None::<usize>);

    // Track whether the workspace has been modified
    let dirty = use_state(|| false);

    // Track pending action that needs confirmation
    let pending_action: UseStateHandle<Option<PendingAction>> = use_state(|| None);

    // Set up hashchange listener for URL navigation
    {
        let state = state.clone();
        let dirty = dirty.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move || {
                let route = current_route();
                if let Some(new_state) = load_route(&route) {
                    state.set(new_state);
                    dirty.set(false); // Fresh load is not dirty
                }
            }) as Box<dyn FnMut()>);
            window
                .add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref())
                .unwrap();
            let w = window.clone();
            let c = cb;
            move || {
                let _ =
                    w.remove_event_listener_with_callback("hashchange", c.as_ref().unchecked_ref());
            }
        });
    }

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

    let cbs = callbacks::create_callbacks(callbacks::CallbackConfig {
        state: state.clone(),
        help_open: help_open.clone(),
        user_level: user_level.clone(),
        workspace_open: workspace_open.clone(),
        dragged_box_id: dragged_box_id.clone(),
        pending_new_box: pending_new_box.clone(),
        dirty: dirty.clone(),
        pending_action: pending_action.clone(),
    });

    let planes = partition_into_planes(&state, *user_level);

    // Placeholder workspace list - will be populated from storage in Part 2
    let workspaces: Vec<WorkspaceMetadata> = vec![];

    // Confirmation dialog callbacks
    let on_confirm = {
        let pending_action = pending_action.clone();
        let state = state.clone();
        let dirty = dirty.clone();
        let user_level = user_level.clone();
        Callback::from(move |_| {
            if let Some(action) = (*pending_action).clone() {
                match action {
                    PendingAction::Reset => {
                        // Get the current route at reset time (not at callback creation time)
                        let current_id = match current_route() {
                            Route::Puzzle(id) => {
                                // Don't double-prefix if ID already has a prefix
                                if id.starts_with("puzzle-") || id.starts_with("tutorial-") {
                                    Some(id)
                                } else {
                                    Some(format!("puzzle-{id}"))
                                }
                            }
                            Route::Tutorial(id) => {
                                if id.starts_with("tutorial-") {
                                    Some(id)
                                } else {
                                    Some(format!("tutorial-{id}"))
                                }
                            }
                            Route::Sandbox => None,
                        };
                        log::info!("Reset confirm: current_id = {:?}", current_id);

                        // Reset to initial state
                        if let Some(ref id) = current_id {
                            // Reload puzzle/tutorial
                            if let Some(workspace) = crate::workspace::load_bundled_puzzle(id) {
                                log::info!("Loaded workspace for reset: {}", id);
                                let new_state = crate::workspace::from_workspace(&workspace);
                                state.set(new_state);
                            } else {
                                log::warn!("Failed to load workspace: {}", id);
                            }
                        } else {
                            // Sandbox mode - reset to default state
                            log::info!("Reset to sandbox default state");
                            state.set(AppState::new());
                        }
                        dirty.set(false);
                    }
                    PendingAction::LevelChange(level) => {
                        user_level.set(level);
                        // Reset to default state for the new level
                        let mut new_state = AppState::new();
                        new_state.text_pane_content =
                            crate::state::default_notes_for_level(level).to_string();
                        state.set(new_state);
                        dirty.set(false);
                        // Update URL to sandbox
                        crate::routing::set_route(&Route::Sandbox);
                    }
                }
                pending_action.set(None);
            }
        })
    };

    let on_cancel = {
        let pending_action = pending_action.clone();
        Callback::from(move |_| {
            pending_action.set(None);
        })
    };

    // Generate dialog message based on pending action
    let dialog_message = (*pending_action).as_ref().map(|action| match action {
        PendingAction::Reset => {
            "You have unsaved changes. Reset will discard all changes. Continue?"
        }
        PendingAction::LevelChange(_) => {
            "You have unsaved changes. Changing level will discard all changes. Continue?"
        }
    });

    html! {
        <TooltipLayerProvider>
            { render::render_app(&state, *help_open, *workspace_open, *user_level, &cbs, &planes, &workspaces) }
            if let Some(message) = dialog_message {
                <ConfirmDialog
                    title="Discard Changes?"
                    message={message}
                    confirm_label="Discard"
                    cancel_label="Keep Working"
                    on_confirm={on_confirm}
                    on_cancel={on_cancel}
                />
            }
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
