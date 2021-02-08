use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Isometry, UnitComplex, Vector2},
    rapier::dynamics::RigidBody,
};

pub struct Rcs {
    direction_setting: Vec2,
}

impl Rcs {
    pub fn new() -> Self {
        Self {
            direction_setting: Vec2::unit_x(),
        }
    }

    pub fn direction_setting(&self) -> Vec2 {
        self.direction_setting
    }

    pub fn update_direction_setting(&mut self, body: &RigidBody, target: Vec2) {
        let target = Vector2::new(target.x, target.y);
        let direction = target - body.position().translation.vector;
        self.direction_setting = Vec2::new(direction.x, direction.y);
    }

    // TASK: Improve realism. Ships should require torque to rotate, not just
    //       change rotation magically.
    pub fn control_rotation(&self, body: &mut RigidBody) {
        let nav_marker_angle =
            Vec2::unit_x().angle_between(self.direction_setting);

        body.set_position(
            Isometry::from_parts(
                body.position().translation,
                UnitComplex::from_angle(nav_marker_angle),
            ),
            true,
        );
    }
}
