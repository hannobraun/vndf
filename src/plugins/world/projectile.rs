use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{self, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
};

use crate::world::{physics::ColliderMap, projectile::Projectile, ship::Ship};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::clean_up.system())
            .add_system(Self::handle_impact.system());
    }
}

impl ProjectilePlugin {
    fn clean_up(
        commands: &mut Commands,
        bodies: Res<RigidBodySet>,
        ships: Query<&RigidBodyHandleComponent, With<Ship>>,
        projectiles: Query<
            (Entity, &RigidBodyHandleComponent),
            With<Projectile>,
        >,
    ) {
        let mut ships = ships.iter();
        let body = ships.next().unwrap();
        let body = bodies.get(body.handle()).unwrap();
        let ship_position = body.position().translation.vector;

        // We assume that there's only one ship. If more ships are ever added,
        // this whole thing needs to be updated.
        assert!(ships.next().is_none());

        for (entity, body) in projectiles.iter() {
            let body = bodies.get(body.handle()).unwrap();
            let projectile_position = body.position().translation.vector;

            if (ship_position - projectile_position).norm() > 2000.0 {
                commands.despawn(entity);
            }
        }
    }

    // TASK: On impact, apply force to body that is hit by projectile.
    fn handle_impact(
        commands: &mut Commands,
        collider_map: Res<ColliderMap>,
        events: Res<physics::EventQueue>,
        projectiles: Query<Entity, With<Projectile>>,
    ) {
        while let Ok(event) = events.intersection_events.pop() {
            for collider in &[event.collider1, event.collider2] {
                if let Some(entity) = collider_map.get(collider) {
                    if projectiles.get(entity).is_ok() {
                        // If the colliding entity is a projectile, remove it.
                        commands.despawn(entity);
                    }
                }
            }
        }
    }
}
