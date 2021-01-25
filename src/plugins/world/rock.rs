use bevy::{ecs::QueryError::NoSuchEntity, prelude::*};
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
            .add_system(Self::spawn_rocks.system())
            .add_system(Self::clean_up_rocks.system());
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
                let entity = commands
                    .spawn((Rock::new(size),))
                    .with(
                        RigidBodyBuilder::new_dynamic()
                            .translation(position.x, position.y),
                    )
                    .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0))
                    .current_entity()
                    .unwrap();

                trace!("ID of spawned rock: {:?}", entity);

                entity
            });
        }
    }

    fn clean_up_rocks(
        commands: &mut Commands,
        mut rock_spawner: ResMut<RockSpawner>,
        bodies: Res<RigidBodySet>,
        ships: Query<&RigidBodyHandleComponent, With<Ship>>,
        rocks: Query<&RigidBodyHandleComponent, With<Rock>>,
    ) {
        let mut ships = ships.iter();
        let body = ships.next().unwrap();
        let body = bodies.get(body.handle()).unwrap();
        let ship_position = body.position().translation.vector;

        // We assume that there's only one ship. If more ships are ever added,
        // this whole thing needs to be updated.
        assert!(ships.next().is_none());

        rock_spawner.clean_up(
            ship_position,
            &|rock| {
                debug!("Querying rock position: {:?}", rock);

                match rocks.get(rock) {
                    Ok(body) => {
                        let body = bodies.get(body.handle()).unwrap();
                        Some(body.position().translation.vector)
                    }
                    Err(NoSuchEntity) => {
                        // If the rock doesn't exist in the regular query, then // theoretically its rigid body must not have
                        // initialized yet. I tried to verify that using a
                        // second query, hoping to prevent missing any bugs,
                        // but for some reason it can't be found in the second
                        // query either. No idea what's wrong.

                        // TASK: Figure out what the valid reasons are that
                        //       rocks aren't found, and verify here that one of
                        //       these reasons actually applies here.

                        None
                    }
                    Err(err) => {
                        panic!("Unexpected error: {:?}", err);
                    }
                }
            },
            &mut |rock| {
                info!("Despawning rock: {:?}", rock);
                commands.despawn(rock);
            },
        );
    }
}
