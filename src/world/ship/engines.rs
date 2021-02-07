use bevy_rapier2d::{na::Vector2, rapier::dynamics::RigidBody};

pub struct Engines {
    engines: Vec<Engine>,
}

impl Engines {
    pub fn new() -> Self {
        Self {
            engines: vec![Engine::new()],
        }
    }

    /// The thrust, as a factor of maximum thrust
    ///
    /// Limited to the range of 0.0 to 1.0 (inclusive).
    pub fn thrust(&self) -> f32 {
        // TASK: Support multiple engines.
        assert_eq!(self.engines.len(), 1);
        self.engines[0].thrust()
    }

    /// Change the thrust by the given amount
    ///
    /// `change` will be added to thrust, and the result will be clamped to the
    /// range of 0.0 and 1.0 (inclusive).
    pub fn change_thrust(&mut self, change: f32) {
        // TASK: Support multiple engines.
        assert_eq!(self.engines.len(), 1);
        self.engines[0].change_thrust(change)
    }

    pub fn apply_thrust(&self, ship: &mut RigidBody) {
        // TASK: Support multiple engines.
        assert_eq!(self.engines.len(), 1);
        self.engines[0].apply_thrust(ship)
    }
}

pub struct Engine {
    thrust: f32,
    max_thrust: f32,
}

impl Engine {
    fn new() -> Self {
        Self {
            thrust: 0.0,
            max_thrust: 1_000_000.0,
        }
    }

    fn thrust(&self) -> f32 {
        self.thrust
    }

    fn change_thrust(&mut self, change: f32) {
        self.thrust += change;
        self.thrust = f32::min(f32::max(self.thrust, 0.0), 1.0);
    }

    fn apply_thrust(&self, ship: &mut RigidBody) {
        let direction = ship.position().rotation * Vector2::new(1.0, 0.0);

        let thrust = self.max_thrust * self.thrust * direction;
        ship.apply_force(thrust, true);
    }
}
