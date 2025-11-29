//! Widget item enum for app state.

mod render;
mod tooltip;

use tt_rs_core::{Widget, WidgetId};
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
}
