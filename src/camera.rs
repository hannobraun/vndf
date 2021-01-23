use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBody;

pub struct Focus {
    camera: Entity,
}

impl Focus {
    pub fn new(camera: Entity) -> Self {
        Self { camera }
    }

    pub fn camera(&self) -> Entity {
        self.camera
    }

    pub fn update_camera(body: &RigidBody, camera: &mut Transform) {
        let position = body.position().translation.vector;
        *camera =
            Transform::from_translation(Vec3::new(position.x, position.y, 1.0));
    }
}
