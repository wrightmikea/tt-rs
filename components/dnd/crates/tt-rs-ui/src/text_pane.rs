//! TextPane widget for workspace documentation.
//!
//! An editable, resizable floating text panel for displaying
//! workspace descriptions, instructions, and tutorial content.

use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Properties for the TextPane component.
#[derive(Properties, Clone, PartialEq)]
pub struct TextPaneProps {
    /// The text content.
    pub content: String,
    /// Callback when content changes.
    pub on_change: Callback<String>,
    /// Whether the pane is in read-only mode.
    #[prop_or(false)]
    pub readonly: bool,
    /// Initial width in pixels.
    #[prop_or(300.0)]
    pub width: f64,
    /// Initial height in pixels.
    #[prop_or(200.0)]
    pub height: f64,
    /// Callback when size changes (width, height).
    #[prop_or_default]
    pub on_resize: Callback<(f64, f64)>,
    /// Title shown in the header.
    #[prop_or("Notes".into())]
    pub title: AttrValue,
    /// Whether the pane can be collapsed.
    #[prop_or(true)]
    pub collapsible: bool,
}

/// Editable text pane widget.
#[function_component(TextPane)]
pub fn text_pane(props: &TextPaneProps) -> Html {
    let collapsed = use_state(|| false);
    let is_resizing = use_state(|| false);
    let resize_start = use_state(|| (0.0, 0.0, 0.0, 0.0)); // (startX, startY, startW, startH)
    let current_size = use_state(|| (props.width, props.height));

    // Update size when props change
    {
        let current_size = current_size.clone();
        let width = props.width;
        let height = props.height;
        use_effect_with((width, height), move |(w, h)| {
            current_size.set((*w, *h));
            || ()
        });
    }

    let on_toggle_collapse = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    let on_content_input = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            on_change.emit(textarea.value());
        })
    };

    // Resize handling
    let on_resize_start = {
        let is_resizing = is_resizing.clone();
        let resize_start = resize_start.clone();
        let current_size = current_size.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_resizing.set(true);
            let (w, h) = *current_size;
            resize_start.set((e.client_x() as f64, e.client_y() as f64, w, h));
        })
    };

    // Set up global mouse move/up handlers for resize using use_effect
    {
        let is_resizing = is_resizing.clone();
        let resize_start = resize_start.clone();
        let current_size = current_size.clone();
        let on_resize = props.on_resize.clone();

        use_effect_with(*is_resizing, move |resizing| {
            let resizing = *resizing;
            let cleanup: Box<dyn FnOnce()> = if !resizing {
                Box::new(|| {})
            } else {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();

                let current_size_clone = current_size.clone();
                let resize_start_clone = resize_start.clone();
                let on_resize_clone = on_resize.clone();

                let on_mouse_move = {
                    let current_size = current_size_clone.clone();
                    let resize_start = resize_start_clone.clone();
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        let (start_x, start_y, start_w, start_h) = *resize_start;
                        let dx = e.client_x() as f64 - start_x;
                        let dy = e.client_y() as f64 - start_y;
                        let new_w = (start_w + dx).clamp(150.0, 800.0);
                        let new_h = (start_h + dy).clamp(100.0, 600.0);
                        current_size.set((new_w, new_h));
                    })
                        as Box<dyn FnMut(_)>)
                };

                let is_resizing_clone = is_resizing.clone();
                let current_size_final = current_size_clone;
                let on_resize_final = on_resize_clone;

                let on_mouse_up =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                        is_resizing_clone.set(false);
                        let (w, h) = *current_size_final;
                        on_resize_final.emit((w, h));
                    })
                        as Box<dyn FnMut(_)>);

                document
                    .add_event_listener_with_callback(
                        "mousemove",
                        on_mouse_move.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                document
                    .add_event_listener_with_callback(
                        "mouseup",
                        on_mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let doc = document.clone();
                let move_cb = on_mouse_move;
                let up_cb = on_mouse_up;

                Box::new(move || {
                    let _ = doc.remove_event_listener_with_callback(
                        "mousemove",
                        move_cb.as_ref().unchecked_ref(),
                    );
                    let _ = doc.remove_event_listener_with_callback(
                        "mouseup",
                        up_cb.as_ref().unchecked_ref(),
                    );
                })
            };

            cleanup
        });
    }

    let (width, height) = *current_size;
    let style = if *collapsed {
        format!("width: {width}px;")
    } else {
        format!("width: {width}px; height: {height}px;")
    };

    let pane_class = classes!(
        "text-pane",
        (*collapsed).then_some("collapsed"),
        props.readonly.then_some("readonly"),
    );

    html! {
        <div class={pane_class} style={style}>
            <div class="text-pane-header">
                <span class="text-pane-title">{ &props.title }</span>
                <div class="text-pane-controls">
                    if props.collapsible {
                        <button
                            class="text-pane-btn collapse-btn"
                            onclick={on_toggle_collapse}
                            title={if *collapsed { "Expand" } else { "Collapse" }}
                        >
                            { if *collapsed { "\u{25B6}" } else { "\u{25BC}" } }
                        </button>
                    }
                </div>
            </div>
            if !*collapsed {
                <div class="text-pane-content">
                    <textarea
                        class="text-pane-textarea"
                        value={props.content.clone()}
                        oninput={on_content_input}
                        readonly={props.readonly}
                        placeholder="Enter workspace notes..."
                    />
                </div>
                <div class="text-pane-resize-handle" onmousedown={on_resize_start} />
            }
        </div>
    }
}
