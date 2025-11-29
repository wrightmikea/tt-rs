//! Bird struct and constructors.

use tt_rs_core::WidgetId;

/// A bird widget that delivers messages to its home nest.
///
/// Birds are the sending end of ToonTalk's message passing system.
/// When a widget is dropped on a bird, the bird flies to its paired
/// nest and delivers the message.
#[derive(Debug, Clone)]
pub struct Bird {
    pub(crate) id: WidgetId,
    pub(crate) color: BirdColor,
    pub(crate) nest_id: Option<WidgetId>,
    pub(crate) state: BirdState,
    pub(crate) is_copy_source: bool,
}

/// Colors for bird/nest pairs (must match NestColor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BirdColor {
    #[default]
    Blue,
    Red,
    Green,
    Yellow,
}

impl From<tt_rs_nest::NestColor> for BirdColor {
    fn from(color: tt_rs_nest::NestColor) -> Self {
        match color {
            tt_rs_nest::NestColor::Blue => BirdColor::Blue,
            tt_rs_nest::NestColor::Red => BirdColor::Red,
            tt_rs_nest::NestColor::Green => BirdColor::Green,
            tt_rs_nest::NestColor::Yellow => BirdColor::Yellow,
        }
    }
}

/// State of the bird.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BirdState {
    /// Bird is idle at rest, ready to receive a message.
    #[default]
    Idle,
    /// Bird is flying to deliver a message.
    Flying,
    /// Bird is returning from delivery.
    Returning,
}

impl Bird {
    /// Creates a new bird with the default color and no nest.
    pub fn new() -> Self {
        Self {
            id: WidgetId::new(),
            color: BirdColor::default(),
            nest_id: None,
            state: BirdState::Idle,
            is_copy_source: false,
        }
    }

    /// Creates a new bird with a specific color.
    pub fn with_color(color: BirdColor) -> Self {
        Self {
            id: WidgetId::new(),
            color,
            nest_id: None,
            state: BirdState::Idle,
            is_copy_source: false,
        }
    }

    /// Creates a new bird paired with a specific nest.
    pub fn with_nest(nest_id: WidgetId, color: BirdColor) -> Self {
        Self {
            id: WidgetId::new(),
            color,
            nest_id: Some(nest_id),
            state: BirdState::Idle,
            is_copy_source: false,
        }
    }
}

impl Default for Bird {
    fn default() -> Self {
        Self::new()
    }
}
