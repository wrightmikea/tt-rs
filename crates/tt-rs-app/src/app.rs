//! Main application component.

use std::collections::HashMap;
use tt_rs_box::ToonBox;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{Draggable, Position};
use tt_rs_number::{ArithOperator, Number};
use tt_rs_text::Text;
use tt_rs_ui::Footer;
use yew::prelude::*;

/// A widget item with its type for rendering.
enum WidgetItem {
    Number(Number),
    Text(Text),
    Box(ToonBox),
}

impl WidgetItem {
    fn id(&self) -> WidgetId {
        match self {
            WidgetItem::Number(n) => n.id(),
            WidgetItem::Text(t) => t.id(),
            WidgetItem::Box(b) => b.id(),
        }
    }

    fn render(&self) -> Html {
        match self {
            WidgetItem::Number(n) => n.render(),
            WidgetItem::Text(t) => t.render(),
            WidgetItem::Box(b) => b.render(),
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
        WidgetItem::Box(ToonBox::new(3)),
        WidgetItem::Box(ToonBox::new(5)),
        WidgetItem::Box(ToonBox::erased()),
    ]
}

/// Creates initial positions for widgets in a grid layout.
fn initial_positions(widgets: &[WidgetItem]) -> HashMap<WidgetId, Position> {
    let mut positions = HashMap::new();
    let cols = 4;
    let spacing_x = 120.0;
    let spacing_y = 80.0;
    let start_x = 20.0;
    let start_y = 20.0;

    for (i, widget) in widgets.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let x = start_x + (col as f64) * spacing_x;
        let y = start_y + (row as f64) * spacing_y;
        positions.insert(widget.id(), Position::new(x, y));
    }

    positions
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    // Create widgets once and store them
    let widgets = use_memo((), |()| demo_widgets());

    // Manage positions state
    let positions = use_state(|| initial_positions(&widgets));

    // Callback for when a widget is moved
    let on_move = {
        let positions = positions.clone();
        Callback::from(move |(id, new_pos): (WidgetId, Position)| {
            let mut new_positions = (*positions).clone();
            new_positions.insert(id, new_pos);
            positions.set(new_positions);
        })
    };

    html! {
        <div class="workspace">
            <div class="workspace-header">
                {"tt-rs - Visual Programming Environment"}
            </div>
            <div class="workspace-content">
                {
                    widgets.iter().map(|widget| {
                        let id = widget.id();
                        let pos = positions.get(&id).copied().unwrap_or_default();
                        html! {
                            <Draggable
                                widget_id={id}
                                position={pos}
                                on_move={on_move.clone()}
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
