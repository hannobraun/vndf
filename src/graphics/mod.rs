pub mod nav_marker;

use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use crate::Player;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TASK: Set window title to "Von Neumann Defense Force"
        // TASK: Draw background grid. Currently (Bevy 0.3) this is not
        //       straight-forward. Bevy itself doesn't have support for easy 2D
        //       drawing. There's `bevy_prototype_lyon`, but that isn't quite
        //       what I'd like either.
        //       The best idea I could come up with (using
        //       `bevy_prototype_lyon`) is to not draw the grid as several lines
        //       that leave the screen, but use a single polyline instead
        //       (connecting the otherwise separate lines outside of the
        //       screen). Sounds workable, but unless I'm seized by motivation,
        //       I'd rather just wait for easier drawing in Bevy.
        // TASK: Add system that sets z coordinate of ships explicitly.
        app.add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.15)))
            .add_system(update_camera.system())
            .add_plugin(nav_marker::NavMarkerPlugin);
    }
}

fn update_camera(
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
