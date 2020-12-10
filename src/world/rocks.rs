use bevy::prelude::*;

pub struct RocksPlugin;

impl Plugin for RocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub struct Rock {
    pub position: Vec2,
}

// TASK: Make rocks round. At this point, I only know how to easily display
//       rectangular sprites, but once we get accessible 2D drawing primitives,
//       it would be nice to make rocks round.
// TASK: Add physics components to rock entities.
// TASK: Add multiple rocks.
fn setup(mut commands: Commands) {
    commands.spawn((Rock {
        position: Vec2::new(500.0, -300.0),
    },));
}
