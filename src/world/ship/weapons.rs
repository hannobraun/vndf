use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Point2, Vector2},
    rapier::dynamics::RigidBody,
};

use crate::world::target::Target;

use super::SHIP_SIZE;

pub struct Weapons {
    // TASK: Add support different positions and orientations that are offset to
    //       the ship position.
    weapons: Vec<Weapon>,
}

impl Weapons {
    pub fn new() -> Self {
        let weapon = Weapon::new();

        Self {
            weapons: vec![weapon],
        }
    }

    pub fn update(
        &mut self,
        body: &RigidBody,
        target: &Target,
        time: &Time,
        mut spawn_projectile: impl FnMut(Point2<f32>, Vector2<f32>),
    ) {
        for weapon in &mut self.weapons {
            if let Some(target) = target.position() {
                if weapon.timer.tick(time.delta_seconds()).just_finished() {
                    let position = body.position().translation.vector;
                    let to_target = (Vector2::new(target.x, target.y)
                        - position)
                        .normalize();

                    let spawn_position =
                        position + to_target * SHIP_SIZE.max_element() * 1.1;
                    let velocity = to_target * 250.0;

                    spawn_projectile(spawn_position.into(), velocity);
                }
            }
        }
    }
}

pub struct Weapon {
    timer: Timer,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, true),
        }
    }
}
