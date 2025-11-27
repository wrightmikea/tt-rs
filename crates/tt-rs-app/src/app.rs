//! Main application component.

use std::collections::HashMap;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{Draggable, DropEvent, Position};
use tt_rs_number::{ArithOperator, Number};
use tt_rs_text::Text;
use tt_rs_ui::Footer;
use web_sys::Element;
use yew::prelude::*;

/// A widget item with its type for rendering.
#[derive(Clone)]
enum WidgetItem {
    Number(Number),
    Text(Text),
}

impl WidgetItem {
    fn id(&self) -> WidgetId {
        match self {
            WidgetItem::Number(n) => n.id(),
            WidgetItem::Text(t) => t.id(),
        }
    }

    fn render(&self) -> Html {
        match self {
            WidgetItem::Number(n) => n.render(),
            WidgetItem::Text(t) => t.render(),
        }
    }

    fn render_small(&self) -> Html {
        // Render a smaller version for inside box holes
        match self {
            WidgetItem::Number(n) => {
                html! {
                    <div class="widget number in-hole">
                        <span class="number-operator">{ n.operator().symbol() }</span>
                        <span class="number-value">{ n.display_value() }</span>
                    </div>
                }
            }
            WidgetItem::Text(t) => {
                html! {
                    <div class="widget text in-hole">
                        <span class="text-value">{ format!("\"{}\"", t.value()) }</span>
                    </div>
                }
            }
        }
    }
}

/// A box with its holes that can contain widgets.
#[derive(Clone)]
struct BoxState {
    id: WidgetId,
    num_holes: usize,
    /// Map from hole index to the widget ID in that hole.
    contents: HashMap<usize, WidgetId>,
    erased: bool,
}

impl BoxState {
    fn new(num_holes: usize) -> Self {
        Self {
            id: WidgetId::new(),
            num_holes,
            contents: HashMap::new(),
            erased: false,
        }
    }

