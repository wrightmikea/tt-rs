//! Accessor methods for DropZone.

use crate::dropzone::DropZone;

impl DropZone {
    /// Returns the label/instruction text.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the success URL, if any.
    pub fn on_success_url(&self) -> Option<&str> {
        self.on_success_url.as_deref()
    }

    /// Returns the success message, if any.
    pub fn on_success_message(&self) -> Option<&str> {
        self.on_success_message.as_deref()
    }

    /// Returns whether the drop zone has been satisfied.
    pub fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    /// Marks the drop zone as satisfied.
    pub fn set_satisfied(&mut self, satisfied: bool) {
        self.satisfied = satisfied;
    }
}
