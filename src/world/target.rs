use bevy::prelude::*;

#[derive(Debug)]
pub struct Target {
    position: Option<Vec2>,
}

impl Target {
    pub fn new() -> Self {
        Self { position: None }
    }

    pub fn set(&mut self, position: Vec2) {
        self.position = Some(position);
    }

    pub fn clear(&mut self) {
        self.position = None;
    }

    pub fn is_set(&self) -> bool {
        self.position.is_some()
    }

    pub fn position(&self) -> Option<Vec2> {
        self.position
    }
}
