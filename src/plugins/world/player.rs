use bevy::prelude::*;

use crate::world::player::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system());
    }
}

impl PlayerPlugin {
    fn setup(commands: &mut Commands) {
        commands.spawn((Player::new(),));
    }
}
