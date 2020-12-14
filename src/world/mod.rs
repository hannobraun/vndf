pub mod player;
pub mod rock;
pub mod ship;

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin);
    }
}
