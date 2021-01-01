use std::collections::HashSet;

use bevy::prelude::*;
use decorum::R32;
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

pub struct RockSpawner {
    rocks: HashSet<(R32, R32)>,
}

impl RockSpawner {
    pub fn new() -> Self {
        RockSpawner {
            rocks: HashSet::new(),
        }
    }

    // TASK: Spawn new rocks at each position that is passed.
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
                let pos = (R32::from_inner(x), R32::from_inner(y));

                if !self.rocks.contains(&pos) {
                    let size =
                        min_size + (max_size - min_size) * rng.gen::<f32>();
                    spawn(x, y, size);
                    self.rocks.insert(pos);
                }
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

    // TASK: Add method that cleans up rocks that are far away from the player.
    //       This should be possible using the following approach:
    //       - Convert `rocks` into `HashMap`.
    //       - Return `Entity` from the closure in `spawn_rocks`, store it in
    //         `rocks`.
    //       - When cleaning up, iterate over `HashMap`. Check actual position
    //         of each rock. If it's too far away, remove it from ECS and
    //         `rocks`.
}
