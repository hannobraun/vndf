use bevy::prelude::*;
use bevy_rapier2d::{
    na::{self, Vector2},
    rapier::dynamics::RigidBody,
};
use na::Rotation2;

use super::LAYER_UI;

pub struct Course {
    // TASK: Make private.
    pub entity: Entity,
}

impl Course {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn update(
        body: &RigidBody,
        sprite: &mut Sprite,
        transform: &mut Transform,
    ) {
        let translation = body.position().translation;
        let rotation =
            Rotation2::rotation_between(&Vector2::new(1.0, 0.0), body.linvel())
                .angle();

        let speed = body.linvel().magnitude();
        let length = speed * 30.0; // show course for next 30 seconds

        let translation = Transform::from_translation(Vec3::new(
            translation.x,
            translation.y,
            LAYER_UI,
        ));
        let offset =
            Transform::from_translation(Vec3::new(length / 2.0, 0.0, 0.0));
        let rotation =
            Transform::from_rotation(Quat::from_rotation_z(rotation));

        *sprite = Sprite::new(Vec2::new(length, 1.0));
        *transform = translation.mul_transform(rotation).mul_transform(offset);
    }
}
