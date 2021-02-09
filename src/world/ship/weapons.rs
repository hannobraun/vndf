use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Point2, Vector2},
    rapier::dynamics::RigidBody,
};

use crate::world::target::Target;

use super::SHIP_SIZE;

// TASK: Add support for weapon orientations. Weapons should point in a specific
//       direction and have a maximum angle at which they can shoot.
pub struct Weapons {
    weapons: Vec<Weapon>,
}

impl Weapons {
    pub fn new() -> Self {
        let weapon_back = Weapon::new(Vector2::new(SHIP_SIZE.x * -0.75, 0.0));
        let weapon_front = Weapon::new(Vector2::new(SHIP_SIZE.x * 0.75, 0.0));

        Self {
            weapons: vec![weapon_back, weapon_front],
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
                    let position =
                        body.position().translation.vector + weapon.offset;
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
    offset: Vector2<f32>,
    timer: Timer,
}

impl Weapon {
    pub fn new(offset: Vector2<f32>) -> Self {
        Self {
            offset,
            timer: Timer::from_seconds(0.2, true),
        }
    }
}
