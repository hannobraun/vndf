use bevy_rapier2d::{na, rapier::dynamics::RigidBody};

pub const SHIP_SIZE: [f32; 2] = [150.0, 50.0];

pub struct Ship {
    // TASK: Make private
    pub thrust_setting: f32,
}

impl Ship {
    pub fn thrust_setting(&self) -> f32 {
        self.thrust_setting
    }

    /// Change the thrust setting by the given amount
    pub fn change_thrust_setting(&mut self, change: f32) {
        self.thrust_setting += change;
        self.thrust_setting = f32::min(f32::max(self.thrust_setting, 0.0), 1.0);
    }

    pub fn apply_thrust(&self, body: &mut RigidBody) {
        let direction = body.position().rotation * na::Vector2::new(1.0, 0.0);

        let thrust = 1_000_000.0 * direction;
        body.apply_force(self.thrust_setting * thrust, true);
    }
}
