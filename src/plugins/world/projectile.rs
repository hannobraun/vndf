use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder,
};

use crate::world::projectile::{Projectile, SIZE};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system());
    }
}

impl ProjectilePlugin {
    fn setup(commands: &mut Commands) {
        commands
            .spawn((Projectile,))
            .with(RigidBodyBuilder::new_dynamic().translation(100.0, 0.0))
            .with(ColliderBuilder::cuboid(SIZE.x / 2.0, SIZE.y / 2.0));
    }
}
