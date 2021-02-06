use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Isometry, Point2, UnitComplex, Vector2},
    rapier::dynamics::RigidBody,
};

use crate::world::target::Target;

use super::engine::Engine;

pub const SHIP_SIZE: Vec2 = Vec2 { x: 150.0, y: 50.0 };

// TASK: Factor out maneuvering thrusters from ship.
// TASK: Factor out weapon from ship.
pub struct Ship {
    direction_setting: Vec2,
    weapon_timer: Timer,

    // TASK: Convert into separate component.
    pub engine: Engine,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction_setting: Vec2::unit_x(),
            weapon_timer: Timer::from_seconds(0.2, true),

            engine: Engine::new(),
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

    pub fn update_weapon(
        &mut self,
        body: &RigidBody,
        target: &Target,
        time: &Time,
        mut spawn_projectile: impl FnMut(Point2<f32>, Vector2<f32>),
    ) {
        if let Some(target) = target.position() {
            if self.weapon_timer.tick(time.delta_seconds()).just_finished() {
                let position = body.position().translation.vector;
                let to_target =
                    (Vector2::new(target.x, target.y) - position).normalize();

                let spawn_position =
                    position + to_target * SHIP_SIZE.max_element() * 1.1;
                let velocity = to_target * 250.0;

                spawn_projectile(spawn_position.into(), velocity);
            }
        }
    }
}
