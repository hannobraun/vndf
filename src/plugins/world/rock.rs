use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

use crate::world::{
    rock::{Rock, RockSpawner},
    ship::Ship,
};

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(RockSpawner::new())
            .add_system(Self::spawn_rocks.system());
    }
}

impl RockPlugin {
    // TASK: Make rocks round. At this point, I only know how to easily display
    //       rectangular sprites, but once we get accessible 2D drawing
    //       primitives, it would be nice to make rocks round.
    fn spawn_rocks(
        commands: &mut Commands,
        mut rock_spawner: ResMut<RockSpawner>,
        bodies: Res<RigidBodySet>,
        ships: Query<&RigidBodyHandleComponent, With<Ship>>,
    ) {
        for player_ship in ships.iter() {
            let body = bodies.get(player_ship.handle()).unwrap();
            let spawn_position = body.position().translation.vector;

            rock_spawner.spawn(spawn_position, |position, size| {
                commands
                    .spawn((Rock::new(size),))
                    .with(
                        RigidBodyBuilder::new_dynamic()
                            .translation(position.x, position.y),
                    )
                    .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
            });
        }
    }
}
