use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder, geometry::ColliderBuilder,
};
use rand::{thread_rng, Rng as _};

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
// TASK: Improve procedural rock generation.
fn setup(mut commands: Commands) {
    let mut rng = thread_rng();

    let min_size = 50.0;
    let max_size = 300.0;

    for x in -5..=5 {
        for y in -5..=5 {
            // Leave out ship spawn point.
            if x == 0 && y == 0 {
                continue;
            }

            let size = min_size + (max_size - min_size) * rng.gen::<f32>();
            let position = Vec2::new(x as f32 * 500.0, y as f32 * 500.0);

            commands
                .spawn((Rock { size },))
                .with(
                    RigidBodyBuilder::new_dynamic()
                        .translation(position.x(), position.y()),
                )
                .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
        }
    }
}
