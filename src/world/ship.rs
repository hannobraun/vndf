use bevy::prelude::*;
use bevy_rapier2d::{
    na::{self, Isometry, UnitComplex},
    physics::RigidBodyHandleComponent,
    rapier::dynamics::RigidBodySet,
};

use crate::{Player, Ship};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(control_rotation.system())
            .add_system(control_thrust.system());
    }
}

// TASK: Improve realism. Ships should require torque to rotate, not just change
//       rotation magically.
fn control_rotation(
    mut bodies: ResMut<RigidBodySet>,
    mut players: Query<(&Player, &RigidBodyHandleComponent)>,
) {
    for (player, body) in players.iter_mut() {
        let body = bodies.get_mut(body.handle()).unwrap();

        let nav_marker_angle =
            Vec2::unit_x().angle_between(player.direction_setting);

        body.set_position(
            Isometry::from_parts(
                body.position().translation,
                UnitComplex::from_angle(nav_marker_angle),
            ),
            true,
        );
    }
}

fn control_thrust(
    mut bodies: ResMut<RigidBodySet>,
    ships: Query<(&Ship, &RigidBodyHandleComponent)>,
) {
    for (ship, body) in ships.iter() {
        let body = bodies.get_mut(body.handle()).unwrap();

        let direction = body.position().rotation * na::Vector2::new(1.0, 0.0);

        let thrust = 1_000_000.0 * direction;
        body.apply_force(ship.thrust_setting * thrust, true);
    }
}
