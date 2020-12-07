use bevy::prelude::*;
use bevy_rapier2d::{
    na, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::{Player, Ship, LAYER_UI};

// TASK: Split into two system, one that updates position, another that updates
//       size.
pub fn update_nav_marker(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &Ship, &RigidBodyHandleComponent)>,
    mut nav_markers: Query<(&mut Transform, &mut Sprite)>,
) {
    for (player, ship, body) in players.iter() {
        let body = bodies.get(body.handle()).unwrap();
        let (mut transform, mut sprite) =
            nav_markers.get_mut(player.nav_marker.entity).unwrap();

        let dir = player.nav_marker.direction.normalize();

        let position = body.position().translation.vector
            + na::Vector2::new(dir.x(), dir.y()) * 250.0;
        *transform = Transform::from_translation(Vec3::new(
            position.x, position.y, LAYER_UI,
        ));

        let min_size = 5.0;
        let max_size = 25.0;
        let size = min_size + (max_size - min_size) * ship.thrust;
        *sprite = Sprite::new(Vec2::new(size, size));
    }
}