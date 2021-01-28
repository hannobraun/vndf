use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder,
};

pub const SIZE: Vec2 = Vec2 { x: 10.0, y: 10.0 };

pub struct Projectile;

impl Projectile {
    pub fn create() -> (Self, RigidBodyBuilder, ColliderBuilder) {
        (
            Projectile,
            RigidBodyBuilder::new_dynamic().translation(100.0, 0.0),
            ColliderBuilder::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
        )
    }
}
