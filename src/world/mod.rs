pub mod rock;

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_plugin(rock::RockPlugin);
    }
}
