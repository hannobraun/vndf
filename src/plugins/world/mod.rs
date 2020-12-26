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
            .add_plugin(crate::world::player::PlayerPlugin)
            .add_plugin(crate::world::rock::RockPlugin)
            .add_plugin(crate::world::ship::ShipPlugin);
    }
}

impl WorldPlugin {
    fn setup(mut rapier: ResMut<RapierConfiguration>) {
        rapier.gravity = na::Vector2::zeros();

        info!("Set up world.");
    }
}
