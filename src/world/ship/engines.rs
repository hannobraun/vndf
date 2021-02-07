use bevy_rapier2d::{na::Vector2, rapier::dynamics::RigidBody};

pub struct Engines {
    // TASK: Support different engine positions and orientations.
    engines: Vec<Engine>,
    thrust: f32,
}

impl Engines {
    pub fn new() -> Self {
        Self {
            engines: vec![Engine::new()],
            thrust: 0.0,
        }
    }

    /// The thrust, as a factor of maximum thrust
    ///
    /// Limited to the range of 0.0 to 1.0 (inclusive).
    pub fn thrust(&self) -> f32 {
        self.thrust
    }

    /// Change the thrust by the given amount
    ///
    /// `change` will be added to thrust, and the result will be clamped to the
    /// range of 0.0 and 1.0 (inclusive).
    pub fn change_thrust(&mut self, change: f32) {
        self.thrust += change;
        self.thrust = f32::min(f32::max(self.thrust, 0.0), 1.0);

        for engine in &mut self.engines {
            engine.thrust = self.thrust;
        }
    }

    pub fn apply_thrust(&self, ship: &mut RigidBody) {
        let direction = ship.position().rotation * Vector2::new(1.0, 0.0);

        for engine in &self.engines {
            let thrust = engine.max_thrust * engine.thrust * direction;
            ship.apply_force(thrust, true);
        }
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
}
