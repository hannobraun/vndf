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
// TASK: Create rocks procedurally.
fn setup(mut commands: Commands) {
    let mut rng = thread_rng();

    let min_size = 50.0;
    let max_size = 300.0;

    let positions = [
        (
            min_size + (max_size - min_size) * rng.gen::<f32>(),
            Vec2::new(500.0, -300.0),
        ),
        (
            min_size + (max_size - min_size) * rng.gen::<f32>(),
            Vec2::new(-50.0, -350.0),
        ),
        (
            min_size + (max_size - min_size) * rng.gen::<f32>(),
            Vec2::new(-400.0, 200.0),
        ),
    ];
    for &(size, position) in positions.iter() {
        commands
            .spawn((Rock { size },))
            .with(
                RigidBodyBuilder::new_dynamic()
                    .translation(position.x(), position.y()),
            )
            .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
    }
}
