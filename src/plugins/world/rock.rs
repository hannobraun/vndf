use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder,
};

use crate::world::rock::{Rock, RockSpawner};

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(RockSpawner)
            .add_startup_system(Self::setup.system());
    }
}

impl RockPlugin {
    // TASK: Make rocks round. At this point, I only know how to easily display
    //       rectangular sprites, but once we get accessible 2D drawing
    //       primitives, it would be nice to make rocks round.
    // TASK: Convert into regular system that is called every frame.
    fn setup(commands: &mut Commands, mut rock_spawner: ResMut<RockSpawner>) {
        rock_spawner.spawn_rocks(Vec2::new(0.0, 0.0), |x, y, size| {
            commands
                .spawn((Rock::new(size),))
                .with(RigidBodyBuilder::new_dynamic().translation(x, y))
                .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
        })
    }
}
