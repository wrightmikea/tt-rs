//! Enhanced Tooltip component with extended hover text.
//!
//! Provides rich tooltips that appear on hover with title, description,
//! and optional usage hints. Tooltips are rendered on z-plane 500 via
//! the TooltipLayer for guaranteed visibility.

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::tooltip_layer::TooltipLayerContext;

/// Properties for the Tooltip component.
#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// The content to wrap with a tooltip.
    pub children: Children,
    /// Short title for the tooltip.
    pub title: AttrValue,
    /// Longer description text.
    #[prop_or_default]
    pub description: AttrValue,
    /// Optional usage hint (e.g., "Drag onto a number").
    #[prop_or_default]
    pub hint: AttrValue,
    /// Position of the tooltip relative to the element (currently only affects offset).
    #[prop_or(TooltipPosition::Bottom)]
    pub position: TooltipPosition,
}

/// Position of the tooltip relative to its anchor element.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TooltipPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Enhanced tooltip component with title, description, and hints.
/// Uses TooltipLayerContext to render on z-plane 500.
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let ctx = use_context::<TooltipLayerContext>();

    // Hide tooltip when window loses focus (e.g., ctrl-tab to another app)
    {
        let ctx = ctx.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move || {
                if let Some(ref ctx) = ctx {
                    ctx.hide();
                }
            }) as Box<dyn Fn()>);
            window
                .add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref())
                .unwrap();
            let w = window;
            let c = cb;
            move || {
                let _ = w.remove_event_listener_with_callback("blur", c.as_ref().unchecked_ref());
            }
        });
    }

    let on_mouse_enter = {
        let ctx = ctx.clone();
        let title = props.title.clone();
        let description = props.description.clone();
        let hint = props.hint.clone();
        let position = props.position;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref ctx) = ctx {
                // Position tooltip to the right of cursor with offset
                let (offset_x, offset_y) = match position {
                    TooltipPosition::Right => (20.0, 0.0),
                    TooltipPosition::Left => (-200.0, 0.0),
                    TooltipPosition::Bottom => (0.0, 20.0),
                    TooltipPosition::Top => (0.0, -80.0),
                };
                let x = e.client_x() as f64 + offset_x;
                let y = e.client_y() as f64 + offset_y;
                ctx.show(title.clone(), description.clone(), hint.clone(), x, y);
            }
        })
    };

    let on_mouse_move = {
        let ctx = ctx.clone();
        let title = props.title.clone();
        let description = props.description.clone();
        let hint = props.hint.clone();
        let position = props.position;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref ctx) = ctx {
                let (offset_x, offset_y) = match position {
                    TooltipPosition::Right => (20.0, 0.0),
                    TooltipPosition::Left => (-200.0, 0.0),
                    TooltipPosition::Bottom => (0.0, 20.0),
                    TooltipPosition::Top => (0.0, -80.0),
                };
                let x = e.client_x() as f64 + offset_x;
                let y = e.client_y() as f64 + offset_y;
                ctx.show(title.clone(), description.clone(), hint.clone(), x, y);
            }
        })
    };

    let on_mouse_leave = {
        let ctx = ctx.clone();
        Callback::from(move |_| {
            if let Some(ref ctx) = ctx {
                ctx.hide();
            }
        })
    };

    html! {
        <div class="tooltip-wrapper"
             onmouseenter={on_mouse_enter}
             onmousemove={on_mouse_move}
             onmouseleave={on_mouse_leave}>
            { for props.children.iter() }
        </div>
    }
}
