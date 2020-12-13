use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder,
};

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub struct Rock {
    size: f32,
}

impl Rock {
    pub fn size(&self) -> f32 {
        self.size
    }
}

// TASK: Make rocks round. At this point, I only know how to easily display
//       rectangular sprites, but once we get accessible 2D drawing primitives,
//       it would be nice to make rocks round.
// TASK: Create rocks procedurally.
// TASK: Create rocks of varying sizes.
fn setup(mut commands: Commands) {
    let positions = [
        Vec2::new(500.0, -300.0),
        Vec2::new(-50.0, -350.0),
        Vec2::new(-400.0, 200.0),
    ];
    for &position in positions.iter() {
        let size = 300.0;

        commands
            .spawn((Rock { size },))
            .with(
                RigidBodyBuilder::new_dynamic()
                    .translation(position.x(), position.y()),
            )
            .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
    }
}
