use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub struct Player {
    pub direction_setting: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn((Player {
        direction_setting: Vec2::unit_x(),
    },));
}
