pub mod engines;
pub mod rcs;

use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Point2, Vector2},
    rapier::dynamics::RigidBody,
};

use crate::world::target::Target;

use self::{engines::Engines, rcs::Rcs};

pub const SHIP_SIZE: Vec2 = Vec2 { x: 150.0, y: 50.0 };

// TASK: Factor out maneuvering thrusters from ship.
// TASK: Factor out weapon from ship.
pub struct Ship {
    weapon_timer: Timer,

    engines: Engines,
    rcs: Rcs,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            weapon_timer: Timer::from_seconds(0.2, true),

            engines: Engines::new(),
            rcs: Rcs::new(),
        }
    }

    pub fn engines(&self) -> &Engines {
        &self.engines
    }

    pub fn engines_mut(&mut self) -> &mut Engines {
        &mut self.engines
    }

    pub fn rcs(&self) -> &Rcs {
        &self.rcs
    }

    pub fn rcs_mut(&mut self) -> &mut Rcs {
        &mut self.rcs
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
