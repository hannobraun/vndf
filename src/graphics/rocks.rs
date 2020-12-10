use bevy::prelude::*;

pub struct RocksPlugin;

impl Plugin for RocksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

// TASK: Make rocks round. At this point, I only know how to easily display
//       rectangular sprites, but once we get accessible 2D drawing primitives,
//       it would be nice to make rocks round.
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb_linear(0.5, 0.5, 1.0).into()),
        sprite: Sprite::new(Vec2::new(300.0, 300.0)),
        transform: Transform::from_translation(Vec3::new(500.0, -300.0, 0.0)),
        ..Default::default()
    });
}
