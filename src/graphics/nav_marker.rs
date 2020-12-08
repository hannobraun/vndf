use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{ui::NavMarker, Player, Ship, LAYER_UI};

pub struct NavMarkerPlugin;

impl Plugin for NavMarkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_position.system())
            .add_system(update_size.system());
    }
}

pub fn update_position(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut nav_markers: Query<(&NavMarker, &mut Transform)>,
) {
    for (player, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let (nav_marker, mut transform) =
            nav_markers.get_mut(player.nav_marker).unwrap();

        let dir = nav_marker.direction.normalize();

        let position = body.position().translation.vector
            + na::Vector2::new(dir.x(), dir.y()) * 250.0;
        *transform = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));
    }
}

pub fn update_size(
    players: Query<(&Player, &Ship)>,
    mut nav_markers: Query<&mut Sprite>,
) {
    for (player, ship) in players.iter() {
        let mut sprite = nav_markers.get_mut(player.nav_marker).unwrap();

        let min_size = 5.0;
        let max_size = 25.0;
        let size = min_size + (max_size - min_size) * ship.thrust;
        *sprite = Sprite::new(Vec2::new(size, size));
    }
}
