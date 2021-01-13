use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, rapier::dynamics::RigidBody};

use super::{LAYER_MARKER, LAYER_WORLD};

pub struct Heading {
    // TASK: Make private
    pub entity: Entity,
}

impl Heading {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn update(body: &RigidBody, heading: &mut Transform) {
        let offset = body.position().rotation * Vector2::new(200.0, 0.0);
        let position = body.position().translation.vector + offset;
        *heading = Transform::from_translation(Vec3::new(
            position.x,
            position.y,
            LAYER_MARKER,
        ));
    }
}

pub fn set_layer(transform: &mut Transform) {
    transform.translation.z = LAYER_WORLD;
}
