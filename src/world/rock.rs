use bevy::prelude::*;
use rand::{thread_rng, Rng as _};

pub struct Rock {
    size: f32,
}

impl Rock {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

pub struct RockSpawner;

impl RockSpawner {
    // TASK: Store information about spawned rocks, so this methods can be
    //       called with overlapping spawn areas, without causing the same rocks
    //       to be spawned multiple times.
    // TASK: Improve rock generation algorithm:
    //       - Spawn at random positions, not on a grid.
    //       - Vary min and max size, according to position.
    //       - Vary rock density, according to position.
    pub fn spawn_rocks(
        &mut self,
        center: Vec2,
        mut spawn: impl FnMut(f32, f32, f32),
    ) {
        let area = Rect {
            left: center.x - 2500.0,
            right: center.x + 2500.0,
            top: center.y - 2500.0,
            bottom: center.y + 2500.0,
        };

        let mut rng = thread_rng();

        let min_size = 50.0;
        let max_size = 300.0;

        let mut x = area.left;
        let mut y = area.top;

        loop {
            if y >= 0.0 {
                let size = min_size + (max_size - min_size) * rng.gen::<f32>();
                spawn(x, y, size);
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
