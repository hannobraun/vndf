use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Point2, Vector2},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

pub const SIZE: Vec2 = Vec2 { x: 10.0, y: 10.0 };

pub struct Projectile;

impl Projectile {
    pub fn create(
        position: Point2<f32>,
        velocity: Vector2<f32>,
    ) -> (Self, RigidBodyBuilder, ColliderBuilder) {
        (
            Projectile,
            RigidBodyBuilder::new_dynamic()
                .translation(position.x, position.y)
                .linvel(velocity.x, velocity.y),
            ColliderBuilder::cuboid(SIZE.x / 2.0, SIZE.y / 2.0).sensor(true),
        )
    }
}
