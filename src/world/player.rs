use bevy::prelude::*;

pub struct Player {
    pub direction_setting: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            direction_setting: Vec2::unit_x(),
        }
    }
}
