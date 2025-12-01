//! Animated cursor component for "Show Me" demonstrations.
//!
//! Displays a hand pointer that follows demo animation steps,
//! showing users how to perform actions in the workspace.

use yew::prelude::*;

/// Properties for the DemoCursor component.
#[derive(Properties, Clone, PartialEq)]
pub struct DemoCursorProps {
    /// X position of the cursor.
    pub x: f64,
    /// Y position of the cursor.
    pub y: f64,
    /// Whether the cursor is currently dragging.
    #[prop_or(false)]
    pub is_dragging: bool,
    /// Duration of the transition animation in milliseconds.
    #[prop_or(300)]
    pub transition_duration: u32,
}

/// Animated hand cursor for demo playback.
#[function_component(DemoCursor)]
pub fn demo_cursor(props: &DemoCursorProps) -> Html {
    let style = format!(
        "position: fixed; \
         left: {}px; \
         top: {}px; \
         z-index: 999999; \
         pointer-events: none; \
         transition: left {}ms ease-out, top {}ms ease-out; \
         filter: drop-shadow(2px 2px 4px rgba(0,0,0,0.4));",
        props.x, props.y, props.transition_duration, props.transition_duration
    );

    // Use a pointing hand emoji that changes when dragging
    let cursor_class = if props.is_dragging {
        "demo-cursor dragging"
    } else {
        "demo-cursor"
    };

    html! {
        <div class={cursor_class} style={style}>
            <svg width="48" height="48" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                // White background circle for visibility
                <circle cx="24" cy="24" r="20" fill="white" opacity="0.9"/>
                // Hand pointer icon - larger and more visible
                <g transform="translate(8, 6)">
                    <g fill="none" stroke="#333" stroke-width="2">
                        // Index finger (pointing)
                        <path d="M16 4 L16 16" stroke-linecap="round"/>
                        // Palm and other fingers
                        <path d="M11 16 L11 22" stroke-linecap="round"/>
                        <path d="M21 16 L21 20" stroke-linecap="round"/>
                        <path d="M26 18 L26 22" stroke-linecap="round"/>
                        // Palm body
                        <path d="M8 22 Q8 32 16 36 Q24 32 24 22" stroke-linecap="round"/>
                    </g>
                    // Fingertip circle - bright yellow for visibility
                    <circle cx="16" cy="8" r="5" fill="#FFD93D" stroke="#333" stroke-width="1.5"/>
                </g>
                if props.is_dragging {
                    // Show "grabbing" indicator - pulsing ring
                    <circle cx="24" cy="14" r="8" fill="none" stroke="#FF6B35" stroke-width="3" opacity="0.9">
                        <animate attributeName="r" values="8;14;8" dur="0.8s" repeatCount="indefinite"/>
                        <animate attributeName="opacity" values="0.9;0.3;0.9" dur="0.8s" repeatCount="indefinite"/>
                    </circle>
                }
            </svg>
        </div>
    }
}
