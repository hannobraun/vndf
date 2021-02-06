use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, rapier::dynamics::RigidBody};

use crate::world::ship::Ship;

use super::LAYER_UI;

pub struct NavMarker {
    entity: Entity,
}

impl NavMarker {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn update_position(
        ship: &Ship,
        body: &RigidBody,
        transform: &mut Transform,
    ) {
        let dir = ship.direction_setting().normalize();

        let position = body.position().translation.vector
            + Vector2::new(dir.x, dir.y) * 250.0;
        *transform = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));
    }

    pub fn update_size(ship: &Ship, sprite: &mut Sprite) {
        const MIN_SIZE: f32 = 5.0;
        const MAX_SIZE: f32 = 25.0;

        let size = MIN_SIZE + (MAX_SIZE - MIN_SIZE) * ship.engine.thrust();
        *sprite = Sprite::new(Vec2::new(size, size));
    }
}
