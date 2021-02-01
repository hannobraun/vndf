use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::world::{projectile::Projectile, ship::Ship};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::clean_up.system());
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
}

// TASK: Destroy projectiles on impact.
// TASK: On impact, apply force to body that is hit by projectile.
