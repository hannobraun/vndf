use bevy::prelude::*;

#[derive(Debug)]
pub struct Target {
    position: Option<Vec2>,

    /// Tracks whether `position` has changed
    ///
    /// It would be nicer to use Bevy's built-in change tracking for this, but
    /// that depends on the order of system execution. Issue:
    /// https://github.com/bevyengine/bevy/issues/68
    changed: bool,
}

impl Target {
    pub fn new() -> Self {
        Self {
            position: None,
            changed: false,
        }
    }

    pub fn set(&mut self, position: Vec2) {
        self.position = Some(position);
        self.changed = true;
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }

    pub fn position(&mut self) -> Option<Vec2> {
        self.changed = false;
        self.position
    }
}
