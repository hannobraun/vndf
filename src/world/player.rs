use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system());
    }
}

impl PlayerPlugin {
    fn setup(commands: &mut Commands) {
        commands.spawn((Player {
            direction_setting: Vec2::unit_x(),
        },));
    }
}

pub struct Player {
    pub direction_setting: Vec2,
}
