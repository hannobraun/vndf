pub mod player;
pub mod rock;
pub mod ship;

use bevy::prelude::*;
use bevy_rapier2d::{
    na,
    physics::{RapierConfiguration, RapierPhysicsPlugin},
};
use slog::{info, Logger};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin)
            .add_startup_system(setup.system())
            .add_plugin(player::PlayerPlugin)
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin);
    }
}

fn setup(mut rapier: ResMut<RapierConfiguration>, log: Res<Logger>) {
    rapier.gravity = na::Vector2::zeros();

    info!(log, "Set up world.");
}
