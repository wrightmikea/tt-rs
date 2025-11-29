//! Enhanced Tooltip component with extended hover text.
//!
//! Provides rich tooltips that appear on hover with title, description,
//! and optional usage hints.

use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

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
    /// Position of the tooltip relative to the element.
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

impl TooltipPosition {
    fn class_name(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "tooltip-top",
            TooltipPosition::Bottom => "tooltip-bottom",
            TooltipPosition::Left => "tooltip-left",
            TooltipPosition::Right => "tooltip-right",
        }
    }
}

/// Enhanced tooltip component with title, description, and hints.
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let visible = use_state(|| false);

    // Hide tooltip when window loses focus (e.g., ctrl-tab to another app)
    {
        let visible = visible.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move || visible.set(false)) as Box<dyn Fn()>);
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
        let visible = visible.clone();
        Callback::from(move |_| visible.set(true))
    };

    let on_mouse_leave = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    let tooltip_class = format!(
        "tooltip-content {}{}",
        props.position.class_name(),
        if *visible { " visible" } else { "" }
    );

    let has_description = !props.description.is_empty();
    let has_hint = !props.hint.is_empty();

    html! {
        <div class="tooltip-wrapper"
             onmouseenter={on_mouse_enter}
             onmouseleave={on_mouse_leave}>
            { for props.children.iter() }
            <div class={tooltip_class}>
                <div class="tooltip-title">{ &props.title }</div>
                if has_description {
                    <div class="tooltip-description">{ &props.description }</div>
                }
                if has_hint {
                    <div class="tooltip-hint">{ &props.hint }</div>
                }
            </div>
        </div>
    }
}
