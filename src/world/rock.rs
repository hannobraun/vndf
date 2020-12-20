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
// TASK: Convert into regular system that is called every frame.
fn setup(mut commands: Commands) {
    RockSpawner.spawn_rocks(
        Rect {
            left: -2500.0,
            right: 2500.0,
            top: -2500.0,
            bottom: 2500.0,
        },
        &mut commands,
    )
}

struct RockSpawner;

impl RockSpawner {
    // TASK: Only spawn rocks above ship position.
    // TASK: Only pass position (center of spawn area) here, and build spawn
    //       area from that.
    // TASK: Accept a closure that is called when a rock should be spawned
    //       instead of doing ECS stuff here.
    // TASK: Store information about spawned rocks, so this methods can be
    //       called with overlapping spawn areas, without causing the same rocks
    //       to be spawned multiple times.
    // TASK: Improve rock generation algorithm:
    //       - Spawn at random positions, not on a grid.
    //       - Vary min and max size, according to position.
    //       - Vary rock density, according to position.
    fn spawn_rocks(&mut self, area: Rect<f32>, commands: &mut Commands) {
        let mut rng = thread_rng();

        let min_size = 50.0;
        let max_size = 300.0;

        let mut x = area.left;
        let mut y = area.top;

        loop {
            // Leave out ship spawn point.
            if x != 0.0 || y != 0.0 {
                let size = min_size + (max_size - min_size) * rng.gen::<f32>();

                commands
                    .spawn((Rock { size },))
                    .with(RigidBodyBuilder::new_dynamic().translation(x, y))
                    .with(ColliderBuilder::cuboid(size / 2.0, size / 2.0));
            }

            x += 500.0;
            if x > area.right {
                y += 500.0;
                x = area.left;
            }
            if y > area.bottom {
                break;
            }
        }
    }
}
