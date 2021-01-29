use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
        math::Isometry,
    },
};

use crate::world::{
    projectile::Projectile,
    ship::{Ship, SHIP_SIZE},
    target::Target,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system())
            .add_system(Self::control_rotation.system())
            .add_system(Self::control_thrust.system())
            .add_system(Self::update_weapon.system());
    }
}

impl ShipPlugin {
    fn setup(commands: &mut Commands) {
        commands
            .spawn((Ship::new(),))
            .with(
                RigidBodyBuilder::new_dynamic()
                    .position(Isometry::translation(0.0, -200.0))
                    .linvel(10.0, 0.0),
            )
            .with(ColliderBuilder::cuboid(
                SHIP_SIZE.x / 2.0,
                SHIP_SIZE.y / 2.0,
            ));
    }

    fn control_rotation(
        mut bodies: ResMut<RigidBodySet>,
        mut ships: Query<(&Ship, &RigidBodyHandleComponent)>,
    ) {
        for (ship, body) in ships.iter_mut() {
            let body = bodies.get_mut(body.handle()).unwrap();
            ship.control_rotation(body);
        }
    }

    fn control_thrust(
        mut bodies: ResMut<RigidBodySet>,
        ships: Query<(&Ship, &RigidBodyHandleComponent)>,
    ) {
        for (ship, body) in ships.iter() {
            let body = bodies.get_mut(body.handle()).unwrap();
            ship.apply_thrust(body);
        }
    }

    fn update_weapon(
        commands: &mut Commands,
        time: Res<Time>,
        bodies: Res<RigidBodySet>,
        mut ships: Query<(&mut Ship, &RigidBodyHandleComponent, &Target)>,
    ) {
        for (mut ship, body, target) in ships.iter_mut() {
            let body = bodies.get(body.handle()).unwrap();
            ship.update_weapon(body, &target, &time, |position| {
                commands.spawn(Projectile::create(position));
            });
        }
    }
}
