//! Tooltip layer component - renders tooltips on z-plane 500.
//!
//! Uses a context-based approach where tooltips register their content
//! on hover, and the layer renders them at the highest z-plane.

use yew::prelude::*;

/// Data for an active tooltip.
#[derive(Clone, PartialEq, Default)]
pub struct TooltipData {
    /// Title text.
    pub title: AttrValue,
    /// Description text.
    pub description: AttrValue,
    /// Hint text.
    pub hint: AttrValue,
    /// X position (left) in pixels.
    pub x: f64,
    /// Y position (top) in pixels.
    pub y: f64,
    /// Whether the tooltip is visible.
    pub visible: bool,
}

/// Context for tooltip layer communication.
#[derive(Clone, PartialEq)]
pub struct TooltipLayerContext {
    /// Current tooltip data.
    pub data: UseStateHandle<TooltipData>,
}

impl TooltipLayerContext {
    /// Show a tooltip at the given position.
    pub fn show(&self, title: AttrValue, description: AttrValue, hint: AttrValue, x: f64, y: f64) {
        self.data.set(TooltipData {
            title,
            description,
            hint,
            x,
            y,
            visible: true,
        });
    }

    /// Hide the tooltip.
    pub fn hide(&self) {
        let mut data = (*self.data).clone();
        data.visible = false;
        self.data.set(data);
    }
}

/// Props for TooltipLayerProvider.
#[derive(Properties, PartialEq)]
pub struct TooltipLayerProviderProps {
    pub children: Children,
}

/// Provider component that wraps the app and provides tooltip context.
#[function_component(TooltipLayerProvider)]
pub fn tooltip_layer_provider(props: &TooltipLayerProviderProps) -> Html {
    let data = use_state(TooltipData::default);
    let ctx = TooltipLayerContext { data };

    html! {
        <ContextProvider<TooltipLayerContext> context={ctx}>
            { for props.children.iter() }
        </ContextProvider<TooltipLayerContext>>
    }
}

/// The tooltip layer that renders on z-plane 500.
#[function_component(TooltipLayer)]
pub fn tooltip_layer() -> Html {
    let ctx = use_context::<TooltipLayerContext>();

    if let Some(ctx) = ctx {
        let data = &*ctx.data;
        if data.visible {
            let style = format!(
                "position: fixed; left: {}px; top: {}px; z-index: 500; pointer-events: none;",
                data.x, data.y
            );

            let has_description = !data.description.is_empty();
            let has_hint = !data.hint.is_empty();

            return html! {
                <div class="tooltip-layer-content" style={style}>
                    <div class="tooltip-title">{ &data.title }</div>
                    if has_description {
                        <div class="tooltip-description">{ &data.description }</div>
                    }
                    if has_hint {
                        <div class="tooltip-hint">{ &data.hint }</div>
                    }
                </div>
            };
        }
    }

    html! {}
}
