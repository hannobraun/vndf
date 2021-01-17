mod rock;
mod ship;
mod target;

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
            .add_plugin(rock::RockPlugin)
            .add_plugin(ship::ShipPlugin)
            .add_plugin(target::TargetPlugin);
    }
}

impl WorldPlugin {
    fn setup(mut rapier: ResMut<RapierConfiguration>) {
        rapier.gravity = na::Vector2::zeros();

        info!("Set up world.");
    }
}
