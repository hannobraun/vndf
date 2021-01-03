use bevy::math::Vec2;
use bevy_rapier2d::{
    na::{Isometry, UnitComplex, Vector2},
    rapier::dynamics::RigidBody,
};

pub const SHIP_SIZE: [f32; 2] = [150.0, 50.0];

pub struct Ship {
    pub direction_setting: Vec2,
    thrust_setting: f32,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction_setting: Vec2::unit_x(),
            thrust_setting: 0.0,
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

    pub fn thrust_setting(&self) -> f32 {
        self.thrust_setting
    }

    /// Change the thrust setting by the given amount
    pub fn change_thrust_setting(&mut self, change: f32) {
        self.thrust_setting += change;
        self.thrust_setting = f32::min(f32::max(self.thrust_setting, 0.0), 1.0);
    }

    pub fn apply_thrust(&self, body: &mut RigidBody) {
        let direction = body.position().rotation * Vector2::new(1.0, 0.0);

        let thrust = 1_000_000.0 * direction;
        body.apply_force(self.thrust_setting * thrust, true);
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
