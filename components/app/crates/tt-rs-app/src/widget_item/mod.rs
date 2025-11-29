//! Widget item enum for app state.

mod render;
mod tooltip;

use tt_rs_bird::Bird;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_nest::Nest;
use tt_rs_number::Number;
use tt_rs_robot::Robot;
use tt_rs_scales::Scales;
use tt_rs_text::Text;
use tt_rs_vacuum::Vacuum;
use tt_rs_wand::Wand;
use yew::prelude::*;

pub use render::render_small;
pub use tooltip::TooltipInfo;

/// A widget item with its type for rendering.
#[derive(Clone)]
#[allow(dead_code)]
pub enum WidgetItem {
    Number(Number),
    Text(Text),
    Scales(Scales),
    Vacuum(Vacuum),
    Wand(Wand),
    Robot(Robot),
    Nest(Nest),
    Bird(Bird),
}

impl WidgetItem {
    pub fn id(&self) -> WidgetId {
        match self {
            WidgetItem::Number(n) => n.id(),
            WidgetItem::Text(t) => t.id(),
            WidgetItem::Scales(s) => s.id(),
            WidgetItem::Vacuum(v) => v.id(),
            WidgetItem::Wand(w) => w.id(),
            WidgetItem::Robot(r) => r.id(),
            WidgetItem::Nest(nest) => nest.id(),
            WidgetItem::Bird(bird) => bird.id(),
        }
    }

    pub fn render(&self) -> Html {
        render::render(self)
    }

    pub fn is_vacuum(&self) -> bool {
        matches!(self, WidgetItem::Vacuum(_))
    }
    pub fn is_wand(&self) -> bool {
        matches!(self, WidgetItem::Wand(_))
    }
    pub fn is_robot(&self) -> bool {
        matches!(self, WidgetItem::Robot(_))
    }

    pub fn tooltip_info(&self) -> &'static TooltipInfo {
        tooltip::tooltip_info(self)
    }

    #[allow(dead_code)]
    pub fn as_robot_mut(&mut self) -> Option<&mut Robot> {
        match self {
            WidgetItem::Robot(r) => Some(r),
            _ => None,
        }
    }

    /// Returns true if this widget is a copy source (palette item).
    /// Note: Only widgets that track copy source status have the method.
    /// Others (Text, Scales, tools) are never copy sources in practice.
    pub fn is_copy_source(&self) -> bool {
        match self {
            WidgetItem::Number(n) => n.is_copy_source(),
            WidgetItem::Nest(nest) => nest.is_copy_source(),
            WidgetItem::Bird(bird) => bird.is_copy_source(),
            // These widget types don't track copy source status
            WidgetItem::Text(_)
            | WidgetItem::Scales(_)
            | WidgetItem::Vacuum(_)
            | WidgetItem::Wand(_)
            | WidgetItem::Robot(_) => false,
        }
    }

    /// Creates a copy of this widget with a new ID.
    pub fn copy_widget(&self) -> WidgetItem {
        match self {
            WidgetItem::Number(n) => WidgetItem::Number(n.copy_number()),
            WidgetItem::Text(t) => WidgetItem::Text(t.copy_text()),
            WidgetItem::Scales(s) => WidgetItem::Scales(s.copy_scales()),
            WidgetItem::Vacuum(v) => WidgetItem::Vacuum(v.copy_vacuum()),
            WidgetItem::Wand(w) => WidgetItem::Wand(w.copy_wand()),
            WidgetItem::Robot(r) => WidgetItem::Robot(r.copy_robot()),
            WidgetItem::Nest(nest) => WidgetItem::Nest(nest.copy_nest()),
            WidgetItem::Bird(bird) => WidgetItem::Bird(bird.copy_bird()),
        }
    }

    /// Converts this widget to a boxed trait object for message passing.
    pub fn to_boxed_widget(&self) -> Box<dyn Widget> {
        match self {
            WidgetItem::Number(n) => Box::new(n.clone()),
            WidgetItem::Text(t) => Box::new(t.clone()),
            WidgetItem::Scales(s) => Box::new(s.clone()),
            WidgetItem::Vacuum(v) => Box::new(v.clone()),
            WidgetItem::Wand(w) => Box::new(w.clone()),
            WidgetItem::Robot(r) => Box::new(r.clone()),
            WidgetItem::Nest(nest) => Box::new(nest.clone()),
            WidgetItem::Bird(bird) => Box::new(bird.clone()),
        }
    }
}
