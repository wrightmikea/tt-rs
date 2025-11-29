//! Accessor methods for Bird.

use tt_rs_core::WidgetId;

use crate::{Bird, BirdColor, BirdState};

impl Bird {
    /// Returns the bird's unique ID.
    pub fn id(&self) -> WidgetId {
        self.id
    }

    /// Returns the bird's color.
    pub fn color(&self) -> BirdColor {
        self.color
    }

    /// Returns the ID of the bird's home nest, if any.
    pub fn nest_id(&self) -> Option<WidgetId> {
        self.nest_id
    }

    /// Returns the bird's current state.
    pub fn state(&self) -> BirdState {
        self.state
    }

    /// Returns whether this bird is a copy source (palette item).
    pub fn is_copy_source(&self) -> bool {
        self.is_copy_source
    }

    /// Sets whether this bird is a copy source.
    pub fn set_copy_source(&mut self, is_copy: bool) {
        self.is_copy_source = is_copy;
    }

    /// Creates a copy of this bird with a new ID.
    pub fn copy_bird(&self) -> Self {
        Self {
            id: WidgetId::new(),
            color: self.color,
            nest_id: self.nest_id,
            state: BirdState::Idle,
            is_copy_source: false,
        }
    }

    /// Marks this bird as a copy source (for palette).
    pub fn as_copy_source(mut self) -> Self {
        self.is_copy_source = true;
        self
    }

    /// Pairs this bird with a nest.
    pub fn pair_with_nest(&mut self, nest_id: WidgetId) {
        self.nest_id = Some(nest_id);
    }

    /// Starts the bird flying to deliver a message.
    pub fn start_flying(&mut self) {
        self.state = BirdState::Flying;
    }

    /// Bird has delivered its message, now returning.
    pub fn start_returning(&mut self) {
        self.state = BirdState::Returning;
    }

    /// Bird has returned and is idle.
    pub fn arrive_home(&mut self) {
        self.state = BirdState::Idle;
    }
}

impl BirdColor {
    /// Returns the CSS color for this bird color.
    pub fn css_color(&self) -> &'static str {
        match self {
            BirdColor::Blue => "#63B3ED",
            BirdColor::Red => "#FC8181",
            BirdColor::Green => "#68D391",
            BirdColor::Yellow => "#F6E05E",
        }
    }
}
