use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system());
    }
}

pub fn update(
    bodies: Res<RigidBodySet>,
    players: Query<(&Player, &RigidBodyHandleComponent)>,
    mut transforms: Query<(&mut Transform,)>,
) {
    for (player, body) in players.iter() {
        let body = bodies
            .get(body.handle())
            .expect("Could not find body for ship");

        let mut camera = transforms.get_mut(player.camera).unwrap().0;
        let position = body.position().translation.vector;
        *camera =
            Transform::from_translation(Vec3::new(position.x, position.y, 1.0));
    }
}
