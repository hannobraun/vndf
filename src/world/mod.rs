pub mod player;
pub mod rock;
pub mod ship;

use bevy::prelude::*;
use bevy_rapier2d::{
    na,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_startup_system(Self::setup.system())
            .add_plugin(player::PlayerPlugin)
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin);
    }
}

impl WorldPlugin {
    fn setup(mut rapier: ResMut<RapierConfiguration>) {
        rapier.gravity = na::Vector2::zeros();

        info!("Set up world.");
    }
}
