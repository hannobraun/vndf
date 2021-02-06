use bevy_rapier2d::{na::Vector2, rapier::dynamics::RigidBody};

pub struct Engine {
    thrust_setting: f32,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            thrust_setting: 0.0,
        }
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
}
