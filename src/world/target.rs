use bevy::prelude::*;

#[derive(Debug)]
pub struct Target {
    position: Option<Vec2>,

    /// Tracks whether `position` has changed
    ///
    /// It would be nicer to use Bevy's built-in change tracking for this, but
    /// that depends on the order of system execution. Issue:
    /// https://github.com/bevyengine/bevy/issues/68
    has_changed: bool,
}

impl Target {
    pub fn new() -> Self {
        Self {
            position: None,
            has_changed: false,
        }
    }

    pub fn set(&mut self, position: Vec2) {
        self.position = Some(position);
        self.has_changed = true;
    }

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub fn position(&mut self) -> Option<Vec2> {
        self.has_changed = false;
        self.position
    }
}