    fn erased() -> Self {
        Self {
            id: WidgetId::new(),
            num_holes: 0,
            contents: HashMap::new(),
            erased: true,
        }
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    /// Place a widget in a specific hole.
    fn place_in_hole(&mut self, hole_index: usize, widget_id: WidgetId) {
        self.contents.insert(hole_index, widget_id);
    }

    /// Get the widget in a specific hole.
    fn widget_in_hole(&self, hole_index: usize) -> Option<WidgetId> {
        self.contents.get(&hole_index).copied()
    }

    fn render(&self, widgets: &HashMap<WidgetId, WidgetItem>) -> Html {
        if self.erased {
            html! {
                <div class="widget box erased" data-box-id={self.id.to_string()}>
                    <span class="box-erased">{ "?" }</span>
                </div>
            }
        } else {
            html! {
                <div class="widget box" data-box-id={self.id.to_string()}>
                    <div class="box-holes">
                        { for (0..self.num_holes).map(|i| {
                            let content = if let Some(widget_id) = self.contents.get(&i) {
                                if let Some(widget) = widgets.get(widget_id) {
                                    widget.render_small()
                                } else {
                                    html! { <span class="hole-empty">{ "\u{00A0}" }</span> }
                                }
                            } else {
                                html! { <span class="hole-empty">{ "\u{00A0}" }</span> }
                            };
                            html! {
                                <div class="box-hole" data-box-id={self.id.to_string()} data-hole-index={i.to_string()}>
                                    { content }
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        }
    }
}

/// Creates sample widgets for demonstration.
fn demo_widgets() -> Vec<WidgetItem> {
    vec![
        WidgetItem::Number(Number::new(5)),
        WidgetItem::Number(Number::new(3).with_operator(ArithOperator::Subtract)),
        WidgetItem::Number(Number::new(2).with_operator(ArithOperator::Multiply)),
        WidgetItem::Number(Number::new(4).with_operator(ArithOperator::Divide)),
        WidgetItem::Number(Number::rational(1, 2)),
        WidgetItem::Number(Number::rational(3, 4)),
        WidgetItem::Number(Number::erased()),
        WidgetItem::Text(Text::new("Hello")),
        WidgetItem::Text(Text::new("World")),
        WidgetItem::Text(Text::new("ToonTalk")),
        WidgetItem::Text(Text::erased()),
    ]
}

/// Creates sample boxes.
fn demo_boxes() -> Vec<BoxState> {
    vec![BoxState::new(3), BoxState::new(5), BoxState::erased()]
}

/// Application state.
#[derive(Clone)]
struct AppState {
    /// All widgets (numbers, text).
    widgets: HashMap<WidgetId, WidgetItem>,
    /// All boxes.
    boxes: HashMap<WidgetId, BoxState>,
    /// Positions of free-floating widgets (not in boxes).
    positions: HashMap<WidgetId, Position>,
    /// Which widgets are currently inside boxes (widget_id -> (box_id, hole_index)).
    widget_in_box: HashMap<WidgetId, (WidgetId, usize)>,
}

impl AppState {
    fn new() -> Self {
        let widgets_vec = demo_widgets();
        let boxes_vec = demo_boxes();

        let mut widgets = HashMap::new();
        let mut positions = HashMap::new();
        let mut boxes = HashMap::new();

        // Initialize widgets
        let cols = 4;
        let spacing_x = 120.0;
        let spacing_y = 80.0;
        let start_x = 20.0;
        let start_y = 50.0;

        for (i, widget) in widgets_vec.into_iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = start_x + (col as f64) * spacing_x;
            let y = start_y + (row as f64) * spacing_y;
            positions.insert(widget.id(), Position::new(x, y));
            widgets.insert(widget.id(), widget);
        }

        // Initialize boxes (placed below the widgets)
        let box_start_y = start_y + 3.0 * spacing_y;
        for (i, box_state) in boxes_vec.into_iter().enumerate() {
            let x = start_x + (i as f64) * 200.0;
            positions.insert(box_state.id(), Position::new(x, box_start_y));
            boxes.insert(box_state.id(), box_state);
        }

        Self {
            widgets,
            boxes,
            positions,
            widget_in_box: HashMap::new(),
        }
    }
}

/// Find which box hole (if any) is under the given mouse position.
fn find_box_hole_at(x: f64, y: f64) -> Option<(WidgetId, usize)> {
    let window = web_sys::window()?;
    let document = window.document()?;

    // Get the element at the mouse position
    let element = document.element_from_point(x as f32, y as f32)?;

    // Check if it's a box hole or inside one
    let hole_element = find_box_hole_element(&element)?;

    // Get the box ID and hole index from data attributes
    let box_id_str = hole_element.get_attribute("data-box-id")?;
    let hole_index_str = hole_element.get_attribute("data-hole-index")?;

    let box_id = box_id_str.parse::<u64>().ok().map(WidgetId::from_u64)?;
    let hole_index = hole_index_str.parse::<usize>().ok()?;

    Some((box_id, hole_index))
}

/// Walk up the DOM tree to find a box-hole element.
fn find_box_hole_element(element: &Element) -> Option<Element> {
    let mut current = Some(element.clone());
    while let Some(el) = current {
        if el.class_list().contains("box-hole") {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    // Application state
    let state = use_state(AppState::new);

    // Callback for when a widget is moved
    let on_move = {
        let state = state.clone();
        Callback::from(move |(id, new_pos): (WidgetId, Position)| {
            let mut new_state = (*state).clone();
            new_state.positions.insert(id, new_pos);
            state.set(new_state);
        })
    };

    // Callback for when a widget is dropped
    let on_drop = {
        let state = state.clone();
        Callback::from(move |event: DropEvent| {
            let mut new_state = (*state).clone();

            // Check if we're dropping onto a box hole
            if let Some((box_id, hole_index)) =
                find_box_hole_at(event.mouse_position.x, event.mouse_position.y)
            {
                // Check if this is a widget (not a box) and the box exists with an empty hole
                if new_state.widgets.contains_key(&event.widget_id)
                    && let Some(box_state) = new_state.boxes.get_mut(&box_id)
                    && box_state.widget_in_hole(hole_index).is_none()
                {
                    // Place widget in the hole
                    box_state.place_in_hole(hole_index, event.widget_id);
                    new_state
                        .widget_in_box
                        .insert(event.widget_id, (box_id, hole_index));
                    // Remove from free-floating positions
                    new_state.positions.remove(&event.widget_id);
                }
            }

            state.set(new_state);
        })
    };

    // Get free-floating widgets (not in boxes)
    let free_widgets: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, _)| !state.widget_in_box.contains_key(id))
        .collect();

    html! {
        <div class="workspace">
            <div class="workspace-header">
                {"tt-rs - Visual Programming Environment"}
            </div>
            <div class="workspace-content">
                // Render boxes first (so widgets can be dropped on them)
                {
                    state.boxes.iter().map(|(id, box_state)| {
                        let pos = state.positions.get(id).copied().unwrap_or_default();
                        html! {
                            <Draggable
                                widget_id={*id}
                                position={pos}
                                on_move={on_move.clone()}
                            >
                                { box_state.render(&state.widgets) }
                            </Draggable>
                        }
                    }).collect::<Html>()
                }
                // Render free-floating widgets
                {
                    free_widgets.iter().map(|(id, widget)| {
                        let pos = state.positions.get(id).copied().unwrap_or_default();
                        html! {
                            <Draggable
                                widget_id={**id}
                                position={pos}
                                on_move={on_move.clone()}
                                on_drop={on_drop.clone()}
                            >
                                { widget.render() }
                            </Draggable>
                        }
                    }).collect::<Html>()
                }
            </div>
            <Footer />
        </div>
    }
}
