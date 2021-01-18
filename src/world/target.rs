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
        self.position = Some(position)
    }
}
